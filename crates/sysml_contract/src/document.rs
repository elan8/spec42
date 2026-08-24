//! The document handle, and the string form it takes when it has to leave the process.
//!
//! A document identity is a URI string, but a *fact* about a document does not need to carry a
//! copy of it. `DocumentId` is the identity every published location addresses its document by:
//! a `Copy`, publication-scoped ordinal the authority mints. Materialising the URI --
//! `PublishedModel::document_identity` for a borrowed one, `document_token` for an owned form
//! that crosses a boundary -- is asked for explicitly, once, by the consumer that needs text.

use std::num::NonZeroU32;

/// A document of one publication.
///
/// # Validity
///
/// **A `DocumentId` is valid for exactly the publication that minted it.** It is a dense ordinal
/// into that publication's document storage, so the same URI in the next publication of the same
/// sources may carry a different one, and an id from an older publication addresses whatever now
/// sits at that ordinal. Holding one across a rebuild is a bug: an out-of-range ordinal answers
/// `None`, but an in-range stale id cannot be detected, and a query for one answers about the
/// wrong document rather than failing.
///
/// To hold a document across publications, take a [`DocumentToken`]: it is the document's
/// normalised identity rather than its storage slot, so it survives a rebuild, and
/// `PublishedModel::resolve_document_token` turns it back into a `DocumentId`.
///
/// The ordinal is opaque. It is not an admission order or anything a consumer may compute with;
/// the `Ord` implementation exists so results can be given a stable order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct DocumentId(NonZeroU32);

impl DocumentId {
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
    /// The inverse of [`DocumentId::from_index`], for the authority that minted it.
    pub fn index(self) -> usize {
        (self.0.get() - 1) as usize
    }
}

/// The stable, serialisable string form of a document handle.
///
/// A [`DocumentId`] is a slot in one publication; a token is the document's normalised identity
/// -- the same URI string `PublishedModel::document_identity` borrows -- and is therefore equal
/// across builds of the same sources. It is what crosses a process or protocol boundary: an LSP
/// DTO, a JSON report, a KPAR entry, the generator protocol.
///
/// A token is obtained from a `PublishedModel` (`document_token`) and turned back into a handle
/// by the same model or a later one (`resolve_document_token`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DocumentToken(Box<str>);

impl DocumentToken {
    /// Wraps an encoding the semantic authority produced.
    pub fn from_encoded(encoded: impl Into<Box<str>>) -> Self {
        Self(encoded.into())
    }

    /// The token's serialisable text: the document's normalised identity.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the token, yielding the owned text a boundary needs.
    pub fn into_string(self) -> String {
        self.0.into_string()
    }
}

impl std::fmt::Display for DocumentToken {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl AsRef<str> for DocumentToken {
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
            let id = DocumentId::from_index(index).expect("index is a handle");
            assert_eq!(id.index(), index);
        }
    }

    #[test]
    fn an_index_beyond_the_dense_range_mints_no_handle() {
        assert!(DocumentId::from_index(u32::MAX as usize).is_none());
    }

    #[test]
    fn a_handle_is_word_sized_and_niche_packed() {
        assert_eq!(std::mem::size_of::<DocumentId>(), 4);
        assert_eq!(std::mem::size_of::<Option<DocumentId>>(), 4);
    }

    #[test]
    fn a_token_is_the_identity_string_itself() {
        let token = DocumentToken::from_encoded("file:///m.sysml");
        assert_eq!(token.as_str(), "file:///m.sysml");
        assert_eq!(token.to_string(), "file:///m.sysml");
        assert_eq!(token.clone().into_string(), "file:///m.sysml");
    }
}
