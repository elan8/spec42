//! Native/legacy ELK layout geometry parity over production-shaped graphs (issue #118).
//!
//! `layout_shadow::layout_elk_graph_shadow` is exercised against every checked-in parity
//! fixture: the hand-authored cases in `tools/elkrs_parity/fixtures/*.json` plus the exact ELK
//! graph JSON captured from the real repository diagram-product corpus and synthetic node-chrome
//! stress cases in `tools/elkrs_parity/fixtures/corpus/*.json` (regenerated from
//! `vscode/diagram-renderer/src/render/elk-parity-corpus-fixtures.test.ts` via
//! `UPDATE_ELK_FIXTURES=1 npm test`).
//!
//! Parity is geometry-scalar equality (node/port/label rectangles, routed edge-section points),
//! not raw JSON equality: elkrs and ELK.js disagree on incidental shape, such as which container
//! owns an intra-hierarchy edge or which internal solver-state layout options get echoed back per
//! node, that `ShadowLayoutResult::exactly_equal` would flag as a difference even though Spec42
//! draws identical diagrams from either output. `elkrs_parity`'s comparison library is the
//! reviewed definition of parity (see its own geometry-comparison tests); this test reuses it
//! in-process as the fast CI gate, leaving `cargo run --release -p elkrs_parity` as the
//! human-readable/benchmark-oriented tool for manual or release-profile runs.

#![cfg(feature = "native-layout-shadow")]

use std::fs;
use std::path::{Path, PathBuf};

use elkrs_parity::{compare_geometry, extract_geometry};
use spec42::layout_shadow::layout_elk_graph_shadow;

const TOLERANCE: f64 = 1e-9;

/// Fixtures with a known, investigated geometry divergence between elkrs and ELK.js, kept out of
/// the hard-fail set so one open cross-implementation gap does not block the shadow-mode CI gate
/// for the rest of the corpus.
///
/// Each entry lists the *exact* scalar paths (`GeometryDifference::path`, sorted) the known
/// divergence produces — not just the fixture name. `native_and_legacy_layout_agree_on_the_full_parity_corpus`
/// fails if the actual diverging set differs at all from this one: a new or additional diverging
/// path means a second, unrelated regression snuck into this fixture and must not be silently
/// absorbed into the tracked entry; a smaller diverging set means the tracked divergence has
/// narrowed or been fixed and this allowlist is stale and must be tightened or removed.
///
/// - `timer_interconnection.json`: `n_15`/`n_24` (see `tools/elkrs_parity/fixtures/corpus/`) both
///   use `org.eclipse.elk.portConstraints=FIXED_ORDER` with multiple ports and CENTER alignment.
///   FIXED_ORDER only pins relative order, not coordinates, so each engine's own port-placement
///   math decides the node's height needed to fit its ports; elkrs and ELK.js disagree on that
///   height here. That single sizing difference at `n_6` (the lowest common ancestor of the ports
///   involved) cascades through ELK's stacked sibling layout: it shifts every edge and node
///   position in `n_6`'s subtree and every sibling laid out after `n_6` under their shared parent
///   `n_0`, which is why the diverging set below is large (43 scalars) despite the root cause
///   being one node's height. The root-relative coordinate translation itself was hand-verified
///   correct (offsets accumulate n_0.y + n_6.y exactly as
///   `diagram_layout::collect_root_relative_edges` computes them); this is a genuine
///   elkrs-vs-ELK.js algorithm difference in multi-port FIXED_ORDER sizing, not a normalization
///   bug. Tracked for follow-up under #118/#119.
const KNOWN_DIVERGENCES: &[(&str, &[&str])] = &[(
    "timer_interconnection.json",
    &[
        "edge:e:10/section:0/end/y",
        "edge:e:10/section:0/start/y",
        "edge:e:12/section:0/bend:0/y",
        "edge:e:12/section:0/bend:1/y",
        "edge:e:12/section:0/end/y",
        "edge:e:12/section:0/start/y",
        "edge:e:14/section:0/bend:0/y",
        "edge:e:14/section:0/bend:1/y",
        "edge:e:14/section:0/end/y",
        "edge:e:14/section:0/start/y",
        "edge:e:29/section:0/bend:1/y",
        "edge:e:29/section:0/bend:2/y",
        "edge:e:29/section:0/bend:3/y",
        "edge:e:29/section:0/end/y",
        "edge:e:31/section:0/bend:1/y",
        "edge:e:31/section:0/bend:2/x",
        "edge:e:31/section:0/bend:2/y",
        "edge:e:31/section:0/bend:3/x",
        "edge:e:31/section:0/bend:3/y",
        "edge:e:31/section:0/end/y",
        "edge:e:33/section:0/bend:0/x",
        "edge:e:33/section:0/bend:0/y",
        "edge:e:33/section:0/bend:1/x",
        "edge:e:33/section:0/bend:1/y",
        "edge:e:33/section:0/end/y",
        "edge:e:35/section:0/bend:0/x",
        "edge:e:35/section:0/bend:0/y",
        "edge:e:35/section:0/bend:1/x",
        "edge:e:35/section:0/bend:1/y",
        "edge:e:35/section:0/end/y",
        "edge:e:37/section:0/end/y",
        "edge:e:37/section:0/start/y",
        "edge:e:8/section:0/end/y",
        "edge:e:8/section:0/start/y",
        "graph/height",
        "graph/node:n_0/height",
        "graph/node:n_0/node:n_28/y",
        "graph/node:n_0/node:n_6/height",
        "graph/node:n_0/node:n_6/node:n_11/y",
        "graph/node:n_0/node:n_6/node:n_15/y",
        "graph/node:n_0/node:n_6/node:n_21/y",
        "graph/node:n_0/node:n_6/node:n_24/y",
        "graph/node:n_0/node:n_6/y",
    ],
)];

fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crates/server is two levels below the repository root")
        .to_path_buf()
}

fn fixture_paths() -> Vec<PathBuf> {
    let fixtures_dir = repository_root().join("tools/elkrs_parity/fixtures");
    let mut paths = Vec::new();
    collect_json_files(&fixtures_dir, &mut paths);
    paths.sort();
    paths
}

fn collect_json_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let entries = fs::read_dir(dir)
        .unwrap_or_else(|error| panic!("read fixture directory {}: {error}", dir.display()));
    for entry in entries {
        let entry = entry.expect("read fixture directory entry");
        let path = entry.path();
        if path.is_dir() {
            collect_json_files(&path, out);
        } else if path
            .extension()
            .is_some_and(|extension| extension == "json")
        {
            out.push(path);
        }
    }
}

/// Compares the actual diverging scalar paths for one fixture against its expected set (empty for
/// every fixture except the entries in `KNOWN_DIVERGENCES`). Returns a description of any
/// unexpected paths (present but not expected — a new or additional regression) and any missing
/// expected paths (expected but no longer present — the tracked divergence narrowed or was fixed,
/// so the allowlist is stale), or `None` if the actual and expected sets match exactly.
fn compare_against_known_divergence(
    path: &Path,
    actual: &std::collections::BTreeSet<&str>,
    expected: &[&str],
) -> Option<String> {
    let expected: std::collections::BTreeSet<&str> = expected.iter().copied().collect();
    let unexpected: Vec<_> = actual.difference(&expected).collect();
    let missing: Vec<_> = expected.difference(actual).collect();
    if unexpected.is_empty() && missing.is_empty() {
        return None;
    }
    let mut parts = Vec::new();
    if !unexpected.is_empty() {
        parts.push(format!(
            "{} new/unexpected diverging path(s) not in KNOWN_DIVERGENCES: {unexpected:?}",
            unexpected.len()
        ));
    }
    if !missing.is_empty() {
        parts.push(format!(
            "{} path(s) listed in KNOWN_DIVERGENCES no longer diverge: {missing:?} \
             (the tracked divergence narrowed or was fixed; tighten or remove the allowlist entry)",
            missing.len()
        ));
    }
    Some(format!("{}: {}", path.display(), parts.join("; ")))
}

#[test]
fn native_and_legacy_layout_agree_on_the_full_parity_corpus() {
    let paths = fixture_paths();
    assert!(
        paths.len() >= 11,
        "expected at least the 11 spike-era fixtures, found {}",
        paths.len()
    );

    let mut mismatches = Vec::new();
    let mut known_divergences_seen = Vec::new();
    for path in &paths {
        let file_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default();
        let known_divergence = KNOWN_DIVERGENCES
            .iter()
            .find(|(name, _)| *name == file_name)
            .map(|(_, paths)| *paths)
            .unwrap_or(&[]);

        let input = fs::read_to_string(path)
            .unwrap_or_else(|error| panic!("read fixture {}: {error}", path.display()));

        match layout_elk_graph_shadow(&input) {
            Ok(result) => {
                let comparison = compare_geometry(
                    &extract_geometry(&result.primary).values,
                    &extract_geometry(&result.native).values,
                    TOLERANCE,
                );
                let actual: std::collections::BTreeSet<&str> = comparison
                    .differences
                    .iter()
                    .map(|diff| diff.path.as_str())
                    .collect();
                if let Some(mismatch) =
                    compare_against_known_divergence(path, &actual, known_divergence)
                {
                    mismatches.push(mismatch);
                } else if !known_divergence.is_empty() {
                    known_divergences_seen.push(format!(
                        "{}: {} tracked diverging scalars (see KNOWN_DIVERGENCES)",
                        path.display(),
                        known_divergence.len()
                    ));
                }
            }
            Err(error) => {
                mismatches.push(format!("{}: shadow layout failed: {error}", path.display()))
            }
        }
    }

    eprintln!(
        "{} known, tracked divergences:\n\n{}",
        known_divergences_seen.len(),
        known_divergences_seen.join("\n\n")
    );
    assert!(
        mismatches.is_empty(),
        "{} of {} fixtures diverged unexpectedly:\n\n{}",
        mismatches.len(),
        paths.len(),
        mismatches.join("\n\n")
    );
}

#[test]
fn compare_against_known_divergence_accepts_an_exact_match() {
    let actual: std::collections::BTreeSet<&str> = ["a/x", "a/y"].into_iter().collect();
    assert_eq!(
        compare_against_known_divergence(Path::new("fixture.json"), &actual, &["a/x", "a/y"]),
        None
    );
}

#[test]
fn compare_against_known_divergence_flags_a_new_unexpected_path() {
    let actual: std::collections::BTreeSet<&str> = ["a/x", "a/y", "b/z"].into_iter().collect();
    let mismatch =
        compare_against_known_divergence(Path::new("fixture.json"), &actual, &["a/x", "a/y"])
            .expect("an extra diverging path must be reported, not silently absorbed");
    assert!(
        mismatch.contains("new/unexpected"),
        "message was: {mismatch}"
    );
    assert!(mismatch.contains("b/z"), "message was: {mismatch}");
}

#[test]
fn compare_against_known_divergence_flags_a_narrowed_divergence() {
    let actual: std::collections::BTreeSet<&str> = ["a/x"].into_iter().collect();
    let mismatch =
        compare_against_known_divergence(Path::new("fixture.json"), &actual, &["a/x", "a/y"])
            .expect("a stale allowlist entry must be reported, not silently ignored");
    assert!(
        mismatch.contains("no longer diverge"),
        "message was: {mismatch}"
    );
    assert!(mismatch.contains("a/y"), "message was: {mismatch}");
}
