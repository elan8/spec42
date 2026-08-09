use std::path::Path;

#[test]
fn semantic_evaluation_stays_typed_at_its_ownership_boundary() {
    let evaluation = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/semantic/evaluation");
    for file in ["engine.rs", "mod.rs", "outcome.rs"] {
        let path = evaluation.join(file);
        let source = std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
        for forbidden in [
            "serde_json::Value",
            ".operator.as_deref()",
            "operator: &str",
            "match operator {",
        ] {
            assert!(
                !source.contains(forbidden),
                "{} must not use {forbidden}; evaluator dispatch consumes typed declared facts",
                path.display()
            );
        }
    }
}
