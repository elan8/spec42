#[test]
fn semantic_core_stays_runtime_agnostic() {
    let cargo_toml = include_str!("../Cargo.toml");

    for forbidden in ["tokio", "software-architecture", "kernel"] {
        assert!(
            !cargo_toml.contains(forbidden),
            "semantic_core must not depend on runtime crate '{forbidden}'"
        );
    }
}
