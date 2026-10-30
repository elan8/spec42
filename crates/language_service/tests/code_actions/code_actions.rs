use language_service::{
    explicit_redefinition_quick_fix, position_to_byte_offset, reference_quick_fixes,
    TextEditSuggestion, WorkspaceSnapshot,
};
use sysml_query::resolved_slice::{DiagnosticCode, TextPosition};

use crate::support::{multi_doc, single_doc};

const PATH: &str = "model.sysml";
const LIBRARY: &str = "components.sysml";
const COMPONENTS: &str = "package Components {\n    part def Motor;\n    private part def Secret;\n}\npackage Ports {\n    port def Plug;\n}\n";

/// Where the published model reports `code` in `PATH`: the position a host asks for fixes at.
fn diagnostic_position(
    workspace: &impl WorkspaceSnapshot,
    code: &DiagnosticCode,
) -> Option<TextPosition> {
    let uri = workspace.resolve_uri_for_path(PATH)?;
    workspace
        .published_model()?
        .diagnostics()
        .for_document(uri.as_str())
        .iter()
        .find(|diagnostic| diagnostic.code() == code)
        .map(|diagnostic| diagnostic.location().range().start)
}

fn codes(sources: &[(&str, &str)]) -> Vec<DiagnosticCode> {
    let workspace = multi_doc(sources);
    let uri = workspace.resolve_uri_for_path(PATH).expect("model URI");
    workspace
        .published_model()
        .expect("model")
        .diagnostics()
        .for_document(uri.as_str())
        .iter()
        .map(|diagnostic| diagnostic.code().clone())
        .collect()
}

fn reference_fixes(sources: &[(&str, &str)], code: DiagnosticCode) -> Vec<TextEditSuggestion> {
    let workspace = multi_doc(sources);
    let position = diagnostic_position(&workspace, &code).expect("diagnostic");
    reference_quick_fixes(&workspace, PATH, position)
}

fn apply(source: &str, fix: &TextEditSuggestion) -> String {
    let mut edits = fix.edits.clone();
    assert!(edits.iter().all(|edit| edit.path.ends_with(PATH)));
    edits.sort_by_key(|edit| (edit.range.start.line, edit.range.start.character));
    let mut text = source.to_string();
    for edit in edits.iter().rev() {
        let start =
            position_to_byte_offset(&text, edit.range.start.line, edit.range.start.character)
                .expect("start");
        let end = position_to_byte_offset(&text, edit.range.end.line, edit.range.end.character)
            .expect("end");
        text.replace_range(start..end, &edit.replacement);
    }
    text
}

fn titles(fixes: &[TextEditSuggestion]) -> Vec<(&str, bool)> {
    fixes
        .iter()
        .map(|fix| (fix.title.as_str(), fix.is_preferred))
        .collect()
}

#[test]
fn the_only_importable_definition_is_a_preferred_import_after_the_opening_brace() {
    let source = "package Machine {\n    part drive : Motor;\n}\n";
    let fixes = reference_fixes(
        &[(LIBRARY, COMPONENTS), (PATH, source)],
        DiagnosticCode::UnresolvedTypeReference,
    );
    assert_eq!(titles(&fixes), [("Import `Components::Motor`", true)]);
    let fixed = apply(source, &fixes[0]);
    assert_eq!(
        fixed,
        "package Machine {\n    private import Components::Motor;\n    part drive : Motor;\n}\n"
    );
    assert!(codes(&[(LIBRARY, COMPONENTS), (PATH, &fixed)]).is_empty());
}

#[test]
fn an_import_follows_the_packages_last_import_with_its_indentation() {
    let source = "package Machine {\n  private import Ports::*;\n\n  part drive : Motor;\n}\n";
    let fixes = reference_fixes(
        &[(LIBRARY, COMPONENTS), (PATH, source)],
        DiagnosticCode::UnresolvedTypeReference,
    );
    let fixed = apply(source, &fixes[0]);
    assert_eq!(
        fixed,
        "package Machine {\n  private import Ports::*;\n  private import Components::Motor;\n\n  part drive : Motor;\n}\n"
    );
    assert!(codes(&[(LIBRARY, COMPONENTS), (PATH, &fixed)]).is_empty());
}

#[test]
fn an_import_goes_into_the_innermost_package_around_the_reference() {
    let source = "package Outer {\n    private import Ports::*;\n    package Machine {\n        part drive : Motor;\n    }\n}\n";
    let fixes = reference_fixes(
        &[(LIBRARY, COMPONENTS), (PATH, source)],
        DiagnosticCode::UnresolvedTypeReference,
    );
    let fixed = apply(source, &fixes[0]);
    assert_eq!(
        fixed,
        "package Outer {\n    private import Ports::*;\n    package Machine {\n        private import Components::Motor;\n        part drive : Motor;\n    }\n}\n"
    );
    assert!(codes(&[(LIBRARY, COMPONENTS), (PATH, &fixed)]).is_empty());
}

#[test]
fn an_import_into_a_one_line_package_is_written_inline() {
    let source = "package Machine { part drive : Motor; }\n";
    let fixes = reference_fixes(
        &[(LIBRARY, COMPONENTS), (PATH, source)],
        DiagnosticCode::UnresolvedTypeReference,
    );
    let fixed = apply(source, &fixes[0]);
    assert_eq!(
        fixed,
        "package Machine { private import Components::Motor; part drive : Motor; }\n"
    );
    assert!(codes(&[(LIBRARY, COMPONENTS), (PATH, &fixed)]).is_empty());
}

#[test]
fn a_specialization_imports_the_definition_it_names() {
    let source = "package Machine {\n    part def Gearbox :> Motor;\n}\n";
    let fixes = reference_fixes(
        &[(LIBRARY, COMPONENTS), (PATH, source)],
        DiagnosticCode::UnresolvedSpecializesReference,
    );
    assert_eq!(titles(&fixes), [("Import `Components::Motor`", true)]);
    let fixed = apply(source, &fixes[0]);
    assert!(codes(&[(LIBRARY, COMPONENTS), (PATH, &fixed)]).is_empty());
}

#[test]
fn several_importable_definitions_are_offered_without_a_preference() {
    let other = "package Spares {\n    part def Motor;\n}\n";
    let source = "package Machine {\n    part drive : Motor;\n}\n";
    let fixes = reference_fixes(
        &[
            (LIBRARY, COMPONENTS),
            ("spares.sysml", other),
            (PATH, source),
        ],
        DiagnosticCode::UnresolvedTypeReference,
    );
    assert_eq!(
        titles(&fixes),
        [
            ("Import `Components::Motor`", false),
            ("Import `Spares::Motor`", false),
        ]
    );
}

#[test]
fn a_name_nothing_importable_declares_gets_a_definition_of_the_slots_kind() {
    let source = "package Machine {\n    part drive : Secret;\n    port plug : Socket;\n}\n";
    let fixes = reference_fixes(
        &[(LIBRARY, COMPONENTS), (PATH, source)],
        DiagnosticCode::UnresolvedTypeReference,
    );
    assert_eq!(titles(&fixes), [("Create `part def Secret`", true)]);
    let fixed = apply(source, &fixes[0]);
    assert_eq!(
        fixed,
        "package Machine {\n    part drive : Secret;\n    port plug : Socket;\n    part def Secret;\n}\n"
    );

    let workspace = multi_doc(&[(LIBRARY, COMPONENTS), (PATH, &fixed)]);
    let position = diagnostic_position(&workspace, &DiagnosticCode::UnresolvedTypeReference)
        .expect("the port is still unresolved");
    let fixes = reference_quick_fixes(&workspace, PATH, position);
    assert_eq!(titles(&fixes), [("Create `port def Socket`", true)]);
    let fixed = apply(&fixed, &fixes[0]);
    assert!(codes(&[(LIBRARY, COMPONENTS), (PATH, &fixed)]).is_empty());
}

#[test]
fn a_definition_in_a_one_line_package_is_written_before_its_closing_brace() {
    let source = "package P { part car : Vehicle; }\n";
    let fixes = reference_fixes(&[(PATH, source)], DiagnosticCode::UnresolvedTypeReference);
    let fixed = apply(source, &fixes[0]);
    assert_eq!(
        fixed,
        "package P { part car : Vehicle; part def Vehicle; }\n"
    );
    assert!(codes(&[(PATH, &fixed)]).is_empty());
}

#[test]
fn a_definition_outside_any_package_is_appended_to_the_document() {
    let source = "part car : Vehicle;";
    let fixes = reference_fixes(&[(PATH, source)], DiagnosticCode::UnresolvedTypeReference);
    let fixed = apply(source, &fixes[0]);
    assert_eq!(fixed, "part car : Vehicle;\npart def Vehicle;\n");
    assert!(codes(&[(PATH, &fixed)]).is_empty());
}

#[test]
fn a_qualified_reference_gets_no_fix() {
    let source = "package Machine {\n    part wheel : Components::Wheel;\n}\n";
    let fixes = reference_fixes(
        &[(LIBRARY, COMPONENTS), (PATH, source)],
        DiagnosticCode::UnresolvedTypeReference,
    );
    assert!(fixes.is_empty());
}

#[test]
fn an_ambiguous_name_is_qualified_by_each_candidate() {
    let source = "package A { part def Thing; }\npackage B { part def Thing; }\npackage Use {\n    private import A::*;\n    private import B::*;\n    part usage : Thing;\n}\n";
    let fixes = reference_fixes(&[(PATH, source)], DiagnosticCode::AmbiguousReference);
    assert_eq!(
        titles(&fixes),
        [
            ("Qualify as `A::Thing`", false),
            ("Qualify as `B::Thing`", false)
        ]
    );
    for fix in &fixes {
        let fixed = apply(source, fix);
        assert!(!codes(&[(PATH, &fixed)]).contains(&DiagnosticCode::AmbiguousReference));
    }
}

/// A workspace whose editor has typed past the text the model was published from.
struct EditedAhead {
    published: language_service::InMemoryWorkspace,
    editor: String,
}

impl WorkspaceSnapshot for EditedAhead {
    fn resolve_uri_for_path(&self, path: &str) -> Option<url::Url> {
        self.published.resolve_uri_for_path(path)
    }
    fn path_for_uri(&self, uri: &url::Url) -> String {
        self.published.path_for_uri(uri)
    }
    fn document_text(&self, uri: &url::Url) -> Option<&str> {
        self.published.document_text(uri)
    }
    fn editor_text(&self, _uri: &url::Url) -> Option<&str> {
        Some(&self.editor)
    }
    fn parsed(&self, uri: &url::Url) -> Option<sysml_query::syntax::ParsedSource> {
        self.published.parsed(uri)
    }
    fn published_model(&self) -> Option<&sysml_query::resolved_slice::PublishedModel> {
        self.published.published_model()
    }
    fn symbol_table(&self) -> &[language_service::SymbolEntry] {
        self.published.symbol_table()
    }
    fn index_uris(&self) -> Vec<url::Url> {
        self.published.index_uris()
    }
}

#[test]
fn no_fix_is_offered_once_the_editor_text_has_moved_past_the_publication() {
    let source = "package P {\n    part car : Vehicle;\n}\n";
    let published = single_doc(PATH, source);
    let position = diagnostic_position(&published, &DiagnosticCode::UnresolvedTypeReference)
        .expect("diagnostic");
    assert!(!reference_quick_fixes(&published, PATH, position).is_empty());
    let ahead = EditedAhead {
        published,
        editor: format!("// edited\n{source}"),
    };
    assert!(reference_quick_fixes(&ahead, PATH, position).is_empty());
}

#[test]
fn an_implicit_redefinition_is_made_explicit() {
    let source = "package Base {\n    part def Vehicle { attribute mass = 1000; }\n    part def Car :> Vehicle { attribute mass = 1200; }\n}\n";
    let code = DiagnosticCode::ImplicitRedefinitionWithoutOperator;
    let workspace = single_doc(PATH, source);
    let position = diagnostic_position(&workspace, &code).expect("diagnostic");
    let fix = explicit_redefinition_quick_fix(&workspace, PATH, position).expect("fix");
    assert!(fix.is_preferred);
    let fixed = apply(source, &fix);
    assert_eq!(
        fixed,
        "package Base {\n    part def Vehicle { attribute mass = 1000; }\n    part def Car :> Vehicle { attribute :>> mass = 1200; }\n}\n"
    );
    assert!(!codes(&[(PATH, &fixed)]).contains(&code));
}
