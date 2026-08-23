//! Assembly of the published element-inspection answer.
//!
//! Every field is read from a typed fact this crate already owns. Nothing is recovered by
//! re-reading authored text, and nothing is defaulted: a fact the parser cannot express is absent,
//! and a fact resolution could not settle keeps its own outcome.

use crate::diagnose::document_range;
use crate::evaluate::EvaluationFact;
use crate::index::documents::leaf_ranges_containing;
use crate::index::documents::record_visited_index_entries;
use crate::index::types;
use crate::lower::facts::ParameterDirection;
use crate::lower::storage::SemanticModelStorage;
use crate::model::element_kind;
use crate::model::render as writer;
use crate::model::resolver::SemanticModel;
use crate::model::AuthoredReferenceId;
use crate::model::DeclarationId;
use crate::model::DocumentId;
use crate::model::ReferenceKind;
use crate::model::SymbolPathId;
use crate::resolve::names::EffectiveVisibility;
use crate::resolve::results::ResolutionError;
use crate::resolve::results::ResolutionResults;
use crate::resolve::results::ResolutionStatus;
use crate::ElementSearch;
use crate::ElementSource;
use crate::EvaluationState;
use crate::OccurrenceRole;
use crate::QueryOutcome;
use crate::SourceLocation;
use crate::SymbolId;
use crate::TextPosition;
use source_identity::SourceRole;

use crate::inspection::{
    AnnotationForm, AuthoredValue, Documentation, ElementInspection, ElementInspectionAt,
    ElementModifier, ElementRelationship, FeatureDirection, MembershipFacts, MembershipKind,
    MultiplicityBound, MultiplicityFacts, PortionKind, ReferenceAt, RelationshipProvenance,
    RelationshipTarget, SymbolEntry, ValueKind, Visibility, VisibilityProvenance,
};

/// Per-declaration ranges into the record tables, so a per-element question never scans them all.
///
/// Each table is sorted by declaration at publication time, and the ranges are the contiguous run
/// belonging to each declaration -- the same shape the sibling compiler's CSR edge index uses.
#[derive(Debug, Default)]
pub(crate) struct ElementFactIndex {
    /// Per-declaration ranges into `documentation_order`, not into the storage table directly:
    /// the ranges are computed for the *ordered* view, so slicing anything else would rest on the
    /// lowering happening to emit records already grouped by declaration.
    pub(crate) documentation: Box<[(u32, u32)]>,
    pub(crate) documentation_order: Box<[u32]>,
    pub(crate) feature_values: Box<[(u32, u32)]>,
    pub(crate) feature_value_order: Box<[u32]>,
    pub(crate) references: Box<[(u32, u32)]>,
    /// Reference ids ordered by source declaration, then by canonical reference order.
    pub(crate) reference_order: Box<[AuthoredReferenceId]>,
    /// Implied relationships ordered by source declaration.
    pub(crate) implied: Box<[(u32, u32)]>,
    pub(crate) implied_order: Box<[u32]>,
    /// Implied relationships ordered by *target* declaration.
    ///
    /// The authored direction already has [`crate::index::reverse_references::ReverseReferenceIndex`]; without this one, the
    /// relationships the resolver synthesized would be visible only from their source, so an
    /// element would be told nothing points at it when something does.
    pub(crate) incoming_implied: Box<[(u32, u32)]>,
    pub(crate) incoming_implied_order: Box<[u32]>,
    /// Per-declaration ranges into `child_order`: the declarations each one owns.
    ///
    /// Prebuilt because the alternative is a scan of every declaration in the publication per
    /// element, which makes one element's inherited-feature answer cost the size of the model.
    pub(crate) children: Box<[(u32, u32)]>,
    /// Child declaration ids, ordered by owner and then by source position within the owner.
    pub(crate) child_order: Box<[DeclarationId]>,
    /// Each declaration's evaluation fact, as an index into the publication's evaluation table.
    ///
    /// Dense rather than a range, because a declaration has at most one evaluation outcome; the
    /// alternative was a linear search of the evaluation table per inspected element, which made
    /// inspecting one element cost the size of the model.
    pub(crate) evaluation: Box<[Option<u32>]>,
}

/// Builds the contiguous per-declaration ranges of an ordered view of a record table.
///
/// `owners` must be the owning declaration of each entry **in the order the view will be sliced**;
/// the returned ranges index that view, not the underlying table.
pub(crate) fn ranges_by_declaration(
    declarations: usize,
    owners: impl Iterator<Item = DeclarationId>,
) -> Box<[(u32, u32)]> {
    let mut counts = vec![0u32; declarations];
    for owner in owners {
        if let Some(slot) = counts.get_mut(owner.index()) {
            *slot += 1;
        }
    }
    let mut ranges = Vec::with_capacity(declarations);
    let mut start = 0u32;
    for count in counts {
        ranges.push((start, start + count));
        start += count;
    }
    ranges.into_boxed_slice()
}

/// What makes one effective feature shadow another: the authored name, or -- for an anonymous
/// member, which no name can shadow -- the member's own handle.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum ShadowKey {
    Named(Box<str>),
    Anonymous(SymbolId),
}

impl ElementFactIndex {
    pub(crate) fn build(
        storage: &SemanticModelStorage,
        resolution: &ResolutionResults,
        evaluation: &[EvaluationFact],
    ) -> Result<Self, ResolutionError> {
        let declarations = storage.declarations.len();

        // Each record table gets an explicit declaration-ordered view, and the ranges below index
        // that view. Nothing here assumes the lowering emitted records already grouped by
        // declaration -- it does today, but that would be an invariant of the producer rather than
        // of this index, and slicing the raw table on ranges computed for a sorted order would be
        // silently wrong the moment it stopped holding.
        let mut documentation_order: Vec<u32> = (0..storage.documentation.len() as u32).collect();
        documentation_order
            .sort_by_key(|index| storage.documentation[*index as usize].declaration.index());
        let mut feature_value_order: Vec<u32> = (0..storage.feature_values.len() as u32).collect();
        feature_value_order
            .sort_by_key(|index| storage.feature_values[*index as usize].declaration.index());

        let mut reference_order: Vec<AuthoredReferenceId> = (0..storage.references.len())
            .filter_map(|index| AuthoredReferenceId::from_index(index).ok())
            .collect();
        reference_order.sort_by_key(|id| {
            let reference = &storage.references[id.index()];
            (reference.source.index(), reference.kind, reference.ordinal)
        });

        let mut implied_order: Vec<u32> =
            (0..resolution.implied_relationships.len() as u32).collect();
        implied_order.sort_by_key(|index| {
            resolution.implied_relationships[*index as usize]
                .source
                .index()
        });

        let mut incoming_implied_order: Vec<u32> =
            (0..resolution.implied_relationships.len() as u32).collect();
        incoming_implied_order.sort_by_key(|index| {
            resolution.implied_relationships[*index as usize]
                .target
                .index()
        });

        // Source order within an owner, so an inherited-feature list reads the way the author
        // wrote the type rather than the way lowering happened to visit it.
        let mut child_order: Vec<DeclarationId> = (0..declarations)
            .filter_map(|index| DeclarationId::from_index(index).ok())
            .filter(|id| {
                storage
                    .declaration(*id)
                    .is_some_and(|declaration| declaration.owner.is_some())
            })
            .collect();
        child_order.sort_by_key(|id| {
            let declaration = storage.declaration(*id);
            (
                declaration
                    .and_then(|declaration| declaration.owner)
                    .map(|owner| owner.index())
                    .unwrap_or_default(),
                declaration
                    .map(|declaration| declaration.document.0)
                    .unwrap_or_default(),
                declaration
                    .map(|declaration| declaration.span.offset)
                    .unwrap_or_default(),
                id.index(),
            )
        });

        let mut evaluation_by_declaration = vec![None; declarations];
        for (index, fact) in evaluation.iter().enumerate() {
            if let Some(slot) = evaluation_by_declaration.get_mut(fact.declaration.index()) {
                // A declaration publishes at most one evaluation outcome; keeping the first is
                // deterministic if that ever stops holding.
                slot.get_or_insert(index as u32);
            }
        }

        Ok(Self {
            documentation: ranges_by_declaration(
                declarations,
                documentation_order
                    .iter()
                    .map(|index| storage.documentation[*index as usize].declaration),
            ),
            documentation_order: documentation_order.into_boxed_slice(),
            feature_values: ranges_by_declaration(
                declarations,
                feature_value_order
                    .iter()
                    .map(|index| storage.feature_values[*index as usize].declaration),
            ),
            feature_value_order: feature_value_order.into_boxed_slice(),
            references: ranges_by_declaration(
                declarations,
                reference_order
                    .iter()
                    .map(|id| storage.references[id.index()].source),
            ),
            reference_order: reference_order.into_boxed_slice(),
            implied: ranges_by_declaration(
                declarations,
                implied_order
                    .iter()
                    .map(|index| resolution.implied_relationships[*index as usize].source),
            ),
            implied_order: implied_order.into_boxed_slice(),
            incoming_implied: ranges_by_declaration(
                declarations,
                incoming_implied_order
                    .iter()
                    .map(|index| resolution.implied_relationships[*index as usize].target),
            ),
            incoming_implied_order: incoming_implied_order.into_boxed_slice(),
            children: ranges_by_declaration(
                declarations,
                child_order.iter().filter_map(|id| {
                    storage
                        .declaration(*id)
                        .and_then(|declaration| declaration.owner)
                }),
            ),
            child_order: child_order.into_boxed_slice(),
            evaluation: evaluation_by_declaration.into_boxed_slice(),
        })
    }
}

pub(crate) fn slice_range<'a, T>(
    entries: &'a [T],
    ranges: &[(u32, u32)],
    declaration: DeclarationId,
) -> &'a [T] {
    match ranges.get(declaration.index()) {
        Some((start, end)) => {
            let slice = &entries[*start as usize..*end as usize];
            // The range lookup plus the entries the caller will read: this declaration's own
            // facts, and never another's.
            record_visited_index_entries(1 + slice.len());
            slice
        }
        None => {
            record_visited_index_entries(1);
            &[]
        }
    }
}

impl<D> SemanticModel<D> {
    /// The `::`-joined owner path of a declaration, borrowed from the settled blob.
    ///
    /// A display convenience, not an identity: an anonymous ancestor contributes an empty
    /// segment, so two elements can share a qualified name. `symbol_id` is the identity.
    ///
    /// The path is written once at the barrier by `QualifiedNameIndex`, so this is a slice, not
    /// the owner-chain walk and `join` it used to be on every element of every result.
    pub(crate) fn qualified_name(&self, id: DeclarationId) -> &str {
        self.qualified_names.qualified_name(id).unwrap_or_default()
    }

    pub(crate) fn source_location(&self, id: DeclarationId) -> Option<SourceLocation> {
        let declaration = self.storage.declaration(id)?;
        let document = self
            .storage
            .document(declaration.document)?
            .identity
            .clone();
        // The settled identifier range, not a text scan: the publication no longer owns the source
        // this used to re-read. A named declaration whose identifier the barrier could not settle
        // has no location, exactly as the failed search had none; only an unnamed one falls back to
        // its whole span.
        let range = match declaration.name {
            Some(_) => self.documents.declaration_identifier(id)?,
            None => document_range(&self.storage, declaration.document, &declaration.span).ok()?,
        };
        Some(SourceLocation {
            document,
            range,
            role: OccurrenceRole::Declaration,
        })
    }

    pub(crate) fn membership_facts(&self, id: DeclarationId) -> Option<MembershipFacts> {
        let membership = self.memberships.get(id)?;
        Some(MembershipFacts {
            kind: match membership.kind {
                crate::model::MembershipKind::Owning => MembershipKind::Owning,
                crate::model::MembershipKind::Feature => MembershipKind::Feature,
                crate::model::MembershipKind::Import => MembershipKind::Import,
                crate::model::MembershipKind::Alias => MembershipKind::Alias,
            },
            visibility: match membership.visibility {
                EffectiveVisibility::Public => Visibility::Public,
                EffectiveVisibility::Private => Visibility::Private,
                EffectiveVisibility::Protected => Visibility::Protected,
            },
            provenance: if membership.authored {
                VisibilityProvenance::Authored
            } else {
                VisibilityProvenance::Default
            },
        })
    }

    pub(crate) fn documentation(&self, id: DeclarationId) -> Box<[Documentation]> {
        slice_range(
            &self.facts.documentation_order,
            &self.facts.documentation,
            id,
        )
        .iter()
        .map(|index| &self.storage.documentation[*index as usize])
        .map(|record| Documentation {
            form: match record.form {
                crate::lower::facts::AnnotationForm::Documentation => AnnotationForm::Documentation,
                crate::lower::facts::AnnotationForm::Comment => AnnotationForm::Comment,
                crate::lower::facts::AnnotationForm::TextualRepresentation => {
                    AnnotationForm::TextualRepresentation
                }
            },
            locale: record
                .locale
                .and_then(|id| self.storage.symbol(id))
                .map(Into::into),
            language: record
                .language
                .and_then(|id| self.storage.symbol(id))
                .map(Into::into),
            text: self.storage.symbol(record.text).unwrap_or_default().into(),
        })
        .collect()
    }

    pub(crate) fn authored_value(&self, id: DeclarationId) -> Option<AuthoredValue> {
        slice_range(
            &self.facts.feature_value_order,
            &self.facts.feature_values,
            id,
        )
        .first()
        .map(|index| &self.storage.feature_values[*index as usize])
        .map(|record| AuthoredValue {
            kind: match record.kind {
                crate::lower::facts::FeatureValueKind::Bind => ValueKind::Bind,
                crate::lower::facts::FeatureValueKind::Assign => ValueKind::Assign,
            },
            is_default: record.is_default,
            has_operator: record.has_operator,
        })
    }

    pub(crate) fn multiplicity(&self, id: DeclarationId) -> MultiplicityFacts {
        let Some(facts) = self.storage.declaration_facts(id) else {
            return MultiplicityFacts::Absent;
        };
        let Some(multiplicity) = &facts.multiplicity else {
            return MultiplicityFacts::Absent;
        };
        let bound = |value: crate::lower::facts::MultiplicityBound| match value {
            crate::lower::facts::MultiplicityBound::Unbounded => MultiplicityBound::Unbounded,
            crate::lower::facts::MultiplicityBound::Literal(value) => {
                MultiplicityBound::Literal(value)
            }
            crate::lower::facts::MultiplicityBound::Expression => MultiplicityBound::Expression,
        };
        MultiplicityFacts::Declared {
            lower: bound(multiplicity.lower),
            upper: bound(multiplicity.upper),
            ordered: facts.modifiers.ordered,
            nonunique: facts.modifiers.nonunique,
        }
    }

    pub(crate) fn modifiers(&self, id: DeclarationId) -> Box<[ElementModifier]> {
        let Some(facts) = self.storage.declaration_facts(id) else {
            return Box::default();
        };
        let modifiers = &facts.modifiers;
        [
            (modifiers.is_abstract, ElementModifier::Abstract),
            (modifiers.variation, ElementModifier::Variation),
            (modifiers.individual, ElementModifier::Individual),
            (modifiers.derived, ElementModifier::Derived),
            (modifiers.end, ElementModifier::End),
            (modifiers.reference, ElementModifier::Reference),
            (modifiers.constant, ElementModifier::Constant),
            (modifiers.event, ElementModifier::Event),
            (modifiers.standard, ElementModifier::Standard),
            (modifiers.all, ElementModifier::All),
            (modifiers.composite, ElementModifier::Composite),
            (modifiers.portion, ElementModifier::Portion),
            (modifiers.var, ElementModifier::Var),
            (modifiers.member, ElementModifier::Member),
            (modifiers.ordered, ElementModifier::Ordered),
            (modifiers.nonunique, ElementModifier::Nonunique),
        ]
        .into_iter()
        .filter_map(|(present, modifier)| present.then_some(modifier))
        .collect()
    }

    pub(crate) fn relationships(&self, id: DeclarationId) -> Box<[ElementRelationship]> {
        self.relationships_matching(id, |_| true)
    }

    /// A typed subset of this declaration's canonical authored and implied relationship facts.
    ///
    /// The predicate is evaluated against the private `ReferenceKind`, before presentation turns
    /// it into a rendered name. Derived-property consumers therefore cannot reclassify a
    /// relationship from text or accidentally omit an unresolved target.
    pub(crate) fn relationships_of_kinds(
        &self,
        id: DeclarationId,
        kinds: &[ReferenceKind],
    ) -> Box<[ElementRelationship]> {
        self.relationships_matching(id, |kind| kinds.contains(&kind))
    }

    pub(crate) fn relationships_matching(
        &self,
        id: DeclarationId,
        accepts: impl Fn(ReferenceKind) -> bool,
    ) -> Box<[ElementRelationship]> {
        let mut relationships = Vec::new();
        for reference_id in slice_range(&self.facts.reference_order, &self.facts.references, id) {
            let reference = &self.storage.references[reference_id.index()];
            if !accepts(reference.kind) {
                continue;
            }
            let Ok(range) = document_range(
                &self.storage,
                self.storage
                    .declaration(reference.source)
                    .map(|source| source.document)
                    .unwrap_or(DocumentId(0)),
                &reference.span,
            ) else {
                continue;
            };
            let Some(document) = self
                .storage
                .declaration(reference.source)
                .and_then(|source| self.storage.document(source.document))
                .map(|document| document.identity.clone())
            else {
                continue;
            };
            let target = match self.resolution.outcome(*reference_id) {
                Some(ResolutionStatus::Resolved(target)) => match self.symbol_id(target) {
                    Some(identity) => RelationshipTarget::Resolved(identity),
                    None => RelationshipTarget::Unresolved,
                },
                Some(ResolutionStatus::Ambiguous(candidates)) => RelationshipTarget::Ambiguous(
                    self.resolution
                        .ambiguous_candidates(candidates)
                        .iter()
                        .filter_map(|candidate| self.symbol_id(*candidate))
                        .collect(),
                ),
                Some(ResolutionStatus::Unsupported) => RelationshipTarget::Unsupported,
                Some(ResolutionStatus::Unresolved)
                | Some(ResolutionStatus::NonConverged)
                | None => RelationshipTarget::Unresolved,
            };
            relationships.push(ElementRelationship {
                kind: writer::reference_kind(reference.kind),
                provenance: if reference.flags.implied {
                    RelationshipProvenance::Implied
                } else {
                    RelationshipProvenance::Authored
                },
                authored: Some(self.authored_path(reference.path).into()),
                target,
                location: Some(SourceLocation {
                    document,
                    range,
                    role: OccurrenceRole::Reference,
                }),
            });
        }

        // Relationships the resolver synthesized carry no authored text and no source range, so
        // both are absent rather than fabricated from the rule that produced them.
        for index in slice_range(&self.facts.implied_order, &self.facts.implied, id) {
            let implied = &self.resolution.implied_relationships[*index as usize];
            if !accepts(implied.kind) {
                continue;
            }
            let Some(target) = self.symbol_id(implied.target) else {
                continue;
            };
            relationships.push(ElementRelationship {
                kind: writer::reference_kind(implied.kind),
                provenance: RelationshipProvenance::Implied,
                authored: None,
                target: RelationshipTarget::Resolved(target),
                location: None,
            });
        }
        relationships.into_boxed_slice()
    }

    /// The authored path text of a reference, as written.
    pub(crate) fn authored_path(&self, path: SymbolPathId) -> String {
        let Some((segments, rooted)) = self.storage.paths.get(path) else {
            return String::new();
        };
        let mut text = String::new();
        if rooted {
            text.push_str("$::");
        }
        for (index, segment) in segments.iter().enumerate() {
            if index != 0 {
                text.push_str("::");
            }
            text.push_str(self.storage.symbol(*segment).unwrap_or_default());
        }
        text
    }

    /// The published evaluation state of one declaration.
    ///
    /// One indexed lookup, not a search of the evaluation table: an inspector renders many
    /// elements, and a scan here would make each one cost the size of the model.
    pub(crate) fn evaluation_for(&self, id: DeclarationId) -> EvaluationState {
        record_visited_index_entries(2);
        self.facts
            .evaluation
            .get(id.index())
            .copied()
            .flatten()
            .and_then(|index| self.evaluation.get(index as usize))
            .map(|fact| fact.state.clone())
            .unwrap_or(EvaluationState::NotApplicable)
    }

    /// The authored and implied references this declaration is the source of, in canonical order.
    pub(crate) fn outgoing_reference_ids(&self, id: DeclarationId) -> &[AuthoredReferenceId] {
        slice_range(&self.facts.reference_order, &self.facts.references, id)
    }

    /// Indices into the publication's implied relationships that this declaration is the source of.
    pub(crate) fn outgoing_implied_indices(&self, id: DeclarationId) -> &[u32] {
        slice_range(&self.facts.implied_order, &self.facts.implied, id)
    }

    /// Indices into the publication's implied relationships that target this declaration.
    pub(crate) fn incoming_implied_indices(&self, id: DeclarationId) -> &[u32] {
        slice_range(
            &self.facts.incoming_implied_order,
            &self.facts.incoming_implied,
            id,
        )
    }

    /// The declarations this one owns, in source order.
    pub(crate) fn child_declarations(&self, id: DeclarationId) -> &[DeclarationId] {
        slice_range(&self.facts.child_order, &self.facts.children, id)
    }

    /// The full inspection answer for one declaration.
    pub(crate) fn inspection(&self, id: DeclarationId) -> Option<ElementInspection> {
        let declaration = self.storage.declaration(id)?;
        let facts = self.storage.declaration_facts(id)?;
        Some(ElementInspection {
            identity: self.symbol_id(id)?,
            kind: element_kind::element_kind(declaration.kind),
            role: element_kind::membership_role_with_trigger(
                declaration.kind,
                facts.is_trigger_action,
            ),
            name: declaration
                .name
                .and_then(|name| self.storage.symbol(name))
                .map(Into::into),
            short_name: facts
                .short_name
                .and_then(|name| self.storage.symbol(name))
                .map(Into::into),
            qualified_name: self.qualified_name(id).into(),
            location: self.source_location(id)?,
            declaration_range: document_range(
                &self.storage,
                declaration.document,
                &declaration.span,
            )
            .ok()?,
            owner: declaration
                .owner
                .and_then(|owner| self.symbol_id(owner)),
            membership: self.membership_facts(id)?,
            documentation: self.documentation(id),
            multiplicity: self.multiplicity(id),
            modifiers: self.modifiers(id),
            portion_kind: facts.portion_kind.map(|kind| match kind {
                crate::lower::facts::PortionKind::Snapshot => PortionKind::Snapshot,
                crate::lower::facts::PortionKind::Timeslice => PortionKind::Timeslice,
            }),
            direction: facts.direction.map(|direction| match direction {
                ParameterDirection::In => FeatureDirection::In,
                ParameterDirection::Out => FeatureDirection::Out,
                ParameterDirection::InOut => FeatureDirection::InOut,
            }),
            value: self.authored_value(id),
            evaluation: self.evaluation_for(id),
            relationships: self.relationships(id),
        })
    }

    pub(crate) fn inspect(&self, symbol: SymbolId) -> QueryOutcome<ElementInspection> {
        let Some(id) = self.declaration_of(symbol) else {
            return QueryOutcome::Unresolved;
        };
        match self.inspection(id) {
            Some(inspection) => self.resolved_outcome(inspection),
            None => QueryOutcome::Unresolved,
        }
    }

    pub(crate) fn inspect_at(
        &self,
        document: &str,
        position: TextPosition,
    ) -> QueryOutcome<ElementInspectionAt> {
        let Some(document_id) = self.documents.document(&self.storage, document) else {
            return QueryOutcome::Unresolved;
        };
        let Some(positions) = self.documents.positions(document_id) else {
            return QueryOutcome::Unresolved;
        };

        let containing = positions
            .spans
            .innermost_containing(position)
            .and_then(|id| self.inspection(id));

        // The reference's own outcome is carried through rather than filtered to the resolved
        // case: "nothing here" and "here, but unresolved" are different answers.
        let referenced = leaf_ranges_containing(&positions.references, position)
            .next()
            .map_or(ReferenceAt::None, |reference_id| {
                match self.resolution.outcome(reference_id) {
                    Some(ResolutionStatus::Resolved(target)) => self
                        .inspection(target)
                        .map_or(ReferenceAt::Unresolved, |inspection| {
                            ReferenceAt::Resolved(Box::new(inspection))
                        }),
                    Some(ResolutionStatus::Ambiguous(candidates)) => ReferenceAt::Ambiguous(
                        self.resolution
                            .ambiguous_candidates(candidates)
                            .iter()
                            .filter_map(|candidate| self.inspection(*candidate))
                            .collect(),
                    ),
                    Some(ResolutionStatus::Unsupported) => ReferenceAt::Unsupported,
                    Some(ResolutionStatus::NonConverged) => ReferenceAt::Incomplete,
                    Some(ResolutionStatus::Unresolved) | None => ReferenceAt::Unresolved,
                }
            });

        self.resolved_outcome(ElementInspectionAt {
            containing,
            referenced,
        })
    }

    pub(crate) fn document_symbols(&self, document: &str) -> QueryOutcome<Box<[SymbolEntry]>> {
        let Some(document_id) = self.documents.document(&self.storage, document) else {
            return QueryOutcome::Unresolved;
        };
        let Some(positions) = self.documents.positions(document_id) else {
            return QueryOutcome::Unresolved;
        };
        let entries = positions
            .spans
            .iter()
            .filter_map(|(range, id)| {
                let declaration = self.storage.declaration(*id)?;
                Some(SymbolEntry {
                    identity: self.symbol_id(*id)?,
                    kind: element_kind::element_kind(declaration.kind),
                    name: declaration
                        .name
                        .and_then(|name| self.storage.symbol(name))
                        .map(Into::into),
                    qualified_name: self.qualified_name(*id).into(),
                    owner: declaration
                        .owner
                        .and_then(|owner| self.symbol_id(owner)),
                    location: self.source_location(*id)?,
                    declaration_range: *range,
                })
            })
            .collect::<Vec<_>>();
        self.resolved_outcome(entries.into_boxed_slice())
    }

    pub(crate) fn search_elements(
        &self,
        search: ElementSearch,
    ) -> QueryOutcome<Box<[SymbolEntry]>> {
        let role = match search.source {
            ElementSource::Workspace => SourceRole::Workspace,
            ElementSource::StandardLibrary => SourceRole::StandardLibrary,
            ElementSource::Library => SourceRole::Library,
            ElementSource::External => SourceRole::External,
        };
        let mut entries = self
            .storage
            .declarations
            .iter()
            .enumerate()
            .filter_map(|(index, declaration)| {
                let document = self.storage.document(declaration.document)?;
                if document.role != role
                    || element_kind::element_kind(declaration.kind) != search.kind
                {
                    return None;
                }
                let id = DeclarationId::from_index(index).ok()?;
                let location = self.source_location(id)?;
                Some(SymbolEntry {
                    identity: self.symbol_id(id)?,
                    kind: search.kind,
                    name: declaration
                        .name
                        .and_then(|name| self.storage.symbol(name))
                        .map(Into::into),
                    qualified_name: self.qualified_name(id).into(),
                    owner: declaration
                        .owner
                        .and_then(|owner| self.symbol_id(owner)),
                    declaration_range: location.range,
                    location,
                })
            })
            .collect::<Vec<_>>();
        entries.sort_by(|left, right| {
            left.location
                .document
                .cmp(&right.location.document)
                .then_with(|| left.location.range.cmp(&right.location.range))
                .then_with(|| left.identity.cmp(&right.identity))
        });
        self.resolved_outcome(entries.into_boxed_slice())
    }

    pub(crate) fn effective_features(
        &self,
        symbol: SymbolId,
    ) -> QueryOutcome<Box<[SymbolEntry]>> {
        let declaration = match self.single_declaration(symbol) {
            Ok(declaration) => declaration,
            Err(outcome) => return outcome,
        };
        let mut queue = std::collections::VecDeque::from([declaration]);
        // A usage's effective feature set starts with the definitions it is typed by. Keeping the
        // usage itself first also preserves any direct nested features it authors.
        let mut direct_types = self
            .types
            .direct_types(declaration)
            .iter()
            .map(|(target, _)| *target)
            .collect::<Vec<_>>();
        direct_types.sort_by_key(|target| self.symbol_id(*target));
        queue.extend(direct_types);

        let mut visited = std::collections::BTreeSet::new();
        let mut names = std::collections::BTreeSet::<ShadowKey>::new();
        let mut result = Vec::new();
        while let Some(owner) = queue.pop_front() {
            if !visited.insert(owner) {
                continue;
            }
            let mut children = self
                .child_declarations(owner)
                .iter()
                .copied()
                .filter(|id| self.types.featuring_type(*id) == Some(owner))
                .collect::<Vec<_>>();
            children.sort_by(|left, right| {
                match (self.source_location(*left), self.source_location(*right)) {
                    (Some(left), Some(right)) => left
                        .document
                        .cmp(&right.document)
                        .then_with(|| left.range.cmp(&right.range)),
                    (Some(_), None) => std::cmp::Ordering::Less,
                    (None, Some(_)) => std::cmp::Ordering::Greater,
                    (None, None) => left.cmp(right),
                }
            });
            for child in children {
                let Some(entry) = self.symbol_entry(child) else {
                    continue;
                };
                // An anonymous member shadows nothing and is shadowed by nothing, so its key
                // is its own handle rather than a materialised identity string.
                let shadow_key = match entry.name.clone() {
                    Some(name) => ShadowKey::Named(name),
                    None => ShadowKey::Anonymous(entry.identity),
                };
                if names.insert(shadow_key) {
                    result.push(entry);
                }
            }

            let mut bases = self
                .types
                .supertypes(owner)
                .iter()
                .filter(|(_, scopes)| {
                    types::scopes_of(*scopes)
                        .any(|scope| scope == types::ScopeBits::Subclassification)
                })
                .map(|(target, _)| *target)
                .collect::<Vec<_>>();
            bases.sort_by_key(|target| self.symbol_id(*target));
            queue.extend(bases);
        }
        self.resolved_outcome(result.into_boxed_slice())
    }

    pub(crate) fn symbol_entry(&self, id: DeclarationId) -> Option<SymbolEntry> {
        let declaration = self.storage.declaration(id)?;
        let location = self.source_location(id)?;
        Some(SymbolEntry {
            identity: self.symbol_id(id)?,
            kind: element_kind::element_kind(declaration.kind),
            name: declaration
                .name
                .and_then(|name| self.storage.symbol(name))
                .map(Into::into),
            qualified_name: self.qualified_name(id).into(),
            owner: declaration
                .owner
                .and_then(|owner| self.symbol_id(owner)),
            declaration_range: location.range,
            location,
        })
    }
}
