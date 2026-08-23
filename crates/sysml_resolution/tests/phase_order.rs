//! The pipeline's phase order is a property of the source tree, not just of a call sequence.
//!
//! `planning/B_resolution_phases.md` establishes that this crate is a barrier-ordered pipeline:
//! each phase reads the frozen product of the phases before it and publishes its own, and nothing
//! loops back. Ordering is enforced at runtime by the coordinator's call sequence — which no test
//! can see once a module has been added that quietly reaches forward. This file guards the
//! ordering structurally, in three rules:
//!
//! 1. **The phase order is declared once.** [`PHASES`] is that declaration: one module directory
//!    per phase, in build order. A new phase directory under `src/` that is not listed here fails
//!    rule 1, so the order cannot be extended silently.
//! 2. **A phase does not import a later phase.** Every remaining forward edge is enumerated in
//!    [`FORWARD_EDGES`] with the reason it stands. A new one fails; a listed one that no longer
//!    exists fails too, so the list shrinks as the tree is fixed and never rots.
//! 3. **Evaluation has exactly one writer.** `planning/B_resolution_phases.md` §2 item 3 recorded
//!    two producers for one fact category: lowering classified constraint/calc expressions into
//!    `ExpressionEvalShape` while evaluation settled the same category later. Lowering now records
//!    the authored site and evaluation alone classifies it, so the shape vocabulary must not
//!    appear outside `evaluate/`, and the retired two-writer helpers must stay deleted.
//!
//! Rules 1 and 2 read `use` paths and `crate::` mentions out of the source text. That is a
//! deliberately shallow reading: it sees the names a phase spells, which is exactly what the
//! ordering claim is about. It does not see a phase reaching forward through a re-export, which is
//! why rule 3 names the one fact category that has actually gone wrong here before.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

/// The pipeline's phases, in build order, by the module directory that owns each one.
///
/// Sourced from `planning/B_resolution_phases.md` §1, which lists the coordinator's call order.
/// Only the phases that own a directory appear; the phases that are still spread across
/// `model.rs`/`lib.rs` are the subject of that document's remaining work, not of this guard.
const PHASES: &[(&str, &str)] = &[
    (
        "syntax",
        "phase 1b — syntax-fidelity queries over the parsed tree",
    ),
    ("library", "phase 1c — library closure and package index"),
    (
        "lower",
        "phase 2 — authored facts derived from parsed trees",
    ),
    ("resolve", "phase 3 — name resolution to a fixed point"),
    (
        "evaluate",
        "phase 5 — constant evaluation over settled resolution",
    ),
    (
        "index",
        "phase 6 — derived-fact indexes over the assembled model",
    ),
    ("check", "phase 7 — conformance over the assembled model"),
    (
        "diagnose",
        "phase 8 — diagnostics derived from the assembled model",
    ),
];

/// One phase naming a later one, with the reason it is still here.
///
/// Every entry is a defect with a known shape, not an accepted design. The point of enumerating
/// them is that the set is visible and cannot grow without this test failing.
struct ForwardEdge {
    from: &'static str,
    to: &'static str,
    /// The item the earlier phase names in the later one.
    item: &'static str,
    reason: &'static str,
}

const FORWARD_EDGES: &[ForwardEdge] = &[
    ForwardEdge {
        from: "check",
        to: "diagnose",
        item: "document_range",
        reason: "a pure span-projection helper misfiled in the diagnostics phase; it derives no \
                 diagnostic and reads no assembled model",
    },
    ForwardEdge {
        from: "index",
        to: "diagnose",
        item: "document_range",
        reason: "same span-projection helper as check -> diagnose",
    },
    ForwardEdge {
        from: "index",
        to: "diagnose",
        item: "declaration_identifier_range",
        reason: "span projection for a declaration's own identifier; derives no diagnostic",
    },
    ForwardEdge {
        from: "evaluate",
        to: "index",
        item: "expressions",
        reason: "phase 5 publishes SettledFilter, whose record type is defined with the phase 6 \
                 expression index that consumes it; the type moves, not the writer",
    },
    ForwardEdge {
        from: "library",
        to: "resolve",
        item: "implied",
        reason: "the settled library stratum carries implied relationships forward as a seed for \
                 the workspace solve, so it names the shape phase 3 produces",
    },
    ForwardEdge {
        from: "resolve",
        to: "index",
        item: "documents",
        reason: "the solver reads the document index to scope a lookup; the index is built from \
                 lowered facts alone and does not depend on resolution",
    },
    ForwardEdge {
        from: "lower",
        to: "evaluate",
        item: "classify",
        reason: "AST-shape predicates (flatten_member_access_chain, is_*_operator, \
                 classify_filter_predicate) that lowering needs to walk an expression. They \
                 classify syntax, never evaluation: rule 3 holds them to that",
    },
    ForwardEdge {
        from: "lower",
        to: "evaluate",
        item: "fold",
        reason: "quantity_unit_text, which reads a unit token's identity out of the parser arena \
                 so lowering can record the authored token",
    },
];

/// The vocabulary evaluation classifies into. Naming it is claiming to be evaluation.
const EVALUATION_SHAPE: &str = "ExpressionEvalShape";

/// Helpers removed when evaluation stopped having two writers. Matched as whole identifiers.
const RETIRED_NAMES: &[&str] = &[
    "constraint_evaluation_shape",
    "calc_evaluation_shape",
    "classify_constraint_expression",
    "classify_calc_expression",
    "classify_constraint_expression_from",
];

fn src_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

/// Every `.rs` file under one phase's directory.
fn phase_files(phase: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();
    collect_rs(&src_dir().join(phase), &mut files);
    files.sort();
    files
}

fn collect_rs(dir: &Path, out: &mut Vec<PathBuf>) {
    let entries = fs::read_dir(dir).unwrap_or_else(|e| panic!("reading {}: {e}", dir.display()));
    for entry in entries {
        let path = entry.expect("directory entry").path();
        if path.is_dir() {
            collect_rs(&path, out);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            out.push(path);
        }
    }
}

/// Whether `text` contains `name` as a whole Rust identifier rather than as a substring.
fn mentions_identifier(text: &str, name: &str) -> bool {
    let boundary = |c: char| !(c.is_alphanumeric() || c == '_');
    let mut rest = text;
    while let Some(at) = rest.find(name) {
        let before_ok = rest[..at].chars().next_back().is_none_or(boundary);
        let after = &rest[at + name.len()..];
        let after_ok = after.chars().next().is_none_or(boundary);
        if before_ok && after_ok {
            return true;
        }
        rest = &rest[at + name.len()..];
    }
    false
}

/// Every `(from, to, item)` triple where a phase names a later phase, read out of the source text.
fn observed_forward_edges() -> BTreeSet<(String, String, String)> {
    let rank = |phase: &str| PHASES.iter().position(|(name, _)| *name == phase);
    let mut edges = BTreeSet::new();
    for (phase, _) in PHASES {
        let from_rank = rank(phase).expect("phase is listed");
        for file in phase_files(phase) {
            let text = fs::read_to_string(&file).expect("phase source is readable");
            for hit in text.match_indices("crate::") {
                let tail = &text[hit.0 + "crate::".len()..];
                let mut segments = tail.split("::");
                let Some(target) = segments.next() else {
                    continue;
                };
                let target: String = target
                    .chars()
                    .take_while(|c| c.is_alphanumeric() || *c == '_')
                    .collect();
                let Some(to_rank) = rank(&target) else {
                    continue;
                };
                if to_rank <= from_rank {
                    continue;
                }
                let item: String = segments
                    .next()
                    .unwrap_or_default()
                    .chars()
                    .take_while(|c| c.is_alphanumeric() || *c == '_')
                    .collect();
                edges.insert((phase.to_string(), target, item));
            }
        }
    }
    edges
}

/// Rule 1: every phase directory under `src/` is declared in `PHASES`, and every declared phase
/// exists. A phase that is added to the tree but not to the order is the failure this catches.
#[test]
fn every_phase_directory_is_declared_in_the_phase_order() {
    for (phase, description) in PHASES {
        assert!(
            src_dir().join(phase).is_dir(),
            "PHASES lists `{phase}` ({description}) but src/{phase}/ does not exist; \
             delete the entry or restore the directory"
        );
    }

    // A directory under src/ holding a phase's code but absent from PHASES. The non-phase module
    // directories are named so the rule addresses exactly one question.
    const NOT_PHASES: &[&str] = &["model", "pipeline", "publication"];
    let declared: BTreeSet<&str> = PHASES
        .iter()
        .map(|(name, _)| *name)
        .chain(NOT_PHASES.iter().copied())
        .collect();
    for entry in fs::read_dir(src_dir()).expect("src/ is readable") {
        let path = entry.expect("directory entry").path();
        if !path.is_dir() {
            continue;
        }
        let name = path
            .file_name()
            .and_then(|name| name.to_str())
            .expect("module directory name");
        assert!(
            declared.contains(name),
            "src/{name}/ is a module directory that is neither a declared phase nor listed in \
             NOT_PHASES. Add it to PHASES in its build position, or to NOT_PHASES with the \
             reason it is not a phase"
        );
    }
}

/// Rule 2: a phase does not name a later phase, except for the enumerated edges.
///
/// Both directions are checked. An unlisted edge is a new violation. A listed edge that no longer
/// exists is a stale exception, and leaving it would let a future reintroduction pass unnoticed.
#[test]
fn no_phase_imports_a_later_phase_except_the_enumerated_edges() {
    let observed = observed_forward_edges();
    let allowed: BTreeSet<(String, String, String)> = FORWARD_EDGES
        .iter()
        .map(|edge| {
            (
                edge.from.to_string(),
                edge.to.to_string(),
                edge.item.to_string(),
            )
        })
        .collect();

    for edge in FORWARD_EDGES {
        assert!(
            !edge.reason.trim().is_empty(),
            "the {} -> {} edge on `{}` is enumerated with no reason. An exception with no \
             justification is indistinguishable from an oversight.",
            edge.from,
            edge.to,
            edge.item
        );
    }

    let new: Vec<_> = observed.difference(&allowed).collect();
    assert!(
        new.is_empty(),
        "these phases reach forward to a later phase and are not enumerated in FORWARD_EDGES: \
         {new:?}. A phase reads the frozen product of the phases before it \
         (planning/B_resolution_phases.md §1). Move the item down to the phase that owns it, or \
         add a FORWARD_EDGES entry recording why the edge stands."
    );

    let stale: Vec<_> = allowed.difference(&observed).collect();
    assert!(
        stale.is_empty(),
        "these FORWARD_EDGES entries no longer describe the tree: {stale:?}. The edge was fixed \
         — delete the entry, so a reintroduction fails this test."
    );
}

/// Rule 3, first half: `ExpressionEvalShape` is evaluation's vocabulary and no other phase spells
/// it.
///
/// Lowering used to classify each constraint/calc expression into this enum as it lowered it, and
/// store the result on the fact. One fact category then had two producers in two phases. Lowering
/// now records `AuthoredExpression` — the site — and `evaluate::classify::classify_authored` is
/// the single place a site becomes a shape.
#[test]
fn only_the_evaluation_phase_names_the_evaluation_shape() {
    for (phase, description) in PHASES {
        if *phase == "evaluate" {
            continue;
        }
        for file in phase_files(phase) {
            let text = fs::read_to_string(&file).expect("phase source is readable");
            assert!(
                !mentions_identifier(&text, EVALUATION_SHAPE),
                "{} ({description}) names `{EVALUATION_SHAPE}`. Classifying an authored \
                 expression into an evaluation shape is phase 5's alone; record the authored site \
                 (`AuthoredExpression`) and let `evaluate/` classify it.",
                file.display()
            );
        }
    }
}

/// Rule 3, second half: the helpers that made lowering the second writer stay deleted.
///
/// Rule 3's first half stops the *type* from spreading. These names are what the two-writer
/// arrangement was actually spelled as, and a reintroduction would most likely reuse them.
#[test]
fn the_retired_evaluation_writer_helpers_stay_deleted() {
    for (phase, _) in PHASES {
        for file in phase_files(phase) {
            let text = fs::read_to_string(&file).expect("phase source is readable");
            for name in RETIRED_NAMES {
                assert!(
                    !mentions_identifier(&text, name),
                    "{} names `{name}`, retired when evaluation stopped having two writers. \
                     Lowering records the authored expression site; \
                     `evaluate::classify::classify_expression` classifies it.",
                    file.display()
                );
            }
        }
    }
}
