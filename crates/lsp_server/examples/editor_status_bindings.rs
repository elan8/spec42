use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let check = match args.as_slice() {
        [] => false,
        [flag] if flag == "--check" => true,
        _ => {
            return Err(
                "usage: cargo run -p lsp_server --example editor_status_bindings -- [--check]"
                    .into(),
            )
        }
    };
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("workspace root");
    let path = root.join("vscode/src/generated/editor-status.ts");
    let source = lsp_server::views::dto::editor_status_typescript();
    if check {
        if std::fs::read_to_string(&path).ok().as_deref() != Some(&source) {
            return Err("editor status bindings are stale; run cargo run -p lsp_server --example editor_status_bindings".into());
        }
    } else {
        std::fs::write(path, source)?;
    }
    Ok(())
}
