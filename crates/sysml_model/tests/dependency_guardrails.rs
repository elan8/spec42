#[test]
fn sysml_model_stays_runtime_agnostic() {
    let cargo_toml = include_str!("../Cargo.toml");

    for forbidden in ["tokio", "software-architecture", "lsp_server"] {
        assert!(
            !cargo_toml.contains(forbidden),
            "sysml_model must not depend on runtime crate '{forbidden}'"
        );
    }
}

#[test]
fn graph_builder_does_not_use_ast_expansion_helpers() {
    let part_usage = include_str!("../src/semantic/graph_builder/part_usage.rs");
    let analysis_case = include_str!("../src/semantic/graph_builder/analysis_case.rs");
    let part_def = include_str!("../src/semantic/graph_builder/part_def.rs");
    let package_body = include_str!("../src/semantic/graph_builder/package_body.rs");

    assert!(
        !part_usage.contains("find_part_def_in_root"),
        "part_usage graph builder must not resolve members by walking local AST roots"
    );
    for (name, content) in [
        ("analysis_case", analysis_case),
        ("part_def", part_def),
        ("package_body", package_body),
        ("part_usage", part_usage),
    ] {
        assert!(
            !content.contains("expand_typed_part_usage("),
            "{name} must not mirror typed members under usage nodes"
        );
    }
}
