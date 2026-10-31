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

#[test]
fn bulk_imports_merge_repeated_insertions_and_preserve_crlf_comments() {
    let source = "package Machine { // header\r\n    // keep this comment\r\n    part driveA : Motor;\r\n    part driveB : Motor;\r\n    port plug : Plug;\r\n}\r\n";
    let workspace = multi_doc(&[(LIBRARY, COMPONENTS), (PATH, source)]);
    let fix = language_service::fix_all_missing_imports(&workspace, PATH).unwrap_or_else(|| {
        let uri = workspace.resolve_uri_for_path(PATH).unwrap();
        panic!(
            "bulk fix: {:?}",
            workspace
                .published_model()
                .unwrap()
                .edits()
                .missing_import_plan(uri.as_str())
        );
    });
    assert_eq!(fix.edits.len(), 1);
    assert_eq!(
        fix.title,
        "Add all unambiguous missing imports (3 references; 0 remaining)"
    );
    assert_eq!(apply(source, &fix), "package Machine { // header\r\n    private import Components::Motor;\r\n    private import Ports::Plug;\r\n    // keep this comment\r\n    part driveA : Motor;\r\n    part driveB : Motor;\r\n    port plug : Plug;\r\n}\r\n");
}

#[test]
fn bulk_imports_use_exact_package_ranges_when_packages_share_a_line() {
    let source = "package One { part drive : Motor; } package Two { port plug : Plug; }";
    let workspace = multi_doc(&[(LIBRARY, COMPONENTS), (PATH, source)]);
    let fix = language_service::fix_all_missing_imports(&workspace, PATH).unwrap_or_else(|| {
        let uri = workspace.resolve_uri_for_path(PATH).unwrap();
        panic!(
            "bulk fix: {:?}",
            workspace
                .published_model()
                .unwrap()
                .edits()
                .missing_import_plan(uri.as_str())
        );
    });
    assert_eq!(fix.edits.len(), 2);
    assert_eq!(apply(source, &fix), "package One { private import Components::Motor; part drive : Motor; } package Two { private import Ports::Plug; port plug : Plug; }");
}

#[test]
fn bulk_imports_keep_conflicting_names_for_explicit_selection() {
    let source = "package Machine { part a : Same; port b : Same; part good : Good; }";
    let library = "package A { part def Same; part def Good; } package B { port def Same; }";
    let workspace = multi_doc(&[(LIBRARY, library), (PATH, source)]);
    let fix = language_service::fix_all_missing_imports(&workspace, PATH).expect("safe subset");
    assert_eq!(
        fix.title,
        "Add all unambiguous missing imports (1 reference; 2 remaining)"
    );
    assert_eq!(apply(source, &fix), "package Machine { private import A::Good; part a : Same; port b : Same; part good : Good; }");
}

#[test]
fn bulk_imports_are_not_computed_against_stale_editor_text() {
    let source = "package Machine { part drive : Motor; }";
    let published = multi_doc(&[(LIBRARY, COMPONENTS), (PATH, source)]);
    let ahead = EditedAhead {
        published,
        editor: format!("// newer\n{source}"),
    };
    assert!(language_service::fix_all_missing_imports(&ahead, PATH).is_none());
    let source = "package Machine { part drive : Mot; }";
    let ahead = EditedAhead {
        published: multi_doc(&[(LIBRARY, COMPONENTS), (PATH, source)]),
        editor: format!("// newer\n{source}"),
    };
    let result = language_service::complete(&ahead, PATH, TextPosition::new(1, 34)).unwrap();
    assert!(result.semantic_status.predates_source);
    assert!(result.items.iter().all(|item| item.import.is_none()));
}

fn auto_import_items(
    source: &str,
    library: &str,
    prefix: &str,
) -> (
    language_service::InMemoryWorkspace,
    Vec<language_service::CompletionItemDto>,
) {
    let workspace = multi_doc(&[(LIBRARY, library), (PATH, source)]);
    let offset = source.rfind(prefix).expect("prefix") + prefix.len();
    let before = &source[..offset];
    let line = before.bytes().filter(|b| *b == b'\n').count() as u32;
    let character = before.rsplit('\n').next().unwrap().encode_utf16().count() as u32;
    let result = language_service::complete(&workspace, PATH, TextPosition::new(line, character))
        .expect("completion");
    let imports = result
        .items
        .into_iter()
        .filter(|item| item.import.is_some())
        .collect();
    (workspace, imports)
}

#[test]
fn completion_import_edits_keep_unicode_positions_and_form_one_transaction() {
    let source = "package Machine {\r\n    /*🧭*/ part drive : Mot;\r\n}\r\n";
    let (workspace, items) = auto_import_items(source, COMPONENTS, "Mot");
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].label, "Motor");
    let (primary, additional) = language_service::resolve_completion_import(
        &workspace,
        PATH,
        items[0].import.as_ref().unwrap(),
    )
    .expect("resolved import");
    assert_eq!(items[0].text_edit.as_ref(), Some(&primary));
    let mut edits = additional;
    edits.push(language_service::TextEditDto {
        path: PATH.into(),
        range: primary.range,
        replacement: primary.new_text,
    });
    let fix = TextEditSuggestion::new("completion", edits);
    assert_eq!(apply(source, &fix), "package Machine {\r\n    private import Components::Motor;\r\n    /*🧭*/ part drive : Motor;\r\n}\r\n");
}

#[test]
fn completion_disambiguates_duplicate_names_by_qualified_package() {
    let source = "package Machine { part drive : Mot; }";
    let (workspace, items) = auto_import_items(
        source,
        "package A { part def Motor; } package B { part def Motor; }",
        "Mot",
    );
    let paths = items
        .iter()
        .map(|item| {
            item.label_details
                .as_ref()
                .unwrap()
                .description
                .as_deref()
                .unwrap()
        })
        .collect::<Vec<_>>();
    assert_eq!(paths, ["A::Motor", "B::Motor"]);
    assert!(items
        .windows(2)
        .all(|pair| pair[0].sort_text < pair[1].sort_text));
    for item in items {
        assert!(language_service::resolve_completion_import(
            &workspace,
            PATH,
            item.import.as_ref().unwrap()
        )
        .is_some());
    }
}

#[test]
fn completion_imports_alias_membership_instead_of_its_referent_name() {
    let source = "package Machine { part drive : Eng; }";
    let (workspace, items) = auto_import_items(
        source,
        "package Components { part def Motor; alias Engine for Motor; }",
        "Eng",
    );
    assert_eq!(items.len(), 1);
    let (_, edits) = language_service::resolve_completion_import(
        &workspace,
        PATH,
        items[0].import.as_ref().unwrap(),
    )
    .expect("alias import");
    assert_eq!(edits[0].replacement, " private import Components::Engine;");
}

#[test]
fn completion_does_not_add_an_import_for_an_already_visible_definition() {
    let source = "package Machine { private import Components::Motor; part drive : Mot; }";
    let (_, items) = auto_import_items(source, COMPONENTS, "Mot");
    assert!(items.is_empty());
}

#[test]
fn completion_escapes_package_and_definition_membership_spellings() {
    let source = "package Machine { part drive : Mot; }";
    let (workspace, items) = auto_import_items(
        source,
        "package 'My Catalog' { part def 'Motor Unit'; }",
        "Mot",
    );
    assert_eq!(items.len(), 1);
    let (primary, edits) = language_service::resolve_completion_import(
        &workspace,
        PATH,
        items[0].import.as_ref().unwrap(),
    )
    .unwrap();
    assert_eq!(primary.new_text, "'Motor Unit'");
    assert_eq!(
        edits[0].replacement,
        " private import 'My Catalog'::'Motor Unit';"
    );
}

#[test]
fn completion_can_import_from_an_admitted_installed_library() {
    let services = sysml_query::Services::new();
    let source = services
        .source
        .admit_memory(
            "installed",
            PATH,
            "package Machine { part drive : LibMot; }",
            sysml_query::source::SourceKind::Workspace,
        )
        .unwrap();
    let library = services
        .source
        .admit_memory(
            "installed",
            LIBRARY,
            "package Installed { part def LibMotor; }",
            sysml_query::source::SourceKind::Library,
        )
        .unwrap();
    let publication = services
        .publication
        .publish(&[source.clone(), library.clone()], [])
        .unwrap();
    let workspace = language_service::InMemoryWorkspace::from_documents_and_publication(
        vec![source, library],
        publication,
    )
    .unwrap();
    let source_text = workspace
        .document_text(&workspace.resolve_uri_for_path(PATH).unwrap())
        .unwrap();
    let column = source_text.find("LibMot").unwrap() + 6;
    let result =
        language_service::complete(&workspace, PATH, TextPosition::new(0, column as u32)).unwrap();
    let item = result
        .items
        .into_iter()
        .find(|item| item.label == "LibMotor")
        .expect("installed definition");
    let (_, edits) = language_service::resolve_completion_import(
        &workspace,
        PATH,
        item.import.as_ref().unwrap(),
    )
    .unwrap();
    assert_eq!(edits[0].replacement, " private import Installed::LibMotor;");
}

#[test]
fn bulk_root_imports_are_deduplicated() {
    let source = "part a : Motor;\npart b : Motor;\n";
    let workspace = multi_doc(&[(LIBRARY, COMPONENTS), (PATH, source)]);
    let fix = language_service::fix_all_missing_imports(&workspace, PATH).unwrap();
    assert_eq!(fix.edits.len(), 1);
    assert_eq!(
        apply(source, &fix),
        "private import Components::Motor;\npart a : Motor;\npart b : Motor;\n"
    );
}

#[test]
fn completion_and_bulk_leave_incomplete_publications_without_import_edits() {
    let source = "package Machine { part drive : Mot;";
    let workspace = multi_doc(&[(LIBRARY, COMPONENTS), (PATH, source)]);
    assert!(language_service::fix_all_missing_imports(&workspace, PATH).is_none());
    let (_, items) = auto_import_items(source, COMPONENTS, "Mot");
    assert!(items.is_empty());
}
