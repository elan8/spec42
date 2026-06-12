# Spec42 for Zed

This directory contains the Zed extension for `spec42`, focused on fast SysML v2 LSP support in Zed.
It is useful if you want lightweight editing, diagnostics, and navigation backed by the same `spec42` language server used by the VS Code extension and CLI.

## Current scope

- `.sysml` language registration
- Tree-sitter grammar ([`tree-sitter-sysml`](https://gitlab.com/nomograph/tree-sitter-sysml)) pinned to `main` (2026 OMG constructs; see `extension.toml`)
- Launching the existing `spec42` language server over stdio
- Automatic download of the matching `spec42` release binary when Zed cannot find one locally
- Forwarding Zed `lsp.spec42` settings to the server

KerML support, snippets, custom queries, Model Explorer, and Model Visualizer features from the VS Code extension are intentionally out of scope for this first pass.

## Best fit

Use the Zed extension when you want:

- a lightweight SysML v2 editing loop in Zed
- LSP-backed diagnostics and navigation
- automatic server-binary discovery or download

Use the VS Code extension when you want the fuller `spec42` experience, including snippets, Model Explorer, Model Visualizer, and richer command integration.

## Develop locally

1. Build the server binary from the repository root:

   ```powershell
   cargo build --release -p spec42
   ```

2. Install the WebAssembly target if needed:

   ```powershell
   rustup target add wasm32-wasip2
   ```

3. In Zed, run `zed: extensions`, choose `Install Dev Extension`, and select this `zed` directory.

4. Either let the extension download the matching `spec42` release automatically, make sure Zed can find `spec42` on `PATH`, or point it at the built binary explicitly in your Zed settings:

   ```json
   {
     "lsp": {
       "spec42": {
         "binary": {
           "path": "C:/Git/spec42/target/release/spec42.exe"
         }
       }
     }
   }
   ```

## Notes

- The extension language name is `SysML v2`.
- The language server id is `spec42`.
- By default the extension prefers a user-configured binary path, then a `spec42` binary on `PATH`, and finally downloads the matching GitHub Release asset for the current platform.
