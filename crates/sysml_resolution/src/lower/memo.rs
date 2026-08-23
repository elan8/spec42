//! Phase 2's memo: one document's lowering product, keyed by the content that produced it.
//!
//! Lowering reads one parsed document and nothing else, so content is the complete key. The memo
//! is owned by the publication authority and reached only through it: `design.md` — *caches,
//! memos, and stratum reuse are implementation details behind service handles*. A consumer cannot
//! hold, name, or invalidate an entry, and cannot tell a hit from a miss except through the
//! counted facts on [`crate::BuildMeasurements`].
//!
//! **Eviction.** Each build takes a generation and marks every entry it reads or writes with it.
//! When the build's lowering phase finishes it sweeps entries last used by an *older* generation.
//! The memo therefore holds the documents of the most recent build (plus anything a concurrently
//! running newer build has added), not a growing history: a document dropped from the source set
//! is gone one build later, and a superseded revision of an edited document is gone immediately.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, PoisonError};

use source_identity::ContentDigest;
use sysml_v2_parser::ParsedDocument;

use crate::lower::document::{lower_document, LoweredDocument};
use crate::model::ConstructionError;

/// The build-scoped generation a memo marks its entries with.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct MemoGeneration(u64);

#[derive(Debug)]
struct Entry {
    lowered: Arc<LoweredDocument>,
    used: u64,
}

/// The per-document lowering memo shared by every publication of one authority.
#[derive(Debug, Default)]
pub(crate) struct LoweringMemo {
    entries: Mutex<HashMap<ContentDigest, Entry>>,
    next_generation: AtomicU64,
}

impl LoweringMemo {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Opens a build's generation. Every later `lower` call in that build carries it.
    pub(crate) fn begin(&self) -> MemoGeneration {
        MemoGeneration(self.next_generation.fetch_add(1, Ordering::Relaxed))
    }

    /// The product for `digest`, from the memo or from one lowering of `parsed`.
    ///
    /// Returns whether the product was reused. The walk runs outside the lock, so two builds that
    /// miss on the same document lower it twice and store one product rather than serialising;
    /// lowering is a pure function of content, so the two products are equal.
    pub(crate) fn lower(
        &self,
        digest: ContentDigest,
        generation: MemoGeneration,
        parsed: &Arc<ParsedDocument>,
    ) -> Result<(Arc<LoweredDocument>, bool), ConstructionError> {
        if let Some(hit) = self.touch(digest, generation) {
            return Ok((hit, true));
        }
        let lowered = Arc::new(lower_document(Arc::clone(parsed))?);
        let mut entries = self.entries.lock().unwrap_or_else(PoisonError::into_inner);
        let entry = entries.entry(digest).or_insert_with(|| Entry {
            lowered: Arc::clone(&lowered),
            used: generation.0,
        });
        entry.used = entry.used.max(generation.0);
        Ok((Arc::clone(&entry.lowered), false))
    }

    /// Drops every entry not read or written by `generation` or a newer build.
    pub(crate) fn retain(&self, generation: MemoGeneration) {
        let mut entries = self.entries.lock().unwrap_or_else(PoisonError::into_inner);
        entries.retain(|_, entry| entry.used >= generation.0);
    }

    /// How many documents the memo currently holds. Test-only: no consumer may observe this.
    #[cfg(test)]
    pub(crate) fn len(&self) -> usize {
        self.entries
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .len()
    }

    fn touch(
        &self,
        digest: ContentDigest,
        generation: MemoGeneration,
    ) -> Option<Arc<LoweredDocument>> {
        let mut entries = self.entries.lock().unwrap_or_else(PoisonError::into_inner);
        let entry = entries.get_mut(&digest)?;
        entry.used = entry.used.max(generation.0);
        Some(Arc::clone(&entry.lowered))
    }
}
