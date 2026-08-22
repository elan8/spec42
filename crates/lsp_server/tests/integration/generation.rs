use super::harness::TestSession;
use base64::Engine as _;
use spec42_generator_protocol::COMPATIBILITY_TOKEN;

fn empty_generator() -> Vec<u8> {
    let packed_result = 2_u64 << 32 | 1024;
    wat::parse_str(format!(
        r#"(module
          (import "spec42" "query" (func $query (param i32 i32 i32 i32 i32) (result i64)))
          (import "spec42" "diagnostic" (func $diagnostic (param i32 i32 i32 i32 i32)))
          (memory (export "memory") 1)
          (data (i32.const 1024) "\00\00")
          (func (export "spec42_abi_version") (result i64) (i64.const {COMPATIBILITY_TOKEN}))
          (func (export "spec42_alloc") (param i32) (result i32) (i32.const 2048))
          (func (export "spec42_generate") (param i32 i32) (result i64)
            (i64.const {packed_result})))"#
    ))
    .expect("valid guest")
}

#[test]
fn persistent_generation_reuses_prepared_module_for_one_publication() {
    let fixture = tempfile::tempdir().expect("fixture directory");
    let model_path = fixture.path().join("model.sysml");
    std::fs::write(&model_path, "package P { part def Widget; }\n").expect("model");
    let model_uri = url::Url::from_file_path(&model_path)
        .expect("model URI")
        .to_string();

    let mut session = TestSession::new();
    session.initialize_default("persistent-generation-test");
    session.did_open(&model_uri, "package P { part def Widget; }\n", 1);
    session.barrier();

    let params = serde_json::json!({
        "generatorBase64": base64::engine::general_purpose::STANDARD.encode(empty_generator()),
        "modelUri": model_uri,
        "args": []
    });
    let cold = session.request("spec42/generate", params.clone());
    let warm = session.request("spec42/generate", params);
    assert!(cold.get("error").is_none(), "cold response: {cold}");
    assert!(warm.get("error").is_none(), "warm response: {warm}");
    assert_eq!(cold["result"]["modelDigest"], warm["result"]["modelDigest"]);
    assert_eq!(cold["result"]["artifacts"], warm["result"]["artifacts"]);
    assert_eq!(cold["result"]["timings"]["preparedReused"], false);
    assert_eq!(warm["result"]["timings"]["preparedReused"], true);
    assert_eq!(warm["result"]["timings"]["compilationCacheEnabled"], true);
}
