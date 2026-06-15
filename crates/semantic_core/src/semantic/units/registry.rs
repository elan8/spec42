use std::collections::{BTreeMap, HashMap, HashSet};

use crate::semantic::graph::SemanticGraph;
use crate::semantic::units::graph_ingest::ingest_units_from_graph;

#[derive(Debug, Clone, PartialEq)]
pub struct UnitDef {
    pub symbol: String,
    pub dimension: String,
    pub reference_unit: Option<String>,
    pub conversion_factor: f64,
    pub conversion_offset: f64,
    /// Algebraic unit expression from `attribute newton : ForceUnit = kg * m / s^2`.
    pub algebraic_expr: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct UnitRegistry {
    by_symbol: HashMap<String, UnitDef>,
    conflicted_symbols: HashSet<String>,
    prefixes_by_name: HashMap<String, f64>,
    prefixes_by_symbol: HashMap<String, f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitError {
    UnknownUnit,
    IncompatibleDimension,
    UnsupportedConversion,
    AmbiguousMetadata,
}

#[derive(Debug, Clone, PartialEq)]
struct ReducedUnit {
    root_symbol: String,
    scale: f64,
    offset: f64,
    dimension: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct CanonicalUnitExpr {
    exponents: BTreeMap<String, i32>,
}

impl CanonicalUnitExpr {
    fn add_power(&mut self, symbol: String, power: i32) {
        let entry = self.exponents.entry(symbol).or_insert(0);
        *entry += power;
        if *entry == 0 {
            self.exponents.retain(|_, value| *value != 0);
        }
    }
}

impl UnitRegistry {
    /// Builds a unit index from the linked semantic graph (graph-only path).
    pub fn from_graph(graph: &SemanticGraph) -> Self {
        let mut registry = UnitRegistry::default();
        ingest_units_from_graph(graph, &mut registry);
        registry.finalize_ingest();
        registry
    }

    /// Alias for [`Self::from_graph`].
    pub fn from_semantic_graph(graph: &SemanticGraph) -> Self {
        Self::from_graph(graph)
    }

    pub(crate) fn ingest_unit_prefix(&mut self, name: &str, symbol: Option<&str>, factor: f64) {
        self.prefixes_by_name.insert(name.to_string(), factor);
        if let Some(symbol) = symbol {
            self.prefixes_by_symbol.insert(symbol.to_string(), factor);
        }
    }

    pub(crate) fn prefix_factor_by_name(&self, prefix_name: &str) -> Option<f64> {
        self.prefixes_by_name.get(prefix_name).copied().or_else(|| {
            prefix_name
                .rsplit_once("::")
                .and_then(|(_, base)| self.prefixes_by_name.get(base).copied())
        })
    }

    pub(crate) fn ingest_unit_def(&mut self, def: UnitDef) {
        self.upsert_unit_def(def);
    }

    pub fn get(&self, symbol: &str) -> Option<&UnitDef> {
        self.by_symbol.get(&normalize_symbol(symbol))
    }

    pub fn hover_markdown_for_unit_literal(&self, raw_unit: &str) -> Option<String> {
        let cleaned = strip_quotes(raw_unit.trim());
        if cleaned.is_empty() {
            return None;
        }
        let factors = parse_unit_expression(&cleaned).ok()?;
        let mut lines = vec![format!("**Unit literal** `[{}]`", cleaned), String::new()];
        for (symbol, exp) in factors {
            if self.conflicted_symbols.contains(&symbol) {
                lines.push(format!("*{symbol}* — ambiguous unit metadata"));
                continue;
            }
            let def = self.by_symbol.get(&symbol)?;
            let mut line = format!("*{}*", def.symbol);
            if exp != 1 {
                line.push_str(&format!("^{exp}"));
            }
            line.push_str(&format!(" — `{}`", def.dimension));
            if let Some(ref_unit) = &def.reference_unit {
                line.push_str(&format!(", reference `{ref_unit}`"));
            }
            lines.push(line);
        }
        Some(lines.join("\n"))
    }

    pub fn hover_markdown_for_unknown_unit_literal(raw_unit: &str) -> String {
        format!(
            "**Unit literal** `[{}]`\n\nSysML value-expression unit suffix. Not found in indexed quantity/unit catalogs.",
            raw_unit.trim()
        )
    }

    pub fn has_symbol(&self, symbol: &str) -> bool {
        self.by_symbol.contains_key(&normalize_symbol(symbol))
    }

    /// Returns true when every factor in a unit expression resolves against indexed catalogs.
    pub fn is_recognized_unit_expression(&self, raw_unit: &str) -> bool {
        if self.canonicalize_unit_expr(Some(raw_unit)).is_ok() {
            return true;
        }
        let normalized = normalize_symbol(raw_unit);
        if let Some(def) = self.by_symbol.get(&normalized) {
            if let Some(expr) = &def.algebraic_expr {
                return self.canonicalize_unit_expr(Some(expr)).is_ok();
            }
        }
        false
    }

    /// Returns the quantity-unit dimension string for a recognized unit expression (e.g. `PowerUnit`).
    pub fn unit_expression_dimension(&self, raw_unit: &str) -> Option<String> {
        let factors = parse_unit_expression(raw_unit).ok()?;
        let [(symbol, 1)] = factors.as_slice() else {
            return None;
        };
        self.by_symbol
            .get(symbol)
            .map(|def| def.dimension.clone())
            .or_else(|| {
                self.reduce_to_root(symbol)
                    .ok()
                    .map(|reduced| reduced.dimension)
            })
    }

    pub fn convert_value(&self, value: f64, from: &str, to: &str) -> Result<f64, UnitError> {
        let from_norm = normalize_symbol(from);
        let to_norm = normalize_symbol(to);
        if from_norm == to_norm {
            return Ok(value);
        }
        let from_reduced = self.reduce_to_root(&from_norm)?;
        let to_reduced = self.reduce_to_root(&to_norm)?;
        if from_reduced.root_symbol != to_reduced.root_symbol
            || from_reduced.dimension != to_reduced.dimension
        {
            return Err(UnitError::IncompatibleDimension);
        }
        let root_value = value * from_reduced.scale + from_reduced.offset;
        Ok((root_value - to_reduced.offset) / to_reduced.scale)
    }

    pub fn compose_product(
        &self,
        left_value: f64,
        left_unit: Option<&str>,
        right_value: f64,
        right_unit: Option<&str>,
        divide: bool,
    ) -> Result<(f64, Option<String>), UnitError> {
        let (left_expr, left_scale) = self.canonicalize_unit_expr(left_unit)?;
        let (right_expr, right_scale) = self.canonicalize_unit_expr(right_unit)?;
        let mut out_expr = left_expr;
        for (symbol, power) in right_expr.exponents {
            out_expr.add_power(symbol, if divide { -power } else { power });
        }
        let value = if divide {
            if right_value == 0.0 {
                return Err(UnitError::UnsupportedConversion);
            }
            (left_value * left_scale) / (right_value * right_scale)
        } else {
            (left_value * left_scale) * (right_value * right_scale)
        };
        let out_unit = format_canonical_unit_expr(&out_expr);
        Ok((value, out_unit))
    }

    fn reduce_to_root(&self, symbol: &str) -> Result<ReducedUnit, UnitError> {
        let mut current = normalize_symbol(symbol);
        if self.conflicted_symbols.contains(&current) {
            return Err(UnitError::AmbiguousMetadata);
        }
        let mut scale = 1.0_f64;
        let mut offset = 0.0_f64;
        let mut guard = HashSet::new();
        loop {
            if !guard.insert(current.clone()) {
                return Err(UnitError::UnsupportedConversion);
            }
            if self.conflicted_symbols.contains(&current) {
                return Err(UnitError::AmbiguousMetadata);
            }
            let Some(def) = self.by_symbol.get(&current) else {
                return Err(UnitError::UnknownUnit);
            };
            if let Some(reference) = def.reference_unit.as_ref() {
                let reference_norm = normalize_symbol(reference);
                let next_scale = scale * def.conversion_factor;
                let next_offset = offset * def.conversion_factor + def.conversion_offset;
                if !self.by_symbol.contains_key(&reference_norm) {
                    return Ok(ReducedUnit {
                        root_symbol: reference_norm,
                        scale: next_scale,
                        offset: next_offset,
                        dimension: def.dimension.clone(),
                    });
                }
                scale = next_scale;
                offset = next_offset;
                current = reference_norm;
                continue;
            }
            return Ok(ReducedUnit {
                root_symbol: current,
                scale,
                offset,
                dimension: def.dimension.clone(),
            });
        }
    }

    fn canonicalize_unit_expr(
        &self,
        raw_unit: Option<&str>,
    ) -> Result<(CanonicalUnitExpr, f64), UnitError> {
        let Some(raw_unit) = raw_unit else {
            return Ok((CanonicalUnitExpr::default(), 1.0));
        };
        let factors = parse_unit_expression(raw_unit)?;
        let mut expr = CanonicalUnitExpr::default();
        let mut scale = 1.0_f64;
        for (symbol, power) in factors {
            if power == 0 {
                continue;
            }
            let reduced = self.reduce_to_root(&symbol)?;
            if reduced.offset != 0.0 {
                return Err(UnitError::UnsupportedConversion);
            }
            scale *= reduced.scale.powi(power);
            let root_factors = parse_unit_expression(&reduced.root_symbol)?;
            for (root_symbol, root_power) in root_factors {
                expr.add_power(root_symbol, root_power * power);
            }
        }
        Ok((expr, scale))
    }

    fn upsert_unit_def(&mut self, def: UnitDef) {
        let key = normalize_symbol(&def.symbol);
        if let Some(existing) = self.by_symbol.get(&key) {
            if existing != &def {
                self.conflicted_symbols.insert(key);
            }
            return;
        }
        self.by_symbol.insert(key, def);
    }

    fn finalize_ingest(&mut self) {
        self.register_well_known_compound_units();
        self.resolve_algebraic_unit_definitions();
        self.derive_si_prefixed_units();
    }

    fn resolve_algebraic_unit_definitions(&mut self) {
        let algebraic: Vec<(String, String, String)> = self
            .by_symbol
            .iter()
            .filter(|(_, def)| {
                def.algebraic_expr.is_some()
                    && def.reference_unit.is_none()
                    && def.algebraic_expr.as_deref() != Some("")
            })
            .map(|(sym, def)| {
                (
                    sym.clone(),
                    def.dimension.clone(),
                    def.algebraic_expr.clone().unwrap_or_default(),
                )
            })
            .collect();
        for (symbol, dimension, expr) in algebraic {
            let Ok((canonical, _scale)) = self.canonicalize_unit_expr(Some(&expr)) else {
                continue;
            };
            let Some(canonical_unit) = format_canonical_unit_expr(&canonical) else {
                continue;
            };
            self.upsert_unit_def(UnitDef {
                symbol,
                dimension,
                reference_unit: Some(canonical_unit),
                conversion_factor: 1.0,
                conversion_offset: 0.0,
                algebraic_expr: Some(expr),
            });
        }
    }

    fn register_well_known_compound_units(&mut self) {
        const COMPOUNDS: &[(&str, &str, &str, &[&str])] = &[
            ("Wh", "EnergyUnit", "W*h", &["W", "h"]),
            ("VA", "ApparentPowerUnit", "V*A", &["V", "A"]),
        ];
        for (symbol, dimension, reference_unit, factors) in COMPOUNDS {
            if self.has_symbol(symbol) {
                continue;
            }
            if !factors.iter().all(|factor| self.has_symbol(factor)) {
                continue;
            }
            self.upsert_unit_def(UnitDef {
                symbol: symbol.to_string(),
                dimension: dimension.to_string(),
                reference_unit: Some(reference_unit.to_string()),
                conversion_factor: 1.0,
                conversion_offset: 0.0,
                algebraic_expr: None,
            });
        }
    }

    fn derive_si_prefixed_units(&mut self) {
        // Only derive from root library units (no referenceUnit). Prefixed units such as
        // `km` already carry a reference and must not seed further prefix combinations.
        let base_symbols: Vec<String> = self
            .by_symbol
            .iter()
            .filter(|(symbol, def)| {
                !self.conflicted_symbols.contains(*symbol)
                    && (def.reference_unit.is_none() || is_prefixable_compound_symbol(symbol))
            })
            .map(|(symbol, _)| symbol.clone())
            .collect();
        let mut prefix_symbols: Vec<(String, f64)> = self
            .prefixes_by_symbol
            .iter()
            .map(|(symbol, factor)| (symbol.clone(), *factor))
            .collect();
        prefix_symbols.sort_by_key(|symbol| std::cmp::Reverse(symbol.0.len()));

        for (prefix_symbol, prefix_factor) in &prefix_symbols {
            for base in &base_symbols {
                let derived = format!("{prefix_symbol}{base}");
                if self.has_symbol(&derived) {
                    continue;
                }
                let Some(base_def) = self.by_symbol.get(base) else {
                    continue;
                };
                self.upsert_unit_def(UnitDef {
                    symbol: derived,
                    dimension: base_def.dimension.clone(),
                    reference_unit: Some(base.clone()),
                    conversion_factor: *prefix_factor,
                    conversion_offset: 0.0,
                    algebraic_expr: None,
                });
            }
        }
    }
}

fn is_prefixable_compound_symbol(symbol: &str) -> bool {
    matches!(symbol, "Wh" | "VA")
}

fn parse_unit_expression(raw: &str) -> Result<Vec<(String, i32)>, UnitError> {
    let cleaned = strip_quotes(raw.trim());
    if cleaned.is_empty() {
        return Err(UnitError::UnsupportedConversion);
    }
    let chars: Vec<char> = cleaned.chars().collect();
    let mut idx = 0usize;
    let mut sign = 1_i32;
    let mut factors = Vec::new();
    while idx < chars.len() {
        while idx < chars.len() && chars[idx].is_whitespace() {
            idx += 1;
        }
        if idx >= chars.len() {
            break;
        }
        if chars[idx] == '*' {
            sign = 1;
            idx += 1;
            continue;
        }
        if chars[idx] == '/' {
            sign = -1;
            idx += 1;
            continue;
        }
        let start = idx;
        while idx < chars.len() && chars[idx] != '*' && chars[idx] != '/' {
            idx += 1;
        }
        let token = chars[start..idx]
            .iter()
            .collect::<String>()
            .trim()
            .to_string();
        if token.is_empty() || token == "1" {
            continue;
        }
        let (symbol_raw, exp_raw) = if let Some(pow_idx) = token.rfind('^') {
            (&token[..pow_idx], Some(&token[pow_idx + 1..]))
        } else {
            (token.as_str(), None)
        };
        let symbol = unit_token_symbol(symbol_raw);
        if symbol.is_empty() || symbol == "1" {
            continue;
        }
        let exponent = exp_raw
            .and_then(|raw_exp| raw_exp.trim().parse::<i32>().ok())
            .unwrap_or(1);
        factors.push((symbol, exponent * sign));
        sign = 1;
    }
    if factors.is_empty() {
        return Err(UnitError::UnsupportedConversion);
    }
    Ok(factors)
}

fn format_canonical_unit_expr(expr: &CanonicalUnitExpr) -> Option<String> {
    if expr.exponents.is_empty() {
        return None;
    }
    let mut numerator = Vec::new();
    let mut denominator = Vec::new();
    for (symbol, exponent) in &expr.exponents {
        if *exponent > 0 {
            numerator.push(if *exponent == 1 {
                symbol.clone()
            } else {
                format!("{symbol}^{exponent}")
            });
        } else if *exponent < 0 {
            let abs = exponent.abs();
            denominator.push(if abs == 1 {
                symbol.clone()
            } else {
                format!("{symbol}^{abs}")
            });
        }
    }
    if denominator.is_empty() {
        return Some(numerator.join("*"));
    }
    let num = if numerator.is_empty() {
        "1".to_string()
    } else {
        numerator.join("*")
    };
    Some(format!("{num}/{}", denominator.join("*")))
}

fn normalize_symbol(value: &str) -> String {
    strip_quotes(value.trim())
}

/// Strips optional package qualification (`SI::s` → `s`).
fn unit_token_symbol(token: &str) -> String {
    let normalized = normalize_symbol(token);
    normalized
        .rsplit_once("::")
        .map(|(_, symbol)| normalize_symbol(symbol))
        .unwrap_or(normalized)
}

fn strip_quotes(value: &str) -> String {
    let mut out = value.trim().to_string();
    if out.len() > 1 {
        if (out.starts_with('\'') && out.ends_with('\''))
            || (out.starts_with('"') && out.ends_with('"'))
        {
            out = out[1..out.len() - 1].to_string();
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use url::Url;

    use crate::semantic::graph_builder::build_graph_from_doc;
    use crate::semantic::relationships::link_workspace_relationships;
    use sysml_v2_parser::parse;

    const SI_PREFIXES: &str = r#"
package SIPrefixes {
    attribute kilo: UnitPrefix { :>> symbol = "k"; :>> conversionFactor = 1E3; }
    attribute mega: UnitPrefix { :>> symbol = "M"; :>> conversionFactor = 1E6; }
    attribute centi: UnitPrefix { :>> symbol = "c"; :>> conversionFactor = 1E-2; }
}
"#;

    fn registry_from_sysml(content: &str) -> UnitRegistry {
        let uri = Url::parse("file:///test/units.sysml").expect("uri");
        let parsed = parse(content).expect("parse");
        let mut graph = build_graph_from_doc(&parsed, &uri);
        link_workspace_relationships(&mut graph);
        UnitRegistry::from_graph(&graph)
    }

    fn with_prefixes(units: &str) -> String {
        format!("{SI_PREFIXES}\npackage Units {{\n{units}\n}}")
    }

    #[test]
    fn km_graph_carries_conversion_metadata() {
        use crate::semantic::graph_builder::unit_metadata::UNIT_CONVERSION_KEY;

        let content = with_prefixes(
            "attribute <m> metre : LengthUnit;\nattribute <km> kilometre : LengthUnit { :>> unitConversion: ConversionByPrefix { :>> prefix = kilo; :>> referenceUnit = m; } }",
        );
        let uri = Url::parse("file:///test/units.sysml").expect("uri");
        let parsed = parse(&content).expect("parse");
        let graph = build_graph_from_doc(&parsed, &uri);
        let km = graph
            .nodes_for_uri(&uri)
            .into_iter()
            .find(|n| n.name == "kilometre")
            .expect("kilometre");
        let conv = km
            .attributes
            .get(UNIT_CONVERSION_KEY)
            .expect("unitConversion attr");
        assert_eq!(
            conv.get("referenceUnit").and_then(|v| v.as_str()),
            Some("m")
        );
    }

    #[test]
    fn derives_kv_from_graph_prefixes() {
        let registry = registry_from_sysml(&with_prefixes(
            "attribute <V> volt : ElectricPotentialUnit;",
        ));
        assert!(registry.prefix_factor_by_name("kilo").is_some());
        assert!(registry.has_symbol("V"), "volt shortName should index as V");
        assert!(
            registry.has_symbol("kV"),
            "derive_si_prefixed_units should add kV"
        );
        assert!(registry.is_recognized_unit_expression("kV"));
    }

    #[test]
    fn parses_conversion_entries_from_graph() {
        let registry = registry_from_sysml(&with_prefixes(
            "attribute <m> metre : LengthUnit;\nattribute <ft> 'foot' : LengthUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m; :>> conversionFactor = 3.048E-01; } }",
        ));
        let def = registry.get("ft").expect("ft def");
        assert_eq!(def.dimension, "LengthUnit");
        assert_eq!(def.reference_unit.as_deref(), Some("m"));
        assert_eq!(def.conversion_factor, 0.3048);
    }

    #[test]
    fn converts_between_compatible_units() {
        let registry = registry_from_sysml(&with_prefixes(
            "attribute <m> 'metre' : LengthUnit;\nattribute <ft> 'foot' : LengthUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m; :>> conversionFactor = 3.048E-01; } }",
        ));
        let converted = registry.convert_value(1.0, "m", "ft").expect("m->ft");
        assert!((converted - 3.280839895).abs() < 1e-6);
    }

    #[test]
    fn rejects_incompatible_dimensions() {
        let registry = registry_from_sysml(&with_prefixes(
            "attribute <m> 'metre' : LengthUnit;\nattribute <kg> 'kilogram' : MassUnit;",
        ));
        let err = registry
            .convert_value(1.0, "m", "kg")
            .expect_err("incompatible");
        assert_eq!(err, UnitError::IncompatibleDimension);
    }

    #[test]
    fn fahrenheit_short_name_materializes_on_graph() {
        use crate::semantic::graph_builder::unit_metadata::{SHORT_NAME_KEY, UNIT_CONVERSION_KEY};

        let content = with_prefixes(&format!(
            "attribute <K> kelvin : TemperatureDifferenceUnit;\nattribute <'\u{00B0}F'> 'degree Fahrenheit' : TemperatureDifferenceUnit {{ :>> unitConversion: ConversionByConvention {{ :>> referenceUnit = K; :>> conversionFactor = 5/9; }} }}"
        ));
        let uri = Url::parse("file:///test/units.sysml").expect("uri");
        let parsed = parse(&content).expect("parse");
        let graph = build_graph_from_doc(&parsed, &uri);
        let f = graph
            .nodes_for_uri(&uri)
            .into_iter()
            .find(|n| n.name == "degree Fahrenheit")
            .expect("degree Fahrenheit node");
        assert_eq!(
            f.attributes.get(SHORT_NAME_KEY).and_then(|v| v.as_str()),
            Some("\u{00B0}F"),
            "attrs: {:?}",
            f.attributes
        );
        let conv = f.attributes.get(UNIT_CONVERSION_KEY).expect("conv");
        assert_eq!(
            conv.get("conversionFactor").and_then(|v| v.as_f64()),
            Some(5.0 / 9.0)
        );
    }

    #[test]
    fn parses_fractional_conversion_factor() {
        let registry = registry_from_sysml(&with_prefixes(&format!(
            "attribute <K> 'kelvin' : TemperatureDifferenceUnit;\nattribute <'\u{00B0}F'> 'degree Fahrenheit' : TemperatureDifferenceUnit {{ :>> unitConversion: ConversionByConvention {{ :>> referenceUnit = K; :>> conversionFactor = 5/9; :>> isExact = true; }} }}"
        )));
        assert!(
            registry.has_symbol("K"),
            "kelvin shortName K should be indexed"
        );
        assert!(
            registry.has_symbol("\u{00B0}F"),
            "degree Fahrenheit shortName should be indexed"
        );
        let c = registry.convert_value(9.0, "\u{00B0}F", "K").expect("F->K");
        assert!((c - 5.0).abs() < 1e-9);
    }

    #[test]
    fn ingests_conversion_by_prefix_units() {
        let registry = registry_from_sysml(&with_prefixes(
            "attribute <m> 'metre' : LengthUnit;\nattribute <km> kilometre : LengthUnit { :>> unitConversion: ConversionByPrefix { :>> prefix = kilo; :>> referenceUnit = m; } }",
        ));
        assert!(registry.is_recognized_unit_expression("km"));
        let def = registry.get("km").expect("km def");
        assert_eq!(def.dimension, "LengthUnit");
        assert_eq!(def.reference_unit.as_deref(), Some("m"));
        assert!((def.conversion_factor - 1E3).abs() < 1e-9);
    }

    #[test]
    fn resolves_qualified_si_unit_literals() {
        let registry = registry_from_sysml(&with_prefixes("attribute <s> second : DurationUnit;"));
        assert!(registry.is_recognized_unit_expression("SI::s"));
    }

    #[test]
    fn derives_engineering_prefixed_units() {
        let registry = registry_from_sysml(&with_prefixes(
            r#"
            attribute <V> volt : ElectricPotentialUnit;
            attribute <W> watt : PowerUnit;
            attribute <A> ampere : ElectricCurrentUnit;
            attribute <h> hour: DurationUnit;
            attribute <s> second : DurationUnit;
            attribute <m> metre : LengthUnit;
            "#,
        ));
        for unit in ["kV", "MW", "MVA", "MWh", "km"] {
            assert!(
                registry.is_recognized_unit_expression(unit),
                "expected derived unit {unit}"
            );
        }
    }

    #[test]
    fn well_known_compound_units_have_explicit_dimensions() {
        let registry = registry_from_sysml(&with_prefixes(
            r#"
            attribute <V> volt : ElectricPotentialUnit;
            attribute <W> watt : PowerUnit;
            attribute <A> ampere : ElectricCurrentUnit;
            attribute <h> hour: DurationUnit;
            attribute <s> second : DurationUnit;
            "#,
        ));

        assert_eq!(
            registry.unit_expression_dimension("Wh").as_deref(),
            Some("EnergyUnit")
        );
        assert_eq!(
            registry.unit_expression_dimension("MWh").as_deref(),
            Some("EnergyUnit")
        );
        assert_eq!(
            registry.unit_expression_dimension("VA").as_deref(),
            Some("ApparentPowerUnit")
        );
        assert_eq!(
            registry.unit_expression_dimension("MVA").as_deref(),
            Some("ApparentPowerUnit")
        );
    }

    #[test]
    fn composite_expression_without_named_unit_has_no_claimed_dimension() {
        let registry = registry_from_sysml(&with_prefixes(
            "attribute <m> 'metre' : LengthUnit;\nattribute <s> second : DurationUnit;",
        ));

        assert!(registry.is_recognized_unit_expression("m/s"));
        assert_eq!(registry.unit_expression_dimension("m/s"), None);
    }

    #[test]
    fn qualified_unit_prefix_type_is_normalized() {
        let registry = registry_from_sysml(
            r#"
            package SI {
                attribute kilo: SI::UnitPrefix { :>> symbol = "k"; :>> conversionFactor = 1E3; }
                attribute <V> volt : SI::ElectricPotentialUnit;
            }
            "#,
        );

        assert!(registry.prefix_factor_by_name("kilo").is_some());
        assert!(registry.is_recognized_unit_expression("kV"));
        assert_eq!(
            registry.unit_expression_dimension("kV").as_deref(),
            Some("ElectricPotentialUnit")
        );
    }

    #[test]
    fn custom_measurement_unit_type_is_discovered_from_graph_ancestry() {
        let registry = registry_from_sysml(
            r#"
            package Measurement {
                attribute def MeasurementUnit;
                attribute def WidgetMeasure :> MeasurementUnit;
                attribute <widget> widget : WidgetMeasure;
            }
            "#,
        );

        assert!(registry.is_recognized_unit_expression("widget"));
        assert_eq!(
            registry.unit_expression_dimension("widget").as_deref(),
            Some("WidgetMeasure")
        );
    }

    #[test]
    fn hover_markdown_for_known_unit_literal() {
        let registry = registry_from_sysml(&with_prefixes(
            "attribute <V> volt : ElectricPotentialUnit;",
        ));
        let md = registry
            .hover_markdown_for_unit_literal("kV")
            .expect("kV hover");
        assert!(md.contains("Unit literal"));
        assert!(md.contains("kV"));
    }

    #[test]
    fn hover_markdown_for_composite_unit_literal() {
        let registry = registry_from_sysml(&with_prefixes(
            "attribute <m> 'metre' : LengthUnit;\nattribute <s> second : DurationUnit;",
        ));
        let md = registry
            .hover_markdown_for_unit_literal("m/s")
            .expect("m/s hover");
        assert!(md.contains("m"));
        assert!(md.contains("s"));
    }

    #[test]
    fn ingests_monetary_units_from_graph() {
        let registry = registry_from_sysml(
            "package MonetaryUnits { attribute <EUR> 'euro' : MonetaryUnit; attribute <USD> 'US dollar' : MonetaryUnit; }",
        );
        assert!(registry.has_symbol("EUR"));
        assert!(registry.has_symbol("USD"));
        assert_eq!(
            registry.get("EUR").map(|def| def.dimension.as_str()),
            Some("MonetaryUnit")
        );
    }

    #[test]
    fn canonicalizes_multiply_and_divide_units() {
        let registry = registry_from_sysml(&with_prefixes(
            "attribute <m> 'metre' : LengthUnit;\nattribute <s> second : DurationUnit;\nattribute <cm> 'centimetre' : LengthUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m; :>> conversionFactor = 1E-02; } }",
        ));
        let (value, unit) = registry
            .compose_product(2.0, Some("cm"), 3.0, Some("m"), false)
            .expect("multiply");
        assert!((value - 0.06).abs() < 1e-9);
        assert_eq!(unit.as_deref(), Some("m^2"));
        let (value, unit) = registry
            .compose_product(10.0, Some("m"), 2.0, Some("s"), true)
            .expect("divide");
        assert!((value - 5.0).abs() < 1e-9);
        assert_eq!(unit.as_deref(), Some("m/s"));
    }

    #[test]
    fn resolves_algebraic_derived_units() {
        let registry = registry_from_sysml(&with_prefixes(
            "attribute <m> metre : LengthUnit;\nattribute <kg> kilogram : MassUnit;\nattribute <s> second : DurationUnit;\nattribute newton : ForceUnit = kg * m / s^2;",
        ));
        assert!(
            registry.is_recognized_unit_expression("newton"),
            "algebraic derived unit should resolve"
        );
    }

    #[test]
    fn custom_unit_definitions_materialize_from_graph() {
        let registry = registry_from_sysml(
            "package CustomMeasurements { attribute <widget> widget : WidgetUnit; }",
        );
        assert!(registry.is_recognized_unit_expression("widget"));
    }

    #[test]
    fn stdlib_path_contains_si_and_imperial_pairs_when_available() {
        let path = Path::new(
            "C:/Git/sysml-v2-release/sysml.library/Domain Libraries/Quantities and Units/USCustomaryUnits.sysml",
        );
        if !path.is_file() {
            return;
        }
        let contents = std::fs::read_to_string(path).expect("read customary units");
        assert!(contents.contains("<ft>") && contents.contains("referenceUnit = m"));
        assert!(contents.contains("<lb>") && contents.contains("referenceUnit = kg"));
        assert!(contents.contains("<lbf>") && contents.contains("referenceUnit = N"));
    }
}
