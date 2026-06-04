use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct DiagnosticCatalogEntry {
    pub code: &'static str,
    pub severity: &'static str,
    pub meaning: &'static str,
    pub typical_fix: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editor_quick_fixes: Option<&'static [&'static str]>,
}

const CATALOG: &[DiagnosticCatalogEntry] = &[
    DiagnosticCatalogEntry {
        code: "unresolved_type_reference",
        severity: "error",
        meaning: "A type name on a usage or feature does not resolve to a known definition in the workspace or libraries.",
        typical_fix: "Add or import the missing definition, fix the qualified name, or configure library paths / standard library.",
        editor_quick_fixes: Some(&[
            "create_matching_part_def",
            "create_definition_for_unresolved_type",
        ]),
    },
    DiagnosticCatalogEntry {
        code: "unresolved_ref_type_reference",
        severity: "error",
        meaning: "A type referenced after `ref` does not resolve.",
        typical_fix: "Ensure the referenced type exists and is visible via imports or namespace.",
        editor_quick_fixes: Some(&["create_definition_for_unresolved_type"]),
    },
    DiagnosticCatalogEntry {
        code: "unresolved_import_target",
        severity: "error",
        meaning: "An import statement targets a package or namespace that cannot be found.",
        typical_fix: "Fix the import path, add the defining file to the workspace, or index the library root.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unresolved_specializes_reference",
        severity: "warning",
        meaning: "A specializes target does not resolve to a known definition.",
        typical_fix: "Correct the specializes clause or add the base definition.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "implicit_redefinition_without_operator",
        severity: "error",
        meaning: "An inherited feature is redefined without an explicit redefinition operator (`:>>` / `redefines`).",
        typical_fix: "Add an explicit redefines clause on the redefining feature.",
        editor_quick_fixes: Some(&["explicit_redefinition_quick_fix"]),
    },
    DiagnosticCatalogEntry {
        code: "inherited_attribute_value_type_mismatch",
        severity: "error",
        meaning: "A redefining attribute value is not compatible with the inherited attribute typing.",
        typical_fix: "Align value expression type with the inherited attribute or adjust typing.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "untyped_part_usage",
        severity: "information",
        meaning: "A part usage has no typing clause (`: Type`).",
        typical_fix: "Add a type if the usage should be typed; otherwise this may be intentional.",
        editor_quick_fixes: Some(&["create_matching_part_def"]),
    },
    DiagnosticCatalogEntry {
        code: "missing_library_context",
        severity: "information",
        meaning: "The document imports library symbols but no SysML library paths are configured.",
        typical_fix: "Configure spec42.libraryPaths / --library-path or install the standard library.",
        editor_quick_fixes: Some(&[
            "manage_custom_libraries",
            "show_standard_library_info",
        ]),
    },
    DiagnosticCatalogEntry {
        code: "port_type_mismatch",
        severity: "error",
        meaning: "Connected ports have incompatible port definitions or types.",
        typical_fix: "Use compatible port types or an interface that connects them.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "connection_endpoint_not_port",
        severity: "error",
        meaning: "A connection endpoint is not a port-like feature.",
        typical_fix: "Connect port usages or adjust the connection statement.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unconnected_port",
        severity: "warning",
        meaning: "A port is not connected in the current structural context.",
        typical_fix: "Add a connection or mark the port as intentionally unused.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "duplicate_connection",
        severity: "warning",
        meaning: "The same connection appears more than once.",
        typical_fix: "Remove duplicate connect/bind statements.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "invalid_multiplicity",
        severity: "error",
        meaning: "A multiplicity clause is not valid for the usage.",
        typical_fix: "Fix multiplicity syntax or bounds.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "invalid_redefines_reference",
        severity: "error",
        meaning: "A redefines target does not resolve or is not redefinable.",
        typical_fix: "Point redefines at an existing inherited or visible feature.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unresolved_pending_relationship",
        severity: "warning",
        meaning: "A cross-document relationship could not be resolved after indexing.",
        typical_fix: "Ensure both ends exist, imports are correct, and workspace indexing completed.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unresolved_allocate_source",
        severity: "error",
        meaning: "The source of an allocate relationship does not resolve.",
        typical_fix: "Use a resolvable usage or definition as the allocate source.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unresolved_allocate_target",
        severity: "error",
        meaning: "The target of an allocate relationship does not resolve.",
        typical_fix: "Use a resolvable usage or definition as the allocate target.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "allocate_endpoint_prefers_usage",
        severity: "warning",
        meaning: "Allocate should reference a usage rather than a bare definition.",
        typical_fix: "Point the allocate end at a part usage (or other usage) instead of a definition.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unresolved_viewpoint_conformance_target",
        severity: "error",
        meaning: "A viewpoint conformance target does not resolve.",
        typical_fix: "Import the viewpoint/view package or fix the qualified target name.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "viewpoint_conformance_invalid_target_kind",
        severity: "error",
        meaning: "The target of viewpoint conformance is not a viewpoint element.",
        typical_fix: "Reference a viewpoint definition or usage as required by the conformance statement.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "analysis_constraint_failed",
        severity: "warning",
        meaning: "An analysis constraint evaluated to false.",
        typical_fix: "Adjust the model or constraint expression so the analysis passes.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "objective_binding_unresolved",
        severity: "warning",
        meaning: "An analysis or verification objective binding could not be resolved.",
        typical_fix: "Ensure subject/objective identifiers match defined case elements.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "semantic_diagnostic",
        severity: "warning",
        meaning: "A semantic rule reported by the graph builder (see message for detail).",
        typical_fix: "Follow the diagnostic message; use spec42_check for exact range.",
        editor_quick_fixes: None,
    },
];

pub fn lookup(code: &str) -> Option<&'static DiagnosticCatalogEntry> {
    CATALOG.iter().find(|entry| entry.code == code)
}

pub fn all_codes() -> Vec<&'static str> {
    CATALOG.iter().map(|e| e.code).collect()
}

#[cfg(test)]
mod tests {
    use super::{all_codes, lookup};

    #[test]
    fn lookup_returns_entry_for_known_code() {
        let entry = lookup("unresolved_type_reference").expect("catalog entry");
        assert_eq!(entry.code, "unresolved_type_reference");
        assert_eq!(entry.severity, "error");
    }

    #[test]
    fn lookup_returns_none_for_unknown_code() {
        assert!(lookup("not_a_real_diagnostic_code").is_none());
    }

    #[test]
    fn all_codes_includes_common_semantic_codes() {
        let codes = all_codes();
        assert!(codes.contains(&"unresolved_type_reference"));
        assert!(codes.contains(&"missing_library_context"));
    }
}
