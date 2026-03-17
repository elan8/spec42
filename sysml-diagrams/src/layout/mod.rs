mod elk_adapter;
mod engine;
mod metrics;
mod svg;
mod types;

pub(crate) use elk_adapter::compute_layout;
pub(crate) use metrics::evaluate;
pub(crate) use svg::render_svg;
pub(crate) use engine::layout_with_report;

// Keep layout input/result types internal to sysml-diagrams for now.
pub(crate) use types::*;

// Public output contract re-exported by crate root.
pub use types::{Bounds, HitRegion, HitRegionKind, LayoutMetrics};

