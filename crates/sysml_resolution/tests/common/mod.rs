//! Shared fixtures for the relocated contract tests.
//!
//! Every helper here moved verbatim out of the inline `#[cfg(test)]` modules of
//! `src/lib.rs` and `src/model.rs`; each one drives the crate through its public surface.

#![allow(dead_code)]

use std::fmt;

use sysml_resolution::*;

pub fn build_semantic_sexpr(source: &str) -> String {
    let request = sysml_resolution::BuildRequest::new(
        vec![sysml_resolution::SourceInput::new(
            "memory://test/enum.sysml",
            source.to_string(),
            sysml_resolution::SourceKind::Workspace,
        )],
        sysml_resolution::ConstructionSchedule::Sequential,
        "test-contract-v1",
    )
    .unwrap();
    let published = sysml_resolution::build(request).unwrap();
    let mut output = String::new();
    published.debug().write_semantic_sexpr(&mut output).unwrap();
    output
}

pub fn build_diagnostics_sexpr(source: &str) -> String {
    let request = sysml_resolution::BuildRequest::new(
        vec![sysml_resolution::SourceInput::new(
            "memory://test/enum.sysml",
            source.to_string(),
            sysml_resolution::SourceKind::Workspace,
        )],
        sysml_resolution::ConstructionSchedule::Sequential,
        "test-contract-v1",
    )
    .unwrap();
    let published = sysml_resolution::build(request).unwrap();
    let mut output = String::new();
    published
        .debug()
        .write_diagnostics_sexpr(&mut output)
        .unwrap();
    output
}

pub fn semantic_sexpr_for(source: &str) -> String {
    let request = BuildRequest::new(
        vec![SourceInput::new(
            "memory://test.sysml",
            source.to_string(),
            SourceKind::Workspace,
        )],
        ConstructionSchedule::Sequential,
        "contract-v1",
    )
    .unwrap();
    let published = build(request).unwrap();
    let mut output = String::new();
    published.debug().write_semantic_sexpr(&mut output).unwrap();
    output
}

/// Like `semantic_sexpr_for`, but renders the per-document diagnostics sexpr (which carries
/// the actual `unsupported_*_definition_member` diagnostic codes) instead of the semantic
/// model sexpr (which only carries the coarser `(completeness unsupported-syntax)` summary
/// flag) -- needed for tests asserting a *specific* diagnostic code is present, not merely
/// that publication completeness is degraded.
pub fn diagnostics_sexpr_for(source: &str) -> String {
    let request = BuildRequest::new(
        vec![SourceInput::new(
            "memory://test.sysml",
            source.to_string(),
            SourceKind::Workspace,
        )],
        ConstructionSchedule::Sequential,
        "contract-v1",
    )
    .unwrap();
    let published = build(request).unwrap();
    let mut output = String::new();
    published
        .debug()
        .write_diagnostics_sexpr(&mut output)
        .unwrap();
    output
}

/// The typed diagnostics of one single-document publication.
pub fn diagnostics_for(source: &str) -> Vec<Diagnostic> {
    published_for(source).diagnostics().diagnostics.into_vec()
}

pub fn published_for(source: &str) -> PublishedResolution {
    let request = BuildRequest::new(
        vec![SourceInput::new(
            "memory://test.sysml",
            source.to_string(),
            SourceKind::Workspace,
        )],
        ConstructionSchedule::Sequential,
        "contract-v1",
    )
    .unwrap();
    build(request).unwrap()
}

// --- Canonical element identity ---------------------------------------------------------

pub fn publication_for(sources: &[(&str, &str)]) -> PublishedResolution {
    let request = BuildRequest::new(
        sources
            .iter()
            .map(|(identity, source)| {
                SourceInput::new(*identity, source.to_string(), SourceKind::Workspace)
            })
            .collect(),
        ConstructionSchedule::Sequential,
        "contract-v1",
    )
    .unwrap();
    build(request).unwrap()
}

pub fn target_symbol(
    published: &PublishedResolution,
    document: &str,
    line: u32,
    character: u32,
) -> SymbolIdentity {
    match published.target_at(document, TextPosition { line, character }) {
        QueryOutcome::Resolved(target) => target.symbol,
        other => panic!("expected a resolved navigation target, got: {other:?}"),
    }
}

// --- Element inspection -----------------------------------------------------------------

pub fn inspect_named(
    published: &PublishedResolution,
    document: &str,
    line: u32,
    character: u32,
) -> ElementInspection {
    let symbol = target_symbol(published, document, line, character);
    match published.inspect(&symbol) {
        QueryOutcome::Resolved(inspection) => inspection,
        other => panic!("expected a resolved inspection, got: {other:?}"),
    }
}

pub fn position_of(source: &str, needle: &str) -> TextPosition {
    let (line, character) = source
        .lines()
        .enumerate()
        .find_map(|(line, text)| text.find(needle).map(|column| (line, column)))
        .unwrap_or_else(|| panic!("{needle:?} does not occur in the fixture"));
    TextPosition {
        line: u32::try_from(line).expect("fixture line fits"),
        character: u32::try_from(character).expect("fixture column fits"),
    }
}

/// The probed document, unchanged across every variant below.
pub const PROBED: &str = "package P {\n  part def Wheel;\n  part w : Wheel;\n}";

/// Declarations that are cheap to write and land in every published fact table -- a name, a
/// documentation record, a reference and an evaluated value -- so that a scan of any of them
/// shows up in the measurement.
pub fn padding(members: usize) -> String {
    (0..members)
        .map(|index| {
            format!(
                "  part def Pad{index} {{ doc /* pad */ }}\n                       part padUse{index} : Pad{index};\n                       attribute padValue{index} = {index} + 1;\n"
            )
        })
        .collect()
}

/// The identity of the declaration containing `needle`, and the publication it belongs to.
pub fn probe_symbol(
    published: &PublishedResolution,
    source: &str,
    document: &str,
    needle: &str,
) -> SymbolIdentity {
    match published.inspect_at(document, position_of(source, needle)) {
        QueryOutcome::Resolved(at) => {
            at.containing
                .expect("the probe must land inside a declaration")
                .identity
        }
        other => panic!("the probe must resolve to an inspection, got: {other:?}"),
    }
}

// --- Type queries -------------------------------------------------------------------------
//
// The `# TYPES` snapshot section already shows the published facts these queries read. What it
// cannot show is the rules layered over them: reflexivity, scope selection, what a cycle does
// to an answer, and the two conformance rules' treatment of untyped and unrelated features.

pub fn symbol_named(
    published: &PublishedResolution,
    document: &str,
    qualified: &str,
) -> SymbolIdentity {
    match published.document_symbols(document) {
        QueryOutcome::Resolved(entries)
        | QueryOutcome::Recovered(entries)
        | QueryOutcome::UnsupportedWith(entries) => entries
            .iter()
            .find(|entry| entry.qualified_name.as_ref() == qualified)
            .unwrap_or_else(|| panic!("no declaration named {qualified}"))
            .identity
            .clone(),
        other => panic!("expected document symbols, got: {other:?}"),
    }
}

pub fn conformance(outcome: QueryOutcome<Conformance>) -> Conformance {
    match outcome {
        QueryOutcome::Resolved(value)
        | QueryOutcome::Recovered(value)
        | QueryOutcome::UnsupportedWith(value) => value,
        other => panic!("expected a settled conformance answer, got: {other:?}"),
    }
}

pub fn symbols(outcome: QueryOutcome<Box<[SymbolIdentity]>>) -> Vec<SymbolIdentity> {
    match outcome {
        QueryOutcome::Resolved(values)
        | QueryOutcome::Recovered(values)
        | QueryOutcome::UnsupportedWith(values) => values.into_vec(),
        other => panic!("expected settled symbols, got: {other:?}"),
    }
}

// --- Reusing a settled library --------------------------------------------------------------

pub const LIBRARY_SOURCE: &str =
    "standard library package Lib { part def Base; part def Wheel :> Base; attribute def Mass; }";

pub fn library_stratum() -> std::sync::Arc<LibraryStratum> {
    std::sync::Arc::new(
        build_library_stratum(vec![SourceInput::new(
            "memory://lib.sysml",
            LIBRARY_SOURCE.to_string(),
            SourceKind::StandardLibrary,
        )])
        .expect("library stratum"),
    )
}

pub fn seeded_and_unseeded_with_library(library_source: &str, workspace: &str) -> (String, String) {
    let library = || {
        std::sync::Arc::new(
            build_library_stratum(vec![SourceInput::new(
                "memory://lib.sysml",
                library_source.to_string(),
                SourceKind::StandardLibrary,
            )])
            .expect("library stratum"),
        )
    };
    let seeded = build(
        BuildRequest::with_library(
            vec![SourceInput::new(
                "memory://workspace.sysml",
                workspace.to_string(),
                SourceKind::Workspace,
            )],
            ConstructionSchedule::Sequential,
            "contract-v1",
            library(),
        )
        .expect("seeded request"),
    )
    .expect("seeded build");
    let unseeded = build(
        BuildRequest::new(
            vec![
                SourceInput::new(
                    "memory://workspace.sysml",
                    workspace.to_string(),
                    SourceKind::Workspace,
                ),
                SourceInput::new(
                    "memory://lib.sysml",
                    library_source.to_string(),
                    SourceKind::StandardLibrary,
                ),
            ],
            ConstructionSchedule::Sequential,
            "contract-v1",
        )
        .expect("unseeded request"),
    )
    .expect("unseeded build");
    let render = |published: &PublishedResolution| {
        let mut semantic = String::new();
        published
            .debug()
            .write_semantic_sexpr(&mut semantic)
            .expect("semantic");
        let mut types = String::new();
        published
            .debug()
            .write_types_sexpr(&mut types)
            .expect("types");
        let mut diagnostics = String::new();
        published
            .debug()
            .write_diagnostics_sexpr(&mut diagnostics)
            .expect("diagnostics");
        format!("{semantic}\n{types}\n{diagnostics}")
    };
    (render(&seeded), render(&unseeded))
}

pub fn seeded_and_unseeded(workspace: &str) -> (String, String) {
    seeded_and_unseeded_with_library(LIBRARY_SOURCE, workspace)
}

/// A minimal but structurally faithful measurement library.
///
/// The unit rules are rooted in library declarations, so parity for them cannot be shown
/// against a library that declares none. This mirrors the standard library's shape exactly
/// where the rules read it: `MeasurementUnit` as the root of unit types, `TensorQuantityValue`
/// with the `mRef` feature a quantity value redefines, and units declared as attribute usages
/// carrying a short-name symbol.
pub const MEASUREMENT_LIBRARY_SOURCE: &str = concat!(
    "standard library package ScalarValues { datatype Boolean; datatype String; ",
    "datatype Real; datatype Integer :> Real; }\n",
    "standard library package MeasurementReferences { ",
    "abstract attribute def MeasurementUnit; ",
    "attribute def MassUnit :> MeasurementUnit; ",
    "attribute def DurationUnit :> MeasurementUnit; }\n",
    "standard library package Quantities { ",
    "abstract attribute def TensorQuantityValue { ",
    "attribute mRef : MeasurementReferences::MeasurementUnit; } ",
    "attribute def MassValue :> TensorQuantityValue { ",
    "attribute :>> mRef : MeasurementReferences::MassUnit; } }\n",
    "standard library package SI { ",
    "attribute <kg> kilogram : MeasurementReferences::MassUnit; ",
    "attribute <s> second : MeasurementReferences::DurationUnit; }",
);

/// A workspace exercising every migrated expression-conformance rule that reads the library.
pub const MEASUREMENT_WORKSPACE: &str = concat!(
    "package W { ",
    "attribute good : Quantities::MassValue = 1 [kg]; ",
    "attribute wrongDimension : Quantities::MassValue = 1 [s]; ",
    "attribute unknownUnit : Quantities::MassValue = 1 [zz]; ",
    "attribute mistyped : ScalarValues::Boolean = \"no\"; ",
    "constraint def Counted { 1 + 2 } }",
);

/// One publication of `workspace` against the measurement library above.
pub fn against_measurement_library(
    workspace: &str,
    schedule: ConstructionSchedule,
) -> PublishedResolution {
    build(
        BuildRequest::new(
            vec![
                SourceInput::new(
                    "memory://workspace.sysml",
                    workspace.to_string(),
                    SourceKind::Workspace,
                ),
                SourceInput::new(
                    "memory://measurement.sysml",
                    MEASUREMENT_LIBRARY_SOURCE.to_string(),
                    SourceKind::StandardLibrary,
                ),
            ],
            schedule,
            "contract-v1",
        )
        .expect("measurement request"),
    )
    .expect("measurement build")
}

pub fn measurement_publication(schedule: ConstructionSchedule) -> String {
    render_publication(&against_measurement_library(
        MEASUREMENT_WORKSPACE,
        schedule,
    ))
}

pub fn render_publication(published: &PublishedResolution) -> String {
    let mut semantic = String::new();
    published
        .debug()
        .write_semantic_sexpr(&mut semantic)
        .expect("semantic");
    let mut types = String::new();
    published
        .debug()
        .write_types_sexpr(&mut types)
        .expect("types");
    let mut diagnostics = String::new();
    published
        .debug()
        .write_diagnostics_sexpr(&mut diagnostics)
        .expect("diagnostics");
    format!("{semantic}\n{types}\n{diagnostics}")
}

// --- Element details --------------------------------------------------------------------

pub fn detail_publication(
    sources: &[(&str, &str)],
    schedule: ConstructionSchedule,
) -> PublishedResolution {
    let request = BuildRequest::new(
        sources
            .iter()
            .map(|(identity, source)| {
                SourceInput::new(*identity, source.to_string(), SourceKind::Workspace)
            })
            .collect(),
        schedule,
        "contract-v1",
    )
    .unwrap();
    build(request).unwrap()
}

pub fn settled<T: fmt::Debug>(outcome: QueryOutcome<T>) -> T {
    match outcome {
        QueryOutcome::Resolved(value)
        | QueryOutcome::Recovered(value)
        | QueryOutcome::UnsupportedWith(value) => value,
        other => panic!("expected a settled outcome, got: {other:?}"),
    }
}

pub fn identity_of(
    published: &PublishedResolution,
    document: &str,
    qualified_name: &str,
) -> SymbolIdentity {
    settled(published.document_symbols(document))
        .iter()
        .find(|entry| entry.qualified_name.as_ref() == qualified_name)
        .unwrap_or_else(|| panic!("no declaration named {qualified_name} in {document}"))
        .identity
        .clone()
}

pub fn details_of(
    published: &PublishedResolution,
    document: &str,
    qualified_name: &str,
) -> ElementDetails {
    settled(published.element_details(&identity_of(published, document, qualified_name)))
}

pub fn names(entries: &[SymbolEntry]) -> Vec<&str> {
    entries
        .iter()
        .map(|entry| entry.name.as_deref().unwrap_or("<anonymous>"))
        .collect()
}

/// One deterministic rendering of an element's details, for equivalence assertions.
pub fn render_details(details: &ElementDetails) -> String {
    let mut output = String::new();
    let family = |output: &mut String, label: &str, family: &RelationshipFamily| {
        output.push_str(&format!(
            "{label} {} {:?} {:?}\n",
            family.outcome.as_str(),
            names(&family.targets),
            names(&family.candidates)
        ));
    };
    output.push_str(&format!(
        "element {} {}\n",
        details.inspection.qualified_name,
        details.inspection.kind.as_str()
    ));
    output.push_str(&format!(
        "owner {:?}\n",
        details
            .owner
            .as_ref()
            .map(|owner| owner.qualified_name.clone())
    ));
    family(&mut output, "typing", &details.typing);
    family(&mut output, "specialization", &details.specialization);
    family(&mut output, "subsetting", &details.subsetting);
    family(&mut output, "redefinition", &details.redefinition);
    output.push_str(&format!(
        "effective-typing {} {:?}\n",
        details.effective_typing.outcome.as_str(),
        details
            .effective_typing
            .types
            .iter()
            .map(|entry| (
                entry.element.qualified_name.clone(),
                format!("{:?}", entry.origin)
            ))
            .collect::<Vec<_>>()
    ));
    output.push_str(&format!(
        "inherited {:?}\n",
        details
            .inherited_features
            .iter()
            .map(|entry| (
                entry.feature.qualified_name.clone(),
                entry.declared_in.qualified_name.clone()
            ))
            .collect::<Vec<_>>()
    ));
    output.push_str(&format!("metadata {:?}\n", names(&details.metadata)));
    for (label, connected) in [
        ("incoming", &details.incoming),
        ("outgoing", &details.outgoing),
    ] {
        output.push_str(&format!(
            "{label} {:?}\n",
            connected
                .iter()
                .map(|entry| (
                    entry.kind,
                    entry.peer.qualified_name.clone(),
                    format!("{:?}", entry.provenance)
                ))
                .collect::<Vec<_>>()
        ));
    }
    output.push_str(&format!("evaluation {}\n", details.evaluation.state));
    output.push_str(&format!("analysis {}\n", details.analysis.as_str()));
    output
}

pub fn part_definition_library() -> SourceInput {
    SourceInput::new(
        "memory://parts.sysml",
        concat!(
            "standard library package Parts { ",
            "part def Part; ",
            "part def Vehicle specializes Part; ",
            "}"
        )
        .to_string(),
        SourceKind::StandardLibrary,
    )
}

pub fn part_definition_workspace() -> SourceInput {
    SourceInput::new(
        "memory://model.sysml",
        concat!(
            "package Model { import Parts::*; ",
            "part def Component; ",
            "part def Equivalent specializes Part; ",
            "part def Specific specializes Vehicle; ",
            "}"
        )
        .to_string(),
        SourceKind::Workspace,
    )
}

pub fn specialization_relationships(
    published: &PublishedResolution,
    document: &str,
    qualified_name: &str,
) -> Vec<ElementRelationship> {
    settled(published.inspect(&identity_of(published, document, qualified_name)))
        .relationships
        .into_vec()
        .into_iter()
        .filter(|relationship| relationship.kind == "specialization")
        .collect()
}

pub fn type_featuring_relationships(
    published: &PublishedResolution,
    document: &str,
    qualified_name: &str,
) -> Vec<ElementRelationship> {
    settled(published.inspect(&identity_of(published, document, qualified_name)))
        .relationships
        .into_vec()
        .into_iter()
        .filter(|relationship| relationship.kind == "typeFeaturing")
        .collect()
}

/// A workspace exercising ownership, metadata, multiplicity, redefinition and an
/// unresolvable reference in one model.
pub const VEHICLE_MODEL: &str = concat!(
    "package P {\n",
    "  metadata def Safety;\n",
    "  part def Wheel;\n",
    "  part def Vehicle {\n",
    "    @Safety;\n",
    "    part wheel[4] : Wheel;\n",
    "    part spare[0..*] : Wheel;\n",
    "  }\n",
    "  part def Rover :> Vehicle {\n",
    "    part :>> wheel[4];\n",
    "  }\n",
    "  part rover : Rover;\n",
    "  part broken : Missing;\n",
    "  part selected subsets rover;\n",
    "}\n",
);
