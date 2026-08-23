//! The element handle, and the string form it takes when it has to leave the process.
//!
//! An element handle is an identity, not a string. `SymbolId` is that identity: a `Copy`,
//! publication-scoped ordinal the authority mints and every query result carries. Materialising
//! text for one -- a qualified name to show a reader, a token to put in a JSON report -- is a
//! boundary operation a consumer asks `PublishedModel` for explicitly.

use std::num::NonZeroU32;

/// An element of one publication.
///
/// # Validity
///
/// **A `SymbolId` is valid for exactly the publication that minted it.** It is a dense ordinal
/// into that publication's declaration storage, so the same element in the next publication of
/// the same sources may carry a different one, and an id from an older publication addresses
/// whatever now sits at that ordinal. Holding one across a rebuild is a bug: the authority
/// debug-asserts the ordinal is in range, but an in-range stale id cannot be detected, and in
/// release builds a query for one answers about the wrong element rather than failing.
///
/// To hold an element across publications, take a [`SymbolToken`]: it is derived from the
/// element's structure rather than its storage slot, so it survives a rebuild of the same
/// sources, and `PublishedModel::resolve_token` turns it back into a `SymbolId`.
///
/// The ordinal is opaque. It is not a source order, a line number, or anything a consumer may
/// compute with; the `Ord` implementation exists so results can be given a stable order, not to
/// express a semantic ranking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct SymbolId(NonZeroU32);

impl SymbolId {
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
    /// The inverse of [`SymbolId::from_index`], for the authority that minted it.
    pub fn index(self) -> usize {
        (self.0.get() - 1) as usize
    }
}

/// The stable, serialisable string form of an element handle.
///
/// A [`SymbolId`] is a slot in one publication; a token is the element's canonical structural
/// identity, derived from its owner chain, and therefore equal across builds of the same
/// sources. It is what crosses a process or protocol boundary -- an LSP DTO, a JSON report, a
/// KPAR entry, the generator protocol -- and the only form a consumer may persist.
///
/// A token is obtained from a `PublishedModel` (`symbol_token`) and turned back into a handle by
/// the same model or a later one (`resolve_token`). Its contents are opaque: consumers compare,
/// store, and hand tokens back, and never parse one.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SymbolToken(Box<str>);

impl SymbolToken {
    /// Wraps an encoding the semantic authority produced.
    ///
    /// The authority calls this; a consumer obtains a token from `PublishedModel::symbol_token`
    /// or by round-tripping one it stored through [`SymbolToken::from_encoded`].
    pub fn from_encoded(encoded: impl Into<Box<str>>) -> Self {
        Self(encoded.into())
    }

    /// The token's serialisable text.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the token, yielding the owned text a boundary needs.
    pub fn into_string(self) -> String {
        self.0.into_string()
    }
}

impl std::fmt::Display for SymbolToken {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl AsRef<str> for SymbolToken {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_handle_round_trips_its_storage_index() {
        for index in [0usize, 1, 41, u32::MAX as usize - 1] {
            let id = SymbolId::from_index(index).expect("index is a handle");
            assert_eq!(id.index(), index);
        }
    }

    #[test]
    fn an_index_beyond_the_dense_range_mints_no_handle() {
        assert!(SymbolId::from_index(u32::MAX as usize).is_none());
    }

    #[test]
    fn a_handle_is_word_sized_and_niche_packed() {
        assert_eq!(std::mem::size_of::<SymbolId>(), 4);
        assert_eq!(std::mem::size_of::<Option<SymbolId>>(), 4);
    }

    #[test]
    fn a_token_round_trips_its_encoding() {
        let token = SymbolToken::from_encoded("element/v1|4:main");
        assert_eq!(token.as_str(), "element/v1|4:main");
        assert_eq!(token.to_string(), "element/v1|4:main");
        assert_eq!(token.clone().into_string(), "element/v1|4:main");
    }
}
