use rquickjs::{Context, Runtime};

const ELK_API_JS: &str = include_str!("../assets/elkjs/elk-api.js");
const ELK_WORKER_JS: &str = include_str!("../assets/elkjs/elk-worker.min.js");
const HEADLESS_RENDERER_JS: &str = include_str!("../assets/diagram-renderer/headless-renderer.js");

pub fn render_shared_svg(payload_json: &str) -> Result<String, String> {
    let payload_json = payload_json.to_string();
    std::thread::Builder::new()
        .name("spec42-headless-diagram-renderer".to_string())
        .stack_size(32 * 1024 * 1024)
        .spawn(move || render_shared_svg_inner(&payload_json))
        .map_err(|err| format!("Failed to spawn headless renderer thread: {err}"))?
        .join()
        .map_err(|_| "Headless renderer thread panicked".to_string())?
}

fn render_shared_svg_inner(payload_json: &str) -> Result<String, String> {
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
        .map_err(|err| format!("Failed to initialize headless renderer globals: {err}"))?;
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
        .map_err(|err| format!("Failed to evaluate vendored ELK worker for headless renderer: {err}"))?;
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
        .map_err(|err| format!("Failed to evaluate vendored ELK API for headless renderer: {err}"))?;
        ctx.eval::<(), _>(HEADLESS_RENDERER_JS)
            .map_err(|err| format!("Failed to evaluate headless renderer bundle: {err:?}"))?;

        let escaped = serde_json::to_string(payload_json)
            .map_err(|err| format!("Failed to prepare diagram payload JSON: {err}"))?;
        ctx.eval::<(), _>(format!("globalThis.__spec42DiagramPayloadJson = {escaped};"))
            .map_err(|err| format!("Failed to pass diagram payload to headless renderer: {err}"))?;
        ctx.eval::<(), _>(
            r#"
            globalThis.__spec42Done = "pending";
            globalThis.__spec42Value = "";
            try {
              var renderer = globalThis.Spec42HeadlessRenderer;
              if (!renderer || typeof renderer.exportHeadlessSvg !== "function") {
                throw new Error("Headless renderer bundle did not expose exportHeadlessSvg");
              }
              renderer.exportHeadlessSvg(JSON.parse(globalThis.__spec42DiagramPayloadJson), {
                colorScheme: "light",
                width: 1280,
                height: 900
              }).then(
                function (svg) {
                  globalThis.__spec42Done = "ok";
                  globalThis.__spec42Value = String(svg);
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
        .map_err(|err| format!("Failed to start headless renderer: {err}"))?;

        for _ in 0..20_000 {
            let done: String = ctx
                .eval("globalThis.__spec42Done")
                .map_err(|err| format!("Failed to read headless renderer status: {err}"))?;
            match done.as_str() {
                "ok" => {
                    return ctx
                        .eval("globalThis.__spec42Value")
                        .map_err(|err| format!("Failed to read headless renderer SVG: {err}"));
                }
                "err" => {
                    let value: String = ctx
                        .eval("globalThis.__spec42Value")
                        .unwrap_or_else(|_| "unknown headless renderer error".to_string());
                    return Err(format!("Headless shared renderer failed: {value}"));
                }
                _ => {
                    let timers: i32 = ctx
                        .eval("globalThis.__spec42RunTimers()")
                        .map_err(|err| format!("Failed while polling renderer timer jobs: {err}"))?;
                    if timers == 0 && !ctx.execute_pending_job() {
                        return Err(
                            "Headless shared renderer did not resolve after QuickJS jobs were drained"
                                .to_string(),
                        );
                    }
                }
            }
        }
        Err("Headless shared renderer did not resolve before the QuickJS polling limit".to_string())
    })
}
