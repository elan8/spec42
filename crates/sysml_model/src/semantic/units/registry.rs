use std::collections::{BTreeMap, HashMap, HashSet};

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
    /// Records one unit prefix and the factor it scales its reference unit by.
    pub fn ingest_unit_prefix(&mut self, name: &str, symbol: Option<&str>, factor: f64) {
        self.prefixes_by_name.insert(name.to_string(), factor);
        if let Some(symbol) = symbol {
            self.prefixes_by_symbol.insert(symbol.to_string(), factor);
        }
    }

    pub fn prefix_factor_by_name(&self, prefix_name: &str) -> Option<f64> {
        self.prefixes_by_name.get(prefix_name).copied().or_else(|| {
            prefix_name
                .rsplit_once("::")
                .and_then(|(_, base)| self.prefixes_by_name.get(base).copied())
        })
    }

    /// Records one unit definition.
    pub fn ingest_unit_def(&mut self, def: UnitDef) {
        self.upsert_unit_def(def);
    }

    pub fn get(&self, symbol: &str) -> Option<&UnitDef> {
        self.by_symbol.get(&normalize_symbol(symbol))
    }

    pub fn has_symbol(&self, symbol: &str) -> bool {
        self.by_symbol.contains_key(&normalize_symbol(symbol))
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

    /// Resolves algebraic definitions once every unit has been ingested.
    pub fn finalize_ingest(&mut self) {
        self.resolve_algebraic_unit_definitions();
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
    if out.len() > 1
        && ((out.starts_with('\'') && out.ends_with('\''))
            || (out.starts_with('"') && out.ends_with('"')))
    {
        out = out[1..out.len() - 1].to_string();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The catalog is populated by its owner, not ingested from a graph, so the tests build one
    /// the same way the evaluator would.
    fn unit(
        symbol: &str,
        dimension: &str,
        reference: Option<&str>,
        factor: f64,
        offset: f64,
    ) -> UnitDef {
        UnitDef {
            symbol: symbol.to_string(),
            dimension: dimension.to_string(),
            reference_unit: reference.map(str::to_string),
            conversion_factor: factor,
            conversion_offset: offset,
            algebraic_expr: None,
        }
    }

    fn catalog() -> UnitRegistry {
        let mut registry = UnitRegistry::default();
        registry.ingest_unit_def(unit("m", "LengthUnit", None, 1.0, 0.0));
        registry.ingest_unit_def(unit("km", "LengthUnit", Some("m"), 1000.0, 0.0));
        registry.ingest_unit_def(unit("s", "DurationUnit", None, 1.0, 0.0));
        registry.ingest_unit_def(unit("K", "ThermodynamicTemperatureUnit", None, 1.0, 0.0));
        registry.ingest_unit_def(unit(
            "degC",
            "ThermodynamicTemperatureUnit",
            Some("K"),
            1.0,
            273.15,
        ));
        registry.finalize_ingest();
        registry
    }

    #[test]
    fn converts_between_units_of_one_dimension() {
        let registry = catalog();
        assert_eq!(registry.convert_value(2.0, "km", "m"), Ok(2000.0));
        assert_eq!(registry.convert_value(2000.0, "m", "km"), Ok(2.0));
    }

    /// A conversion across dimensions is an explicit error, never a silently rescaled number.
    #[test]
    fn rejects_a_conversion_across_dimensions() {
        assert_eq!(
            catalog().convert_value(1.0, "m", "s"),
            Err(UnitError::IncompatibleDimension)
        );
    }

    /// An affine scale carries an offset, so its conversion is not a bare multiplication.
    #[test]
    fn converts_an_affine_scale_through_its_offset() {
        let registry = catalog();
        let kelvin = registry.convert_value(0.0, "degC", "K").expect("convert");
        assert!((kelvin - 273.15).abs() < 1e-9, "got {kelvin}");
    }

    /// A symbol no unit was ingested for is unknown, which is distinct from a wrong dimension.
    #[test]
    fn an_uningested_symbol_is_unknown() {
        assert_eq!(
            catalog().convert_value(1.0, "furlong", "m"),
            Err(UnitError::UnknownUnit)
        );
    }

    #[test]
    fn a_product_composes_the_units_of_both_operands() {
        let registry = catalog();
        let (value, unit) = registry
            .compose_product(3.0, Some("m"), 2.0, Some("s"), false)
            .expect("compose");
        assert_eq!(value, 6.0);
        assert_eq!(unit.as_deref(), Some("m*s"));
    }

    #[test]
    fn a_quotient_of_one_unit_by_itself_is_dimensionless() {
        let registry = catalog();
        let (_, unit) = registry
            .compose_product(6.0, Some("m"), 2.0, Some("m"), true)
            .expect("compose");
        assert_eq!(unit, None);
    }
}
