//! Private batch resolution over the dense canonical semantic storage.
//!
//! Resolution first materializes canonical direct-name ranges, then solves import targets and
//! effective imported scopes together to a fixed point. Each pass visits the preclassified import
//! slots and indexed candidate ranges only; it never rescans declarations or all references for
//! an individual lookup. Downstream reference families read the frozen effective index after the
//! import barrier converges.

use super::element_kind;
use super::*;
use crate::diagnostics::UNCODED_PARSE_ERROR;
use crate::evaluate::*;
use crate::evaluation::EvaluationPolicy;
#[cfg(test)]
use crate::evaluation::EvaluationState;
pub(crate) use crate::index::bindings as binding;
use crate::index::documents::*;
pub(crate) use crate::index::elements as inspection;
pub(crate) use crate::index::expressions as expression;
use crate::index::identity::*;
use crate::index::reverse_references::*;
pub(crate) use crate::index::types;
use crate::resolve::implied::*;
use crate::resolve::library_seed::*;
use crate::resolve::names::*;
use crate::resolve::results::*;
use crate::resolve::*;
use crate::{
    ActionDerivedFactCollection, ActionDerivedFactOutcome, ActionDerivedFactPrerequisite,
    AnnotationForm as InspectionAnnotationForm, BindingConnector, BindingConnectorCheckKind,
    BindingConnectorValidationOutcome, BindingEndpoint, Conformance, ConformanceObstacle,
    DefinitionUsageDerivedKind, DefinitionUsageDerivedOutcome, DefinitionUsageDerivedPrerequisite,
    DerivedElementOwner, Diagnostic, DiagnosticCategory, DiagnosticCode, DiagnosticLocation,
    DiagnosticOrigin, DiagnosticSeverity, Documentation, EffectiveType, EffectiveTypeOrigin,
    ElementDerivedDocumentationCollection, ElementRelationship,
    FeatureDerivedRelationshipCollection, LibrarySpecializationAnchorBranch,
    NamespaceDerivedElementCollection, NamespaceImportDerivedElement, NavigationTarget,
    OccurrenceRole, PublicationCompleteness as PublicCompleteness, PublishedDiagnostics,
    QueryOutcome, RedefinitionCheckKind, RedefinitionCheckOutcome, RedefinitionCheckPrerequisite,
    RelatedLocation, RelationshipProvenance, RelationshipTarget, RenameOutcome,
    RequirementDerivedFactCollection, RequirementDerivedFactOutcome,
    RequirementDerivedFactPrerequisite, RequirementUsageTyping, RequirementVerification,
    SatisfyEndpoint, SatisfyPolarity, SatisfyRelationship, SourceLocation, SpecializationCheckKind,
    SpecializationCheckOutcome, SpecializationCheckPrerequisite, SpecializationScope,
    SubsettingConformance, SymbolIdentity, TextPosition, TextRange, TypeDerivedElementCollection,
    TypeDerivedFactCollection, TypeDerivedFactOutcome, TypeDerivedFactPrerequisite,
    TypeDerivedFactValue, TypeDerivedRelationshipCollection, TypeFeaturingCheckKind,
    TypeFeaturingCheckOutcome, TypeFeaturingCheckPrerequisite, TypeReference, VerificationOutcome,
    VerificationRequirement, VisibleMember,
};
#[cfg(test)]
use spec42_constraint_manifest::LibrarySpecializationPredicate;
use spec42_constraint_manifest::{ElementDerivedOwnerKind, NamespaceImportDerivedElementKind};

pub(crate) mod conformance;
pub(crate) mod details;
pub(crate) mod expression_conformance;
pub(crate) mod host_conformance;
pub(crate) mod structural;
pub(crate) mod writer;

/// The note attached to each declaration an ambiguous reference could have named.
pub(crate) const RELATED_AMBIGUOUS_CANDIDATE: &str = "Candidate this reference could name.";

/// The settled diagnostics of one publication with the per-document ranges that index them.
///
/// The two are derived together and are only correct together, so they are returned together
/// rather than as two calls a caller could interleave.
pub(crate) type DerivedDiagnostics = (Box<[Diagnostic]>, Box<[(u32, u32)]>);

#[derive(Debug)]
pub(crate) struct ResolvedSemanticModel {
    pub(crate) storage: SemanticModelStorage,
    pub(crate) direct_names: NameIndex,
    pub(crate) effective_imports: NameIndex,
    pub(crate) identities: IdentityIndex,
    pub(crate) documents: DocumentIndex,
    pub(crate) memberships: MembershipIndex,
    pub(crate) reverse_references: ReverseReferenceIndex,
    pub(crate) effective_scopes: EffectiveScopeIndex,
    pub(crate) facts: inspection::ElementFactIndex,
    /// Canonical paired binding-connector endpoints, assembled once after resolution.
    pub(crate) bindings: binding::BindingConnectorIndex,
    pub(crate) types: types::TypeIndex,
    pub(crate) resolution: ResolutionResults,
    pub(crate) evaluation: Box<[EvaluationFact]>,
    /// Settled unit, measurement and filter facts over the expressions this publication admitted.
    pub(crate) expressions: expression::ExpressionIndex,
    /// Settled at the publication barrier alongside the indexes, so reading them is a lookup and
    /// a broken storage invariant fails the build instead of a later query.
    pub(crate) diagnostics: Box<[Diagnostic]>,
    /// Where each document's diagnostics begin and end inside `diagnostics`, by `DocumentId`.
    ///
    /// The derivation groups one document's diagnostics contiguously, so a document-scoped query
    /// is a slice of the settled sequence rather than a scan of it. Built at the same barrier so
    /// the two can never disagree.
    pub(crate) diagnostics_by_document: Box<[(u32, u32)]>,
    pub(crate) metadata: PublicationMetadata,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PublicationPhase {
    Resolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PublicationCompleteness {
    Complete,
    ParseRecovery,
    UnsupportedSyntax,
    NonConverged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PublicationMetadata {
    pub(crate) phase: PublicationPhase,
    pub(crate) completeness: PublicationCompleteness,
    pub(crate) has_evaluation: bool,
}

impl ResolvedSemanticModel {
    pub(crate) fn completeness(&self) -> PublicCompleteness {
        match self.metadata.completeness {
            PublicationCompleteness::Complete => PublicCompleteness::Complete,
            PublicationCompleteness::ParseRecovery => PublicCompleteness::ParseRecovery,
            PublicationCompleteness::UnsupportedSyntax => PublicCompleteness::UnsupportedSyntax,
            PublicationCompleteness::NonConverged => PublicCompleteness::NonConverged,
        }
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
            .map(|index| DocumentId(index as u32))
        else {
            return QueryOutcome::Unresolved;
        };

        // `provider -> consumer`. Both resolved and ambiguous outcomes are owned facts: every
        // ambiguous candidate can affect the consumer if it changes. Unresolved/unsupported
        // dependency-shaping references remain explicit through the publication completeness or
        // the conservative result below; they are never guessed from authored text.
        let mut reverse = vec![Vec::<DocumentId>::new(); self.storage.documents.len()];
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
            match self.metadata.completeness {
                PublicationCompleteness::NonConverged => QueryOutcome::Incomplete,
                PublicationCompleteness::UnsupportedSyntax => {
                    QueryOutcome::UnsupportedWith(affected)
                }
                _ => QueryOutcome::Recovered(affected),
            }
        } else {
            self.resolved_outcome(affected)
        }
    }

    pub(crate) fn resolved_outcome<T>(&self, value: T) -> QueryOutcome<T> {
        match self.metadata.completeness {
            PublicationCompleteness::Complete => QueryOutcome::Resolved(value),
            PublicationCompleteness::ParseRecovery => QueryOutcome::Recovered(value),
            PublicationCompleteness::UnsupportedSyntax => QueryOutcome::UnsupportedWith(value),
            PublicationCompleteness::NonConverged => QueryOutcome::Incomplete,
        }
    }

    /// The canonical structural identity of one declaration.
    ///
    /// Stable across builds of the same sources, unlike the dense storage ordinal, so a consumer
    /// may hold one across a rebuild; see `IdentityIndex`.
    pub(crate) fn symbol_identity(&self, id: DeclarationId) -> Option<SymbolIdentity> {
        self.identities
            .identity(id)
            .map(|text| SymbolIdentity(text.into()))
    }

    /// Every declaration carrying `identity`.
    ///
    /// More than one only when the source authors identically named siblings; callers publish that
    /// as an explicit ambiguous outcome rather than choosing between them.
    pub(crate) fn identity_declarations(&self, identity: &SymbolIdentity) -> Vec<DeclarationId> {
        self.identities.declarations(&identity.0)
    }

    pub(crate) fn declaration_target(&self, id: DeclarationId) -> Option<NavigationTarget> {
        let declaration = self.storage.declaration(id)?;
        let name = self.storage.symbol(declaration.name?)?;
        Some(NavigationTarget {
            symbol: self.symbol_identity(id)?,
            name: name.into(),
            location: SourceLocation {
                document: self
                    .storage
                    .document(declaration.document)?
                    .identity
                    .clone(),
                range: declaration_identifier_range(
                    &self.storage,
                    declaration.document,
                    &declaration.span,
                    name,
                )
                .ok()?,
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
            return QueryOutcome::Unresolved;
        };
        let Some(positions) = self.documents.positions(document_id) else {
            return QueryOutcome::Unresolved;
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
                    targets.sort_by(target_order);
                    targets.dedup_by(|a, b| a.symbol == b.symbol);
                    return QueryOutcome::Ambiguous(targets.into_boxed_slice());
                }
                Some(ResolutionStatus::Unsupported) => return QueryOutcome::Unsupported,
                Some(ResolutionStatus::NonConverged) => return QueryOutcome::Incomplete,
                Some(ResolutionStatus::Unresolved) | None => return QueryOutcome::Unresolved,
            }
        }
        reference_matches.sort_by(target_order);
        reference_matches.dedup_by(|a, b| a.symbol == b.symbol);
        if reference_matches.len() == 1 {
            return self.resolved_outcome(reference_matches.remove(0));
        }
        if reference_matches.len() > 1 {
            return QueryOutcome::Ambiguous(reference_matches.into_boxed_slice());
        }
        let mut declarations = leaf_ranges_containing(&positions.identifiers, position)
            .filter_map(|id| self.declaration_target(id))
            .collect::<Vec<_>>();
        declarations.sort_by(target_order);
        match declarations.len() {
            0 => QueryOutcome::Unresolved,
            1 => self.resolved_outcome(declarations.remove(0)),
            _ => QueryOutcome::Ambiguous(declarations.into_boxed_slice()),
        }
    }

    pub(crate) fn references(
        &self,
        symbol: &SymbolIdentity,
        include_declaration: bool,
    ) -> QueryOutcome<Box<[SourceLocation]>> {
        if matches!(
            self.metadata.completeness,
            PublicationCompleteness::NonConverged
        ) {
            return QueryOutcome::Incomplete;
        }
        let mut targets = self.identity_declarations(symbol);
        if targets.len() > 1 {
            // The caller's identity names identically authored siblings. Answering for one of
            // them would silently pick; answering for all of them as one list would merge distinct
            // elements' references, so each candidate's own list is published separately.
            let per_candidate = targets
                .into_iter()
                .map(
                    |target| match self.references_for(target, include_declaration) {
                        QueryOutcome::Resolved(locations)
                        | QueryOutcome::Recovered(locations)
                        | QueryOutcome::UnsupportedWith(locations) => locations,
                        _ => Box::default(),
                    },
                )
                .collect::<Vec<_>>();
            return QueryOutcome::Ambiguous(per_candidate.into_boxed_slice());
        }
        let Some(target) = targets.pop() else {
            return QueryOutcome::Unresolved;
        };
        self.references_for(target, include_declaration)
    }

    pub(crate) fn references_for(
        &self,
        target: DeclarationId,
        include_declaration: bool,
    ) -> QueryOutcome<Box<[SourceLocation]>> {
        let Some(target_declaration) = self.storage.declaration(target) else {
            return QueryOutcome::Unresolved;
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
                return QueryOutcome::Incomplete;
            };
            let Some(source) = self.storage.declaration(reference.source) else {
                return QueryOutcome::Incomplete;
            };
            let Some(name) = self
                .storage
                .symbol(target_declaration.name.unwrap_or(SymbolId(u32::MAX)))
            else {
                return QueryOutcome::Incomplete;
            };
            let range = identifier_range(&self.storage, source.document, &reference.span, name)
                .or_else(|_| document_range(&self.storage, source.document, &reference.span));
            let Ok(range) = range else {
                return QueryOutcome::Incomplete;
            };
            locations.push(SourceLocation {
                document: self
                    .storage
                    .document(source.document)
                    .map(|d| d.identity.clone())
                    .unwrap_or_default(),
                range,
                role: OccurrenceRole::Reference,
            });
        }
        locations.sort_by(location_order);
        locations.dedup();
        self.resolved_outcome(locations.into_boxed_slice())
    }

    pub(crate) fn prepare_rename(
        &self,
        document: &str,
        position: TextPosition,
        new_name: Option<&str>,
    ) -> RenameOutcome {
        let target = match self.target_at(document, position) {
            QueryOutcome::Resolved(target)
            | QueryOutcome::Recovered(target)
            | QueryOutcome::UnsupportedWith(target) => target,
            QueryOutcome::Ambiguous(targets) => return RenameOutcome::Ambiguous(targets),
            QueryOutcome::Unsupported => return RenameOutcome::Unsupported,
            QueryOutcome::Recovery => return RenameOutcome::Recovery,
            QueryOutcome::Incomplete => return RenameOutcome::Incomplete,
            QueryOutcome::Unresolved => return RenameOutcome::Unresolved,
        };
        if let Some(name) = new_name {
            if !valid_identifier(name) {
                return RenameOutcome::InvalidName;
            }
            let mut candidates = self.identity_declarations(&target.symbol);
            if candidates.len() > 1 {
                let mut ambiguous = candidates
                    .into_iter()
                    .filter_map(|candidate| self.declaration_target(candidate))
                    .collect::<Vec<_>>();
                ambiguous.sort_by(target_order);
                return RenameOutcome::Ambiguous(ambiguous.into_boxed_slice());
            }
            let Some(id) = candidates.pop() else {
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
                collisions.sort_by(target_order);
                if !collisions.is_empty() {
                    return RenameOutcome::Collision(collisions.into_boxed_slice());
                }
            }
        }
        let occurrences = match self.references(&target.symbol, true) {
            QueryOutcome::Resolved(value) => value,
            _ => return RenameOutcome::Incomplete,
        };
        let range = occurrences
            .iter()
            .find(|location| {
                location.document.as_ref() == document && range_contains(location.range, position)
            })
            .map(|location| location.range)
            .unwrap_or(target.location.range);
        RenameOutcome::Ready {
            symbol: target.symbol,
            name: target.name,
            range,
            occurrences,
        }
    }

    pub(crate) fn visible_members(
        &self,
        document: &str,
        position: TextPosition,
        qualifier: Option<&str>,
    ) -> QueryOutcome<Box<[VisibleMember]>> {
        let recovered = matches!(
            self.metadata.completeness,
            PublicationCompleteness::ParseRecovery
        );
        let unsupported = matches!(
            self.metadata.completeness,
            PublicationCompleteness::UnsupportedSyntax
        );
        if matches!(
            self.metadata.completeness,
            PublicationCompleteness::NonConverged
        ) {
            return QueryOutcome::Incomplete;
        }
        let Some(document_id) = self.documents.document(&self.storage, document) else {
            return QueryOutcome::Unresolved;
        };
        let Some(positions) = self.documents.positions(document_id) else {
            return QueryOutcome::Unresolved;
        };
        let owner = positions.spans.innermost_containing(position);
        let mut ids = Vec::new();
        if let Some(qualifier) = qualifier {
            let scopes = match self.resolve_qualifier_scopes(owner, qualifier) {
                Ok(scopes) if !scopes.is_empty() => scopes,
                Ok(_) => return QueryOutcome::Unresolved,
                Err(_) => return QueryOutcome::Incomplete,
            };
            if scopes.len() > 1 {
                let candidates = scopes
                    .into_iter()
                    .map(|scope| {
                        self.visible_member_records(self.effective_scopes.members(Some(scope)))
                    })
                    .collect::<Vec<_>>()
                    .into_boxed_slice();
                return QueryOutcome::Ambiguous(candidates);
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
            QueryOutcome::Recovered(members)
        } else if unsupported {
            QueryOutcome::UnsupportedWith(members)
        } else {
            QueryOutcome::Resolved(members)
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

    pub(crate) fn visible_member_records(&self, ids: &[DeclarationId]) -> Box<[VisibleMember]> {
        let mut ids = ids.to_vec();
        ids.sort_unstable();
        ids.dedup();
        let mut members = ids
            .into_iter()
            .filter_map(|id| {
                let target = self.declaration_target(id)?;
                let declaration = self.storage.declaration(id)?;
                let qualified_name = declaration_qualified_name(&self.storage, id)?;
                let container_name = declaration
                    .owner
                    .and_then(|owner| self.storage.declaration(owner)?.name)
                    .and_then(|name| self.storage.symbol(name))
                    .map(Into::into);
                Some(VisibleMember {
                    symbol: target.symbol,
                    name: target.name,
                    kind: element_kind::element_kind(declaration.kind),
                    role: element_kind::membership_role(declaration.kind),
                    qualified_name: qualified_name.into_boxed_str(),
                    container_name,
                    declaring_document: target.location.document,
                    declaration_range: target.location.range,
                })
            })
            .collect::<Vec<_>>();
        members.sort_by(|a, b| {
            a.name
                .cmp(&b.name)
                .then_with(|| a.declaring_document.cmp(&b.declaring_document))
                .then_with(|| a.declaration_range.cmp(&b.declaration_range))
        });
        members.into_boxed_slice()
    }

    /// Every diagnostic this publication settled, as facts rather than rendered text.
    ///
    /// Derived once while the model is sealed, for the same reason the other indexes are: a query
    /// against a settled publication reads a fact instead of recomputing one, and a storage
    /// inconsistency fails the build rather than surfacing later as a silently missing diagnostic.
    ///
    /// Only workspace-authored documents contribute. Library sources take part in the same
    /// semantic system, but their own diagnostics are not the authoring surface, and this also
    /// keeps the barrier's cost proportional to the workspace rather than to the library.
    pub(crate) fn derive_diagnostics(
        &self,
        reported: &[Box<str>],
    ) -> Result<DerivedDiagnostics, ResolutionError> {
        let mut diagnostics = Vec::new();
        let mut by_document = vec![(0u32, 0u32); self.storage.documents.len()];
        // Built once, then sliced per document. Every rule below asks "what did this document
        // declare", and answering that by scanning every declaration each time made the barrier
        // quadratic in the admitted corpus -- invisible while only workspace documents were
        // derived, and the reason a library's own documents could not be.
        let declarations_by_document = self.declarations_by_document()?;
        for document_index in self.reported_document_indices(reported) {
            let document = &self.storage.documents[document_index];
            let document_id = DocumentId(document_index as u32);
            let first = diagnostics.len();

            for error in document.parse_errors.iter() {
                let range = parse_error_range(&document.parsed, error)
                    .ok_or(ResolutionError::InvalidStorage)?;
                diagnostics.push(Diagnostic {
                    // The parser owns both the code and the sentence; neither is re-derived here.
                    message: error.message.as_str().into(),
                    subject: None,
                    code: DiagnosticCode::Parser {
                        code: match error.code.as_deref() {
                            Some(code) => code.into(),
                            None => UNCODED_PARSE_ERROR.into(),
                        },
                        category: parser_diagnostic_category(error.category),
                    },
                    severity: match error.severity {
                        Some(sysml_v2_parser::DiagnosticSeverity::Warning) => {
                            DiagnosticSeverity::Warning
                        }
                        Some(sysml_v2_parser::DiagnosticSeverity::Error) | None => {
                            DiagnosticSeverity::Error
                        }
                    },
                    origin: DiagnosticOrigin::Parser,
                    location: DiagnosticLocation {
                        document: document.identity.clone(),
                        range,
                    },
                    related: Box::default(),
                });
            }

            for record in self
                .storage
                .unsupported
                .iter()
                .filter(|record| record.document == document_id)
            {
                let code = unsupported_construct_code(record.family);
                diagnostics.push(Diagnostic {
                    message: code.describe().into(),
                    subject: None,
                    code,
                    severity: DiagnosticSeverity::Warning,
                    origin: DiagnosticOrigin::Semantic,
                    location: DiagnosticLocation {
                        document: document.identity.clone(),
                        range: document_range(&self.storage, document_id, &record.span)?,
                    },
                    related: Box::default(),
                });
            }

            for (index, reference) in self.storage.references.iter().enumerate() {
                let source = self
                    .storage
                    .declaration(reference.source)
                    .ok_or(ResolutionError::InvalidStorage)?;
                if source.document != document_id {
                    continue;
                }
                let reference_id = AuthoredReferenceId::from_index(index)
                    .map_err(|_| ResolutionError::Capacity)?;
                let status = self
                    .resolution
                    .outcome(reference_id)
                    .ok_or(ResolutionError::InvalidStorage)?;
                let Some((severity, code)) = reference_diagnostic(reference.kind, status) else {
                    continue;
                };
                let mut related = Vec::new();
                if let ResolutionStatus::Ambiguous(candidates) = status {
                    // Every candidate, in the resolver's canonical candidate order: choosing one
                    // would settle an ambiguity the publication deliberately left open.
                    for target in self.resolution.ambiguous_candidates(candidates) {
                        related
                            .push(self.related_declaration(*target, RELATED_AMBIGUOUS_CANDIDATE)?);
                    }
                }
                diagnostics.push(Diagnostic {
                    message: code.describe().into(),
                    subject: self.symbol_identity(reference.source),
                    code,
                    severity,
                    origin: DiagnosticOrigin::Semantic,
                    location: DiagnosticLocation {
                        document: document.identity.clone(),
                        range: document_range(&self.storage, document_id, &reference.span)?,
                    },
                    related: related.into_boxed_slice(),
                });
            }

            let declared = declarations_by_document
                .get(document_index)
                .map(Vec::as_slice)
                .unwrap_or_default();
            self.collect_conformance(document_id, declared, &mut diagnostics)?;
            self.collect_structural_conformance(document_id, declared, &mut diagnostics)?;
            self.collect_expression_conformance(document_id, declared, &mut diagnostics)?;
            self.collect_host_conformance(document_id, declared, &mut diagnostics)?;
            self.collect_library_context(document_id, first, &mut diagnostics)?;

            // Ordering is owned here so no consumer has to sort, and so the order cannot vary with
            // which storage collection a diagnostic happened to come from. The sort is stable, so
            // parser, unsupported-construct, and reference diagnostics that share a range keep the
            // order they were derived in.
            diagnostics[first..].sort_by(|left, right| {
                left.location
                    .range
                    .cmp(&right.location.range)
                    .then_with(|| left.code.as_str().cmp(right.code.as_str()))
            });
            by_document[document_index] = (
                u32::try_from(first).map_err(|_| ResolutionError::Capacity)?,
                u32::try_from(diagnostics.len()).map_err(|_| ResolutionError::Capacity)?,
            );
        }
        Ok((
            diagnostics.into_boxed_slice(),
            by_document.into_boxed_slice(),
        ))
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

    /// The diagnostics one document owns, as the settled slice rather than a filtered scan.
    ///
    /// A document this publication did not admit has no diagnostics, which is a different answer
    /// from "no diagnostic was reported": the caller asked about a document that is not part of
    /// this model, and the empty slice says so alongside the publication's completeness.
    pub(crate) fn published_document_diagnostics(&self, document: &str) -> PublishedDiagnostics {
        let diagnostics = match self.documents.document(&self.storage, document) {
            Some(id) => match self.diagnostics_by_document.get(id.index()) {
                Some((start, end)) => self.diagnostics[*start as usize..*end as usize].into(),
                None => Box::default(),
            },
            None => Box::default(),
        };
        PublishedDiagnostics {
            completeness: self.completeness(),
            diagnostics,
        }
    }

    pub(crate) fn published_diagnostics(&self) -> PublishedDiagnostics {
        PublishedDiagnostics {
            completeness: self.completeness(),
            diagnostics: self.diagnostics.clone(),
        }
    }

    pub(crate) fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
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

    /// Resolves one published identity to the single declaration it names.
    ///
    /// Every type query needs this, and every one of them owes the caller the same three explicit
    /// answers: the publication did not converge, the identity names nothing, or it names several
    /// identically authored siblings and choosing between them would be a guess.
    pub(crate) fn single_declaration<T>(
        &self,
        symbol: &SymbolIdentity,
    ) -> Result<DeclarationId, QueryOutcome<T>> {
        if matches!(
            self.metadata.completeness,
            PublicationCompleteness::NonConverged
        ) {
            return Err(QueryOutcome::Incomplete);
        }
        let mut candidates = self.identity_declarations(symbol);
        if candidates.len() > 1 {
            return Err(QueryOutcome::Unresolved);
        }
        candidates.pop().ok_or(QueryOutcome::Unresolved)
    }

    pub(crate) fn symbols(
        &self,
        declarations: impl Iterator<Item = DeclarationId>,
    ) -> Box<[SymbolIdentity]> {
        let mut symbols = declarations
            .filter_map(|id| self.symbol_identity(id))
            .collect::<Vec<_>>();
        symbols.sort();
        symbols.dedup();
        symbols.into_boxed_slice()
    }

    pub(crate) fn direct_types(
        &self,
        symbol: &SymbolIdentity,
    ) -> QueryOutcome<Box<[TypeReference]>> {
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
                    symbol: self.symbol_identity(*target)?,
                    provenance: match provenance {
                        types::FactProvenance::Authored => RelationshipProvenance::Authored,
                        types::FactProvenance::Implied => RelationshipProvenance::Implied,
                    },
                })
            })
            .collect::<Vec<_>>();
        types.sort_by(|left, right| left.symbol.cmp(&right.symbol));
        self.resolved_outcome(types.into_boxed_slice())
    }

    /// Projects one exact KerML Feature relationship collection from the canonical relationship
    /// index. No relationship is re-derived here: authored and implied edges, their provenance,
    /// and target-resolution state are all the same facts an element inspection publishes.
    pub(crate) fn feature_derived_relationships(
        &self,
        symbol: &SymbolIdentity,
        collection: FeatureDerivedRelationshipCollection,
    ) -> QueryOutcome<Box<[ElementRelationship]>> {
        let declaration = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let Some(rule) = feature_derived_relationship_rule(collection) else {
            // A public enum value with no generated pinned-manifest contract is not a silently
            // empty collection. It is an incomplete implementation boundary.
            return QueryOutcome::Unsupported;
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
            return QueryOutcome::Unsupported;
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
        symbol: &SymbolIdentity,
        collection: TypeDerivedRelationshipCollection,
    ) -> QueryOutcome<Box<[ElementRelationship]>> {
        let declaration = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let Some(rule) = type_derived_relationship_rule(collection) else {
            return QueryOutcome::Unsupported;
        };
        if rule.metaclass != "Type"
            || self
                .storage
                .declaration(declaration)
                .is_none_or(|declaration| !DeclarationDomain::Type.accepts(declaration.kind))
        {
            return QueryOutcome::Unsupported;
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
        symbol: &SymbolIdentity,
        collection: TypeDerivedElementCollection,
    ) -> QueryOutcome<Box<[SymbolIdentity]>> {
        let declaration = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let Some(rule) = type_derived_element_rule(collection) else {
            return QueryOutcome::Unsupported;
        };
        if rule.metaclass != "Type"
            || self
                .storage
                .declaration(declaration)
                .is_none_or(|value| !DeclarationDomain::Type.accepts(value.kind))
        {
            return QueryOutcome::Unsupported;
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
        symbol: &SymbolIdentity,
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
            return QueryOutcome::Unsupported;
        }
        let _rule_id = rule.rule_id;
        let unavailable = match collection {
            TypeDerivedFactCollection::OwnedFeatureMembership => {
                Some(TypeDerivedFactPrerequisite::FeatureMembershipIdentity)
            }
            TypeDerivedFactCollection::Multiplicity => {
                Some(TypeDerivedFactPrerequisite::MultiplicityIdentity)
            }
            TypeDerivedFactCollection::OwnedConjugator => {
                Some(TypeDerivedFactPrerequisite::ConjugationRelationshipIdentity)
            }
            _ => None,
        };
        if let Some(prerequisite) = unavailable {
            return self.resolved_outcome(TypeDerivedFactOutcome::Unsupported { prerequisite });
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
    /// This intentionally stops at the first unavailable owner for the broader `feature`,
    /// `directedFeature`, VariantMembership, and time-variation predicates.  A direct child scan
    /// is never substituted for an inherited collection, and a VariantMembership relationship is
    /// never fabricated from an element role.
    pub(crate) fn definition_usage_derived(
        &self,
        symbol: &SymbolIdentity,
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
            return QueryOutcome::Incomplete;
        };
        if !definition_usage_source_matches(rule.metaclass, source.kind) {
            return QueryOutcome::Unsupported;
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
                    self.owned_feature_members(declaration)
                        .into_iter()
                        .chain(self.inherited_feature_members(declaration))
                        .filter(|member| {
                            self.storage
                                .declaration(*member)
                                .is_some_and(|member| is_usage_declaration(member.kind))
                                && (!directed
                                    || self
                                        .storage
                                        .declaration_facts(*member)
                                        .is_some_and(|facts| facts.direction.is_some()))
                        }),
                );
                self.resolved_outcome(DefinitionUsageDerivedOutcome::Elements(values))
            }
            DefinitionUsageDerivedKind::DefinitionVariant
            | DefinitionUsageDerivedKind::DefinitionVariantMembership
            | DefinitionUsageDerivedKind::UsageVariant
            | DefinitionUsageDerivedKind::UsageVariantMembership => {
                self.resolved_outcome(DefinitionUsageDerivedOutcome::Unsupported {
                    prerequisite: DefinitionUsageDerivedPrerequisite::VariantMembershipIdentity,
                })
            }
            DefinitionUsageDerivedKind::UsageMayTimeVary => {
                self.resolved_outcome(DefinitionUsageDerivedOutcome::Unsupported {
                    prerequisite:
                        DefinitionUsageDerivedPrerequisite::EffectiveOccurrenceTimeVariationFacts,
                })
            }
            DefinitionUsageDerivedKind::UsageIsReference => {
                let is_composite = self
                    .storage
                    .declaration_facts(declaration)
                    .is_some_and(|facts| facts.modifiers.composite);
                self.resolved_outcome(DefinitionUsageDerivedOutcome::Boolean(!is_composite))
            }
            _ => {
                let values = self.symbols(self.storage.declarations.iter().enumerate().filter_map(
                    |(index, candidate)| {
                        let candidate_id = DeclarationId::from_index(index).ok()?;
                        (candidate_id != declaration
                            && candidate.owner == Some(declaration)
                            && self
                                .memberships
                                .get(candidate_id)
                                .is_some_and(|membership| {
                                    membership.kind == MembershipKind::Feature
                                })
                            && definition_usage_candidate_matches(kind, candidate.kind))
                        .then_some(candidate_id)
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
        symbol: &SymbolIdentity,
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
            return QueryOutcome::Incomplete;
        };
        if !requirement_derived_source_matches(rule.metaclass, source.kind) {
            return QueryOutcome::Unsupported;
        }
        let _rule_id = rule.rule_id;
        if collection.requires_text() {
            let values = self
                .documentation(declaration)
                .into_vec()
                .into_iter()
                .filter(|value| value.form == InspectionAnnotationForm::Documentation)
                .map(|value| value.text)
                .collect::<Vec<_>>()
                .into_boxed_slice();
            return self.resolved_outcome(RequirementDerivedFactOutcome::Text(values));
        }
        let Some(role) = requirement_derived_membership_role(collection) else {
            return self.resolved_outcome(RequirementDerivedFactOutcome::Unsupported {
                prerequisite: RequirementDerivedFactPrerequisite::CanonicalMembershipRole,
            });
        };
        let values = self.symbols(self.storage.declarations.iter().enumerate().filter_map(
            |(index, candidate)| {
                let candidate_id = DeclarationId::from_index(index).ok()?;
                (candidate.owner == Some(declaration)
                    && self
                        .memberships
                        .get(candidate_id)
                        .is_some_and(|membership| membership.kind == MembershipKind::Feature)
                    && element_kind::membership_role(candidate.kind) == Some(role))
                .then_some(candidate_id)
            },
        ));
        self.resolved_outcome(RequirementDerivedFactOutcome::Elements(values))
    }

    /// The exact Actions derivation boundary.  The current model preserves selected action forms
    /// and references but not the normative ordered argument/input-parameter or inherited-usage
    /// identities, so this returns the first unavailable canonical fact rather than guessing.
    pub(crate) fn action_derived_fact(
        &self,
        symbol: &SymbolIdentity,
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
        let prerequisite = match collection {
            ActionDerivedFactCollection::ActionDefinitionAction => {
                ActionDerivedFactPrerequisite::EffectiveUsageClosure
            }
            ActionDerivedFactCollection::AssignmentReferent => {
                ActionDerivedFactPrerequisite::OwnedMembershipIdentity
            }
            ActionDerivedFactCollection::ForLoopVariable => {
                ActionDerivedFactPrerequisite::OrderedOwnedFeatureIdentity
            }
            ActionDerivedFactCollection::LoopBodyAction
            | ActionDerivedFactCollection::AcceptPayloadParameter
            | ActionDerivedFactCollection::WhileArgument
            | ActionDerivedFactCollection::UntilArgument
            | ActionDerivedFactCollection::IfThenAction
            | ActionDerivedFactCollection::IfElseAction
            | ActionDerivedFactCollection::IfArgument => {
                ActionDerivedFactPrerequisite::OrderedInputParameterIdentity
            }
            ActionDerivedFactCollection::TerminateOccurrenceArgument
            | ActionDerivedFactCollection::SendSenderArgument
            | ActionDerivedFactCollection::SendReceiverArgument
            | ActionDerivedFactCollection::SendPayloadArgument => {
                ActionDerivedFactPrerequisite::ActionMetaclassIdentity
            }
            _ => ActionDerivedFactPrerequisite::OrderedActionArgumentIdentity,
        };
        let _source_kind = self
            .storage
            .declaration(declaration)
            .map(|value| value.kind);
        self.resolved_outcome(ActionDerivedFactOutcome::Unsupported { prerequisite })
    }

    /// Decides the exact FeatureMembership TypeFeaturing implication from the canonical
    /// membership and effective TypeFeaturing facts. It deliberately does not inspect source
    /// spelling or reconstruct `isFeaturingType` downstream.
    pub(crate) fn type_featuring_check(
        &self,
        symbol: &SymbolIdentity,
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
        let prerequisite = match kind {
            SpecializationCheckKind::FeatureCrossing => {
                SpecializationCheckPrerequisite::CrossFeatureProjection
            }
            SpecializationCheckKind::FeatureObject | SpecializationCheckKind::FeatureOccurrence => {
                SpecializationCheckPrerequisite::FeatureTypingMetaclassAndLibraryAnchor
            }
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
        symbol: &SymbolIdentity,
    ) -> QueryOutcome<DerivedElementOwner> {
        let declaration = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let Some(rule) = element_derived_owner_rule() else {
            return QueryOutcome::Unsupported;
        };
        if rule.metaclass != "Element" || rule.kind != ElementDerivedOwnerKind::Owner {
            return QueryOutcome::Unsupported;
        }
        let _rule_id = rule.rule_id;
        let value = self
            .storage
            .declaration(declaration)
            .and_then(|declaration| declaration.owner)
            .and_then(|owner| self.symbol_identity(owner))
            .map_or(DerivedElementOwner::NoOwner, DerivedElementOwner::Owner);
        self.resolved_outcome(value)
    }

    /// Projects one exact Root Element documentation-form derivation from canonical
    /// documentation records. It does not inspect source syntax or recreate ownership paths.
    pub(crate) fn element_derived_documentation(
        &self,
        symbol: &SymbolIdentity,
        collection: ElementDerivedDocumentationCollection,
    ) -> QueryOutcome<Box<[Documentation]>> {
        let declaration = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let Some(rule) = element_derived_documentation_rule(collection) else {
            return QueryOutcome::Unsupported;
        };
        if rule.metaclass != "Element" {
            return QueryOutcome::Unsupported;
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
        symbol: &SymbolIdentity,
        collection: NamespaceDerivedElementCollection,
    ) -> QueryOutcome<Box<[SymbolIdentity]>> {
        let declaration = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let Some(rule) = namespace_derived_element_rule(collection) else {
            return QueryOutcome::Unsupported;
        };
        if rule.metaclass != "Namespace"
            || self
                .storage
                .declaration(declaration)
                .is_none_or(|value| !DeclarationDomain::Namespace.accepts(value.kind))
        {
            return QueryOutcome::Unsupported;
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
        symbol: &SymbolIdentity,
    ) -> QueryOutcome<Box<[NamespaceImportDerivedElement]>> {
        let namespace = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let Some(rule) = namespace_import_derived_element_rule() else {
            return QueryOutcome::Unsupported;
        };
        if rule.metaclass != "NamespaceImport"
            || rule.kind != NamespaceImportDerivedElementKind::ImportedElement
            || self
                .storage
                .declaration(namespace)
                .is_none_or(|value| !DeclarationDomain::Namespace.accepts(value.kind))
        {
            return QueryOutcome::Unsupported;
        }
        let _rule_id = rule.rule_id;
        let mut values = Vec::new();
        for (index, declaration) in self.storage.declarations.iter().enumerate() {
            if declaration.owner != Some(namespace) || declaration.kind != DeclarationKind::Import {
                continue;
            }
            let import = match DeclarationId::from_index(index) {
                Ok(import) => import,
                Err(_) => return QueryOutcome::Unsupported,
            };
            let relationships =
                self.relationships_of_kinds(import, &[ReferenceKind::NamespaceImport]);
            let relationship = match relationships.as_ref() {
                [] => continue,
                [relationship] => relationship,
                _ => return QueryOutcome::Unsupported,
            };
            let Some(import) = self.symbol_identity(import) else {
                return QueryOutcome::Unsupported;
            };
            values.push(NamespaceImportDerivedElement {
                import,
                relationship: relationship.clone(),
            });
        }
        values.sort_by(|left, right| left.import.cmp(&right.import));
        self.resolved_outcome(values.into_boxed_slice())
    }

    pub(crate) fn requirement_usage_typing(
        &self,
        symbol: &SymbolIdentity,
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
            return QueryOutcome::Unsupported;
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
                    .filter_map(|(target, _)| self.symbol_identity(*target))
                    .collect::<Vec<_>>();
                RequirementUsageTyping::Ambiguous(candidates.into_boxed_slice())
            }
            (Some(relationship), None) => match &relationship.target {
                RelationshipTarget::Resolved(target) => {
                    let target_is_requirement_definition = self
                        .identity_declarations(target)
                        .into_iter()
                        .any(|target| {
                            self.storage.declaration(target).is_some_and(|declaration| {
                                declaration.kind == DeclarationKind::RequirementDefinition
                            })
                        });
                    if target_is_requirement_definition {
                        RequirementUsageTyping::Resolved(TypeReference {
                            symbol: target.clone(),
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
                .symbol_identity(target)
                .map(SatisfyEndpoint::Resolved)
                .unwrap_or(SatisfyEndpoint::Unresolved),
            Some(ResolutionStatus::Ambiguous(candidates)) => SatisfyEndpoint::Ambiguous(
                self.resolution
                    .ambiguous_candidates(candidates)
                    .iter()
                    .filter_map(|candidate| self.symbol_identity(*candidate))
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
                    identity: self.symbol_identity(id)?,
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
            left.location
                .document
                .cmp(&right.location.document)
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
                .symbol_identity(*target)
                .map(BindingEndpoint::Resolved)
                .unwrap_or(BindingEndpoint::Unresolved),
            binding::BindingEndpointFact::Ambiguous(candidates) => BindingEndpoint::Ambiguous(
                candidates
                    .iter()
                    .filter_map(|candidate| self.symbol_identity(*candidate))
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
                    identity: self.symbol_identity(fact.connector)?,
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
            left.location
                .document
                .cmp(&right.location.document)
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
                .symbol_identity(target)
                .map(VerificationRequirement::Resolved)
                .unwrap_or(VerificationRequirement::Unresolved),
            Some(ResolutionStatus::Ambiguous(candidates)) => VerificationRequirement::Ambiguous(
                self.resolution
                    .ambiguous_candidates(candidates)
                    .iter()
                    .filter_map(|candidate| self.symbol_identity(*candidate))
                    .collect(),
            ),
            Some(ResolutionStatus::Unsupported) => VerificationRequirement::Unsupported,
            Some(ResolutionStatus::Unresolved) | Some(ResolutionStatus::NonConverged) => {
                VerificationRequirement::Unresolved
            }
            None => VerificationRequirement::Unsupported,
        };
        let mut values = Vec::new();
        for (index, declaration) in self.storage.declarations.iter().enumerate() {
            let Some(document) = self.storage.document(declaration.document) else {
                continue;
            };
            if document.role != SourceRole::Workspace
                || declaration.kind != DeclarationKind::VerifyRequirement
            {
                continue;
            }
            let Some(id) = DeclarationId::from_index(index).ok() else {
                continue;
            };
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
                self.symbol_identity(id),
                self.symbol_identity(case_id),
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
            left.location
                .document
                .cmp(&right.location.document)
                .then_with(|| left.location.range.cmp(&right.location.range))
                .then_with(|| left.identity.cmp(&right.identity))
        });
        self.resolved_outcome(values.into_boxed_slice())
    }

    pub(crate) fn effective_types(
        &self,
        symbol: &SymbolIdentity,
    ) -> QueryOutcome<Box<[EffectiveType]>> {
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
                    symbol: self.symbol_identity(*target)?,
                    origin: match source {
                        types::EffectiveTypeSource::Direct => EffectiveTypeOrigin::Direct,
                        types::EffectiveTypeSource::Inherited(from) => {
                            EffectiveTypeOrigin::Inherited(self.symbol_identity(*from)?)
                        }
                    },
                })
            })
            .collect::<Vec<_>>();
        types.sort_by(|left, right| left.symbol.cmp(&right.symbol));
        self.resolved_outcome(types.into_boxed_slice())
    }

    /// The settled standard-library anchor used by `checkPartDefinitionSpecialization`.
    ///
    /// A missing anchor remains `Unresolved`; multiple standard-library candidates remain
    /// `Ambiguous` with every canonical identity. Callers therefore never need to recover the
    /// anchor from a rendered name or substitute a workspace declaration.
    pub(crate) fn part_definition_specialization_anchor(&self) -> QueryOutcome<SymbolIdentity> {
        self.library_specialization_anchor("sysml-2.0:8.3.11.2:checkPartDefinitionSpecialization")
    }

    pub(crate) fn library_specialization_anchor(
        &self,
        rule_id: &str,
    ) -> QueryOutcome<SymbolIdentity> {
        self.library_rule_anchor(rule_id)
    }

    /// The canonical anchor outcome for one explicitly selected branch of a generated
    /// specialization rule. `Default` preserves the legacy single-anchor projection.
    pub(crate) fn library_specialization_anchor_branch(
        &self,
        rule_id: &str,
        branch: LibrarySpecializationAnchorBranch,
    ) -> QueryOutcome<SymbolIdentity> {
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
    pub(crate) fn library_rule_anchor(&self, rule_id: &str) -> QueryOutcome<SymbolIdentity> {
        self.library_anchor_outcome(self.resolution.library_specialization_anchor(rule_id))
    }

    pub(crate) fn library_anchor_outcome(
        &self,
        outcome: Option<&LibrarySpecializationAnchor>,
    ) -> QueryOutcome<SymbolIdentity> {
        match outcome {
            Some(LibrarySpecializationAnchor::Resolved(anchor)) => self
                .symbol_identity(*anchor)
                .map_or(QueryOutcome::Unresolved, |anchor| {
                    self.resolved_outcome(anchor)
                }),
            Some(LibrarySpecializationAnchor::Ambiguous(candidates)) => QueryOutcome::Ambiguous(
                candidates
                    .iter()
                    .filter_map(|candidate| self.symbol_identity(*candidate))
                    .collect(),
            ),
            Some(LibrarySpecializationAnchor::Missing) | None => QueryOutcome::Unresolved,
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
            return QueryOutcome::Unresolved;
        };
        if lowered_redefinition_source_kind(rule.metaclass).is_some() {
            self.resolved_outcome(())
        } else {
            QueryOutcome::Unsupported
        }
    }

    pub(crate) fn direct_supertypes(
        &self,
        symbol: &SymbolIdentity,
        scope: SpecializationScope,
    ) -> QueryOutcome<Box<[SymbolIdentity]>> {
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
        symbol: &SymbolIdentity,
        scope: SpecializationScope,
    ) -> QueryOutcome<Box<[SymbolIdentity]>> {
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
        symbol: &SymbolIdentity,
        scope: SpecializationScope,
    ) -> QueryOutcome<Box<[SymbolIdentity]>> {
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

    pub(crate) fn featuring_type(
        &self,
        symbol: &SymbolIdentity,
    ) -> QueryOutcome<Option<SymbolIdentity>> {
        let declaration = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        if self.types.featuring_requires_snapshots(declaration) {
            return QueryOutcome::Unsupported;
        }
        let featuring = self
            .types
            .featuring_types(declaration)
            .iter()
            .filter_map(|(owner, _)| self.symbol_identity(*owner))
            .collect::<Vec<_>>();
        match featuring.as_slice() {
            [] => self.resolved_outcome(None),
            [owner] => self.resolved_outcome(Some(owner.clone())),
            _ => QueryOutcome::Ambiguous(featuring.into_iter().map(Some).collect()),
        }
    }

    /// Every effective featuring type produced by the canonical TypeFeaturing/FeatureChaining
    /// fact family, retaining authored versus implied provenance.
    pub(crate) fn featuring_types(
        &self,
        symbol: &SymbolIdentity,
    ) -> QueryOutcome<Box<[TypeReference]>> {
        let declaration = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let values = self
            .types
            .featuring_types(declaration)
            .iter()
            .filter_map(|(target, provenance)| {
                self.symbol_identity(*target).map(|symbol| TypeReference {
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
                QueryOutcome::Unsupported
            } else {
                QueryOutcome::UnsupportedWith(values)
            }
        } else {
            self.resolved_outcome(values)
        }
    }

    pub(crate) fn conforms_to(
        &self,
        specific: &SymbolIdentity,
        general: &SymbolIdentity,
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
        specific: &SymbolIdentity,
        general: &SymbolIdentity,
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
                            types::SpecializationScope::FeatureSpecialization,
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
        subsetting: &SymbolIdentity,
        subsetted: &SymbolIdentity,
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

pub(crate) fn internal_scope(scope: SpecializationScope) -> types::SpecializationScope {
    match scope {
        SpecializationScope::AnySpecialization => types::SpecializationScope::AnySpecialization,
        SpecializationScope::Subclassification => types::SpecializationScope::Subclassification,
        SpecializationScope::FeatureSpecialization => {
            types::SpecializationScope::FeatureSpecialization
        }
    }
}

pub(crate) fn document_range(
    storage: &SemanticModelStorage,
    document: DocumentId,
    span: &Span,
) -> Result<TextRange, ResolutionError> {
    let parsed = &storage
        .document(document)
        .ok_or(ResolutionError::InvalidStorage)?
        .parsed;
    let range = parsed.range(span).ok_or(ResolutionError::InvalidStorage)?;
    Ok(TextRange {
        start: TextPosition {
            line: range.start.line.saturating_sub(1),
            character: u32::try_from(range.start.column.saturating_sub(1))
                .map_err(|_| ResolutionError::Capacity)?,
        },
        end: TextPosition {
            line: range.end.line.saturating_sub(1),
            character: u32::try_from(range.end.column.saturating_sub(1))
                .map_err(|_| ResolutionError::Capacity)?,
        },
    })
}

pub(crate) fn parser_diagnostic_category(
    category: Option<sysml_v2_parser::DiagnosticCategory>,
) -> DiagnosticCategory {
    match category {
        Some(sysml_v2_parser::DiagnosticCategory::ParseError) => {
            DiagnosticCategory::MalformedSyntax
        }
        Some(sysml_v2_parser::DiagnosticCategory::UnsupportedGrammarForm) => {
            DiagnosticCategory::UnsupportedSyntax
        }
        Some(sysml_v2_parser::DiagnosticCategory::UnresolvedSymbol) => {
            DiagnosticCategory::Unresolved
        }
        None => DiagnosticCategory::UnclassifiedParser,
    }
}

pub(crate) fn parse_error_range(
    document: &ParsedDocument,
    error: &ParseError,
) -> Option<TextRange> {
    let start_offset = error.offset?;
    let end_offset = start_offset.checked_add(error.length.unwrap_or(1))?;
    let start = document.source.position_at(start_offset)?;
    let end = document.source.position_at(end_offset).unwrap_or(start);
    Some(TextRange {
        start: TextPosition {
            line: start.line.saturating_sub(1),
            character: u32::try_from(start.column.saturating_sub(1)).ok()?,
        },
        end: TextPosition {
            line: end.line.saturating_sub(1),
            character: u32::try_from(end.column.saturating_sub(1)).ok()?,
        },
    })
}

/// The public code for a construct this publication does not model.
///
/// Exhaustive by construction: a new lowering family cannot be added without deciding its code.
pub(crate) fn unsupported_construct_code(family: UnsupportedFamily) -> DiagnosticCode {
    match family {
        UnsupportedFamily::PackageMember => DiagnosticCode::UnsupportedPackageMember,
        UnsupportedFamily::PartDefinitionMember => DiagnosticCode::UnsupportedPartDefinitionMember,
        UnsupportedFamily::PartUsageMember => DiagnosticCode::UnsupportedPartUsageMember,
        UnsupportedFamily::AttributeMember => DiagnosticCode::UnsupportedAttributeMember,
        UnsupportedFamily::RequirementDefinitionMember => {
            DiagnosticCode::UnsupportedRequirementDefinitionMember
        }
        UnsupportedFamily::PortDefinitionMember => DiagnosticCode::UnsupportedPortDefinitionMember,
        UnsupportedFamily::PortUsageMember => DiagnosticCode::UnsupportedPortUsageMember,
        UnsupportedFamily::ActionDefinitionMember => {
            DiagnosticCode::UnsupportedActionDefinitionMember
        }
        UnsupportedFamily::ActionUsageMember => DiagnosticCode::UnsupportedActionUsageMember,
        UnsupportedFamily::StateDefinitionMember => {
            DiagnosticCode::UnsupportedStateDefinitionMember
        }
        UnsupportedFamily::ConnectionDefinitionMember => {
            DiagnosticCode::UnsupportedConnectionDefinitionMember
        }
        UnsupportedFamily::InterfaceDefinitionMember => {
            DiagnosticCode::UnsupportedInterfaceDefinitionMember
        }
        UnsupportedFamily::ViewDefinitionMember => DiagnosticCode::UnsupportedViewDefinitionMember,
        UnsupportedFamily::ConstraintDefinitionMember => {
            DiagnosticCode::UnsupportedConstraintDefinitionMember
        }
        UnsupportedFamily::CalcDefinitionMember => DiagnosticCode::UnsupportedCalcDefinitionMember,
        UnsupportedFamily::RenderingDefinitionMember => {
            DiagnosticCode::UnsupportedRenderingDefinitionMember
        }
        UnsupportedFamily::OccurrenceDefinitionMember => {
            DiagnosticCode::UnsupportedOccurrenceDefinitionMember
        }
        UnsupportedFamily::AnalysisCaseDefinitionMember => {
            DiagnosticCode::UnsupportedAnalysisCaseDefinitionMember
        }
        UnsupportedFamily::CaseDefinitionMember => DiagnosticCode::UnsupportedCaseDefinitionMember,
        UnsupportedFamily::VerificationCaseDefinitionMember => {
            DiagnosticCode::UnsupportedVerificationCaseDefinitionMember
        }
        UnsupportedFamily::UseCaseDefinitionMember => {
            DiagnosticCode::UnsupportedUseCaseDefinitionMember
        }
        UnsupportedFamily::ReferenceUsageMember => DiagnosticCode::UnsupportedReferenceUsageMember,
        UnsupportedFamily::RelationshipBodyMember => {
            DiagnosticCode::UnsupportedRelationshipBodyMember
        }
        UnsupportedFamily::ParserUnsupported => DiagnosticCode::UnsupportedParserConstruct,
    }
}

/// What one authored reference's settled outcome reports, or `None` when it resolved.
///
/// A resolved reference has nothing to report; that is not the same answer as any of the failure
/// states below, and the three failure classes stay distinct all the way to the consumer.
pub(crate) fn reference_diagnostic(
    kind: ReferenceKind,
    status: ResolutionStatus,
) -> Option<(DiagnosticSeverity, DiagnosticCode)> {
    match status {
        ResolutionStatus::Resolved(_) => None,
        ResolutionStatus::Unresolved => Some((
            DiagnosticSeverity::Warning,
            match kind {
                ReferenceKind::FeatureTyping => DiagnosticCode::UnresolvedTypeReference,
                ReferenceKind::Subclassification => DiagnosticCode::UnresolvedSpecializesReference,
                ReferenceKind::NamespaceImport | ReferenceKind::MembershipImport => {
                    DiagnosticCode::UnresolvedImportTarget
                }
                // A view names what it shows; an expose target that resolves to nothing means the
                // view shows nothing, which is a different thing to a reader than an unresolved
                // name in a declaration.
                ReferenceKind::ViewExpose => DiagnosticCode::ViewExposeUnresolved,
                _ => DiagnosticCode::UnresolvedReference,
            },
        )),
        ResolutionStatus::Unsupported => Some((
            DiagnosticSeverity::Warning,
            match kind {
                ReferenceKind::NamespaceImport
                | ReferenceKind::MembershipImport
                | ReferenceKind::FilterImport => DiagnosticCode::UnsupportedFilteredImport,
                _ => DiagnosticCode::UnsupportedReference,
            },
        )),
        ResolutionStatus::NonConverged => Some((
            DiagnosticSeverity::Error,
            DiagnosticCode::NonConvergedResolution,
        )),
        ResolutionStatus::Ambiguous(_) => Some((
            DiagnosticSeverity::Error,
            match kind {
                ReferenceKind::NamespaceImport | ReferenceKind::MembershipImport => {
                    DiagnosticCode::AmbiguousImportTarget
                }
                _ => DiagnosticCode::AmbiguousReference,
            },
        )),
    }
}

/// Where a *declaration* writes its own name.
///
/// Distinct from [`identifier_range`], which searches a reference span and takes the last
/// word-boundary match because a qualified path names its target in the final segment. A
/// declaration span covers the whole declaration including its body, so the same rule finds the
/// last mention of the name anywhere inside -- for `part def Vehicle { part engine : Vehicle; }`
/// it points at the body's reference rather than at the declared name.
///
/// The declared name is in the header, after the keywords and after an optional `<shortName>`, so
/// the search is bounded to the text before the body opener and skips the short-name group. A
/// declaration whose header is unrecoverable -- a parse recovery that lost its `{` or `;` -- falls
/// back to the whole-span search rather than losing its location entirely.
pub(crate) fn declaration_identifier_range(
    storage: &SemanticModelStorage,
    document: DocumentId,
    span: &Span,
    identifier: &str,
) -> Result<TextRange, ResolutionError> {
    let parsed = &storage
        .document(document)
        .ok_or(ResolutionError::InvalidStorage)?
        .parsed;
    let source = parsed
        .source
        .slice(span)
        .ok_or(ResolutionError::InvalidStorage)?;
    let header = source
        .find(['{', ';'])
        .map_or(source, |body| &source[..body]);
    let relative = word_boundary_matches(header, identifier)
        .find(|start| !inside_short_name(header, *start))
        .or_else(|| word_boundary_matches(header, identifier).next())
        .or_else(|| word_boundary_matches(source, identifier).last())
        .ok_or(ResolutionError::InvalidStorage)?;
    identifier_text_range(parsed, span, relative, identifier.len())
}

/// Whether `start` falls inside an unclosed `<`...`>` short-name group.
pub(crate) fn inside_short_name(header: &str, start: usize) -> bool {
    let before = &header[..start];
    before
        .rfind('<')
        .is_some_and(|open| !before[open..].contains('>'))
}

/// Every occurrence of `identifier` in `text` that is not part of a longer identifier.
pub(crate) fn word_boundary_matches<'a>(
    text: &'a str,
    identifier: &'a str,
) -> impl Iterator<Item = usize> + 'a {
    text.match_indices(identifier)
        .filter(move |(start, _)| {
            let before = text[..*start].chars().next_back();
            let after = text[*start + identifier.len()..].chars().next();
            !before.is_some_and(identifier_character) && !after.is_some_and(identifier_character)
        })
        .map(|(start, _)| start)
}

pub(crate) fn identifier_range(
    storage: &SemanticModelStorage,
    document: DocumentId,
    span: &Span,
    identifier: &str,
) -> Result<TextRange, ResolutionError> {
    let parsed = &storage
        .document(document)
        .ok_or(ResolutionError::InvalidStorage)?
        .parsed;
    let source = parsed
        .source
        .slice(span)
        .ok_or(ResolutionError::InvalidStorage)?;
    let relative = word_boundary_matches(source, identifier)
        .last()
        .ok_or(ResolutionError::InvalidStorage)?;
    identifier_text_range(parsed, span, relative, identifier.len())
}

pub(crate) fn identifier_text_range(
    parsed: &ParsedDocument,
    span: &Span,
    relative: usize,
    length: usize,
) -> Result<TextRange, ResolutionError> {
    let start_offset = span
        .offset
        .checked_add(relative)
        .ok_or(ResolutionError::Capacity)?;
    let end_offset = start_offset
        .checked_add(length)
        .ok_or(ResolutionError::Capacity)?;
    let start = parsed
        .source
        .position_at(start_offset)
        .ok_or(ResolutionError::InvalidStorage)?;
    let end = parsed
        .source
        .position_at(end_offset)
        .ok_or(ResolutionError::InvalidStorage)?;
    Ok(TextRange {
        start: TextPosition {
            line: start.line.saturating_sub(1),
            character: u32::try_from(start.column.saturating_sub(1))
                .map_err(|_| ResolutionError::Capacity)?,
        },
        end: TextPosition {
            line: end.line.saturating_sub(1),
            character: u32::try_from(end.column.saturating_sub(1))
                .map_err(|_| ResolutionError::Capacity)?,
        },
    })
}

pub(crate) fn identifier_character(character: char) -> bool {
    character.is_alphanumeric() || matches!(character, '_' | '-')
}

pub(crate) fn valid_identifier(value: &str) -> bool {
    let mut characters = value.chars();
    characters
        .next()
        .is_some_and(|first| first.is_alphabetic() || first == '_')
        && characters.all(identifier_character)
}

pub(crate) fn range_contains(range: TextRange, position: TextPosition) -> bool {
    range.start <= position && position <= range.end
}

pub(crate) fn target_order(
    left: &NavigationTarget,
    right: &NavigationTarget,
) -> std::cmp::Ordering {
    left.location
        .document
        .cmp(&right.location.document)
        .then_with(|| left.location.range.cmp(&right.location.range))
        .then_with(|| left.name.cmp(&right.name))
}

pub(crate) fn location_order(left: &SourceLocation, right: &SourceLocation) -> std::cmp::Ordering {
    left.document
        .cmp(&right.document)
        .then_with(|| left.range.cmp(&right.range))
        .then_with(|| left.role.cmp(&right.role))
}

pub(crate) fn declaration_qualified_name(
    storage: &SemanticModelStorage,
    mut declaration: DeclarationId,
) -> Option<String> {
    let mut names = Vec::new();
    loop {
        let value = storage.declaration(declaration)?;
        if let Some(name) = value.name.and_then(|name| storage.symbol(name)) {
            names.push(name);
        }
        let Some(owner) = value.owner else {
            break;
        };
        declaration = owner;
    }
    names.reverse();
    Some(names.join("::"))
}

impl ResolvedSemanticModel {
    /// Everything a later publication needs to reuse this one as a library.
    ///
    /// The parsed documents come out by reference-counted handle rather than by copy, so reuse
    /// shares one parse of the library across every publication built against it.
    pub(crate) fn prepared_library(
        &self,
    ) -> Result<crate::pipeline::PreparedLibrary, crate::pipeline::CoordinatorError> {
        let documents = self
            .storage
            .documents
            .iter()
            .map(|document| crate::pipeline::PreparedDocument {
                identity: document.identity.clone(),
                role: document.role,
                parsed: Arc::clone(&document.parsed),
                parse_errors: document.parse_errors.to_vec(),
            })
            .collect();
        Ok(crate::pipeline::PreparedLibrary {
            documents,
            settled: self
                .settled_library()
                .map_err(|_| crate::pipeline::CoordinatorError::ConstructionFailed)?,
        })
    }

    /// The reusable settled state of a library-only publication.
    pub(crate) fn settled_library(&self) -> Result<SettledLibrary, ResolutionError> {
        let mut root_names = std::collections::BTreeSet::new();
        for declaration in self.storage.declarations.iter() {
            if declaration.owner.is_some() {
                continue;
            }
            if let Some(name) = declaration.name.and_then(|name| self.storage.symbol(name)) {
                root_names.insert(name.into());
            }
        }
        let mut unsettled_roots = std::collections::BTreeSet::new();
        for (index, reference) in self.storage.references.iter().enumerate() {
            let id =
                AuthoredReferenceId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
            let settled = matches!(
                self.resolution.outcome(id),
                Some(ResolutionStatus::Resolved(_)) | Some(ResolutionStatus::Unsupported)
            );
            if settled {
                continue;
            }
            let Some((segments, _)) = self.storage.paths.get(reference.path) else {
                continue;
            };
            let Some(first) = segments.first().and_then(|id| self.storage.symbol(*id)) else {
                continue;
            };
            unsettled_roots.insert(first.into());
        }
        Ok(SettledLibrary {
            outcomes: self.resolution.outcomes.clone(),
            root_names,
            unsettled_roots,
        })
    }
}

impl SemanticModelStorage {
    pub(crate) fn resolve(
        self,
        policy: EvaluationPolicy,
        library: Option<&SettledLibrary>,
        reported: &[Box<str>],
    ) -> Result<ResolvedSemanticModel, ResolutionError> {
        let has_recovery = self
            .documents
            .iter()
            .any(|document| !document.parse_errors.is_empty())
            || !self.recovery.is_empty();
        let has_unsupported = !self.unsupported.is_empty();
        let seed = library
            .filter(|library| library.admits(&self))
            .map(|library| library.outcomes.as_ref());
        let (direct_names, effective_imports, memberships, mut resolution) = resolve_dense(
            &self.declarations,
            &self.memberships,
            &self.paths,
            &self.references,
            seed,
        )?;
        // `checkPartDefinitionSpecialization` is an implied semantic fact, so its anchor and
        // relationships are settled here, before every index and diagnostic consumer below. The
        // lookup is owned by semantic construction: neither a renderer nor a validation rule gets
        // to rediscover `Parts::Part` from text or a display path.
        let library_anchors = library_specialization_anchors(&self);
        if matches!(resolution.solver_status, SolverStatus::Converged) {
            let mut implied = resolution.implied_relationships.into_vec();
            implied.extend(
                synthesize_generated_library_specializations(
                    &self,
                    &self.references,
                    &resolution.outcomes,
                    &library_anchors,
                )?
                .into_vec(),
            );
            implied.extend(
                synthesize_generated_library_redefinitions(
                    &self,
                    &self.references,
                    &library_anchors,
                )?
                .into_vec(),
            );
            implied.extend(
                synthesize_feature_membership_type_featurings(&self, &self.references)?.into_vec(),
            );
            implied.sort_by_key(|relationship| {
                (
                    relationship.kind,
                    relationship.source.0,
                    relationship.target.0,
                )
            });
            implied.dedup();
            resolution.implied_relationships = implied.into_boxed_slice();
        }
        resolution.library_specialization_anchors = library_anchors;
        let completeness = if has_recovery {
            PublicationCompleteness::ParseRecovery
        } else if has_unsupported {
            PublicationCompleteness::UnsupportedSyntax
        } else if !matches!(resolution.solver_status, SolverStatus::Converged) {
            PublicationCompleteness::NonConverged
        } else {
            PublicationCompleteness::Complete
        };
        let settled = compute_evaluation(&self, &resolution, policy);
        let (evaluation, filter_conditions) = match settled {
            SettledEvaluation::Settled { facts, filters } => (facts, Some(filters)),
            SettledEvaluation::Vacuous => (Box::default(), None),
        };
        let has_evaluation = !evaluation.is_empty();
        let identities = IdentityIndex::build(&self)?;
        let documents = DocumentIndex::build(&self)?;
        let reverse_references =
            ReverseReferenceIndex::build(self.declarations.len(), &resolution)?;
        let effective_scopes = EffectiveScopeIndex::build(
            self.declarations.len(),
            &direct_names,
            &effective_imports,
            &resolution.inherited_names,
        )?;
        let facts = inspection::ElementFactIndex::build(&self, &resolution, &evaluation)?;
        let bindings = binding::BindingConnectorIndex::build(&self, &resolution)?;
        // A barrier product, not a solver family: every type fact here is derived from settled
        // outcomes and feeds nothing back into scope, imports or inheritance. The resolver's own
        // ancestor closure for inherited names stays separate and unchanged -- widening that one
        // would silently change name resolution.
        let type_facts = types::TypeIndex::build(&self, &resolution)?;
        let mut model = ResolvedSemanticModel {
            storage: self,
            direct_names,
            effective_imports,
            identities,
            documents,
            memberships,
            reverse_references,
            effective_scopes,
            facts,
            bindings,
            types: type_facts,
            resolution,
            evaluation,
            expressions: expression::ExpressionIndex::default(),
            diagnostics: Box::default(),
            diagnostics_by_document: Box::default(),
            metadata: PublicationMetadata {
                phase: PublicationPhase::Resolved,
                completeness,
                has_evaluation,
            },
        };
        // Expression facts read the type closure and the settled evaluation, so they are assembled
        // once the model holds both, and before diagnostics, which report what they settled.
        model.expressions = expression::ExpressionIndex::build(&model, filter_conditions)?;
        // Last barrier product: diagnostics report what every earlier phase settled, so they are
        // derived from the assembled model rather than from any one phase's intermediate state.
        let (diagnostics, diagnostics_by_document) = model.derive_diagnostics(reported)?;
        model.diagnostics = diagnostics;
        model.diagnostics_by_document = diagnostics_by_document;
        Ok(model)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_feature_relationship_collection_contracts_are_complete_and_closed() {
        assert_eq!(GENERATED_FEATURE_DERIVED_RELATIONSHIP_RULES.len(), 5);
        assert_eq!(
            GENERATED_FEATURE_DERIVED_RELATIONSHIP_RULES
                .iter()
                .map(|rule| (rule.rule_id, rule.metaclass, rule.collection))
                .collect::<Vec<_>>(),
            vec![
                (
                    "kerml-1.0:8.3.3.3.4:deriveFeatureOwnedFeatureChaining",
                    "Feature",
                    FeatureDerivedRelationshipCollection::OwnedFeatureChaining,
                ),
                (
                    "kerml-1.0:8.3.3.3.4:deriveFeatureOwnedRedefinition",
                    "Feature",
                    FeatureDerivedRelationshipCollection::OwnedRedefinition,
                ),
                (
                    "kerml-1.0:8.3.3.3.4:deriveFeatureOwnedSubsetting",
                    "Feature",
                    FeatureDerivedRelationshipCollection::OwnedSubsetting,
                ),
                (
                    "kerml-1.0:8.3.3.3.4:deriveFeatureOwnedTypeFeaturing",
                    "Feature",
                    FeatureDerivedRelationshipCollection::OwnedTypeFeaturing,
                ),
                (
                    "kerml-1.0:8.3.3.3.4:deriveFeatureOwnedTyping",
                    "Feature",
                    FeatureDerivedRelationshipCollection::OwnedTyping,
                ),
            ]
        );
    }

    #[test]
    fn generated_type_fact_contract_rows_are_canonical_rule_id_ordered() {
        assert!(GENERATED_TYPE_DERIVED_FACT_RULES
            .windows(2)
            .all(|pair| pair[0].rule_id < pair[1].rule_id));
    }

    #[test]
    fn generated_unconditional_library_specialization_table_covers_all_manifest_check_rules() {
        assert_eq!(generated_library_specialization_rule_count(), 85);
        assert_eq!(
            generated_conditional_library_specialization_rule_count(),
            56
        );
        let unique_rules = GENERATED_LIBRARY_SPECIALIZATION_RULES
            .iter()
            .map(|rule| rule.rule_id)
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(
            unique_rules.len(),
            85,
            "rule IDs must remain unique map keys"
        );
        assert!(GENERATED_LIBRARY_SPECIALIZATION_RULES.iter().all(|rule| {
            !rule.rule_id.is_empty() && !rule.metaclass.is_empty() && !rule.anchor.is_empty()
        }));
        assert!(GENERATED_CONDITIONAL_LIBRARY_SPECIALIZATION_RULES
            .iter()
            .all(|rule| {
                !rule.rule_id.is_empty()
                    && !rule.metaclass.is_empty()
                    && !rule.anchor.is_empty()
                    && (rule.predicate != LibrarySpecializationPredicate::CompositeOwnedBy
                        || rule.owner_metaclasses.len() == 2)
            }));
        assert_eq!(generated_library_redefinition_rule_count(), 1);
        assert_eq!(
            GENERATED_LIBRARY_REDEFINITION_RULES,
            &[LibraryRedefinitionRule {
                rule_id: "kerml-1.0:8.3.4.9.5:checkPayloadFeatureRedefinition",
                metaclass: "PayloadFeature",
                anchor: "Transfers::Transfer::payload",
            }]
        );
        for rule in GENERATED_LIBRARY_SPECIALIZATION_RULES {
            assert_eq!(
                library_specialization_rules(rule.metaclass).count(),
                1,
                "{} must retain its exact generated applicability metaclass",
                rule.rule_id
            );
        }

        // The stored fact owner is total over the generated table even when the admitted model
        // contains no library declarations. That makes every missing prerequisite explicit, and
        // catches a manifest/generator change that adds a rule without publication coverage.
        let anchors = library_specialization_anchors(&storage_with_one_filter());
        assert_eq!(anchors.by_rule.len(), 148);
        for rule in GENERATED_LIBRARY_SPECIALIZATION_RULES {
            assert!(matches!(
                anchors.outcome(rule.rule_id),
                Some(LibrarySpecializationAnchor::Missing)
            ));
        }
        for rule in GENERATED_CONDITIONAL_LIBRARY_SPECIALIZATION_RULES {
            assert!(matches!(
                anchors.outcome(rule.rule_id),
                Some(LibrarySpecializationAnchor::Missing)
            ));
            if rule.true_anchor.is_some() {
                assert!(matches!(
                    anchors.outcome_for(
                        rule.rule_id,
                        LibrarySpecializationAnchorBranch::PredicateTrue,
                    ),
                    Some(LibrarySpecializationAnchor::Missing)
                ));
            }
        }
        for rule in GENERATED_LIBRARY_REDEFINITION_RULES {
            assert!(matches!(
                anchors.outcome(rule.rule_id),
                Some(LibrarySpecializationAnchor::Missing)
            ));
        }
        let shared_anchor_rules = GENERATED_LIBRARY_SPECIALIZATION_RULES
            .iter()
            .filter(|rule| rule.anchor == "Performances::literalIntegerEvaluations")
            .collect::<Vec<_>>();
        assert_eq!(shared_anchor_rules.len(), 2);
        assert_ne!(
            shared_anchor_rules[0].rule_id,
            shared_anchor_rules[1].rule_id
        );
        assert!(matches!(
            anchors.outcome(shared_anchor_rules[0].rule_id),
            Some(LibrarySpecializationAnchor::Missing)
        ));
        assert!(matches!(
            anchors.outcome(shared_anchor_rules[1].rule_id),
            Some(LibrarySpecializationAnchor::Missing)
        ));
    }

    #[test]
    fn generated_conditional_specialization_rows_preserve_the_typed_manifest_contract() {
        let manifest_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("specifications/constraint_manifest.toml");
        let manifest = spec42_constraint_manifest::ConstraintManifest::load_toml(&manifest_path)
            .expect("load typed manifest for generated-row drift test");
        let mut expected = manifest
            .specifications
            .iter()
            .flat_map(|specification| &specification.constraints)
            .filter_map(|entry| {
                entry
                    .conditional_specializes_from_library
                    .as_ref()
                    .map(|contract| {
                        (
                            entry.rule_id.clone(),
                            entry.metaclass.clone(),
                            contract.predicate,
                            contract.owner_metaclasses.clone(),
                            contract.true_anchor.clone(),
                            contract.anchor.clone(),
                        )
                    })
            })
            .collect::<Vec<_>>();
        expected.sort_by(|left, right| left.0.cmp(&right.0));
        let actual = GENERATED_CONDITIONAL_LIBRARY_SPECIALIZATION_RULES
            .iter()
            .map(|rule| {
                (
                    rule.rule_id.to_string(),
                    rule.metaclass.to_string(),
                    rule.predicate,
                    rule.owner_metaclasses
                        .iter()
                        .map(|metaclass| (*metaclass).to_string())
                        .collect::<Vec<_>>(),
                    rule.true_anchor.map(str::to_string),
                    rule.anchor.to_string(),
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(actual, expected);
    }

    #[test]
    fn membership_role_specialization_predicates_use_the_canonical_role_and_owner_facts() {
        let storage = storage_with_membership_role_specializations();
        let rule = |id| {
            GENERATED_CONDITIONAL_LIBRARY_SPECIALIZATION_RULES
                .iter()
                .find(|rule| rule.rule_id == id)
                .expect("generated membership-role rule")
        };
        let id = |index| DeclarationId::from_index(index).expect("test declaration id");

        let framed = rule("sysml-2.0:8.3.21.4:checkConcernUsageFramedConcernSpecialization");
        assert!(conditional_library_specialization_predicate_holds(
            &storage,
            id(2),
            framed
        ));
        assert!(!conditional_library_specialization_predicate_holds(
            &storage,
            id(9),
            framed
        ));

        let constraint =
            rule("sysml-2.0:8.3.20.4:checkConstraintUsageRequirementConstraintSpecialization");
        assert!(conditional_library_specialization_predicate_holds(
            &storage,
            id(3),
            constraint
        ));
        assert_eq!(
            conditional_library_specialization_anchor_branch(&storage, id(3), constraint),
            LibrarySpecializationAnchorBranch::PredicateTrue,
        );
        assert_eq!(
            conditional_library_specialization_anchor_branch(&storage, id(4), constraint),
            LibrarySpecializationAnchorBranch::Default,
        );

        let actor = rule("sysml-2.0:8.3.11.3:checkPartUsageActorSpecialization");
        assert!(conditional_library_specialization_predicate_holds(
            &storage,
            id(5),
            actor
        ));
        assert_eq!(
            conditional_library_specialization_anchor_branch(&storage, id(5), actor),
            LibrarySpecializationAnchorBranch::PredicateTrue,
        );
        assert_eq!(
            conditional_library_specialization_anchor_branch(&storage, id(6), actor),
            LibrarySpecializationAnchorBranch::Default,
        );
        assert!(!conditional_library_specialization_predicate_holds(
            &storage,
            id(9),
            actor
        ));

        let stakeholder = rule("sysml-2.0:8.3.11.3:checkPartUsageStakeholderSpecialization");
        assert!(conditional_library_specialization_predicate_holds(
            &storage,
            id(8),
            stakeholder
        ));

        let verification =
            rule("sysml-2.0:8.3.21.9:checkRequirementUsageRequirementVerificationSpecialization");
        assert!(conditional_library_specialization_predicate_holds(
            &storage,
            id(10),
            verification
        ));
    }

    #[test]
    fn accept_action_specialization_predicates_use_trigger_and_subaction_facts() {
        let storage = storage_with_accept_action_specializations();
        let rule = |id| {
            GENERATED_CONDITIONAL_LIBRARY_SPECIALIZATION_RULES
                .iter()
                .find(|rule| rule.rule_id == id)
                .expect("generated accept-action rule")
        };
        let id = |index| DeclarationId::from_index(index).expect("test declaration id");
        let ordinary = rule("sysml-2.0:8.3.17.2:checkAcceptActionUsageSpecialization");
        let subaction = rule("sysml-2.0:8.3.17.2:checkAcceptActionUsageSubactionSpecialization");
        let trigger = rule("sysml-2.0:8.3.17.2:checkAcceptActionUsageTriggerActionSpecialization");

        // A top-level accept action is non-trigger but not a subaction.
        assert!(conditional_library_specialization_predicate_holds(
            &storage,
            id(1),
            ordinary
        ));
        assert!(!conditional_library_specialization_predicate_holds(
            &storage,
            id(1),
            subaction
        ));
        assert!(!conditional_library_specialization_predicate_holds(
            &storage,
            id(1),
            trigger
        ));

        // The exact owner/composite facts add the subaction specialization without changing the
        // non-trigger one.
        assert!(conditional_library_specialization_predicate_holds(
            &storage,
            id(3),
            ordinary
        ));
        assert!(conditional_library_specialization_predicate_holds(
            &storage,
            id(3),
            subaction
        ));
        assert!(!conditional_library_specialization_predicate_holds(
            &storage,
            id(3),
            trigger
        ));

        // A transition trigger is explicitly suppressed from both non-trigger rules.
        assert!(!conditional_library_specialization_predicate_holds(
            &storage,
            id(5),
            ordinary
        ));
        assert!(!conditional_library_specialization_predicate_holds(
            &storage,
            id(5),
            subaction
        ));
        assert!(conditional_library_specialization_predicate_holds(
            &storage,
            id(5),
            trigger
        ));
    }

    #[test]
    fn if_action_specialization_selects_the_canonical_else_action_branch() {
        let id = |index| DeclarationId::from_index(index).expect("test declaration id");
        let storage = SemanticModelStorage {
            documents: Box::new([]),
            declarations: vec![
                declaration(DocumentId(0), None, None, DeclarationKind::Package),
                declaration(DocumentId(0), Some(id(0)), None, DeclarationKind::If),
                declaration(DocumentId(0), Some(id(0)), None, DeclarationKind::If),
                declaration(DocumentId(0), Some(id(0)), None, DeclarationKind::If),
            ]
            .into_boxed_slice(),
            declaration_facts: vec![
                DeclarationFacts::none(),
                DeclarationFacts::none(),
                DeclarationFacts {
                    has_else_action: Some(false),
                    ..DeclarationFacts::none()
                },
                DeclarationFacts {
                    has_else_action: Some(true),
                    ..DeclarationFacts::none()
                },
            ]
            .into_boxed_slice(),
            memberships: Box::new([]),
            references: Box::new([]),
            documentation: Box::new([]),
            feature_values: Box::new([]),
            unsupported: Box::new([]),
            recovery: Box::new([]),
            symbols: SymbolTableBuilder::default().freeze(),
            paths: SymbolPathArenaBuilder::default().freeze(),
            evaluation_facts: Box::new([]),
            unit_tokens: Box::new([]),
            filter_conditions: Box::new([]),
            invocations: Box::new([]),
        };
        let rule = GENERATED_CONDITIONAL_LIBRARY_SPECIALIZATION_RULES
            .iter()
            .find(|rule| rule.rule_id == "sysml-2.0:8.3.17.10:checkIfActionUsageSpecialization")
            .expect("generated if-action rule");

        assert!(!conditional_library_specialization_predicate_holds(
            &storage,
            id(1),
            rule
        ));
        assert!(conditional_library_specialization_predicate_holds(
            &storage,
            id(2),
            rule
        ));
        assert!(conditional_library_specialization_predicate_holds(
            &storage,
            id(3),
            rule
        ));
        assert_eq!(
            conditional_library_specialization_anchor_branch(&storage, id(2), rule),
            LibrarySpecializationAnchorBranch::Default,
        );
        assert_eq!(
            conditional_library_specialization_anchor_branch(&storage, id(3), rule),
            LibrarySpecializationAnchorBranch::PredicateTrue,
        );
    }

    #[test]
    fn flow_specialization_predicates_use_owned_endpoint_facts_and_suppress_incomplete_forms() {
        let id = |index| DeclarationId::from_index(index).expect("test declaration id");
        let storage = SemanticModelStorage {
            documents: Box::new([]),
            declarations: vec![
                declaration(DocumentId(0), None, None, DeclarationKind::Package),
                declaration(
                    DocumentId(0),
                    Some(id(0)),
                    None,
                    DeclarationKind::FlowDefinition,
                ),
                declaration(
                    DocumentId(0),
                    Some(id(1)),
                    None,
                    DeclarationKind::ConnectionUsage,
                ),
                declaration(
                    DocumentId(0),
                    Some(id(1)),
                    None,
                    DeclarationKind::ConnectionUsage,
                ),
                declaration(
                    DocumentId(0),
                    Some(id(0)),
                    None,
                    DeclarationKind::FlowDefinition,
                ),
                declaration(
                    DocumentId(0),
                    Some(id(4)),
                    None,
                    DeclarationKind::ConnectionUsage,
                ),
                declaration(DocumentId(0), Some(id(0)), None, DeclarationKind::Flow),
                declaration(DocumentId(0), Some(id(0)), None, DeclarationKind::Flow),
            ]
            .into_boxed_slice(),
            declaration_facts: vec![
                DeclarationFacts::none(),
                DeclarationFacts::none(),
                DeclarationFacts {
                    positional_end: Some(0),
                    ..DeclarationFacts::none()
                },
                DeclarationFacts {
                    positional_end: Some(1),
                    ..DeclarationFacts::none()
                },
                DeclarationFacts::none(),
                DeclarationFacts {
                    positional_end: Some(0),
                    ..DeclarationFacts::none()
                },
                DeclarationFacts {
                    owned_end_feature_count: Some(2),
                    ..DeclarationFacts::none()
                },
                DeclarationFacts::none(),
            ]
            .into_boxed_slice(),
            memberships: Box::new([]),
            references: Box::new([]),
            documentation: Box::new([]),
            feature_values: Box::new([]),
            unsupported: Box::new([]),
            recovery: Box::new([]),
            symbols: SymbolTableBuilder::default().freeze(),
            paths: SymbolPathArenaBuilder::default().freeze(),
            evaluation_facts: Box::new([]),
            unit_tokens: Box::new([]),
            filter_conditions: Box::new([]),
            invocations: Box::new([]),
        };
        let rule = |rule_id| {
            GENERATED_CONDITIONAL_LIBRARY_SPECIALIZATION_RULES
                .iter()
                .find(|rule| rule.rule_id == rule_id)
                .expect("generated flow rule")
        };
        let binary = rule("sysml-2.0:8.3.16.2:checkFlowDefinitionBinarySpecialization");
        let flow_usage = rule("sysml-2.0:8.3.16.3:checkFlowUsageFlowSpecialization");
        let flow_with_ends = rule("kerml-1.0:8.3.4.9.2:checkFlowWithEndsSpecialization");

        assert!(conditional_library_specialization_predicate_holds(
            &storage,
            id(1),
            binary
        ));
        assert!(!conditional_library_specialization_predicate_holds(
            &storage,
            id(4),
            binary
        ));
        assert!(conditional_library_specialization_predicate_holds(
            &storage,
            id(6),
            flow_usage
        ));
        assert!(conditional_library_specialization_predicate_holds(
            &storage,
            id(6),
            flow_with_ends
        ));
        assert!(!conditional_library_specialization_predicate_holds(
            &storage,
            id(7),
            flow_usage
        ));
        assert!(!conditional_library_specialization_predicate_holds(
            &storage,
            id(7),
            flow_with_ends
        ));
    }

    #[test]
    fn parser_categories_are_mapped_without_code_or_message_heuristics() {
        assert_eq!(
            parser_diagnostic_category(Some(sysml_v2_parser::DiagnosticCategory::ParseError)),
            DiagnosticCategory::MalformedSyntax
        );
        assert_eq!(
            parser_diagnostic_category(Some(
                sysml_v2_parser::DiagnosticCategory::UnsupportedGrammarForm
            )),
            DiagnosticCategory::UnsupportedSyntax
        );
        assert_eq!(
            parser_diagnostic_category(Some(sysml_v2_parser::DiagnosticCategory::UnresolvedSymbol)),
            DiagnosticCategory::Unresolved
        );
        assert_eq!(
            parser_diagnostic_category(None),
            DiagnosticCategory::UnclassifiedParser
        );
    }

    #[derive(Debug, Clone, Copy)]
    struct TestReference {
        source: DeclarationId,
        kind: ReferenceKind,
        path: SymbolPathId,
        flags: RelationshipFlags,
    }

    impl ResolutionReferenceFact for TestReference {
        fn source(&self) -> DeclarationId {
            self.source
        }

        fn kind(&self) -> ReferenceKind {
            self.kind
        }

        fn path(&self) -> SymbolPathId {
            self.path
        }

        fn flags(&self) -> RelationshipFlags {
            self.flags
        }
    }

    fn declaration(
        document: DocumentId,
        owner: Option<DeclarationId>,
        name: Option<SymbolId>,
        kind: DeclarationKind,
    ) -> Declaration {
        Declaration {
            document,
            owner,
            name,
            anonymous_ordinal: name.is_none().then_some(0),
            kind,
            span: Span::dummy(),
        }
    }

    /// A storage holding nothing but one authored `filter` condition.
    ///
    /// Enough for the evaluation pass, which reads the condition table, the evaluation candidates
    /// and the references, and nothing else.
    fn storage_with_one_filter() -> SemanticModelStorage {
        SemanticModelStorage {
            documents: Box::new([]),
            declarations: Box::new([]),
            declaration_facts: Box::new([]),
            memberships: Box::new([]),
            references: Box::new([]),
            documentation: Box::new([]),
            feature_values: Box::new([]),
            unsupported: Box::new([]),
            recovery: Box::new([]),
            symbols: SymbolTableBuilder::default().freeze(),
            paths: SymbolPathArenaBuilder::default().freeze(),
            evaluation_facts: Box::new([]),
            unit_tokens: Box::new([]),
            filter_conditions: Box::new([AuthoredFilterCondition {
                owner: DeclarationId(0),
                document: DocumentId(0),
                form: FilterForm::View,
                span: Span::dummy(),
                shape: ExpressionEvalShape::Literal(EvaluatedValue::Integer(5)),
                predicate: FilterPredicate::Unsupported,
            }]),
            invocations: Box::new([]),
        }
    }

    /// One semantic storage slice with every typed membership role used by the exact generated
    /// specialization contracts. No parser text or declaration name participates in the test:
    /// applicability and branch selection consume the canonical declaration kind/owner facts.
    fn storage_with_membership_role_specializations() -> SemanticModelStorage {
        let id = |index| DeclarationId::from_index(index).expect("test declaration id");
        SemanticModelStorage {
            documents: Box::new([]),
            declarations: vec![
                declaration(DocumentId(0), None, None, DeclarationKind::Package),
                declaration(
                    DocumentId(0),
                    Some(id(0)),
                    None,
                    DeclarationKind::RequirementDefinition,
                ),
                declaration(DocumentId(0), Some(id(1)), None, DeclarationKind::Frame),
                declaration(
                    DocumentId(0),
                    Some(id(1)),
                    None,
                    DeclarationKind::AssumeConstraintUsage,
                ),
                declaration(
                    DocumentId(0),
                    Some(id(1)),
                    None,
                    DeclarationKind::RequireConstraintUsage,
                ),
                declaration(
                    DocumentId(0),
                    Some(id(1)),
                    None,
                    DeclarationKind::RequirementActor,
                ),
                declaration(
                    DocumentId(0),
                    Some(id(0)),
                    None,
                    DeclarationKind::CaseDefinition,
                ),
                declaration(DocumentId(0), Some(id(6)), None, DeclarationKind::CaseActor),
                declaration(
                    DocumentId(0),
                    Some(id(1)),
                    None,
                    DeclarationKind::StakeholderUsage,
                ),
                declaration(DocumentId(0), Some(id(1)), None, DeclarationKind::PartUsage),
                declaration(
                    DocumentId(0),
                    Some(id(1)),
                    None,
                    DeclarationKind::VerifyRequirement,
                ),
            ]
            .into_boxed_slice(),
            declaration_facts: vec![DeclarationFacts::none(); 11].into_boxed_slice(),
            memberships: Box::new([]),
            references: Box::new([]),
            documentation: Box::new([]),
            feature_values: Box::new([]),
            unsupported: Box::new([]),
            recovery: Box::new([]),
            symbols: SymbolTableBuilder::default().freeze(),
            paths: SymbolPathArenaBuilder::default().freeze(),
            evaluation_facts: Box::new([]),
            unit_tokens: Box::new([]),
            filter_conditions: Box::new([]),
            invocations: Box::new([]),
        }
    }

    /// Canonical fact slice for the three exact `AcceptActionUsage` predicates. It separates the
    /// action's metaclass, its explicit trigger membership fact, and the independently owned
    /// composite/owner facts used by `isSubactionUsage()`.
    fn storage_with_accept_action_specializations() -> SemanticModelStorage {
        let id = |index| DeclarationId::from_index(index).expect("test declaration id");
        let non_trigger = |composite| DeclarationFacts {
            modifiers: DeclarationModifiers {
                composite,
                ..DeclarationModifiers::default()
            },
            is_trigger_action: Some(false),
            ..DeclarationFacts::none()
        };
        SemanticModelStorage {
            documents: Box::new([]),
            declarations: vec![
                declaration(DocumentId(0), None, None, DeclarationKind::Package),
                declaration(
                    DocumentId(0),
                    Some(id(0)),
                    None,
                    DeclarationKind::AcceptActionUsage,
                ),
                declaration(
                    DocumentId(0),
                    Some(id(0)),
                    None,
                    DeclarationKind::ActionDefinition,
                ),
                declaration(
                    DocumentId(0),
                    Some(id(2)),
                    None,
                    DeclarationKind::AcceptActionUsage,
                ),
                declaration(
                    DocumentId(0),
                    Some(id(0)),
                    None,
                    DeclarationKind::Transition,
                ),
                declaration(
                    DocumentId(0),
                    Some(id(4)),
                    None,
                    DeclarationKind::AcceptActionUsage,
                ),
            ]
            .into_boxed_slice(),
            declaration_facts: vec![
                DeclarationFacts::none(),
                non_trigger(true),
                DeclarationFacts::none(),
                non_trigger(true),
                DeclarationFacts::none(),
                DeclarationFacts {
                    modifiers: DeclarationModifiers {
                        composite: true,
                        ..DeclarationModifiers::default()
                    },
                    is_trigger_action: Some(true),
                    ..DeclarationFacts::none()
                },
            ]
            .into_boxed_slice(),
            memberships: Box::new([]),
            references: Box::new([]),
            documentation: Box::new([]),
            feature_values: Box::new([]),
            unsupported: Box::new([]),
            recovery: Box::new([]),
            symbols: SymbolTableBuilder::default().freeze(),
            paths: SymbolPathArenaBuilder::default().freeze(),
            evaluation_facts: Box::new([]),
            unit_tokens: Box::new([]),
            filter_conditions: Box::new([]),
            invocations: Box::new([]),
        }
    }

    fn resolution_with_status(status: SolverStatus) -> ResolutionResults {
        ResolutionResults {
            outcomes: Box::new([]),
            ambiguous_candidates: Box::new([]),
            inherited_names: NameIndex::build(Vec::new()).unwrap(),
            solver_status: status,
            implied_relationships: Box::new([]),
            library_specialization_anchors: LibrarySpecializationAnchorFacts::default(),
            work: ResolutionWork::default(),
        }
    }

    /// A converged publication settles every authored condition.
    #[test]
    fn every_authored_filter_condition_settles_when_resolution_converges() {
        let storage = storage_with_one_filter();
        let settled = compute_evaluation(
            &storage,
            &resolution_with_status(SolverStatus::Converged),
            EvaluationPolicy::Evaluate,
        );
        let SettledEvaluation::Settled { filters, .. } = settled else {
            panic!("a converged publication settles its filter conditions");
        };
        assert_eq!(filters.len(), storage.filter_conditions.len());
        assert_eq!(
            filters[0].state,
            EvaluationState::Literal(crate::evaluation::EvaluatedScalar::Integer(5))
        );
    }

    /// A publication whose resolution did not converge has no outcomes to publish, and saying so
    /// is not the same as failing to build.
    ///
    /// The evaluation pass and the filter table used to be joined by index, so the branch that
    /// could not produce an outcome per condition published none and the join rejected the whole
    /// publication -- turning an explicitly supported incomplete state into a construction error
    /// for any model that authored a `filter`.
    #[test]
    fn a_non_converged_publication_settles_nothing_rather_than_failing() {
        let storage = storage_with_one_filter();
        let settled = compute_evaluation(
            &storage,
            &resolution_with_status(SolverStatus::NonConverged),
            EvaluationPolicy::Evaluate,
        );
        assert!(
            matches!(settled, SettledEvaluation::Vacuous),
            "a non-converged publication must settle no expression outcome"
        );
    }

    fn reference(
        source: DeclarationId,
        kind: ReferenceKind,
        path: SymbolPathId,
        wildcard: bool,
    ) -> TestReference {
        TestReference {
            source,
            kind,
            path,
            flags: RelationshipFlags {
                wildcard,
                ..RelationshipFlags::default()
            },
        }
    }

    struct ResolverFixture {
        declarations: Box<[Declaration]>,
        memberships: Box<[MembershipRecord]>,
        paths: SymbolPathArena,
        references: Box<[TestReference]>,
    }

    fn memberships_for(
        declarations: &[Declaration],
        public_imports: &[DeclarationId],
    ) -> Box<[MembershipRecord]> {
        declarations
            .iter()
            .enumerate()
            .map(|(index, declaration)| {
                let member = DeclarationId::from_index(index).unwrap();
                MembershipRecord {
                    member,
                    kind: if declaration.kind == DeclarationKind::Import {
                        MembershipKind::Import
                    } else {
                        MembershipKind::Owning
                    },
                    visibility: if public_imports.contains(&member) {
                        Visibility::Public
                    } else {
                        Visibility::Default
                    },
                    span: Span::dummy(),
                }
            })
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn cross_file_fixture(duplicate_vehicle: bool) -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let definitions_name = symbols.intern("Definitions").unwrap();
        let usage_name = symbols.intern("Usage").unwrap();
        let vehicle_name = symbols.intern("Vehicle").unwrap();
        let v_name = symbols.intern("v").unwrap();

        let mut paths = SymbolPathArenaBuilder::default();
        let definitions_path = paths.push(&[definitions_name], false).unwrap();
        let vehicle_path = paths.push(&[vehicle_name], false).unwrap();

        let definition_document = DocumentId(0);
        let usage_document = DocumentId(1);
        let definitions = DeclarationId(0);
        let usage = DeclarationId(2);
        let import = DeclarationId(3);
        let v = DeclarationId(4);
        let mut declarations = vec![
            declaration(
                definition_document,
                None,
                Some(definitions_name),
                DeclarationKind::Package,
            ),
            declaration(
                definition_document,
                Some(definitions),
                Some(vehicle_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                usage_document,
                None,
                Some(usage_name),
                DeclarationKind::Package,
            ),
            declaration(usage_document, Some(usage), None, DeclarationKind::Import),
            declaration(
                usage_document,
                Some(usage),
                Some(v_name),
                DeclarationKind::PartUsage,
            ),
        ];
        if duplicate_vehicle {
            declarations.push(declaration(
                definition_document,
                Some(definitions),
                Some(vehicle_name),
                DeclarationKind::PartDefinition,
            ));
        }

        let references = vec![
            reference(
                import,
                ReferenceKind::NamespaceImport,
                definitions_path,
                true,
            ),
            reference(v, ReferenceKind::FeatureTyping, vehicle_path, false),
        ];
        let _symbols = symbols.freeze();
        let memberships = memberships_for(&declarations, &[]);
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    fn resolve_fixture(
        fixture: &ResolverFixture,
    ) -> (NameIndex, NameIndex, MembershipIndex, ResolutionResults) {
        resolve_dense(
            &fixture.declarations,
            &fixture.memberships,
            &fixture.paths,
            &fixture.references,
            None,
        )
        .unwrap()
    }

    fn transitive_import_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let a_name = symbols.intern("A").unwrap();
        let b_name = symbols.intern("B").unwrap();
        let c_name = symbols.intern("C").unwrap();
        let use_name = symbols.intern("Use").unwrap();
        let thing_name = symbols.intern("Thing").unwrap();
        let v_name = symbols.intern("v").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let a_path = paths.push(&[a_name], false).unwrap();
        let b_path = paths.push(&[b_name], false).unwrap();
        let c_path = paths.push(&[c_name], false).unwrap();
        let thing_path = paths.push(&[thing_name], false).unwrap();

        let declarations = vec![
            declaration(DocumentId(0), None, Some(a_name), DeclarationKind::Package),
            declaration(
                DocumentId(0),
                Some(DeclarationId(0)),
                Some(thing_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(DocumentId(1), None, Some(b_name), DeclarationKind::Package),
            declaration(
                DocumentId(1),
                Some(DeclarationId(2)),
                None,
                DeclarationKind::Import,
            ),
            declaration(DocumentId(2), None, Some(c_name), DeclarationKind::Package),
            declaration(
                DocumentId(2),
                Some(DeclarationId(4)),
                None,
                DeclarationKind::Import,
            ),
            declaration(
                DocumentId(3),
                None,
                Some(use_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentId(3),
                Some(DeclarationId(6)),
                None,
                DeclarationKind::Import,
            ),
            declaration(
                DocumentId(3),
                Some(DeclarationId(6)),
                Some(v_name),
                DeclarationKind::PartUsage,
            ),
        ];
        let references = vec![
            reference(
                DeclarationId(3),
                ReferenceKind::NamespaceImport,
                a_path,
                true,
            ),
            reference(
                DeclarationId(5),
                ReferenceKind::NamespaceImport,
                b_path,
                true,
            ),
            reference(
                DeclarationId(7),
                ReferenceKind::NamespaceImport,
                c_path,
                true,
            ),
            reference(
                DeclarationId(8),
                ReferenceKind::FeatureTyping,
                thing_path,
                false,
            ),
        ];
        let _symbols = symbols.freeze();
        let memberships = memberships_for(&declarations, &[DeclarationId(3), DeclarationId(5)]);
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    fn imported_target_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let a_name = symbols.intern("A").unwrap();
        let nested_name = symbols.intern("Nested").unwrap();
        let thing_name = symbols.intern("Thing").unwrap();
        let b_name = symbols.intern("B").unwrap();
        let v_name = symbols.intern("v").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let a_path = paths.push(&[a_name], false).unwrap();
        let nested_path = paths.push(&[nested_name], false).unwrap();
        let thing_path = paths.push(&[thing_name], false).unwrap();

        let declarations = vec![
            declaration(DocumentId(0), None, Some(a_name), DeclarationKind::Package),
            declaration(
                DocumentId(0),
                Some(DeclarationId(0)),
                Some(nested_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentId(0),
                Some(DeclarationId(1)),
                Some(thing_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(DocumentId(1), None, Some(b_name), DeclarationKind::Package),
            declaration(
                DocumentId(1),
                Some(DeclarationId(3)),
                None,
                DeclarationKind::Import,
            ),
            declaration(
                DocumentId(1),
                Some(DeclarationId(3)),
                None,
                DeclarationKind::Import,
            ),
            declaration(
                DocumentId(1),
                Some(DeclarationId(3)),
                Some(v_name),
                DeclarationKind::PartUsage,
            ),
        ];
        let references = vec![
            reference(
                DeclarationId(4),
                ReferenceKind::NamespaceImport,
                a_path,
                true,
            ),
            reference(
                DeclarationId(5),
                ReferenceKind::NamespaceImport,
                nested_path,
                true,
            ),
            reference(
                DeclarationId(6),
                ReferenceKind::FeatureTyping,
                thing_path,
                false,
            ),
        ];
        let _symbols = symbols.freeze();
        let memberships = memberships_for(&declarations, &[]);
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    fn cyclic_import_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let a_name = symbols.intern("A").unwrap();
        let b_name = symbols.intern("B").unwrap();
        let type_a_name = symbols.intern("TypeA").unwrap();
        let type_b_name = symbols.intern("TypeB").unwrap();
        let usage_name = symbols.intern("value").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let b_path = paths.push(&[b_name], false).unwrap();
        let a_path = paths.push(&[a_name], false).unwrap();
        let type_b_path = paths.push(&[type_b_name], false).unwrap();

        let declarations = vec![
            declaration(DocumentId(0), None, Some(a_name), DeclarationKind::Package),
            declaration(
                DocumentId(0),
                Some(DeclarationId(0)),
                Some(type_a_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(DeclarationId(0)),
                None,
                DeclarationKind::Import,
            ),
            declaration(
                DocumentId(0),
                Some(DeclarationId(0)),
                Some(usage_name),
                DeclarationKind::PartUsage,
            ),
            declaration(DocumentId(1), None, Some(b_name), DeclarationKind::Package),
            declaration(
                DocumentId(1),
                Some(DeclarationId(4)),
                Some(type_b_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentId(1),
                Some(DeclarationId(4)),
                None,
                DeclarationKind::Import,
            ),
        ];
        let references = vec![
            reference(
                DeclarationId(2),
                ReferenceKind::NamespaceImport,
                b_path,
                true,
            ),
            reference(
                DeclarationId(6),
                ReferenceKind::NamespaceImport,
                a_path,
                true,
            ),
            reference(
                DeclarationId(3),
                ReferenceKind::FeatureTyping,
                type_b_path,
                false,
            ),
        ];
        let _symbols = symbols.freeze();
        let memberships = memberships_for(&declarations, &[DeclarationId(2), DeclarationId(6)]);
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    fn qualified_import_target_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let a_name = symbols.intern("A").unwrap();
        let nested_name = symbols.intern("Nested").unwrap();
        let thing_name = symbols.intern("Thing").unwrap();
        let b_name = symbols.intern("B").unwrap();
        let use_name = symbols.intern("Use").unwrap();
        let v_name = symbols.intern("v").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let a_path = paths.push(&[a_name], false).unwrap();
        let qualified_nested_path = paths.push(&[b_name, nested_name], false).unwrap();
        let thing_path = paths.push(&[thing_name], false).unwrap();

        let declarations = vec![
            declaration(DocumentId(0), None, Some(a_name), DeclarationKind::Package),
            declaration(
                DocumentId(0),
                Some(DeclarationId(0)),
                Some(nested_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentId(0),
                Some(DeclarationId(1)),
                Some(thing_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(DocumentId(1), None, Some(b_name), DeclarationKind::Package),
            declaration(
                DocumentId(1),
                Some(DeclarationId(3)),
                None,
                DeclarationKind::Import,
            ),
            declaration(
                DocumentId(2),
                None,
                Some(use_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentId(2),
                Some(DeclarationId(5)),
                None,
                DeclarationKind::Import,
            ),
            declaration(
                DocumentId(2),
                Some(DeclarationId(5)),
                Some(v_name),
                DeclarationKind::PartUsage,
            ),
        ];
        let references = vec![
            reference(
                DeclarationId(4),
                ReferenceKind::NamespaceImport,
                a_path,
                true,
            ),
            reference(
                DeclarationId(6),
                ReferenceKind::NamespaceImport,
                qualified_nested_path,
                true,
            ),
            reference(
                DeclarationId(7),
                ReferenceKind::FeatureTyping,
                thing_path,
                false,
            ),
        ];
        let _symbols = symbols.freeze();
        let memberships = memberships_for(&declarations, &[DeclarationId(4)]);
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    fn redefinition_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let p_name = symbols.intern("P").unwrap();
        let base_name = symbols.intern("Base").unwrap();
        let child_name = symbols.intern("Child").unwrap();
        let mass_name = symbols.intern("mass").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let base_path = paths.push(&[base_name], false).unwrap();

        let package = DeclarationId(0);
        let base = DeclarationId(1);
        let child = DeclarationId(3);
        let declarations = vec![
            declaration(DocumentId(0), None, Some(p_name), DeclarationKind::Package),
            declaration(
                DocumentId(0),
                Some(package),
                Some(base_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(base),
                Some(mass_name),
                DeclarationKind::AttributeUsage,
            ),
            declaration(
                DocumentId(0),
                Some(package),
                Some(child_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(child),
                Some(mass_name),
                DeclarationKind::AttributeUsage,
            ),
        ];
        let memberships = declarations
            .iter()
            .enumerate()
            .map(|(index, declaration)| {
                let member = DeclarationId::from_index(index).unwrap();
                let kind = if matches!(
                    declaration.kind,
                    DeclarationKind::AttributeUsage | DeclarationKind::PartUsage
                ) {
                    MembershipKind::Feature
                } else {
                    MembershipKind::Owning
                };
                MembershipRecord {
                    member,
                    kind,
                    visibility: Visibility::Default,
                    span: Span::dummy(),
                }
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let references = vec![reference(
            child,
            ReferenceKind::Subclassification,
            base_path,
            false,
        )];
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn same_name_direct_parent_feature_synthesizes_implied_redefinition() {
        let fixture = redefinition_fixture();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.implied_relationships.as_ref(),
            &[ImpliedRelationship {
                kind: ReferenceKind::Redefinition,
                source: DeclarationId(4),
                target: DeclarationId(2),
            }]
        );
    }

    #[test]
    fn explicit_redefinition_suppresses_implied_duplicate() {
        let mut fixture = redefinition_fixture();
        let mut references = fixture.references.into_vec();
        references.push(reference(
            DeclarationId(4),
            ReferenceKind::Redefinition,
            references[0].path,
            false,
        ));
        fixture.references = references.into_boxed_slice();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert!(resolution.implied_relationships.is_empty());
    }

    #[test]
    fn ambiguous_immediate_parent_candidates_leave_no_implied_fact() {
        let mut fixture = redefinition_fixture();
        let mut declarations = fixture.declarations.into_vec();
        // A second directly owned `mass` feature on Base makes the immediate-parent same-name
        // lookup ambiguous; the synthesis must not guess a target.
        declarations.push(declaration(
            DocumentId(0),
            Some(DeclarationId(1)),
            Some(declarations[2].name.unwrap()),
            DeclarationKind::AttributeUsage,
        ));
        fixture.declarations = declarations.into_boxed_slice();
        let mut memberships = fixture.memberships.into_vec();
        memberships.push(MembershipRecord {
            member: DeclarationId(5),
            kind: MembershipKind::Feature,
            visibility: Visibility::Default,
            span: Span::dummy(),
        });
        fixture.memberships = memberships.into_boxed_slice();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert!(resolution.implied_relationships.is_empty());
    }

    /// Like `redefinition_fixture`, but exposes an unqualified single-segment `mass` symbol path
    /// (in addition to the `Base` path used for the Subclassification reference) so callers can
    /// author an unqualified `Subsetting`/`Redefinition` reference to the inherited `mass`
    /// feature, resolved through lexical/ancestor lookup rather than a qualified path. `Child`'s
    /// own redefining/subsetting attribute (`DeclarationId(4)`) is deliberately left unnamed --
    /// matching an authored `attribute :>> mass = ...;`/`attribute :> mass;`, whose usage has no
    /// name of its own -- so it is never itself indexed under the `mass` name and cannot shadow
    /// the inherited `Base::mass` target it is trying to reach.
    fn redefinition_fixture_with_mass_path() -> (ResolverFixture, SymbolPathId) {
        let mut symbols = SymbolTableBuilder::default();
        let p_name = symbols.intern("P").unwrap();
        let base_name = symbols.intern("Base").unwrap();
        let child_name = symbols.intern("Child").unwrap();
        let mass_name = symbols.intern("mass").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let base_path = paths.push(&[base_name], false).unwrap();
        let mass_path = paths.push(&[mass_name], false).unwrap();

        let package = DeclarationId(0);
        let base = DeclarationId(1);
        let child = DeclarationId(3);
        let declarations = vec![
            declaration(DocumentId(0), None, Some(p_name), DeclarationKind::Package),
            declaration(
                DocumentId(0),
                Some(package),
                Some(base_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(base),
                Some(mass_name),
                DeclarationKind::AttributeUsage,
            ),
            declaration(
                DocumentId(0),
                Some(package),
                Some(child_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(child),
                None,
                DeclarationKind::AttributeUsage,
            ),
        ];
        let memberships = declarations
            .iter()
            .enumerate()
            .map(|(index, declaration)| {
                let member = DeclarationId::from_index(index).unwrap();
                let kind = if matches!(
                    declaration.kind,
                    DeclarationKind::AttributeUsage | DeclarationKind::PartUsage
                ) {
                    MembershipKind::Feature
                } else {
                    MembershipKind::Owning
                };
                MembershipRecord {
                    member,
                    kind,
                    visibility: Visibility::Default,
                    span: Span::dummy(),
                }
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let references = vec![reference(
            child,
            ReferenceKind::Subclassification,
            base_path,
            false,
        )];
        let _symbols = symbols.freeze();
        (
            ResolverFixture {
                declarations: declarations.into_boxed_slice(),
                memberships,
                paths: paths.freeze(),
                references: references.into_boxed_slice(),
            },
            mass_path,
        )
    }

    #[test]
    fn explicit_redefinition_resolves_through_inherited_ancestor_lookup() {
        // `attribute :>> mass = ...;` on `Child` (which specializes `Base`) must resolve its
        // authored `Redefinition` reference against `Base::mass`, an inherited member reachable
        // only through the ancestor-closure lookup built for Subclassification -- not just a
        // directly owned member of `Child` itself.
        let (mut fixture, mass_path) = redefinition_fixture_with_mass_path();
        let mut references = fixture.references.into_vec();
        let redefinition_index = u32::try_from(references.len()).unwrap();
        references.push(reference(
            DeclarationId(4),
            ReferenceKind::Redefinition,
            mass_path,
            false,
        ));
        fixture.references = references.into_boxed_slice();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(redefinition_index)),
            Some(ResolutionStatus::Resolved(DeclarationId(2)))
        );
    }

    #[test]
    fn subsetting_reference_resolves_to_an_inherited_feature_target() {
        // `attribute simpleMass :> mass;` subsets another *feature*, not a type/definition, so
        // `Subsetting` must resolve against `DeclarationDomain::Any` rather than the
        // Subclassification/FeatureTyping `Type` domain, and must reach the inherited `Base::mass`
        // feature through the same ancestor-scoped inherited lookup used by
        // `FeatureTyping`/`Redefinition`.
        let (mut fixture, mass_path) = redefinition_fixture_with_mass_path();
        let mut references = fixture.references.into_vec();
        let subsetting_index = u32::try_from(references.len()).unwrap();
        references.push(reference(
            DeclarationId(4),
            ReferenceKind::Subsetting,
            mass_path,
            false,
        ));
        fixture.references = references.into_boxed_slice();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(subsetting_index)),
            Some(ResolutionStatus::Resolved(DeclarationId(2)))
        );
    }

    #[test]
    fn redefinition_inside_a_usage_body_resolves_through_the_usages_feature_typing_target() {
        // `need : Need { attribute :>> status = ...; }` -- the redefining attribute is owned by a
        // *usage* (`need`), not a def/type, so it has no Subclassification ancestors of its own.
        // `status` is only reachable by first following `need`'s own `FeatureTyping` reference to
        // `Need`, then walking `Need`'s ancestor closure (`Need -> UserRequirement ->
        // ManagedRequirement`) to find `ManagedRequirement::status`. Mirrors
        // tests/snapshots/resolution/enum_status_redefinition.md.
        let mut symbols = SymbolTableBuilder::default();
        let demo_name = symbols.intern("Demo").unwrap();
        let managed_requirement_name = symbols.intern("ManagedRequirement").unwrap();
        let status_name = symbols.intern("status").unwrap();
        let user_requirement_name = symbols.intern("UserRequirement").unwrap();
        let need_def_name = symbols.intern("Need").unwrap();
        let need_usage_name = symbols.intern("need").unwrap();

        let mut paths = SymbolPathArenaBuilder::default();
        let managed_requirement_path = paths.push(&[managed_requirement_name], false).unwrap();
        let user_requirement_path = paths.push(&[user_requirement_name], false).unwrap();
        let need_def_path = paths.push(&[need_def_name], false).unwrap();
        let status_path = paths.push(&[status_name], false).unwrap();

        let demo = DeclarationId(0);
        let managed_requirement = DeclarationId(1);
        let status = DeclarationId(2);
        let user_requirement = DeclarationId(3);
        let need_def = DeclarationId(4);
        let need_usage = DeclarationId(5);
        let declarations = vec![
            declaration(
                DocumentId(0),
                None,
                Some(demo_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(managed_requirement_name),
                DeclarationKind::RequirementDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(managed_requirement),
                Some(status_name),
                DeclarationKind::AttributeUsage,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(user_requirement_name),
                DeclarationKind::RequirementDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(need_def_name),
                DeclarationKind::RequirementDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(need_usage_name),
                DeclarationKind::RequirementUsage,
            ),
            declaration(
                DocumentId(0),
                Some(need_usage),
                None,
                DeclarationKind::AttributeUsage,
            ),
        ];
        let memberships = declarations
            .iter()
            .enumerate()
            .map(|(index, declaration)| {
                let member = DeclarationId::from_index(index).unwrap();
                let kind = if matches!(
                    declaration.kind,
                    DeclarationKind::AttributeUsage | DeclarationKind::RequirementUsage
                ) {
                    MembershipKind::Feature
                } else {
                    MembershipKind::Owning
                };
                MembershipRecord {
                    member,
                    kind,
                    visibility: Visibility::Default,
                    span: Span::dummy(),
                }
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();

        let references = vec![
            reference(
                user_requirement,
                ReferenceKind::Subclassification,
                managed_requirement_path,
                false,
            ),
            reference(
                need_def,
                ReferenceKind::Subclassification,
                user_requirement_path,
                false,
            ),
            reference(
                need_usage,
                ReferenceKind::FeatureTyping,
                need_def_path,
                false,
            ),
            reference(
                DeclarationId(6),
                ReferenceKind::Redefinition,
                status_path,
                false,
            ),
        ];
        let redefinition_index = u32::try_from(references.len() - 1).unwrap();

        let _symbols = symbols.freeze();
        let fixture = ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        };

        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(redefinition_index)),
            Some(ResolutionStatus::Resolved(status))
        );
    }

    #[test]
    fn qualified_reference_resolves_to_an_enum_defs_owned_literal_member() {
        // `enum def StatusKind { enum approved; }` -- StatusKind::approved is looked up through
        // exactly the same generic multi-segment lexical lookup as any other owned member; no
        // enum-specific resolver code is needed once EnumerationDefinition/EnumerationLiteral are
        // lowered as ordinary owned declarations.
        let mut symbols = SymbolTableBuilder::default();
        let demo_name = symbols.intern("Demo").unwrap();
        let status_kind_name = symbols.intern("StatusKind").unwrap();
        let approved_name = symbols.intern("approved").unwrap();
        let alias_name = symbols.intern("aliasToApproved").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let qualified_path = paths
            .push(&[status_kind_name, approved_name], false)
            .unwrap();

        let demo = DeclarationId(0);
        let status_kind = DeclarationId(1);
        let approved = DeclarationId(2);
        let alias = DeclarationId(3);
        let declarations = vec![
            declaration(
                DocumentId(0),
                None,
                Some(demo_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(status_kind_name),
                DeclarationKind::EnumerationDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(status_kind),
                Some(approved_name),
                DeclarationKind::EnumerationLiteral,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(alias_name),
                DeclarationKind::Alias,
            ),
        ];
        let memberships = declarations
            .iter()
            .enumerate()
            .map(|(index, declaration)| {
                let member = DeclarationId::from_index(index).unwrap();
                let kind = match declaration.kind {
                    DeclarationKind::EnumerationLiteral => MembershipKind::Feature,
                    DeclarationKind::Alias => MembershipKind::Alias,
                    _ => MembershipKind::Owning,
                };
                // Interior/final segments of a multi-segment qualified name are looked up through
                // the exported-names index (`build_effective_import_indexes`'s sibling,
                // `build_direct_name_index(.., Some(&memberships))`), which only admits publicly
                // visible members -- the same rule every other owned-member kind is subject to, not
                // an enum-specific one. Publicize the literal explicitly here to exercise that
                // generic path rather than asserting anything enum-specific about visibility
                // defaults.
                let visibility = if declaration.kind == DeclarationKind::EnumerationLiteral {
                    Visibility::Public
                } else {
                    Visibility::Default
                };
                MembershipRecord {
                    member,
                    kind,
                    visibility,
                    span: Span::dummy(),
                }
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let references = vec![reference(
            alias,
            ReferenceKind::AliasBinding,
            qualified_path,
            false,
        )];
        let _symbols = symbols.freeze();
        let fixture = ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        };

        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(approved))
        );
    }

    /// Builds a `Demo { port def Base; port def Derived :> Base; }`-shaped fixture: `Derived`'s
    /// `:>` specialization reference is authored with `conjugated` set per `typing_conjugated`,
    /// exercising `port def`'s participation in the shared Subclassification/FeatureTyping
    /// lexical lookup fixed point (`DeclarationDomain::Type`) exactly like `part def`.
    fn port_def_specialization_fixture(typing_conjugated: bool) -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let demo_name = symbols.intern("Demo").unwrap();
        let base_name = symbols.intern("Base").unwrap();
        let derived_name = symbols.intern("Derived").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let base_path = paths.push(&[base_name], false).unwrap();

        let demo = DeclarationId(0);
        let derived = DeclarationId(2);
        let declarations = vec![
            declaration(
                DocumentId(0),
                None,
                Some(demo_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(base_name),
                DeclarationKind::PortDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(derived_name),
                DeclarationKind::PortDefinition,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let references = vec![TestReference {
            source: derived,
            kind: ReferenceKind::Subclassification,
            path: base_path,
            flags: RelationshipFlags {
                conjugated: typing_conjugated,
                ..RelationshipFlags::default()
            },
        }];
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn port_def_specialization_resolves_through_the_ancestor_fixed_point() {
        let fixture = port_def_specialization_fixture(false);
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    /// Builds a `Demo { occurrence def Base; occurrence def Derived :> Base; }`-shaped fixture,
    /// exercising `occurrence def`'s participation in the shared Subclassification/FeatureTyping
    /// lexical lookup fixed point (`DeclarationDomain::Type`) exactly like `port def`/`state def`.
    fn occurrence_def_specialization_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let demo_name = symbols.intern("Demo").unwrap();
        let base_name = symbols.intern("Base").unwrap();
        let derived_name = symbols.intern("Derived").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let base_path = paths.push(&[base_name], false).unwrap();

        let demo = DeclarationId(0);
        let derived = DeclarationId(2);
        let declarations = vec![
            declaration(
                DocumentId(0),
                None,
                Some(demo_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(base_name),
                DeclarationKind::OccurrenceDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(derived_name),
                DeclarationKind::OccurrenceDefinition,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let references = vec![TestReference {
            source: derived,
            kind: ReferenceKind::Subclassification,
            path: base_path,
            flags: RelationshipFlags::default(),
        }];
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn occurrence_def_specialization_resolves_through_the_ancestor_fixed_point() {
        let fixture = occurrence_def_specialization_fixture();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    /// Builds a `Demo { analysis def Base; analysis def Derived :> Base; }`-shaped fixture,
    /// exercising `analysis def`'s participation in the shared Subclassification/FeatureTyping
    /// lexical lookup fixed point (`DeclarationDomain::Type`) exactly like `occurrence def`.
    fn analysis_case_def_specialization_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let demo_name = symbols.intern("Demo").unwrap();
        let base_name = symbols.intern("Base").unwrap();
        let derived_name = symbols.intern("Derived").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let base_path = paths.push(&[base_name], false).unwrap();

        let demo = DeclarationId(0);
        let derived = DeclarationId(2);
        let declarations = vec![
            declaration(
                DocumentId(0),
                None,
                Some(demo_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(base_name),
                DeclarationKind::AnalysisCaseDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(derived_name),
                DeclarationKind::AnalysisCaseDefinition,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let references = vec![TestReference {
            source: derived,
            kind: ReferenceKind::Subclassification,
            path: base_path,
            flags: RelationshipFlags::default(),
        }];
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn analysis_case_def_specialization_resolves_through_the_ancestor_fixed_point() {
        let fixture = analysis_case_def_specialization_fixture();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    /// Builds a `Demo { Base; Derived :> Base; }`-shaped fixture for the given case-family
    /// `DeclarationKind` (`CaseDefinition`/`VerificationCaseDefinition`/`UseCaseDefinition`),
    /// exercising its participation in the shared Subclassification/FeatureTyping lexical lookup
    /// fixed point (`DeclarationDomain::Type`) exactly like `analysis def`/`occurrence def`.
    fn case_family_def_specialization_fixture(kind: DeclarationKind) -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let demo_name = symbols.intern("Demo").unwrap();
        let base_name = symbols.intern("Base").unwrap();
        let derived_name = symbols.intern("Derived").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let base_path = paths.push(&[base_name], false).unwrap();

        let demo = DeclarationId(0);
        let derived = DeclarationId(2);
        let declarations = vec![
            declaration(
                DocumentId(0),
                None,
                Some(demo_name),
                DeclarationKind::Package,
            ),
            declaration(DocumentId(0), Some(demo), Some(base_name), kind),
            declaration(DocumentId(0), Some(demo), Some(derived_name), kind),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let references = vec![TestReference {
            source: derived,
            kind: ReferenceKind::Subclassification,
            path: base_path,
            flags: RelationshipFlags::default(),
        }];
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn case_def_specialization_resolves_through_the_ancestor_fixed_point() {
        let fixture = case_family_def_specialization_fixture(DeclarationKind::CaseDefinition);
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    #[test]
    fn verification_case_def_specialization_resolves_through_the_ancestor_fixed_point() {
        let fixture =
            case_family_def_specialization_fixture(DeclarationKind::VerificationCaseDefinition);
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    #[test]
    fn use_case_def_specialization_resolves_through_the_ancestor_fixed_point() {
        let fixture = case_family_def_specialization_fixture(DeclarationKind::UseCaseDefinition);
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    #[test]
    fn conjugated_port_typing_reference_resolves_to_the_correct_target_and_carries_the_flag() {
        // `port source : ~InputPort;` -- the conjugated `~` polarity must be visible as an
        // explicit fact on the authored reference, distinct from the (unconjugated) target
        // declaration itself, which the resolved outcome still names correctly.
        let fixture = port_def_specialization_fixture(true);
        assert!(fixture.references[0].flags.conjugated);
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    #[test]
    fn non_conjugated_port_typing_reference_does_not_carry_the_conjugated_flag() {
        // Regression guard: an ordinary (non-`~`) port typing/specialization reference must not
        // spuriously pick up the conjugated flag.
        let fixture = port_def_specialization_fixture(false);
        assert!(!fixture.references[0].flags.conjugated);
    }

    /// Builds a `Demo { item def Base; item def Derived :> Base; }`-shaped fixture: `Derived`'s
    /// `:>` specialization reference exercises `item def`'s participation in the shared
    /// Subclassification/FeatureTyping lexical lookup fixed point (`DeclarationDomain::Type`)
    /// exactly like `part def`/`port def`.
    fn item_def_specialization_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let demo_name = symbols.intern("Demo").unwrap();
        let base_name = symbols.intern("Base").unwrap();
        let derived_name = symbols.intern("Derived").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let base_path = paths.push(&[base_name], false).unwrap();

        let demo = DeclarationId(0);
        let derived = DeclarationId(2);
        let declarations = vec![
            declaration(
                DocumentId(0),
                None,
                Some(demo_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(base_name),
                DeclarationKind::ItemDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(derived_name),
                DeclarationKind::ItemDefinition,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let references = vec![TestReference {
            source: derived,
            kind: ReferenceKind::Subclassification,
            path: base_path,
            flags: RelationshipFlags::default(),
        }];
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn item_def_specialization_resolves_through_the_ancestor_fixed_point() {
        let fixture = item_def_specialization_fixture();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    /// Builds a `Demo { class Base; class Derived :> Base; }`-shaped fixture: `Derived`'s `:>`
    /// specialization reference exercises KerML `class def`'s participation in the shared
    /// Subclassification/FeatureTyping lexical lookup fixed point (`DeclarationDomain::Type`)
    /// exactly like `item def`.
    fn class_def_specialization_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let demo_name = symbols.intern("Demo").unwrap();
        let base_name = symbols.intern("Base").unwrap();
        let derived_name = symbols.intern("Derived").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let base_path = paths.push(&[base_name], false).unwrap();

        let demo = DeclarationId(0);
        let derived = DeclarationId(2);
        let declarations = vec![
            declaration(
                DocumentId(0),
                None,
                Some(demo_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(base_name),
                DeclarationKind::ClassDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(derived_name),
                DeclarationKind::ClassDefinition,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let references = vec![TestReference {
            source: derived,
            kind: ReferenceKind::Subclassification,
            path: base_path,
            flags: RelationshipFlags::default(),
        }];
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn class_def_specialization_resolves_through_the_ancestor_fixed_point() {
        let fixture = class_def_specialization_fixture();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    /// Builds a `Demo { action def Base; action def Derived :> Base; }`-shaped fixture: `Derived`'s
    /// `:>` specialization reference exercises `action def`'s participation in the shared
    /// Subclassification/FeatureTyping lexical lookup fixed point (`DeclarationDomain::Type`)
    /// exactly like `item def`/`part def`/`port def`.
    fn action_def_specialization_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let demo_name = symbols.intern("Demo").unwrap();
        let base_name = symbols.intern("Base").unwrap();
        let derived_name = symbols.intern("Derived").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let base_path = paths.push(&[base_name], false).unwrap();

        let demo = DeclarationId(0);
        let derived = DeclarationId(2);
        let declarations = vec![
            declaration(
                DocumentId(0),
                None,
                Some(demo_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(base_name),
                DeclarationKind::ActionDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(derived_name),
                DeclarationKind::ActionDefinition,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let references = vec![TestReference {
            source: derived,
            kind: ReferenceKind::Subclassification,
            path: base_path,
            flags: RelationshipFlags::default(),
        }];
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn action_def_specialization_resolves_through_the_ancestor_fixed_point() {
        let fixture = action_def_specialization_fixture();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    /// Builds a `Demo { state def Base; state def Derived :> Base; }`-shaped fixture: `Derived`'s
    /// `:>` specialization reference exercises `state def`'s participation in the shared
    /// Subclassification/FeatureTyping lexical lookup fixed point (`DeclarationDomain::Type`)
    /// exactly like `action def`/`item def`/`part def`/`port def`.
    fn state_def_specialization_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let demo_name = symbols.intern("Demo").unwrap();
        let base_name = symbols.intern("Base").unwrap();
        let derived_name = symbols.intern("Derived").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let base_path = paths.push(&[base_name], false).unwrap();

        let demo = DeclarationId(0);
        let derived = DeclarationId(2);
        let declarations = vec![
            declaration(
                DocumentId(0),
                None,
                Some(demo_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(base_name),
                DeclarationKind::StateDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(derived_name),
                DeclarationKind::StateDefinition,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let references = vec![TestReference {
            source: derived,
            kind: ReferenceKind::Subclassification,
            path: base_path,
            flags: RelationshipFlags::default(),
        }];
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn state_def_specialization_resolves_through_the_ancestor_fixed_point() {
        let fixture = state_def_specialization_fixture();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    /// Builds a `Demo { metadata def Base; metadata def Derived :> Base; }`-shaped fixture:
    /// `Derived`'s `:>` specialization reference exercises `metadata def`'s participation in the
    /// shared Subclassification/FeatureTyping lexical lookup fixed point
    /// (`DeclarationDomain::Type`) exactly like `item def`/`action def`/`part def`/`port def`.
    fn metadata_def_specialization_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let demo_name = symbols.intern("Demo").unwrap();
        let base_name = symbols.intern("Base").unwrap();
        let derived_name = symbols.intern("Derived").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let base_path = paths.push(&[base_name], false).unwrap();

        let demo = DeclarationId(0);
        let derived = DeclarationId(2);
        let declarations = vec![
            declaration(
                DocumentId(0),
                None,
                Some(demo_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(base_name),
                DeclarationKind::MetadataDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(derived_name),
                DeclarationKind::MetadataDefinition,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let references = vec![TestReference {
            source: derived,
            kind: ReferenceKind::Subclassification,
            path: base_path,
            flags: RelationshipFlags::default(),
        }];
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn metadata_def_specialization_resolves_through_the_ancestor_fixed_point() {
        let fixture = metadata_def_specialization_fixture();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    /// Builds a `Demo { metadata def Safety; part seatBelt {@Safety;} }`-shaped fixture: the part
    /// usage's `@Safety` metadata annotation exercises `ReferenceKind::MetadataAnnotation`'s
    /// participation in the shared Subclassification/FeatureTyping lexical lookup fixed point
    /// (`DeclarationDomain::Type`), sourced directly at the part usage's own declaration (not an
    /// anonymous nested feature).
    fn metadata_annotation_fixture(target_name: &str) -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let demo_name = symbols.intern("Demo").unwrap();
        let safety_name = symbols.intern("Safety").unwrap();
        let seat_belt_name = symbols.intern("seatBelt").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let target_symbol = symbols.intern(target_name).unwrap();
        let target_path = paths.push(&[target_symbol], false).unwrap();

        let demo = DeclarationId(0);
        let seat_belt = DeclarationId(2);
        let declarations = vec![
            declaration(
                DocumentId(0),
                None,
                Some(demo_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(safety_name),
                DeclarationKind::MetadataDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(seat_belt_name),
                DeclarationKind::PartUsage,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let references = vec![TestReference {
            source: seat_belt,
            kind: ReferenceKind::MetadataAnnotation,
            path: target_path,
            flags: RelationshipFlags::default(),
        }];
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn metadata_annotation_on_part_usage_resolves_to_metadata_def() {
        let fixture = metadata_annotation_fixture("Safety");
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    #[test]
    fn metadata_annotation_with_unresolvable_target_stays_unresolved() {
        let fixture = metadata_annotation_fixture("NoSuchMetadata");
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Unresolved)
        );
    }

    /// Builds a `Demo { connection def Base; connection def Derived :> Base; }`-shaped fixture:
    /// `Derived`'s `:>` specialization reference exercises `connection def`'s participation in the
    /// shared Subclassification/FeatureTyping lexical lookup fixed point
    /// (`DeclarationDomain::Type`) exactly like `item def`/`action def`/`part def`/`port def`.
    fn connection_def_specialization_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let demo_name = symbols.intern("Demo").unwrap();
        let base_name = symbols.intern("Base").unwrap();
        let derived_name = symbols.intern("Derived").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let base_path = paths.push(&[base_name], false).unwrap();

        let demo = DeclarationId(0);
        let derived = DeclarationId(2);
        let declarations = vec![
            declaration(
                DocumentId(0),
                None,
                Some(demo_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(base_name),
                DeclarationKind::ConnectionDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(derived_name),
                DeclarationKind::ConnectionDefinition,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let references = vec![TestReference {
            source: derived,
            kind: ReferenceKind::Subclassification,
            path: base_path,
            flags: RelationshipFlags::default(),
        }];
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn connection_def_specialization_resolves_through_the_ancestor_fixed_point() {
        let fixture = connection_def_specialization_fixture();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    fn interface_def_specialization_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let demo_name = symbols.intern("Demo").unwrap();
        let base_name = symbols.intern("Base").unwrap();
        let derived_name = symbols.intern("Derived").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let base_path = paths.push(&[base_name], false).unwrap();

        let demo = DeclarationId(0);
        let derived = DeclarationId(2);
        let declarations = vec![
            declaration(
                DocumentId(0),
                None,
                Some(demo_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(base_name),
                DeclarationKind::InterfaceDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(derived_name),
                DeclarationKind::InterfaceDefinition,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let references = vec![TestReference {
            source: derived,
            kind: ReferenceKind::Subclassification,
            path: base_path,
            flags: RelationshipFlags::default(),
        }];
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn interface_def_specialization_resolves_through_the_ancestor_fixed_point() {
        let fixture = interface_def_specialization_fixture();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    /// Builds a `Demo { part d1; connection bus connect d1 to d1; }`-shaped fixture: `bus`'s
    /// `ConnectorEnd` reference exercises the `DeclarationDomain::Any` resolution slot
    /// (`connector_end_slots`) exactly like `AliasBinding` -- a connector end can reference any
    /// feature, not just a Type, so it must not join the Subclassification/FeatureTyping `Type`
    /// domain passes.
    fn connector_end_reference_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let demo_name = symbols.intern("Demo").unwrap();
        let d1_name = symbols.intern("d1").unwrap();
        let bus_name = symbols.intern("bus").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let d1_path = paths.push(&[d1_name], false).unwrap();

        let demo = DeclarationId(0);
        let bus = DeclarationId(2);
        let declarations = vec![
            declaration(
                DocumentId(0),
                None,
                Some(demo_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(d1_name),
                DeclarationKind::PartUsage,
            ),
            declaration(
                DocumentId(0),
                Some(demo),
                Some(bus_name),
                DeclarationKind::ConnectionUsage,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let references = vec![TestReference {
            source: bus,
            kind: ReferenceKind::ConnectorEnd,
            path: d1_path,
            flags: RelationshipFlags::default(),
        }];
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn connector_end_reference_resolves_to_its_target() {
        let fixture = connector_end_reference_fixture();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    #[test]
    fn namespace_import_populates_index_used_by_feature_typing() {
        let fixture = cross_file_fixture(false);
        let (direct_names, effective_imports, _memberships, resolution) = resolve_fixture(&fixture);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(0)))
        );
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(1)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
        assert_eq!(
            direct_names.candidates(Some(DeclarationId(0)), SymbolId(2)),
            &[DeclarationId(1)]
        );
        assert_eq!(
            effective_imports.candidates(Some(DeclarationId(2)), SymbolId(2)),
            &[DeclarationId(1)]
        );
        assert_eq!(fixture.paths.paths.len(), 2);
        assert_eq!(
            fixture.paths.get(SymbolPathId(0)),
            Some((&[SymbolId(0)][..], false))
        );
        assert_eq!(resolution.outcomes.len(), fixture.references.len());
    }

    #[test]
    fn default_visibility_is_settled_once_from_membership_context() {
        let declarations = [
            declaration(
                DocumentId(0),
                None,
                Some(SymbolId(0)),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentId(0),
                Some(DeclarationId(0)),
                Some(SymbolId(1)),
                DeclarationKind::Namespace,
            ),
            declaration(
                DocumentId(0),
                Some(DeclarationId(1)),
                Some(SymbolId(2)),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(DeclarationId(0)),
                None,
                DeclarationKind::Import,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let index = MembershipIndex::build(&declarations, &memberships).unwrap();

        assert!(index.is_public(DeclarationId(0)));
        assert!(index.is_public(DeclarationId(1)));
        assert!(!index.is_public(DeclarationId(2)));
        assert!(!index.is_public(DeclarationId(3)));
    }

    #[test]
    fn namespace_import_excludes_explicitly_private_members() {
        let mut fixture = cross_file_fixture(false);
        fixture.memberships[1].visibility = Visibility::Private;
        let (_, effective_imports, _memberships, resolution) = resolve_fixture(&fixture);

        assert!(effective_imports
            .candidates(Some(DeclarationId(2)), SymbolId(2))
            .is_empty());
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(1)),
            Some(ResolutionStatus::Unresolved)
        );
    }

    #[test]
    fn duplicate_imported_type_is_canonically_ambiguous() {
        let fixture = cross_file_fixture(true);
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        let Some(ResolutionStatus::Ambiguous(range)) = resolution.outcome(AuthoredReferenceId(1))
        else {
            panic!("feature typing must retain ambiguity");
        };
        assert_eq!(
            resolution.ambiguous_candidates(range),
            &[DeclarationId(1), DeclarationId(5)]
        );
    }

    #[test]
    fn transitive_namespace_imports_converge_without_reference_scans() {
        let fixture = transitive_import_fixture();
        let (_, effective_imports, _memberships, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(resolution.work.passes, 4);
        assert_eq!(resolution.work.import_evaluations, 12);
        assert_eq!(resolution.work.downstream_evaluations, 1);
        assert_eq!(resolution.work.direct_index_entries, 6);
        assert_eq!(resolution.work.effective_index_entries, 3);
        assert_eq!(
            effective_imports.candidates(Some(DeclarationId(6)), SymbolId(4)),
            &[DeclarationId(1)]
        );
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(3)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    #[test]
    fn an_import_target_can_become_visible_through_an_earlier_import() {
        let fixture = imported_target_fixture();
        let (_, effective_imports, _memberships, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(1)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
        assert_eq!(
            effective_imports.candidates(Some(DeclarationId(3)), SymbolId(2)),
            &[DeclarationId(2)]
        );
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(2)),
            Some(ResolutionStatus::Resolved(DeclarationId(2)))
        );
    }

    #[test]
    fn cyclic_namespace_imports_reach_a_finite_canonical_closure() {
        let fixture = cyclic_import_fixture();
        let (_, effective_imports, _memberships, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(resolution.work.passes, 3);
        assert_eq!(
            effective_imports.candidates(Some(DeclarationId(0)), SymbolId(3)),
            &[DeclarationId(5)]
        );
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(2)),
            Some(ResolutionStatus::Resolved(DeclarationId(5)))
        );
    }

    #[test]
    fn later_qualified_segments_use_the_effective_import_index() {
        let fixture = qualified_import_target_fixture();
        let (_, effective_imports, _memberships, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(1)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
        assert_eq!(
            effective_imports.candidates(Some(DeclarationId(5)), SymbolId(2)),
            &[DeclarationId(2)]
        );
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(2)),
            Some(ResolutionStatus::Resolved(DeclarationId(2)))
        );
    }

    #[test]
    fn exhausted_bound_is_a_typed_non_converged_publication_state() {
        let fixture = cross_file_fixture(false);
        let (_, _, _memberships, resolution) = resolve_dense_with_limit(
            &fixture.declarations,
            &fixture.memberships,
            &fixture.paths,
            &fixture.references,
            1,
            None,
        )
        .unwrap();
        assert_eq!(resolution.solver_status, SolverStatus::NonConverged);
        assert_eq!(resolution.work.passes, 1);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::NonConverged)
        );
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(1)),
            Some(ResolutionStatus::NonConverged)
        );
    }

    #[test]
    fn missing_and_filtered_references_remain_explicit() {
        let mut fixture = cross_file_fixture(false);
        fixture.references[0].kind = ReferenceKind::FilterImport;
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Unsupported)
        );
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(1)),
            Some(ResolutionStatus::Unresolved)
        );
    }

    /// Builds `package Diamond { part def Base { part def Member; } part def Left :> Base;
    /// part def Right :> Base; part def Diamond :> Left, Right { part <feature_name> : <typed>; } }`.
    /// `feature_name`/`typed` let the ambiguous-diamond test override the leaf feature and its
    /// authored typing target while sharing the rest of the diamond shape.
    fn diamond_fixture(feature_name: &str, typed: &str) -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let diamond_pkg = symbols.intern("Diamond").unwrap();
        let base_name = symbols.intern("Base").unwrap();
        let member_name = symbols.intern("Member").unwrap();
        let left_name = symbols.intern("Left").unwrap();
        let right_name = symbols.intern("Right").unwrap();
        let diamond_name = symbols.intern("Diamond").unwrap();
        let feature = symbols.intern(feature_name).unwrap();
        let typed_name = symbols.intern(typed).unwrap();

        let mut paths = SymbolPathArenaBuilder::default();
        let base_path = paths.push(&[base_name], false).unwrap();
        let left_path = paths.push(&[left_name], false).unwrap();
        let right_path = paths.push(&[right_name], false).unwrap();
        let typed_path = paths.push(&[typed_name], false).unwrap();

        let package = DeclarationId(0);
        let base = DeclarationId(1);
        let left = DeclarationId(3);
        let right = DeclarationId(4);
        let diamond = DeclarationId(5);
        let declarations = vec![
            declaration(
                DocumentId(0),
                None,
                Some(diamond_pkg),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentId(0),
                Some(package),
                Some(base_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(base),
                Some(member_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(package),
                Some(left_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(package),
                Some(right_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(package),
                Some(diamond_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(diamond),
                Some(feature),
                DeclarationKind::PartUsage,
            ),
        ];
        let references = vec![
            reference(left, ReferenceKind::Subclassification, base_path, false),
            reference(right, ReferenceKind::Subclassification, base_path, false),
            reference(diamond, ReferenceKind::Subclassification, left_path, false),
            reference(diamond, ReferenceKind::Subclassification, right_path, false),
            reference(
                DeclarationId(6),
                ReferenceKind::FeatureTyping,
                typed_path,
                false,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn diamond_inherited_member_lookup_dedups_to_a_single_target() {
        let fixture = diamond_fixture("p", "Member");
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        // Member is owned only by Base (id 2), reached via both Left -> Base and Right -> Base;
        // the diamond must dedup to exactly one Resolved outcome rather than an Ambiguous one.
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(4)),
            Some(ResolutionStatus::Resolved(DeclarationId(2)))
        );
    }

    #[test]
    fn single_ancestor_inherited_lookup_resolves_through_one_specialization_hop() {
        // A minimal non-diamond case: Diamond specializes only Left (drop the Right edge by
        // reusing the diamond fixture's Left -> Base -> Member chain through a direct
        // single-parent shape) still exercises the same inherited-lookup path.
        let mut fixture = diamond_fixture("p", "Member");
        // Remove the `Diamond :> Right` edge (reference index 3) and the `Right :> Base` edge
        // (reference index 1) so only one specialization hop feeds the closure.
        let mut references = fixture.references.into_vec();
        references.remove(3);
        references.remove(1);
        fixture.references = references.into_boxed_slice();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(2)),
            Some(ResolutionStatus::Resolved(DeclarationId(2)))
        );
    }

    /// `package P { part def Left { part def Special; } part def Right { part def Special; }
    /// part def Diamond :> Left, Right { part q : Special; } }`. Left and Right each directly own
    /// their own distinct `Special` member (no `Base`), so the diamond conflict is genuine: two
    /// different ancestors reach two different same-named targets, not one target through two
    /// paths.
    fn diamond_conflict_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let package_name = symbols.intern("P").unwrap();
        let left_name = symbols.intern("Left").unwrap();
        let right_name = symbols.intern("Right").unwrap();
        let diamond_name = symbols.intern("Diamond").unwrap();
        let special_name = symbols.intern("Special").unwrap();
        let q_name = symbols.intern("q").unwrap();

        let mut paths = SymbolPathArenaBuilder::default();
        let left_path = paths.push(&[left_name], false).unwrap();
        let right_path = paths.push(&[right_name], false).unwrap();
        let special_path = paths.push(&[special_name], false).unwrap();

        let package = DeclarationId(0);
        let left = DeclarationId(1);
        let right = DeclarationId(3);
        let diamond = DeclarationId(5);
        let declarations = vec![
            declaration(
                DocumentId(0),
                None,
                Some(package_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentId(0),
                Some(package),
                Some(left_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(left),
                Some(special_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(package),
                Some(right_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(right),
                Some(special_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(package),
                Some(diamond_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(diamond),
                Some(q_name),
                DeclarationKind::PartUsage,
            ),
        ];
        let references = vec![
            reference(diamond, ReferenceKind::Subclassification, left_path, false),
            reference(diamond, ReferenceKind::Subclassification, right_path, false),
            reference(
                DeclarationId(6),
                ReferenceKind::FeatureTyping,
                special_path,
                false,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn diamond_with_conflicting_ancestor_members_publishes_an_explicit_ambiguous_outcome() {
        let fixture = diamond_conflict_fixture();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        match resolution.outcome(AuthoredReferenceId(2)) {
            Some(ResolutionStatus::Ambiguous(range)) => {
                let mut candidates = resolution.ambiguous_candidates(range).to_vec();
                candidates.sort_by_key(|id| id.0);
                assert_eq!(candidates, vec![DeclarationId(2), DeclarationId(4)]);
            }
            other => panic!("expected an explicit ambiguous outcome, got {other:?}"),
        }
    }

    fn cyclic_specialization_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let package_name = symbols.intern("P").unwrap();
        let a_name = symbols.intern("A").unwrap();
        let b_name = symbols.intern("B").unwrap();
        let f_name = symbols.intern("f").unwrap();
        let x_name = symbols.intern("X").unwrap();

        let mut paths = SymbolPathArenaBuilder::default();
        let a_path = paths.push(&[a_name], false).unwrap();
        let b_path = paths.push(&[b_name], false).unwrap();
        let x_path = paths.push(&[x_name], false).unwrap();

        let package = DeclarationId(0);
        let a = DeclarationId(1);
        let b = DeclarationId(2);
        let declarations = vec![
            declaration(
                DocumentId(0),
                None,
                Some(package_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentId(0),
                Some(package),
                Some(a_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(package),
                Some(b_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(a),
                Some(f_name),
                DeclarationKind::PartUsage,
            ),
        ];
        let references = vec![
            reference(a, ReferenceKind::Subclassification, b_path, false),
            reference(b, ReferenceKind::Subclassification, a_path, false),
            reference(
                DeclarationId(3),
                ReferenceKind::FeatureTyping,
                x_path,
                false,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn cyclic_specialization_yields_a_typed_non_converged_typing_outcome_not_a_loop() {
        let fixture = cyclic_specialization_fixture();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        // The import/typing fixed point above this family still converges; only the
        // ancestor-closure-dependent FeatureTyping outcome for the cyclically-specialized owner
        // is explicitly NonConverged, rather than the solver looping forever or silently guessing
        // an inherited candidate through the self-referential closure.
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(2)),
            Some(ResolutionStatus::NonConverged)
        );
    }

    /// `package P { part def Device; alias DeviceAlias for Device; part device : DeviceAlias; }`
    /// — mirrors `tests/snapshots/resolution/alias_target_binding.md`.
    fn alias_binding_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let package_name = symbols.intern("P").unwrap();
        let device_name = symbols.intern("Device").unwrap();
        let alias_name = symbols.intern("DeviceAlias").unwrap();
        let device_usage_name = symbols.intern("device").unwrap();

        let mut paths = SymbolPathArenaBuilder::default();
        let device_path = paths.push(&[device_name], false).unwrap();
        let alias_path = paths.push(&[alias_name], false).unwrap();

        let package = DeclarationId(0);
        let alias = DeclarationId(2);
        let device_usage = DeclarationId(3);
        let declarations = vec![
            declaration(
                DocumentId(0),
                None,
                Some(package_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentId(0),
                Some(package),
                Some(device_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(
                DocumentId(0),
                Some(package),
                Some(alias_name),
                DeclarationKind::Alias,
            ),
            declaration(
                DocumentId(0),
                Some(package),
                Some(device_usage_name),
                DeclarationKind::PartUsage,
            ),
        ];
        let references = vec![
            reference(alias, ReferenceKind::AliasBinding, device_path, false),
            reference(
                device_usage,
                ReferenceKind::FeatureTyping,
                alias_path,
                false,
            ),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn alias_binding_resolves_through_the_shared_lexical_lookup_fixed_point() {
        let fixture = alias_binding_fixture();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        // DeviceAlias's own authored `AliasBinding` reference resolves to Device (id 1), using
        // the same fixed point as every other authored reference kind rather than a separate
        // ad hoc path.
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    #[test]
    fn typing_through_an_alias_resolves_transitively_to_the_ultimate_target() {
        let fixture = alias_binding_fixture();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        // `device : DeviceAlias`'s own FeatureTyping outcome targets the alias declaration (id 2)
        // itself -- the alias's own authored fact is never weakened or bypassed.
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(1)),
            Some(ResolutionStatus::Resolved(DeclarationId(2)))
        );
        // Downstream typing is nonetheless transparent: an implied FeatureTyping fact chases the
        // alias chain to publish device -> Device directly, with implied provenance.
        assert_eq!(
            resolution.implied_relationships.as_ref(),
            &[ImpliedRelationship {
                kind: ReferenceKind::FeatureTyping,
                source: DeclarationId(3),
                target: DeclarationId(1),
            }],
        );
    }

    /// `package P { alias A for B; alias B for A; }` — a two-hop alias cycle, mirroring the
    /// specialization-cycle shape of `cyclic_specialization_fixture` above.
    fn cyclic_alias_binding_fixture() -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let package_name = symbols.intern("P").unwrap();
        let a_name = symbols.intern("A").unwrap();
        let b_name = symbols.intern("B").unwrap();

        let mut paths = SymbolPathArenaBuilder::default();
        let a_path = paths.push(&[a_name], false).unwrap();
        let b_path = paths.push(&[b_name], false).unwrap();

        let package = DeclarationId(0);
        let a = DeclarationId(1);
        let b = DeclarationId(2);
        let declarations = vec![
            declaration(
                DocumentId(0),
                None,
                Some(package_name),
                DeclarationKind::Package,
            ),
            declaration(
                DocumentId(0),
                Some(package),
                Some(a_name),
                DeclarationKind::Alias,
            ),
            declaration(
                DocumentId(0),
                Some(package),
                Some(b_name),
                DeclarationKind::Alias,
            ),
        ];
        let references = vec![
            reference(a, ReferenceKind::AliasBinding, b_path, false),
            reference(b, ReferenceKind::AliasBinding, a_path, false),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn alias_cycle_yields_a_typed_non_converged_outcome_not_a_hang() {
        // Bounded by `detect_cyclic_alias_bindings`'s `declarations.len() + 1` hop limit: this
        // test would time out (rather than merely fail an assertion) if alias cycle detection
        // ever degenerated into an unbounded chase.
        let fixture = cyclic_alias_binding_fixture();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::NonConverged)
        );
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(1)),
            Some(ResolutionStatus::NonConverged)
        );
        assert!(resolution.implied_relationships.is_empty());
    }

    /// `package A { part def T; } package C { import A::*; part T; part p : T; }` — mirrors
    /// `tests/snapshots/resolution/lexical_inner_shadow.md`. `nested` controls whether the
    /// FeatureTyping reference lives directly on `C::p` (false) or one namespace level deeper, on
    /// a feature owned by an intermediate `Inner` namespace inside `C` (true), so the same local
    /// binding is reached by walking one extra step of the enclosing-namespace chain.
    fn local_shadow_fixture(nested: bool) -> ResolverFixture {
        let mut symbols = SymbolTableBuilder::default();
        let a_name = symbols.intern("A").unwrap();
        let t_name = symbols.intern("T").unwrap();
        let c_name = symbols.intern("C").unwrap();
        let p_name = symbols.intern("p").unwrap();
        let inner_name = symbols.intern("Inner").unwrap();
        let mut paths = SymbolPathArenaBuilder::default();
        let a_path = paths.push(&[a_name], false).unwrap();
        let t_path = paths.push(&[t_name], false).unwrap();

        let a = DeclarationId(0);
        let c = DeclarationId(2);
        let mut declarations = vec![
            declaration(DocumentId(0), None, Some(a_name), DeclarationKind::Package),
            declaration(
                DocumentId(0),
                Some(a),
                Some(t_name),
                DeclarationKind::PartDefinition,
            ),
            declaration(DocumentId(0), None, Some(c_name), DeclarationKind::Package),
            declaration(DocumentId(0), Some(c), None, DeclarationKind::Import),
            declaration(
                DocumentId(0),
                Some(c),
                Some(t_name),
                DeclarationKind::PartUsage,
            ),
        ];
        let p_owner = if nested {
            let inner = DeclarationId(u32::try_from(declarations.len()).unwrap());
            declarations.push(declaration(
                DocumentId(0),
                Some(c),
                Some(inner_name),
                DeclarationKind::Namespace,
            ));
            inner
        } else {
            c
        };
        declarations.push(declaration(
            DocumentId(0),
            Some(p_owner),
            Some(p_name),
            DeclarationKind::PartUsage,
        ));
        let p = DeclarationId(u32::try_from(declarations.len() - 1).unwrap());
        let references = vec![
            reference(
                DeclarationId(3),
                ReferenceKind::NamespaceImport,
                a_path,
                true,
            ),
            reference(p, ReferenceKind::FeatureTyping, t_path, false),
        ];
        let memberships = memberships_for(&declarations, &[]);
        let _symbols = symbols.freeze();
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            memberships,
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    #[test]
    fn local_feature_shadows_an_incompatible_imported_type_of_the_same_name() {
        // C::T (a PartUsage feature) is domain-incompatible as a FeatureTyping target, but it is
        // still owned directly by C, the reference's enclosing namespace, so per
        // it must shadow the imported, domain-compatible
        // A::T rather than being silently discarded in favor of the import or left Unresolved.
        let fixture = local_shadow_fixture(false);
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(1)),
            Some(ResolutionStatus::Resolved(DeclarationId(4)))
        );
    }

    #[test]
    fn without_a_local_binding_lookup_still_falls_through_to_the_import() {
        // Regression guard: removing C::T (and its membership record) must not disturb the
        // fallback to the imported A::T once no local/inherited candidate exists at any tier.
        let mut fixture = local_shadow_fixture(false);
        let mut declarations = fixture.declarations.into_vec();
        declarations.remove(4);
        fixture.declarations = declarations.into_boxed_slice();
        // Rebuild memberships from scratch rather than splicing: `MembershipRecord::member` is a
        // `DeclarationId` into the just-mutated declarations array, so it must be recomputed
        // against the post-removal indices, not the pre-removal ones.
        fixture.memberships = memberships_for(&fixture.declarations, &[]);
        // The FeatureTyping reference source shifts down by one index after the removal.
        let mut references = fixture.references.into_vec();
        references[1].source = DeclarationId(4);
        fixture.references = references.into_boxed_slice();
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(1)),
            Some(ResolutionStatus::Resolved(DeclarationId(1)))
        );
    }

    #[test]
    fn intermediate_namespace_local_binding_shadows_the_outer_import() {
        // p lives inside C::Inner, one level below C itself. The local C::T binding is neither
        // owned by nor inherited into Inner directly, so the walk must climb the enclosing-scope
        // chain to C, find C::T there, and shadow A::T at that outer tier before ever consulting
        // imports.
        let fixture = local_shadow_fixture(true);
        let (_, _, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(resolution.solver_status, SolverStatus::Converged);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(1)),
            Some(ResolutionStatus::Resolved(DeclarationId(4)))
        );
    }

    #[test]
    fn candidate_ranges_are_canonical_regardless_of_input_order() {
        let index = NameIndex::build(vec![
            (
                NameKey {
                    owner: None,
                    name: SymbolId(0),
                },
                DeclarationId(2),
            ),
            (
                NameKey {
                    owner: None,
                    name: SymbolId(0),
                },
                DeclarationId(1),
            ),
        ])
        .unwrap();
        assert_eq!(
            index.candidates(None, SymbolId(0)),
            &[DeclarationId(1), DeclarationId(2)]
        );
    }

    #[test]
    fn packed_name_entry_order_matches_the_canonical_tuple_order() {
        let entries = vec![
            (
                NameKey {
                    owner: Some(DeclarationId(u32::MAX)),
                    name: SymbolId(u32::MAX),
                },
                DeclarationId(u32::MAX),
            ),
            (
                NameKey {
                    owner: Some(DeclarationId(0)),
                    name: SymbolId(u32::MAX),
                },
                DeclarationId(0),
            ),
            (
                NameKey {
                    owner: None,
                    name: SymbolId(u32::MAX),
                },
                DeclarationId(u32::MAX),
            ),
            (
                NameKey {
                    owner: Some(DeclarationId(0)),
                    name: SymbolId(0),
                },
                DeclarationId(u32::MAX),
            ),
            (
                NameKey {
                    owner: None,
                    name: SymbolId(0),
                },
                DeclarationId(0),
            ),
        ];
        let mut canonical = entries.clone();
        canonical.sort_unstable();
        let mut packed = entries;
        packed.sort_unstable_by_key(name_entry_sort_key);
        assert_eq!(packed, canonical);
    }
}
