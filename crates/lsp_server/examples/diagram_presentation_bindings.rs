use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let check = std::env::args()
        .skip(1)
        .any(|argument| argument == "--check");
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../vscode/src/generated/diagram-presentation.ts");
    let source = lsp_server::views::dto::diagram_presentation_typescript();
    if check {
        if std::fs::read_to_string(&path).ok().as_deref() != Some(&source) {
            return Err("diagram presentation bindings are stale; run cargo run -p lsp_server --example diagram_presentation_bindings".into());
        }
    } else {
        std::fs::write(path, source)?;
    }
    Ok(())
}
