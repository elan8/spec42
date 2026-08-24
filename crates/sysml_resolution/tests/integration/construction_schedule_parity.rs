//! Construction-schedule parity: the sequential and the parallel schedule build one publication.
//!
//! The authority may distribute lowering across a thread pool (`ConstructionSchedule::Parallel`)
//! or run it on the calling thread (`Sequential`); the schedule is an execution detail that must
//! not be observable. This is an authority invariant, proven here once over the examples corpus
//! and a synthetic library-plus-workspace pair -- not re-proven per fixture by the snapshot tool.
use std::path::{Path, PathBuf};

use sysml_contract::SEMANTIC_CONTRACT_VERSION;
use sysml_resolution::{build, BuildRequest, ConstructionSchedule, PublishedResolution, SourceInput};
use sysml_source::SourceKind;

/// Every rendered projection of a publication, concatenated: the whole observable model.
fn render(publication: &PublishedResolution) -> String {
    let mut output = String::new();
    let debug = publication.debug();
    debug.write_semantic_sexpr(&mut output).expect("semantic");
    debug
        .write_diagnostics_sexpr(&mut output)
        .expect("diagnostics");
    debug
        .write_navigation_sexpr(&mut output)
        .expect("navigation");
    debug.write_types_sexpr(&mut output).expect("types");
    output
}

fn publish(sources: &[(String, String, SourceKind)], schedule: ConstructionSchedule) -> PublishedResolution {
    let inputs = sources
        .iter()
        .map(|(identity, content, kind)| SourceInput::new(identity.clone(), content.clone(), *kind))
        .collect();
    let request = BuildRequest::new(inputs, schedule, SEMANTIC_CONTRACT_VERSION.as_str())
        .expect("valid build request");
    build(request).expect("publication")
}

fn assert_schedules_agree(sources: &[(String, String, SourceKind)]) {
    let sequential = publish(sources, ConstructionSchedule::Sequential);
    let parallel = publish(sources, ConstructionSchedule::Parallel);
    assert_eq!(
        sequential.identity(),
        parallel.identity(),
        "the sequential and parallel schedules publish one identity"
    );
    assert_eq!(
        render(&sequential),
        render(&parallel),
        "the sequential and parallel schedules render identically"
    );
}

fn collect_sources(directory: &Path, into: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(directory) else {
        return;
    };
    for entry in entries.flatten() {
        if entry.file_type().map_or(true, |kind| kind.is_symlink()) {
            continue;
        }
        let path = entry.path();
        if path.is_dir() {
            if path.file_name().is_some_and(|name| name == "target") {
                continue;
            }
            collect_sources(&path, into);
        } else if path.extension().is_some_and(|extension| extension == "sysml") {
            into.push(path);
        }
    }
}

fn examples_corpus() -> Vec<(String, String, SourceKind)> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples");
    let mut paths = Vec::new();
    collect_sources(&root, &mut paths);
    paths.sort();
    assert!(!paths.is_empty(), "no examples under {}", root.display());
    paths
        .into_iter()
        .map(|path| {
            let text = std::fs::read_to_string(&path).expect("readable example");
            let relative = path.strip_prefix(&root).expect("under examples");
            (
                format!("memory://examples/{}", relative.display()),
                text,
                SourceKind::Workspace,
            )
        })
        .collect()
}

#[test]
fn the_examples_corpus_publishes_the_same_under_both_schedules() {
    assert_schedules_agree(&examples_corpus());
}

#[test]
fn a_library_and_workspace_pair_publish_the_same_under_both_schedules() {
    let sources = vec![
        (
            "memory://library/Base.sysml".to_string(),
            "package Base { part def Thing { attribute mass : Real; } attribute def Real; }".to_string(),
            SourceKind::StandardLibrary,
        ),
        (
            "memory://workspace/vehicle.sysml".to_string(),
            "package Vehicle { import Base::*; part def Car :> Thing { attribute redefines mass = 1200; part wheels : Wheel[4]; } part def Wheel :> Thing; }".to_string(),
            SourceKind::Workspace,
        ),
        (
            "memory://workspace/fleet.sysml".to_string(),
            "package Fleet { import Vehicle::*; part fleet : Car[3]; part spare : Wheel; }".to_string(),
            SourceKind::Workspace,
        ),
    ];
    assert_schedules_agree(&sources);
}
