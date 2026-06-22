//! Cold-path profiler entry point for the robot-vacuum embedding host scenario.
//!
//! ```bash
//! export CARGO_TARGET_DIR=/path/to/spec42/target
//! cargo build -p spec42_host --profile profiling --example profile_robot_vacuum
//! target/profiling/examples/profile_robot_vacuum
//! ```

use spec42_host::robot_vacuum_perf::{
    default_matrix_scenarios, emit_perf_report, run_perf_matrix, run_robot_vacuum_perf, PerfConfig,
};

fn main() {
    let cache_root = std::env::temp_dir().join("spec42-robot-vacuum-perf");
    std::fs::create_dir_all(&cache_root).expect("cache root");

    let matrix_mode = std::env::args().any(|arg| arg == "--matrix");
    let no_stdlib = !std::env::args().any(|arg| arg == "--embedded-libs");
    let skip_prepare_view = std::env::args().any(|arg| arg == "--load-only");

    if matrix_mode {
        let release_build = !cfg!(debug_assertions);
        let matrix = run_perf_matrix(&default_matrix_scenarios(release_build), 3, &cache_root);
        let path = emit_perf_report(&matrix, "robot-vacuum-host-matrix.json");
        println!("SPEC42_PERF_REPORT {}", serde_json::to_string(&matrix).expect("json"));
        println!("wrote {}", path.display());
        return;
    }

    let config = PerfConfig {
        label: if no_stdlib {
            "profile_example_no_stdlib".into()
        } else {
            "profile_example_embedded_libs".into()
        },
        no_stdlib,
        include_prepare_view: !skip_prepare_view,
        release_build: !cfg!(debug_assertions),
    };

    let report = run_robot_vacuum_perf(&config, &cache_root);
    let path = emit_perf_report(&report, "robot-vacuum-host-phases.json");
    println!("SPEC42_PERF_REPORT {}", serde_json::to_string(&report).expect("json"));
    println!("wrote {}", path.display());
}
