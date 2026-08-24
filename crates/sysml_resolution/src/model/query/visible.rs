//! Borrowed views over the visible members of a scope.
//!
//! The facade rule is identities, enums and borrowed views -- never owned storage. A completion
//! request asks for every member visible at a keystroke position; materialising a name, a
//! qualified name and a document identity per candidate allocated four strings per element of a
//! result the caller usually filters down to a handful. These views carry the publication and a
//! declaration handle instead, and read every string as a slice of the settled blobs.

use crate::model::element_kind;
use crate::model::resolver::ResolvedSemanticModel;
use crate::model::DeclarationId;
use crate::ElementKind;
use crate::MembershipRole;
use crate::SymbolId;
use crate::TextRange;

/// The members visible at a position, in canonical order.
///
/// Construction settles the order and the membership; nothing here computes afterwards, so
/// repeating a read costs a slice.
#[derive(Clone)]
pub struct VisibleMembers<'m> {
    model: &'m ResolvedSemanticModel,
    /// Only declarations that satisfy every accessor's prerequisite: a settled name, a rank, an
    /// identifier range and a document. The filter is applied once, at construction, which is what
    /// lets the accessors answer without an outcome of their own.
    ids: Box<[DeclarationId]>,
}

impl<'m> VisibleMembers<'m> {
    pub(crate) fn new(model: &'m ResolvedSemanticModel, ids: Box<[DeclarationId]>) -> Self {
        Self { model, ids }
    }

    pub fn len(&self) -> usize {
        self.ids.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<VisibleMemberRef<'m>> {
        let id = *self.ids.get(index)?;
        Some(VisibleMemberRef {
            model: self.model,
            id,
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = VisibleMemberRef<'m>> + 'm {
        let model = self.model;
        let ids = self.ids.clone();
        (0..ids.len()).map(move |index| VisibleMemberRef {
            model,
            id: ids[index],
        })
    }
}

impl std::fmt::Debug for VisibleMembers<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_list().entries(self.iter()).finish()
    }
}

impl PartialEq for VisibleMembers<'_> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.model, other.model) && self.ids == other.ids
    }
}

impl Eq for VisibleMembers<'_> {}

/// One visible member, read from the publication.
#[derive(Clone, Copy)]
pub struct VisibleMemberRef<'m> {
    model: &'m ResolvedSemanticModel,
    id: DeclarationId,
}

impl<'m> VisibleMemberRef<'m> {
    /// The element's handle in this publication.
    pub fn symbol(&self) -> SymbolId {
        self.model
            .symbol_id(self.id)
            .expect("a visible member ranks in its own publication")
    }

    /// The authored name, borrowed from the symbol blob.
    pub fn name(&self) -> &'m str {
        self.model
            .authored_name(self.id)
            .expect("a visible member is named")
    }

    pub fn kind(&self) -> ElementKind {
        element_kind::element_kind(
            self.model
                .declaration_kind(self.id)
                .expect("a visible member is a declaration of this publication"),
        )
    }

    /// The role this member plays in its owner, where the OMG carries that on the owning
    /// membership rather than on the element; `None` for an ordinary member.
    pub fn role(&self) -> Option<MembershipRole> {
        self.model.effective_membership_role(self.id)
    }

    /// The `::`-joined owner path, borrowed from the path blob settled at the barrier.
    pub fn qualified_name(&self) -> &'m str {
        self.model.qualified_name(self.id)
    }

    /// The authored name of the owner, where it has one.
    pub fn container_name(&self) -> Option<&'m str> {
        self.model.declaration_owner_name(self.id)
    }

    /// The document that declares this member.
    pub fn declaring_document(&self) -> &'m str {
        self.model
            .declaration_document_identity(self.id)
            .expect("a visible member belongs to an admitted document")
    }

    pub fn declaration_range(&self) -> TextRange {
        self.model
            .declaration_identifier_range(self.id)
            .expect("a visible member has a settled identifier range")
    }
}

impl std::fmt::Debug for VisibleMemberRef<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("VisibleMember")
            .field("symbol", &self.symbol())
            .field("name", &self.name())
            .field("kind", &self.kind())
            .field("role", &self.role())
            .field("qualified_name", &self.qualified_name())
            .field("container_name", &self.container_name())
            .field("declaring_document", &self.declaring_document())
            .field("declaration_range", &self.declaration_range())
            .finish()
    }
}

impl PartialEq for VisibleMemberRef<'_> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.model, other.model) && self.id == other.id
    }
}

impl Eq for VisibleMemberRef<'_> {}
