//! Assembly of the published element-details answer.
//!
//! Every field is read from an index this publication settled before it became visible. Nothing
//! here traverses the model, resolves a name, evaluates an expression or infers a type: the
//! relationship families read the reference table through the per-declaration ranges, the
//! effective types and inherited features read the type index, and both evaluation channels read
//! the same evaluation fact `inspect` reads.

use super::*;
use crate::details::{
    ConnectedElement, EffectiveTypeEntry, EffectiveTyping, ElementDetails, ElementDetailsAt,
    InheritedFeature, ReferencedDetails, RelationshipFamily, RelationshipOutcome,
};
use crate::evaluation::{AnalysisEvaluation, EvaluatedScalar, EvaluationState};
use crate::inspection::SymbolEntry;
use crate::type_query::EffectiveTypeOrigin;
use crate::ElementKind;

/// The relationship families an element-details answer reports separately.
///
/// `Subsetting` deliberately spans `subsets`, `references` and `crosses`: KerML makes all three
/// `Subsetting`, and reporting them apart would make an inspector show three families where the
/// specification has one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Family {
    Typing,
    Specialization,
    Subsetting,
    Redefinition,
}

impl Family {
    fn accepts(self, kind: ReferenceKind) -> bool {
        match self {
            Self::Typing => kind == ReferenceKind::FeatureTyping,
            Self::Specialization => kind == ReferenceKind::Subclassification,
            Self::Subsetting => matches!(
                kind,
                ReferenceKind::Subsetting | ReferenceKind::References | ReferenceKind::Crosses
            ),
            Self::Redefinition => kind == ReferenceKind::Redefinition,
        }
    }
}

/// What one authored reference settled to, reduced to what a family summary needs.
enum ReferenceOutcome {
    Resolved(DeclarationId),
    Ambiguous(Vec<DeclarationId>),
    Unresolved,
    Unsupported,
}

/// Whether an element's kind makes its expression a verdict or a computed analysis result.
///
/// A closed set rather than a shape test. `constraint def` and `calc def` are excluded on purpose:
/// a definition states what would be evaluated, and publishing a verdict for it would report the
/// definition as passing or failing rather than its usages.
fn is_verdict_bearing(kind: ElementKind) -> bool {
    matches!(
        kind,
        ElementKind::AnalysisCaseDefinition
            | ElementKind::AnalysisCaseUsage
            | ElementKind::VerificationCaseDefinition
            | ElementKind::VerificationCaseUsage
            | ElementKind::RequirementDefinition
            | ElementKind::RequirementUsage
            | ElementKind::ConstraintUsage
            | ElementKind::AssertConstraintUsage
    )
}

impl ResolvedSemanticModel {
    pub(crate) fn element_details(&self, symbol: &SymbolIdentity) -> QueryOutcome<ElementDetails> {
        if matches!(
            self.metadata.completeness,
            PublicationCompleteness::NonConverged
        ) {
            return QueryOutcome::Incomplete;
        }
        let mut candidates = self.identity_declarations(symbol);
        if candidates.len() > 1 {
            let details = candidates
                .into_iter()
                .filter_map(|id| self.details(id))
                .collect::<Vec<_>>();
            return QueryOutcome::Ambiguous(details.into_boxed_slice());
        }
        let Some(id) = candidates.pop() else {
            return QueryOutcome::Unresolved;
        };
        match self.details(id) {
            Some(details) => self.resolved_outcome(details),
            None => QueryOutcome::Unresolved,
        }
    }

    pub(crate) fn element_details_at(
        &self,
        document: &str,
        position: TextPosition,
    ) -> QueryOutcome<ElementDetailsAt> {
        let Some(document_id) = self.documents.document(&self.storage, document) else {
            return QueryOutcome::Unresolved;
        };
        let Some(positions) = self.documents.positions(document_id) else {
            return QueryOutcome::Unresolved;
        };

        let containing = positions
            .spans
            .innermost_containing(position)
            .and_then(|id| self.details(id));

        let referenced = leaf_ranges_containing(&positions.references, position)
            .next()
            .map_or(ReferencedDetails::None, |reference_id| {
                match self.resolution.outcome(reference_id) {
                    Some(ResolutionStatus::Resolved(target)) => self
                        .details(target)
                        .map_or(ReferencedDetails::Unresolved, |details| {
                            ReferencedDetails::Resolved(Box::new(details))
                        }),
                    Some(ResolutionStatus::Ambiguous(candidates)) => ReferencedDetails::Ambiguous(
                        self.resolution
                            .ambiguous_candidates(candidates)
                            .iter()
                            .filter_map(|candidate| self.details(*candidate))
                            .collect(),
                    ),
                    Some(ResolutionStatus::Unsupported) => ReferencedDetails::Unsupported,
                    Some(ResolutionStatus::NonConverged) => ReferencedDetails::Incomplete,
                    Some(ResolutionStatus::Unresolved) | None => ReferencedDetails::Unresolved,
                }
            });

        self.resolved_outcome(ElementDetailsAt {
            containing,
            referenced,
        })
    }

    fn details(&self, id: DeclarationId) -> Option<ElementDetails> {
        let inspection = self.inspection(id)?;
        let declaration = self.storage.declaration(id)?;
        let typing = self.family(id, Family::Typing);
        let specialization = self.family(id, Family::Specialization);
        let subsetting = self.family(id, Family::Subsetting);
        let redefinition = self.family(id, Family::Redefinition);
        let effective_typing = self.effective_typing(id, &typing, &subsetting, &redefinition);
        let evaluation = self.element_evaluation(id)?;
        Some(ElementDetails {
            owner: declaration.owner.and_then(|owner| self.symbol_entry(owner)),
            analysis: self.analysis_evaluation(id, &evaluation.state),
            evaluation,
            inherited_features: self.inherited_features(id),
            metadata: self.metadata_annotations(id),
            incoming: self.incoming_relationships(id),
            outgoing: self.outgoing_relationships(id),
            typing,
            effective_typing,
            specialization,
            subsetting,
            redefinition,
            inspection,
        })
    }

    /// The authored references of one family, reduced to a single closed outcome.
    ///
    /// Implied references are excluded: this answers *what the author declared*, and a resolver-
    /// synthesized redefinition would otherwise make every usage report a redefinition nobody
    /// wrote. The implied ones remain visible through
    /// [`crate::details::ElementDetails::outgoing`], with their provenance intact.
    fn family(&self, id: DeclarationId, family: Family) -> RelationshipFamily {
        let mut targets = Vec::new();
        let mut candidates = Vec::new();
        let mut authored = 0usize;
        let mut resolved = 0usize;
        let mut ambiguous = false;
        let mut unsupported = false;

        for reference_id in self.outgoing_reference_ids(id) {
            let reference = &self.storage.references[reference_id.index()];
            if reference.flags.implied || !family.accepts(reference.kind) {
                continue;
            }
            authored += 1;
            match self.reference_outcome(*reference_id) {
                ReferenceOutcome::Resolved(target) => {
                    resolved += 1;
                    targets.push(target);
                }
                ReferenceOutcome::Ambiguous(found) => {
                    ambiguous = true;
                    candidates.extend(found);
                }
                ReferenceOutcome::Unsupported => unsupported = true,
                ReferenceOutcome::Unresolved => {}
            }
        }

        let outcome = if authored == 0 {
            RelationshipOutcome::NotApplicable
        } else if ambiguous {
            RelationshipOutcome::Ambiguous
        } else if unsupported {
            RelationshipOutcome::Unsupported
        } else if resolved == authored {
            RelationshipOutcome::Resolved
        } else if resolved == 0 {
            RelationshipOutcome::Unresolved
        } else {
            RelationshipOutcome::Partial
        };
        RelationshipFamily {
            outcome,
            targets: self.entries(targets),
            candidates: self.entries(candidates),
        }
    }

    /// The types the element has once inherited typing is taken into account.
    ///
    /// The outcome is a fact about the declaration rather than about the list. When the type index
    /// settled nothing, the answer is the least settled of the declared typing and the feature
    /// specializations it would have inherited along -- so "declares nothing" and "declared a
    /// typing that did not resolve" stay apart.
    fn effective_typing(
        &self,
        id: DeclarationId,
        typing: &RelationshipFamily,
        subsetting: &RelationshipFamily,
        redefinition: &RelationshipFamily,
    ) -> EffectiveTyping {
        let mut types = self
            .types
            .effective_types(id)
            .iter()
            .filter_map(|(target, source)| {
                Some(EffectiveTypeEntry {
                    element: self.symbol_entry(*target)?,
                    origin: match source {
                        types::EffectiveTypeSource::Direct => EffectiveTypeOrigin::Direct,
                        types::EffectiveTypeSource::Inherited(from) => {
                            EffectiveTypeOrigin::Inherited(self.symbol_identity(*from)?)
                        }
                    },
                })
            })
            .collect::<Vec<_>>();
        types.sort_by(|left, right| left.element.identity.cmp(&right.element.identity));
        types.dedup_by(|left, right| left.element.identity == right.element.identity);

        let outcome = if !types.is_empty() {
            RelationshipOutcome::Resolved
        } else {
            [typing.outcome, subsetting.outcome, redefinition.outcome]
                .into_iter()
                .filter(|outcome| *outcome != RelationshipOutcome::NotApplicable)
                .max()
                .unwrap_or(RelationshipOutcome::NotApplicable)
        };
        EffectiveTyping {
            outcome,
            types: types.into_boxed_slice(),
        }
    }

    /// Features the element has without declaring them, with the type that declares each.
    ///
    /// Starts from what the element is typed by and what it directly specializes, then walks the
    /// specialization edges of each owner. A name the element declares itself, or that a nearer
    /// owner already contributed, shadows the further one -- so a redefinition suppresses the
    /// feature it redefines rather than listing both.
    fn inherited_features(&self, id: DeclarationId) -> Box<[InheritedFeature]> {
        let mut queue = std::collections::VecDeque::new();
        let mut seed = self
            .types
            .direct_types(id)
            .iter()
            .map(|(target, _)| *target)
            .chain(
                self.types
                    .supertypes(id)
                    .iter()
                    .filter(|(_, scopes)| {
                        types::scopes_of(*scopes)
                            .any(|scope| scope == types::SpecializationScope::Subclassification)
                    })
                    .map(|(target, _)| *target),
            )
            .collect::<Vec<_>>();
        seed.sort_by_key(|target| self.symbol_identity(*target));
        seed.dedup();
        queue.extend(seed);

        // A feature the element declares shadows an inherited one of the same name, and a feature
        // it redefines is shadowed whatever either is called: KerML makes a redefinition *replace*
        // the redefined feature, and an anonymous `part :>> wheel[4];` carries no name of its own
        // to collide with.
        let mut shadowed = std::collections::BTreeSet::new();
        let mut redefined = std::collections::BTreeSet::new();
        for child in self.child_declarations(id) {
            if let Some(name) = self.declaration_name(*child) {
                shadowed.insert(name);
            }
            for reference_id in self.outgoing_reference_ids(*child) {
                if self.storage.references[reference_id.index()].kind != ReferenceKind::Redefinition
                {
                    continue;
                }
                if let ReferenceOutcome::Resolved(target) = self.reference_outcome(*reference_id) {
                    redefined.insert(target);
                }
            }
        }
        let mut visited = std::collections::BTreeSet::new();
        let mut inherited = Vec::new();
        while let Some(owner) = queue.pop_front() {
            if !visited.insert(owner) {
                continue;
            }
            let Some(declared_in) = self.symbol_entry(owner) else {
                continue;
            };
            for child in self.child_declarations(owner) {
                if self.types.featuring_type(*child) != Some(owner) || redefined.contains(child) {
                    continue;
                }
                let Some(name) = self.declaration_name(*child) else {
                    continue;
                };
                if !shadowed.insert(name) {
                    continue;
                }
                let Some(feature) = self.symbol_entry(*child) else {
                    continue;
                };
                inherited.push(InheritedFeature {
                    feature,
                    declared_in: declared_in.clone(),
                });
            }
            let mut bases = self
                .types
                .supertypes(owner)
                .iter()
                .filter(|(_, scopes)| {
                    types::scopes_of(*scopes)
                        .any(|scope| scope == types::SpecializationScope::Subclassification)
                })
                .map(|(target, _)| *target)
                .collect::<Vec<_>>();
            bases.sort_by_key(|target| self.symbol_identity(*target));
            queue.extend(bases);
        }
        inherited.into_boxed_slice()
    }

    /// The metadata definitions annotating this element, in canonical order.
    ///
    /// The annotation reference is authored *on* the annotated element and names the metadata
    /// definition, so this is an outgoing binding. Only settled targets appear; an annotation whose
    /// name did not resolve stays visible as an unresolved relationship on the inspection rather
    /// than as a metadata entry that would claim the element carries something it does not.
    fn metadata_annotations(&self, id: DeclarationId) -> Box<[SymbolEntry]> {
        let targets = self
            .outgoing_reference_ids(id)
            .iter()
            .filter(|reference_id| {
                self.storage.references[reference_id.index()].kind
                    == ReferenceKind::MetadataAnnotation
            })
            .filter_map(|reference_id| match self.reference_outcome(*reference_id) {
                ReferenceOutcome::Resolved(target) => Some(target),
                _ => None,
            })
            .collect::<Vec<_>>();
        self.entries(targets)
    }

    fn incoming_relationships(&self, id: DeclarationId) -> Box<[ConnectedElement]> {
        let mut connected = Vec::new();
        for reference_id in self.reverse_references.references(id) {
            let reference = &self.storage.references[reference_id.index()];
            let Some(kind) = writer::relationship_kind(reference.kind) else {
                continue;
            };
            let Some(peer) = self.symbol_entry(reference.source) else {
                continue;
            };
            connected.push(ConnectedElement {
                kind,
                provenance: if reference.flags.implied {
                    RelationshipProvenance::Implied
                } else {
                    RelationshipProvenance::Authored
                },
                peer,
                location: self.reference_location(reference),
            });
        }
        for index in self.incoming_implied_indices(id) {
            let implied = &self.resolution.implied_relationships[*index as usize];
            let Some(kind) = writer::relationship_kind(implied.kind) else {
                continue;
            };
            let Some(peer) = self.symbol_entry(implied.source) else {
                continue;
            };
            connected.push(ConnectedElement {
                kind,
                provenance: RelationshipProvenance::Implied,
                peer,
                location: None,
            });
        }
        canonicalize(connected)
    }

    fn outgoing_relationships(&self, id: DeclarationId) -> Box<[ConnectedElement]> {
        let mut connected = Vec::new();
        for reference_id in self.outgoing_reference_ids(id) {
            let reference = &self.storage.references[reference_id.index()];
            let Some(kind) = writer::relationship_kind(reference.kind) else {
                continue;
            };
            let ReferenceOutcome::Resolved(target) = self.reference_outcome(*reference_id) else {
                continue;
            };
            let Some(peer) = self.symbol_entry(target) else {
                continue;
            };
            connected.push(ConnectedElement {
                kind,
                provenance: if reference.flags.implied {
                    RelationshipProvenance::Implied
                } else {
                    RelationshipProvenance::Authored
                },
                peer,
                location: self.reference_location(reference),
            });
        }
        for index in self.outgoing_implied_indices(id) {
            let implied = &self.resolution.implied_relationships[*index as usize];
            let Some(kind) = writer::relationship_kind(implied.kind) else {
                continue;
            };
            let Some(peer) = self.symbol_entry(implied.target) else {
                continue;
            };
            connected.push(ConnectedElement {
                kind,
                provenance: RelationshipProvenance::Implied,
                peer,
                location: None,
            });
        }
        canonicalize(connected)
    }

    /// The verdict channel of one element, read from the same settled state its value channel
    /// publishes, so the two can never disagree.
    fn analysis_evaluation(
        &self,
        id: DeclarationId,
        state: &EvaluationState,
    ) -> AnalysisEvaluation {
        let kind = self
            .storage
            .declaration(id)
            .map(|declaration| element_kind::element_kind(declaration.kind));
        if !kind.is_some_and(is_verdict_bearing) {
            return AnalysisEvaluation::NotApplicable;
        }
        match state {
            EvaluationState::NotApplicable => AnalysisEvaluation::NotApplicable,
            EvaluationState::NotRun => AnalysisEvaluation::NotRun,
            EvaluationState::Literal(value) | EvaluationState::Evaluated(value) => match value {
                EvaluatedScalar::Boolean(passed) => AnalysisEvaluation::Verdict(*passed),
                other => AnalysisEvaluation::Computed(other.clone()),
            },
            other => AnalysisEvaluation::Unsettled(other.clone()),
        }
    }

    fn reference_outcome(&self, reference_id: AuthoredReferenceId) -> ReferenceOutcome {
        match self.resolution.outcome(reference_id) {
            Some(ResolutionStatus::Resolved(target)) => ReferenceOutcome::Resolved(target),
            Some(ResolutionStatus::Ambiguous(candidates)) => ReferenceOutcome::Ambiguous(
                self.resolution.ambiguous_candidates(candidates).to_vec(),
            ),
            Some(ResolutionStatus::Unsupported) => ReferenceOutcome::Unsupported,
            Some(ResolutionStatus::Unresolved) | Some(ResolutionStatus::NonConverged) | None => {
                ReferenceOutcome::Unresolved
            }
        }
    }

    fn reference_location(&self, reference: &AuthoredReference) -> Option<SourceLocation> {
        let source = self.storage.declaration(reference.source)?;
        let document = self.storage.document(source.document)?.identity.clone();
        let range = document_range(&self.storage, source.document, &reference.span).ok()?;
        Some(SourceLocation {
            document,
            range,
            role: OccurrenceRole::Reference,
        })
    }

    fn declaration_name(&self, id: DeclarationId) -> Option<Box<str>> {
        self.storage
            .declaration(id)
            .and_then(|declaration| declaration.name)
            .and_then(|name| self.storage.symbol(name))
            .filter(|name| !name.trim().is_empty())
            .map(Into::into)
    }

    /// Distinct symbol entries in canonical identity order.
    fn entries(&self, declarations: Vec<DeclarationId>) -> Box<[SymbolEntry]> {
        let mut entries = declarations
            .into_iter()
            .filter_map(|id| self.symbol_entry(id))
            .collect::<Vec<_>>();
        entries.sort_by(|left, right| left.identity.cmp(&right.identity));
        entries.dedup_by(|left, right| left.identity == right.identity);
        entries.into_boxed_slice()
    }
}

/// One canonical order for a relationship list, independent of the order the tables were built in.
fn canonicalize(mut connected: Vec<ConnectedElement>) -> Box<[ConnectedElement]> {
    connected.sort_by(|left, right| {
        left.kind
            .cmp(right.kind)
            .then_with(|| left.peer.identity.cmp(&right.peer.identity))
            .then_with(|| left.provenance.cmp(&right.provenance))
    });
    connected.dedup_by(|left, right| {
        left.kind == right.kind
            && left.peer.identity == right.peer.identity
            && left.provenance == right.provenance
    });
    connected.into_boxed_slice()
}
