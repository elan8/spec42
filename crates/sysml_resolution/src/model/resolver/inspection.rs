//! Assembly of the published element-inspection answer.
//!
//! Every field is read from a typed fact this crate already owns. Nothing is recovered by
//! re-reading authored text, and nothing is defaulted: a fact the parser cannot express is absent,
//! and a fact resolution could not settle keeps its own outcome.

use super::*;
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
pub(super) struct ElementFactIndex {
    /// Per-declaration ranges into `documentation_order`, not into the storage table directly:
    /// the ranges are computed for the *ordered* view, so slicing anything else would rest on the
    /// lowering happening to emit records already grouped by declaration.
    documentation: Box<[(u32, u32)]>,
    documentation_order: Box<[u32]>,
    feature_values: Box<[(u32, u32)]>,
    feature_value_order: Box<[u32]>,
    references: Box<[(u32, u32)]>,
    /// Reference ids ordered by source declaration, then by canonical reference order.
    reference_order: Box<[AuthoredReferenceId]>,
    /// Implied relationships ordered by source declaration.
    implied: Box<[(u32, u32)]>,
    implied_order: Box<[u32]>,
    /// Each declaration's evaluation fact, as an index into the publication's evaluation table.
    ///
    /// Dense rather than a range, because a declaration has at most one evaluation outcome; the
    /// alternative was a linear search of the evaluation table per inspected element, which made
    /// inspecting one element cost the size of the model.
    evaluation: Box<[Option<u32>]>,
}

/// Builds the contiguous per-declaration ranges of an ordered view of a record table.
///
/// `owners` must be the owning declaration of each entry **in the order the view will be sliced**;
/// the returned ranges index that view, not the underlying table.
fn ranges_by_declaration(
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

impl ElementFactIndex {
    pub(super) fn build(
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
            evaluation: evaluation_by_declaration.into_boxed_slice(),
        })
    }
}

fn slice_range<'a, T>(
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

impl ResolvedSemanticModel {
    /// The `::`-joined owner path of a declaration.
    ///
    /// A display convenience, not an identity: an anonymous ancestor contributes no segment, so
    /// two elements can share a qualified name. `symbol_identity` is the identity.
    fn qualified_name(&self, id: DeclarationId) -> String {
        let mut chain = vec![id];
        let mut cursor = self.storage.declaration(id).and_then(|node| node.owner);
        while let Some(current) = cursor {
            if chain.len() > self.storage.declarations.len() {
                break;
            }
            chain.push(current);
            cursor = self
                .storage
                .declaration(current)
                .and_then(|node| node.owner);
        }
        let mut segments = Vec::new();
        for current in chain.iter().rev() {
            let name = self
                .storage
                .declaration(*current)
                .and_then(|node| node.name)
                .and_then(|name| self.storage.symbol(name))
                .unwrap_or_default();
            segments.push(name);
        }
        segments.join("::")
    }

    pub(super) fn source_location(&self, id: DeclarationId) -> Option<SourceLocation> {
        let declaration = self.storage.declaration(id)?;
        let document = self
            .storage
            .document(declaration.document)?
            .identity
            .clone();
        let range = match declaration.name.and_then(|name| self.storage.symbol(name)) {
            Some(name) => declaration_identifier_range(
                &self.storage,
                declaration.document,
                &declaration.span,
                name,
            )
            .ok()?,
            None => document_range(&self.storage, declaration.document, &declaration.span).ok()?,
        };
        Some(SourceLocation {
            document,
            range,
            role: OccurrenceRole::Declaration,
        })
    }

    fn membership_facts(&self, id: DeclarationId) -> Option<MembershipFacts> {
        let membership = self.memberships.get(id)?;
        Some(MembershipFacts {
            kind: match membership.kind {
                super::MembershipKind::Owning => MembershipKind::Owning,
                super::MembershipKind::Feature => MembershipKind::Feature,
                super::MembershipKind::Import => MembershipKind::Import,
                super::MembershipKind::Alias => MembershipKind::Alias,
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

    fn documentation(&self, id: DeclarationId) -> Box<[Documentation]> {
        slice_range(
            &self.facts.documentation_order,
            &self.facts.documentation,
            id,
        )
        .iter()
        .map(|index| &self.storage.documentation[*index as usize])
        .map(|record| Documentation {
            form: match record.form {
                super::AnnotationForm::Documentation => AnnotationForm::Documentation,
                super::AnnotationForm::Comment => AnnotationForm::Comment,
                super::AnnotationForm::TextualRepresentation => {
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

    fn authored_value(&self, id: DeclarationId) -> Option<AuthoredValue> {
        slice_range(
            &self.facts.feature_value_order,
            &self.facts.feature_values,
            id,
        )
        .first()
        .map(|index| &self.storage.feature_values[*index as usize])
        .map(|record| AuthoredValue {
            kind: match record.kind {
                super::FeatureValueKind::Bind => ValueKind::Bind,
                super::FeatureValueKind::Assign => ValueKind::Assign,
            },
            is_default: record.is_default,
            has_operator: record.has_operator,
        })
    }

    fn multiplicity(&self, id: DeclarationId) -> MultiplicityFacts {
        let Some(facts) = self.storage.declaration_facts(id) else {
            return MultiplicityFacts::Absent;
        };
        let Some(multiplicity) = &facts.multiplicity else {
            return MultiplicityFacts::Absent;
        };
        let bound = |value: super::MultiplicityBound| match value {
            super::MultiplicityBound::Unbounded => MultiplicityBound::Unbounded,
            super::MultiplicityBound::Literal(value) => MultiplicityBound::Literal(value),
            super::MultiplicityBound::Expression => MultiplicityBound::Expression,
        };
        MultiplicityFacts::Declared {
            lower: bound(multiplicity.lower),
            upper: bound(multiplicity.upper),
            ordered: facts.modifiers.ordered,
            nonunique: facts.modifiers.nonunique,
        }
    }

    fn modifiers(&self, id: DeclarationId) -> Box<[ElementModifier]> {
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

    pub(super) fn relationships(&self, id: DeclarationId) -> Box<[ElementRelationship]> {
        let mut relationships = Vec::new();
        for reference_id in slice_range(&self.facts.reference_order, &self.facts.references, id) {
            let reference = &self.storage.references[reference_id.index()];
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
                Some(ResolutionStatus::Resolved(target)) => match self.symbol_identity(target) {
                    Some(identity) => RelationshipTarget::Resolved(identity),
                    None => RelationshipTarget::Unresolved,
                },
                Some(ResolutionStatus::Ambiguous(candidates)) => RelationshipTarget::Ambiguous(
                    self.resolution
                        .ambiguous_candidates(candidates)
                        .iter()
                        .filter_map(|candidate| self.symbol_identity(*candidate))
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
            let Some(target) = self.symbol_identity(implied.target) else {
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
    fn authored_path(&self, path: SymbolPathId) -> String {
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
    pub(super) fn evaluation_for(&self, id: DeclarationId) -> EvaluationState {
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

    /// The full inspection answer for one declaration.
    fn inspection(&self, id: DeclarationId) -> Option<ElementInspection> {
        let declaration = self.storage.declaration(id)?;
        let facts = self.storage.declaration_facts(id)?;
        Some(ElementInspection {
            identity: self.symbol_identity(id)?,
            kind: element_kind::element_kind(declaration.kind),
            role: element_kind::membership_role(declaration.kind),
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
                .and_then(|owner| self.symbol_identity(owner)),
            membership: self.membership_facts(id)?,
            documentation: self.documentation(id),
            multiplicity: self.multiplicity(id),
            modifiers: self.modifiers(id),
            portion_kind: facts.portion_kind.map(|kind| match kind {
                super::PortionKind::Snapshot => PortionKind::Snapshot,
                super::PortionKind::Timeslice => PortionKind::Timeslice,
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

    pub(crate) fn inspect(&self, symbol: &SymbolIdentity) -> QueryOutcome<ElementInspection> {
        let mut candidates = self.identity_declarations(symbol);
        if candidates.len() > 1 {
            let inspections = candidates
                .into_iter()
                .filter_map(|id| self.inspection(id))
                .collect::<Vec<_>>();
            return QueryOutcome::Ambiguous(inspections.into_boxed_slice());
        }
        let Some(id) = candidates.pop() else {
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
                    identity: self.symbol_identity(*id)?,
                    kind: element_kind::element_kind(declaration.kind),
                    name: declaration
                        .name
                        .and_then(|name| self.storage.symbol(name))
                        .map(Into::into),
                    qualified_name: self.qualified_name(*id).into(),
                    owner: declaration
                        .owner
                        .and_then(|owner| self.symbol_identity(owner)),
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
                    identity: self.symbol_identity(id)?,
                    kind: search.kind,
                    name: declaration
                        .name
                        .and_then(|name| self.storage.symbol(name))
                        .map(Into::into),
                    qualified_name: self.qualified_name(id).into(),
                    owner: declaration
                        .owner
                        .and_then(|owner| self.symbol_identity(owner)),
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
        symbol: &SymbolIdentity,
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
        direct_types.sort_by_key(|target| self.symbol_identity(*target));
        queue.extend(direct_types);

        let mut visited = std::collections::BTreeSet::new();
        let mut names = std::collections::BTreeSet::<Box<str>>::new();
        let mut result = Vec::new();
        while let Some(owner) = queue.pop_front() {
            if !visited.insert(owner) {
                continue;
            }
            let mut children = self
                .storage
                .declarations
                .iter()
                .enumerate()
                .filter_map(|(index, child)| {
                    let id = DeclarationId::from_index(index).ok()?;
                    (child.owner == Some(owner) && self.types.featuring_type(id) == Some(owner))
                        .then_some(id)
                })
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
                let shadow_key = entry
                    .name
                    .clone()
                    .unwrap_or_else(|| entry.identity.as_str().into());
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
                        .any(|scope| scope == types::SpecializationScope::Subclassification)
                })
                .map(|(target, _)| *target)
                .collect::<Vec<_>>();
            bases.sort_by_key(|target| self.symbol_identity(*target));
            queue.extend(bases);
        }
        self.resolved_outcome(result.into_boxed_slice())
    }

    fn symbol_entry(&self, id: DeclarationId) -> Option<SymbolEntry> {
        let declaration = self.storage.declaration(id)?;
        let location = self.source_location(id)?;
        Some(SymbolEntry {
            identity: self.symbol_identity(id)?,
            kind: element_kind::element_kind(declaration.kind),
            name: declaration
                .name
                .and_then(|name| self.storage.symbol(name))
                .map(Into::into),
            qualified_name: self.qualified_name(id).into(),
            owner: declaration
                .owner
                .and_then(|owner| self.symbol_identity(owner)),
            declaration_range: location.range,
            location,
        })
    }
}
