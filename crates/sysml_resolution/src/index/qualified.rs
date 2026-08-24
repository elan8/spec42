//! Phase 6: every declaration's `::`-joined display path, written once into one blob.

use crate::lower::storage::SemanticModelStorage;
use crate::model::DeclarationId;
use crate::resolve::results::ResolutionError;

/// The `::`-joined owner path of every declaration, concatenated into a single buffer.
///
/// A display convenience, not an identity: an anonymous ancestor contributes an empty segment, so
/// two declarations can share a qualified name. `SymbolId` is the identity.
///
/// Each path is its owner's path plus `::` and its own segment, so the whole blob is built in one
/// forward pass -- the storage invariant that an owner precedes its members (checked by
/// `IdentityIndex::build`) is what makes that possible. Consumers borrow a slice of the blob
/// instead of paying an owner-chain walk and a `join` per element of every result.
pub(crate) struct QualifiedNameIndex {
    blob: Box<str>,
    /// `bounds[index]` and `bounds[index + 1]` delimit the path of declaration `index`.
    bounds: Box<[u32]>,
}

impl std::fmt::Debug for QualifiedNameIndex {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("QualifiedNameIndex")
            .field("declarations", &self.bounds.len().saturating_sub(1))
            .field("bytes", &self.blob.len())
            .finish()
    }
}

impl QualifiedNameIndex {
    pub(crate) fn build(storage: &SemanticModelStorage) -> Result<Self, ResolutionError> {
        let declarations = storage.declarations.len();
        let mut blob: Vec<u8> = Vec::new();
        let mut bounds: Vec<u32> = Vec::with_capacity(declarations + 1);
        bounds.push(0);
        for index in 0..declarations {
            let id = DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
            let declaration = storage
                .declaration(id)
                .ok_or(ResolutionError::InvalidStorage)?;
            // An owner always precedes its members in storage order, so its path is already
            // written and can simply be copied ahead of this segment.
            if let Some(owner) = declaration.owner {
                if owner.index() >= index {
                    return Err(ResolutionError::InvalidStorage);
                }
                let start = bounds[owner.index()] as usize;
                let end = bounds[owner.index() + 1] as usize;
                if end > blob.len() || start > end {
                    return Err(ResolutionError::InvalidStorage);
                }
                // Copied in place: the owner's path is already in the buffer, so no per-element
                // temporary is allocated to prepend it.
                blob.extend_from_within(start..end);
                blob.extend_from_slice(b"::");
            }
            let name = declaration
                .name
                .and_then(|name| storage.symbol(name))
                .unwrap_or_default();
            blob.extend_from_slice(name.as_bytes());
            bounds.push(u32::try_from(blob.len()).map_err(|_| ResolutionError::Capacity)?);
        }
        // Every byte pushed above came from a `&str`, and each segment boundary falls between
        // whole segments, so the buffer is valid UTF-8 by construction.
        let blob = String::from_utf8(blob).map_err(|_| ResolutionError::InvalidStorage)?;
        Ok(Self {
            blob: blob.into_boxed_str(),
            bounds: bounds.into_boxed_slice(),
        })
    }

    /// One declaration's display path, borrowed from the blob.
    pub(crate) fn qualified_name(&self, id: DeclarationId) -> Option<&str> {
        let start = *self.bounds.get(id.index())? as usize;
        let end = *self.bounds.get(id.index() + 1)? as usize;
        self.blob.get(start..end)
    }
}
