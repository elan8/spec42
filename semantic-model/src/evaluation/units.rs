use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use tower_lsp::lsp_types::Url;

use crate::graph::SemanticGraph;

#[derive(Debug, Clone, PartialEq)]
pub struct UnitDef {
    pub symbol: String,
    pub dimension: String,
    pub reference_unit: Option<String>,
    pub conversion_factor: f64,
}

#[derive(Debug, Clone, Default)]
pub struct UnitRegistry {
    by_symbol: HashMap<String, UnitDef>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitError {
    UnknownUnit,
    IncompatibleDimension,
    UnsupportedConversion,
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
        let (from_root, from_scale, from_dim) = self.reduce_to_root(&from_norm)?;
        let (to_root, to_scale, to_dim) = self.reduce_to_root(&to_norm)?;
        if from_root != to_root || from_dim != to_dim {
            return Err(UnitError::IncompatibleDimension);
        }
        Ok(value * from_scale / to_scale)
    }

    fn reduce_to_root(&self, symbol: &str) -> Result<(String, f64, String), UnitError> {
        let mut current = normalize_symbol(symbol);
        let mut scale = 1.0_f64;
        let mut guard = HashSet::new();
        loop {
            if !guard.insert(current.clone()) {
                return Err(UnitError::UnsupportedConversion);
            }
            let Some(def) = self.by_symbol.get(&current) else {
                return Err(UnitError::UnknownUnit);
            };
            if let Some(reference) = def.reference_unit.as_ref() {
                let reference_norm = normalize_symbol(reference);
                // If reference points to an expression (e.g. m^2), we can't chain through
                // symbol-to-symbol conversion yet; treat as canonical endpoint.
                if !self.by_symbol.contains_key(&reference_norm) {
                    return Ok((reference_norm, scale * def.conversion_factor, def.dimension.clone()));
                }
                scale *= def.conversion_factor;
                current = reference_norm;
                continue;
            }
            return Ok((current, scale, def.dimension.clone()));
        }
    }

    fn ingest_file_contents(&mut self, contents: &str) {
        for line in contents.lines() {
            let trimmed = line.trim();
            if !trimmed.starts_with("attribute ") {
                continue;
            }
            let Some(symbol) = extract_symbol(trimmed) else {
                continue;
            };
            let Some(dimension) = extract_dimension(trimmed) else {
                continue;
            };
            let reference_unit = extract_assignment(trimmed, "referenceUnit");
            let conversion_factor = extract_assignment(trimmed, "conversionFactor")
                .and_then(|raw| parse_factor_expression(&raw))
                .unwrap_or(1.0);
            let def = UnitDef {
                symbol: symbol.clone(),
                dimension,
                reference_unit,
                conversion_factor,
            };
            self.by_symbol
                .entry(normalize_symbol(&symbol))
                .or_insert(def);
        }
    }
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
    let end = after.find(|ch: char| ch == '{' || ch == '=' || ch == ';').unwrap_or(after.len());
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
    fn stdlib_path_contains_si_and_imperial_pairs_when_available() {
        let path = Path::new("C:/Git/sysml-v2-release/sysml.library/Domain Libraries/Quantities and Units/USCustomaryUnits.sysml");
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
