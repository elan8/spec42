//! No consumer re-implements what the authorities answer.
//!
//! The dependency-graph guards (`deny.toml`, `parser_authority.rs`, `authority_chain.rs`,
//! `architecture.rs`) make it impossible for a consumer to *name* the parser or the authority
//! crates. They cannot see a consumer that re-answers a syntax question from the text — a
//! `starts_with("part ")`, a brace count, a second table of keywords, a struct that keeps its own
//! parsed trees. This file is the guard for that, in four rules:
//!
//! 1. **Retired helpers stay deleted.** An exact list of the names this repository removed when
//!    their answers moved behind the facade. Precise, and the rule to extend first.
//! 2. **No downstream function shadows a facade query.** The set of facade function names is
//!    derived from the facade's own source, so a new query protects its name automatically.
//! 3. **No second cache and no source I/O downstream.** Fields holding parsed trees, strata or
//!    AST versions, and file reads of SysML text, belong to the authorities; the editor host's
//!    index entries are the allow-listed exception (an `Arc` into the memo is not a cache).
//! 4. **No text probing for SysML syntax.** A `syn` visitor flags string probes, comparisons and
//!    `matches!` arms against reserved keywords, qualified-name fragments, SysML operators and
//!    braces. The keyword set is read from the facade, so there is one vocabulary.
//!
//! Rule 4 is a heuristic and says so: a re-implementation that spells no keyword, operator or
//! brace is invisible to it, which is why rule 1 exists. Files that legitimately work on SysML
//! text — a whitespace-only formatter, completion over text the grammar has not accepted, hover
//! prose, wire vocabularies — are exempt one by one, each with a reason and, where one exists, a
//! predicate that keeps the exemption honest. Every deferred retirement is recorded in
//! `planning/SYNTAX_FOLLOW_UPS.md`; an exemption with no entry there is a bug.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use syn::visit::{self, Visit};
use syn::{Expr, Item, Lit, Pat, Type};

/// Crates that may spell SysML syntax in Rust source: the authorities and the facade.
const AUTHORITY_CRATES: &[&str] = &["sysml_source", "sysml_resolution", "sysml_query"];

/// Helpers this repository removed when their answers moved behind the facade.
const RETIRED_NAMES: &[&str] = &[
    "extract_package_name",
    "content_contains_unit_definition",
    "workspace_contains_unit_literal",
    "resolve_library_closure",
    "library_closure_seed_signature",
    "SemanticPublicationAuthority",
    "PublicationCoordinator",
    "WorkspaceSession",
    "SyntaxDocument",
    "SyntaxParse",
    "parse_strict",
    "parse_cache",
    "ParseOutcome",
    "SemanticCoordinator",
    "build_semantic_model_from_documents",
    "import_statement_ranges",
    "find_reference_ranges",
    "word_at_position",
    "unit_value_suffix_at_position",
    "unit_value_suffix_selection_at_position",
    // An element identity is a `SymbolId` handle now; the string encoding is `SymbolToken` and
    // exists only where one has to cross a boundary.
    "SymbolIdentity",
];

/// Files that may work on SysML text, each with the reason and (where one exists) a predicate
/// that must hold for the exemption to stand. A file not listed here is held to rule 4 in full.
struct Exemption {
    path: &'static str,
    reason: &'static str,
    /// Text the exempt file must still contain: the property that justifies the exemption.
    must_contain: Option<&'static str>,
}

const EXEMPTIONS: &[Exemption] = &[
    Exemption {
        path: "crates/language_service/src/formatting.rs",
        reason: "whitespace-only formatter; every candidate is validated by the syntax service",
        must_contain: Some("reformatting_preserves_meaning"),
    },
    Exemption {
        path: "crates/language_service/src/completion.rs",
        reason: "completion over text the grammar has not accepted yet; its keyword table is held to the facade's",
        must_contain: Some("use crate::keywords::{keyword_doc, sysml_keywords};"),
    },
    Exemption {
        path: "crates/lsp_server/src/common/util.rs",
        reason: "reads LSP configuration JSON whose boolean literals collide with the keyword table; it probes no SysML text",
        must_contain: Some("parse_diagnose_library_paths_from_value"),
    },
    Exemption {
        path: "crates/language_service/src/navigation.rs",
        reason: "splits a published qualified name for its container; awaits a container accessor on the published name (planning/SYNTAX_FOLLOW_UPS.md)",
        must_contain: Some("rsplit_once(\"::\")"),
    },
    Exemption {
        path: "crates/language_service/src/symbol.rs",
        reason: "splits a published qualified name for its container; awaits a container accessor on the published name (planning/SYNTAX_FOLLOW_UPS.md)",
        must_contain: Some("rsplit_once(\"::\")"),
    },
    Exemption {
        path: "crates/language_service/src/keywords.rs",
        reason: "keyword hover prose keyed by keyword; the table itself is the facade's",
        must_contain: Some("pub use sysml_query::syntax::{is_reserved_keyword, RESERVED_KEYWORDS};"),
    },
    Exemption {
        path: "crates/lsp_server/src/lsp_runtime/features/completion.rs",
        reason: "completion snippet bodies are presentation",
        must_contain: None,
    },
    Exemption {
        path: "crates/sysml_tokens/src/lexer.rs",
        reason: "fallback lexer for text the parser classified nothing in; its keyword table is the facade's",
        must_contain: Some("sysml_query::syntax::is_reserved_keyword"),
    },
    Exemption {
        path: "crates/sysml_tokens/src/ast_ranges.rs",
        reason: "declaration-name narrowing awaiting name-only token roles (planning/SYNTAX_FOLLOW_UPS.md)",
        must_contain: None,
    },
    Exemption {
        path: "crates/server/src/lib.rs",
        reason: "matches its own advice prose for the words `standard library`; not SysML text",
        must_contain: None,
    },
    Exemption {
        path: "crates/kpar/src/read.rs",
        reason: "reads KerML Project Archive members by name to materialise and verify them; parses no syntax",
        must_contain: None,
    },
    Exemption {
        path: "crates/generator_conformance/src/runner.rs",
        reason: "reads generator modules and case fixtures; model text is read through the source service",
        must_contain: Some(".read_text("),
    },
    Exemption {
        path: "crates/server/src/sysand.rs",
        reason: "classifies sysand TOML manifest paths, not SysML text",
        must_contain: None,
    },
    Exemption {
        path: "crates/generator_protocol/src/lib.rs",
        reason: "wire vocabulary of the generator protocol",
        must_contain: None,
    },
    Exemption {
        path: "crates/generator_api/src/model.rs",
        reason: "wire vocabulary of the generator protocol",
        must_contain: None,
    },
    Exemption {
        path: "crates/library_catalog/src/library/bundle.rs",
        reason: "provisioning materialises library trees from archives; it reads archives, not SysML",
        must_contain: None,
    },
    Exemption {
        path: "crates/library_catalog/src/library/stdlib.rs",
        reason: "provisioning materialises library trees; it reads archives, not SysML",
        must_contain: None,
    },
    Exemption {
        path: "crates/library_catalog/src/library/managed.rs",
        reason: "provisioning materialises library trees; it reads archives, not SysML",
        must_contain: None,
    },
    Exemption {
        path: "crates/library_catalog/src/catalog.rs",
        reason: "catalog digests library files by byte content to identify a root; it reads no syntax",
        must_contain: None,
    },
];

/// Fields outside the authorities that may hold a parsed tree: the editor host's index entries,
/// which are `Arc`s into the syntax memo rather than a cache.
const PARSED_TREE_FIELD_ALLOWLIST: &[(&str, &str)] = &[
    ("crates/lsp_server/src/session/state.rs", "IndexEntry"),
    (
        "crates/lsp_server/src/session/services.rs",
        "ParsedScanEntry",
    ),
];

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/<name> is two levels below the repository root")
        .to_path_buf()
}

fn rust_sources(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            rust_sources(&path, out);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            out.push(path);
        }
    }
}

/// Every `src/` tree of a crate outside the authorities.
fn consumer_sources(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for entry in fs::read_dir(root.join("crates"))
        .expect("crates dir")
        .flatten()
    {
        let name = entry.file_name().to_string_lossy().to_string();
        if AUTHORITY_CRATES.contains(&name.as_str()) {
            continue;
        }
        rust_sources(&entry.path().join("src"), &mut out);
    }
    out.sort();
    out
}

fn relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

/// Production items only: a module carrying `#[cfg(test)]` is skipped.
fn is_test_module(item: &syn::ItemMod) -> bool {
    item.attrs.iter().any(|attribute| {
        attribute.path().is_ident("cfg") && attribute.to_token_stream_string().contains("test")
    })
}

trait TokenText {
    fn to_token_stream_string(&self) -> String;
}

impl TokenText for syn::Attribute {
    fn to_token_stream_string(&self) -> String {
        quote_tokens(&self.meta)
    }
}

fn quote_tokens(meta: &syn::Meta) -> String {
    match meta {
        syn::Meta::Path(path) => path_string(path),
        syn::Meta::List(list) => format!("{}({})", path_string(&list.path), list.tokens),
        syn::Meta::NameValue(pair) => {
            format!(
                "{} = {}",
                path_string(&pair.path),
                pair.value.to_string_lossy()
            )
        }
    }
}

fn path_string(path: &syn::Path) -> String {
    path.segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect::<Vec<_>>()
        .join("::")
}

trait ExprText {
    fn to_string_lossy(&self) -> String;
}

impl ExprText for Expr {
    fn to_string_lossy(&self) -> String {
        match self {
            Expr::Lit(literal) => match &literal.lit {
                Lit::Str(value) => value.value(),
                _ => String::new(),
            },
            _ => String::new(),
        }
    }
}

fn parse_production(path: &Path) -> syn::File {
    let source = fs::read_to_string(path).expect("read Rust source");
    let mut file = syn::parse_file(&source).expect("parse Rust source");
    file.items
        .retain(|item| !matches!(item, Item::Mod(module) if is_test_module(module)));
    file
}

// ─────────────────────────────────────────────────────────────────────────────────────────
// Rule 1: retired helpers stay deleted.

#[test]
fn retired_helpers_stay_deleted() {
    let root = repository_root();
    let mut violations = Vec::new();
    for file in consumer_sources(&root) {
        let source = fs::read_to_string(&file).expect("read Rust source");
        for name in RETIRED_NAMES {
            if mentions_ident(&source, name) {
                violations.push(format!("{}: mentions `{name}`", relative(&root, &file)));
            }
        }
    }
    assert!(
        violations.is_empty(),
        "a retired helper returned; its answer lives behind sysml_query now:\n{}",
        violations.join("\n")
    );
}

/// Whole-identifier mention, so `parse_strict` does not match `parse_strictly`.
fn mentions_ident(source: &str, name: &str) -> bool {
    let bytes = source.as_bytes();
    let mut start = 0;
    while let Some(offset) = source[start..].find(name) {
        let begin = start + offset;
        let end = begin + name.len();
        let before_ok = begin == 0 || !is_ident_byte(bytes[begin - 1]);
        let after_ok = end == bytes.len() || !is_ident_byte(bytes[end]);
        if before_ok && after_ok {
            return true;
        }
        start = end;
    }
    false
}

fn is_ident_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

// ─────────────────────────────────────────────────────────────────────────────────────────
// Rule 2: no downstream function shadows a facade query.

/// Facade names generic enough to mean something else downstream, plus the two identity helpers
/// hosts wrap one-for-one (`workspace::path_to_file_url` maps the error; `WorkspaceSnapshot::
/// normalize_uri` is a trait method that delegates).
const SHADOW_ALLOW: &[&str] = &[
    "path_to_file_url",
    "normalize_uri",
    "new",
    "default",
    "parse",
    "build",
    "admit",
    "load",
    "list",
    "discover",
    "retain",
    "resolve",
    "publish",
    "prepare",
    "construct",
    "identity",
    "uri",
    "kind",
    "digest",
    "content",
    "source",
    "outline",
    "lifecycle",
    "version",
    "current",
    "token",
    "failure",
    "close",
    "reset",
    "message",
    "stage",
    "documents",
    "request",
    "inner",
    "authority",
    "debug",
    "publication",
    "dependencies",
    "navigation",
    "edits",
    "completion",
    "inspection",
    "types",
    "evaluation",
    "diagnostics",
    "diagrams",
    "catalog",
    "view",
    "evaluate",
    "roles",
    "syntax",
    "library",
];

#[derive(Default)]
struct FnIdents {
    idents: BTreeSet<String>,
    public_only: bool,
}

impl<'ast> Visit<'ast> for FnIdents {
    fn visit_item_fn(&mut self, item: &'ast syn::ItemFn) {
        if !self.public_only || matches!(item.vis, syn::Visibility::Public(_)) {
            self.idents.insert(item.sig.ident.to_string());
        }
        visit::visit_item_fn(self, item);
    }

    fn visit_impl_item_fn(&mut self, item: &'ast syn::ImplItemFn) {
        if !self.public_only || matches!(item.vis, syn::Visibility::Public(_)) {
            self.idents.insert(item.sig.ident.to_string());
        }
        visit::visit_impl_item_fn(self, item);
    }

    fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {
        if !is_test_module(item) {
            visit::visit_item_mod(self, item);
        }
    }
}

#[test]
fn downstream_functions_do_not_shadow_facade_queries() {
    // The syntax, library and source services: the queries a consumer could re-derive from text.
    // Publication queries are delegated to, not re-derived, so wrapping their names is fine.
    let root = repository_root();
    let mut facade = Vec::new();
    for file in ["syntax.rs", "library.rs", "source.rs"] {
        facade.push(root.join("crates/sysml_query/src").join(file));
    }
    for module in ["syntax", "library"] {
        rust_sources(
            &root.join("crates/sysml_resolution/src").join(module),
            &mut facade,
        );
    }
    facade.push(root.join("crates/sysml_source/src/lib.rs"));
    let mut facade_names = FnIdents {
        public_only: true,
        ..FnIdents::default()
    };
    for file in facade {
        facade_names.visit_file(&parse_production(&file));
    }
    let protected: BTreeSet<String> = facade_names
        .idents
        .into_iter()
        .filter(|name| !SHADOW_ALLOW.contains(&name.as_str()))
        .collect();
    assert!(
        protected.contains("closure_facts") && protected.contains("reserved_keywords"),
        "the derived facade name set is missing known queries: {protected:?}"
    );

    let mut violations = Vec::new();
    for file in consumer_sources(&root) {
        let mut names = FnIdents::default();
        names.visit_file(&parse_production(&file));
        for name in names.idents.intersection(&protected) {
            violations.push(format!(
                "{}: fn `{name}` shadows a facade query; call the facade instead",
                relative(&root, &file)
            ));
        }
    }
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

// ─────────────────────────────────────────────────────────────────────────────────────────
// Rule 3: no second cache and no source I/O downstream.

const TREE_TYPES: &[&str] = &[
    "ParsedSource",
    "LibraryStratum",
    "PublishedResolution",
    "ParsedDocument",
];

struct CacheVisitor<'a> {
    file: String,
    allow_structs: Vec<&'a str>,
    violations: &'a mut Vec<String>,
}

fn type_mentions(ty: &Type, names: &[&str]) -> Option<String> {
    struct Finder<'a> {
        names: &'a [&'a str],
        found: Option<String>,
    }
    impl<'ast> Visit<'ast> for Finder<'_> {
        fn visit_ident(&mut self, ident: &'ast syn::Ident) {
            let text = ident.to_string();
            if self.names.contains(&text.as_str()) && self.found.is_none() {
                self.found = Some(text);
            }
        }
    }
    let mut finder = Finder { names, found: None };
    finder.visit_type(ty);
    finder.found
}

impl<'ast> Visit<'ast> for CacheVisitor<'_> {
    fn visit_item_struct(&mut self, item: &'ast syn::ItemStruct) {
        let name = item.ident.to_string();
        if !self.allow_structs.contains(&name.as_str()) {
            for field in &item.fields {
                if let Some(found) = type_mentions(&field.ty, TREE_TYPES) {
                    self.violations.push(format!(
                        "{}: struct `{name}` holds a `{found}`; only the authorities hold trees",
                        self.file
                    ));
                }
            }
        }
        visit::visit_item_struct(self, item);
    }

    fn visit_item_enum(&mut self, item: &'ast syn::ItemEnum) {
        for variant in &item.variants {
            for field in &variant.fields {
                if let Some(found) = type_mentions(&field.ty, TREE_TYPES) {
                    self.violations.push(format!(
                        "{}: enum `{}` holds a `{found}`",
                        self.file, item.ident
                    ));
                }
            }
        }
        visit::visit_item_enum(self, item);
    }

    fn visit_ident(&mut self, ident: &'ast syn::Ident) {
        let text = ident.to_string();
        let lowered = text.to_ascii_lowercase();
        if text == "SYNTAX_AST_VERSION"
            || lowered.contains("parse_cache")
            || lowered.contains("parsecache")
            || lowered.contains("parseoutcome")
            || lowered.contains("syntaxcache")
        {
            self.violations.push(format!(
                "{}: `{text}` names a parse cache; caches belong to the authorities",
                self.file
            ));
        }
    }

    fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {
        if !is_test_module(item) {
            visit::visit_item_mod(self, item);
        }
    }
}

#[test]
fn no_second_cache_and_no_source_io_downstream() {
    let root = repository_root();
    let mut violations = Vec::new();
    for file in consumer_sources(&root) {
        let rel = relative(&root, &file);
        let allow_structs = PARSED_TREE_FIELD_ALLOWLIST
            .iter()
            .filter(|(path, _)| *path == rel)
            .map(|(_, name)| *name)
            .collect::<Vec<_>>();
        CacheVisitor {
            file: rel.clone(),
            allow_structs,
            violations: &mut violations,
        }
        .visit_file(&parse_production(&file));

        // Source I/O: a file that names SysML extensions and reads files is reading SysML text.
        if EXEMPTIONS.iter().any(|exemption| exemption.path == rel) {
            continue;
        }
        let source = fs::read_to_string(&file).expect("read Rust source");
        let production = source.split("#[cfg(test)]").next().unwrap_or(&source);
        let names_sysml = production.contains(".sysml") || production.contains(".kerml");
        let reads = [
            "fs::read_to_string(",
            "fs::read(",
            "walkdir::",
            "ignore::",
            "WalkDir::",
        ]
        .iter()
        .any(|needle| production.contains(needle));
        if names_sysml && reads {
            violations.push(format!(
                "{rel}: reads SysML files itself; admit them through sysml_query::source"
            ));
        }
    }
    for (path, name) in PARSED_TREE_FIELD_ALLOWLIST {
        let source = fs::read_to_string(root.join(path)).expect("allow-listed file exists");
        assert!(
            source.contains(&format!("struct {name}")),
            "{path} no longer defines `{name}`; remove the allow-list entry"
        );
    }
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

// ─────────────────────────────────────────────────────────────────────────────────────────
// Rule 4: no text probing for SysML syntax.

const PROBE_METHODS: &[&str] = &[
    "find",
    "rfind",
    "contains",
    "starts_with",
    "ends_with",
    "strip_prefix",
    "strip_suffix",
    "split_once",
    "rsplit_once",
    "split",
    "trim_start_matches",
    "trim_end_matches",
    "matches",
];

const OPERATOR_LITERALS: &[&str] = &["::", ":>", ":>>", "::>", "attribute <", "::*", "::**"];

struct Probe<'a> {
    keywords: &'a BTreeSet<&'a str>,
}

impl Probe<'_> {
    /// Why a string literal reads as SysML syntax, if it does.
    fn literal_reason(&self, value: &str) -> Option<String> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return None;
        }
        if OPERATOR_LITERALS.contains(&trimmed) {
            return Some(format!("operator literal {value:?}"));
        }
        let tokens: Vec<&str> = trimmed.split_whitespace().collect();
        let all_keywords = tokens.iter().all(|token| self.keywords.contains(token));
        if tokens.len() >= 2 && all_keywords {
            return Some(format!("keyword phrase {value:?}"));
        }
        if tokens.len() == 1 && all_keywords && value != trimmed {
            return Some(format!("padded keyword {value:?}"));
        }
        if tokens.len() >= 2 && self.keywords.contains(tokens[0]) {
            return Some(format!("keyword-led phrase {value:?}"));
        }
        if let Some((head, _)) = trimmed.split_once("::") {
            if !head.is_empty() && head.chars().all(|c| c.is_alphanumeric() || c == '_') {
                return Some(format!("qualified-name fragment {value:?}"));
            }
        }
        None
    }

    fn bare_keyword(&self, value: &str) -> bool {
        self.keywords.contains(value.trim())
    }
}

struct ProbeVisitor<'a> {
    file: String,
    probe: Probe<'a>,
    consts: std::collections::BTreeMap<String, String>,
    violations: &'a mut Vec<String>,
}

fn lit_str(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Lit(literal) => match &literal.lit {
            Lit::Str(value) => Some(value.value()),
            _ => None,
        },
        Expr::Reference(reference) => lit_str(&reference.expr),
        _ => None,
    }
}

fn lit_brace(expr: &Expr) -> bool {
    match expr {
        Expr::Lit(literal) => match &literal.lit {
            Lit::Char(value) => matches!(value.value(), '{' | '}'),
            Lit::Byte(value) => matches!(value.value(), b'{' | b'}'),
            _ => false,
        },
        Expr::Reference(reference) => lit_brace(&reference.expr),
        _ => false,
    }
}

impl ProbeVisitor<'_> {
    fn literal_or_const(&self, expr: &Expr) -> Option<String> {
        if let Some(value) = lit_str(expr) {
            return Some(value);
        }
        if let Expr::Path(path) = expr {
            if let Some(ident) = path.path.get_ident() {
                return self.consts.get(&ident.to_string()).cloned();
            }
        }
        None
    }

    fn flag(&mut self, what: String) {
        self.violations.push(format!("{}: {what}", self.file));
    }
}

impl<'ast> Visit<'ast> for ProbeVisitor<'_> {
    fn visit_item_fn(&mut self, item: &'ast syn::ItemFn) {
        let is_documented_code_action_text_operation = self.file
            == "crates/language_service/src/code_actions.rs"
            && matches!(
                item.sig.ident.to_string().as_str(),
                "parse_untyped_part_usage_name"
                    | "rewrite_untyped_part_usage_line"
                    | "rewrite_implicit_redefinition_line"
            );
        if !is_documented_code_action_text_operation {
            visit::visit_item_fn(self, item);
        }
    }

    fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {
        if !is_test_module(item) {
            visit::visit_item_mod(self, item);
        }
    }

    fn visit_expr_method_call(&mut self, call: &'ast syn::ExprMethodCall) {
        let method = call.method.to_string();
        if PROBE_METHODS.contains(&method.as_str()) {
            for argument in &call.args {
                if let Some(value) = self.literal_or_const(argument) {
                    if let Some(reason) = self.probe.literal_reason(&value) {
                        self.flag(format!(".{method}({reason}) probes SysML text"));
                    } else if self.probe.bare_keyword(&value) {
                        self.flag(format!(".{method}({value:?}) probes for a keyword"));
                    }
                }
                if lit_brace(argument) {
                    self.flag(format!(".{method}('{{' / '}}') counts braces"));
                }
            }
        }
        visit::visit_expr_method_call(self, call);
    }

    fn visit_expr_binary(&mut self, binary: &'ast syn::ExprBinary) {
        if matches!(binary.op, syn::BinOp::Eq(_) | syn::BinOp::Ne(_)) {
            for side in [&*binary.left, &*binary.right] {
                if let Some(value) = self.literal_or_const(side) {
                    if self.probe.bare_keyword(&value)
                        || self.probe.literal_reason(&value).is_some()
                    {
                        self.flag(format!("compares text against {value:?}"));
                    }
                }
                if lit_brace(side) {
                    self.flag("compares a character against a brace".to_string());
                }
            }
        }
        visit::visit_expr_binary(self, binary);
    }

    fn visit_pat(&mut self, pattern: &'ast Pat) {
        if let Pat::Lit(literal) = pattern {
            if let Lit::Str(value) = &literal.lit {
                let value = value.value();
                if self.probe.bare_keyword(&value) || self.probe.literal_reason(&value).is_some() {
                    self.flag(format!("matches text against {value:?}"));
                }
            }
        }
        visit::visit_pat(self, pattern);
    }

    fn visit_macro(&mut self, mac: &'ast syn::Macro) {
        let name = mac
            .path
            .segments
            .last()
            .map(|segment| segment.ident.to_string())
            .unwrap_or_default();
        if name == "matches" {
            for token in mac.tokens.clone() {
                if let proc_macro2::TokenTree::Literal(literal) = token {
                    let text = literal.to_string();
                    if text.starts_with('"') && text.ends_with('"') && text.len() >= 2 {
                        let value = &text[1..text.len() - 1];
                        if self.probe.bare_keyword(value) {
                            self.flag(format!("matches! arm against keyword {value:?}"));
                        }
                    }
                }
            }
        }
    }
}

fn const_strings(file: &syn::File) -> std::collections::BTreeMap<String, String> {
    let mut out = std::collections::BTreeMap::new();
    for item in &file.items {
        if let Item::Const(constant) = item {
            if let Some(value) = lit_str(&constant.expr) {
                out.insert(constant.ident.to_string(), value);
            }
        }
    }
    out
}

fn probe_file(
    rel: &str,
    file: &syn::File,
    keywords: &BTreeSet<&str>,
    violations: &mut Vec<String>,
) {
    ProbeVisitor {
        file: rel.to_string(),
        probe: Probe { keywords },
        consts: const_strings(file),
        violations,
    }
    .visit_file(file);
}

#[test]
fn no_text_probing_for_sysml_syntax_outside_exempt_files() {
    let root = repository_root();
    let keywords: BTreeSet<&str> = sysml_query::syntax::reserved_keywords()
        .iter()
        .copied()
        .collect();
    let mut violations = Vec::new();
    for file in consumer_sources(&root) {
        let rel = relative(&root, &file);
        if EXEMPTIONS.iter().any(|exemption| exemption.path == rel) {
            continue;
        }
        probe_file(&rel, &parse_production(&file), &keywords, &mut violations);
    }
    assert!(
        violations.is_empty(),
        "SysML syntax is answered by the syntax service, not by probing text:\n{}",
        violations.join("\n")
    );
}

#[test]
fn every_exemption_names_an_existing_file_whose_justifying_property_still_holds() {
    let root = repository_root();
    for exemption in EXEMPTIONS {
        let path = root.join(exemption.path);
        assert!(
            path.exists(),
            "{}: exempt file no longer exists; remove the exemption ({})",
            exemption.path,
            exemption.reason
        );
        if let Some(needle) = exemption.must_contain {
            let source = fs::read_to_string(&path).expect("read exempt file");
            assert!(
                source.contains(needle),
                "{}: the property justifying its exemption no longer holds ({})",
                exemption.path,
                exemption.reason
            );
        }
        if exemption.reason.contains("SYNTAX_FOLLOW_UPS") {
            let planning = fs::read_to_string(root.join("planning/SYNTAX_FOLLOW_UPS.md"))
                .expect("planning record");
            let file_name = Path::new(exemption.path)
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string();
            assert!(
                planning.contains(&file_name),
                "{}: deferred retirement is not recorded in planning/SYNTAX_FOLLOW_UPS.md",
                exemption.path
            );
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────────────────
// Meta-tests: the detector fires on code and not on prose, messages, or test modules.

fn probe_fixture(source: &str) -> Vec<String> {
    let keywords: BTreeSet<&str> = ["package", "part", "def", "import", "attribute"]
        .into_iter()
        .collect();
    let file = syn::parse_file(source).expect("fixture parses");
    let mut file = file;
    file.items
        .retain(|item| !matches!(item, Item::Mod(module) if is_test_module(module)));
    let mut violations = Vec::new();
    probe_file("fixture.rs", &file, &keywords, &mut violations);
    violations
}

#[test]
fn detector_flags_keyword_phrases_probes_comparisons_braces_and_matches_arms() {
    let violations = probe_fixture(
        r#"
        const HEADER: &str = "requirement def ";
        fn a(line: &str) -> bool { line.strip_prefix("package ").is_some() }
        fn b(t: &str) -> bool { t.starts_with("part ") || t.starts_with("part def") }
        fn c(l: &str) -> bool { l.strip_prefix(HEADER).is_some() }
        fn d(l: &str) -> bool { l.split_once("attribute <").is_some() }
        fn e(c: &str) -> bool { c.contains("SysML::") }
        fn f(w: &str) -> bool { w == "package" }
        fn g(line: &str) -> usize { line.chars().filter(|ch| *ch == '{').count() }
        fn h(t: &str) -> bool { matches!(t, "package" | "part") }
        fn i(t: &str) -> bool { match t { "import" => true, _ => false } }
        fn j(w: &str) -> bool { w.starts_with("package") }
        "#,
    );
    for expected in [
        "padded keyword",
        "keyword phrase",
        "operator literal",
        "qualified-name fragment",
        "probes for a keyword",
        "compares text against",
        "compares a character against a brace",
        "matches! arm against keyword",
        "matches text against",
    ] {
        assert!(
            violations.iter().any(|v| v.contains(expected)),
            "expected a violation containing {expected:?}, got: {violations:#?}"
        );
    }
    assert!(violations.len() >= 10, "{violations:#?}");
}

#[test]
fn detector_ignores_comments_messages_bare_labels_paths_and_test_modules() {
    let violations = probe_fixture(
        r#"
        // line.strip_prefix("package ")
        fn a(name: &str) -> String { format!("part {name} not found") }
        fn b() -> &'static str { "part" }
        fn c(uri: &str) -> bool { uri.starts_with("file://") }
        fn d(p: &str) -> bool { p.contains("/") }
        fn e(e: &str) { tracing::warn!("import failed: {e}") }
        #[cfg(test)]
        mod tests { fn t(l: &str) -> bool { l.starts_with("part def") } }
        "#,
    );
    assert!(violations.is_empty(), "{violations:#?}");
}

#[test]
fn retired_name_matching_is_whole_identifier() {
    assert!(mentions_ident("fn parse_strict() {}", "parse_strict"));
    assert!(!mentions_ident("fn parse_strictly() {}", "parse_strict"));
    assert!(!mentions_ident("fn my_parse_strict() {}", "parse_strict"));
}
