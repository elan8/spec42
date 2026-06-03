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
