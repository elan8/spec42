//! Direct parser-to-semantic canonicalization storage.
//!
//! Private parser-owned semantic construction.
//!
//! This module deliberately exposes no storage, graph adapter, or independently publishable
//! authored model. The publication owner consumes the typed coordinator outcome below.

use std::{
    collections::{hash_map::RandomState, BTreeMap},
    hash::BuildHasher,
    sync::Arc,
};

use hashbrown::HashTable;
use sysml_v2_parser_next::{
    ast::{
        AttributeBody, AttributeBodyElement, AttributeDef, AttributeUsage, Import, ImportShape,
        LibraryPackage, Membership, MembershipKind as ParserMembershipKind, NamespaceDecl, Node,
        Package, PackageBody, PackageBodyElement, PartDef, PartDefBody, PartDefBodyElement,
        PartUsage, PartUsageBody, PartUsageBodyElement, QualifiedIdentification,
        QualifiedReferenceId, RootElement, Span, SubsettingKind, SubsettingRelationship,
        Visibility as ParserVisibility,
    },
    ParsedDocument,
};

macro_rules! semantic_id {
    ($name:ident) => {
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        struct $name(u32);

        impl $name {
            fn from_index(index: usize) -> Result<Self, ConstructionError> {
                Ok(Self(
                    u32::try_from(index).map_err(|_| ConstructionError::Capacity)?,
                ))
            }

            fn index(self) -> usize {
                self.0 as usize
            }
        }
    };
}

semantic_id!(DocumentId);
semantic_id!(DeclarationId);
semantic_id!(SymbolId);
semantic_id!(SymbolPathId);
semantic_id!(AuthoredReferenceId);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ConstructionError {
    Capacity,
    InvalidIdentity,
    DuplicateDocumentIdentity,
    InvalidParserReference,
    InvalidMembership,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DeclarationKind {
    Namespace,
    Package,
    LibraryPackage,
    PartDefinition,
    PartUsage,
    AttributeDefinition,
    AttributeUsage,
    Import,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MembershipKind {
    Owning,
    Feature,
    Import,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Visibility {
    Default,
    Public,
    Private,
    Protected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum ReferenceKind {
    NamespaceImport,
    MembershipImport,
    FilterImport,
    FeatureTyping,
    Subclassification,
    Subsetting,
    Redefinition,
    References,
    Crosses,
    Intersects,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AuthoredImportShape {
    Membership,
    Namespace,
    Filter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct AuthoredImportFacts {
    shape: AuthoredImportShape,
    recursive: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct RelationshipFlags {
    conjugated: bool,
    implied: bool,
    recursive: bool,
    wildcard: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ParserReferenceId {
    document: DocumentId,
    local: QualifiedReferenceId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UnsupportedFamily {
    PackageMember,
    PartDefinitionMember,
    PartUsageMember,
    AttributeMember,
    ParserUnsupported,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct UnsupportedRecord {
    document: DocumentId,
    family: UnsupportedFamily,
    span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RecoveryRecord {
    document: DocumentId,
    span: Span,
}

#[derive(Debug)]
struct CanonicalDocument {
    identity: Box<str>,
    parsed: Arc<ParsedDocument>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Declaration {
    document: DocumentId,
    owner: Option<DeclarationId>,
    name: Option<SymbolId>,
    kind: DeclarationKind,
    span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MembershipRecord {
    member: DeclarationId,
    kind: MembershipKind,
    visibility: Visibility,
    span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AuthoredReference {
    source: DeclarationId,
    kind: ReferenceKind,
    target: ParserReferenceId,
    path: SymbolPathId,
    ordinal: u32,
    import: Option<AuthoredImportFacts>,
    flags: RelationshipFlags,
    span: Span,
}

#[derive(Debug)]
struct SemanticModelStorage {
    documents: Box<[CanonicalDocument]>,
    declarations: Box<[Declaration]>,
    memberships: Box<[MembershipRecord]>,
    references: Box<[AuthoredReference]>,
    unsupported: Box<[UnsupportedRecord]>,
    recovery: Box<[RecoveryRecord]>,
    symbols: SymbolTable,
    paths: SymbolPathArena,
}

impl SemanticModelStorage {
    fn document(&self, id: DocumentId) -> Option<&CanonicalDocument> {
        self.documents.get(id.index())
    }

    fn declaration(&self, id: DeclarationId) -> Option<&Declaration> {
        self.declarations.get(id.index())
    }

    fn symbol(&self, id: SymbolId) -> Option<&str> {
        self.symbols.get(id)
    }
}

#[derive(Debug, Default)]
struct SemanticModelBuilder {
    documents: Vec<CanonicalDocument>,
    declarations: Vec<Declaration>,
    memberships: Vec<MembershipRecord>,
    references: Vec<AuthoredReference>,
    unsupported: Vec<UnsupportedRecord>,
    recovery: Vec<RecoveryRecord>,
    symbols: SymbolTableBuilder,
    paths: SymbolPathArenaBuilder,
    next_reference_ordinals: BTreeMap<(DeclarationId, ReferenceKind), u32>,
}

impl SemanticModelBuilder {
    fn admit_document(
        &mut self,
        identity: impl Into<Box<str>>,
        parsed: Arc<ParsedDocument>,
    ) -> Result<DocumentId, ConstructionError> {
        let identity = identity.into();
        if self.documents.iter().any(|item| item.identity == identity) {
            return Err(ConstructionError::DuplicateDocumentIdentity);
        }
        let id = DocumentId::from_index(self.documents.len())?;
        self.documents.push(CanonicalDocument { identity, parsed });
        Ok(id)
    }

    fn intern_name(&mut self, value: &str) -> Result<SymbolId, ConstructionError> {
        self.symbols.intern(value)
    }

    fn push_declaration(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        name: Option<SymbolId>,
    ) -> Result<DeclarationId, ConstructionError> {
        self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::Package,
            name,
            Span::dummy(),
        )
    }

    fn push_typed_declaration(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        kind: DeclarationKind,
        name: Option<SymbolId>,
        span: Span,
    ) -> Result<DeclarationId, ConstructionError> {
        if document.index() >= self.documents.len()
            || owner.is_some_and(|id| id.index() >= self.declarations.len())
            || name.is_some_and(|id| id.index() >= self.symbols.len())
        {
            return Err(ConstructionError::InvalidIdentity);
        }
        let id = DeclarationId::from_index(self.declarations.len())?;
        self.declarations.push(Declaration {
            document,
            owner,
            name,
            kind,
            span,
        });
        Ok(id)
    }

    fn push_membership(
        &mut self,
        member: DeclarationId,
        kind: MembershipKind,
        visibility: Visibility,
        span: Span,
    ) -> Result<(), ConstructionError> {
        if member.index() >= self.declarations.len() {
            return Err(ConstructionError::InvalidIdentity);
        }
        self.memberships.push(MembershipRecord {
            member,
            kind,
            visibility,
            span,
        });
        Ok(())
    }

    fn push_reference(
        &mut self,
        source: DeclarationId,
        kind: ReferenceKind,
        document: DocumentId,
        local: QualifiedReferenceId,
        flags: RelationshipFlags,
        span: Span,
        import: Option<AuthoredImportFacts>,
    ) -> Result<AuthoredReferenceId, ConstructionError> {
        if source.index() >= self.declarations.len() || document.index() >= self.documents.len() {
            return Err(ConstructionError::InvalidParserReference);
        }
        let parsed = Arc::clone(&self.documents[document.index()].parsed);
        let reference = parsed
            .qualified_reference(local)
            .ok_or(ConstructionError::InvalidParserReference)?;
        let mut segments = Vec::new();
        segments
            .try_reserve(reference.segments.len())
            .map_err(|_| ConstructionError::Capacity)?;
        for index in 0..reference.segments.len() {
            let decoded = reference
                .segment_decoded_text(index)
                .ok_or(ConstructionError::InvalidParserReference)?;
            segments.push(self.intern_name(decoded.as_ref())?);
        }
        let path = self.paths.push(&segments, reference.metadata.is_absolute)?;
        let ordinal = self
            .next_reference_ordinals
            .entry((source, kind))
            .or_insert(0);
        let authored_ordinal = *ordinal;
        *ordinal = ordinal.checked_add(1).ok_or(ConstructionError::Capacity)?;
        let id = AuthoredReferenceId::from_index(self.references.len())?;
        self.references.push(AuthoredReference {
            source,
            kind,
            target: ParserReferenceId { document, local },
            path,
            ordinal: authored_ordinal,
            import,
            flags,
            span,
        });
        Ok(id)
    }

    fn push_unsupported(&mut self, document: DocumentId, family: UnsupportedFamily, span: Span) {
        self.unsupported.push(UnsupportedRecord {
            document,
            family,
            span,
        });
    }

    fn push_recovery(&mut self, document: DocumentId, span: Span) {
        self.recovery.push(RecoveryRecord { document, span });
    }

    fn freeze(self) -> SemanticModelStorage {
        SemanticModelStorage {
            documents: self.documents.into_boxed_slice(),
            declarations: self.declarations.into_boxed_slice(),
            memberships: self.memberships.into_boxed_slice(),
            references: self.references.into_boxed_slice(),
            unsupported: self.unsupported.into_boxed_slice(),
            recovery: self.recovery.into_boxed_slice(),
            symbols: self.symbols.freeze(),
            paths: self.paths.freeze(),
        }
    }

    fn canonicalize_document(&mut self, document: DocumentId) -> Result<(), ConstructionError> {
        let parsed = Arc::clone(
            &self
                .documents
                .get(document.index())
                .ok_or(ConstructionError::InvalidIdentity)?
                .parsed,
        );
        for element in &parsed.root.elements {
            self.lower_root_element(document, element)?;
        }
        Ok(())
    }

    fn lower_root_element(
        &mut self,
        document: DocumentId,
        element: &Node<RootElement>,
    ) -> Result<(), ConstructionError> {
        match &element.value {
            RootElement::Package(node) => self.lower_package(document, None, node),
            RootElement::LibraryPackage(node) => self.lower_library_package(document, None, node),
            RootElement::Namespace(node) => self.lower_namespace(document, None, node),
            RootElement::Import(node) => self.lower_import(document, None, node),
            RootElement::Member(node) => self.lower_package_element(document, None, node),
        }
    }

    fn lower_package(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<Package>,
    ) -> Result<(), ConstructionError> {
        let name = self.simple_name(&node.identification)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::Package,
            name,
            node.span.clone(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Owning,
            Visibility::Default,
            node.span.clone(),
        )?;
        self.lower_package_body(document, Some(declaration), &node.value.body)
    }

    fn lower_library_package(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<LibraryPackage>,
    ) -> Result<(), ConstructionError> {
        let name = self.simple_name(&node.identification)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::LibraryPackage,
            name,
            node.span.clone(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Owning,
            Visibility::Default,
            node.span.clone(),
        )?;
        self.lower_package_body(document, Some(declaration), &node.value.body)
    }

    fn lower_namespace(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<NamespaceDecl>,
    ) -> Result<(), ConstructionError> {
        let name = self.simple_name(&node.identification)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::Namespace,
            name,
            node.span.clone(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Owning,
            Visibility::Default,
            node.span.clone(),
        )?;
        self.lower_package_body(document, Some(declaration), &node.value.body)
    }

    fn simple_name(
        &mut self,
        identification: &QualifiedIdentification,
    ) -> Result<Option<SymbolId>, ConstructionError> {
        identification
            .simple_name()
            .map(|name| self.intern_name(name))
            .transpose()
    }

    fn lower_package_body(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        body: &PackageBody,
    ) -> Result<(), ConstructionError> {
        if let PackageBody::Brace { elements } = body {
            for element in elements {
                self.lower_package_element(document, owner, element)?;
            }
        }
        Ok(())
    }

    fn lower_package_element(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        element: &Node<PackageBodyElement>,
    ) -> Result<(), ConstructionError> {
        match &element.value {
            PackageBodyElement::Error(node) => {
                self.push_recovery(document, node.span.clone());
            }
            PackageBodyElement::Unsupported(node) => {
                self.push_unsupported(
                    document,
                    UnsupportedFamily::ParserUnsupported,
                    node.span.clone(),
                );
            }
            PackageBodyElement::Doc(_)
            | PackageBodyElement::Comment(_)
            | PackageBodyElement::TextualRep(_) => {}
            PackageBodyElement::Filter(node) => {
                self.push_unsupported(
                    document,
                    UnsupportedFamily::PackageMember,
                    node.span.clone(),
                );
            }
            PackageBodyElement::Package(node) => self.lower_package(document, owner, node)?,
            PackageBodyElement::LibraryPackage(node) => {
                self.lower_library_package(document, owner, node)?
            }
            PackageBodyElement::Import(node) => self.lower_import(document, owner, node)?,
            PackageBodyElement::PartDef(node) => self.lower_part_def(document, owner, node)?,
            PackageBodyElement::PartUsage(node) => self.lower_part_usage(document, owner, node)?,
            PackageBodyElement::AttributeUsage(node) => {
                self.lower_attribute_usage(document, owner, node)?
            }
            PackageBodyElement::PortDef(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::InterfaceDef(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::AliasDef(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::AttributeDef(node) => {
                self.lower_attribute_def(document, owner, node)?
            }
            PackageBodyElement::ActionDef(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::ActionUsage(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::RequirementDef(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::RequirementUsage(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::Satisfy(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::UseCaseDef(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::Actor(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::StateDef(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::StateUsage(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::ItemDef(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::IndividualDef(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::ConstraintDef(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::ConstraintUsage(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::CalcDef(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::ViewDef(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::ViewpointDef(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::RenderingDef(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::ViewUsage(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::ViewpointUsage(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::RenderingUsage(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::ConnectionDef(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::MetadataDef(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::MetadataUsage(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::EnumDef(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::OccurrenceDef(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::OccurrenceUsage(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::Dependency(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::AllocationDef(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::AllocationUsage(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::FlowDef(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::FlowUsage(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::ConcernUsage(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::CaseDef(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::CaseUsage(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::AnalysisCaseDef(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::AnalysisCaseUsage(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::VerificationCaseDef(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::VerificationCaseUsage(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::UseCaseUsage(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::FeatureDecl(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::ClassifierDecl(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::KermlSemanticDecl(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::KermlFeatureDecl(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::ExtendedLibraryDecl(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::ItemUsage(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::PortUsage(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::ConnectionUsage(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::InterfaceUsage(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::Ref(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::EnumerationUsage(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::MetadataKeywordUsage(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::Connect(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::DefaultReferenceUsage(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::AssertConstraint(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
        }
        Ok(())
    }

    fn lower_import(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<Import>,
    ) -> Result<(), ConstructionError> {
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::Import,
            None,
            node.span.clone(),
        )?;
        let membership = &node.value.membership;
        self.push_membership(
            declaration,
            MembershipKind::Import,
            self.member_visibility(membership, ParserMembershipKind::Import)?,
            membership.span.clone(),
        )?;
        let (kind, flags) = match &node.value.target.shape {
            ImportShape::Membership { recursive_suffix } => (
                ReferenceKind::MembershipImport,
                RelationshipFlags {
                    recursive: recursive_suffix.is_some(),
                    ..RelationshipFlags::default()
                },
            ),
            ImportShape::Namespace {
                recursive_suffix, ..
            } => (
                ReferenceKind::NamespaceImport,
                RelationshipFlags {
                    recursive: recursive_suffix.is_some(),
                    wildcard: true,
                    ..RelationshipFlags::default()
                },
            ),
            ImportShape::Filter {
                recursive_suffix, ..
            } => (
                ReferenceKind::FilterImport,
                RelationshipFlags {
                    recursive: recursive_suffix.is_some(),
                    ..RelationshipFlags::default()
                },
            ),
        };
        let import = Some(AuthoredImportFacts {
            shape: match &node.value.target.shape {
                ImportShape::Membership { .. } => AuthoredImportShape::Membership,
                ImportShape::Namespace { .. } => AuthoredImportShape::Namespace,
                ImportShape::Filter { .. } => AuthoredImportShape::Filter,
            },
            recursive: flags.recursive,
        });
        self.push_reference(
            declaration,
            kind,
            document,
            node.value.target.reference,
            flags,
            node.value.target.span.clone(),
            import,
        )?;
        Ok(())
    }

    fn lower_part_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<PartDef>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .identification
            .name
            .as_deref()
            .map(|name| self.intern_name(name))
            .transpose()?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::PartDefinition,
            name,
            node.span.clone(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Owning,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::OwningMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(relationship) = &node.value.specializes {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        if let PartDefBody::Brace { elements } = &node.value.body {
            for element in elements {
                match &element.value {
                    PartDefBodyElement::Error(error) => {
                        self.push_recovery(document, error.span.clone());
                    }
                    PartDefBodyElement::AttributeDef(attribute) => {
                        self.lower_attribute_def(document, Some(declaration), attribute)?;
                    }
                    PartDefBodyElement::AttributeUsage(attribute) => {
                        self.lower_attribute_usage(document, Some(declaration), attribute)?;
                    }
                    PartDefBodyElement::PartUsage(part) => {
                        self.lower_part_usage(document, Some(declaration), part)?;
                    }
                    PartDefBodyElement::PartDef(part) => {
                        self.lower_part_def(document, Some(declaration), part)?;
                    }
                    PartDefBodyElement::Import(import) => {
                        self.lower_import(document, Some(declaration), import)?;
                    }
                    PartDefBodyElement::Doc(_) | PartDefBodyElement::Comment(_) => {}
                    PartDefBodyElement::Annotation(_)
                    | PartDefBodyElement::MetadataAnnotation(_)
                    | PartDefBodyElement::MetadataKeywordUsage(_)
                    | PartDefBodyElement::Dependency(_)
                    | PartDefBodyElement::Other(_)
                    | PartDefBodyElement::DefaultReferenceUsage(_)
                    | PartDefBodyElement::RequirementUsage(_)
                    | PartDefBodyElement::ItemDef(_)
                    | PartDefBodyElement::ItemUsage(_)
                    | PartDefBodyElement::Ref(_)
                    | PartDefBodyElement::PortUsage(_)
                    | PartDefBodyElement::OccurrenceUsage(_)
                    | PartDefBodyElement::InterfaceDef(_)
                    | PartDefBodyElement::InterfaceUsage(_)
                    | PartDefBodyElement::Connect(_)
                    | PartDefBodyElement::FlowUsage(_)
                    | PartDefBodyElement::Connection(_)
                    | PartDefBodyElement::Perform(_)
                    | PartDefBodyElement::Allocate(_)
                    | PartDefBodyElement::ExhibitState(_)
                    | PartDefBodyElement::CalcUsage(_)
                    | PartDefBodyElement::ConstraintDef(_)
                    | PartDefBodyElement::ConstraintUsage(_)
                    | PartDefBodyElement::ActionUsage(_)
                    | PartDefBodyElement::ActionDef(_)
                    | PartDefBodyElement::StateUsage(_)
                    | PartDefBodyElement::EnumerationUsage(_)
                    | PartDefBodyElement::AssertConstraint(_)
                    | PartDefBodyElement::Satisfy(_)
                    | PartDefBodyElement::VariantUsage(_)
                    | PartDefBodyElement::StateDef(_)
                    | PartDefBodyElement::MetadataDef(_)
                    | PartDefBodyElement::MetadataUsage(_)
                    | PartDefBodyElement::FlowDef(_)
                    | PartDefBodyElement::RequirementDef(_)
                    | PartDefBodyElement::OccurrenceDef(_)
                    | PartDefBodyElement::ConnectionDef(_)
                    | PartDefBodyElement::PortDef(_)
                    | PartDefBodyElement::CalcDef(_)
                    | PartDefBodyElement::EnumDef(_)
                    | PartDefBodyElement::AllocationDef(_)
                    | PartDefBodyElement::AllocationUsage(_)
                    | PartDefBodyElement::ViewDef(_)
                    | PartDefBodyElement::ViewUsage(_)
                    | PartDefBodyElement::ViewpointDef(_)
                    | PartDefBodyElement::ViewpointUsage(_)
                    | PartDefBodyElement::RenderingDef(_)
                    | PartDefBodyElement::RenderingUsage(_)
                    | PartDefBodyElement::CaseDef(_)
                    | PartDefBodyElement::CaseUsage(_)
                    | PartDefBodyElement::UseCaseDef(_)
                    | PartDefBodyElement::UseCaseUsage(_)
                    | PartDefBodyElement::AnalysisCaseDef(_)
                    | PartDefBodyElement::AnalysisCaseUsage(_)
                    | PartDefBodyElement::VerificationCaseDef(_)
                    | PartDefBodyElement::VerificationCaseUsage(_)
                    | PartDefBodyElement::FirstStmt(_)
                    | PartDefBodyElement::Bind(_)
                    | PartDefBodyElement::AliasDef(_) => self.push_unsupported(
                        document,
                        UnsupportedFamily::PartDefinitionMember,
                        element.span.clone(),
                    ),
                    PartDefBodyElement::UnsupportedMember(node) => self.push_unsupported(
                        document,
                        UnsupportedFamily::ParserUnsupported,
                        node.span.clone(),
                    ),
                }
            }
        }
        Ok(())
    }

    fn lower_part_usage(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<PartUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::PartUsage,
            Some(name),
            node.span.clone(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::FeatureMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(relationship) = &node.value.typing {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        if let Some((relationship, _)) = &node.value.subsets {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.redefines {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let PartUsageBody::Brace { elements } = &node.value.body {
            for element in elements {
                match &element.value {
                    PartUsageBodyElement::Error(error) => {
                        self.push_recovery(document, error.span.clone());
                    }
                    PartUsageBodyElement::AttributeUsage(attribute) => {
                        self.lower_attribute_usage(document, Some(declaration), attribute)?;
                    }
                    PartUsageBodyElement::PartUsage(part) => {
                        self.lower_part_usage(document, Some(declaration), part)?;
                    }
                    PartUsageBodyElement::Import(import) => {
                        self.lower_import(document, Some(declaration), import)?;
                    }
                    PartUsageBodyElement::Doc(_) => {}
                    PartUsageBodyElement::Annotation(_)
                    | PartUsageBodyElement::DefaultReferenceUsage(_)
                    | PartUsageBodyElement::EnumerationUsage(_)
                    | PartUsageBodyElement::OccurrenceUsage(_)
                    | PartUsageBodyElement::PortUsage(_)
                    | PartUsageBodyElement::Bind(_)
                    | PartUsageBodyElement::Ref(_)
                    | PartUsageBodyElement::InterfaceUsage(_)
                    | PartUsageBodyElement::Connect(_)
                    | PartUsageBodyElement::FlowUsage(_)
                    | PartUsageBodyElement::Perform(_)
                    | PartUsageBodyElement::SuccessionUsage(_)
                    | PartUsageBodyElement::Allocate(_)
                    | PartUsageBodyElement::Satisfy(_)
                    | PartUsageBodyElement::StateUsage(_)
                    | PartUsageBodyElement::ActionUsage(_)
                    | PartUsageBodyElement::MetadataAnnotation(_)
                    | PartUsageBodyElement::MetadataKeywordUsage(_)
                    | PartUsageBodyElement::VariantUsage(_)
                    | PartUsageBodyElement::StateDef(_)
                    | PartUsageBodyElement::MetadataDef(_)
                    | PartUsageBodyElement::FlowDef(_)
                    | PartUsageBodyElement::RequirementDef(_)
                    | PartUsageBodyElement::OccurrenceDef(_)
                    | PartUsageBodyElement::PortDef(_)
                    | PartUsageBodyElement::CalcDef(_)
                    | PartUsageBodyElement::ConnectionDef(_)
                    | PartUsageBodyElement::EnumDef(_)
                    | PartUsageBodyElement::Connection(_)
                    | PartUsageBodyElement::AssertConstraint(_)
                    | PartUsageBodyElement::ConstraintDef(_)
                    | PartUsageBodyElement::ConstraintUsage(_)
                    | PartUsageBodyElement::CalcUsage(_)
                    | PartUsageBodyElement::RequirementUsage(_)
                    | PartUsageBodyElement::ItemDef(_)
                    | PartUsageBodyElement::ItemUsage(_)
                    | PartUsageBodyElement::MetadataUsage(_)
                    | PartUsageBodyElement::AnalysisCaseDef(_)
                    | PartUsageBodyElement::AnalysisCaseUsage(_)
                    | PartUsageBodyElement::AliasDef(_)
                    | PartUsageBodyElement::IncludeUseCase(_)
                    | PartUsageBodyElement::UseCaseUsage(_)
                    | PartUsageBodyElement::VerificationCaseUsage(_) => self.push_unsupported(
                        document,
                        UnsupportedFamily::PartUsageMember,
                        element.span.clone(),
                    ),
                }
            }
        }
        Ok(())
    }

    fn lower_attribute_usage(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<AttributeUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::AttributeUsage,
            Some(name),
            node.span.clone(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::FeatureMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(relationship) = &node.value.typing {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.subsets {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.redefines {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.references {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.crosses {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.intersects {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        self.lower_attribute_body(document, declaration, &node.value.body)?;
        Ok(())
    }

    fn lower_attribute_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<AttributeDef>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::AttributeDefinition,
            Some(name),
            node.span.clone(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Owning,
            self.member_visibility(
                &node.value.membership,
                ParserMembershipKind::OwningMembership,
            )?,
            node.value.membership.span.clone(),
        )?;
        if let Some(relationship) = &node.value.typing {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        self.lower_attribute_body(document, declaration, &node.value.body)
    }

    fn lower_attribute_body(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        body: &AttributeBody,
    ) -> Result<(), ConstructionError> {
        let AttributeBody::Brace { elements } = body else {
            return Ok(());
        };
        for element in elements {
            match &element.value {
                AttributeBodyElement::Error(error) => {
                    self.push_recovery(document, error.span.clone());
                }
                AttributeBodyElement::Doc(_) => {}
                AttributeBodyElement::AttributeDef(attribute) => {
                    self.lower_attribute_def(document, Some(owner), attribute)?;
                }
                AttributeBodyElement::AttributeUsage(attribute) => {
                    self.lower_attribute_usage(document, Some(owner), attribute)?;
                }
                AttributeBodyElement::PartUsage(part) => {
                    self.lower_part_usage(document, Some(owner), part)?;
                }
                AttributeBodyElement::Connect(_)
                | AttributeBodyElement::MetadataKeywordUsage(_)
                | AttributeBodyElement::AssertConstraint(_)
                | AttributeBodyElement::RefDecl(_)
                | AttributeBodyElement::OccurrenceUsage(_)
                | AttributeBodyElement::Other(_) => self.push_unsupported(
                    document,
                    UnsupportedFamily::AttributeMember,
                    element.span.clone(),
                ),
            }
        }
        Ok(())
    }

    fn lower_subsetting_relationship(
        &mut self,
        document: DocumentId,
        source: DeclarationId,
        relationship: &Node<SubsettingRelationship>,
    ) -> Result<(), ConstructionError> {
        let kind = match relationship.value.kind {
            SubsettingKind::Subsets => ReferenceKind::Subsetting,
            SubsettingKind::References => ReferenceKind::References,
            SubsettingKind::Redefines => ReferenceKind::Redefinition,
            SubsettingKind::Crosses => ReferenceKind::Crosses,
            SubsettingKind::Intersects => ReferenceKind::Intersects,
        };
        for target in relationship.value.target.iter().copied() {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(target)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            self.push_reference(
                source,
                kind,
                document,
                target,
                RelationshipFlags {
                    implied: relationship.value.is_implied,
                    ..RelationshipFlags::default()
                },
                span,
                None,
            )?;
        }
        Ok(())
    }

    fn lower_typing_relationship(
        &mut self,
        document: DocumentId,
        source: DeclarationId,
        relationship: &Node<sysml_v2_parser_next::ast::TypingRelationship>,
    ) -> Result<(), ConstructionError> {
        let kind = match relationship.value.kind {
            sysml_v2_parser_next::ast::TypingKind::Typing => ReferenceKind::FeatureTyping,
            sysml_v2_parser_next::ast::TypingKind::Subclassification => {
                ReferenceKind::Subclassification
            }
        };
        for target in relationship.value.target.iter().copied() {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(target)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            self.push_reference(
                source,
                kind,
                document,
                target,
                RelationshipFlags {
                    conjugated: relationship.value.is_conjugated,
                    implied: relationship.value.is_implied,
                    ..RelationshipFlags::default()
                },
                span,
                None,
            )?;
        }
        Ok(())
    }

    fn member_visibility(
        &self,
        membership: &Membership,
        expected: ParserMembershipKind,
    ) -> Result<Visibility, ConstructionError> {
        let actual = match membership.kind {
            ParserMembershipKind::OwningMembership => ParserMembershipKind::OwningMembership,
            ParserMembershipKind::FeatureMembership => ParserMembershipKind::FeatureMembership,
            ParserMembershipKind::Import => ParserMembershipKind::Import,
            ParserMembershipKind::Alias => ParserMembershipKind::Alias,
            ParserMembershipKind::VariantMembership => ParserMembershipKind::VariantMembership,
            ParserMembershipKind::ActorMembership => ParserMembershipKind::ActorMembership,
        };
        if actual != expected {
            return Err(ConstructionError::InvalidMembership);
        }
        Ok(membership
            .visibility
            .map(Self::visibility)
            .unwrap_or(Visibility::Default))
    }

    fn visibility(value: ParserVisibility) -> Visibility {
        match value {
            ParserVisibility::Public => Visibility::Public,
            ParserVisibility::Private => Visibility::Private,
            ParserVisibility::Protected => Visibility::Protected,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SymbolPath {
    start: u32,
    len: u32,
    rooted: bool,
}

#[derive(Debug)]
struct SymbolPathArena {
    paths: Box<[SymbolPath]>,
    segments: Box<[SymbolId]>,
}

impl SymbolPathArena {
    fn get(&self, id: SymbolPathId) -> Option<(&[SymbolId], bool)> {
        let path = self.paths.get(id.index())?;
        if path.len == 0 {
            return None;
        }
        let end = path.start.checked_add(path.len)?;
        let segments = self.segments.get(path.start as usize..end as usize)?;
        Some((segments, path.rooted))
    }
}

#[derive(Debug, Default)]
struct SymbolPathArenaBuilder {
    paths: Vec<SymbolPath>,
    segments: Vec<SymbolId>,
}

impl SymbolPathArenaBuilder {
    fn push(
        &mut self,
        segments: &[SymbolId],
        rooted: bool,
    ) -> Result<SymbolPathId, ConstructionError> {
        if segments.is_empty() {
            return Err(ConstructionError::InvalidParserReference);
        }
        let id = SymbolPathId::from_index(self.paths.len())?;
        let start = u32::try_from(self.segments.len()).map_err(|_| ConstructionError::Capacity)?;
        let len = u32::try_from(segments.len()).map_err(|_| ConstructionError::Capacity)?;
        start.checked_add(len).ok_or(ConstructionError::Capacity)?;
        self.paths
            .try_reserve(1)
            .map_err(|_| ConstructionError::Capacity)?;
        self.segments
            .try_reserve(segments.len())
            .map_err(|_| ConstructionError::Capacity)?;
        self.segments.extend_from_slice(segments);
        self.paths.push(SymbolPath { start, len, rooted });
        Ok(id)
    }

    fn freeze(self) -> SymbolPathArena {
        SymbolPathArena {
            paths: self.paths.into_boxed_slice(),
            segments: self.segments.into_boxed_slice(),
        }
    }
}

#[derive(Debug)]
struct SymbolTable {
    bytes: Box<str>,
    spans: Box<[(u32, u32)]>,
}

impl SymbolTable {
    fn get(&self, id: SymbolId) -> Option<&str> {
        let (start, len) = *self.spans.get(id.index())?;
        let end = start.checked_add(len)?;
        self.bytes.get(start as usize..end as usize)
    }
}

#[derive(Debug, Default)]
struct SymbolTableBuilder {
    bytes: String,
    spans: Vec<(u32, u32)>,
    index: HashTable<SymbolId>,
    hash_builder: RandomState,
}

impl SymbolTableBuilder {
    fn len(&self) -> usize {
        self.spans.len()
    }

    fn get(&self, id: SymbolId) -> &str {
        let (start, len) = self.spans[id.index()];
        &self.bytes[start as usize..(start + len) as usize]
    }

    fn intern(&mut self, value: &str) -> Result<SymbolId, ConstructionError> {
        let hash = self.hash_builder.hash_one(value);
        if let Some(existing) = self.index.find(hash, |id| self.get(*id) == value) {
            return Ok(*existing);
        }

        let id = SymbolId::from_index(self.spans.len())?;
        let start = u32::try_from(self.bytes.len()).map_err(|_| ConstructionError::Capacity)?;
        let len = u32::try_from(value.len()).map_err(|_| ConstructionError::Capacity)?;
        start.checked_add(len).ok_or(ConstructionError::Capacity)?;
        self.bytes
            .try_reserve(value.len())
            .map_err(|_| ConstructionError::Capacity)?;
        self.spans
            .try_reserve(1)
            .map_err(|_| ConstructionError::Capacity)?;
        self.index
            .try_reserve(1, |_| 0)
            .map_err(|_| ConstructionError::Capacity)?;

        self.bytes.push_str(value);
        self.spans.push((start, len));
        let bytes = &self.bytes;
        let spans = &self.spans;
        let hash_builder = &self.hash_builder;
        self.index.insert_unique(hash, id, |candidate| {
            let (candidate_start, candidate_len) = spans[candidate.index()];
            hash_builder.hash_one(
                &bytes[candidate_start as usize..(candidate_start + candidate_len) as usize],
            )
        });
        Ok(id)
    }

    fn freeze(self) -> SymbolTable {
        SymbolTable {
            bytes: self.bytes.into_boxed_str(),
            spans: self.spans.into_boxed_slice(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct OwnedSourceRecord {
    pub(crate) identity: Box<str>,
    pub(crate) content: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BuildSchedule {
    Sequential,
    Parallel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CoordinatorError {
    DuplicateSourceIdentity,
    ParseFailed,
    ConstructionFailed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CoordinatorIncomplete {
    ParseRecovery,
    UnsupportedSyntax,
    NonConverged,
}

#[derive(Debug)]
pub(crate) enum CoordinatorOutcome {
    Complete(resolver::ResolvedSemanticModel),
    Incomplete(CoordinatorIncomplete),
}

pub(crate) struct SemanticModelBuildCoordinator;

pub(crate) fn write_resolved_debug(
    model: &resolver::ResolvedSemanticModel,
    output: &mut dyn std::fmt::Write,
) -> std::fmt::Result {
    model.write_debug_sexpr(output)
}

impl SemanticModelBuildCoordinator {
    pub(crate) fn build(
        mut sources: Vec<OwnedSourceRecord>,
        schedule: BuildSchedule,
    ) -> Result<CoordinatorOutcome, CoordinatorError> {
        sources.sort_unstable_by(|left, right| left.identity.cmp(&right.identity));
        if sources
            .windows(2)
            .any(|pair| pair[0].identity == pair[1].identity)
        {
            return Err(CoordinatorError::DuplicateSourceIdentity);
        }

        let parsed = match schedule {
            BuildSchedule::Sequential => sources
                .into_iter()
                .map(Self::parse_source)
                .collect::<Result<Vec<_>, _>>()?,
            BuildSchedule::Parallel => {
                use rayon::prelude::*;
                sources
                    .into_par_iter()
                    .map(Self::parse_source)
                    .collect::<Result<Vec<_>, _>>()?
            }
        };

        let mut builder = SemanticModelBuilder::default();
        let mut documents = Vec::with_capacity(parsed.len());
        for (identity, parsed) in parsed {
            let document = builder
                .admit_document(identity, Arc::new(parsed))
                .map_err(|_| CoordinatorError::DuplicateSourceIdentity)?;
            documents.push(document);
        }
        for document in documents {
            builder
                .canonicalize_document(document)
                .map_err(|_| CoordinatorError::ConstructionFailed)?;
        }
        if !builder.recovery.is_empty() {
            return Ok(CoordinatorOutcome::Incomplete(
                CoordinatorIncomplete::ParseRecovery,
            ));
        }
        if !builder.unsupported.is_empty() {
            return Ok(CoordinatorOutcome::Incomplete(
                CoordinatorIncomplete::UnsupportedSyntax,
            ));
        }
        let model = builder
            .freeze()
            .resolve()
            .map_err(|_| CoordinatorError::ConstructionFailed)?;
        if !model.is_converged() {
            return Ok(CoordinatorOutcome::Incomplete(
                CoordinatorIncomplete::NonConverged,
            ));
        }
        Ok(CoordinatorOutcome::Complete(model))
    }

    fn parse_source(
        source: OwnedSourceRecord,
    ) -> Result<(Box<str>, ParsedDocument), CoordinatorError> {
        let parsed = sysml_v2_parser_next::parse_owned(source.content)
            .map_err(|_| CoordinatorError::ParseFailed)?;
        Ok((source.identity, parsed))
    }
}

pub(crate) mod resolver;

#[cfg(test)]
mod tests {
    use super::*;
    use sysml_v2_parser_next::ast::{QualifiedReferenceArena, RootNamespace, SourceStorage};

    fn empty_document() -> Arc<ParsedDocument> {
        Arc::new(ParsedDocument {
            source: SourceStorage::default(),
            qualified_references: QualifiedReferenceArena::default(),
            root: RootNamespace {
                elements: Vec::new(),
            },
        })
    }

    #[test]
    fn canonicalization_assigns_dense_typed_slots_and_interns_names() {
        let mut builder = SemanticModelBuilder::default();
        let parsed = empty_document();
        let document = builder.admit_document("model", parsed.clone()).unwrap();
        let first_name = builder.intern_name("Vehicle").unwrap();
        let second_name = builder.intern_name("Vehicle").unwrap();
        assert_eq!(first_name, second_name);
        let root = builder
            .push_declaration(document, None, Some(first_name))
            .unwrap();
        let child = builder
            .push_declaration(document, Some(root), Some(second_name))
            .unwrap();

        let model = builder.freeze();
        assert_eq!(model.document(document).unwrap().identity.as_ref(), "model");
        assert!(Arc::ptr_eq(
            &model.document(document).unwrap().parsed,
            &parsed
        ));
        assert_eq!(model.declaration(root).unwrap().owner, None);
        assert_eq!(model.declaration(child).unwrap().owner, Some(root));
        assert_eq!(model.symbol(first_name), Some("Vehicle"));
        assert_eq!(model.symbols.spans.len(), 1);
    }

    #[test]
    fn foreign_typed_ids_are_rejected_before_mutation() {
        let mut builder = SemanticModelBuilder::default();
        let invalid_document = DocumentId(0);
        let name = builder.intern_name("Vehicle").unwrap();
        let error = builder
            .push_declaration(invalid_document, None, Some(name))
            .unwrap_err();
        assert_eq!(error, ConstructionError::InvalidIdentity);
        assert!(builder.declarations.is_empty());
    }
}
