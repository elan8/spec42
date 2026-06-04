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
        severity: "warning",
        meaning: "A type name on a usage or feature does not resolve to a known definition in the workspace or libraries.",
        typical_fix: "Add or import the missing definition, fix the qualified name, or configure library paths / standard library.",
        editor_quick_fixes: Some(&[
            "create_matching_part_def",
            "create_definition_for_unresolved_type",
        ]),
    },
    DiagnosticCatalogEntry {
        code: "unresolved_ref_type_reference",
        severity: "warning",
        meaning: "A type referenced after `ref` does not resolve.",
        typical_fix: "Ensure the referenced type exists and is visible via imports or namespace.",
        editor_quick_fixes: Some(&["create_definition_for_unresolved_type"]),
    },
    DiagnosticCatalogEntry {
        code: "unresolved_import_target",
        severity: "warning",
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
        severity: "warning",
        meaning: "Connected ports have incompatible port definitions or types.",
        typical_fix: "Use compatible port types or an interface that connects them.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "connection_endpoint_not_port",
        severity: "warning",
        meaning: "A connection endpoint is not a port-like feature.",
        typical_fix: "Connect port usages or adjust the connection statement.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unconnected_port",
        severity: "information",
        meaning: "A port is not connected in the current structural context.",
        typical_fix: "Add a connection or mark the port as intentionally unused.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "duplicate_connection",
        severity: "information",
        meaning: "The same connection appears more than once.",
        typical_fix: "Remove duplicate connect/bind statements.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "invalid_multiplicity",
        severity: "warning",
        meaning: "A multiplicity clause is not valid for the usage.",
        typical_fix: "Fix multiplicity syntax or bounds.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "invalid_redefines_reference",
        severity: "warning",
        meaning: "A redefines target does not resolve or is not redefinable.",
        typical_fix: "Point redefines at an existing inherited or visible feature.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unresolved_pending_relationship",
        severity: "error",
        meaning: "A cross-document relationship could not be resolved after indexing.",
        typical_fix: "Ensure both ends exist, imports are correct, and workspace indexing completed.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unresolved_allocate_source",
        severity: "warning",
        meaning: "The source of an allocate relationship does not resolve.",
        typical_fix: "Use a resolvable usage or definition as the allocate source.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unresolved_allocate_target",
        severity: "warning",
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
        severity: "warning",
        meaning: "A viewpoint conformance target does not resolve.",
        typical_fix: "Import the viewpoint/view package or fix the qualified target name.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "viewpoint_conformance_invalid_target_kind",
        severity: "warning",
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
        code: "unresolved_pending_expression_relationship",
        severity: "error",
        meaning: "An expression-based relationship could not be resolved after graph construction.",
        typical_fix: "Check both endpoint expressions, imports, and qualified feature paths.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "allocation_type_not_allocation_def",
        severity: "warning",
        meaning: "An allocation usage is typed by something other than an allocation definition.",
        typical_fix: "Type the allocation with an allocation definition or remove the incompatible type.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "invalid_allocation_endpoints",
        severity: "warning",
        meaning: "An allocation usage declares only one endpoint of an allocate-to pair.",
        typical_fix: "Declare both source and target endpoints, or remove the incomplete allocate clause.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unresolved_satisfy_source",
        severity: "warning",
        meaning: "The source of a satisfy relationship does not resolve.",
        typical_fix: "Use a resolvable requirement usage or qualified requirement path as the satisfy source.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "unresolved_satisfy_target",
        severity: "warning",
        meaning: "The target of a satisfy relationship does not resolve.",
        typical_fix: "Use a resolvable design feature or qualified feature path after `by`.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "satisfy_endpoint_prefers_usage",
        severity: "warning",
        meaning: "A satisfy endpoint resolves to a definition where a usage or feature path is expected.",
        typical_fix: "Point the endpoint at the concrete usage that satisfies the requirement.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "analysis_evaluation_incomplete",
        severity: "information",
        meaning: "An analysis expression depends on declared values that are not assigned.",
        typical_fix: "Assign the required values or accept that the analysis cannot be fully evaluated yet.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "analysis_evaluation_unresolved",
        severity: "warning",
        meaning: "An analysis expression could not be evaluated.",
        typical_fix: "Check referenced values, operators, and expression syntax supported by Spec42.",
        editor_quick_fixes: None,
    },
    DiagnosticCatalogEntry {
        code: "invalid_verdict_value",
        severity: "warning",
        meaning: "A verdict value is outside the SysML verdict domain.",
        typical_fix: "Use one of: pass, fail, inconclusive, or error.",
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
    use std::collections::HashSet;

    use super::{all_codes, lookup, CATALOG};

    const EMITTED_DIAGNOSTIC_CODES: &[(&str, &str)] = &[
        ("allocate_endpoint_prefers_usage", "warning"),
        ("allocation_type_not_allocation_def", "warning"),
        ("analysis_constraint_failed", "warning"),
        ("analysis_evaluation_incomplete", "information"),
        ("analysis_evaluation_unresolved", "warning"),
        ("connection_endpoint_not_port", "warning"),
        ("duplicate_connection", "information"),
        ("implicit_redefinition_without_operator", "error"),
        ("inherited_attribute_value_type_mismatch", "error"),
        ("invalid_allocation_endpoints", "warning"),
        ("invalid_multiplicity", "warning"),
        ("invalid_redefines_reference", "warning"),
        ("invalid_verdict_value", "warning"),
        ("missing_library_context", "information"),
        ("objective_binding_unresolved", "warning"),
        ("port_type_mismatch", "warning"),
        ("satisfy_endpoint_prefers_usage", "warning"),
        ("semantic_diagnostic", "warning"),
        ("unconnected_port", "information"),
        ("unresolved_allocate_source", "warning"),
        ("unresolved_allocate_target", "warning"),
        ("unresolved_import_target", "warning"),
        ("unresolved_pending_expression_relationship", "error"),
        ("unresolved_pending_relationship", "error"),
        ("unresolved_ref_type_reference", "warning"),
        ("unresolved_satisfy_source", "warning"),
        ("unresolved_satisfy_target", "warning"),
        ("unresolved_specializes_reference", "warning"),
        ("unresolved_type_reference", "warning"),
        ("unresolved_viewpoint_conformance_target", "warning"),
        ("untyped_part_usage", "information"),
        ("viewpoint_conformance_invalid_target_kind", "warning"),
    ];

    #[test]
    fn lookup_returns_entry_for_known_code() {
        let entry = lookup("unresolved_type_reference").expect("catalog entry");
        assert_eq!(entry.code, "unresolved_type_reference");
        assert_eq!(entry.severity, "warning");
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

    #[test]
    fn catalog_codes_are_unique() {
        let mut seen = HashSet::new();
        for entry in CATALOG {
            assert!(
                seen.insert(entry.code),
                "duplicate catalog code {}",
                entry.code
            );
        }
    }

    #[test]
    fn catalog_covers_known_emitted_diagnostics_with_current_severity() {
        for (code, expected_severity) in EMITTED_DIAGNOSTIC_CODES {
            let entry = lookup(code).unwrap_or_else(|| panic!("missing catalog entry for {code}"));
            assert_eq!(
                entry.severity, *expected_severity,
                "catalog severity for {code} must match current emitted severity"
            );
        }
    }
}
