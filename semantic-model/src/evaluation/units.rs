use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::PathBuf;

use tower_lsp::lsp_types::Url;

use crate::graph::SemanticGraph;

#[derive(Debug, Clone, PartialEq)]
pub struct UnitDef {
    pub symbol: String,
    pub dimension: String,
    pub reference_unit: Option<String>,
    pub conversion_factor: f64,
    pub conversion_offset: f64,
}

#[derive(Debug, Clone, Default)]
pub struct UnitRegistry {
    by_symbol: HashMap<String, UnitDef>,
    conflicted_symbols: HashSet<String>,
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
    pub fn from_semantic_graph(graph: &SemanticGraph) -> Self {
        let mut registry = UnitRegistry::default();
        let mut candidate_files = HashSet::new();
        for uri in graph.nodes_by_uri.keys() {
            if let Some(path) = uri_to_path(uri) {
                if !path.to_string_lossy().contains("Quantities and Units") {
                    continue;
                }
                if path
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("sysml"))
                {
                    candidate_files.insert(path);
                }
            }
        }
        for path in candidate_files {
            if let Ok(contents) = std::fs::read_to_string(&path) {
                registry.ingest_file_contents(&contents);
            }
        }
        registry
    }

    #[cfg(test)]
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

    fn ingest_file_contents(&mut self, contents: &str) {
        let lines: Vec<&str> = contents.lines().collect();
        let mut idx = 0usize;
        while idx < lines.len() {
            let trimmed = lines[idx].trim();
            if !trimmed.starts_with("attribute ") {
                idx += 1;
                continue;
            }
            if trimmed.contains(": IntervalScale {") {
                idx = self.ingest_interval_scale_block(&lines, idx);
                continue;
            }
            if trimmed.contains('{') {
                let mut depth = brace_delta(trimmed);
                let mut end = idx + 1;
                while end < lines.len() && depth > 0 {
                    depth += brace_delta(lines[end].trim());
                    end += 1;
                }
                let block = lines[idx..end]
                    .iter()
                    .map(|line| line.trim())
                    .collect::<Vec<_>>()
                    .join(" ");
                if let Some(def) = parse_linear_unit_def(&block) {
                    self.upsert_unit_def(def);
                }
                idx = end;
                continue;
            }
            if let Some(def) = parse_linear_unit_def(trimmed) {
                self.upsert_unit_def(def);
            }
            idx += 1;
        }
    }

    fn ingest_interval_scale_block(&mut self, lines: &[&str], start: usize) -> usize {
        let header = lines[start].trim();
        let Some(scale_symbol) = extract_symbol(header) else {
            return start + 1;
        };
        let mut depth = brace_delta(header);
        let mut idx = start + 1;
        let mut unit_symbol: Option<String> = None;
        let mut zero_offset_kelvin: Option<f64> = None;
        while idx < lines.len() && depth > 0 {
            let line = lines[idx].trim();
            if line.contains(":>> unit =") {
                unit_symbol = extract_assignment(line, "unit");
            }
            if line.contains("zeroDegree") && line.contains('=') {
                zero_offset_kelvin = parse_zero_point_kelvin(line);
            }
            depth += brace_delta(line);
            idx += 1;
        }

        let Some(zero_kelvin) = zero_offset_kelvin else {
            return idx;
        };
        let unit_for_scale = unit_symbol.unwrap_or_else(|| scale_symbol.clone());
        let base_scale = self
            .by_symbol
            .get(&normalize_symbol(&unit_for_scale))
            .map(|unit| unit.conversion_factor)
            .unwrap_or(1.0);
        let abs_def = UnitDef {
            symbol: scale_symbol,
            dimension: "ThermodynamicTemperatureUnit".to_string(),
            reference_unit: Some("K".to_string()),
            conversion_factor: base_scale,
            conversion_offset: zero_kelvin,
        };
        self.upsert_unit_def(abs_def);
        idx
    }
}

fn parse_linear_unit_def(line: &str) -> Option<UnitDef> {
    let symbol = extract_symbol(line)?;
    let dimension = extract_dimension(line)?;
    let reference_unit = extract_assignment(line, "referenceUnit");
    let conversion_factor = extract_assignment(line, "conversionFactor")
        .and_then(|raw| parse_factor_expression(&raw))
        .unwrap_or(1.0);
    Some(UnitDef {
        symbol,
        dimension,
        reference_unit,
        conversion_factor,
        conversion_offset: 0.0,
    })
}

fn uri_to_path(uri: &Url) -> Option<PathBuf> {
    if uri.scheme() != "file" {
        return None;
    }
    uri.to_file_path().ok()
}

fn extract_symbol(line: &str) -> Option<String> {
    let start = line.find('<')?;
    let end = line[start + 1..].find('>')?;
    let raw = &line[start + 1..start + 1 + end];
    Some(strip_quotes(raw.trim()))
}

fn extract_dimension(line: &str) -> Option<String> {
    let colon = line.find(':')?;
    let after = line[colon + 1..].trim_start();
    let end = after.find(['{', '=', ';']).unwrap_or(after.len());
    let dim = after[..end].trim();
    if dim.is_empty() {
        None
    } else {
        Some(strip_quotes(dim))
    }
}

fn extract_assignment(line: &str, key: &str) -> Option<String> {
    let needle = format!("{key} =");
    let idx = line.find(&needle)?;
    let value_start = idx + needle.len();
    let rest = line[value_start..].trim_start();
    let end = rest.find(';').unwrap_or(rest.len());
    let value = rest[..end].trim();
    if value.is_empty() {
        None
    } else {
        Some(strip_quotes(value))
    }
}

fn parse_zero_point_kelvin(line: &str) -> Option<f64> {
    let equals = line.find('=')?;
    let bracket = line[equals + 1..].find('[')?;
    let raw = line[equals + 1..equals + 1 + bracket].trim();
    parse_factor_expression(raw)
}

fn brace_delta(line: &str) -> i32 {
    line.chars().fold(0_i32, |acc, ch| {
        if ch == '{' {
            acc + 1
        } else if ch == '}' {
            acc - 1
        } else {
            acc
        }
    })
}

fn parse_factor_expression(raw: &str) -> Option<f64> {
    let trimmed = raw.trim();
    if let Some(div) = trimmed.find('/') {
        let left = trimmed[..div].trim().parse::<f64>().ok()?;
        let right = trimmed[div + 1..].trim().parse::<f64>().ok()?;
        if right == 0.0 {
            return None;
        }
        return Some(left / right);
    }
    trimmed.parse::<f64>().ok()
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
        let symbol = normalize_symbol(symbol_raw);
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

fn strip_quotes(value: &str) -> String {
    let mut out = value.trim().to_string();
    if out.starts_with('\'') && out.ends_with('\'') && out.len() > 1 {
        out = out[1..out.len() - 1].to_string();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn parses_conversion_entries_from_line() {
        let mut registry = UnitRegistry::default();
        registry.ingest_file_contents(
            "attribute <ft> 'foot' : LengthUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m; :>> conversionFactor = 3.048E-01; } }",
        );
        let def = registry.get("ft").expect("ft def");
        assert_eq!(def.dimension, "LengthUnit");
        assert_eq!(def.reference_unit.as_deref(), Some("m"));
        assert_eq!(def.conversion_factor, 0.3048);
    }

    #[test]
    fn converts_between_compatible_units() {
        let mut registry = UnitRegistry::default();
        registry.ingest_file_contents("attribute <m> 'metre' : LengthUnit;");
        registry.ingest_file_contents(
            "attribute <ft> 'foot' : LengthUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m; :>> conversionFactor = 3.048E-01; } }",
        );
        let converted = registry.convert_value(1.0, "m", "ft").expect("m->ft");
        assert!((converted - 3.280839895).abs() < 1e-6);
    }

    #[test]
    fn rejects_incompatible_dimensions() {
        let mut registry = UnitRegistry::default();
        registry.ingest_file_contents("attribute <m> 'metre' : LengthUnit;");
        registry.ingest_file_contents("attribute <kg> 'kilogram' : MassUnit;");
        let err = registry.convert_value(1.0, "m", "kg").expect_err("incompatible");
        assert_eq!(err, UnitError::IncompatibleDimension);
    }

    #[test]
    fn parses_fractional_conversion_factor() {
        let mut registry = UnitRegistry::default();
        registry.ingest_file_contents("attribute <K> 'kelvin' : TemperatureDifferenceUnit;");
        registry.ingest_file_contents(
            "attribute <'°F'> 'degree Fahrenheit' : TemperatureDifferenceUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = K; :>> conversionFactor = 5/9; :>> isExact = true; } }",
        );
        let c = registry.convert_value(9.0, "°F", "K").expect("F->K");
        assert!((c - 5.0).abs() < 1e-9);
    }

    #[test]
    fn parses_affine_interval_scales_and_converts_absolute_temperatures() {
        let mut registry = UnitRegistry::default();
        registry.ingest_file_contents(
            r#"
            attribute <K> kelvin : ThermodynamicTemperatureUnit, TemperatureDifferenceUnit;
            attribute <'°C'> 'degree celsius (temperature difference)' : TemperatureDifferenceUnit {
                attribute :>> unitConversion: ConversionByConvention { :>> referenceUnit = K; :>> conversionFactor = 1; }
            }
            attribute <'°F'> 'degree Fahrenheit (temperature difference)' : TemperatureDifferenceUnit {
                :>> unitConversion: ConversionByConvention { :>> referenceUnit = K; :>> conversionFactor = 5/9; :>> isExact = true; }
            }
            attribute <'°C_abs'> 'degree celsius (absolute temperature scale)' : IntervalScale {
                attribute :>> unit = '°C';
                private attribute zeroDegreeCelsiusInKelvin: ThermodynamicTemperatureValue = 273.15 [K];
            }
            attribute <'°F_abs'> 'degree fahrenheit (absolute temperature scale)' : IntervalScale {
                :>> unit = '°F';
                private attribute zeroDegreeFahrenheitInKelvin: ThermodynamicTemperatureValue = 229835/900 [K];
            }
            "#,
        );
        let value = registry
            .convert_value(32.0, "°F_abs", "°C_abs")
            .expect("absolute conversion");
        assert!(
            (value - 0.0).abs() < 1e-6,
            "expected 32°F_abs to map to 0°C_abs, got {value}"
        );
    }

    #[test]
    fn canonicalizes_multiply_and_divide_units() {
        let mut registry = UnitRegistry::default();
        registry.ingest_file_contents("attribute <m> 'metre' : LengthUnit;");
        registry.ingest_file_contents("attribute <s> second : TimeUnit;");
        registry.ingest_file_contents(
            "attribute <cm> 'centimetre' : LengthUnit { :>> unitConversion: ConversionByConvention { :>> referenceUnit = m; :>> conversionFactor = 1E-02; } }",
        );
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
    fn rejects_affine_units_in_multiply_divide() {
        let mut registry = UnitRegistry::default();
        registry.ingest_file_contents(
            r#"
            attribute <K> kelvin : ThermodynamicTemperatureUnit, TemperatureDifferenceUnit;
            attribute <'°C'> 'degree celsius (temperature difference)' : TemperatureDifferenceUnit {
                attribute :>> unitConversion: ConversionByConvention { :>> referenceUnit = K; :>> conversionFactor = 1; }
            }
            attribute <'°C_abs'> 'degree celsius (absolute temperature scale)' : IntervalScale {
                attribute :>> unit = '°C';
                private attribute zeroDegreeCelsiusInKelvin: ThermodynamicTemperatureValue = 273.15 [K];
            }
            "#,
        );
        let err = registry
            .compose_product(1.0, Some("°C_abs"), 2.0, Some("m"), false)
            .expect_err("affine in product");
        assert_eq!(err, UnitError::UnsupportedConversion);
    }

    #[test]
    fn stdlib_path_contains_si_and_imperial_pairs_when_available() {
        let path = Path::new(
            "C:/Git/sysml-v2-release/sysml.library/Domain Libraries/Quantities and Units/USCustomaryUnits.sysml",
        );
        if !path.is_file() {
            // Developer machine may not have the SysML-v2 release checkout.
            return;
        }
        let contents = std::fs::read_to_string(path).expect("read customary units");
        assert!(contents.contains("<ft>") && contents.contains("referenceUnit = m"));
        assert!(contents.contains("<lb>") && contents.contains("referenceUnit = kg"));
        assert!(contents.contains("<lbf>") && contents.contains("referenceUnit = N"));
    }
}
