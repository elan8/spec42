//! Spec42 server crate.
//!
//! This crate primarily provides the `spec42` binary (LSP server). We keep a small library surface
//! so other crates can reuse default diagram provider wiring.

mod default_diagram_providers;

pub use default_diagram_providers::{default_config, GeneralViewProvider, InterconnectionViewProvider};
