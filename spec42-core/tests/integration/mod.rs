//! Integration tests for the LSP server: spawn the binary and drive it over stdio with JSON-RPC.
//!
//! Run with: `cargo test -p spec42 --test lsp_integration`
//!
//! Workspace awareness: `lsp_workspace_scan_goto_definition` uses a temp dir and proves the
//! server loads files from disk (scan). When `SYSML_V2_RELEASE_DIR` is set,
//! `lsp_workspace_scan_sysml_release` runs and validates indexing of the OMG SysML v2 repo.

mod completion;
mod definition;
mod diagnostics;
mod experimental_capabilities;
mod experimental_requests;
mod harness;
mod hover;
mod lifecycle;
mod model;
mod model_graph;
mod references;
mod rename;
mod semantic_tokens;
mod workspace;
