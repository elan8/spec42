#![recursion_limit = "256"]

//! The only supported consumer facade over Spec42's semantic model implementation.
//!
//! [`resolved_slice::PublishedModel`] is opaque. Consumers select a cohesive service and receive
//! typed answers or stream an owner-defined debug projection; they cannot obtain the structural
//! graph, resolver state, fact collections, or query-index storage.

pub mod resolved_slice;
pub mod source;
pub mod syntax;
