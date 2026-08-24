//! Phase 9: the read-only query surface over a finished model.

mod visible;

pub use visible::VisibleMemberRef;
pub use visible::VisibleMembers;

use crate::diagnose::valid_identifier;
use crate::index::bindings as binding;
use crate::index::documents::leaf_ranges_containing;
use crate::index::documents::record_visited_index_entries;
use crate::index::types;
use crate::lower::facts::ParameterDirection;
use crate::model::element_kind;
use crate::model::render as writer;
use crate::model::resolver::ResolvedSemanticModel;
use crate::model::resolver::SemanticModel;
use crate::model::span::document_range;
use crate::model::AuthoredReferenceId;
use crate::model::DeclarationId;
use crate::model::DeclarationKind;
use crate::model::DocumentIdx;
use crate::model::MembershipKind;
use crate::model::ReferenceKind;
use crate::namespace_query::NamespaceDerivedElementCollection;
use crate::namespace_query::NamespaceImportDerivedElement;
use crate::redefinition_query::RedefinitionCheckKind;
use crate::redefinition_query::RedefinitionCheckOutcome;
use crate::redefinition_query::RedefinitionCheckPrerequisite;
use crate::requirement_query::RequirementDerivedFactCollection;
use crate::requirement_query::RequirementDerivedFactOutcome;
use crate::requirement_query::RequirementDerivedFactPrerequisite;
use crate::resolve::definition_usage_candidate_matches;
use crate::resolve::definition_usage_source_matches;
use crate::resolve::implied::action_derived_fact_rule;
use crate::resolve::implied::binding_connector_check_rule;
use crate::resolve::implied::definition_usage_derived_rule;
use crate::resolve::implied::element_derived_documentation_rule;
use crate::resolve::implied::element_derived_owner_rule;
use crate::resolve::implied::feature_derived_relationship_kinds;
use crate::resolve::implied::feature_derived_relationship_rule;
use crate::resolve::implied::lowered_redefinition_source_kind;
use crate::resolve::implied::namespace_derived_element_rule;
use crate::resolve::implied::namespace_import_derived_element_rule;
use crate::resolve::implied::redefinition_check_rule;
use crate::resolve::implied::requirement_derived_fact_rule;
use crate::resolve::implied::specialization_check_rule;
use crate::resolve::implied::type_derived_element_rule;
use crate::resolve::implied::type_derived_fact_rule;
use crate::resolve::implied::type_derived_relationship_kinds;
use crate::resolve::implied::type_derived_relationship_rule;
use crate::resolve::implied::type_featuring_check_rule;
use crate::resolve::implied::LibrarySpecializationAnchor;
use crate::resolve::implied::GENERATED_LIBRARY_REDEFINITION_RULES;
use crate::resolve::is_action_usage_declaration;
use crate::resolve::is_usage_declaration;
use crate::resolve::names::lookup_lexical_into;
use crate::resolve::names::LookupTarget;
use crate::resolve::requirement_derived_membership_role;
use crate::resolve::requirement_derived_source_matches;
use crate::resolve::results::ResolutionError;
use crate::resolve::results::ResolutionStatus;
use crate::resolve::results::ResolutionWork;
use crate::resolve::DeclarationDomain;
use crate::resolve::ResolutionIndexes;
use crate::specialization_query::SpecializationCheckKind;
use crate::specialization_query::SpecializationCheckOutcome;
use crate::specialization_query::SpecializationCheckPrerequisite;
use crate::traceability::BindingConnector;
use crate::traceability::BindingConnectorCheckKind;
use crate::traceability::BindingConnectorValidationOutcome;
use crate::traceability::BindingEndpoint;
use crate::traceability::SatisfyEndpoint;
use crate::traceability::SatisfyPolarity;
use crate::traceability::SatisfyRelationship;
use crate::type_query::Conformance;
use crate::type_query::ConformanceObstacle;
use crate::type_query::EffectiveType;
use crate::type_query::EffectiveTypeOrigin;
use crate::type_query::RequirementUsageTyping;
use crate::type_query::SpecializationScope;
use crate::type_query::SubsettingConformance;
use crate::type_query::TypeDerivedElementCollection;
use crate::type_query::TypeDerivedFactCollection;
use crate::type_query::TypeDerivedFactOutcome;
use crate::type_query::TypeDerivedFactPrerequisite;
use crate::type_query::TypeDerivedFactValue;
use crate::type_query::TypeDerivedRelationshipCollection;
use crate::type_query::TypeFeaturingCheckKind;
use crate::type_query::TypeFeaturingCheckOutcome;
use crate::type_query::TypeFeaturingCheckPrerequisite;
use crate::type_query::TypeReference;
use crate::verification::RequirementVerification;
use crate::verification::VerificationOutcome;
use crate::verification::VerificationRequirement;
use crate::ActionDerivedFactCollection;
use crate::ActionDerivedFactOutcome;
use crate::ActionDerivedFactPrerequisite;
use crate::AnnotationForm as InspectionAnnotationForm;
use crate::DefinitionUsageDerivedKind;
use crate::DefinitionUsageDerivedOutcome;
use crate::DefinitionUsageDerivedPrerequisite;
use crate::DerivedElementOwner;
use crate::Diagnostic;
use crate::Documentation;
use crate::ElementDerivedDocumentationCollection;
use crate::ElementRelationship;
use crate::FeatureDerivedRelationshipCollection;
use crate::LibrarySpecializationAnchorBranch;
use crate::NavigationTarget;
use crate::OccurrenceRole;
use crate::PublicationCompleteness;
use crate::PublishedDiagnostics;
use crate::QueryAnswer;
use crate::QueryOutcome;
use crate::RelationshipProvenance;
use crate::RelationshipTarget;
use crate::RenameOutcome;
use crate::SourceLocation;
use crate::SymbolId;
use crate::SymbolToken;
use crate::TextPosition;
use crate::TextRange;

use source_identity::SourceRole;
use spec42_constraint_manifest::ElementDerivedOwnerKind;
use spec42_constraint_manifest::NamespaceImportDerivedElementKind;
use sysml_contract::{DocumentId, DocumentToken};

impl<D> SemanticModel<D> {
    pub(crate) fn completeness(&self) -> PublicationCompleteness {
        self.metadata.completeness
    }

    pub(crate) fn affected_documents(
        &self,
        changed_document: &str,
    ) -> QueryOutcome<Box<[crate::AffectedDocument]>> {
        let Some(changed) = self
            .storage
            .documents
            .iter()
            .position(|document| document.identity.as_ref() == changed_document)
            .map(|index| DocumentIdx(index as u32))
        else {
            return self.query_outcome(QueryAnswer::Unresolved);
        };

        // `provider -> consumer`. Both resolved and ambiguous outcomes are owned facts: every
        // ambiguous candidate can affect the consumer if it changes. Unresolved/unsupported
        // dependency-shaping references remain explicit through the publication completeness or
        // the conservative result below; they are never guessed from authored text.
        let mut reverse = vec![Vec::<DocumentIdx>::new(); self.storage.documents.len()];
        let mut unsettled_dependency = false;
        for (ordinal, reference) in self.storage.references.iter().enumerate() {
            if !matches!(
                reference.kind,
                ReferenceKind::NamespaceImport
                    | ReferenceKind::MembershipImport
                    | ReferenceKind::FilterImport
                    | ReferenceKind::AliasBinding
            ) {
                continue;
            }
            let Some(source_document) = self
                .storage
                .declaration(reference.source)
                .map(|declaration| declaration.document)
            else {
                unsettled_dependency = true;
                continue;
            };
            let reference_id = AuthoredReferenceId(ordinal as u32);
            let targets: Vec<DeclarationId> = match self.resolution.outcome(reference_id) {
                Some(ResolutionStatus::Resolved(target)) => vec![target],
                Some(ResolutionStatus::Ambiguous(range)) => {
                    self.resolution.ambiguous_candidates(range).to_vec()
                }
                Some(
                    ResolutionStatus::Unresolved
                    | ResolutionStatus::Unsupported
                    | ResolutionStatus::NonConverged,
                )
                | None => {
                    unsettled_dependency = true;
                    continue;
                }
            };
            for target in targets {
                if let Some(target_document) = self
                    .storage
                    .declaration(target)
                    .map(|declaration| declaration.document)
                {
                    if target_document != source_document {
                        reverse[target_document.index()].push(source_document);
                    }
                }
            }
        }

        let mut seen = vec![false; self.storage.documents.len()];
        let mut pending = vec![changed];
        seen[changed.index()] = true;
        while let Some(provider) = pending.pop() {
            for consumer in reverse[provider.index()].iter().copied() {
                if !seen[consumer.index()] {
                    seen[consumer.index()] = true;
                    pending.push(consumer);
                }
            }
        }
        let mut affected = seen
            .into_iter()
            .enumerate()
            .filter(|(index, affected)| *affected && *index != changed.index())
            .filter_map(|(index, _)| self.storage.documents.get(index))
            .map(|document| crate::AffectedDocument {
                identity: document.identity.clone(),
                source: match document.role {
                    source_identity::SourceRole::Workspace => crate::ElementSource::Workspace,
                    source_identity::SourceRole::StandardLibrary => {
                        crate::ElementSource::StandardLibrary
                    }
                    source_identity::SourceRole::Library => crate::ElementSource::Library,
                    source_identity::SourceRole::External => crate::ElementSource::External,
                },
            })
            .collect::<Vec<_>>();
        affected.sort_by(|left, right| left.identity.cmp(&right.identity));
        let affected = affected.into_boxed_slice();
        if unsettled_dependency {
            if self
                .metadata
                .completeness
                .contains(crate::PublicationObstacle::NonConverged)
            {
                QueryOutcome::new(self.metadata.completeness, QueryAnswer::Incomplete)
            } else if self
                .metadata
                .completeness
                .contains(crate::PublicationObstacle::UnsupportedSyntax)
            {
                QueryOutcome::new(self.metadata.completeness, QueryAnswer::Resolved(affected))
            } else {
                QueryOutcome::new(self.metadata.completeness, QueryAnswer::Resolved(affected))
            }
        } else {
            self.resolved_outcome(affected)
        }
    }

    pub(crate) fn resolved_outcome<T>(&self, value: T) -> QueryOutcome<T> {
        self.query_outcome(QueryAnswer::Resolved(value))
    }

    pub(crate) fn query_outcome<T>(&self, answer: QueryAnswer<T>) -> QueryOutcome<T> {
        QueryOutcome::new(self.metadata.completeness, answer)
    }

    /// The handle for one declaration of this publication.
    ///
    /// The handle is the declaration's rank in canonical-identity order, so minting one is an
    /// array read and comparing two is an integer compare that orders exactly as comparing the
    /// two canonical identity strings did. It is valid for this publication only; a consumer that
    /// needs to survive a rebuild takes `symbol_token` instead.
    pub(crate) fn symbol_id(&self, id: DeclarationId) -> Option<SymbolId> {
        SymbolId::from_index(self.identities.rank_of(id)?)
    }

    /// The authored name of one declaration, borrowed from the symbol blob.
    pub(crate) fn authored_name(&self, id: DeclarationId) -> Option<&str> {
        self.storage.symbol(self.storage.declaration(id)?.name?)
    }

    pub(crate) fn declaration_kind(&self, id: DeclarationId) -> Option<DeclarationKind> {
        Some(self.storage.declaration(id)?.kind)
    }

    /// The authored name of the declaration's owner, borrowed from the symbol blob.
    pub(crate) fn declaration_owner_name(&self, id: DeclarationId) -> Option<&str> {
        let owner = self.storage.declaration(id)?.owner?;
        self.authored_name(owner)
    }

    /// The identity of the document that declares this element.
    pub(crate) fn declaration_document_identity(&self, id: DeclarationId) -> Option<&str> {
        let declaration = self.storage.declaration(id)?;
        Some(&self.storage.document(declaration.document)?.identity)
    }

    /// The settled range of the declaration's identifier.
    pub(crate) fn declaration_identifier_range(&self, id: DeclarationId) -> Option<TextRange> {
        self.documents.declaration_identifier(id)
    }

    /// The declaration a handle addresses, or `None` when it is not one of this publication's.
    ///
    /// A handle that ranks beyond this publication names nothing, and every query built on this
    /// answers `Unresolved` for it rather than an empty result -- an identity that names nothing
    /// is unanswerable, not false. A handle from *another* publication whose rank is still in
    /// range is indistinguishable from a valid one and will answer about whatever element now
    /// holds that rank; that is why `SymbolId` documents one-publication validity, and why
    /// `symbol_token` exists for anything that has to outlive a build.
    pub(crate) fn declaration_of(&self, symbol: SymbolId) -> Option<DeclarationId> {
        self.identities.at_rank(symbol.index())
    }

    /// One interned run of authored text, borrowed from the symbol blob.
    pub(crate) fn text(&self, id: crate::TextId) -> Option<&str> {
        self.storage
            .symbol(crate::model::NameId(u32::try_from(id.index()).ok()?))
    }

    /// The authored name behind one handle, borrowed from the symbol blob.
    pub(crate) fn symbol_name(&self, symbol: SymbolId) -> Option<&str> {
        self.authored_name(self.declaration_of(symbol)?)
    }

    /// One declaration's `::`-joined display path, borrowed from the settled blob.
    pub(crate) fn symbol_qualified_name(&self, symbol: SymbolId) -> Option<&str> {
        let id = self.declaration_of(symbol)?;
        self.qualified_names.qualified_name(id)
    }

    /// The stable structural encoding of a handle, materialised for a boundary.
    ///
    /// The encoding is derived from the owner chain, so this allocates: it is what a consumer
    /// asks for explicitly when a string has to leave the process.
    pub(crate) fn symbol_token(&self, symbol: SymbolId) -> Option<SymbolToken> {
        let id = self.declaration_of(symbol)?;
        self.identities
            .identity(&self.storage, id)
            .map(SymbolToken::from_encoded)
    }

    /// The public handle for one of this publication's stored documents.
    pub(crate) fn document_handle(&self, document: DocumentIdx) -> Option<DocumentId> {
        // The lookup proves the ordinal addresses a document of *this* publication, so a handle
        // that leaves here always resolves back through `document_identity`.
        self.storage.document(document)?;
        DocumentId::from_index(document.index())
    }

    /// The normalised identity of one document, borrowed from the settled blob.
    pub(crate) fn document_identity(&self, document: DocumentId) -> Option<&str> {
        let id = DocumentIdx(u32::try_from(document.index()).ok()?);
        Some(&self.storage.document(id)?.identity)
    }

    /// The document's identity, materialised for a boundary.
    pub(crate) fn document_token(&self, document: DocumentId) -> Option<DocumentToken> {
        self.document_identity(document)
            .map(DocumentToken::from_encoded)
    }

    /// The handle an identity string names in this publication, if it names one.
    pub(crate) fn document_of(&self, identity: &str) -> Option<DocumentId> {
        let index = self
            .storage
            .documents
            .iter()
            .position(|document| &*document.identity == identity)?;
        DocumentId::from_index(index)
    }

    /// The handle a document token names in this publication, if it still names one.
    pub(crate) fn resolve_document_token(&self, token: &DocumentToken) -> Option<DocumentId> {
        self.document_of(token.as_str())
    }

    /// Orders two locations by the identity string of their document, then their range.
    ///
    /// The comparison is on the identity, not the handle: a document ordinal is a storage slot
    /// the authority is free to reassign, and contractually ordered output must not depend on it.
    pub(crate) fn location_order(
        &self,
        left: &SourceLocation,
        right: &SourceLocation,
    ) -> std::cmp::Ordering {
        self.document_order(left.document, right.document)
            .then_with(|| left.range.cmp(&right.range))
            .then_with(|| left.role.cmp(&right.role))
    }

    pub(crate) fn document_order(&self, left: DocumentId, right: DocumentId) -> std::cmp::Ordering {
        self.document_identity(left)
            .cmp(&self.document_identity(right))
    }

    pub(crate) fn target_order(
        &self,
        left: &NavigationTarget,
        right: &NavigationTarget,
    ) -> std::cmp::Ordering {
        self.document_order(left.location.document, right.location.document)
            .then_with(|| left.location.range.cmp(&right.location.range))
            .then_with(|| {
                self.symbol_name(left.symbol)
                    .cmp(&self.symbol_name(right.symbol))
            })
    }

    /// The handle a token names in this publication, if it still names one.
    pub(crate) fn resolve_token(&self, token: &SymbolToken) -> Option<SymbolId> {
        self.identities
            .declarations(&self.storage, token.as_str())
            .first()
            .copied()
            .and_then(|id| self.symbol_id(id))
    }

    pub(crate) fn declaration_target(&self, id: DeclarationId) -> Option<NavigationTarget> {
        let declaration = self.storage.declaration(id)?;
        declaration.name?;
        Some(NavigationTarget {
            symbol: self.symbol_id(id)?,
            location: SourceLocation {
                document: self.document_handle(declaration.document)?,
                range: self.documents.declaration_identifier(id)?,
                role: OccurrenceRole::Declaration,
            },
        })
    }

    pub(crate) fn target_at(
        &self,
        document: &str,
        position: TextPosition,
    ) -> QueryOutcome<NavigationTarget> {
        let Some(document_id) = self.documents.document(&self.storage, document) else {
            return self.query_outcome(QueryAnswer::Unresolved);
        };
        let Some(positions) = self.documents.positions(document_id) else {
            return self.query_outcome(QueryAnswer::Unresolved);
        };
        let mut reference_matches = Vec::new();
        for reference_id in leaf_ranges_containing(&positions.references, position) {
            match self.resolution.outcome(reference_id) {
                Some(ResolutionStatus::Resolved(target)) => {
                    if let Some(target) = self.declaration_target(target) {
                        reference_matches.push(target);
                    }
                }
                Some(ResolutionStatus::Ambiguous(range)) => {
                    let mut targets = self
                        .resolution
                        .ambiguous_candidates(range)
                        .iter()
                        .filter_map(|id| self.declaration_target(*id))
                        .collect::<Vec<_>>();
                    targets.sort_by(|left, right| self.target_order(left, right));
                    targets.dedup_by(|a, b| a.symbol == b.symbol);
                    return self.query_outcome(QueryAnswer::Ambiguous(targets.into_boxed_slice()));
                }
                Some(ResolutionStatus::Unsupported) => {
                    return self.query_outcome(QueryAnswer::Unsupported)
                }
                Some(ResolutionStatus::NonConverged) => {
                    return self.query_outcome(QueryAnswer::Incomplete)
                }
                Some(ResolutionStatus::Unresolved) | None => {
                    return self.query_outcome(QueryAnswer::Unresolved)
                }
            }
        }
        reference_matches.sort_by(|left, right| self.target_order(left, right));
        reference_matches.dedup_by(|a, b| a.symbol == b.symbol);
        if reference_matches.len() == 1 {
            return self.resolved_outcome(reference_matches.remove(0));
        }
        if reference_matches.len() > 1 {
            return self
                .query_outcome(QueryAnswer::Ambiguous(reference_matches.into_boxed_slice()));
        }
        let mut declarations = leaf_ranges_containing(&positions.identifiers, position)
            .filter_map(|id| self.declaration_target(id))
            .collect::<Vec<_>>();
        declarations.sort_by(|left, right| self.target_order(left, right));
        match declarations.len() {
            0 => self.query_outcome(QueryAnswer::Unresolved),
            1 => self.resolved_outcome(declarations.remove(0)),
            _ => self.query_outcome(QueryAnswer::Ambiguous(declarations.into_boxed_slice())),
        }
    }

    pub(crate) fn references(
        &self,
        symbol: SymbolId,
        include_declaration: bool,
    ) -> QueryOutcome<Box<[SourceLocation]>> {
        if self
            .metadata
            .completeness
            .contains(crate::PublicationObstacle::NonConverged)
        {
            return self.query_outcome(QueryAnswer::Incomplete);
        }
        let Some(target) = self.declaration_of(symbol) else {
            return self.query_outcome(QueryAnswer::Unresolved);
        };
        self.references_for(target, include_declaration)
    }

    pub(crate) fn references_for(
        &self,
        target: DeclarationId,
        include_declaration: bool,
    ) -> QueryOutcome<Box<[SourceLocation]>> {
        let Some(target_declaration) = self.storage.declaration(target) else {
            return self.query_outcome(QueryAnswer::Unresolved);
        };
        let mut locations = Vec::new();
        if include_declaration {
            if let Some(target) = self.declaration_target(target) {
                locations.push(target.location);
            }
        }
        let references = self.reverse_references.references(target);
        record_visited_index_entries(references.len());
        for id in references {
            let Some(reference) = self.storage.references.get(id.index()) else {
                return self.query_outcome(QueryAnswer::Incomplete);
            };
            let Some(source) = self.storage.declaration(reference.source) else {
                return self.query_outcome(QueryAnswer::Incomplete);
            };
            // An unnamed target has no identifier to point at, so the whole reference span is the
            // honest location -- the same answer the text search used to fall back to.
            let range = match target_declaration
                .name
                .and_then(|name| self.documents.reference_identifier(*id, name))
            {
                Some(range) => range,
                None => match document_range(&self.storage, source.document, &reference.span) {
                    Ok(range) => range,
                    Err(_) => return self.query_outcome(QueryAnswer::Incomplete),
                },
            };
            locations.push(SourceLocation {
                document: match self.document_handle(source.document) {
                    Some(document) => document,
                    None => return self.query_outcome(QueryAnswer::Incomplete),
                },
                range,
                role: OccurrenceRole::Reference,
            });
        }
        locations.sort_by(|left, right| self.location_order(left, right));
        locations.dedup();
        self.resolved_outcome(locations.into_boxed_slice())
    }

    pub(crate) fn prepare_rename(
        &self,
        document: &str,
        position: TextPosition,
        new_name: Option<&str>,
    ) -> RenameOutcome {
        // An edit must never be derived from a publication whose semantic facts are partial.
        // Read-only consumers may deliberately present partial values, but applying a rename
        // makes those values authoritative in source and therefore requires a complete model.
        if !self.metadata.completeness.is_complete() {
            return RenameOutcome::Incomplete;
        }
        let target = match self.target_at(document, position).answer {
            QueryAnswer::Resolved(target) => target,
            QueryAnswer::Ambiguous(targets) => return RenameOutcome::Ambiguous(targets),
            QueryAnswer::Unsupported => return RenameOutcome::Unsupported,
            QueryAnswer::Recovery => return RenameOutcome::Recovery,
            QueryAnswer::Incomplete => return RenameOutcome::Incomplete,
            QueryAnswer::Unresolved => return RenameOutcome::Unresolved,
        };
        if let Some(name) = new_name {
            if !valid_identifier(name) {
                return RenameOutcome::InvalidName;
            }
            let Some(id) = self.declaration_of(target.symbol) else {
                return RenameOutcome::Incomplete;
            };
            let Some(declaration) = self.storage.declaration(id) else {
                return RenameOutcome::Incomplete;
            };
            if let Some(symbol) = self.storage.symbols.find(name) {
                let mut collisions = self
                    .direct_names
                    .candidates(declaration.owner, symbol)
                    .iter()
                    .filter(|candidate| **candidate != id)
                    .filter_map(|candidate| self.declaration_target(*candidate))
                    .collect::<Vec<_>>();
                collisions.sort_by(|left, right| self.target_order(left, right));
                if !collisions.is_empty() {
                    return RenameOutcome::Collision(collisions.into_boxed_slice());
                }
            }
        }
        let occurrences = match self.references(target.symbol, true).answer {
            QueryAnswer::Resolved(value) => value,
            _ => return RenameOutcome::Incomplete,
        };
        let range = occurrences
            .iter()
            .find(|location| {
                self.document_identity(location.document) == Some(document)
                    && range_contains(location.range, position)
            })
            .map(|location| location.range)
            .unwrap_or(target.location.range);
        RenameOutcome::Ready {
            symbol: target.symbol,
            range,
            occurrences,
        }
    }

    pub(crate) fn resolve_qualifier_scopes(
        &self,
        owner: Option<DeclarationId>,
        qualifier: &str,
    ) -> Result<Vec<DeclarationId>, ResolutionError> {
        let Some(segments) = qualifier
            .split("::")
            .filter(|segment| !segment.is_empty())
            .map(|segment| self.storage.symbols.find(segment))
            .collect::<Option<Vec<_>>>()
        else {
            return Ok(Vec::new());
        };
        let Some(first) = segments.first().copied() else {
            return Ok(Vec::new());
        };
        let mut candidates = Vec::new();
        let mut work = ResolutionWork::default();
        lookup_lexical_into(
            &self.storage.declarations,
            &ResolutionIndexes {
                direct_names: &self.direct_names,
                exported_names: &self.direct_names,
                effective_imports: Some(&self.effective_imports),
                exported_imports: Some(&self.effective_imports),
                inherited_names: Some(&self.resolution.inherited_names),
            },
            owner,
            first,
            // An interactive lookup has no redefining feature to exclude.
            LookupTarget {
                domain: DeclarationDomain::Any,
                excluded: None,
            },
            &mut candidates,
            &mut work,
        )?;
        let mut next = Vec::new();
        for segment in &segments[1..] {
            next.clear();
            for candidate in candidates.iter().copied() {
                let direct = self.direct_names.candidates(Some(candidate), *segment);
                if direct.is_empty() {
                    next.extend_from_slice(
                        self.effective_imports.candidates(Some(candidate), *segment),
                    );
                } else {
                    next.extend_from_slice(direct);
                }
            }
            next.sort_unstable();
            next.dedup();
            std::mem::swap(&mut candidates, &mut next);
        }
        Ok(candidates)
    }

    /// The documents whose diagnostics this publication derives, in canonical order.
    ///
    /// Every workspace-authored document, plus any admitted document the host explicitly named.
    ///
    /// The default is provenance: a workspace does not inherit its library's diagnostics, and
    /// deriving every admitted document would make the barrier cost the whole library on every
    /// rebuild. But provenance is not the same question as *authoring surface*: an editor with a
    /// library file open is authoring it, and only the host knows that. Naming the document is how
    /// it says so, and it is a build input rather than a query option because a diagnostic must be
    /// settled before the publication is visible.
    pub(crate) fn reported_document_indices(&self, reported: &[Box<str>]) -> Vec<usize> {
        let mut indices = (0..self.storage.documents.len())
            .filter(|index| {
                let document = &self.storage.documents[*index];
                document.role == source_identity::SourceRole::Workspace
                    || reported
                        .iter()
                        .any(|identity| identity.as_ref() == document.identity.as_ref())
            })
            .collect::<Vec<_>>();
        indices.sort_by(|left, right| {
            self.storage.documents[*left]
                .identity
                .cmp(&self.storage.documents[*right].identity)
                .then_with(|| left.cmp(right))
        });
        indices
    }

    /// Every declaration each admitted document authored, indexed by document.
    ///
    /// One pass over storage, so the diagnostic barrier costs the corpus once rather than once per
    /// document per rule.
    pub(crate) fn declarations_by_document(
        &self,
    ) -> Result<Vec<Vec<DeclarationId>>, ResolutionError> {
        let mut by_document = vec![Vec::new(); self.storage.documents.len()];
        for index in 0..self.storage.declarations.len() {
            let id = DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
            let declaration = self
                .storage
                .declaration(id)
                .ok_or(ResolutionError::InvalidStorage)?;
            by_document
                .get_mut(declaration.document.index())
                .ok_or(ResolutionError::InvalidStorage)?
                .push(id);
        }
        Ok(by_document)
    }
}

impl ResolvedSemanticModel {
    pub(crate) fn visible_members(
        &self,
        document: &str,
        position: TextPosition,
        qualifier: Option<&str>,
    ) -> QueryOutcome<VisibleMembers<'_>> {
        let recovered = self
            .metadata
            .completeness
            .contains(crate::PublicationObstacle::ParseRecovery);
        let unsupported = self
            .metadata
            .completeness
            .contains(crate::PublicationObstacle::UnsupportedSyntax);
        if self
            .metadata
            .completeness
            .contains(crate::PublicationObstacle::NonConverged)
        {
            return self.query_outcome(QueryAnswer::Incomplete);
        }
        let Some(document_id) = self.documents.document(&self.storage, document) else {
            return self.query_outcome(QueryAnswer::Unresolved);
        };
        let Some(positions) = self.documents.positions(document_id) else {
            return self.query_outcome(QueryAnswer::Unresolved);
        };
        let owner = positions.spans.innermost_containing(position);
        let mut ids = Vec::new();
        if let Some(qualifier) = qualifier {
            let scopes = match self.resolve_qualifier_scopes(owner, qualifier) {
                Ok(scopes) if !scopes.is_empty() => scopes,
                Ok(_) => return self.query_outcome(QueryAnswer::Unresolved),
                Err(_) => return self.query_outcome(QueryAnswer::Incomplete),
            };
            if scopes.len() > 1 {
                let candidates = scopes
                    .into_iter()
                    .map(|scope| {
                        self.visible_member_records(self.effective_scopes.members(Some(scope)))
                    })
                    .collect::<Vec<_>>()
                    .into_boxed_slice();
                return self.query_outcome(QueryAnswer::Ambiguous(candidates));
            }
            ids.extend_from_slice(self.effective_scopes.members(Some(scopes[0])));
        } else {
            let mut scope = owner;
            loop {
                ids.extend_from_slice(self.effective_scopes.members(scope));
                let Some(current) = scope else {
                    break;
                };
                scope = self
                    .storage
                    .declaration(current)
                    .and_then(|declaration| declaration.owner);
            }
        }
        let members = self.visible_member_records(&ids);
        if recovered {
            self.query_outcome(QueryAnswer::Resolved(members))
        } else if unsupported {
            self.query_outcome(QueryAnswer::Resolved(members))
        } else {
            self.query_outcome(QueryAnswer::Resolved(members))
        }
    }

    /// The declarations a completion request can see, ordered and filtered once.
    ///
    /// Nothing is materialised: the result carries handles, and every string a caller reads is a
    /// slice of a blob this publication already holds. The filter keeps exactly the declarations
    /// whose facts are all settled, which is what makes [`VisibleMemberRef`]'s accessors total.
    pub(crate) fn visible_member_records(&self, ids: &[DeclarationId]) -> VisibleMembers<'_> {
        let mut ids = ids.to_vec();
        ids.sort_unstable();
        ids.dedup();
        ids.retain(|id| {
            self.symbol_id(*id).is_some()
                && self.authored_name(*id).is_some()
                && self.declaration_document_identity(*id).is_some()
                && self.declaration_identifier_range(*id).is_some()
        });
        ids.sort_by(|left, right| {
            self.authored_name(*left)
                .cmp(&self.authored_name(*right))
                .then_with(|| {
                    self.declaration_document_identity(*left)
                        .cmp(&self.declaration_document_identity(*right))
                })
                .then_with(|| {
                    self.declaration_identifier_range(*left)
                        .cmp(&self.declaration_identifier_range(*right))
                })
        });
        VisibleMembers::new(self, ids.into_boxed_slice())
    }

    /// The diagnostics one document owns, as the settled slice rather than a filtered scan.
    ///
    /// A document this publication did not admit has no diagnostics, which is a different answer
    /// from "no diagnostic was reported": the caller asked about a document that is not part of
    /// this model, and the empty slice says so alongside the publication's completeness.
    pub(crate) fn published_document_diagnostics(
        &self,
        document: &str,
    ) -> PublishedDiagnostics<'_> {
        let diagnostics = match self.documents.document(&self.storage, document) {
            Some(id) => match self.diagnostics.by_document.get(id.index()) {
                Some((start, end)) => self
                    .diagnostics
                    .diagnostics
                    .get(*start as usize..*end as usize)
                    .unwrap_or_default(),
                None => &[],
            },
            None => &[],
        };
        PublishedDiagnostics::new(self.completeness(), diagnostics)
    }

    pub(crate) fn published_diagnostics(&self) -> PublishedDiagnostics<'_> {
        PublishedDiagnostics::new(self.completeness(), &self.diagnostics.diagnostics)
    }

    pub(crate) fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics.diagnostics
    }
    pub(crate) fn write_semantic_sexpr(
        &self,
        source_digest: &source_identity::RootDigest,
        semantic_contract_version: &str,
        output: &mut dyn std::fmt::Write,
    ) -> std::fmt::Result {
        writer::write_semantic(self, source_digest, semantic_contract_version, output)
    }

    pub(crate) fn write_navigation_sexpr(
        &self,
        output: &mut dyn std::fmt::Write,
    ) -> std::fmt::Result {
        writer::write_navigation_only(self, output)
    }

    pub(crate) fn write_diagnostics_sexpr(
        &self,
        output: &mut dyn std::fmt::Write,
    ) -> std::fmt::Result {
        writer::write_diagnostics(self, output)
    }

    pub(crate) fn write_types_sexpr(&self, output: &mut dyn std::fmt::Write) -> std::fmt::Result {
        writer::write_types_only(self, output)
    }
}

impl<D> SemanticModel<D> {
    /// Resolves one published identity to the single declaration it names.
    ///
    /// Every type query needs this, and every one of them owes the caller the same three explicit
    /// answers: the publication did not converge, the identity names nothing, or it names several
    /// identically authored siblings and choosing between them would be a guess.
    pub(crate) fn single_declaration<T>(
        &self,
        symbol: SymbolId,
    ) -> Result<DeclarationId, QueryOutcome<T>> {
        if self
            .metadata
            .completeness
            .contains(crate::PublicationObstacle::NonConverged)
        {
            return Err(self.query_outcome(QueryAnswer::Incomplete));
        }
        self.declaration_of(symbol)
            .ok_or(self.query_outcome(QueryAnswer::Unresolved))
    }

    pub(crate) fn symbols(
        &self,
        declarations: impl Iterator<Item = DeclarationId>,
    ) -> Box<[SymbolId]> {
        let mut symbols = declarations
            .filter_map(|id| self.symbol_id(id))
            .collect::<Vec<_>>();
        symbols.sort();
        symbols.dedup();
        symbols.into_boxed_slice()
    }

    pub(crate) fn direct_types(&self, symbol: SymbolId) -> QueryOutcome<Box<[TypeReference]>> {
        let declaration = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let mut types = self
            .types
            .direct_types(declaration)
            .iter()
            .filter_map(|(target, provenance)| {
                Some(TypeReference {
                    symbol: self.symbol_id(*target)?,
                    provenance: match provenance {
                        types::FactProvenance::Authored => RelationshipProvenance::Authored,
                        types::FactProvenance::Implied => RelationshipProvenance::Implied,
                    },
                })
            })
            .collect::<Vec<_>>();
        types.sort_by_key(|left| left.symbol);
        self.resolved_outcome(types.into_boxed_slice())
    }

    /// Projects one exact KerML Feature relationship collection from the canonical relationship
    /// index. No relationship is re-derived here: authored and implied edges, their provenance,
    /// and target-resolution state are all the same facts an element inspection publishes.
    pub(crate) fn feature_derived_relationships(
        &self,
        symbol: SymbolId,
        collection: FeatureDerivedRelationshipCollection,
    ) -> QueryOutcome<Box<[ElementRelationship]>> {
        let declaration = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let Some(rule) = feature_derived_relationship_rule(collection) else {
            // A public enum value with no generated pinned-manifest contract is not a silently
            // empty collection. It is an incomplete implementation boundary.
            return self.query_outcome(QueryAnswer::Unsupported);
        };
        if rule.metaclass != "Feature"
            || self
                .memberships
                .get(declaration)
                .is_none_or(|membership| membership.kind != MembershipKind::Feature)
        {
            // The rule is defined on raw KerML Feature. A non-feature source has no valid empty
            // answer, and a lowering projection we do not yet classify as a Feature must remain
            // explicit rather than being accepted by display name or owning syntax.
            return self.query_outcome(QueryAnswer::Unsupported);
        }
        let _rule_id = rule.rule_id;
        self.resolved_outcome(
            self.relationships_of_kinds(
                declaration,
                feature_derived_relationship_kinds(collection),
            ),
        )
    }

    /// Projects one exact KerML Type relationship collection or operand projection from the
    /// canonical relationship index. Operand queries intentionally return their relationships,
    /// retaining authored/implied provenance and unresolved target state.
    pub(crate) fn type_derived_relationships(
        &self,
        symbol: SymbolId,
        collection: TypeDerivedRelationshipCollection,
    ) -> QueryOutcome<Box<[ElementRelationship]>> {
        let declaration = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let Some(rule) = type_derived_relationship_rule(collection) else {
            return self.query_outcome(QueryAnswer::Unsupported);
        };
        if rule.metaclass != "Type"
            || self
                .storage
                .declaration(declaration)
                .is_none_or(|declaration| !DeclarationDomain::Type.accepts(declaration.kind))
        {
            return self.query_outcome(QueryAnswer::Unsupported);
        }
        let _rule_id = rule.rule_id;
        self.resolved_outcome(
            self.relationships_of_kinds(declaration, type_derived_relationship_kinds(collection)),
        )
    }

    /// Projects one exact final Type member-element collection from canonical declaration owner
    /// membership, and modifier facts. It deliberately does not reconstruct `FeatureMembership`
    /// objects or consult source spelling, because callers ask for the selected member elements
    /// themselves.
    pub(crate) fn type_derived_elements(
        &self,
        symbol: SymbolId,
        collection: TypeDerivedElementCollection,
    ) -> QueryOutcome<Box<[SymbolId]>> {
        let declaration = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let Some(rule) = type_derived_element_rule(collection) else {
            return self.query_outcome(QueryAnswer::Unsupported);
        };
        if rule.metaclass != "Type"
            || self
                .storage
                .declaration(declaration)
                .is_none_or(|value| !DeclarationDomain::Type.accepts(value.kind))
        {
            return self.query_outcome(QueryAnswer::Unsupported);
        }
        let _rule_id = rule.rule_id;
        let values = self.symbols(
            self.storage
                .declarations
                .iter()
                .enumerate()
                .filter_map(|(index, candidate_declaration)| {
                    let candidate = DeclarationId::from_index(index).ok()?;
                    (candidate != declaration && candidate_declaration.owner == Some(declaration))
                        .then_some(candidate)
                })
                .filter(|candidate| match collection {
                    TypeDerivedElementCollection::OwnedFeature => self
                        .memberships
                        .get(*candidate)
                        .is_some_and(|membership| membership.kind == MembershipKind::Feature),
                    TypeDerivedElementCollection::OwnedEndFeature => {
                        self.memberships
                            .get(*candidate)
                            .is_some_and(|membership| membership.kind == MembershipKind::Feature)
                            && self
                                .storage
                                .declaration_facts(*candidate)
                                .is_some_and(|facts| facts.modifiers.end)
                    }
                }),
        );
        self.resolved_outcome(values)
    }

    /// Every feature membership one Type owns directly, in canonical declaration order.
    pub(crate) fn owned_feature_members(&self, owner: DeclarationId) -> Vec<DeclarationId> {
        self.storage
            .declarations
            .iter()
            .enumerate()
            .filter_map(|(index, candidate_declaration)| {
                let candidate = DeclarationId::from_index(index).ok()?;
                (candidate != owner
                    && candidate_declaration.owner == Some(owner)
                    && self
                        .memberships
                        .get(candidate)
                        .is_some_and(|membership| membership.kind == MembershipKind::Feature))
                .then_some(candidate)
            })
            .collect()
    }

    /// Records every member `member` redefines, from settled `Redefinition` facts.
    ///
    /// Authored and implied redefinitions are read alike: an implied redefinition is a canonical
    /// relationship of the publication, and a member it replaces is inherited no more than one a
    /// written `:>>` replaces.
    pub(crate) fn collect_redefined_members(
        &self,
        member: DeclarationId,
        into: &mut std::collections::BTreeSet<DeclarationId>,
    ) {
        for reference_id in self.outgoing_reference_ids(member) {
            if self.storage.references[reference_id.index()].kind != ReferenceKind::Redefinition {
                continue;
            }
            if let Some(ResolutionStatus::Resolved(target)) = self.resolution.outcome(*reference_id)
            {
                into.insert(target);
            }
        }
        for index in self.outgoing_implied_indices(member) {
            let implied = &self.resolution.implied_relationships[*index as usize];
            if implied.kind == ReferenceKind::Redefinition {
                into.insert(implied.target);
            }
        }
    }

    /// The canonical inherited FeatureMembership closure of one Type.
    ///
    /// KerML derives `inheritedMembership` from the memberships of the general types reached
    /// through specialization, minus the ones a nearer member redefines. The closure itself is the
    /// specialization index this resolver already owns, so a member reached through two paths is
    /// inherited once and a cyclic hierarchy -- which has no closure -- inherits nothing.
    pub(crate) fn inherited_feature_members(
        &self,
        declaration: DeclarationId,
    ) -> Vec<DeclarationId> {
        let ancestors = self
            .types
            .specialization()
            .scoped_ancestors(declaration)
            .map(|(ancestor, _)| ancestor)
            .collect::<Vec<_>>();
        if ancestors.is_empty() {
            return Vec::new();
        }
        let mut candidates = Vec::new();
        for ancestor in ancestors {
            candidates.extend(self.owned_feature_members(ancestor));
        }
        let owned = self.owned_feature_members(declaration);
        let mut redefined = std::collections::BTreeSet::new();
        for member in owned.iter().chain(candidates.iter()) {
            self.collect_redefined_members(*member, &mut redefined);
        }
        candidates.retain(|candidate| !redefined.contains(candidate));
        candidates
    }

    /// The effective `Definition::usage` / `Usage::usage` collection.
    ///
    /// This is the sole owner of the owned-plus-inherited FeatureMembership closure filtered to
    /// SysML Usage metaclasses. Definition/Usage projections and more-specific derivations such as
    /// `ActionDefinition::action` consume this result instead of rebuilding the closure.
    pub(crate) fn effective_usage_members(&self, declaration: DeclarationId) -> Vec<DeclarationId> {
        self.owned_feature_members(declaration)
            .into_iter()
            .chain(self.inherited_feature_members(declaration))
            .filter(|member| {
                self.storage
                    .declaration(*member)
                    .is_some_and(|member| is_usage_declaration(member.kind))
            })
            .collect()
    }

    /// Whether one member of a Type belongs to the selected derived collection.
    pub(crate) fn type_derived_fact_selects(
        &self,
        collection: TypeDerivedFactCollection,
        member: DeclarationId,
    ) -> bool {
        let facts = self.storage.declaration_facts(member);
        match collection {
            TypeDerivedFactCollection::OwnedFeatureMembership
            | TypeDerivedFactCollection::Multiplicity
            | TypeDerivedFactCollection::OwnedConjugator => false,
            TypeDerivedFactCollection::FeatureMembership
            | TypeDerivedFactCollection::Feature
            | TypeDerivedFactCollection::InheritedMembership
            | TypeDerivedFactCollection::InheritedFeature => true,
            TypeDerivedFactCollection::EndFeature => facts.is_some_and(|facts| facts.modifiers.end),
            TypeDerivedFactCollection::DirectedFeature => {
                facts.is_some_and(|facts| facts.direction.is_some())
            }
            TypeDerivedFactCollection::Input => facts.is_some_and(|facts| {
                matches!(
                    facts.direction,
                    Some(ParameterDirection::In | ParameterDirection::InOut)
                )
            }),
            TypeDerivedFactCollection::Output => facts.is_some_and(|facts| {
                matches!(
                    facts.direction,
                    Some(ParameterDirection::Out | ParameterDirection::InOut)
                )
            }),
        }
    }

    /// Returns one exact Type derivation over canonical membership and specialization-closure
    /// facts, or the explicit first missing prerequisite for the derivations whose normative
    /// result is a relationship or multiplicity identity that compact declaration-aligned storage
    /// does not own.
    pub(crate) fn type_derived_fact(
        &self,
        symbol: SymbolId,
        collection: TypeDerivedFactCollection,
    ) -> QueryOutcome<TypeDerivedFactOutcome> {
        let declaration = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let Some(rule) = type_derived_fact_rule(collection) else {
            return self.resolved_outcome(TypeDerivedFactOutcome::Unsupported {
                prerequisite: TypeDerivedFactPrerequisite::RuleNotPublished,
            });
        };
        if rule.metaclass != "Type"
            || self
                .storage
                .declaration(declaration)
                .is_none_or(|value| !DeclarationDomain::Type.accepts(value.kind))
        {
            return self.query_outcome(QueryAnswer::Unsupported);
        }
        let _rule_id = rule.rule_id;
        let unavailable = match collection {
            TypeDerivedFactCollection::OwnedFeatureMembership => {
                Some(TypeDerivedFactPrerequisite::FeatureMembershipIdentity)
            }
            TypeDerivedFactCollection::Multiplicity => {
                Some(TypeDerivedFactPrerequisite::MultiplicityIdentity)
            }
            _ => None,
        };
        if let Some(prerequisite) = unavailable {
            return self.resolved_outcome(TypeDerivedFactOutcome::Unsupported { prerequisite });
        }
        if collection == TypeDerivedFactCollection::OwnedConjugator {
            // `ownedConjugator` is the one `Conjugation` a type owns (KerML 8.3.3.1.10); the
            // lowering sources it at the conjugated type, so the authored reference *is* the
            // owned relationship and its settled target is `originalType`.
            let values = self
                .storage
                .references
                .iter()
                .enumerate()
                .filter(|(_, reference)| {
                    reference.source == declaration && reference.kind == ReferenceKind::Conjugation
                })
                .filter_map(|(index, _)| {
                    let id = AuthoredReferenceId::from_index(index).ok()?;
                    match self.resolution.outcome(id)? {
                        ResolutionStatus::Resolved(target) => {
                            Some(TypeDerivedFactValue::Conjugator {
                                original_type: self.symbol_id(target)?,
                            })
                        }
                        _ => None,
                    }
                })
                .collect::<Vec<_>>();
            return self
                .resolved_outcome(TypeDerivedFactOutcome::Values(values.into_boxed_slice()));
        }
        let inherited = self.inherited_feature_members(declaration);
        let members = match collection {
            TypeDerivedFactCollection::InheritedMembership
            | TypeDerivedFactCollection::InheritedFeature => inherited,
            _ => self
                .owned_feature_members(declaration)
                .into_iter()
                .chain(inherited)
                .collect(),
        };
        // `FeatureMembership`-valued collections still name their member element, never a
        // fabricated Membership relationship identity: that identity remains unpublished, and its
        // own derivation stays explicitly unsupported above.
        let membership_valued = matches!(
            collection,
            TypeDerivedFactCollection::InheritedMembership
                | TypeDerivedFactCollection::FeatureMembership
        );
        let values = self
            .symbols(
                members
                    .into_iter()
                    .filter(|member| self.type_derived_fact_selects(collection, *member)),
            )
            .into_vec()
            .into_iter()
            .map(|member| {
                if membership_valued {
                    TypeDerivedFactValue::FeatureMembership { member }
                } else {
                    TypeDerivedFactValue::Feature(member)
                }
            })
            .collect::<Vec<_>>();
        self.resolved_outcome(TypeDerivedFactOutcome::Values(values.into_boxed_slice()))
    }

    /// Returns one exact Systems::DefinitionAndUsage derived property from the canonical direct
    /// declaration owner, feature-membership, kind, and modifier facts.
    ///
    /// This intentionally stops at the first unavailable owner for broader inherited-feature and
    /// time-variation predicates. A direct child scan is never substituted for an inherited
    /// collection. Variant derivations consume the canonical membership role and return the
    /// relationship's distinct identity where the normative property is relationship-valued.
    pub(crate) fn definition_usage_derived(
        &self,
        symbol: SymbolId,
        kind: DefinitionUsageDerivedKind,
    ) -> QueryOutcome<DefinitionUsageDerivedOutcome> {
        let declaration = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let Some(rule) = definition_usage_derived_rule(kind) else {
            return self.resolved_outcome(DefinitionUsageDerivedOutcome::Unsupported {
                prerequisite: DefinitionUsageDerivedPrerequisite::RuleNotPublished,
            });
        };
        let Some(source) = self.storage.declaration(declaration) else {
            return self.query_outcome(QueryAnswer::Incomplete);
        };
        if !definition_usage_source_matches(rule.metaclass, source.kind) {
            return self.query_outcome(QueryAnswer::Unsupported);
        }
        let _rule_id = rule.rule_id;
        match kind {
            // `usage` and `directedUsage` select over the *effective* feature membership of the
            // definition or usage -- everything it owns plus everything it inherits -- so they
            // read the same canonical specialization closure `Type::inheritedMembership` does,
            // rather than the direct child scan the `owned`/`nested` collections below use.
            DefinitionUsageDerivedKind::DefinitionDirectedUsage
            | DefinitionUsageDerivedKind::UsageDirectedUsage
            | DefinitionUsageDerivedKind::DefinitionUsage
            | DefinitionUsageDerivedKind::UsageUsage => {
                let directed = matches!(
                    kind,
                    DefinitionUsageDerivedKind::DefinitionDirectedUsage
                        | DefinitionUsageDerivedKind::UsageDirectedUsage
                );
                let values = self.symbols(
                    self.effective_usage_members(declaration)
                        .into_iter()
                        .filter(|member| {
                            !directed
                                || self
                                    .storage
                                    .declaration_facts(*member)
                                    .is_some_and(|facts| facts.direction.is_some())
                        }),
                );
                self.resolved_outcome(DefinitionUsageDerivedOutcome::Elements(values))
            }
            DefinitionUsageDerivedKind::DefinitionVariant
            | DefinitionUsageDerivedKind::UsageVariant => {
                let values =
                    self.symbols(self.child_declarations(declaration).iter().copied().filter(
                        |candidate| {
                            self.effective_membership_role(*candidate)
                                == Some(crate::MembershipRole::Variant)
                                && self
                                    .storage
                                    .declaration(*candidate)
                                    .is_some_and(|member| is_usage_declaration(member.kind))
                        },
                    ));
                self.resolved_outcome(DefinitionUsageDerivedOutcome::Elements(values))
            }
            DefinitionUsageDerivedKind::DefinitionVariantMembership
            | DefinitionUsageDerivedKind::UsageVariantMembership => {
                let values = self
                    .child_declarations(declaration)
                    .iter()
                    .copied()
                    .filter(|candidate| {
                        self.effective_membership_role(*candidate)
                            == Some(crate::MembershipRole::Variant)
                    })
                    .filter_map(|candidate| crate::MembershipId::from_index(candidate.index()))
                    .collect::<Vec<_>>()
                    .into_boxed_slice();
                self.resolved_outcome(DefinitionUsageDerivedOutcome::Memberships(values))
            }
            DefinitionUsageDerivedKind::UsageMayTimeVary => {
                match self.types.usage_may_time_vary(&self.storage, declaration) {
                    crate::index::types::UsageTimeVariationOutcome::Resolved(value) => {
                        self.resolved_outcome(DefinitionUsageDerivedOutcome::Boolean(value))
                    }
                    crate::index::types::UsageTimeVariationOutcome::Unresolved => {
                        self.query_outcome(QueryAnswer::Unresolved)
                    }
                    crate::index::types::UsageTimeVariationOutcome::Ambiguous => self
                        .query_outcome(QueryAnswer::Ambiguous(
                            vec![
                                DefinitionUsageDerivedOutcome::Boolean(false),
                                DefinitionUsageDerivedOutcome::Boolean(true),
                            ]
                            .into_boxed_slice(),
                        )),
                }
            }
            DefinitionUsageDerivedKind::UsageIsReference => {
                match self.types.usage_is_reference(&self.storage, declaration) {
                    Some(is_reference) => {
                        self.resolved_outcome(DefinitionUsageDerivedOutcome::Boolean(is_reference))
                    }
                    None => self.query_outcome(QueryAnswer::Unresolved),
                }
            }
            _ => {
                // The members of one element, from the settled owner->member index: a query about
                // one declaration's features costs its features, not the corpus.
                let values =
                    self.symbols(self.child_declarations(declaration).iter().copied().filter(
                        |candidate_id| {
                            *candidate_id != declaration
                                && self
                                    .memberships
                                    .get(*candidate_id)
                                    .is_some_and(|membership| {
                                        membership.kind == MembershipKind::Feature
                                    })
                                && self.storage.declaration(*candidate_id).is_some_and(
                                    |candidate| {
                                        definition_usage_candidate_matches(kind, candidate.kind)
                                    },
                                )
                        },
                    ));
                self.resolved_outcome(DefinitionUsageDerivedOutcome::Elements(values))
            }
        }
    }

    /// Returns one exact Systems::Requirements projection from published feature-membership
    /// roles or documentation records. The generated row fixes both source metaclass and
    /// property identity, while this implementation reads only canonical lowered facts.
    pub(crate) fn requirement_derived_fact(
        &self,
        symbol: SymbolId,
        collection: RequirementDerivedFactCollection,
    ) -> QueryOutcome<RequirementDerivedFactOutcome> {
        let declaration = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let Some(rule) = requirement_derived_fact_rule(collection) else {
            return self.resolved_outcome(RequirementDerivedFactOutcome::Unsupported {
                prerequisite: RequirementDerivedFactPrerequisite::RuleNotPublished,
            });
        };
        let Some(source) = self.storage.declaration(declaration) else {
            return self.query_outcome(QueryAnswer::Incomplete);
        };
        if !requirement_derived_source_matches(rule.metaclass, source.kind) {
            return self.query_outcome(QueryAnswer::Unsupported);
        }
        let _rule_id = rule.rule_id;
        if collection.requires_text() {
            let values = self
                .documentation(declaration)
                .into_vec()
                .into_iter()
                .filter(|value| value.form == InspectionAnnotationForm::Documentation)
                .filter_map(|value| self.text(value.text).map(Box::<str>::from))
                .collect::<Vec<_>>()
                .into_boxed_slice();
            return self.resolved_outcome(RequirementDerivedFactOutcome::Text(values));
        }
        let Some(role) = requirement_derived_membership_role(collection) else {
            return self.resolved_outcome(RequirementDerivedFactOutcome::Unsupported {
                prerequisite: RequirementDerivedFactPrerequisite::CanonicalMembershipRole,
            });
        };
        // Owner-scoped from the settled index rather than a corpus scan.
        let values = self.symbols(self.child_declarations(declaration).iter().copied().filter(
            |candidate_id| {
                self.memberships
                    .get(*candidate_id)
                    .is_some_and(|membership| membership.kind == MembershipKind::Feature)
                    && self
                        .storage
                        .declaration(*candidate_id)
                        .is_some_and(|candidate| {
                            element_kind::membership_role(candidate.kind) == Some(role)
                        })
            },
        ));
        self.resolved_outcome(RequirementDerivedFactOutcome::Elements(values))
    }

    /// The exact Actions derivation boundary. Canonical effective usages and the ordered argument
    /// sites guaranteed by assignment/for-loop lowering resolve here; other collections return
    /// their first unavailable canonical prerequisite rather than guessing.
    pub(crate) fn action_derived_fact(
        &self,
        symbol: SymbolId,
        collection: ActionDerivedFactCollection,
    ) -> QueryOutcome<ActionDerivedFactOutcome> {
        let declaration = match self.single_declaration(symbol) {
            Ok(value) => value,
            Err(outcome) => return outcome,
        };
        let Some(rule) = action_derived_fact_rule(collection) else {
            return self.resolved_outcome(ActionDerivedFactOutcome::Unsupported {
                prerequisite: ActionDerivedFactPrerequisite::RuleNotPublished,
            });
        };
        let _rule_id = rule.rule_id;
        if collection == ActionDerivedFactCollection::ActionDefinitionAction {
            let values = self.symbols(
                self.effective_usage_members(declaration)
                    .into_iter()
                    .filter(|member| {
                        self.storage
                            .declaration(*member)
                            .is_some_and(|member| is_action_usage_declaration(member.kind))
                    }),
            );
            return self.resolved_outcome(ActionDerivedFactOutcome::Values(values));
        }
        let source_kind = self
            .storage
            .declaration(declaration)
            .map(|value| value.kind);
        let argument_position = match (collection, source_kind) {
            (
                ActionDerivedFactCollection::AssignmentTargetArgument,
                Some(DeclarationKind::Assign),
            )
            | (ActionDerivedFactCollection::ForLoopSeqArgument, Some(DeclarationKind::ForLoop)) => {
                Some(1)
            }
            (
                ActionDerivedFactCollection::AssignmentValueExpression,
                Some(DeclarationKind::Assign),
            ) => Some(2),
            _ => None,
        };
        if let Some(position) = argument_position {
            return self.resolved_outcome(ActionDerivedFactOutcome::Arguments(
                vec![crate::ActionArgumentId {
                    action: symbol,
                    position,
                }]
                .into_boxed_slice(),
            ));
        }
        let action_parameter =
            |position| {
                self.symbols(self.child_declarations(declaration).iter().copied().filter(
                    |member| {
                        self.storage
                            .declaration_facts(*member)
                            .is_some_and(|facts| {
                                facts.action_input_parameter_position == Some(position)
                            })
                    },
                ))
            };
        let expression_parameter = |position| {
            self.resolved_outcome(ActionDerivedFactOutcome::Arguments(
                vec![crate::ActionArgumentId {
                    action: symbol,
                    position,
                }]
                .into_boxed_slice(),
            ))
        };
        match (collection, source_kind) {
            (
                ActionDerivedFactCollection::TerminateOccurrenceArgument,
                Some(DeclarationKind::TerminateActionUsage),
            ) => return expression_parameter(1),
            (
                ActionDerivedFactCollection::SendPayloadArgument,
                Some(DeclarationKind::SendActionUsage),
            ) => return expression_parameter(1),
            (
                ActionDerivedFactCollection::SendSenderArgument,
                Some(DeclarationKind::SendActionUsage),
            ) => {
                if self
                    .storage
                    .declaration_facts(declaration)
                    .is_some_and(|facts| facts.send_has_sender_argument == Some(true))
                {
                    return expression_parameter(2);
                }
                return self.resolved_outcome(ActionDerivedFactOutcome::Arguments(Box::new([])));
            }
            (
                ActionDerivedFactCollection::SendReceiverArgument,
                Some(DeclarationKind::SendActionUsage),
            ) => {
                if self
                    .storage
                    .declaration_facts(declaration)
                    .is_some_and(|facts| facts.send_has_receiver_argument == Some(true))
                {
                    return expression_parameter(3);
                }
                return self.resolved_outcome(ActionDerivedFactOutcome::Arguments(Box::new([])));
            }
            (ActionDerivedFactCollection::IfArgument, Some(DeclarationKind::If))
            | (ActionDerivedFactCollection::WhileArgument, Some(DeclarationKind::While)) => {
                return expression_parameter(1);
            }
            (ActionDerivedFactCollection::IfThenAction, Some(DeclarationKind::If)) => {
                return self
                    .resolved_outcome(ActionDerivedFactOutcome::Values(action_parameter(2)));
            }
            (ActionDerivedFactCollection::IfElseAction, Some(DeclarationKind::If)) => {
                return self
                    .resolved_outcome(ActionDerivedFactOutcome::Values(action_parameter(3)));
            }
            (
                ActionDerivedFactCollection::LoopBodyAction,
                Some(DeclarationKind::While | DeclarationKind::Loop | DeclarationKind::ForLoop),
            ) => {
                return self
                    .resolved_outcome(ActionDerivedFactOutcome::Values(action_parameter(2)));
            }
            (
                ActionDerivedFactCollection::AcceptPayloadParameter,
                Some(DeclarationKind::AcceptActionUsage),
            ) => {
                return self.resolved_outcome(ActionDerivedFactOutcome::Parameters(
                    vec![crate::ActionInputParameterId {
                        action: symbol,
                        position: 1,
                    }]
                    .into_boxed_slice(),
                ));
            }
            (
                ActionDerivedFactCollection::AcceptPayloadArgument,
                Some(DeclarationKind::AcceptActionUsage),
            ) => {
                if self
                    .storage
                    .declaration_facts(declaration)
                    .is_some_and(|facts| facts.accept_has_payload_argument == Some(true))
                {
                    return expression_parameter(1);
                }
                return self.resolved_outcome(ActionDerivedFactOutcome::Arguments(Box::new([])));
            }
            (
                ActionDerivedFactCollection::AcceptReceiverArgument,
                Some(DeclarationKind::AcceptActionUsage),
            ) => {
                if self
                    .storage
                    .declaration_facts(declaration)
                    .is_some_and(|facts| facts.accept_has_receiver_argument == Some(true))
                {
                    return expression_parameter(2);
                }
                return self.resolved_outcome(ActionDerivedFactOutcome::Arguments(Box::new([])));
            }
            _ => {}
        }
        if collection == ActionDerivedFactCollection::AssignmentReferent {
            if source_kind != Some(DeclarationKind::Assign) {
                return self.resolved_outcome(ActionDerivedFactOutcome::OwnedMembershipMembers(
                    Box::new([]),
                ));
            }
            let Some(reference_id) = self
                .outgoing_reference_ids(declaration)
                .iter()
                .copied()
                .find(|reference_id| {
                    self.storage.references[reference_id.index()].kind
                        == ReferenceKind::AssignTarget
                })
            else {
                return self.resolved_outcome(ActionDerivedFactOutcome::OwnedMembershipMembers(
                    Box::new([]),
                ));
            };
            let target = match self.resolution.outcome(reference_id) {
                Some(ResolutionStatus::Resolved(target)) => target,
                Some(ResolutionStatus::Ambiguous(candidates)) => {
                    return self.query_outcome(QueryAnswer::Ambiguous(
                        self.resolution
                            .ambiguous_candidates(candidates)
                            .iter()
                            .filter_map(|candidate| {
                                self.symbol_id(*candidate).map(|member| {
                                    ActionDerivedFactOutcome::OwnedMembershipMembers(
                                        vec![crate::ActionOwnedMembershipMember {
                                            identity: crate::ActionOwnedMembershipId {
                                                action: symbol,
                                                position: 1,
                                            },
                                            kind: crate::ActionOwnedMembershipKind::Membership,
                                            member,
                                        }]
                                        .into_boxed_slice(),
                                    )
                                })
                            })
                            .collect::<Vec<_>>()
                            .into_boxed_slice(),
                    ));
                }
                Some(ResolutionStatus::Unsupported) => {
                    return self.query_outcome(QueryAnswer::Unsupported)
                }
                Some(ResolutionStatus::Unresolved) => {
                    return self.query_outcome(QueryAnswer::Unresolved)
                }
                Some(ResolutionStatus::NonConverged) | None => {
                    return self.query_outcome(QueryAnswer::Incomplete)
                }
            };
            let selects_referent = self
                .memberships
                .get(target)
                .is_some_and(|membership| membership.kind == MembershipKind::Feature)
                && self.storage.declaration(target).is_some_and(|target| {
                    element_kind::element_kind(target.kind) != crate::ElementKind::MetadataUsage
                });
            let values = selects_referent
                .then(|| {
                    self.symbol_id(target)
                        .map(|member| crate::ActionOwnedMembershipMember {
                            identity: crate::ActionOwnedMembershipId {
                                action: symbol,
                                position: 1,
                            },
                            kind: crate::ActionOwnedMembershipKind::Membership,
                            member,
                        })
                })
                .flatten()
                .into_iter()
                .collect::<Vec<_>>()
                .into_boxed_slice();
            return self.resolved_outcome(ActionDerivedFactOutcome::OwnedMembershipMembers(values));
        }
        if collection == ActionDerivedFactCollection::ForLoopVariable {
            if source_kind != Some(DeclarationKind::ForLoop) {
                return self.resolved_outcome(ActionDerivedFactOutcome::Values(Box::new([])));
            }
            let values = self.symbols(self.child_declarations(declaration).iter().copied().filter(
                |member| {
                    self.storage
                        .declaration(*member)
                        .is_some_and(|member| member.kind == DeclarationKind::ForLoopVariable)
                        && self
                            .storage
                            .declaration_facts(*member)
                            .is_some_and(|facts| facts.owned_feature_position == Some(1))
                },
            ));
            return self.resolved_outcome(ActionDerivedFactOutcome::Values(values));
        }
        match collection {
            ActionDerivedFactCollection::TerminateOccurrenceArgument
            | ActionDerivedFactCollection::SendSenderArgument
            | ActionDerivedFactCollection::SendReceiverArgument
            | ActionDerivedFactCollection::SendPayloadArgument => {
                self.resolved_outcome(ActionDerivedFactOutcome::Arguments(Box::new([])))
            }
            ActionDerivedFactCollection::ActionDefinitionAction => unreachable!(),
            ActionDerivedFactCollection::AssignmentValueExpression
            | ActionDerivedFactCollection::AssignmentTargetArgument
            | ActionDerivedFactCollection::ForLoopSeqArgument => {
                return self.resolved_outcome(ActionDerivedFactOutcome::Arguments(Box::new([])));
            }
            ActionDerivedFactCollection::AcceptPayloadArgument
            | ActionDerivedFactCollection::AcceptReceiverArgument
            | ActionDerivedFactCollection::WhileArgument
            | ActionDerivedFactCollection::UntilArgument
            | ActionDerivedFactCollection::IfArgument => {
                return self.resolved_outcome(ActionDerivedFactOutcome::Arguments(Box::new([])));
            }
            ActionDerivedFactCollection::AcceptPayloadParameter => {
                return self.resolved_outcome(ActionDerivedFactOutcome::Parameters(Box::new([])));
            }
            ActionDerivedFactCollection::LoopBodyAction
            | ActionDerivedFactCollection::IfThenAction
            | ActionDerivedFactCollection::IfElseAction => {
                return self.resolved_outcome(ActionDerivedFactOutcome::Values(Box::new([])));
            }
            ActionDerivedFactCollection::AssignmentReferent => unreachable!(),
            ActionDerivedFactCollection::ForLoopVariable => unreachable!(),
        }
    }

    /// Decides the exact FeatureMembership TypeFeaturing implication from the canonical
    /// membership and effective TypeFeaturing facts. It deliberately does not inspect source
    /// spelling or reconstruct `isFeaturingType` downstream.
    pub(crate) fn type_featuring_check(
        &self,
        symbol: SymbolId,
        kind: TypeFeaturingCheckKind,
    ) -> QueryOutcome<TypeFeaturingCheckOutcome> {
        let declaration = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let Some(rule) = type_featuring_check_rule(kind) else {
            return self.resolved_outcome(TypeFeaturingCheckOutcome::Unsupported {
                prerequisite: TypeFeaturingCheckPrerequisite::RuleNotPublished,
            });
        };
        if rule.metaclass != "Feature" {
            return self.resolved_outcome(TypeFeaturingCheckOutcome::Unsupported {
                prerequisite: TypeFeaturingCheckPrerequisite::RuleNotPublished,
            });
        }
        let _normative_rule = rule.rule_id;
        if self.types.featuring_requires_snapshots(declaration) {
            return self.resolved_outcome(TypeFeaturingCheckOutcome::Unsupported {
                prerequisite: TypeFeaturingCheckPrerequisite::VariableFeatureSnapshots,
            });
        }
        if !self
            .memberships
            .get(declaration)
            .is_some_and(|membership| membership.kind == MembershipKind::Feature)
        {
            return self.resolved_outcome(TypeFeaturingCheckOutcome::Unsupported {
                prerequisite: TypeFeaturingCheckPrerequisite::FeatureMembershipFacts,
            });
        }
        let outcome = if self.types.featuring_types(declaration).is_empty() {
            TypeFeaturingCheckOutcome::Violated
        } else {
            TypeFeaturingCheckOutcome::Satisfied
        };
        self.resolved_outcome(outcome)
    }

    /// Returns the first missing canonical prerequisite for one exact redefinition check.
    ///
    /// Authored and implied redefinition edges are already settled in this publication. None of
    /// these predicates is reducible to merely having an edge: each selects a particular endpoint
    /// through a metamodel role (for example an end position, state subaction kind, or constructor
    /// result). Those role facts are not yet published as canonical query inputs, so this method
    /// deliberately does not walk source syntax, inspect names, or turn an arbitrary redefinition
    /// into a satisfied result.
    pub(crate) fn redefinition_check(
        &self,
        kind: RedefinitionCheckKind,
    ) -> QueryOutcome<RedefinitionCheckOutcome> {
        let Some(rule) = redefinition_check_rule(kind) else {
            return self.resolved_outcome(RedefinitionCheckOutcome::Unsupported {
                prerequisite: RedefinitionCheckPrerequisite::RuleNotPublished,
            });
        };
        let _normative_rule = (rule.rule_id, rule.metaclass);
        let prerequisite = match kind {
            RedefinitionCheckKind::FeatureEnd => {
                RedefinitionCheckPrerequisite::EndFeaturePositionAndInheritedEnds
            }
            RedefinitionCheckKind::FeatureFlowFeature => {
                RedefinitionCheckPrerequisite::FlowEndOrdinalAndLibraryAnchors
            }
            RedefinitionCheckKind::FeatureOwnedCrossFeatureSpecialization => {
                RedefinitionCheckPrerequisite::CrossFeatureAndSubsettingEndpoints
            }
            RedefinitionCheckKind::FeatureParameter => {
                RedefinitionCheckPrerequisite::ParameterDirectionAndInheritedPosition
            }
            RedefinitionCheckKind::FeatureResult => {
                RedefinitionCheckPrerequisite::FunctionOrExpressionResult
            }
            RedefinitionCheckKind::ConstructorExpressionResultFeature => {
                RedefinitionCheckPrerequisite::ConstructorResultAndInstantiatedTypeFeatures
            }
            RedefinitionCheckKind::FeatureChainExpressionSourceTarget => {
                RedefinitionCheckPrerequisite::FeatureChainSourceTarget
            }
            RedefinitionCheckKind::FeatureChainExpressionTarget => {
                RedefinitionCheckPrerequisite::FeatureChainSourceTargetAndLibraryAnchor
            }
            RedefinitionCheckKind::ActionUsageStateAction => {
                RedefinitionCheckPrerequisite::StateSubactionMembershipAndKind
            }
            RedefinitionCheckKind::AssignmentActionUsageAccessedFeature
            | RedefinitionCheckKind::AssignmentActionUsageReferent
            | RedefinitionCheckKind::AssignmentActionUsageStartingAt => {
                RedefinitionCheckPrerequisite::AssignmentActionInputParameterEndpoints
            }
            RedefinitionCheckKind::ForLoopActionUsageVar => {
                RedefinitionCheckPrerequisite::ForLoopVariableProjection
            }
            RedefinitionCheckKind::RequirementUsageObjective => {
                RedefinitionCheckPrerequisite::ObjectiveMembershipAndCaseObjective
            }
            RedefinitionCheckKind::RenderingUsage => {
                RedefinitionCheckPrerequisite::ViewRenderingMembership
            }
        };
        self.resolved_outcome(RedefinitionCheckOutcome::Unsupported { prerequisite })
    }

    /// Returns the first unpublished canonical input for one exact specialization predicate.
    ///
    /// The resolver owns generic authored and implied specialization facts, but no predicate in
    /// this group can be discharged just because such an edge exists: each selects an endpoint,
    /// applicability role, or library anchor that remains a separate fact boundary.
    pub(crate) fn specialization_check(
        &self,
        kind: SpecializationCheckKind,
    ) -> QueryOutcome<SpecializationCheckOutcome> {
        let Some(rule) = specialization_check_rule(kind) else {
            return self.resolved_outcome(SpecializationCheckOutcome::Unsupported {
                prerequisite: SpecializationCheckPrerequisite::RuleNotPublished,
            });
        };
        let _normative_rule = (rule.rule_id, rule.metaclass);
        if kind == SpecializationCheckKind::FeatureCrossing {
            let outcome = self
                .storage
                .declaration_facts
                .iter()
                .filter_map(|facts| facts.cross_feature_projection)
                .all(|projection| projection.cross_feature == projection.owned_cross_feature)
                .then_some(SpecializationCheckOutcome::Satisfied)
                .unwrap_or(SpecializationCheckOutcome::Violated);
            return self.resolved_outcome(outcome);
        }
        let prerequisite = match kind {
            SpecializationCheckKind::FeatureCrossing => unreachable!("handled above"),
            SpecializationCheckKind::FeatureOwnedCrossFeature => {
                SpecializationCheckPrerequisite::OwnedCrossFeatureOwnerTypes
            }
            SpecializationCheckKind::FeaturePortion
            | SpecializationCheckKind::FeatureSubobject
            | SpecializationCheckKind::FeatureSuboccurrence => {
                SpecializationCheckPrerequisite::FeatureModifiersOwnerTypingAndLibraryAnchor
            }
            SpecializationCheckKind::FeatureValuation => {
                SpecializationCheckPrerequisite::FeatureValueEvaluationResults
            }
            SpecializationCheckKind::MetadataFeatureSemantic => {
                SpecializationCheckPrerequisite::SemanticMetadataProjection
            }
            SpecializationCheckKind::ConnectorBinaryObject
            | SpecializationCheckKind::ConnectorObject => {
                SpecializationCheckPrerequisite::ConnectorAssociationProjectionAndLibraryAnchor
            }
            SpecializationCheckKind::StepOwnedPerformance
            | SpecializationCheckKind::StepSubperformance => {
                SpecializationCheckPrerequisite::StepOwnershipTypingAndLibraryAnchor
            }
            SpecializationCheckKind::SelectExpressionResult
            | SpecializationCheckKind::IndexExpressionResult => {
                SpecializationCheckPrerequisite::ExpressionArgumentResult
            }
            SpecializationCheckKind::ConstructorExpressionResult => {
                SpecializationCheckPrerequisite::ExpressionResultAndInstantiatedType
            }
            SpecializationCheckKind::ConstructorExpression => {
                SpecializationCheckPrerequisite::LibraryAnchorAndImpliedSpecialization
            }
            SpecializationCheckKind::FeatureChainExpressionResult => {
                SpecializationCheckPrerequisite::FeatureChainSourceTargetAndSubsetting
            }
            SpecializationCheckKind::FeatureReferenceExpressionResult => {
                SpecializationCheckPrerequisite::FeatureReferenceReferentAndResult
            }
            SpecializationCheckKind::InvocationExpressionBehaviorResult => {
                SpecializationCheckPrerequisite::InvocationInstantiatedTypeAndResult
            }
            SpecializationCheckKind::InvocationExpression => {
                SpecializationCheckPrerequisite::InvocationInstantiatedType
            }
            SpecializationCheckKind::MergeNodeIncomingSuccession
            | SpecializationCheckKind::DecisionNodeOutgoingSuccession => {
                SpecializationCheckPrerequisite::SuccessionEndpointAndSubsetting
            }
            SpecializationCheckKind::StateUsageExclusiveState
            | SpecializationCheckKind::StateUsageSubstate => {
                SpecializationCheckPrerequisite::StateSubactionKindAndLibraryAnchor
            }
            SpecializationCheckKind::TransitionUsageAction
            | SpecializationCheckKind::TransitionUsageState => {
                SpecializationCheckPrerequisite::TransitionOwnerSourceAndLibraryAnchor
            }
            SpecializationCheckKind::TransitionUsagePayload => {
                SpecializationCheckPrerequisite::TransitionTriggerPayloadEndpoints
            }
            SpecializationCheckKind::TransitionUsageSuccessionSource => {
                SpecializationCheckPrerequisite::TransitionSuccessionSource
            }
            SpecializationCheckKind::TransitionUsageTransitionFeature => {
                SpecializationCheckPrerequisite::TransitionFeatureRolesAndLibraryAnchors
            }
            SpecializationCheckKind::IncludeUseCase => {
                SpecializationCheckPrerequisite::UseCaseOwnerAndLibraryAnchor
            }
            SpecializationCheckKind::UsageVariationDefinition
            | SpecializationCheckKind::UsageVariationUsage => {
                SpecializationCheckPrerequisite::UsageVariationOwner
            }
            SpecializationCheckKind::OccurrenceDefinitionMultiplicity => {
                SpecializationCheckPrerequisite::IndividualMultiplicityAndLibraryAnchor
            }
            SpecializationCheckKind::OccurrenceUsageSuboccurrence => {
                SpecializationCheckPrerequisite::OccurrenceOwnerTypingAndLibraryAnchor
            }
        };
        self.resolved_outcome(SpecializationCheckOutcome::Unsupported { prerequisite })
    }

    /// Projects the exact `deriveElementOwner` result from the canonical declaration ownership
    /// fact. The query neither reads source text nor follows rendered qualified names.
    pub(crate) fn derived_element_owner(
        &self,
        symbol: SymbolId,
    ) -> QueryOutcome<DerivedElementOwner> {
        let declaration = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let Some(rule) = element_derived_owner_rule() else {
            return self.query_outcome(QueryAnswer::Unsupported);
        };
        if rule.metaclass != "Element" || rule.kind != ElementDerivedOwnerKind::Owner {
            return self.query_outcome(QueryAnswer::Unsupported);
        }
        let _rule_id = rule.rule_id;
        let value = self
            .storage
            .declaration(declaration)
            .and_then(|declaration| declaration.owner)
            .and_then(|owner| self.symbol_id(owner))
            .map_or(DerivedElementOwner::NoOwner, DerivedElementOwner::Owner);
        self.resolved_outcome(value)
    }

    /// Projects one exact Root Element documentation-form derivation from canonical
    /// documentation records. It does not inspect source syntax or recreate ownership paths.
    pub(crate) fn element_derived_documentation(
        &self,
        symbol: SymbolId,
        collection: ElementDerivedDocumentationCollection,
    ) -> QueryOutcome<Box<[Documentation]>> {
        let declaration = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let Some(rule) = element_derived_documentation_rule(collection) else {
            return self.query_outcome(QueryAnswer::Unsupported);
        };
        if rule.metaclass != "Element" {
            return self.query_outcome(QueryAnswer::Unsupported);
        }
        let _rule_id = rule.rule_id;
        let form = match collection {
            ElementDerivedDocumentationCollection::Documentation => {
                InspectionAnnotationForm::Documentation
            }
            ElementDerivedDocumentationCollection::TextualRepresentation => {
                InspectionAnnotationForm::TextualRepresentation
            }
        };
        let values = self
            .documentation(declaration)
            .into_vec()
            .into_iter()
            .filter(|documentation| documentation.form == form)
            .collect::<Vec<_>>()
            .into_boxed_slice();
        self.resolved_outcome(values)
    }

    /// Projects one exact Namespace element-valued derivation from settled declaration ownership
    /// and membership facts. It neither traverses rendered names nor turns compact membership
    /// records into a second relationship store.
    pub(crate) fn namespace_derived_elements(
        &self,
        symbol: SymbolId,
        collection: NamespaceDerivedElementCollection,
    ) -> QueryOutcome<Box<[SymbolId]>> {
        let declaration = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let Some(rule) = namespace_derived_element_rule(collection) else {
            return self.query_outcome(QueryAnswer::Unsupported);
        };
        if rule.metaclass != "Namespace"
            || self
                .storage
                .declaration(declaration)
                .is_none_or(|value| !DeclarationDomain::Namespace.accepts(value.kind))
        {
            return self.query_outcome(QueryAnswer::Unsupported);
        }
        let _rule_id = rule.rule_id;
        let values = self.symbols(
            self.storage
                .declarations
                .iter()
                .enumerate()
                .filter_map(|(index, candidate_declaration)| {
                    let candidate = DeclarationId::from_index(index).ok()?;
                    (candidate != declaration && candidate_declaration.owner == Some(declaration))
                        .then_some(candidate)
                })
                .filter(|candidate| match collection {
                    NamespaceDerivedElementCollection::OwnedMember => self
                        .memberships
                        .get(*candidate)
                        .is_some_and(|membership| membership.kind == MembershipKind::Owning),
                    NamespaceDerivedElementCollection::OwnedImport => self
                        .storage
                        .declaration(*candidate)
                        .is_some_and(|candidate| candidate.kind == DeclarationKind::Import),
                }),
        );
        self.resolved_outcome(values)
    }

    /// Projects `deriveNamespaceImportImportedElement` for every direct authored NamespaceImport
    /// owned by a Namespace. The concrete grammar gives an import no authorable name, so the
    /// owner-scoped result carries its canonical identity rather than asking callers to infer one
    /// from rendered source. Each value retains the same typed target outcome as inspection.
    pub(crate) fn namespace_import_derived_elements(
        &self,
        symbol: SymbolId,
    ) -> QueryOutcome<Box<[NamespaceImportDerivedElement]>> {
        let namespace = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let Some(rule) = namespace_import_derived_element_rule() else {
            return self.query_outcome(QueryAnswer::Unsupported);
        };
        if rule.metaclass != "NamespaceImport"
            || rule.kind != NamespaceImportDerivedElementKind::ImportedElement
            || self
                .storage
                .declaration(namespace)
                .is_none_or(|value| !DeclarationDomain::Namespace.accepts(value.kind))
        {
            return self.query_outcome(QueryAnswer::Unsupported);
        }
        let _rule_id = rule.rule_id;
        let mut values = Vec::new();
        // The namespace's own members, from the settled owner->member index.
        for import in self.child_declarations(namespace).iter().copied() {
            if self
                .storage
                .declaration(import)
                .is_none_or(|declaration| declaration.kind != DeclarationKind::Import)
            {
                continue;
            }
            let relationships =
                self.relationships_of_kinds(import, &[ReferenceKind::NamespaceImport]);
            let relationship = match relationships.as_ref() {
                [] => continue,
                [relationship] => relationship,
                _ => return self.query_outcome(QueryAnswer::Unsupported),
            };
            let Some(import) = self.symbol_id(import) else {
                return self.query_outcome(QueryAnswer::Unsupported);
            };
            values.push(NamespaceImportDerivedElement {
                import,
                relationship: relationship.clone(),
            });
        }
        values.sort_by_key(|left| left.import);
        self.resolved_outcome(values.into_boxed_slice())
    }

    pub(crate) fn requirement_usage_typing(
        &self,
        symbol: SymbolId,
    ) -> QueryOutcome<RequirementUsageTyping> {
        let declaration = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        if self
            .storage
            .declaration(declaration)
            .is_none_or(|value| value.kind != DeclarationKind::RequirementUsage)
        {
            return self.query_outcome(QueryAnswer::Unsupported);
        }
        let relationships = self.relationships(declaration);
        let mut typings = relationships
            .iter()
            .filter(|relationship| relationship.kind == "featureTyping");
        let value = match (typings.next(), typings.next()) {
            (None, _) => RequirementUsageTyping::Missing,
            (Some(_), Some(_)) => {
                let candidates = self
                    .types
                    .direct_types(declaration)
                    .iter()
                    .filter_map(|(target, _)| self.symbol_id(*target))
                    .collect::<Vec<_>>();
                RequirementUsageTyping::Ambiguous(candidates.into_boxed_slice())
            }
            (Some(relationship), None) => match &relationship.target {
                RelationshipTarget::Resolved(target) => {
                    let target_is_requirement_definition =
                        self.declaration_of(*target).is_some_and(|target| {
                            self.storage.declaration(target).is_some_and(|declaration| {
                                declaration.kind == DeclarationKind::RequirementDefinition
                            })
                        });
                    if target_is_requirement_definition {
                        RequirementUsageTyping::Resolved(TypeReference {
                            symbol: *target,
                            provenance: relationship.provenance,
                        })
                    } else {
                        RequirementUsageTyping::Unsupported
                    }
                }
                RelationshipTarget::Ambiguous(values) => {
                    RequirementUsageTyping::Ambiguous(values.clone())
                }
                RelationshipTarget::Unresolved => RequirementUsageTyping::Unresolved,
                RelationshipTarget::Unsupported => RequirementUsageTyping::Unsupported,
            },
        };
        self.resolved_outcome(value)
    }

    pub(crate) fn satisfy_relationships(&self) -> QueryOutcome<Box<[SatisfyRelationship]>> {
        let endpoint = |reference: Option<AuthoredReferenceId>| match reference
            .as_ref()
            .and_then(|reference| self.resolution.outcome(*reference))
        {
            Some(ResolutionStatus::Resolved(target)) => self
                .symbol_id(target)
                .map(SatisfyEndpoint::Resolved)
                .unwrap_or(SatisfyEndpoint::Unresolved),
            Some(ResolutionStatus::Ambiguous(candidates)) => SatisfyEndpoint::Ambiguous(
                self.resolution
                    .ambiguous_candidates(candidates)
                    .iter()
                    .filter_map(|candidate| self.symbol_id(*candidate))
                    .collect(),
            ),
            Some(ResolutionStatus::Unsupported) => SatisfyEndpoint::Unsupported,
            Some(ResolutionStatus::Unresolved) | Some(ResolutionStatus::NonConverged) => {
                SatisfyEndpoint::Unresolved
            }
            None => SatisfyEndpoint::Unsupported,
        };
        let mut values = self
            .storage
            .declarations
            .iter()
            .enumerate()
            .filter_map(|(index, declaration)| {
                let document = self.storage.document(declaration.document)?;
                if document.role != SourceRole::Workspace
                    || declaration.kind != DeclarationKind::Satisfy
                {
                    return None;
                }
                let id = DeclarationId::from_index(index).ok()?;
                let requirement = self
                    .storage
                    .references
                    .iter()
                    .enumerate()
                    .find(|(_, value)| {
                        value.source == id && value.kind == ReferenceKind::SatisfySource
                    })
                    .and_then(|(index, _)| AuthoredReferenceId::from_index(index).ok());
                let satisfying = self
                    .storage
                    .references
                    .iter()
                    .enumerate()
                    .find(|(_, value)| {
                        value.source == id && value.kind == ReferenceKind::SatisfyTarget
                    })
                    .and_then(|(index, _)| AuthoredReferenceId::from_index(index).ok());
                let facts = self.storage.declaration_facts(id)?;
                Some(SatisfyRelationship {
                    identity: self.symbol_id(id)?,
                    requirement: endpoint(requirement),
                    satisfying_element: endpoint(satisfying),
                    polarity: if facts.negated.unwrap_or(false) {
                        SatisfyPolarity::NotSatisfied
                    } else {
                        SatisfyPolarity::Satisfied
                    },
                    provenance: RelationshipProvenance::Authored,
                    location: self.source_location(id)?,
                })
            })
            .collect::<Vec<_>>();
        values.sort_by(|left, right| {
            self.document_order(left.location.document, right.location.document)
                .then_with(|| left.location.range.cmp(&right.location.range))
                .then_with(|| left.identity.cmp(&right.identity))
        });
        self.resolved_outcome(values.into_boxed_slice())
    }

    /// The publication-owned paired endpoints of every workspace-authored binding connector.
    ///
    /// `BindingConnectorIndex` is assembled at the barrier from the two directional reference
    /// slots. This projection only converts its settled declaration identities into public ones;
    /// it never searches references, so it cannot pair ends from distinct authored statements.
    pub(crate) fn binding_connectors(&self) -> QueryOutcome<Box<[BindingConnector]>> {
        let endpoint = |endpoint: &binding::BindingEndpointFact| match endpoint {
            binding::BindingEndpointFact::Resolved(target) => self
                .symbol_id(*target)
                .map(BindingEndpoint::Resolved)
                .unwrap_or(BindingEndpoint::Unresolved),
            binding::BindingEndpointFact::Ambiguous(candidates) => BindingEndpoint::Ambiguous(
                candidates
                    .iter()
                    .filter_map(|candidate| self.symbol_id(*candidate))
                    .collect(),
            ),
            binding::BindingEndpointFact::Unresolved => BindingEndpoint::Unresolved,
            binding::BindingEndpointFact::Unsupported => BindingEndpoint::Unsupported,
        };
        let mut values = self
            .bindings
            .facts()
            .iter()
            .filter_map(|fact| {
                let declaration = self.storage.declaration(fact.connector)?;
                let document = self.storage.document(declaration.document)?;
                if document.role != SourceRole::Workspace {
                    return None;
                }
                Some(BindingConnector {
                    identity: self.symbol_id(fact.connector)?,
                    source: endpoint(&fact.source),
                    target: endpoint(&fact.target),
                    provenance: match fact.provenance {
                        types::FactProvenance::Authored => RelationshipProvenance::Authored,
                        types::FactProvenance::Implied => RelationshipProvenance::Implied,
                    },
                    location: self.source_location(fact.connector)?,
                })
            })
            .collect::<Vec<_>>();
        values.sort_by(|left, right| {
            self.document_order(left.location.document, right.location.document)
                .then_with(|| left.location.range.cmp(&right.location.range))
                .then_with(|| left.identity.cmp(&right.identity))
        });
        self.resolved_outcome(values.into_boxed_slice())
    }

    /// The explicit applicability state of one closed binding-connector validation rule.
    ///
    /// The rule reads the binding index, but its FeatureReferenceExpression target/result inputs
    /// are not yet canonical facts. Returning a typed unsupported result preserves that boundary
    /// instead of guessing an endpoint from a name or source expression.
    pub(crate) fn binding_connector_validation(
        &self,
        rule: BindingConnectorCheckKind,
    ) -> QueryOutcome<BindingConnectorValidationOutcome> {
        let Some(contract) = binding_connector_check_rule(rule) else {
            return self.resolved_outcome(BindingConnectorValidationOutcome::Unsupported {
                prerequisite: crate::BindingConnectorValidationPrerequisite::RuleNotPublished,
            });
        };
        // Both fields are manifest-owned contract data. Touch them here so a generated table
        // cannot quietly become a kind-only lookalike while the query retains no rule-ID map.
        let _normative_rule = (contract.rule_id, contract.metaclass);
        self.resolved_outcome(self.bindings.validation(rule))
    }

    pub(crate) fn requirement_verifications(&self) -> QueryOutcome<Box<[RequirementVerification]>> {
        let endpoint = |reference: Option<AuthoredReferenceId>| match reference
            .and_then(|reference| self.resolution.outcome(reference))
        {
            Some(ResolutionStatus::Resolved(target)) => self
                .symbol_id(target)
                .map(VerificationRequirement::Resolved)
                .unwrap_or(VerificationRequirement::Unresolved),
            Some(ResolutionStatus::Ambiguous(candidates)) => VerificationRequirement::Ambiguous(
                self.resolution
                    .ambiguous_candidates(candidates)
                    .iter()
                    .filter_map(|candidate| self.symbol_id(*candidate))
                    .collect(),
            ),
            Some(ResolutionStatus::Unsupported) => VerificationRequirement::Unsupported,
            Some(ResolutionStatus::Unresolved) | Some(ResolutionStatus::NonConverged) => {
                VerificationRequirement::Unresolved
            }
            None => VerificationRequirement::Unsupported,
        };
        let mut values = Vec::new();
        // Workspace-scoped: the settled per-document declaration slices, so an authored-workspace
        // projection never walks the bundled standard library.
        let workspace = (0..self.storage.documents.len())
            .filter_map(|index| DocumentIdx::from_index(index).ok())
            .filter(|document| {
                self.storage
                    .document(*document)
                    .is_some_and(|record| record.role == SourceRole::Workspace)
            })
            .flat_map(|document| {
                self.documents
                    .document_declarations(document)
                    .iter()
                    .copied()
            });
        for id in workspace {
            let Some(declaration) = self.storage.declaration(id) else {
                continue;
            };
            if declaration.kind != DeclarationKind::VerifyRequirement {
                continue;
            }
            let Some(objective) = declaration
                .owner
                .and_then(|id| self.storage.declaration(id))
            else {
                continue;
            };
            if objective.kind != DeclarationKind::RequirementUsage {
                continue;
            }
            let Some(case_id) = objective.owner else {
                continue;
            };
            let Some(case) = self.storage.declaration(case_id) else {
                continue;
            };
            if !matches!(
                case.kind,
                DeclarationKind::VerificationCaseDefinition
                    | DeclarationKind::VerificationCaseUsage
            ) {
                continue;
            }
            let reference = self
                .storage
                .references
                .iter()
                .enumerate()
                .find(|(_, reference)| {
                    reference.source == id
                        && reference.kind == ReferenceKind::VerifyRequirementTarget
                })
                .and_then(|(index, _)| AuthoredReferenceId::from_index(index).ok());
            let (Some(identity), Some(verification_case), Some(location)) = (
                self.symbol_id(id),
                self.symbol_id(case_id),
                self.source_location(id),
            ) else {
                continue;
            };
            values.push(RequirementVerification {
                identity,
                verification_case,
                requirement: endpoint(reference),
                provenance: RelationshipProvenance::Authored,
                location,
                outcome: VerificationOutcome::Unsupported,
            });
        }
        values.sort_by(|left, right| {
            self.document_order(left.location.document, right.location.document)
                .then_with(|| left.location.range.cmp(&right.location.range))
                .then_with(|| left.identity.cmp(&right.identity))
        });
        self.resolved_outcome(values.into_boxed_slice())
    }

    pub(crate) fn effective_types(&self, symbol: SymbolId) -> QueryOutcome<Box<[EffectiveType]>> {
        let declaration = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let mut types = self
            .types
            .effective_types(declaration)
            .iter()
            .filter_map(|(target, source)| {
                Some(EffectiveType {
                    symbol: self.symbol_id(*target)?,
                    origin: match source {
                        types::EffectiveTypeSource::Direct => EffectiveTypeOrigin::Direct,
                        types::EffectiveTypeSource::Inherited(from) => {
                            EffectiveTypeOrigin::Inherited(self.symbol_id(*from)?)
                        }
                    },
                })
            })
            .collect::<Vec<_>>();
        types.sort_by_key(|left| left.symbol);
        self.resolved_outcome(types.into_boxed_slice())
    }

    /// The settled standard-library anchor used by `checkPartDefinitionSpecialization`.
    ///
    /// A missing anchor remains `Unresolved`; multiple standard-library candidates remain
    /// `Ambiguous` with every canonical identity. Callers therefore never need to recover the
    /// anchor from a rendered name or substitute a workspace declaration.
    pub(crate) fn part_definition_specialization_anchor(&self) -> QueryOutcome<SymbolId> {
        self.library_specialization_anchor("sysml-2.0:8.3.11.2:checkPartDefinitionSpecialization")
    }

    pub(crate) fn library_specialization_anchor(&self, rule_id: &str) -> QueryOutcome<SymbolId> {
        self.library_rule_anchor(rule_id)
    }

    /// The canonical anchor outcome for one explicitly selected branch of a generated
    /// specialization rule. `Default` preserves the legacy single-anchor projection.
    pub(crate) fn library_specialization_anchor_branch(
        &self,
        rule_id: &str,
        branch: LibrarySpecializationAnchorBranch,
    ) -> QueryOutcome<SymbolId> {
        self.library_anchor_outcome(
            self.resolution
                .library_specialization_anchors
                .outcome_for(rule_id, branch),
        )
    }

    /// The canonical standard-library anchor outcome for any generated exact library rule.
    ///
    /// The stable manifest rule ID selects the fact; this intentionally does not infer a rule from
    /// a metaclass, display name, or anchor text.
    pub(crate) fn library_rule_anchor(&self, rule_id: &str) -> QueryOutcome<SymbolId> {
        self.library_anchor_outcome(self.resolution.library_specialization_anchor(rule_id))
    }

    pub(crate) fn library_anchor_outcome(
        &self,
        outcome: Option<&LibrarySpecializationAnchor>,
    ) -> QueryOutcome<SymbolId> {
        match outcome {
            Some(LibrarySpecializationAnchor::Resolved(anchor)) => self
                .symbol_id(*anchor)
                .map_or(self.query_outcome(QueryAnswer::Unresolved), |anchor| {
                    self.resolved_outcome(anchor)
                }),
            Some(LibrarySpecializationAnchor::Ambiguous(candidates)) => {
                self.query_outcome(QueryAnswer::Ambiguous(
                    candidates
                        .iter()
                        .filter_map(|candidate| self.symbol_id(*candidate))
                        .collect(),
                ))
            }
            Some(LibrarySpecializationAnchor::Missing) | None => {
                self.query_outcome(QueryAnswer::Unresolved)
            }
        }
    }

    /// Whether an exact unconditional `redefinesFromLibrary` contract can be applied to a
    /// lowered declaration in this publication.
    ///
    /// This is deliberately separate from [`Self::library_rule_anchor`]: the latter reports the
    /// normative library fact even when the parser has not yet lowered the rule's source
    /// metaclass. Returning `Unsupported` here makes that boundary observable instead of silently
    /// treating an unavailable source projection as an absent implied edge.
    pub(crate) fn library_redefinition_applicability(&self, rule_id: &str) -> QueryOutcome<()> {
        let Some(rule) = GENERATED_LIBRARY_REDEFINITION_RULES
            .iter()
            .find(|rule| rule.rule_id == rule_id)
        else {
            return self.query_outcome(QueryAnswer::Unresolved);
        };
        if lowered_redefinition_source_kind(rule.metaclass).is_some() {
            self.resolved_outcome(())
        } else {
            self.query_outcome(QueryAnswer::Unsupported)
        }
    }

    pub(crate) fn direct_supertypes(
        &self,
        symbol: SymbolId,
        scope: SpecializationScope,
    ) -> QueryOutcome<Box<[SymbolId]>> {
        let declaration = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let bit = internal_scope(scope);
        let symbols = self.symbols(
            self.types
                .supertypes(declaration)
                .iter()
                .filter(|(_, scopes)| types::scopes_of(*scopes).any(|candidate| candidate == bit))
                .map(|(target, _)| *target),
        );
        self.resolved_outcome(symbols)
    }

    /// Every supertype of `symbol`, including `symbol` itself.
    ///
    /// Reflexive, matching the Pilot's `allSupertypes() = OrderedSet{self}->closure(supertypes)`.
    /// A caller that wants the strict set removes itself; a caller that expected reflexivity and
    /// did not get it would silently answer "does not conform" for a type against itself.
    pub(crate) fn all_supertypes(
        &self,
        symbol: SymbolId,
        scope: SpecializationScope,
    ) -> QueryOutcome<Box<[SymbolId]>> {
        let declaration = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let bit = internal_scope(scope);
        let symbols = self.symbols(
            std::iter::once(declaration).chain(
                self.types
                    .specialization()
                    .scoped_ancestors(declaration)
                    .filter(move |(_, scopes)| scopes.contains(&bit))
                    .map(|(ancestor, _)| ancestor),
            ),
        );
        self.resolved_outcome(symbols)
    }

    pub(crate) fn direct_subtypes(
        &self,
        symbol: SymbolId,
        scope: SpecializationScope,
    ) -> QueryOutcome<Box<[SymbolId]>> {
        let declaration = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let bit = internal_scope(scope);
        let symbols = self.symbols(
            self.types
                .subtypes(declaration)
                .iter()
                .filter(|(_, scopes)| types::scopes_of(*scopes).any(|candidate| candidate == bit))
                .map(|(source, _)| *source),
        );
        self.resolved_outcome(symbols)
    }

    pub(crate) fn featuring_type(&self, symbol: SymbolId) -> QueryOutcome<Option<SymbolId>> {
        let declaration = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        if self.types.featuring_requires_snapshots(declaration) {
            return self.query_outcome(QueryAnswer::Unsupported);
        }
        let featuring = self
            .types
            .featuring_types(declaration)
            .iter()
            .filter_map(|(owner, _)| self.symbol_id(*owner))
            .collect::<Vec<_>>();
        match featuring.as_slice() {
            [] => self.resolved_outcome(None),
            [owner] => self.resolved_outcome(Some(*owner)),
            _ => self.query_outcome(QueryAnswer::Ambiguous(
                featuring.into_iter().map(Some).collect(),
            )),
        }
    }

    /// Every effective featuring type produced by the canonical TypeFeaturing/FeatureChaining
    /// fact family, retaining authored versus implied provenance.
    pub(crate) fn featuring_types(&self, symbol: SymbolId) -> QueryOutcome<Box<[TypeReference]>> {
        let declaration = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let values = self
            .types
            .featuring_types(declaration)
            .iter()
            .filter_map(|(target, provenance)| {
                self.symbol_id(*target).map(|symbol| TypeReference {
                    symbol,
                    provenance: match provenance {
                        types::FactProvenance::Authored => RelationshipProvenance::Authored,
                        types::FactProvenance::Implied => RelationshipProvenance::Implied,
                    },
                })
            })
            .collect::<Vec<_>>();
        let values = values.into_boxed_slice();
        if self.types.featuring_requires_snapshots(declaration) {
            if values.is_empty() {
                self.query_outcome(QueryAnswer::Unsupported)
            } else {
                self.query_outcome(QueryAnswer::Resolved(values))
            }
        } else {
            self.resolved_outcome(values)
        }
    }

    pub(crate) fn conforms_to(
        &self,
        specific: SymbolId,
        general: SymbolId,
        scope: SpecializationScope,
    ) -> QueryOutcome<Conformance> {
        let specific = match self.single_declaration(specific) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let general = match self.single_declaration(general) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        self.resolved_outcome(self.conformance(specific, general, scope))
    }

    pub(crate) fn conformance(
        &self,
        specific: DeclarationId,
        general: DeclarationId,
        scope: SpecializationScope,
    ) -> Conformance {
        if specific == general {
            return Conformance::Conforms;
        }
        // A declaration that reaches itself has a malformed hierarchy. Its closure is still
        // complete, so an answer could be produced -- but producing one would turn a modelling
        // error into a published semantic fact, which is exactly what the explicit-state rule
        // exists to prevent.
        if self.types.specialization().is_cyclic(specific) {
            return Conformance::Indeterminate(ConformanceObstacle::CyclicSpecialization);
        }
        if self
            .types
            .specialization()
            .reaches(specific, general, internal_scope(scope))
        {
            return Conformance::Conforms;
        }
        // Specialization did not reach it. Conformance is a question about what a type
        // *classifies*, and KerML's type relationships constrain that without stating a
        // generalization, so the set entailments are consulted before concluding.
        if self.set_inclusion(specific, general, scope, &mut Vec::new()) {
            return Conformance::Conforms;
        }
        Conformance::DoesNotConform
    }

    /// Whether `specific`'s instances are all `general`'s, following only entailments that are
    /// sound for KerML's four type relationships.
    ///
    /// Each is one direction of one relationship's set meaning, and nothing here is symmetric:
    ///
    /// - a union owner is included in `general` only when *every* operand is, since it classifies
    ///   whatever any operand does;
    /// - an intersection owner is included in `general` when *any* operand is, since it classifies
    ///   only what every operand does;
    /// - a difference owner is included in `general` when its *first* operand is -- the remaining
    ///   operands are exclusions and carry no positive inclusion;
    /// - `general` being a union owner admits `specific` when `specific` is included in any of its
    ///   operands, since each operand is included in the union;
    /// - `Disjoint` states that two types share no instances and entails no inclusion at all, so
    ///   it appears nowhere below.
    ///
    /// `visiting` bounds the recursion. A malformed model can make these relationships mutually
    /// recursive, and the answer for a type currently being decided is "not established by this
    /// path" rather than a second attempt.
    pub(crate) fn set_inclusion(
        &self,
        specific: DeclarationId,
        general: DeclarationId,
        scope: SpecializationScope,
        visiting: &mut Vec<DeclarationId>,
    ) -> bool {
        if specific == general {
            return true;
        }
        if visiting.contains(&specific) {
            return false;
        }
        if self
            .types
            .specialization()
            .reaches(specific, general, internal_scope(scope))
        {
            return true;
        }
        visiting.push(specific);
        let included = self.set_inclusion_uncached(specific, general, scope, visiting);
        visiting.pop();
        included
    }

    pub(crate) fn set_inclusion_uncached(
        &self,
        specific: DeclarationId,
        general: DeclarationId,
        scope: SpecializationScope,
        visiting: &mut Vec<DeclarationId>,
    ) -> bool {
        let mut union_operands = self
            .types
            .operands_of(specific, types::SetOperator::Union)
            .peekable();
        if union_operands.peek().is_some()
            && union_operands.all(|operand| self.set_inclusion(operand, general, scope, visiting))
        {
            return true;
        }
        if self
            .types
            .operands_of(specific, types::SetOperator::Intersection)
            .any(|operand| self.set_inclusion(operand, general, scope, visiting))
        {
            return true;
        }
        if self
            .types
            .operands_of(specific, types::SetOperator::Difference)
            .next()
            .is_some_and(|reduced| self.set_inclusion(reduced, general, scope, visiting))
        {
            return true;
        }
        self.types
            .operands_of(general, types::SetOperator::Union)
            .any(|operand| self.set_inclusion(specific, operand, scope, visiting))
    }

    /// KerML §7.4.12: every type of the general feature must be conformed to by some type of the
    /// specific feature.
    ///
    /// An untyped side conforms: a feature that declares no typing of its own takes the other's,
    /// so there is nothing to violate. Effective types are used rather than declared ones, so a
    /// feature that inherits its typing along a redefinition chain is not mistaken for untyped.
    pub(crate) fn feature_typing_conforms(
        &self,
        specific: SymbolId,
        general: SymbolId,
    ) -> QueryOutcome<Conformance> {
        let specific = match self.single_declaration(specific) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let general = match self.single_declaration(general) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        self.resolved_outcome(self.typing_conformance(specific, general))
    }

    pub(crate) fn typing_conformance(
        &self,
        specific: DeclarationId,
        general: DeclarationId,
    ) -> Conformance {
        // The specific side contributes only the typings it does not owe to the general side.
        // Counting an inherited typing would make the rule vacuous: every redefining feature
        // inherits the redefined feature's type, so that type would always be there to satisfy
        // the check, and a feature retyped to something unrelated would still pass.
        let specific_types = self
            .types
            .effective_types(specific)
            .iter()
            .filter(|(_, source)| match source {
                types::EffectiveTypeSource::Direct => true,
                // Discard a typing the specific side inherited along a chain that runs *through*
                // the general side: the general side has that typing too, so it cannot be the
                // thing that fails to conform to it.
                //
                // The test is that `general` reaches `from`, not the reverse. `trueEvaluations
                // subsets booleanEvaluations subsets evaluations` inherits `Evaluation` from
                // `evaluations`, and `evaluations` is an ancestor of the general side rather than
                // a descendant, so asking whether `evaluations` reaches `booleanEvaluations`
                // answered no and kept a typing both sides share. The rule then demanded that
                // `Evaluation` conform to `BooleanEvaluation` -- the general side's own narrower
                // typing -- and reported the Kernel Semantic Library's own declarations as
                // incompatible.
                types::EffectiveTypeSource::Inherited(from) => {
                    *from != general
                        && !self.types.specialization().reaches(
                            general,
                            *from,
                            types::ScopeBits::FeatureSpecialization,
                        )
                }
            })
            .map(|(target, _)| *target)
            .collect::<Vec<_>>();
        let general_types = self.types.effective_types(general);
        if specific_types.is_empty() || general_types.is_empty() {
            return Conformance::Conforms;
        }
        let mut obstacle = None;
        for (general_type, _) in general_types {
            let mut satisfied = false;
            for specific_type in &specific_types {
                match self.conformance(
                    *specific_type,
                    *general_type,
                    SpecializationScope::AnySpecialization,
                ) {
                    Conformance::Conforms => {
                        satisfied = true;
                        break;
                    }
                    Conformance::Indeterminate(reason) => obstacle = Some(reason),
                    Conformance::DoesNotConform => {}
                }
            }
            if !satisfied {
                // An unanswerable pair cannot be reported as a violation: the rule was never
                // evaluated for it.
                return match obstacle {
                    Some(reason) => Conformance::Indeterminate(reason),
                    None => Conformance::DoesNotConform,
                };
            }
        }
        Conformance::Conforms
    }

    /// KerML §8.4.3.4, with its two halves kept apart.
    pub(crate) fn subsetting_conforms(
        &self,
        subsetting: SymbolId,
        subsetted: SymbolId,
    ) -> QueryOutcome<SubsettingConformance> {
        let subsetting = match self.single_declaration(subsetting) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let subsetted = match self.single_declaration(subsetted) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        // A feature owned only by namespaces has no featuring type, so the domain half of the rule
        // has nothing to compare and cannot be violated.
        let featuring = match (
            self.types.featuring_type(subsetting),
            self.types.featuring_type(subsetted),
        ) {
            (Some(specific), Some(general)) => {
                self.conformance(specific, general, SpecializationScope::AnySpecialization)
            }
            _ => Conformance::Conforms,
        };
        self.resolved_outcome(SubsettingConformance {
            featuring,
            types: self.typing_conformance(subsetting, subsetted),
        })
    }
}

pub(crate) fn internal_scope(scope: SpecializationScope) -> types::ScopeBits {
    match scope {
        SpecializationScope::AnySpecialization => types::ScopeBits::AnySpecialization,
        SpecializationScope::Subclassification => types::ScopeBits::Subclassification,
        SpecializationScope::FeatureSpecialization => types::ScopeBits::FeatureSpecialization,
    }
}

pub(crate) fn range_contains(range: TextRange, position: TextPosition) -> bool {
    range.start <= position && position <= range.end
}
