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
        ActionDef, ActionDefBody, ActionDefBodyElement, ActionUsage as ParserActionUsage,
        ActionUsageBody, ActionUsageBodyElement, AliasDef, AttributeBody, AttributeBodyElement,
        AttributeDef, AttributeUsage, ConnectStmt, ConnectionDef, ConnectionDefBody,
        ConnectionDefBodyElement, ConnectionEnd, ConnectionUsageMember as ParserConnectionUsage,
        DefinitionBody, DefinitionBodyElement, EndDecl, EndIdentity, EnumDef, EnumerationBody,
        EnumerationUsage as ParserEnumerationUsage, Expression, Import, ImportShape, ItemDef,
        ItemUsage as ParserItemUsage, LibraryPackage, Membership,
        MembershipKind as ParserMembershipKind, MetadataDef, MetadataUsage as ParserMetadataUsage,
        NamespaceDecl, Node, OccurrenceBodyElement, OccurrenceDef,
        OccurrenceUsage as ParserOccurrenceUsage, OccurrenceUsageBody, Package, PackageBody,
        PackageBodyElement, PartDef, PartDefBody, PartDefBodyElement, PartUsage, PartUsageBody,
        PartUsageBodyElement, PortBody, PortBodyElement, PortDef, PortDefBody, PortDefBodyElement,
        PortUsage as ParserPortUsage, QualifiedIdentification, QualifiedReferenceId,
        RequirementDef, RequirementDefBody, RequirementDefBodyElement,
        RequirementUsage as ParserRequirementUsage, RootElement, Span, StateDef, StateDefBody,
        StateDefBodyElement, StateUsage as ParserStateUsage, SubsettingKind,
        SubsettingRelationship, Visibility as ParserVisibility,
    },
    ParseError, ParsedDocument,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum DeclarationKind {
    Namespace,
    Package,
    LibraryPackage,
    PartDefinition,
    PartUsage,
    AttributeDefinition,
    AttributeUsage,
    /// `enum def` (BNF EnumerationDefinition): a type whose owned members are enumeration
    /// literals. Mirrors PartDefinition/AttributeDefinition lowering.
    EnumerationDefinition,
    /// A package/definition/usage-level `enum` feature member (BNF EnumerationUsage), e.g.
    /// `enum color : ColorKind;`. Distinct from `EnumerationLiteral`, which is a value owned
    /// directly by an `enum def` body.
    EnumerationUsage,
    /// One `enum <name>;` (or bare `<name>;`) value owned by an `enum def` body (BNF
    /// EnumeratedValue). Each literal gets its own declaration/qualified name, analogous to how
    /// attribute/part usages become owned members.
    EnumerationLiteral,
    /// `requirement def` (BNF RequirementDefinition): a type whose owned members are
    /// attribute/requirement usages, mirroring PartDefinition lowering. Requirement-specific
    /// semantics (subject binding, assumption/constraint facts) are out of scope here; only
    /// ownership, specialization, and owned-member structure are lowered.
    RequirementDefinition,
    /// A package/definition/usage-level `requirement` feature member (BNF RequirementUsage), e.g.
    /// `requirement r : SomeReq;`. Mirrors PartUsage lowering.
    RequirementUsage,
    /// `port def` (BNF PortDefinition): a type whose owned members are attribute/enum/nested-port
    /// usages, mirroring PartDefinition lowering. Port-specific semantics (interface/flow
    /// binding, conformance, connector-end validation) are out of scope here; only ownership,
    /// specialization, and owned-member structure are lowered.
    PortDefinition,
    /// A package/definition/usage-level `port` feature member (BNF PortUsage), e.g.
    /// `port source : ~InputPort;`. Mirrors PartUsage lowering. Its `:`/`:>` typing target may be
    /// conjugated (a leading `~`, e.g. `~InputPort`); the conjugation polarity is carried as an
    /// explicit `RelationshipFlags::conjugated` fact on the FeatureTyping/Subclassification
    /// reference rather than folded into the reference target itself.
    PortUsage,
    /// `item def` (BNF ItemDefinition): a type whose owned members are attribute/enum/nested-item
    /// usages, mirroring PartDefinition lowering. Item-specific semantics beyond ownership,
    /// specialization, and owned-member structure are out of scope here.
    ItemDefinition,
    /// A package/definition/usage-level `item` feature member (BNF ItemUsage), e.g.
    /// `item i : SomeItem;`. Mirrors PartUsage lowering.
    ItemUsage,
    /// `action def` (BNF ActionDefinition): a type whose owned members are attribute/item/nested
    /// action usages, mirroring PartDefinition lowering. Behavioral/control-flow semantics
    /// (parameters, succession, decision/merge/fork/join, accept/send, perform) are out of scope
    /// here; only ownership, specialization, and owned-declaration structure are lowered.
    ActionDefinition,
    /// A package/definition/usage-level `action` feature member (BNF ActionUsage), e.g.
    /// `action validateRoute;` or `action a : SomeAction;`. Mirrors PartUsage lowering. Like
    /// `PartUsage`, `ActionUsage`'s typing is a structured `TypingRelationship` (not a bare
    /// `QualifiedReferenceId`).
    ActionUsage,
    /// `state def` (BNF StateDefinition): a type whose owned members are attribute/item/action/
    /// nested state usages, mirroring ActionDefinition lowering. State-machine-specific semantics
    /// (entry/do/exit action bindings, transitions, exclusive/parallel substates, history) are out
    /// of scope here; only ownership, specialization, and owned-declaration structure are lowered.
    StateDefinition,
    /// A package/definition/usage-level `state` feature member (BNF StateUsage), e.g.
    /// `state s;` or `state s : SomeState;`. Mirrors ActionUsage lowering. `StateUsage`'s typing
    /// is a structured `TypingRelationship` (not a bare `QualifiedReferenceId`).
    StateUsage,
    /// `metadata def` (BNF MetadataDefinition): a type whose owned members are attribute/nested
    /// usages, mirroring ItemDefinition lowering: ownership, membership, an optional `:>`
    /// specialization relationship, and owned-member structure through the shared
    /// `lower_attribute_body`. Metadata-specific semantics (annotation application, `about`
    /// targets) are out of scope here.
    MetadataDefinition,
    /// A package/definition/usage-level `metadata` feature member (BNF MetadataUsage), e.g.
    /// `metadata m : SomeMetadata;`. Mirrors ItemUsage lowering: its `:` typing target is a bare
    /// `QualifiedReferenceId`. The `about` clause (targets this usage annotates) is out of scope
    /// here -- a distinct annotation-application fact family, not the declaration/typing shape
    /// covered by this slice.
    MetadataUsage,
    /// `connection def` (BNF ConnectionDefinition): a type whose owned members are attribute/
    /// item/port usages, nested `end`/`connect` connector structure, mirroring PortDefinition
    /// lowering. Connector-end referential/multiplicity validation is out of scope here; only
    /// ownership, specialization, owned-member structure, and connector-end reference facts are
    /// lowered.
    ConnectionDefinition,
    /// A package/definition/usage-level `connection` feature member (BNF ConnectionUsage), e.g.
    /// `connection : ConnDef connect a::portA to b::portB;`. Mirrors PortUsage lowering; its `:`
    /// typing target is a bare `QualifiedReferenceId` (like MetadataUsage), not a structured
    /// `TypingRelationship`.
    ConnectionUsage,
    /// `occurrence def` (BNF OccurrenceDefinition): a type whose owned members are attribute/
    /// item/part/nested-occurrence usages, mirroring PartDefinition lowering. Occurrence is the
    /// base kind that `part`/`item`/`state`/`action`/`connection`/`event` etc. specialize from in
    /// the standard library. Occurrence-specific semantics (individual/portion-of-life,
    /// time-slicing, snapshot facts, `exhibit`/`succession`/`satisfy`/`allocate`/connector-end
    /// body constructs) are out of scope here; only ownership, specialization, and owned-member
    /// structure are lowered.
    OccurrenceDefinition,
    /// A package/definition/usage-level `occurrence` feature member (BNF OccurrenceUsage), e.g.
    /// `occurrence o;` or `occurrence o : SomeOccurrence;`. Mirrors PortUsage lowering; its `:`
    /// typing target is a bare `QualifiedReferenceId` (like MetadataUsage/ItemUsage), not a
    /// structured `TypingRelationship`, but does carry an independent conjugation flag
    /// (`type_is_conjugated`) analogous to PortUsage's conjugated typing target.
    OccurrenceUsage,
    Import,
    Alias,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MembershipKind {
    Owning,
    Feature,
    Import,
    Alias,
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
    /// The authored target of an `alias X for Y;` member (`AliasDef::target`), resolved through
    /// the same lexical lookup fixed point as every other authored reference kind. Named
    /// `AliasBinding` to match RESOLUTION_LAYER_DESIGN.md's "alias binding" vocabulary (section
    /// 10.1) rather than inventing new terminology.
    AliasBinding,
    /// The authored target of a connector end (`ConnectStmt`'s `from`/`to`/extra ends, or a bare
    /// `EndDecl`'s `::>`/`references` target), resolved through the same lexical lookup fixed
    /// point as `AliasBinding`: a connector end can reference any feature (not just a Type), so
    /// it is not restricted to the Subclassification/FeatureTyping `Type` domain. Connector-end
    /// referential/multiplicity constraints (matching end types/multiplicities) are explicitly
    /// out of scope; this only resolves the authored reference itself.
    ConnectorEnd,
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
    RequirementDefinitionMember,
    PortDefinitionMember,
    PortUsageMember,
    ActionDefinitionMember,
    ActionUsageMember,
    /// Shared by `state def` and `state` usage bodies (both use `StateDefBody`/
    /// `StateDefBodyElement` in the typed AST -- there is no separate `StateUsageBody`).
    StateDefinitionMember,
    /// Shared by `connection def` and `connection` usage bodies (both use `ConnectionDefBody`/
    /// `ConnectionDefBodyElement` in the typed AST -- there is no separate `ConnectionUsageBody`).
    ConnectionDefinitionMember,
    /// Shared by `occurrence def` and `occurrence` usage bodies (both share `OccurrenceBodyElement`
    /// -- `OccurrenceDef.body` wraps it in the generic `DefinitionBody`/`DefinitionBodyElement`,
    /// while `OccurrenceUsage.body` (`OccurrenceUsageBody`) holds it directly). Occurrence-specific
    /// semantics -- individual/portion-of-life, time-slicing, snapshot facts, `exhibit`/
    /// `succession`/`satisfy`/`allocate`/connector-end body constructs -- are the out-of-scope
    /// surface for this slice.
    OccurrenceDefinitionMember,
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
    parse_errors: Box<[ParseError]>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Declaration {
    document: DocumentId,
    owner: Option<DeclarationId>,
    name: Option<SymbolId>,
    anonymous_ordinal: Option<u32>,
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

struct PendingReference {
    source: DeclarationId,
    kind: ReferenceKind,
    document: DocumentId,
    local: QualifiedReferenceId,
    flags: RelationshipFlags,
    span: Span,
    import: Option<AuthoredImportFacts>,
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
    document_index: HashTable<DocumentId>,
    document_hash_builder: RandomState,
    declarations: Vec<Declaration>,
    memberships: Vec<MembershipRecord>,
    references: Vec<AuthoredReference>,
    unsupported: Vec<UnsupportedRecord>,
    recovery: Vec<RecoveryRecord>,
    symbols: SymbolTableBuilder,
    paths: SymbolPathArenaBuilder,
    path_scratch: Vec<SymbolId>,
    next_anonymous_ordinals: BTreeMap<(DocumentId, Option<DeclarationId>, DeclarationKind), u32>,
    next_reference_ordinals: BTreeMap<(DeclarationId, ReferenceKind), u32>,
}

impl SemanticModelBuilder {
    fn admit_document(
        &mut self,
        identity: impl Into<Box<str>>,
        parsed: Arc<ParsedDocument>,
        parse_errors: Vec<ParseError>,
    ) -> Result<DocumentId, ConstructionError> {
        let identity = identity.into();
        let hash = self.document_hash_builder.hash_one(identity.as_ref());
        if self
            .document_index
            .find(hash, |candidate| {
                self.documents[candidate.index()].identity.as_ref() == identity.as_ref()
            })
            .is_some()
        {
            return Err(ConstructionError::DuplicateDocumentIdentity);
        }
        let id = DocumentId::from_index(self.documents.len())?;
        self.documents
            .try_reserve(1)
            .map_err(|_| ConstructionError::Capacity)?;
        let documents = &self.documents;
        let hash_builder = &self.document_hash_builder;
        self.document_index
            .try_reserve(1, |candidate| {
                hash_builder.hash_one(documents[candidate.index()].identity.as_ref())
            })
            .map_err(|_| ConstructionError::Capacity)?;
        self.documents.push(CanonicalDocument {
            identity,
            parsed,
            parse_errors: parse_errors.into_boxed_slice(),
        });
        let documents = &self.documents;
        let hash_builder = &self.document_hash_builder;
        self.document_index.insert_unique(hash, id, |candidate| {
            hash_builder.hash_one(documents[candidate.index()].identity.as_ref())
        });
        Ok(id)
    }

    fn intern_name(&mut self, value: &str) -> Result<SymbolId, ConstructionError> {
        self.symbols.intern(value)
    }

    fn intern_declared_name(&mut self, value: &str) -> Result<Option<SymbolId>, ConstructionError> {
        (!value.is_empty())
            .then(|| self.intern_name(value))
            .transpose()
    }

    #[cfg(test)]
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
        let anonymous_ordinal = if name.is_none() {
            let ordinal = self
                .next_anonymous_ordinals
                .entry((document, owner, kind))
                .or_insert(0);
            let value = *ordinal;
            *ordinal = ordinal.checked_add(1).ok_or(ConstructionError::Capacity)?;
            Some(value)
        } else {
            None
        };
        self.declarations.push(Declaration {
            document,
            owner,
            name,
            anonymous_ordinal,
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
        pending: PendingReference,
    ) -> Result<AuthoredReferenceId, ConstructionError> {
        let PendingReference {
            source,
            kind,
            document,
            local,
            flags,
            span,
            import,
        } = pending;
        if source.index() >= self.declarations.len() || document.index() >= self.documents.len() {
            return Err(ConstructionError::InvalidParserReference);
        }
        let parsed = Arc::clone(&self.documents[document.index()].parsed);
        let reference = parsed
            .qualified_reference(local)
            .ok_or(ConstructionError::InvalidParserReference)?;
        let mut segments = std::mem::take(&mut self.path_scratch);
        segments.clear();
        segments
            .try_reserve(reference.segments.len())
            .map_err(|_| ConstructionError::Capacity)?;
        let path = (|| {
            for index in 0..reference.segments.len() {
                let decoded = reference
                    .segment_decoded_text(index)
                    .ok_or(ConstructionError::InvalidParserReference)?;
                segments.push(self.intern_name(decoded.as_ref())?);
            }
            self.paths.push(&segments, reference.metadata.is_absolute)
        })();
        segments.clear();
        self.path_scratch = segments;
        let path = path?;
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
            .filter(|name| !name.is_empty())
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
            PackageBodyElement::PortDef(node) => self.lower_port_def(document, owner, node)?,
            PackageBodyElement::InterfaceDef(node) => self.push_unsupported(
                document,
                UnsupportedFamily::PackageMember,
                node.span.clone(),
            ),
            PackageBodyElement::AliasDef(node) => self.lower_alias_def(document, owner, node)?,
            PackageBodyElement::AttributeDef(node) => {
                self.lower_attribute_def(document, owner, node)?
            }
            PackageBodyElement::EnumDef(node) => self.lower_enum_def(document, owner, node)?,
            PackageBodyElement::EnumerationUsage(node) => {
                self.lower_enum_usage(document, owner, node)?
            }
            PackageBodyElement::ActionDef(node) => self.lower_action_def(document, owner, node)?,
            PackageBodyElement::ActionUsage(node) => {
                self.lower_action_usage(document, owner, node)?
            }
            PackageBodyElement::RequirementDef(node) => {
                self.lower_requirement_def(document, owner, node)?
            }
            PackageBodyElement::RequirementUsage(node) => {
                self.lower_requirement_usage(document, owner, node)?
            }
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
            PackageBodyElement::StateDef(node) => self.lower_state_def(document, owner, node)?,
            PackageBodyElement::StateUsage(node) => {
                self.lower_state_usage(document, owner, node)?
            }
            PackageBodyElement::ItemDef(node) => self.lower_item_def(document, owner, node)?,
            PackageBodyElement::MetadataDef(node) => {
                self.lower_metadata_def(document, owner, node)?
            }
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
            PackageBodyElement::ConnectionDef(node) => {
                self.lower_connection_def(document, owner, node)?
            }
            PackageBodyElement::OccurrenceDef(node) => {
                self.lower_occurrence_def(document, owner, node)?
            }
            PackageBodyElement::OccurrenceUsage(node) => {
                self.lower_occurrence_usage(document, owner, node)?
            }
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
            PackageBodyElement::ItemUsage(node) => self.lower_item_usage(document, owner, node)?,
            PackageBodyElement::MetadataUsage(node) => {
                self.lower_metadata_usage(document, owner, node)?
            }
            PackageBodyElement::PortUsage(node) => self.lower_port_usage(document, owner, node)?,
            PackageBodyElement::ConnectionUsage(node) => {
                self.lower_connection_usage(document, owner, node)?
            }
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
        self.push_reference(PendingReference {
            source: declaration,
            kind,
            document,
            local: node.value.target.reference,
            flags,
            span: node.value.target.span.clone(),
            import,
        })?;
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
            .filter(|name| !name.is_empty())
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
                    PartDefBodyElement::EnumDef(enum_def) => {
                        self.lower_enum_def(document, Some(declaration), enum_def)?;
                    }
                    PartDefBodyElement::EnumerationUsage(enum_usage) => {
                        self.lower_enum_usage(document, Some(declaration), enum_usage)?;
                    }
                    PartDefBodyElement::RequirementDef(requirement_def) => {
                        self.lower_requirement_def(document, Some(declaration), requirement_def)?;
                    }
                    PartDefBodyElement::RequirementUsage(requirement_usage) => {
                        self.lower_requirement_usage(
                            document,
                            Some(declaration),
                            requirement_usage,
                        )?;
                    }
                    PartDefBodyElement::PortDef(port_def) => {
                        self.lower_port_def(document, Some(declaration), port_def)?;
                    }
                    PartDefBodyElement::PortUsage(port_usage) => {
                        self.lower_port_usage(document, Some(declaration), port_usage)?;
                    }
                    PartDefBodyElement::ItemDef(item_def) => {
                        self.lower_item_def(document, Some(declaration), item_def)?;
                    }
                    PartDefBodyElement::ItemUsage(item_usage) => {
                        self.lower_item_usage(document, Some(declaration), item_usage)?;
                    }
                    PartDefBodyElement::MetadataDef(metadata_def) => {
                        self.lower_metadata_def(document, Some(declaration), metadata_def)?;
                    }
                    PartDefBodyElement::MetadataUsage(metadata_usage) => {
                        self.lower_metadata_usage(document, Some(declaration), metadata_usage)?;
                    }
                    PartDefBodyElement::ActionDef(action_def) => {
                        self.lower_action_def(document, Some(declaration), action_def)?;
                    }
                    PartDefBodyElement::ActionUsage(action_usage) => {
                        self.lower_action_usage(document, Some(declaration), action_usage)?;
                    }
                    PartDefBodyElement::StateDef(state_def) => {
                        self.lower_state_def(document, Some(declaration), state_def)?;
                    }
                    PartDefBodyElement::StateUsage(state_usage) => {
                        self.lower_state_usage(document, Some(declaration), state_usage)?;
                    }
                    PartDefBodyElement::ConnectionDef(connection_def) => {
                        self.lower_connection_def(document, Some(declaration), connection_def)?;
                    }
                    PartDefBodyElement::Connection(connection_usage) => {
                        self.lower_connection_usage(document, Some(declaration), connection_usage)?;
                    }
                    PartDefBodyElement::OccurrenceDef(occurrence_def) => {
                        self.lower_occurrence_def(document, Some(declaration), occurrence_def)?;
                    }
                    PartDefBodyElement::OccurrenceUsage(occurrence_usage) => {
                        self.lower_occurrence_usage(document, Some(declaration), occurrence_usage)?;
                    }
                    PartDefBodyElement::Doc(_) | PartDefBodyElement::Comment(_) => {}
                    PartDefBodyElement::Annotation(_)
                    | PartDefBodyElement::MetadataAnnotation(_)
                    | PartDefBodyElement::MetadataKeywordUsage(_)
                    | PartDefBodyElement::Dependency(_)
                    | PartDefBodyElement::Other(_)
                    | PartDefBodyElement::DefaultReferenceUsage(_)
                    | PartDefBodyElement::Ref(_)
                    | PartDefBodyElement::InterfaceDef(_)
                    | PartDefBodyElement::InterfaceUsage(_)
                    | PartDefBodyElement::Connect(_)
                    | PartDefBodyElement::FlowUsage(_)
                    | PartDefBodyElement::Perform(_)
                    | PartDefBodyElement::Allocate(_)
                    | PartDefBodyElement::ExhibitState(_)
                    | PartDefBodyElement::CalcUsage(_)
                    | PartDefBodyElement::ConstraintDef(_)
                    | PartDefBodyElement::ConstraintUsage(_)
                    | PartDefBodyElement::AssertConstraint(_)
                    | PartDefBodyElement::Satisfy(_)
                    | PartDefBodyElement::VariantUsage(_)
                    | PartDefBodyElement::FlowDef(_)
                    | PartDefBodyElement::CalcDef(_)
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
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::PartUsage,
            name,
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
                    PartUsageBodyElement::EnumDef(enum_def) => {
                        self.lower_enum_def(document, Some(declaration), enum_def)?;
                    }
                    PartUsageBodyElement::EnumerationUsage(enum_usage) => {
                        self.lower_enum_usage(document, Some(declaration), enum_usage)?;
                    }
                    PartUsageBodyElement::RequirementDef(requirement_def) => {
                        self.lower_requirement_def(document, Some(declaration), requirement_def)?;
                    }
                    PartUsageBodyElement::RequirementUsage(requirement_usage) => {
                        self.lower_requirement_usage(
                            document,
                            Some(declaration),
                            requirement_usage,
                        )?;
                    }
                    PartUsageBodyElement::PortDef(port_def) => {
                        self.lower_port_def(document, Some(declaration), port_def)?;
                    }
                    PartUsageBodyElement::PortUsage(port_usage) => {
                        self.lower_port_usage(document, Some(declaration), port_usage)?;
                    }
                    PartUsageBodyElement::ItemDef(item_def) => {
                        self.lower_item_def(document, Some(declaration), item_def)?;
                    }
                    PartUsageBodyElement::ItemUsage(item_usage) => {
                        self.lower_item_usage(document, Some(declaration), item_usage)?;
                    }
                    PartUsageBodyElement::MetadataDef(metadata_def) => {
                        self.lower_metadata_def(document, Some(declaration), metadata_def)?;
                    }
                    PartUsageBodyElement::MetadataUsage(metadata_usage) => {
                        self.lower_metadata_usage(document, Some(declaration), metadata_usage)?;
                    }
                    PartUsageBodyElement::ActionUsage(action_usage) => {
                        self.lower_action_usage(document, Some(declaration), action_usage)?;
                    }
                    PartUsageBodyElement::StateDef(state_def) => {
                        self.lower_state_def(document, Some(declaration), state_def)?;
                    }
                    PartUsageBodyElement::StateUsage(state_usage) => {
                        self.lower_state_usage(document, Some(declaration), state_usage)?;
                    }
                    PartUsageBodyElement::ConnectionDef(connection_def) => {
                        self.lower_connection_def(document, Some(declaration), connection_def)?;
                    }
                    PartUsageBodyElement::Connection(connection_usage) => {
                        self.lower_connection_usage(document, Some(declaration), connection_usage)?;
                    }
                    PartUsageBodyElement::OccurrenceDef(occurrence_def) => {
                        self.lower_occurrence_def(document, Some(declaration), occurrence_def)?;
                    }
                    PartUsageBodyElement::OccurrenceUsage(occurrence_usage) => {
                        self.lower_occurrence_usage(document, Some(declaration), occurrence_usage)?;
                    }
                    PartUsageBodyElement::Doc(_) => {}
                    PartUsageBodyElement::Annotation(_)
                    | PartUsageBodyElement::DefaultReferenceUsage(_)
                    | PartUsageBodyElement::Bind(_)
                    | PartUsageBodyElement::Ref(_)
                    | PartUsageBodyElement::InterfaceUsage(_)
                    | PartUsageBodyElement::Connect(_)
                    | PartUsageBodyElement::FlowUsage(_)
                    | PartUsageBodyElement::Perform(_)
                    | PartUsageBodyElement::SuccessionUsage(_)
                    | PartUsageBodyElement::Allocate(_)
                    | PartUsageBodyElement::Satisfy(_)
                    | PartUsageBodyElement::MetadataAnnotation(_)
                    | PartUsageBodyElement::MetadataKeywordUsage(_)
                    | PartUsageBodyElement::VariantUsage(_)
                    | PartUsageBodyElement::FlowDef(_)
                    | PartUsageBodyElement::CalcDef(_)
                    | PartUsageBodyElement::AssertConstraint(_)
                    | PartUsageBodyElement::ConstraintDef(_)
                    | PartUsageBodyElement::ConstraintUsage(_)
                    | PartUsageBodyElement::CalcUsage(_)
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
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::AttributeUsage,
            name,
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
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::AttributeDefinition,
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

    /// Lowers an `enum def` (BNF EnumerationDefinition), mirroring `lower_part_def`: ownership,
    /// membership, an optional `:>`/`:` specialization relationship (an enum def may specialize
    /// another enum def or an attribute def), and each owned enumeration literal as its own typed
    /// declaration.
    fn lower_enum_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<EnumDef>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .identification
            .name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::EnumerationDefinition,
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
        if let EnumerationBody::Brace { values } = &node.value.body {
            for value in values {
                self.lower_enumerated_value(document, declaration, value)?;
            }
        }
        Ok(())
    }

    /// Lowers one `enum <name>;` value owned by an `enum def` body (BNF EnumeratedValue) into its
    /// own declaration. Any inline body / `= expr` initializer is discarded by the parser itself
    /// (only the name and its span survive), so there is no nested body to lower here.
    fn lower_enumerated_value(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        node: &Node<sysml_v2_parser_next::ast::EnumeratedValue>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::EnumerationLiteral,
            name,
            node.span.clone(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )
    }

    /// Lowers a package/definition/usage-level `enum` feature member (BNF EnumerationUsage), e.g.
    /// `enum color : ColorKind;`, mirroring `lower_attribute_usage`. `type_name` is a bare
    /// `QualifiedReferenceId` (not a `TypingRelationship` node), so its `FeatureTyping` reference
    /// is pushed directly rather than through `lower_typing_relationship`.
    fn lower_enum_usage(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ParserEnumerationUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::EnumerationUsage,
            name,
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
        if let Some(type_name) = node.value.type_name {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_name)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::FeatureTyping,
                document,
                local: type_name,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        Ok(())
    }

    /// Lowers an `item def` (BNF ItemDefinition), mirroring `lower_part_def`: ownership,
    /// membership, and an optional `:>` specialization relationship. `ItemDef`'s body is a plain
    /// `AttributeBody` (shared with `AttributeDef`/`AttributeUsage`), not a `PartDefBody`, so its
    /// owned members are lowered through the existing `lower_attribute_body` rather than a
    /// dedicated item-specific body walker.
    fn lower_item_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ItemDef>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .identification
            .name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::ItemDefinition,
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
        self.lower_attribute_body(document, declaration, &node.value.body)
    }

    /// Lowers a package/definition/usage-level `item` feature member (BNF ItemUsage), e.g.
    /// `item i : SomeItem;`, mirroring `lower_part_usage`. `type_name` is a bare
    /// `QualifiedReferenceId` (not a `TypingRelationship` node, like `ItemUsage::type_name`'s
    /// `lower_enum_usage` counterpart), so its `FeatureTyping` reference is pushed directly rather
    /// than through `lower_typing_relationship`. `ItemUsage`'s body is a plain `AttributeBody`
    /// (see `lower_item_def`), so owned members are lowered through `lower_attribute_body`.
    fn lower_item_usage(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ParserItemUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::ItemUsage,
            name,
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
        if let Some(type_name) = node.value.type_name {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_name)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::FeatureTyping,
                document,
                local: type_name,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        if let Some(relationship) = &node.value.redefines {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        self.lower_attribute_body(document, declaration, &node.value.body)
    }

    /// Lowers a `metadata def` (BNF MetadataDefinition), mirroring `lower_item_def`: ownership,
    /// membership, and an optional `:>` specialization relationship. `MetadataDef`'s body is a
    /// plain `AttributeBody` (shared with `AttributeDef`/`ItemDef`), so its owned members are
    /// lowered through the existing `lower_attribute_body`.
    fn lower_metadata_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<MetadataDef>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .identification
            .name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::MetadataDefinition,
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
        self.lower_attribute_body(document, declaration, &node.value.body)
    }

    /// Lowers a package/definition/usage-level `metadata` feature member (BNF MetadataUsage),
    /// e.g. `metadata m : SomeMetadata;`, mirroring `lower_item_usage`. `type_reference` is a
    /// bare `QualifiedReferenceId`, so its `FeatureTyping` reference is pushed directly rather
    /// than through `lower_typing_relationship`. `MetadataUsage`'s body is a plain
    /// `AttributeBody` (see `lower_metadata_def`), so owned members are lowered through
    /// `lower_attribute_body`. The `about` clause (annotation targets) is deliberately not
    /// lowered here -- it belongs to the separate annotation-application fact family, out of
    /// scope for this slice.
    fn lower_metadata_usage(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ParserMetadataUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::MetadataUsage,
            name,
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
        if let Some(type_reference) = node.value.type_reference {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_reference)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::FeatureTyping,
                document,
                local: type_reference,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        self.lower_attribute_body(document, declaration, &node.value.body)
    }

    /// Lowers an `action def` (BNF ActionDefinition), mirroring `lower_part_def`: ownership,
    /// membership, an optional `:>` specialization relationship, and owned declarations.
    /// Behavioral/control-flow body elements (parameters, succession, decision/merge/fork/join,
    /// accept/send, perform, assign, loops) are explicitly out of scope; unrecognized body
    /// elements fall through to `unsupported_action_definition_member` via
    /// `lower_action_def_body`.
    fn lower_action_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ActionDef>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .identification
            .name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::ActionDefinition,
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
        self.lower_action_def_body(document, declaration, &node.value.body)
    }

    /// Lowers the `ActionDefBody` shared by `action def` and by an `action` usage's own owned
    /// members (BNF `ActionDefBodyElement`): recognized owned members are nested action usages
    /// and `item` usages (BNF `StructureUsageMember` shape, see `crate::ast::ItemUsage`);
    /// everything else -- in/out parameters, `first`/`then` succession, decision/merge/fork/join,
    /// accept/send, perform, assign, loops -- falls through to
    /// `unsupported_action_definition_member`. This is the genuinely out-of-scope
    /// behavioral/control-flow surface for this slice.
    fn lower_action_def_body(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        body: &ActionDefBody,
    ) -> Result<(), ConstructionError> {
        let ActionDefBody::Brace { elements } = body else {
            return Ok(());
        };
        for element in elements {
            match &element.value {
                ActionDefBodyElement::Error(error) => {
                    self.push_recovery(document, error.span.clone());
                }
                ActionDefBodyElement::ActionUsage(action_usage) => {
                    self.lower_action_usage(document, Some(owner), action_usage)?;
                }
                ActionDefBodyElement::ItemUsage(item_usage) => {
                    self.lower_item_usage(document, Some(owner), item_usage)?;
                }
                ActionDefBodyElement::MetadataUsage(metadata_usage) => {
                    self.lower_metadata_usage(document, Some(owner), metadata_usage)?;
                }
                ActionDefBodyElement::StateUsage(state_usage) => {
                    self.lower_state_usage(document, Some(owner), state_usage)?;
                }
                ActionDefBodyElement::Doc(_) => {}
                ActionDefBodyElement::InOutDecl(_)
                | ActionDefBodyElement::Annotation(_)
                | ActionDefBodyElement::MetadataAnnotation(_)
                | ActionDefBodyElement::MetadataKeywordUsage(_)
                | ActionDefBodyElement::TextualRep(_)
                | ActionDefBodyElement::RefDecl(_)
                | ActionDefBodyElement::Perform(_)
                | ActionDefBodyElement::Bind(_)
                | ActionDefBodyElement::FlowUsage(_)
                | ActionDefBodyElement::FirstStmt(_)
                | ActionDefBodyElement::MergeStmt(_)
                | ActionDefBodyElement::DecisionStmt(_)
                | ActionDefBodyElement::JoinStmt(_)
                | ActionDefBodyElement::ForkStmt(_)
                | ActionDefBodyElement::TerminateStmt(_)
                | ActionDefBodyElement::WhileStmt(_)
                | ActionDefBodyElement::LoopStmt(_)
                | ActionDefBodyElement::IfStmt(_)
                | ActionDefBodyElement::PartUsage(_)
                | ActionDefBodyElement::AssertConstraint(_)
                | ActionDefBodyElement::OccurrenceUsage(_)
                | ActionDefBodyElement::Assign(_)
                | ActionDefBodyElement::ForLoop(_)
                | ActionDefBodyElement::ThenAction(_)
                | ActionDefBodyElement::Decl(_)
                | ActionDefBodyElement::DefaultReferenceUsage(_) => self.push_unsupported(
                    document,
                    UnsupportedFamily::ActionDefinitionMember,
                    element.span.clone(),
                ),
            }
        }
        Ok(())
    }

    /// Lowers a package/definition/usage-level `action` feature member (BNF ActionUsage), e.g.
    /// `action validateRoute;` or `action a : SomeAction;`, mirroring `lower_part_usage`.
    /// `ActionUsage`'s typing is a structured `TypingRelationship` (like `PartUsage.typing`), not
    /// a bare `QualifiedReferenceId`. Behavioral clauses (`accept`/`send`/`via`/`to`, parameters,
    /// abstract/variation/individual prefixes) are explicitly out of scope; owned members lower
    /// through the same `lower_action_def_body` as an `action def`'s body (BNF `ActionUsageBody`
    /// is a structurally near-identical production, differing only in the extra `VariantUsage`
    /// alternative, which is itself out of scope and so folds into the same unsupported family).
    fn lower_action_usage(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ParserActionUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::ActionUsage,
            name,
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
        self.lower_action_usage_body(document, declaration, &node.value.body)
    }

    /// Lowers the `ActionUsageBody` owned by an `action` usage (BNF `ActionUsageBodyElement`):
    /// see `lower_action_def_body` for the shared recognized/unsupported shape. The one
    /// additional alternative here, `VariantUsage`, is out of scope and falls through to the same
    /// `unsupported_action_usage_member` family.
    fn lower_action_usage_body(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        body: &ActionUsageBody,
    ) -> Result<(), ConstructionError> {
        let ActionUsageBody::Brace { elements } = body else {
            return Ok(());
        };
        for element in elements {
            match &element.value {
                ActionUsageBodyElement::Error(error) => {
                    self.push_recovery(document, error.span.clone());
                }
                ActionUsageBodyElement::ActionUsage(action_usage) => {
                    self.lower_action_usage(document, Some(owner), action_usage)?;
                }
                ActionUsageBodyElement::ItemUsage(item_usage) => {
                    self.lower_item_usage(document, Some(owner), item_usage)?;
                }
                ActionUsageBodyElement::MetadataUsage(metadata_usage) => {
                    self.lower_metadata_usage(document, Some(owner), metadata_usage)?;
                }
                ActionUsageBodyElement::StateUsage(state_usage) => {
                    self.lower_state_usage(document, Some(owner), state_usage)?;
                }
                ActionUsageBodyElement::Doc(_) => {}
                ActionUsageBodyElement::Annotation(_)
                | ActionUsageBodyElement::MetadataAnnotation(_)
                | ActionUsageBodyElement::MetadataKeywordUsage(_)
                | ActionUsageBodyElement::TextualRep(_)
                | ActionUsageBodyElement::InOutDecl(_)
                | ActionUsageBodyElement::RefDecl(_)
                | ActionUsageBodyElement::Bind(_)
                | ActionUsageBodyElement::FlowUsage(_)
                | ActionUsageBodyElement::FirstStmt(_)
                | ActionUsageBodyElement::MergeStmt(_)
                | ActionUsageBodyElement::DecisionStmt(_)
                | ActionUsageBodyElement::JoinStmt(_)
                | ActionUsageBodyElement::ForkStmt(_)
                | ActionUsageBodyElement::TerminateStmt(_)
                | ActionUsageBodyElement::WhileStmt(_)
                | ActionUsageBodyElement::LoopStmt(_)
                | ActionUsageBodyElement::IfStmt(_)
                | ActionUsageBodyElement::PartUsage(_)
                | ActionUsageBodyElement::AssertConstraint(_)
                | ActionUsageBodyElement::OccurrenceUsage(_)
                | ActionUsageBodyElement::Assign(_)
                | ActionUsageBodyElement::ForLoop(_)
                | ActionUsageBodyElement::ThenAction(_)
                | ActionUsageBodyElement::Decl(_)
                | ActionUsageBodyElement::DefaultReferenceUsage(_)
                | ActionUsageBodyElement::VariantUsage(_) => self.push_unsupported(
                    document,
                    UnsupportedFamily::ActionUsageMember,
                    element.span.clone(),
                ),
            }
        }
        Ok(())
    }

    /// Lowers a `state def` (BNF StateDefinition), mirroring `lower_action_def`: ownership,
    /// membership, an optional `:>` specialization relationship, and owned declarations.
    /// State-machine-specific semantics (entry/do/exit action bindings, transitions, exclusive/
    /// parallel substates, history) are explicitly out of scope; unrecognized body elements fall
    /// through to `unsupported_state_definition_member` via `lower_state_def_body`.
    fn lower_state_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<StateDef>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .identification
            .name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::StateDefinition,
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
        self.lower_state_def_body(document, declaration, &node.value.body)
    }

    /// Lowers the `StateDefBody` shared by `state def` and by a `state` usage's own owned
    /// members (BNF `StateDefBodyElement`): the only recognized owned member is a nested state
    /// usage; everything else -- entry/do/exit action bindings, `then`/`final` state markers,
    /// `ref` bindings, transitions -- falls through to `unsupported_state_definition_member`.
    /// This is the genuinely out-of-scope state-machine surface for this slice.
    fn lower_state_def_body(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        body: &StateDefBody,
    ) -> Result<(), ConstructionError> {
        let StateDefBody::Brace { elements } = body else {
            return Ok(());
        };
        for element in elements {
            match &element.value {
                StateDefBodyElement::Error(error) => {
                    self.push_recovery(document, error.span.clone());
                }
                StateDefBodyElement::StateUsage(state_usage) => {
                    self.lower_state_usage(document, Some(owner), state_usage)?;
                }
                StateDefBodyElement::Doc(_) => {}
                StateDefBodyElement::Annotation(_)
                | StateDefBodyElement::MetadataAnnotation(_)
                | StateDefBodyElement::MetadataKeywordUsage(_)
                | StateDefBodyElement::Other(_)
                | StateDefBodyElement::InOutDecl(_)
                | StateDefBodyElement::Entry(_)
                | StateDefBodyElement::Do(_)
                | StateDefBodyElement::Exit(_)
                | StateDefBodyElement::Then(_)
                | StateDefBodyElement::FinalState(_)
                | StateDefBodyElement::Ref(_)
                | StateDefBodyElement::RequirementUsage(_)
                | StateDefBodyElement::Transition(_) => self.push_unsupported(
                    document,
                    UnsupportedFamily::StateDefinitionMember,
                    element.span.clone(),
                ),
            }
        }
        Ok(())
    }

    /// Lowers a package/definition/usage-level `state` feature member (BNF StateUsage), e.g.
    /// `state s;` or `state s : SomeState;`, mirroring `lower_action_usage`. `StateUsage`'s
    /// typing is a structured `TypingRelationship` (like `ActionUsage.typing`), not a bare
    /// `QualifiedReferenceId`. Behavioral clauses (`entry`/`do`/`exit`, transitions,
    /// abstract/reference/individual prefixes) are explicitly out of scope; owned members lower
    /// through the same `lower_state_def_body` as a `state def`'s body (both share
    /// `StateDefBody`/`StateDefBodyElement`).
    fn lower_state_usage(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ParserStateUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::StateUsage,
            name,
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
        self.lower_state_def_body(document, declaration, &node.value.body)
    }

    /// Lowers a `requirement def` (BNF RequirementDefinition), mirroring `lower_part_def`:
    /// ownership, membership, an optional `:>` specialization relationship, and owned
    /// attribute/requirement members. Requirement-specific semantics (subject binding,
    /// assumption/constraint facts) are explicitly out of scope; unrecognized body elements fall
    /// through to `unsupported_requirement_definition_member`.
    fn lower_requirement_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<RequirementDef>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .identification
            .name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::RequirementDefinition,
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
        self.lower_requirement_def_body(document, declaration, &node.value.body)
    }

    /// Lowers a package/definition/usage-level `requirement` feature member (BNF
    /// RequirementUsage), mirroring `lower_part_usage`: ownership, membership, an optional
    /// `:`/`:>` typing reference, `subsets`/`references` subsetting relationships, and owned
    /// attribute/requirement members.
    fn lower_requirement_usage(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ParserRequirementUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::RequirementUsage,
            name,
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
        if let Some(type_name) = node.value.type_name {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_name)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::FeatureTyping,
                document,
                local: type_name,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        if let Some(relationship) = &node.value.subsets {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.references {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        self.lower_requirement_def_body(document, declaration, &node.value.body)
    }

    /// Lowers the shared `RequirementDefBody` used by both `requirement def` and `requirement`
    /// usage bodies: recognized owned members are attribute def/usage and nested requirement
    /// usages; everything else falls through to `unsupported_requirement_definition_member` via
    /// the single `RequirementDefinitionMember` family (both def and usage bodies share the same
    /// grammar production, `RequirementBody`, so there is no def/usage-specific distinction to
    /// make here).
    fn lower_requirement_def_body(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        body: &RequirementDefBody,
    ) -> Result<(), ConstructionError> {
        let RequirementDefBody::Brace { elements } = body else {
            return Ok(());
        };
        for element in elements {
            match &element.value {
                RequirementDefBodyElement::Error(error) => {
                    self.push_recovery(document, error.span.clone());
                }
                RequirementDefBodyElement::AttributeDef(attribute) => {
                    self.lower_attribute_def(document, Some(owner), attribute)?;
                }
                RequirementDefBodyElement::AttributeUsage(attribute) => {
                    self.lower_attribute_usage(document, Some(owner), attribute)?;
                }
                RequirementDefBodyElement::RequirementUsage(requirement) => {
                    self.lower_requirement_usage(document, Some(owner), requirement)?;
                }
                RequirementDefBodyElement::Doc(_) => {}
                RequirementDefBodyElement::Other(_)
                | RequirementDefBodyElement::Annotation(_)
                | RequirementDefBodyElement::MetadataAnnotation(_)
                | RequirementDefBodyElement::MetadataKeywordUsage(_)
                | RequirementDefBodyElement::Import(_)
                | RequirementDefBodyElement::SubjectDecl(_)
                | RequirementDefBodyElement::SubjectRef(_)
                | RequirementDefBodyElement::RequirementActorDecl(_)
                | RequirementDefBodyElement::Stakeholder(_)
                | RequirementDefBodyElement::Purpose(_)
                | RequirementDefBodyElement::VariantUsage(_)
                | RequirementDefBodyElement::VerifyRequirement(_)
                | RequirementDefBodyElement::RequireConstraint(_)
                | RequirementDefBodyElement::Constraint(_)
                | RequirementDefBodyElement::Frame(_)
                | RequirementDefBodyElement::TextualRep(_) => self.push_unsupported(
                    document,
                    UnsupportedFamily::RequirementDefinitionMember,
                    element.span.clone(),
                ),
            }
        }
        Ok(())
    }

    /// Lowers a `port def` (BNF PortDefinition), mirroring `lower_part_def`: ownership,
    /// membership, an optional `:>` specialization relationship (participates in the shared
    /// Subclassification/FeatureTyping lexical lookup fixed point, see `DeclarationDomain::Type`
    /// in resolver.rs), and owned attribute/enum/nested-port members. Port-specific semantics
    /// (interface/flow binding, port conformance, connector-end validation) are explicitly out of
    /// scope; unrecognized body elements fall through to `unsupported_port_definition_member`.
    fn lower_port_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<PortDef>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .identification
            .name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::PortDefinition,
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
        if let PortDefBody::Brace { elements } = &node.value.body {
            for element in elements {
                match &element.value {
                    PortDefBodyElement::Error(error) => {
                        self.push_recovery(document, error.span.clone());
                    }
                    PortDefBodyElement::AttributeDef(attribute) => {
                        self.lower_attribute_def(document, Some(declaration), attribute)?;
                    }
                    PortDefBodyElement::AttributeUsage(attribute) => {
                        self.lower_attribute_usage(document, Some(declaration), attribute)?;
                    }
                    PortDefBodyElement::EnumerationUsage(enum_usage) => {
                        self.lower_enum_usage(document, Some(declaration), enum_usage)?;
                    }
                    PortDefBodyElement::PortUsage(port_usage) => {
                        self.lower_port_usage(document, Some(declaration), port_usage)?;
                    }
                    PortDefBodyElement::Doc(_) => {}
                    PortDefBodyElement::InOutDecl(_)
                    | PortDefBodyElement::ItemDef(_)
                    | PortDefBodyElement::ItemUsage(_)
                    | PortDefBodyElement::Other(_) => self.push_unsupported(
                        document,
                        UnsupportedFamily::PortDefinitionMember,
                        element.span.clone(),
                    ),
                }
            }
        }
        Ok(())
    }

    /// Lowers a package/definition/usage-level `port` feature member (BNF PortUsage), mirroring
    /// `lower_part_usage`: ownership, membership, an optional `:`/`:>` typing/subclassification
    /// relationship (whose target may be conjugated, e.g. `port source : ~InputPort;` -- the
    /// polarity is carried as an explicit `RelationshipFlags::conjugated` fact via
    /// `lower_typing_relationship`, never folded into the reference target), `subsets`/
    /// `redefines`/`references`/`crosses`/`intersects` subsetting relationships, and owned
    /// attribute/nested-port members.
    fn lower_port_usage(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ParserPortUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::PortUsage,
            name,
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
        if let Some(relationship) = &node.value.references {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.crosses {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.intersects {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let PortBody::Brace { elements } = &node.value.body {
            for element in elements {
                match &element.value {
                    PortBodyElement::Error(error) => {
                        self.push_recovery(document, error.span.clone());
                    }
                    PortBodyElement::AttributeUsage(attribute) => {
                        self.lower_attribute_usage(document, Some(declaration), attribute)?;
                    }
                    PortBodyElement::PortUsage(port_usage) => {
                        self.lower_port_usage(document, Some(declaration), port_usage)?;
                    }
                    PortBodyElement::Doc(_) => {}
                    PortBodyElement::InOutDecl(_) | PortBodyElement::ItemUsage(_) => self
                        .push_unsupported(
                            document,
                            UnsupportedFamily::PortUsageMember,
                            element.span.clone(),
                        ),
                }
            }
        }
        Ok(())
    }

    /// Lowers a `connection def` (BNF ConnectionDefinition), mirroring `lower_port_def`:
    /// ownership, membership, an optional `:>` specialization relationship (participates in the
    /// shared Subclassification/FeatureTyping lexical lookup fixed point, see
    /// `DeclarationDomain::Type` in resolver.rs), and owned attribute/item/port/nested-connection
    /// members plus connector-end structure via `lower_connection_body`. Connector-end
    /// referential/multiplicity validation is explicitly out of scope; unrecognized body elements
    /// fall through to `unsupported_connection_definition_member`.
    fn lower_connection_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ConnectionDef>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .identification
            .name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::ConnectionDefinition,
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
        self.lower_connection_body(document, declaration, &node.value.body)
    }

    /// Lowers a package/definition/usage-level `connection` feature member (BNF ConnectionUsage),
    /// mirroring `lower_metadata_usage`: ownership, membership, an optional `:` typing reference
    /// (a bare `QualifiedReferenceId`, not a structured `TypingRelationship`),
    /// `subsets`/`redefines` subsetting relationships, an optional inline `connect from to to`
    /// clause (connector-end references), and owned attribute/item/port/nested-connection
    /// members via the same shared `lower_connection_body` as `connection def`.
    fn lower_connection_usage(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ParserConnectionUsage>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::ConnectionUsage,
            name,
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
        if let Some(type_reference) = node.value.type_reference {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_reference)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::FeatureTyping,
                document,
                local: type_reference,
                flags: RelationshipFlags::default(),
                span,
                import: None,
            })?;
        }
        if let Some(relationship) = &node.value.subsets {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.redefines {
            self.lower_subsetting_relationship(document, declaration, relationship)?;
        }
        if let Some(end) = &node.value.connect_from {
            self.lower_connector_end(document, declaration, end)?;
        }
        if let Some(end) = &node.value.connect_to {
            self.lower_connector_end(document, declaration, end)?;
        }
        for end in &node.value.connect_extra_ends {
            self.lower_connector_end(document, declaration, end)?;
        }
        self.lower_connection_body(document, declaration, &node.value.body)
    }

    /// Shared body walker for `connection def`/`connection` usage bodies (both use
    /// `ConnectionDefBody`/`ConnectionDefBodyElement` -- there is no separate
    /// `ConnectionUsageBody`), mirroring `lower_state_def_body`'s single-walker pattern. `end`
    /// declarations and `connect` statements carry the connector-end structure; everything else
    /// beyond attribute/item/port/nested-part-usage members falls through to
    /// `unsupported_connection_definition_member`.
    fn lower_connection_body(
        &mut self,
        document: DocumentId,
        declaration: DeclarationId,
        body: &ConnectionDefBody,
    ) -> Result<(), ConstructionError> {
        if let ConnectionDefBody::Brace { elements } = body {
            for element in elements {
                match &element.value {
                    ConnectionDefBodyElement::Error(error) => {
                        self.push_recovery(document, error.span.clone());
                    }
                    ConnectionDefBodyElement::Doc(_) => {}
                    ConnectionDefBodyElement::EndDecl(end_decl) => {
                        self.lower_end_decl(document, declaration, end_decl)?;
                    }
                    ConnectionDefBodyElement::ConnectStmt(connect_stmt) => {
                        self.lower_connect_stmt(document, declaration, connect_stmt)?;
                    }
                    ConnectionDefBodyElement::AttributeDef(attribute) => {
                        self.lower_attribute_def(document, Some(declaration), attribute)?;
                    }
                    ConnectionDefBodyElement::AttributeUsage(attribute) => {
                        self.lower_attribute_usage(document, Some(declaration), attribute)?;
                    }
                    ConnectionDefBodyElement::ItemDef(item_def) => {
                        self.lower_item_def(document, Some(declaration), item_def)?;
                    }
                    ConnectionDefBodyElement::ItemUsage(item_usage) => {
                        self.lower_item_usage(document, Some(declaration), item_usage)?;
                    }
                    ConnectionDefBodyElement::PortDef(port_def) => {
                        self.lower_port_def(document, Some(declaration), port_def)?;
                    }
                    ConnectionDefBodyElement::PortUsage(port_usage) => {
                        self.lower_port_usage(document, Some(declaration), port_usage)?;
                    }
                    ConnectionDefBodyElement::PartUsage(part_usage) => {
                        self.lower_part_usage(document, Some(declaration), part_usage)?;
                    }
                    ConnectionDefBodyElement::RefDecl(_)
                    | ConnectionDefBodyElement::AssertConstraint(_)
                    | ConnectionDefBodyElement::OccurrenceUsage(_)
                    | ConnectionDefBodyElement::SuccessionUsage(_) => self.push_unsupported(
                        document,
                        UnsupportedFamily::ConnectionDefinitionMember,
                        element.span.clone(),
                    ),
                }
            }
        }
        Ok(())
    }

    /// Lowers an `end` declaration inside a connection/interface def body (BNF `EndDecl`) as its
    /// own nested declaration: a normal declared label (or an anonymous `#original`/`#derive`
    /// derivation role), an optional `:` typing relationship, and an optional `::>`/`references`
    /// reference-subsetting relationship as an authored `ConnectorEnd` reference (resolved
    /// through the same shared lexical lookup as `AliasBinding`, see `DeclarationDomain::Any` in
    /// resolver.rs). `redefines`/`crosses`/`nested_usage` -- connector-end referential
    /// constraints, not the plain reference shape this slice covers -- are explicitly out of
    /// scope and left unlowered.
    fn lower_end_decl(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        node: &Node<EndDecl>,
    ) -> Result<(), ConstructionError> {
        let name = match &node.value.identity {
            EndIdentity::Declaration(label) => self.intern_declared_name(&label.value)?,
            EndIdentity::Derivation(_) => None,
        };
        let declaration = self.push_typed_declaration(
            document,
            Some(owner),
            DeclarationKind::ConnectionUsage,
            name,
            node.span.clone(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Feature,
            Visibility::Default,
            node.span.clone(),
        )?;
        if let Some(relationship) = &node.value.typing {
            self.lower_typing_relationship(document, declaration, relationship)?;
        }
        if let Some(relationship) = &node.value.references {
            for target in relationship.value.target.iter().copied() {
                let span = self.documents[document.index()]
                    .parsed
                    .qualified_reference(target)
                    .ok_or(ConstructionError::InvalidParserReference)?
                    .metadata
                    .span
                    .clone();
                self.push_reference(PendingReference {
                    source: declaration,
                    kind: ReferenceKind::ConnectorEnd,
                    document,
                    local: target,
                    flags: RelationshipFlags::default(),
                    span,
                    import: None,
                })?;
            }
        }
        Ok(())
    }

    /// Lowers an inline `connect from to to (, extra)*` statement (BNF `ConnectStmt`) as
    /// `ConnectorEnd` references from the owning connection def/usage declaration to each end's
    /// target. Real annotation content in a braced body (`ConnectBody::Brace`) is out of scope --
    /// only the endpoint references themselves are lowered.
    fn lower_connect_stmt(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        node: &Node<ConnectStmt>,
    ) -> Result<(), ConstructionError> {
        self.lower_connector_end(document, owner, &node.value.from)?;
        self.lower_connector_end(document, owner, &node.value.to)?;
        for end in &node.value.extra_ends {
            self.lower_connector_end(document, owner, end)?;
        }
        Ok(())
    }

    /// Lowers one connector end (`ConnectionEnd`, used by both `ConnectStmt` and
    /// `ConnectionUsageMember`'s inline `connect` clause): its path expression is a structured
    /// `Expression` (not a flattened string), so a simple/qualified name (`Expression::FeatureRef`)
    /// resolves as an authored `ConnectorEnd` reference through the same shared lexical lookup as
    /// `AliasBinding`. A dotted feature-chain path (`Expression::MemberAccess`, e.g. `a.portA`) or
    /// any other expression shape has no chained-feature-access resolution anywhere in this
    /// pipeline yet (a materially larger resolution problem than a single-segment reference), so
    /// it is left as an explicit `unsupported_connection_definition_member` diagnostic here rather
    /// than a fabricated or partial resolution.
    fn lower_connector_end(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        node: &Node<ConnectionEnd>,
    ) -> Result<(), ConstructionError> {
        match &node.value.expression.value {
            Expression::FeatureRef(target) => {
                let span = self.documents[document.index()]
                    .parsed
                    .qualified_reference(*target)
                    .ok_or(ConstructionError::InvalidParserReference)?
                    .metadata
                    .span
                    .clone();
                self.push_reference(PendingReference {
                    source: owner,
                    kind: ReferenceKind::ConnectorEnd,
                    document,
                    local: *target,
                    flags: RelationshipFlags::default(),
                    span,
                    import: None,
                })?;
            }
            _ => self.push_unsupported(
                document,
                UnsupportedFamily::ConnectionDefinitionMember,
                node.span.clone(),
            ),
        }
        Ok(())
    }

    /// Lowers an `occurrence def` (BNF OccurrenceDefinition), mirroring `lower_port_def`:
    /// ownership, membership, an optional `:>` specialization relationship, and owned
    /// attribute/item/part/nested-occurrence declarations. Occurrence-specific semantics
    /// (individual/portion-of-life, time-slicing, snapshot facts, `exhibit`/`succession`/
    /// `satisfy`/`allocate`/connector-end body constructs) are explicitly out of scope; unrecognized
    /// body elements fall through to `unsupported_occurrence_definition_member` via
    /// `lower_occurrence_body_element`. `OccurrenceDef.body` is the generic `DefinitionBody`
    /// (shared with e.g. `ItemDef`), which wraps the same `OccurrenceBodyElement` that
    /// `OccurrenceUsage.body` (`OccurrenceUsageBody`) holds directly -- both def and usage publish
    /// under one `UnsupportedFamily::OccurrenceDefinitionMember`.
    fn lower_occurrence_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<OccurrenceDef>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .identification
            .name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::OccurrenceDefinition,
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
        let DefinitionBody::Brace { elements } = &node.value.body else {
            return Ok(());
        };
        for element in elements {
            match &element.value {
                DefinitionBodyElement::Error(error) => {
                    self.push_recovery(document, error.span.clone());
                }
                DefinitionBodyElement::Doc(_) => {}
                DefinitionBodyElement::OccurrenceMember(member) => {
                    self.lower_occurrence_body_element(document, declaration, member)?;
                }
                DefinitionBodyElement::Other(_) => self.push_unsupported(
                    document,
                    UnsupportedFamily::OccurrenceDefinitionMember,
                    element.span.clone(),
                ),
            }
        }
        Ok(())
    }

    /// Lowers one `OccurrenceBodyElement`, shared by `occurrence def`'s body (wrapped in
    /// `DefinitionBodyElement::OccurrenceMember`) and by an `occurrence` usage's own owned members
    /// (`OccurrenceUsageBody` holds `OccurrenceBodyElement` directly): recognized owned members are
    /// attribute/part/item/nested-occurrence usages; everything else -- `assert constraint`, flow
    /// usages, succession usages, `satisfy`, `allocate`, `end` declarations, `exhibit` state
    /// usages -- falls through to `unsupported_occurrence_definition_member`. This is the
    /// genuinely out-of-scope occurrence-specific surface for this slice.
    fn lower_occurrence_body_element(
        &mut self,
        document: DocumentId,
        owner: DeclarationId,
        element: &Node<OccurrenceBodyElement>,
    ) -> Result<(), ConstructionError> {
        match &element.value {
            OccurrenceBodyElement::Error(error) => {
                self.push_recovery(document, error.span.clone());
            }
            OccurrenceBodyElement::Doc(_) => {}
            OccurrenceBodyElement::AttributeUsage(attribute) => {
                self.lower_attribute_usage(document, Some(owner), attribute)?;
            }
            OccurrenceBodyElement::PartUsage(part) => {
                self.lower_part_usage(document, Some(owner), part)?;
            }
            OccurrenceBodyElement::ItemUsage(item) => {
                self.lower_item_usage(document, Some(owner), item)?;
            }
            OccurrenceBodyElement::OccurrenceUsage(occurrence) => {
                self.lower_occurrence_usage(document, Some(owner), occurrence)?;
            }
            OccurrenceBodyElement::Annotation(_)
            | OccurrenceBodyElement::AssertConstraint(_)
            | OccurrenceBodyElement::Other(_)
            | OccurrenceBodyElement::FlowUsage(_)
            | OccurrenceBodyElement::SuccessionUsage(_)
            | OccurrenceBodyElement::Satisfy(_)
            | OccurrenceBodyElement::Allocate(_)
            | OccurrenceBodyElement::EndDecl(_)
            | OccurrenceBodyElement::StateUsage(_) => self.push_unsupported(
                document,
                UnsupportedFamily::OccurrenceDefinitionMember,
                element.span.clone(),
            ),
        }
        Ok(())
    }

    /// Lowers a package/definition/usage-level `occurrence` feature member (BNF OccurrenceUsage),
    /// e.g. `occurrence o;` or `occurrence o : SomeOccurrence;`, mirroring `lower_port_usage`.
    /// `type_name` is a bare `QualifiedReferenceId` (like `ItemUsage`/`MetadataUsage`), not a
    /// structured `TypingRelationship`, but does carry an independent `type_is_conjugated` flag
    /// (mirrored as an explicit `RelationshipFlags::conjugated` fact on the pushed `FeatureTyping`
    /// reference, the same convention `lower_typing_relationship` uses for `PortUsage`). Individual/
    /// event/portion-of-life prefixes (`individual`/`then`/`event`/`ref`/`abstract`/`constant`,
    /// `portion_kind`) and the `event path` occurrence-reference shorthand are explicitly out of
    /// scope -- only the ordinary declaration/typing/subsetting shape is lowered. Owned members
    /// lower through the shared `lower_occurrence_body_element` (both def and usage share
    /// `OccurrenceBodyElement`).
    fn lower_occurrence_usage(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<ParserOccurrenceUsage>,
    ) -> Result<(), ConstructionError> {
        let name = self.intern_declared_name(&node.value.name)?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::OccurrenceUsage,
            name,
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
        if let Some(type_name) = node.value.type_name {
            let span = self.documents[document.index()]
                .parsed
                .qualified_reference(type_name)
                .ok_or(ConstructionError::InvalidParserReference)?
                .metadata
                .span
                .clone();
            self.push_reference(PendingReference {
                source: declaration,
                kind: ReferenceKind::FeatureTyping,
                document,
                local: type_name,
                flags: RelationshipFlags {
                    conjugated: node.value.type_is_conjugated,
                    ..RelationshipFlags::default()
                },
                span,
                import: None,
            })?;
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
        let OccurrenceUsageBody::Brace { elements } = &node.value.body else {
            return Ok(());
        };
        for element in elements {
            self.lower_occurrence_body_element(document, declaration, element)?;
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
            self.push_reference(PendingReference {
                source,
                kind,
                document,
                local: target,
                flags: RelationshipFlags {
                    implied: relationship.value.is_implied,
                    ..RelationshipFlags::default()
                },
                span,
                import: None,
            })?;
        }
        Ok(())
    }

    /// Lowers a package-level `alias X for Y;` member into a declaration plus an authored
    /// `AliasBinding` reference for `Y`, following the Subclassification/typing lowering pattern
    /// above: `target` is already a structured `QualifiedReferenceId` (not a flattened string), so
    /// it resolves through the same lexical lookup fixed point as every other authored reference.
    fn lower_alias_def(
        &mut self,
        document: DocumentId,
        owner: Option<DeclarationId>,
        node: &Node<AliasDef>,
    ) -> Result<(), ConstructionError> {
        let name = node
            .value
            .identification
            .name
            .as_deref()
            .filter(|name| !name.is_empty())
            .map(|name| self.intern_name(name))
            .transpose()?;
        let declaration = self.push_typed_declaration(
            document,
            owner,
            DeclarationKind::Alias,
            name,
            node.span.clone(),
        )?;
        self.push_membership(
            declaration,
            MembershipKind::Alias,
            self.member_visibility(&node.value.membership, ParserMembershipKind::Alias)?,
            node.value.membership.span.clone(),
        )?;
        let target = node.value.target;
        let span = self.documents[document.index()]
            .parsed
            .qualified_reference(target)
            .ok_or(ConstructionError::InvalidParserReference)?
            .metadata
            .span
            .clone();
        self.push_reference(PendingReference {
            source: declaration,
            kind: ReferenceKind::AliasBinding,
            document,
            local: target,
            flags: RelationshipFlags::default(),
            span,
            import: None,
        })?;
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
            self.push_reference(PendingReference {
                source,
                kind,
                document,
                local: target,
                flags: RelationshipFlags {
                    conjugated: relationship.value.is_conjugated,
                    implied: relationship.value.is_implied,
                    ..RelationshipFlags::default()
                },
                span,
                import: None,
            })?;
        }
        Ok(())
    }

    fn member_visibility(
        &self,
        membership: &Membership,
        expected: ParserMembershipKind,
    ) -> Result<Visibility, ConstructionError> {
        if membership.kind != expected {
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
    index: HashTable<SymbolPathId>,
    hash_builder: RandomState,
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
        let hash = self.hash_builder.hash_one((rooted, segments));
        if let Some(existing) = self.index.find(hash, |candidate| {
            let path = self.paths[candidate.index()];
            let end = (path.start + path.len) as usize;
            path.rooted == rooted && self.segments[path.start as usize..end] == *segments
        }) {
            return Ok(*existing);
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
        let paths = &self.paths;
        let stored_segments = &self.segments;
        let hash_builder = &self.hash_builder;
        self.index
            .try_reserve(1, |candidate| {
                let path = paths[candidate.index()];
                let end = (path.start + path.len) as usize;
                hash_builder.hash_one((path.rooted, &stored_segments[path.start as usize..end]))
            })
            .map_err(|_| ConstructionError::Capacity)?;
        self.segments.extend_from_slice(segments);
        self.paths.push(SymbolPath { start, len, rooted });
        let paths = &self.paths;
        let stored_segments = &self.segments;
        let hash_builder = &self.hash_builder;
        self.index.insert_unique(hash, id, |candidate| {
            let path = paths[candidate.index()];
            let end = (path.start + path.len) as usize;
            hash_builder.hash_one((path.rooted, &stored_segments[path.start as usize..end]))
        });
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
        let bytes = &self.bytes;
        let spans = &self.spans;
        let hash_builder = &self.hash_builder;
        self.index
            .try_reserve(1, |candidate| {
                let (candidate_start, candidate_len) = spans[candidate.index()];
                hash_builder.hash_one(
                    &bytes[candidate_start as usize..(candidate_start + candidate_len) as usize],
                )
            })
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
    ConstructionFailed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SemanticModelBuildCoordinator;

impl SemanticModelBuildCoordinator {
    pub(crate) fn build(
        mut sources: Vec<OwnedSourceRecord>,
        schedule: BuildSchedule,
    ) -> Result<resolver::ResolvedSemanticModel, CoordinatorError> {
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
                .admit_document(identity, Arc::new(parsed.document), parsed.errors)
                .map_err(|_| CoordinatorError::DuplicateSourceIdentity)?;
            documents.push(document);
        }
        for document in documents {
            builder
                .canonicalize_document(document)
                .map_err(|_| CoordinatorError::ConstructionFailed)?;
        }
        builder
            .freeze()
            .resolve()
            .map_err(|_| CoordinatorError::ConstructionFailed)
    }

    fn parse_source(
        source: OwnedSourceRecord,
    ) -> Result<(Box<str>, sysml_v2_parser_next::ParseResult), CoordinatorError> {
        Ok((
            source.identity,
            sysml_v2_parser_next::parse_for_editor_owned(source.content),
        ))
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
        let document = builder
            .admit_document("model", parsed.clone(), Vec::new())
            .unwrap();
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
    fn symbol_interning_survives_hash_table_growth() {
        let mut symbols = SymbolTableBuilder::default();
        let vehicle = symbols.intern("Vehicle").unwrap();
        for index in 0..256 {
            symbols.intern(&format!("Name{index}")).unwrap();
        }

        assert_eq!(symbols.intern("Vehicle").unwrap(), vehicle);
        assert_eq!(symbols.len(), 257);
    }

    #[test]
    fn semantic_paths_are_interned_across_arena_growth() {
        let mut paths = SymbolPathArenaBuilder::default();
        let vehicle = paths.push(&[SymbolId(1), SymbolId(2)], false).unwrap();
        for index in 0..256 {
            paths
                .push(&[SymbolId(index), SymbolId(index + 1)], true)
                .unwrap();
        }

        assert_eq!(
            paths.push(&[SymbolId(1), SymbolId(2)], false).unwrap(),
            vehicle
        );
        assert_ne!(
            paths.push(&[SymbolId(1), SymbolId(2)], true).unwrap(),
            vehicle
        );
    }

    #[test]
    fn document_identity_index_rejects_duplicates_after_growth_without_mutation() {
        let parsed = empty_document();
        let mut builder = SemanticModelBuilder::default();
        for index in 0..256 {
            builder
                .admit_document(format!("model-{index}"), parsed.clone(), Vec::new())
                .unwrap();
        }
        let before = builder.documents.len();

        assert_eq!(
            builder
                .admit_document("model-0", parsed, Vec::new())
                .unwrap_err(),
            ConstructionError::DuplicateDocumentIdentity
        );
        assert_eq!(builder.documents.len(), before);
    }

    #[test]
    fn anonymous_ordinals_are_owner_local_and_ignore_named_declarations() {
        let parsed = empty_document();
        let mut builder = SemanticModelBuilder::default();
        let document = builder.admit_document("model", parsed, Vec::new()).unwrap();
        let owner_name = builder.intern_name("Owner").unwrap();
        let owner = builder
            .push_typed_declaration(
                document,
                None,
                DeclarationKind::Package,
                Some(owner_name),
                Span::dummy(),
            )
            .unwrap();
        let first = builder
            .push_typed_declaration(
                document,
                Some(owner),
                DeclarationKind::Import,
                None,
                Span::dummy(),
            )
            .unwrap();
        let named = builder.intern_name("Named").unwrap();
        builder
            .push_typed_declaration(
                document,
                Some(owner),
                DeclarationKind::PartUsage,
                Some(named),
                Span::dummy(),
            )
            .unwrap();
        let second = builder
            .push_typed_declaration(
                document,
                Some(owner),
                DeclarationKind::Import,
                None,
                Span::dummy(),
            )
            .unwrap();

        assert_eq!(
            builder.declarations[first.index()].anonymous_ordinal,
            Some(0)
        );
        assert_eq!(
            builder.declarations[second.index()].anonymous_ordinal,
            Some(1)
        );
    }

    fn build_semantic_sexpr(source: &str) -> String {
        let request = crate::BuildRequest::new(
            vec![crate::SourceInput::new(
                "memory://test/enum.sysml",
                source.to_string(),
                crate::SourceKind::Workspace,
            )],
            crate::ConstructionSchedule::Sequential,
            "test-contract-v1",
        )
        .unwrap();
        let published = crate::build(request).unwrap();
        let mut output = String::new();
        published.debug().write_semantic_sexpr(&mut output).unwrap();
        output
    }

    #[test]
    fn enum_def_lowers_to_a_declaration_with_its_literal_as_an_owned_member() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tenum def StatusKind {\n\
             \t\tenum approved;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::StatusKind\"))) (kind enum-def)"),
            "expected an enum-def declaration, got:\n{output}"
        );
        assert!(
            output.contains(
                "(qualified-name \"Demo::StatusKind::approved\"))) (kind enum-literal)"
            ),
            "expected an owned enum-literal declaration with its own qualified name, got:\n{output}"
        );
    }

    #[test]
    fn attribute_typed_by_an_enum_def_resolves_its_feature_typing_reference() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tenum def StatusKind {\n\
             \t\tenum approved;\n\
             \t}\n\
             \tattribute def Holder {\n\
             \t\tattribute status : StatusKind;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind featureTyping) (ordinal 0))\n      (authored-target \"StatusKind\")\n      (outcome (status resolved) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::StatusKind\"))))"
            ),
            "expected the attribute's featureTyping reference to StatusKind to resolve, got:\n{output}"
        );
    }

    #[test]
    fn enum_def_specializing_another_enum_def_resolves_its_subclassification_reference() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tenum def Base {\n\
             \t\tenum on;\n\
             \t}\n\
             \tenum def Derived :> Base;\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
            ),
            "expected Derived's specialization of Base to resolve, got:\n{output}"
        );
    }

    #[test]
    fn requirement_def_lowers_to_a_declaration() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \trequirement def MassRequirement {\n\
             \t\tattribute mass : Real;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::MassRequirement\"))) (kind requirement-def)"),
            "expected a requirement-def declaration, got:\n{output}"
        );
        assert!(
            output.contains("(qualified-name \"Demo::MassRequirement::mass\"))) (kind attribute)"),
            "expected an owned attribute declaration under the requirement def, got:\n{output}"
        );
    }

    #[test]
    fn requirement_def_specializing_another_requirement_def_resolves_its_subclassification_reference(
    ) {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \trequirement def Base;\n\
             \trequirement def Derived :> Base;\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
            ),
            "expected Derived's specialization of Base to resolve, got:\n{output}"
        );
    }

    #[test]
    fn requirement_usage_typed_by_a_requirement_def_resolves_its_feature_typing_reference() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \trequirement def MassRequirement;\n\
             \tpart def Vehicle {\n\
             \t\trequirement massReq : MassRequirement;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind featureTyping) (ordinal 0))\n      (authored-target \"MassRequirement\")\n      (outcome (status resolved) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::MassRequirement\"))))"
            ),
            "expected the requirement usage's featureTyping reference to MassRequirement to resolve, got:\n{output}"
        );
    }

    #[test]
    fn port_def_lowers_to_a_declaration() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tport def InputPort {\n\
             \t\tattribute level : Real;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::InputPort\"))) (kind port-def)"),
            "expected a port-def declaration, got:\n{output}"
        );
        assert!(
            output.contains("(qualified-name \"Demo::InputPort::level\"))) (kind attribute)"),
            "expected an owned attribute declaration under the port def, got:\n{output}"
        );
    }

    #[test]
    fn port_def_specializing_another_port_def_resolves_its_specialization_reference() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tport def Base;\n\
             \tport def Derived :> Base;\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
            ),
            "expected Derived's specialization of Base to resolve, got:\n{output}"
        );
    }

    #[test]
    fn connection_def_lowers_to_a_declaration() {
        // Bare `end name;` (no `:` type, `::>`/`references` target, or nested occurrence/item
        // usage) is not valid `EndDecl` grammar at all -- confirmed against the upstream parser's
        // `end_decl` (`src/parser/connector.rs`), which requires one of those three forms after
        // the name -- so a real end declaration must carry an explicit type here.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tport def P;\n\
             \tconnection def C {\n\
             \t\tend end1 : P;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::C\"))) (kind connection-def)"),
            "expected a connection-def declaration, got:\n{output}"
        );
        assert!(
            output.contains("(qualified-name \"Demo::C::end1\"))) (kind connection)"),
            "expected an owned end declaration under the connection def, got:\n{output}"
        );
    }

    #[test]
    fn connection_def_specializing_another_connection_def_resolves_its_specialization_reference() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tconnection def Base;\n\
             \tconnection def Derived :> Base;\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
            ),
            "expected Derived's specialization of Base to resolve, got:\n{output}"
        );
    }

    #[test]
    fn connection_usage_connector_end_references_resolve_to_their_targets() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tconnection def C;\n\
             \tpart d1;\n\
             \tpart d2;\n\
             \tconnection bus : C connect d1 to d2;\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::bus\"))) (kind connection)"),
            "expected a connection usage declaration, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind connectorEnd) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::bus\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::d1\")))"
            ),
            "expected bus's connector-end reference to d1 to resolve, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind connectorEnd) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::bus\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::d2\")))"
            ),
            "expected bus's connector-end reference to d2 to resolve, got:\n{output}"
        );
    }

    #[test]
    fn occurrence_def_lowers_to_a_declaration() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \toccurrence def Occ {\n\
             \t\titem x;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::Occ\"))) (kind occurrence-def)"),
            "expected an occurrence-def declaration, got:\n{output}"
        );
        assert!(
            output.contains("(qualified-name \"Demo::Occ::x\"))) (kind item)"),
            "expected an owned item usage under the occurrence def, got:\n{output}"
        );
    }

    #[test]
    fn occurrence_def_specializing_another_occurrence_def_resolves_its_specialization_reference() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \toccurrence def Base;\n\
             \toccurrence def Derived :> Base;\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
            ),
            "expected Derived's specialization of Base to resolve, got:\n{output}"
        );
    }

    #[test]
    fn occurrence_usage_typed_by_an_occurrence_def_resolves() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \toccurrence def Occ;\n\
             \toccurrence o : Occ;\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::o\"))) (kind occurrence)"),
            "expected an occurrence usage declaration, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::o\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Occ\")))"
            ),
            "expected o's typing reference to Occ to resolve, got:\n{output}"
        );
    }

    #[test]
    fn occurrence_definition_member_body_construct_stays_explicitly_unsupported() {
        let request = crate::BuildRequest::new(
            vec![crate::SourceInput::new(
                "memory://test/enum.sysml",
                "package Demo {\n\
                 \toccurrence def Occ {\n\
                 \t\tsuccession first x then y;\n\
                 \t}\n\
                 }\n"
                .to_string(),
                crate::SourceKind::Workspace,
            )],
            crate::ConstructionSchedule::Sequential,
            "test-contract-v1",
        )
        .unwrap();
        let published = crate::build(request).unwrap();
        let mut output = String::new();
        published
            .debug()
            .write_diagnostics_sexpr(&mut output)
            .unwrap();
        assert!(
            output.contains("unsupported_occurrence_definition_member"),
            "expected the succession usage to surface as an explicit unsupported diagnostic, got:\n{output}"
        );
    }

    #[test]
    fn conjugated_port_usage_typing_reference_resolves_and_carries_the_conjugated_flag() {
        // `port p : ~Base;` nested inside a `part def` body dispatches through the real
        // `PortUsage` grammar production (package-level bare `port name : Type;` instead folds
        // into `PortDef`, see `lower_port_def`'s doc comment) -- the `~` conjugation polarity
        // must survive as an explicit fact distinct from the (unconjugated) target declaration.
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tport def Base;\n\
             \tpart def Holder {\n\
             \t\tport p : ~Base;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::Holder::p\"))) (kind port)"),
            "expected a port usage declaration, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind typing) (conjugated true) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Holder::p\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
            ),
            "expected p's conjugated typing reference to Base to resolve with the conjugated flag, got:\n{output}"
        );
    }

    #[test]
    fn non_conjugated_port_usage_typing_reference_does_not_carry_the_conjugated_flag() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tport def Base;\n\
             \tpart def Holder {\n\
             \t\tport p : Base;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Holder::p\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
            ),
            "expected p's unconjugated typing reference to Base to resolve without a conjugated flag, got:\n{output}"
        );
        assert!(
            !output.contains("(kind typing) (conjugated true)"),
            "did not expect the conjugated flag on an unconjugated port typing reference, got:\n{output}"
        );
    }

    #[test]
    fn item_def_lowers_to_a_declaration() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \titem def Widget {\n\
             \t\tattribute mass : Real;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::Widget\"))) (kind item-def)"),
            "expected an item-def declaration, got:\n{output}"
        );
        assert!(
            output.contains("(qualified-name \"Demo::Widget::mass\"))) (kind attribute)"),
            "expected an owned attribute declaration under the item def, got:\n{output}"
        );
    }

    #[test]
    fn item_def_specializing_another_item_def_resolves_its_specialization_reference() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \titem def Base;\n\
             \titem def Derived :> Base;\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
            ),
            "expected Derived's specialization of Base to resolve, got:\n{output}"
        );
    }

    #[test]
    fn item_usage_typed_by_an_item_def_resolves() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \titem def Base;\n\
             \tpart def Holder {\n\
             \t\titem w : Base;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::Holder::w\"))) (kind item)"),
            "expected an item usage declaration, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Holder::w\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
            ),
            "expected w's typing reference to Base to resolve, got:\n{output}"
        );
    }

    #[test]
    fn metadata_def_lowers_to_a_declaration() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tmetadata def Safety {\n\
             \t\tattribute isMandatory : Boolean;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::Safety\"))) (kind metadata-def)"),
            "expected a metadata-def declaration, got:\n{output}"
        );
        assert!(
            output.contains("(qualified-name \"Demo::Safety::isMandatory\"))) (kind attribute)"),
            "expected an owned attribute declaration under the metadata def, got:\n{output}"
        );
    }

    #[test]
    fn metadata_def_specializing_another_metadata_def_resolves_its_specialization_reference() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tmetadata def Base;\n\
             \tmetadata def Derived :> Base;\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
            ),
            "expected Derived's specialization of Base to resolve, got:\n{output}"
        );
    }

    #[test]
    fn metadata_usage_typed_by_a_metadata_def_resolves() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tmetadata def Base;\n\
             \tpart def Holder {\n\
             \t\tmetadata m : Base;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::Holder::m\"))) (kind metadata)"),
            "expected a metadata usage declaration, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Holder::m\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
            ),
            "expected m's typing reference to Base to resolve, got:\n{output}"
        );
    }

    #[test]
    fn action_def_lowers_to_a_declaration() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \taction def ExecuteMission {\n\
             \t\taction validateRoute;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::ExecuteMission\"))) (kind action-def)"),
            "expected an action-def declaration, got:\n{output}"
        );
        assert!(
            output.contains(
                "(qualified-name \"Demo::ExecuteMission::validateRoute\"))) (kind action)"
            ),
            "expected an owned nested action usage declaration under the action def, got:\n{output}"
        );
    }

    #[test]
    fn action_def_specializing_another_action_def_resolves_its_specialization_reference() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \taction def Base;\n\
             \taction def Derived :> Base;\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
            ),
            "expected Derived's specialization of Base to resolve, got:\n{output}"
        );
    }

    #[test]
    fn action_usage_typed_by_an_action_def_resolves() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \taction def Base;\n\
             \taction a : Base;\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::a\"))) (kind action)"),
            "expected an action usage declaration, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::a\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
            ),
            "expected a's typing reference to Base to resolve, got:\n{output}"
        );
    }

    #[test]
    fn first_then_succession_inside_an_action_def_still_surfaces_as_unsupported() {
        let request = crate::BuildRequest::new(
            vec![crate::SourceInput::new(
                "memory://test/enum.sysml",
                "package Demo {\n\
                 \taction def ExecuteMission {\n\
                 \t\taction validateRoute;\n\
                 \t\taction startMission;\n\
                 \t\tfirst validateRoute then startMission;\n\
                 \t}\n\
                 }\n"
                .to_string(),
                crate::SourceKind::Workspace,
            )],
            crate::ConstructionSchedule::Sequential,
            "test-contract-v1",
        )
        .unwrap();
        let published = crate::build(request).unwrap();
        let mut output = String::new();
        published
            .debug()
            .write_diagnostics_sexpr(&mut output)
            .unwrap();
        assert!(
            output.contains("unsupported_action_definition_member"),
            "expected the first/then succession statement to surface as an explicit unsupported \
             action-definition-member diagnostic, got:\n{output}"
        );
    }

    #[test]
    fn state_def_lowers_to_a_declaration() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tstate def SD {\n\
             \t\tstate s;\n\
             \t}\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::SD\"))) (kind state-def)"),
            "expected a state-def declaration, got:\n{output}"
        );
        assert!(
            output.contains("(qualified-name \"Demo::SD::s\"))) (kind state)"),
            "expected an owned nested state usage declaration under the state def, got:\n{output}"
        );
    }

    #[test]
    fn state_def_specializing_another_state_def_resolves_its_specialization_reference() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tstate def Base;\n\
             \tstate def Derived :> Base;\n\
             }\n",
        );
        assert!(
            output.contains(
                "(kind specialization) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Derived\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
            ),
            "expected Derived's specialization of Base to resolve, got:\n{output}"
        );
    }

    #[test]
    fn state_usage_typed_by_a_state_def_resolves() {
        let output = build_semantic_sexpr(
            "package Demo {\n\
             \tstate def Base;\n\
             \tstate s : Base;\n\
             }\n",
        );
        assert!(
            output.contains("(qualified-name \"Demo::s\"))) (kind state)"),
            "expected a state usage declaration, got:\n{output}"
        );
        assert!(
            output.contains(
                "(kind typing) (source (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::s\"))) (target (node (document \"memory://test/enum.sysml\") (qualified-name \"Demo::Base\")))"
            ),
            "expected s's typing reference to Base to resolve, got:\n{output}"
        );
    }

    #[test]
    fn transition_inside_a_state_def_still_surfaces_as_unsupported() {
        let request = crate::BuildRequest::new(
            vec![crate::SourceInput::new(
                "memory://test/enum.sysml",
                "package Demo {\n\
                 \tstate def SD {\n\
                 \t\tstate s1;\n\
                 \t\tstate s2;\n\
                 \t\ttransition t first s1 then s2;\n\
                 \t}\n\
                 }\n"
                .to_string(),
                crate::SourceKind::Workspace,
            )],
            crate::ConstructionSchedule::Sequential,
            "test-contract-v1",
        )
        .unwrap();
        let published = crate::build(request).unwrap();
        let mut output = String::new();
        published
            .debug()
            .write_diagnostics_sexpr(&mut output)
            .unwrap();
        assert!(
            output.contains("unsupported_state_definition_member"),
            "expected the transition statement to surface as an explicit unsupported \
             state-definition-member diagnostic, got:\n{output}"
        );
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
