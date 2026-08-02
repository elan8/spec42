//! Rust guest SDK for `elan8:spec42-generator@0.1.0`.

pub mod bindings {
    wit_bindgen::generate!({
        path: "wit",
        world: "generator",
        pub_export_macro: true,
    });
}

pub use bindings::elan8::spec42_generator::{artifacts, diagnostics, model};
pub use bindings::{export, Guest};
