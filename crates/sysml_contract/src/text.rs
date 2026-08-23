//! The handle a published fact carries in place of a copy of authored text.

use std::num::NonZeroU32;

/// One run of authored text, addressed inside the publication that stores it.
///
/// A documentation body or a unit token is text the publication already interned once. A result
/// that carried a `Box<str>` of it allocated a second copy per element of a bulk answer, for
/// text most consumers only render at an editor or protocol edge. This handle is that slot, and
/// `PublishedModel::text` (or `PublishedResolution::text`) borrows the run back.
///
/// Publication-scoped, exactly like [`SymbolId`](crate::SymbolId): a handle from another
/// publication whose slot is still in range names whatever text now holds that slot. Nothing may
/// persist one across a rebuild. The ordinal is opaque; `Ord` exists so results can be given a
/// stable order, not to express a ranking of the text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct TextId(NonZeroU32);

impl TextId {
    /// Mints the handle for a dense storage index.
    ///
    /// The semantic authority calls this; a consumer never mints a handle, it receives one from
    /// a query. `None` when the index cannot be a handle (`u32::MAX` or beyond).
    pub fn from_index(index: usize) -> Option<Self> {
        let ordinal = u32::try_from(index).ok()?.checked_add(1)?;
        // `checked_add` above rules out zero.
        NonZeroU32::new(ordinal).map(Self)
    }

    /// The dense storage index this handle addresses.
    ///
    /// The inverse of [`TextId::from_index`], for the authority that minted it.
    pub fn index(self) -> usize {
        (self.0.get() - 1) as usize
    }
}
