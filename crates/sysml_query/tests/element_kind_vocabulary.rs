//! Source-level guards on the element-kind projection.
//!
//! These live here rather than in `sysml_resolution` because they parse Rust source with `syn`,
//! and that crate's dependency set is pinned to exactly four entries by
//! `architecture.rs::immutable_snapshot_runner_has_an_exact_graph_free_dependency_boundary`.
//! Adding a dev-dependency there would break that gate.
//!
//! What they close is a gap the compiler cannot: Rust proves a `match` covers every variant, but
//! it cannot prove the absence of a `_ =>` arm that makes coverage vacuous, and it cannot
//! enumerate an enum's variants to check a hand-maintained list against it.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use syn::{Item, ItemEnum, ItemFn, Pat, Stmt};

fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("repository root")
        .to_path_buf()
}

fn parse(path: &str) -> syn::File {
    let full = repository_root().join(path);
    let source = std::fs::read_to_string(&full)
        .unwrap_or_else(|error| panic!("read {}: {error}", full.display()));
    syn::parse_file(&source).unwrap_or_else(|error| panic!("parse {}: {error}", full.display()))
}

fn find_enum(file: &syn::File, name: &str) -> ItemEnum {
    file.items
        .iter()
        .find_map(|item| match item {
            Item::Enum(item) if item.ident == name => Some(item.clone()),
            _ => None,
        })
        .unwrap_or_else(|| panic!("enum {name} not found"))
}

fn find_fn(file: &syn::File, name: &str) -> ItemFn {
    file.items
        .iter()
        .find_map(|item| match item {
            Item::Fn(item) if item.sig.ident == name => Some(item.clone()),
            _ => None,
        })
        .unwrap_or_else(|| panic!("fn {name} not found"))
}

/// Every `DeclarationKind` variant named by the arms of a `match kind { .. }` in `body`.
fn match_arm_variants(item: &ItemFn) -> Vec<String> {
    let expression = match item.block.stmts.last() {
        Some(Stmt::Expr(expression, _)) => expression,
        _ => panic!("expected the projection body to end in a match expression"),
    };
    let syn::Expr::Match(matched) = expression else {
        panic!("expected the body to be a single match expression");
    };

    let mut variants = Vec::new();
    for arm in &matched.arms {
        assert!(
            !matches!(arm.pat, Pat::Wild(_)),
            "the projection has a `_ =>` arm, which makes its exhaustiveness vacuous: a new \
             declaration kind would silently acquire whatever that arm returns instead of failing \
             to compile"
        );
        collect_path_variants(&arm.pat, &mut variants);
    }
    variants
}

fn collect_path_variants(pattern: &Pat, out: &mut Vec<String>) {
    match pattern {
        Pat::Path(path) => {
            if let Some(segment) = path.path.segments.last() {
                out.push(segment.ident.to_string());
            }
        }
        Pat::Or(alternatives) => {
            for alternative in &alternatives.cases {
                collect_path_variants(alternative, out);
            }
        }
        _ => panic!(
            "unexpected pattern shape in the projection; only `Kind::Variant` and `|` \
             alternatives are expected"
        ),
    }
}

const MODEL: &str = "crates/sysml_resolution/src/model.rs";
const PROJECTION: &str = "crates/sysml_resolution/src/model/element_kind.rs";

fn declaration_kind_variants() -> BTreeSet<String> {
    find_enum(&parse(MODEL), "DeclarationKind")
        .variants
        .iter()
        .map(|variant| variant.ident.to_string())
        .collect()
}

/// The load-bearing guard: the projection must name every declaration kind exactly once, with no
/// wildcard arm.
#[test]
fn the_element_kind_projection_is_exhaustive_without_a_wildcard() {
    let projection = parse(PROJECTION);
    let declared = declaration_kind_variants();

    for name in ["element_kind", "membership_role"] {
        let arms = match_arm_variants(&find_fn(&projection, name));
        let mut seen = BTreeSet::new();
        for variant in &arms {
            assert!(
                seen.insert(variant.clone()),
                "`{name}` names DeclarationKind::{variant} more than once"
            );
        }
        assert_eq!(
            seen, declared,
            "`{name}` does not name exactly the DeclarationKind variants"
        );
    }
}

/// `ALL_DECLARATION_KINDS` backs the surjectivity and collapse-audit tests, so a variant missing
/// from it would silently weaken both. Rust cannot enumerate an enum, so pin the list against the
/// enum's own source.
#[test]
fn the_test_only_declaration_kind_list_covers_every_variant() {
    let source = std::fs::read_to_string(repository_root().join(PROJECTION)).expect("projection");
    let (_, tail) = source
        .split_once("const ALL_DECLARATION_KINDS: &[DeclarationKind] = &[")
        .expect("ALL_DECLARATION_KINDS not found");
    let (list, _) = tail.split_once("];").expect("unterminated list");

    let listed = list
        .split(',')
        .filter_map(|entry| entry.trim().strip_prefix("DeclarationKind::"))
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();

    assert_eq!(
        listed,
        declaration_kind_variants(),
        "ALL_DECLARATION_KINDS drifted from the DeclarationKind enum"
    );
}
