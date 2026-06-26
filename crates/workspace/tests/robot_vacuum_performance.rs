use workspace::robot_vacuum_perf::{
    assert_release_perf_thresholds, assert_release_validation_perf_thresholds,
    default_matrix_scenarios, emit_perf_report, run_perf_matrix, run_robot_vacuum_perf,
    PerfConfig, ValidationPerfMode,
};
use tempfile::tempdir;

#[test]
#[ignore = "local perf: bash scripts/fetch-robot-vacuum-cleaner.sh then cargo test -- --ignored --nocapture"]
fn robot_vacuum_host_phase_performance_report() {
    let cache = tempdir().expect("cache");
    let config = PerfConfig {
        label: "test_single_run".into(),
        no_stdlib: true,
        include_prepare_view: true,
        release_build: !cfg!(debug_assertions),
        validation_mode: ValidationPerfMode::ViewFirst,
    };
    let report = run_robot_vacuum_perf(&config, cache.path());
    let path = emit_perf_report(&report, "robot-vacuum-host-phases.json");
    eprintln!("robot vacuum host perf report: {}", path.display());
    eprintln!(
        "total_ms={} load_ms={} prepare_view_ms={}",
        report.host_phases.total_ms,
        report.host_phases.load_workspace_total_ms,
        report.host_phases.prepare_view_ms
    );
    assert!(report.host_phases.load_workspace_total_ms > 0);
    assert!(report.fixture.files > 0);
    assert_release_perf_thresholds(&report.host_phases);
}

#[test]
#[ignore = "local perf: bash scripts/fetch-robot-vacuum-cleaner.sh then cargo test -- --ignored --nocapture"]
fn robot_vacuum_host_validation_performance_report() {
    let cache = tempdir().expect("cache");
    for mode in [
        ValidationPerfMode::EagerAtLoad,
        ValidationPerfMode::DeferredEnsure,
        ValidationPerfMode::ViewThenValidation,
    ] {
        let config = PerfConfig {
            label: format!("validation_{mode:?}"),
            no_stdlib: true,
            include_prepare_view: false,
            release_build: !cfg!(debug_assertions),
            validation_mode: mode,
        };
        let report = run_robot_vacuum_perf(&config, cache.path());
        eprintln!(
            "validation_mode={mode:?} time_to_completed_validation_ms={} ensure_validation_ms={}",
            report.host_phases.time_to_completed_validation_ms,
            report.host_phases.ensure_validation_ms
        );
        assert!(report.host_phases.time_to_completed_validation_ms > 0);
        assert_release_validation_perf_thresholds(&report.host_phases);
    }
}

#[test]
#[ignore = "local perf matrix: bash scripts/fetch-robot-vacuum-cleaner.sh then cargo test -- --ignored --nocapture"]
fn robot_vacuum_host_performance_matrix_report() {
    let cache = tempdir().expect("cache");
    let release_build = !cfg!(debug_assertions);
    let matrix = run_perf_matrix(&default_matrix_scenarios(release_build), 3, cache.path());
    let path = emit_perf_report(&matrix, "robot-vacuum-host-matrix.json");
    eprintln!("robot vacuum perf matrix: {}", path.display());
    assert_eq!(matrix.scenarios.len(), 6);
    for scenario in &matrix.scenarios {
        assert!(
            scenario.median_host_phases.load_workspace_total_ms > 0,
            "scenario {} produced no load timing",
            scenario.scenario.label
        );
    }
}
