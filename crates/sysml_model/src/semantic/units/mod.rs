//! Unit algebra for the graph expression evaluator.
//!
//! This is a conversion and composition catalog, not a semantic owner. The typed unit facts of a
//! model -- which declarations are units, what symbols they answer to, and what dimension each
//! measures -- are settled by `sysml_resolution` at its publication barrier and read through
//! `sysml_query`'s evaluation service. The graph-derived catalog that used to recover those facts
//! from node names is gone with the diagnostics that consumed it.
//!
//! What remains is the arithmetic the graph evaluator still performs on quantities. It is
//! populated explicitly by its owner, never ingested from a graph.

pub mod registry;

pub use registry::{UnitDef, UnitError, UnitRegistry};
