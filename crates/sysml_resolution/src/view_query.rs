//! Canonical `ViewUsage::exposedElement` projection.

use crate::SymbolId;

/// The exposed elements of one view after applying all effective view filters.
///
/// `elements` is deduplicated and ordered by canonical symbol identity. Obstacles are retained in
/// canonical order alongside the usable subset, so a consumer can present partial results without
/// describing them as complete.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ViewExposedElements {
    pub view: SymbolId,
    pub elements: Box<[SymbolId]>,
    pub obstacles: Box<[ViewExposureObstacle]>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ViewExposureObstacle {
    ExposureUnresolved {
        exposure: SymbolId,
    },
    ExposureAmbiguous {
        exposure: SymbolId,
        candidates: Box<[SymbolId]>,
    },
    ExposureUnsupported {
        exposure: SymbolId,
    },
    FilterUnresolved {
        candidate: SymbolId,
    },
    FilterAmbiguous {
        candidate: SymbolId,
        predicates: Box<[SymbolId]>,
    },
    FilterUnsupported {
        candidate: SymbolId,
    },
}
