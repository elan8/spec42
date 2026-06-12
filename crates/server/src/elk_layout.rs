use rquickjs::{Context, Runtime};

#[allow(dead_code)]
const ELK_BUNDLED_JS: &str = include_str!("../assets/elkjs/elk.bundled.js");
const ELK_API_JS: &str = include_str!("../assets/elkjs/elk-api.js");
const ELK_WORKER_JS: &str = include_str!("../assets/elkjs/elk-worker.min.js");

pub fn layout_elk_graph(graph_json: &str) -> Result<String, String> {
    let graph_json = graph_json.to_string();
    std::thread::Builder::new()
        .name("spec42-elkjs-quickjs".to_string())
        .stack_size(32 * 1024 * 1024)
        .spawn(move || layout_elk_graph_inner(&graph_json))
        .map_err(|err| format!("Failed to spawn ELK layout thread: {err}"))?
        .join()
        .map_err(|_| "ELK layout thread panicked".to_string())?
}

fn layout_elk_graph_inner(graph_json: &str) -> Result<String, String> {
    let runtime =
        Runtime::new().map_err(|err| format!("Failed to create QuickJS runtime: {err}"))?;
    let context = Context::full(&runtime)
        .map_err(|err| format!("Failed to create QuickJS context: {err}"))?;
    context.with(|ctx| {
        ctx.eval::<(), _>(
            r#"
            globalThis.window = globalThis;
            globalThis.self = globalThis;
            globalThis.console = globalThis.console || { log: function () {}, warn: function () {}, error: function () {} };
            globalThis.__spec42Timers = [];
            globalThis.setTimeout = function (fn) { globalThis.__spec42Timers.push(fn); return globalThis.__spec42Timers.length; };
            globalThis.clearTimeout = function () {};
            globalThis.__spec42RunTimers = function () {
              var timers = globalThis.__spec42Timers.splice(0, globalThis.__spec42Timers.length);
              for (var i = 0; i < timers.length; i++) {
                timers[i]();
              }
              return timers.length;
            };
            globalThis.performance = globalThis.performance || { now: function () { return Date.now(); } };
            "#,
        )
        .map_err(|err| format!("Failed to initialize ELK JavaScript globals: {err}"))?;
        ctx.eval::<(), _>(format!(
            r#"
            (function () {{
              var module = {{ exports: {{}} }};
              var exports = module.exports;
              var self;
              var document;
              {ELK_WORKER_JS}
              globalThis.__spec42ElkWorkerCtor = module.exports.Worker || module.exports.default || module.exports;
            }})();
            "#
        ))
        .map_err(|err| format!("Failed to evaluate vendored ELK worker: {err}"))?;
        ctx.eval::<(), _>(format!(
            r#"
            (function () {{
              var module = {{ exports: {{}} }};
              var exports = module.exports;
              {ELK_API_JS}
              globalThis.__spec42ElkCtor = module.exports.default || module.exports;
            }})();
            "#
        ))
        .map_err(|err| format!("Failed to evaluate vendored ELK API: {err}"))?;
        ctx.eval::<(), _>(
            r#"
            globalThis.__spec42InitError = "";
            try {
              var Spec42ElkCtor = globalThis.__spec42ElkCtor;
              var Spec42WorkerCtor = globalThis.__spec42ElkWorkerCtor;
              if (typeof Spec42ElkCtor !== "function") {
                throw new Error("Vendored ELK.js did not expose an ELK constructor");
              }
              if (typeof Spec42WorkerCtor !== "function") {
                throw new Error("Vendored ELK.js did not expose a worker constructor");
              }
              globalThis.__spec42Elk = new Spec42ElkCtor({
                workerFactory: function () { return new Spec42WorkerCtor(); }
              });
            } catch (err) {
              globalThis.__spec42InitError = String(err && (err.stack || err.message) || err);
            }
            "#,
        )
        .map_err(|err| format!("Failed to instantiate ELK.js: {err}"))?;
        let init_error: String = ctx
            .eval("globalThis.__spec42InitError")
            .map_err(|err| format!("Failed to read ELK.js initialization status: {err}"))?;
        if !init_error.is_empty() {
            return Err(format!("Failed to instantiate ELK.js: {init_error}"));
        }

        let escaped = serde_json::to_string(graph_json)
            .map_err(|err| format!("Failed to prepare ELK graph JSON for JavaScript: {err}"))?;
        ctx.eval::<(), _>(format!("globalThis.__spec42Input = {escaped};"))
            .map_err(|err| format!("Failed to pass ELK graph JSON to JavaScript: {err}"))?;
        ctx.eval::<(), _>(
            r#"
            globalThis.__spec42Done = "pending";
            globalThis.__spec42Value = "";
            try {
              globalThis.__spec42Elk.layout(JSON.parse(globalThis.__spec42Input)).then(
                function (result) {
                  globalThis.__spec42Done = "ok";
                  globalThis.__spec42Value = JSON.stringify(result);
                },
                function (err) {
                  globalThis.__spec42Done = "err";
                  globalThis.__spec42Value = String(err && (err.stack || err.message) || err);
                }
              );
            } catch (err) {
              globalThis.__spec42Done = "err";
              globalThis.__spec42Value = String(err && (err.stack || err.message) || err);
            }
            "#,
        )
        .map_err(|err| format!("Failed to start ELK layout: {err}"))?;

        for _ in 0..20_000 {
            let done: String = ctx
                .eval("globalThis.__spec42Done")
                .map_err(|err| format!("Failed to read ELK layout status: {err}"))?;
            match done.as_str() {
                "ok" => {
                    return ctx
                        .eval("globalThis.__spec42Value")
                        .map_err(|err| format!("Failed to read ELK layout result: {err}"));
                }
                "err" => {
                    let value: String = ctx
                        .eval("globalThis.__spec42Value")
                        .unwrap_or_else(|_| "unknown ELK.js error".to_string());
                    return Err(format!("ELK layout failed: {value}"));
                }
                _ => {
                    let timers: i32 = ctx
                        .eval("globalThis.__spec42RunTimers()")
                        .map_err(|err| format!("Failed while polling ELK timer jobs: {err}"))?;
                    if timers == 0 && !ctx.execute_pending_job()
                    {
                        return Err("ELK layout did not resolve after QuickJS jobs were drained".to_string());
                    }
                }
            }
        }
        Err("ELK layout did not resolve before the QuickJS polling limit".to_string())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;
    use std::collections::HashMap;
    use std::fs;
    use std::path::PathBuf;

    fn fixture_path(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../shared/diagram-renderer/test-fixtures/interconnection")
            .join(name)
    }

    fn sanitize_elk_id(value: &str) -> String {
        value
            .chars()
            .map(|ch| {
                if ch.is_ascii_alphanumeric() || ch == '_' || ch == '.' || ch == '-' {
                    ch
                } else {
                    '_'
                }
            })
            .collect()
    }

    fn collect_layout_positions(
        node: &Value,
        offset_x: f64,
        offset_y: f64,
        out: &mut HashMap<String, (f64, f64, f64, f64)>,
    ) {
        let x = offset_x + node.get("x").and_then(Value::as_f64).unwrap_or(0.0);
        let y = offset_y + node.get("y").and_then(Value::as_f64).unwrap_or(0.0);
        if let Some(id) = node.get("id").and_then(Value::as_str) {
            if node.get("width").is_some() {
                let width = node.get("width").and_then(Value::as_f64).unwrap_or(0.0);
                let height = node.get("height").and_then(Value::as_f64).unwrap_or(0.0);
                out.insert(id.to_string(), (x, y, width, height));
            }
        }
        if let Some(children) = node.get("children").and_then(Value::as_array) {
            for child in children {
                collect_layout_positions(child, x, y, out);
            }
        }
    }

    #[test]
    fn interconnection_elk_layout_matches_typescript_golden_when_present() {
        for fixture_base in ["scene-two-part-chain", "nested-ring-minimal"] {
            let golden_path = fixture_path(&format!("{fixture_base}-elk-layout.json"));
            let elk_input_path = fixture_path(&format!("{fixture_base}-elk-input.json"));
            if !golden_path.exists() || !elk_input_path.exists() {
                continue;
            }
            let elk_input = fs::read_to_string(elk_input_path).expect("read elk input");
            let layouted_json =
                layout_elk_graph(&elk_input).expect("layout elk input from golden fixture");
            let layouted: Value =
                serde_json::from_str(&layouted_json).expect("parse layouted elk graph");
            let mut rust_positions = HashMap::new();
            if let Some(children) = layouted.get("children").and_then(Value::as_array) {
                for child in children {
                    collect_layout_positions(child, 0.0, 0.0, &mut rust_positions);
                }
            }
            let golden: Vec<Value> =
                serde_json::from_str(&fs::read_to_string(golden_path).expect("read layout golden"))
                    .expect("parse layout golden");
            for entry in golden {
                let prepared_id = entry
                    .get("id")
                    .and_then(Value::as_str)
                    .expect("layout golden id");
                let elk_id = sanitize_elk_id(prepared_id);
                let actual = rust_positions
                    .get(&elk_id)
                    .unwrap_or_else(|| panic!("missing laid-out node {elk_id}"));
                for key in ["x", "y", "width", "height"] {
                    let expected = entry.get(key).and_then(Value::as_f64).unwrap_or(0.0);
                    let got = match key {
                        "x" => actual.0,
                        "y" => actual.1,
                        "width" => actual.2,
                        "height" => actual.3,
                        _ => 0.0,
                    };
                    assert!(
                        (expected - got).abs() <= 2.0,
                        "{fixture_base} {prepared_id}.{key}: expected {expected}, got {got}"
                    );
                }
            }
        }
    }

    #[test]
    fn layouts_tiny_graph() {
        let graph = serde_json::json!({
            "id": "root",
            "layoutOptions": { "elk.algorithm": "layered", "elk.direction": "RIGHT" },
            "children": [
                { "id": "a", "width": 100.0, "height": 40.0 },
                { "id": "b", "width": 100.0, "height": 40.0 }
            ],
            "edges": [{ "id": "e", "sources": ["a"], "targets": ["b"] }]
        });
        let layouted =
            layout_elk_graph(&graph.to_string()).expect("ELK should layout a tiny graph");
        assert!(layouted.contains("\"children\""));
        assert!(layouted.contains("\"sections\""));
    }
}
