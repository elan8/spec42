//! What an editor may offer to repair an authored reference or declaration.
//!
//! Each answer is read from settled facts: which outcome a reference settled to, which candidates
//! an ambiguity kept, which memberships are public, and the metaclass rules validation applies.
//! A candidate is offered only when the resolver confirms that the spelling it proposes names it,
//! so applying a repair never depends on a consumer reproducing name lookup.

use crate::index::documents::leaf_ranges_containing;
use crate::model::element_kind::element_kind;
use crate::model::metaclass::{classify, definition_kind, Role};
use crate::model::resolver::ResolvedSemanticModel;
use crate::model::span::document_range;
use crate::model::DeclarationId;
use crate::model::DeclarationKind;
use crate::model::NameId;
use crate::model::ReferenceKind;
use crate::resolve::names::{lookup_lexical_into, FirstScopePolicy, LookupTarget};
use crate::resolve::reference_target_domain;
use crate::resolve::results::{
    EffectiveNameOutcome, ResolutionError, ResolutionStatus, ResolutionWork,
};
use crate::resolve::{DeclarationDomain, ResolutionIndexes};
use crate::ElementKind;
use crate::QueryAnswer;
use crate::QueryOutcome;
use crate::SymbolId;
use crate::TextId;
use crate::TextPosition;
use crate::TextRange;

/// The repair an editor may offer for one authored reference that did not resolve to one target.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReferenceRepair {
    /// The whole authored reference: the text a qualification replaces.
    pub range: TextRange,
    pub slot: RepairSlot,
    pub outcome: ReferenceRepairOutcome,
}

/// The role the reference plays for its source declaration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepairSlot {
    /// A usage's `:` typing.
    Typing,
    /// A definition's `:>` specialization.
    Specialization,
    /// Any other reference; no definition is proposed for it.
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReferenceRepairOutcome {
    /// No visible element has the authored name.
    Unresolved {
        /// The authored name, when the reference is one unqualified name; a qualified or rooted
        /// reference has no single name an import or a new definition could supply.
        name: Option<TextId>,
        /// Definitions the name could mean that an import into `namespace` makes visible, in
        /// canonical order: workspace before library, then by element qualified name.
        imports: Box<[ImportCandidate]>,
        /// The namespace an import or a new definition belongs in: the innermost package that
        /// encloses the reference, or `None` for the document's root namespace.
        namespace: Option<RepairNamespace>,
        /// The definition kind the slot is typed by, where the source's metaclass fixes one.
        expected_definition: Option<ElementKind>,
    },
    /// Several visible elements have the authored name.
    Ambiguous {
        /// The candidates whose element qualified name names exactly that candidate from the
        /// reference's scope, in canonical order by that name. A candidate no qualified name
        /// reaches unambiguously is omitted.
        candidates: Box<[QualificationCandidate]>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RepairNamespace {
    pub symbol: SymbolId,
    /// The namespace's declared-name range, in the reference's document.
    pub identifier: TextRange,
}

/// A definition a membership import of its element qualified name makes the reference resolve to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImportCandidate {
    pub target: SymbolId,
    pub kind: ElementKind,
    /// Whether the target is authored in a library rather than the workspace.
    pub library: bool,
}

/// A candidate whose element qualified name, written in place of the reference, names only it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QualificationCandidate {
    pub target: SymbolId,
    pub kind: ElementKind,
}

/// A declaration whose authored value silently overrides the inherited member its name
/// implicitly redefines.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImplicitRedefinitionRepair {
    pub declaration: SymbolId,
    /// The declared name, which an explicit `:>>` precedes.
    pub identifier: TextRange,
    pub redefined: SymbolId,
}

impl ResolvedSemanticModel {
    /// The repair for the reference at `position`.
    ///
    /// `Unresolved` when no reference is there or it resolved to one target. A repair becomes
    /// authored source, so like rename it requires a complete publication.
    pub(crate) fn reference_repair(
        &self,
        document: &str,
        position: TextPosition,
    ) -> QueryOutcome<ReferenceRepair> {
        if !self.metadata.completeness.is_complete() {
            return self.query_outcome(QueryAnswer::Incomplete);
        }
        match self.reference_repair_at(document, position) {
            Ok(answer) => self.query_outcome(answer),
            Err(_) => self.query_outcome(QueryAnswer::Incomplete),
        }
    }

    fn reference_repair_at(
        &self,
        document: &str,
        position: TextPosition,
    ) -> Result<QueryAnswer<ReferenceRepair>, ResolutionError> {
        let Some(positions) = self
            .documents
            .document(&self.storage, document)
            .and_then(|document| self.documents.positions(document))
        else {
            return Ok(QueryAnswer::Unresolved);
        };
        for id in leaf_ranges_containing(&positions.references, position) {
            let reference = self
                .storage
                .references
                .get(id.index())
                .ok_or(ResolutionError::InvalidStorage)?;
            let source = self
                .storage
                .declaration(reference.source)
                .ok_or(ResolutionError::InvalidStorage)?;
            let range = document_range(&self.storage, source.document, &reference.span)?;
            let slot = match reference.kind {
                ReferenceKind::FeatureTyping => RepairSlot::Typing,
                ReferenceKind::Subclassification => RepairSlot::Specialization,
                _ => RepairSlot::Other,
            };
            let outcome = match self.resolution.outcome(id) {
                Some(ResolutionStatus::Resolved(_)) => continue,
                Some(ResolutionStatus::Unsupported) => return Ok(QueryAnswer::Unsupported),
                Some(ResolutionStatus::NonConverged) | None => return Ok(QueryAnswer::Incomplete),
                // A feature chain resolves hop by hop; neither an import nor a qualification
                // repairs a hop.
                Some(_) if reference.flags.dotted => return Ok(QueryAnswer::Unsupported),
                Some(ResolutionStatus::Unresolved) => {
                    self.unresolved_repair(reference.source, reference.kind, reference.path)?
                }
                Some(ResolutionStatus::Ambiguous(candidates)) => {
                    let mut qualified = Vec::new();
                    for candidate in self.resolution.ambiguous_candidates(candidates) {
                        if let Some(name) = self.spelling_from(source.owner, *candidate)? {
                            let target = self
                                .symbol_id(*candidate)
                                .ok_or(ResolutionError::InvalidStorage)?;
                            qualified.push((
                                name,
                                QualificationCandidate {
                                    target,
                                    kind: self.element_kind_of(*candidate)?,
                                },
                            ));
                        }
                    }
                    qualified.sort_by(|(left_name, left), (right_name, right)| {
                        left_name
                            .cmp(right_name)
                            .then_with(|| left.target.cmp(&right.target))
                    });
                    qualified.dedup_by(|(_, left), (_, right)| left.target == right.target);
                    ReferenceRepairOutcome::Ambiguous {
                        candidates: qualified
                            .into_iter()
                            .map(|(_, candidate)| candidate)
                            .collect(),
                    }
                }
            };
            return Ok(QueryAnswer::Resolved(ReferenceRepair {
                range,
                slot,
                outcome,
            }));
        }
        Ok(QueryAnswer::Unresolved)
    }

    fn unresolved_repair(
        &self,
        source: DeclarationId,
        kind: ReferenceKind,
        path: crate::model::SymbolPathId,
    ) -> Result<ReferenceRepairOutcome, ResolutionError> {
        let (segments, rooted) = self
            .storage
            .paths
            .get(path)
            .ok_or(ResolutionError::InvalidStorage)?;
        let name = match segments {
            [name] if !rooted => Some(*name),
            _ => None,
        };
        let namespace = self.enclosing_package(source)?;
        let expected_definition = match kind {
            ReferenceKind::FeatureTyping | ReferenceKind::Subclassification => self
                .storage
                .declaration(source)
                .and_then(|declaration| classify(declaration.kind))
                .filter(|(_, role)| {
                    (kind == ReferenceKind::FeatureTyping) == (*role == Role::Usage)
                })
                .map(|(family, _)| element_kind(definition_kind(family))),
            _ => None,
        };
        let imports = match name {
            Some(name)
                if matches!(
                    kind,
                    ReferenceKind::FeatureTyping | ReferenceKind::Subclassification
                ) =>
            {
                self.import_candidates(source, kind, name, namespace)?
            }
            _ => Vec::new(),
        };
        Ok(ReferenceRepairOutcome::Unresolved {
            name: match name {
                Some(name) => {
                    Some(TextId::from_index(name.index()).ok_or(ResolutionError::Capacity)?)
                }
                None => None,
            },
            imports: imports.into_boxed_slice(),
            namespace: match namespace {
                Some(package) => Some(RepairNamespace {
                    symbol: self
                        .symbol_id(package)
                        .ok_or(ResolutionError::InvalidStorage)?,
                    identifier: self
                        .documents
                        .declaration_identifier(package)
                        .ok_or(ResolutionError::InvalidStorage)?,
                }),
                None => None,
            },
            expected_definition,
        })
    }

    /// Every definition named `name` that a membership import into `namespace` would make the
    /// reference resolve to.
    fn import_candidates(
        &self,
        source: DeclarationId,
        kind: ReferenceKind,
        name: NameId,
        namespace: Option<DeclarationId>,
    ) -> Result<Vec<ImportCandidate>, ResolutionError> {
        let owner = self
            .storage
            .declaration(source)
            .ok_or(ResolutionError::InvalidStorage)?
            .owner;
        // An element the reference already sees by this name shadows anything an import adds, so
        // the reference is unresolved for a reason an import does not repair.
        if !self.lexical_candidates(owner, name)?.is_empty() {
            return Ok(Vec::new());
        }
        let domain = reference_target_domain(kind);
        let mut candidates = Vec::new();
        for index in 0..self.storage.declarations.len() {
            let candidate =
                DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
            if candidate == source || self.effective_name_id(candidate) != Some(name) {
                continue;
            }
            let declaration = self
                .storage
                .declaration(candidate)
                .ok_or(ResolutionError::InvalidStorage)?;
            // A top-level element is a member of the root namespace every scope already sees.
            if declaration.owner.is_none()
                || !domain.accepts(declaration.kind)
                || self
                    .reference_kind_violation(kind, source, candidate)
                    .is_some()
            {
                continue;
            }
            let Some(import_path) = self.spelling_from(namespace, candidate)? else {
                continue;
            };
            let library = self
                .storage
                .documents
                .get(declaration.document.index())
                .ok_or(ResolutionError::InvalidStorage)?
                .role
                != source_identity::SourceRole::Workspace;
            candidates.push((
                import_path,
                ImportCandidate {
                    target: self
                        .symbol_id(candidate)
                        .ok_or(ResolutionError::InvalidStorage)?,
                    kind: element_kind(declaration.kind),
                    library,
                },
            ));
        }
        candidates.sort_by(|(left_path, left), (right_path, right)| {
            left.library
                .cmp(&right.library)
                .then_with(|| left_path.cmp(right_path))
                .then_with(|| left.target.cmp(&right.target))
        });
        Ok(candidates
            .into_iter()
            .map(|(_, candidate)| candidate)
            .collect())
    }

    /// The escaped qualified name of `target`, provided every segment of it is publicly visible
    /// and, looked up from `scope`, names exactly `target`.
    fn spelling_from(
        &self,
        scope: Option<DeclarationId>,
        target: DeclarationId,
    ) -> Result<Option<String>, ResolutionError> {
        let mut chain = vec![target];
        let mut cursor = target;
        while let Some(owner) = self
            .storage
            .declaration(cursor)
            .ok_or(ResolutionError::InvalidStorage)?
            .owner
        {
            if chain.len() > self.storage.declarations.len() {
                return Err(ResolutionError::InvalidStorage);
            }
            chain.push(owner);
            cursor = owner;
        }
        chain.reverse();
        if chain.len() < 2 || !chain.iter().all(|id| self.memberships.is_public(*id)) {
            return Ok(None);
        }
        let names = chain
            .iter()
            .map(|id| self.effective_name_id(*id))
            .collect::<Option<Vec<_>>>();
        let Some(names) = names else {
            return Ok(None);
        };
        if self.lexical_candidates(scope, names[0])? != [chain[0]] {
            return Ok(None);
        }
        for (pair, name) in chain.windows(2).zip(&names[1..]) {
            if self.direct_names.candidates(Some(pair[0]), *name) != [pair[1]] {
                return Ok(None);
            }
        }
        Ok(self.normative_qualified_name(target, 0))
    }

    fn lexical_candidates(
        &self,
        scope: Option<DeclarationId>,
        name: NameId,
    ) -> Result<Vec<DeclarationId>, ResolutionError> {
        let mut candidates = Vec::new();
        lookup_lexical_into(
            &self.storage.declarations,
            &ResolutionIndexes {
                direct_names: &self.direct_names,
                exported_names: &self.direct_names,
                effective_imports: Some(&self.effective_imports),
                exported_imports: Some(&self.effective_imports),
                inherited_names: Some(&self.resolution.inherited_names),
            },
            scope,
            name,
            LookupTarget {
                domain: DeclarationDomain::Any,
                excluded: None,
                excludes_anonymous_performs: false,
                first_scope: FirstScopePolicy::OwnedThenInherited,
            },
            &mut candidates,
            &mut ResolutionWork::default(),
        )?;
        candidates.sort_unstable();
        candidates.dedup();
        Ok(candidates)
    }

    /// The innermost package or library package that owns `declaration`.
    fn enclosing_package(
        &self,
        declaration: DeclarationId,
    ) -> Result<Option<DeclarationId>, ResolutionError> {
        let mut cursor = self
            .storage
            .declaration(declaration)
            .ok_or(ResolutionError::InvalidStorage)?
            .owner;
        while let Some(owner) = cursor {
            let owner_declaration = self
                .storage
                .declaration(owner)
                .ok_or(ResolutionError::InvalidStorage)?;
            if matches!(
                owner_declaration.kind,
                DeclarationKind::Package | DeclarationKind::LibraryPackage
            ) {
                return Ok(Some(owner));
            }
            cursor = owner_declaration.owner;
        }
        Ok(None)
    }

    fn effective_name_id(&self, declaration: DeclarationId) -> Option<NameId> {
        match self
            .resolution
            .effective_names
            .get(declaration.index())?
            .name
        {
            EffectiveNameOutcome::Resolved(name) => Some(name),
            _ => None,
        }
    }

    fn element_kind_of(&self, declaration: DeclarationId) -> Result<ElementKind, ResolutionError> {
        Ok(element_kind(
            self.storage
                .declaration(declaration)
                .ok_or(ResolutionError::InvalidStorage)?
                .kind,
        ))
    }

    /// The `:>>` repair for the declaration whose span innermost contains `position`.
    pub(crate) fn implicit_redefinition_repair(
        &self,
        document: &str,
        position: TextPosition,
    ) -> QueryOutcome<ImplicitRedefinitionRepair> {
        if !self.metadata.completeness.is_complete() {
            return self.query_outcome(QueryAnswer::Incomplete);
        }
        let Some(declaration) = self
            .documents
            .document(&self.storage, document)
            .and_then(|document| self.documents.positions(document))
            .and_then(|positions| positions.spans.innermost_containing(position))
        else {
            return self.query_outcome(QueryAnswer::Unresolved);
        };
        match self.implicit_redefinition_of(declaration) {
            Ok(Some(repair)) => self.resolved_outcome(repair),
            Ok(None) => self.query_outcome(QueryAnswer::Unresolved),
            Err(_) => self.query_outcome(QueryAnswer::Incomplete),
        }
    }

    fn implicit_redefinition_of(
        &self,
        declaration: DeclarationId,
    ) -> Result<Option<ImplicitRedefinitionRepair>, ResolutionError> {
        for relationship in self.resolution.implied_relationships.iter() {
            if relationship.kind != ReferenceKind::Redefinition
                || relationship.source != declaration
                || self
                    .implicit_value_override(declaration, relationship.target)?
                    .is_none()
            {
                continue;
            }
            let (Some(identifier), Some(symbol), Some(redefined)) = (
                self.documents.declaration_identifier(declaration),
                self.symbol_id(declaration),
                self.symbol_id(relationship.target),
            ) else {
                return Err(ResolutionError::InvalidStorage);
            };
            return Ok(Some(ImplicitRedefinitionRepair {
                declaration: symbol,
                identifier,
                redefined,
            }));
        }
        Ok(None)
    }
}
