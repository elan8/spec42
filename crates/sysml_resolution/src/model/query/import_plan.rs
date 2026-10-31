//! Counterfactual import transactions. Candidate meaning and transaction safety belong to the
//! semantic authority; consumers only choose syntax-owned insertion sites and adapt the edits.

use crate::index::documents::leaf_ranges_containing;
use crate::lower::intern::{SymbolPath, SymbolPathArena};
use crate::model::query::repair::{ImportCandidate, RepairNamespace};
use crate::model::resolver::ResolvedSemanticModel;
use crate::model::{
    AuthoredReferenceId, DeclarationId, DeclarationKind, ReferenceKind, SymbolPathId,
};
use crate::resolve::names::{lookup_lexical_into, FirstScopePolicy, LookupTarget, NameKey};
use crate::resolve::results::{
    ResolutionError, ResolutionResults, ResolutionStatus, ResolutionWork, SolverStatus,
};
use crate::resolve::{DeclarationDomain, ResolutionIndexes};
use crate::{ElementKind, QueryAnswer, QueryOutcome, SymbolId, TextPosition, TextRange};

/// A choice identifies both the imported membership and its intended semantic referent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportChoice {
    pub range: TextRange,
    pub namespace: Option<RepairNamespace>,
    pub candidate: ImportCandidate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImportRemainderReason {
    Ambiguous,
    Unavailable,
    Unsupported,
    Conflict,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportRemainder {
    pub range: TextRange,
    pub reason: ImportRemainderReason,
}

/// The most auto-imports one completion offers. Each is proved by a full counterfactual resolve,
/// so a request's cost must not grow with how many declarations share the typed prefix.
const MAX_COMPLETION_IMPORTS: usize = 10;

/// The most candidates one completion proves, which also bounds requests that keep finding unsafe
/// ones.
const MAX_COMPLETION_IMPORT_ATTEMPTS: usize = 2 * MAX_COMPLETION_IMPORTS;

/// Safe completion imports in canonical order. `truncated` means more candidates exist, so the
/// list must be requested again as the prefix grows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionImports {
    pub choices: Box<[ImportChoice]>,
    pub truncated: bool,
}

/// A deterministic safe subset, with one import per (namespace, membership).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportPlan {
    pub imports: Box<[ImportChoice]>,
    pub repaired: Box<[TextRange]>,
    pub remaining: Box<[ImportRemainder]>,
}

impl ResolvedSemanticModel {
    /// Imports name memberships. An alias has a membership spelling even though it is not an
    /// owned Element and therefore has no Element::qualifiedName of its own.
    pub(crate) fn import_spelling(&self, target: DeclarationId) -> Option<String> {
        if self.storage.declaration(target)?.kind == DeclarationKind::Alias {
            let owner = self.storage.declaration(target)?.owner?;
            Some(format!(
                "{}::{}",
                self.normative_qualified_name(owner, 0)?,
                self.escaped_name(target)?
            ))
        } else {
            self.normative_qualified_name(target, 0)
        }
    }

    pub(crate) fn import_path(&self, target: SymbolId) -> Option<String> {
        self.import_spelling(self.declaration_of(target)?)
    }

    pub(crate) fn import_reference_spelling(&self, target: SymbolId) -> Option<String> {
        self.escaped_name(self.declaration_of(target)?)
    }

    pub(super) fn import_referent(&self, target: DeclarationId) -> Option<DeclarationId> {
        if self.storage.declaration(target)?.kind != DeclarationKind::Alias {
            return Some(target);
        }
        self.storage
            .references
            .iter()
            .enumerate()
            .find(|(_, reference)| {
                reference.source == target && reference.kind == ReferenceKind::AliasBinding
            })
            .and_then(|(index, _)| match self.resolution.outcomes.get(index)? {
                ResolutionStatus::Resolved(target) => Some(*target),
                _ => None,
            })
    }

    fn import_namespace(
        &self,
        source: DeclarationId,
    ) -> Result<Option<RepairNamespace>, ResolutionError> {
        self.enclosing_package(source)?
            .map(|package| {
                Ok(RepairNamespace {
                    symbol: self
                        .symbol_id(package)
                        .ok_or(ResolutionError::InvalidStorage)?,
                    identifier: self
                        .documents
                        .declaration_identifier(package)
                        .ok_or(ResolutionError::InvalidStorage)?,
                })
            })
            .transpose()
    }

    fn import_reference(
        &self,
        document: &str,
        position: TextPosition,
    ) -> Option<AuthoredReferenceId> {
        let id = self.documents.document(&self.storage, document)?;
        let positions = self.documents.positions(id)?;
        leaf_ranges_containing(&positions.references, position).find(|id| {
            self.storage.references.get(id.index()).is_some_and(|r| {
                matches!(
                    r.kind,
                    ReferenceKind::FeatureTyping | ReferenceKind::Subclassification
                ) && !r.flags.dotted
            })
        })
    }

    /// Safe completion imports, checked against the same canonical hypothetical transaction used
    /// at resolve time. Unsafe choices are never presented as an auto-import suggestion.
    pub(crate) fn completion_imports(
        &self,
        document: &str,
        position: TextPosition,
        prefix: &str,
        expected: &[ElementKind],
    ) -> QueryOutcome<CompletionImports> {
        let outcome = self.completion_import_candidates(document, position, prefix, expected);
        let choices = match outcome.answer {
            QueryAnswer::Resolved(choices) => choices,
            // The candidate query never reports a list of lists as ambiguous.
            QueryAnswer::Incomplete | QueryAnswer::Ambiguous(_) => {
                return self.query_outcome(QueryAnswer::Incomplete)
            }
            QueryAnswer::Unresolved => return self.query_outcome(QueryAnswer::Unresolved),
            QueryAnswer::Unsupported => return self.query_outcome(QueryAnswer::Unsupported),
            QueryAnswer::Recovery => return self.query_outcome(QueryAnswer::Recovery),
        };
        let candidates = choices.len();
        let mut safe = Vec::new();
        let mut attempted = 0;
        for choice in choices {
            if safe.len() == MAX_COMPLETION_IMPORTS || attempted == MAX_COMPLETION_IMPORT_ATTEMPTS {
                break;
            }
            attempted += 1;
            match self.import_hypothesis(document, std::slice::from_ref(&choice), true) {
                Ok(Some(_)) => safe.push(choice),
                Ok(None) => {}
                Err(_) => return self.query_outcome(QueryAnswer::Incomplete),
            }
        }
        self.resolved_outcome(CompletionImports {
            choices: safe.into_boxed_slice(),
            truncated: attempted < candidates,
        })
    }

    fn completion_import_candidates(
        &self,
        document: &str,
        position: TextPosition,
        prefix: &str,
        expected: &[ElementKind],
    ) -> QueryOutcome<Box<[ImportChoice]>> {
        if !self.metadata.completeness.is_complete() {
            return self.query_outcome(QueryAnswer::Incomplete);
        }
        let result: Result<QueryAnswer<Box<[ImportChoice]>>, ResolutionError> =
            (|| {
                let Some(id) = self.import_reference(document, position) else {
                    return Ok(QueryAnswer::Unresolved);
                };
                let reference = &self.storage.references[id.index()];
                let (segments, rooted) = self
                    .storage
                    .paths
                    .get(reference.path)
                    .ok_or(ResolutionError::InvalidStorage)?;
                if rooted || segments.len() != 1 {
                    return Ok(QueryAnswer::Unsupported);
                }
                let source = self
                    .storage
                    .declaration(reference.source)
                    .ok_or(ResolutionError::InvalidStorage)?;
                let namespace = self.import_namespace(reference.source)?;
                let range = crate::model::span::document_range(
                    &self.storage,
                    source.document,
                    &reference.span,
                )?;
                let prefix = prefix.to_lowercase();
                let mut choices = Vec::new();
                for index in 0..self.storage.declarations.len() {
                    let target =
                        DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
                    let Some(name) = self.effective_name_id(target) else {
                        continue;
                    };
                    if !self
                        .storage
                        .symbol(name)
                        .is_some_and(|name| name.to_lowercase().starts_with(&prefix))
                    {
                        continue;
                    }
                    if let Some(candidate) = self.import_candidate(
                        reference.source,
                        reference.kind,
                        name,
                        self.enclosing_package(reference.source)?,
                        target,
                    )? {
                        if !expected.is_empty() && !expected.contains(&candidate.kind) {
                            continue;
                        }
                        choices.push(ImportChoice {
                            range,
                            namespace,
                            candidate,
                        });
                    }
                }
                choices.sort_by(|a, b| {
                    a.candidate
                        .library
                        .cmp(&b.candidate.library)
                        .then_with(|| {
                            self.import_spelling(self.declaration_of(a.candidate.target).unwrap())
                                .cmp(&self.import_spelling(
                                    self.declaration_of(b.candidate.target).unwrap(),
                                ))
                        })
                        .then_with(|| a.candidate.target.cmp(&b.candidate.target))
                });
                Ok(QueryAnswer::Resolved(choices.into_boxed_slice()))
            })();
        self.query_outcome(result.unwrap_or(QueryAnswer::Incomplete))
    }

    pub(crate) fn validate_completion_import(
        &self,
        document: &str,
        position: TextPosition,
        target: SymbolId,
    ) -> QueryOutcome<ImportChoice> {
        let Some(name) = self
            .declaration_of(target)
            .and_then(|target| self.effective_name(target))
        else {
            return self.query_outcome(QueryAnswer::Unresolved);
        };
        let QueryAnswer::Resolved(choices) = self
            .completion_import_candidates(document, position, name, &[])
            .answer
        else {
            return self.query_outcome(QueryAnswer::Incomplete);
        };
        let Some(choice) = choices
            .into_vec()
            .into_iter()
            .find(|c| c.candidate.target == target)
        else {
            return self.query_outcome(QueryAnswer::Unresolved);
        };
        match self.import_hypothesis(document, std::slice::from_ref(&choice), true) {
            Ok(Some(_)) => self.resolved_outcome(choice),
            Ok(None) => self.query_outcome(QueryAnswer::Unresolved),
            Err(_) => self.query_outcome(QueryAnswer::Incomplete),
        }
    }

    pub(crate) fn missing_import_plan(&self, document: &str) -> QueryOutcome<ImportPlan> {
        if !self.metadata.completeness.is_complete() {
            return self.query_outcome(QueryAnswer::Incomplete);
        }
        match self.missing_import_plan_inner(document) {
            Ok(answer) => self.query_outcome(answer),
            Err(_) => self.query_outcome(QueryAnswer::Incomplete),
        }
    }

    fn missing_import_plan_inner(
        &self,
        document: &str,
    ) -> Result<QueryAnswer<ImportPlan>, ResolutionError> {
        let Some(doc) = self.documents.document(&self.storage, document) else {
            return Ok(QueryAnswer::Unresolved);
        };
        let Some(positions) = self.documents.positions(doc) else {
            return Ok(QueryAnswer::Unresolved);
        };
        let mut choices = Vec::new();
        let mut pending = Vec::new();
        for (range, id) in positions.references.iter() {
            let reference = &self.storage.references[id.index()];
            let reason = match self.resolution.outcome(*id) {
                Some(ResolutionStatus::Resolved(_)) => continue,
                Some(ResolutionStatus::Ambiguous(_)) => ImportRemainderReason::Ambiguous,
                Some(ResolutionStatus::Unresolved) => {
                    if let QueryAnswer::Resolved(repair) =
                        self.reference_repair(document, range.start).answer
                    {
                        if let super::repair::ReferenceRepairOutcome::Unresolved {
                            imports,
                            namespace,
                            ..
                        } = repair.outcome
                        {
                            if let [candidate] = imports.as_ref() {
                                choices.push(ImportChoice {
                                    range: *range,
                                    namespace,
                                    candidate: *candidate,
                                });
                                pending.push((*id, *range, ImportRemainderReason::Conflict));
                                continue;
                            }
                            if imports.len() > 1 {
                                pending.push((*id, *range, ImportRemainderReason::Ambiguous));
                                continue;
                            }
                        }
                    }
                    ImportRemainderReason::Unavailable
                }
                _ => ImportRemainderReason::Unsupported,
            };
            // Import targets and other reference kinds are retained as explicit remaining cases.
            if matches!(
                reference.kind,
                ReferenceKind::FeatureTyping | ReferenceKind::Subclassification
            ) {
                pending.push((*id, *range, reason));
            }
        }
        choices.sort_by_key(|c| (c.range.start, c.candidate.target));
        let mut accepted = choices.clone();
        let mut result = if choices.is_empty() {
            None
        } else {
            self.import_hypothesis(document, &choices, false)?
        };
        if !choices.is_empty() && result.is_none() {
            accepted.clear();
            for choice in choices {
                accepted.push(choice);
                match self.import_hypothesis(document, &accepted, false)? {
                    Some(resolution) => result = Some(resolution),
                    None => {
                        accepted.pop();
                    }
                }
            }
        }
        let mut repaired = Vec::new();
        let mut remaining = Vec::new();
        for (id, range, reason) in pending {
            if matches!(
                result.as_ref().and_then(|r| r.outcome(id)),
                Some(ResolutionStatus::Resolved(_))
            ) {
                repaired.push(range);
            } else {
                remaining.push(ImportRemainder { range, reason });
            }
        }
        accepted.sort_by_key(|c| (c.namespace.map(|n| n.symbol), c.candidate.target));
        accepted.dedup_by_key(|c| (c.namespace.map(|n| n.symbol), c.candidate.target));
        Ok(QueryAnswer::Resolved(ImportPlan {
            imports: accepted.into_boxed_slice(),
            repaired: repaired.into_boxed_slice(),
            remaining: remaining.into_boxed_slice(),
        }))
    }

    /// Re-run the canonical fixed-point resolver with ephemeral private membership imports.
    /// No text, parser identities, publication facts, or cached indexes are modified. Every
    /// previously settled binding must survive, and no unresolved reference may become ambiguous
    /// or acquire an incompatible target. This includes transitive inheritance and alias effects.
    fn import_hypothesis(
        &self,
        document: &str,
        choices: &[ImportChoice],
        replace: bool,
    ) -> Result<Option<ResolutionResults>, ResolutionError> {
        let mut proposed = Vec::new();
        let mut references = self.storage.references.to_vec();
        let mut paths = self.storage.paths.paths.to_vec();
        let mut segments = self.storage.paths.segments.to_vec();
        let mut intended = Vec::new();
        for choice in choices {
            let id = self
                .import_reference(document, choice.range.start)
                .ok_or(ResolutionError::InvalidStorage)?;
            let target = self
                .declaration_of(choice.candidate.target)
                .ok_or(ResolutionError::InvalidStorage)?;
            let name = self
                .effective_name_id(target)
                .ok_or(ResolutionError::InvalidStorage)?;
            let namespace = choice
                .namespace
                .map(|n| {
                    self.declaration_of(n.symbol)
                        .ok_or(ResolutionError::InvalidStorage)
                })
                .transpose()?;
            proposed.push((
                NameKey {
                    owner: namespace,
                    name,
                },
                target,
            ));
            intended.push((
                id,
                self.declaration_of(choice.candidate.referent)
                    .ok_or(ResolutionError::InvalidStorage)?,
            ));
            if replace {
                let path =
                    SymbolPathId::from_index(paths.len()).map_err(|_| ResolutionError::Capacity)?;
                paths.push(SymbolPath {
                    start: u32::try_from(segments.len()).map_err(|_| ResolutionError::Capacity)?,
                    len: 1,
                    rooted: false,
                });
                segments.push(name);
                references[id.index()].path = path;
            }
        }
        proposed.sort_unstable();
        proposed.dedup();
        let mut storage = self.storage.clone();
        storage.references = references.clone().into_boxed_slice();
        storage.paths = SymbolPathArena {
            paths: paths.into_boxed_slice(),
            segments: segments.into_boxed_slice(),
        };
        let crate::pipeline::phase::ResolvedFacts {
            direct_names: direct,
            effective_imports: effective,
            exported_imports: exported,
            resolution,
            ..
        } = crate::pipeline::phase::resolve_facts(&storage, None, &proposed)?;
        if resolution.solver_status != SolverStatus::Converged {
            return Ok(None);
        }
        for (id, target) in &intended {
            if resolution.outcome(*id) != Some(ResolutionStatus::Resolved(*target)) {
                return Ok(None);
            }
        }
        // The import paths themselves must still name the chosen membership after all imports
        // are present (e.g. importing a definition named like another path's root is unsafe).
        for choice in choices {
            let target = self
                .declaration_of(choice.candidate.target)
                .ok_or(ResolutionError::InvalidStorage)?;
            let mut root = target;
            while let Some(owner) = self
                .storage
                .declaration(root)
                .ok_or(ResolutionError::InvalidStorage)?
                .owner
            {
                root = owner;
            }
            let name = self
                .effective_name_id(root)
                .ok_or(ResolutionError::InvalidStorage)?;
            let scope = choice.namespace.and_then(|n| self.declaration_of(n.symbol));
            let mut found = Vec::new();
            lookup_lexical_into(
                &self.storage.declarations,
                &ResolutionIndexes {
                    direct_names: &direct,
                    exported_names: &direct,
                    effective_imports: Some(&effective),
                    exported_imports: Some(&exported),
                    inherited_names: Some(&resolution.inherited_names),
                },
                scope,
                name,
                LookupTarget {
                    domain: DeclarationDomain::Any,
                    excluded: None,
                    excludes_anonymous_performs: false,
                    first_scope: FirstScopePolicy::OwnedThenInherited,
                },
                &mut found,
                &mut ResolutionWork::default(),
            )?;
            found.sort_unstable();
            found.dedup();
            if found != [root] {
                return Ok(None);
            }
        }
        for (index, before) in self.resolution.outcomes.iter().enumerate() {
            if replace && intended.iter().any(|(id, _)| id.index() == index) {
                continue;
            }
            let after = resolution.outcomes[index];
            let same = match (*before, after) {
                (ResolutionStatus::Resolved(a), ResolutionStatus::Resolved(b)) => a == b,
                (ResolutionStatus::Ambiguous(a), ResolutionStatus::Ambiguous(b)) => {
                    self.resolution.ambiguous_candidates(a) == resolution.ambiguous_candidates(b)
                }
                (ResolutionStatus::Unresolved, ResolutionStatus::Resolved(target)) => {
                    let r = &references[index];
                    self.reference_kind_violation(r.kind, r.source, target)
                        .is_none()
                }
                (ResolutionStatus::Unresolved, ResolutionStatus::Unresolved)
                | (ResolutionStatus::Unsupported, ResolutionStatus::Unsupported) => true,
                _ => false,
            };
            if !same {
                return Ok(None);
            }
        }
        Ok(Some(resolution))
    }
}
