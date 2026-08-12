//! Private batch resolution over the dense canonical semantic storage.

use super::*;
use crate::semantic::text_span::{TextPosition, TextRange};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ResolutionError {
    Capacity,
    InvalidStorage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct NameKey {
    owner: Option<DeclarationId>,
    name: SymbolId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CandidateRange {
    start: u32,
    len: u32,
}

impl CandidateRange {
    fn from_bounds(start: usize, end: usize) -> Result<Self, ResolutionError> {
        let start = u32::try_from(start).map_err(|_| ResolutionError::Capacity)?;
        let len = u32::try_from(
            end.checked_sub(start as usize)
                .ok_or(ResolutionError::InvalidStorage)?,
        )
        .map_err(|_| ResolutionError::Capacity)?;
        start.checked_add(len).ok_or(ResolutionError::Capacity)?;
        Ok(Self { start, len })
    }

    fn slice<'a, T>(&self, values: &'a [T]) -> Option<&'a [T]> {
        let end = self.start.checked_add(self.len)?;
        values.get(self.start as usize..end as usize)
    }
}

#[derive(Debug)]
struct NameIndex {
    keys: Box<[NameKey]>,
    ranges: Box<[CandidateRange]>,
    candidates: Box<[DeclarationId]>,
}

impl NameIndex {
    fn build(mut entries: Vec<(NameKey, DeclarationId)>) -> Result<Self, ResolutionError> {
        entries.sort_unstable();
        entries.dedup();

        let mut keys = Vec::new();
        let mut ranges = Vec::new();
        let mut candidates = Vec::new();
        keys.try_reserve(entries.len())
            .map_err(|_| ResolutionError::Capacity)?;
        ranges
            .try_reserve(entries.len())
            .map_err(|_| ResolutionError::Capacity)?;
        candidates
            .try_reserve(entries.len())
            .map_err(|_| ResolutionError::Capacity)?;

        let mut cursor = 0;
        while cursor < entries.len() {
            let key = entries[cursor].0;
            let start = candidates.len();
            while cursor < entries.len() && entries[cursor].0 == key {
                candidates.push(entries[cursor].1);
                cursor += 1;
            }
            keys.push(key);
            ranges.push(CandidateRange::from_bounds(start, candidates.len())?);
        }

        Ok(Self {
            keys: keys.into_boxed_slice(),
            ranges: ranges.into_boxed_slice(),
            candidates: candidates.into_boxed_slice(),
        })
    }

    fn candidates(&self, owner: Option<DeclarationId>, name: SymbolId) -> &[DeclarationId] {
        let key = NameKey { owner, name };
        let Ok(index) = self.keys.binary_search(&key) else {
            return &[];
        };
        self.ranges[index]
            .slice(&self.candidates)
            .unwrap_or_default()
    }

    fn entries_for_owner(
        &self,
        owner: Option<DeclarationId>,
    ) -> impl Iterator<Item = (SymbolId, &[DeclarationId])> {
        let start = self.keys.partition_point(|key| key.owner < owner);
        let end = self.keys.partition_point(|key| key.owner <= owner);
        self.keys[start..end]
            .iter()
            .zip(&self.ranges[start..end])
            .filter_map(|(key, range)| {
                range
                    .slice(&self.candidates)
                    .map(|candidates| (key.name, candidates))
            })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ResolutionStatus {
    Resolved(DeclarationId),
    Unresolved,
    Ambiguous(CandidateRange),
    Unsupported,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum DiagnosticOutcome {
    Resolved,
    Unresolved,
    Unsupported,
    Ambiguous {
        candidates: Box<[DiagnosticCandidate]>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DiagnosticCandidate {
    target: DeclarationId,
    kind: DeclarationKind,
    range: TextRange,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DiagnosticRecord {
    reference: AuthoredReferenceId,
    source: DeclarationId,
    kind: ReferenceKind,
    range: TextRange,
    outcome: DiagnosticOutcome,
}

#[derive(Debug)]
struct ResolutionResults {
    outcomes: Box<[ResolutionStatus]>,
    ambiguous_candidates: Box<[DeclarationId]>,
}

impl ResolutionResults {
    fn outcome(&self, id: AuthoredReferenceId) -> Option<ResolutionStatus> {
        self.outcomes.get(id.index()).copied()
    }

    fn ambiguous_candidates(&self, range: CandidateRange) -> &[DeclarationId] {
        range.slice(&self.ambiguous_candidates).unwrap_or_default()
    }
}

#[derive(Debug)]
struct ResolvedSemanticModel {
    storage: SemanticModelStorage,
    direct_names: NameIndex,
    effective_imports: NameIndex,
    resolution: ResolutionResults,
}

impl ResolvedSemanticModel {
    fn diagnostic_records(&self) -> Result<Box<[DiagnosticRecord]>, ResolutionError> {
        let mut records = Vec::with_capacity(self.storage.references.len());
        for (index, reference) in self.storage.references.iter().enumerate() {
            let reference_id =
                AuthoredReferenceId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
            let source = self
                .storage
                .declaration(reference.source)
                .ok_or(ResolutionError::InvalidStorage)?;
            let range = document_range(&self.storage, source.document, &reference.span)?;
            let outcome = match self
                .resolution
                .outcome(reference_id)
                .ok_or(ResolutionError::InvalidStorage)?
            {
                ResolutionStatus::Resolved(_) => DiagnosticOutcome::Resolved,
                ResolutionStatus::Unresolved => DiagnosticOutcome::Unresolved,
                ResolutionStatus::Unsupported => DiagnosticOutcome::Unsupported,
                ResolutionStatus::Ambiguous(candidate_range) => {
                    let mut candidates = Vec::new();
                    for target in self.resolution.ambiguous_candidates(candidate_range) {
                        let declaration = self
                            .storage
                            .declaration(*target)
                            .ok_or(ResolutionError::InvalidStorage)?;
                        candidates.push(DiagnosticCandidate {
                            target: *target,
                            kind: declaration.kind,
                            range: document_range(
                                &self.storage,
                                declaration.document,
                                &declaration.span,
                            )?,
                        });
                    }
                    DiagnosticOutcome::Ambiguous {
                        candidates: candidates.into_boxed_slice(),
                    }
                }
            };
            records.push(DiagnosticRecord {
                reference: reference_id,
                source: reference.source,
                kind: reference.kind,
                range,
                outcome,
            });
        }
        Ok(records.into_boxed_slice())
    }
}

fn document_range(
    storage: &SemanticModelStorage,
    document: DocumentId,
    span: &Span,
) -> Result<TextRange, ResolutionError> {
    let parsed = &storage
        .document(document)
        .ok_or(ResolutionError::InvalidStorage)?
        .parsed;
    let range = parsed.range(span).ok_or(ResolutionError::InvalidStorage)?;
    Ok(TextRange::new(
        TextPosition::new(
            range.start.line.saturating_sub(1),
            u32::try_from(range.start.column.saturating_sub(1))
                .map_err(|_| ResolutionError::Capacity)?,
        ),
        TextPosition::new(
            range.end.line.saturating_sub(1),
            u32::try_from(range.end.column.saturating_sub(1))
                .map_err(|_| ResolutionError::Capacity)?,
        ),
    ))
}

impl SemanticModelStorage {
    fn resolve(self) -> Result<ResolvedSemanticModel, ResolutionError> {
        let (direct_names, effective_imports, resolution) =
            resolve_dense(&self.declarations, &self.paths, &self.references)?;
        Ok(ResolvedSemanticModel {
            storage: self,
            direct_names,
            effective_imports,
            resolution,
        })
    }
}

trait ResolutionReferenceFact {
    fn source(&self) -> DeclarationId;
    fn kind(&self) -> ReferenceKind;
    fn path(&self) -> SymbolPathId;
    fn flags(&self) -> RelationshipFlags;
}

impl ResolutionReferenceFact for AuthoredReference {
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

fn resolve_dense<R: ResolutionReferenceFact>(
    declarations: &[Declaration],
    paths: &SymbolPathArena,
    references: &[R],
) -> Result<(NameIndex, NameIndex, ResolutionResults), ResolutionError> {
    let direct_names = build_direct_name_index(declarations)?;
    let mut outcomes = vec![ResolutionStatus::Unsupported; references.len()];
    let mut ambiguous_candidates = Vec::new();
    let mut candidates = Vec::new();
    let mut next_candidates = Vec::new();

    for (index, reference) in references.iter().enumerate() {
        let expected_kind = match reference.kind() {
            ReferenceKind::NamespaceImport
                if reference.flags().wildcard && !reference.flags().recursive =>
            {
                Some(DeclarationDomain::Namespace)
            }
            ReferenceKind::MembershipImport
                if !reference.flags().wildcard && !reference.flags().recursive =>
            {
                Some(DeclarationDomain::Any)
            }
            ReferenceKind::NamespaceImport
            | ReferenceKind::MembershipImport
            | ReferenceKind::FilterImport => None,
            ReferenceKind::FeatureTyping
            | ReferenceKind::Subclassification
            | ReferenceKind::Subsetting
            | ReferenceKind::Redefinition
            | ReferenceKind::References
            | ReferenceKind::Crosses
            | ReferenceKind::Intersects => continue,
        };
        if let Some(expected_kind) = expected_kind {
            outcomes[index] = resolve_reference(
                declarations,
                paths,
                reference,
                &direct_names,
                None,
                expected_kind,
                &mut ambiguous_candidates,
                &mut candidates,
                &mut next_candidates,
            )?;
        }
    }

    let effective_imports =
        build_effective_import_index(declarations, references, &direct_names, &outcomes)?;

    for (index, reference) in references.iter().enumerate() {
        let domain = match reference.kind() {
            ReferenceKind::FeatureTyping | ReferenceKind::Subclassification => {
                DeclarationDomain::Type
            }
            ReferenceKind::NamespaceImport
            | ReferenceKind::MembershipImport
            | ReferenceKind::FilterImport
            | ReferenceKind::Subsetting
            | ReferenceKind::Redefinition
            | ReferenceKind::References
            | ReferenceKind::Crosses
            | ReferenceKind::Intersects => continue,
        };
        outcomes[index] = resolve_reference(
            declarations,
            paths,
            reference,
            &direct_names,
            Some(&effective_imports),
            domain,
            &mut ambiguous_candidates,
            &mut candidates,
            &mut next_candidates,
        )?;
    }

    Ok((
        direct_names,
        effective_imports,
        ResolutionResults {
            outcomes: outcomes.into_boxed_slice(),
            ambiguous_candidates: ambiguous_candidates.into_boxed_slice(),
        },
    ))
}

#[derive(Debug, Clone, Copy)]
enum DeclarationDomain {
    Any,
    Namespace,
    Type,
}

impl DeclarationDomain {
    fn accepts(self, kind: DeclarationKind) -> bool {
        match self {
            Self::Any => true,
            Self::Namespace => matches!(
                kind,
                DeclarationKind::Namespace
                    | DeclarationKind::Package
                    | DeclarationKind::LibraryPackage
            ),
            Self::Type => matches!(
                kind,
                DeclarationKind::PartDefinition | DeclarationKind::AttributeDefinition
            ),
        }
    }
}

fn build_direct_name_index(declarations: &[Declaration]) -> Result<NameIndex, ResolutionError> {
    let mut entries = Vec::new();
    entries
        .try_reserve(declarations.len())
        .map_err(|_| ResolutionError::Capacity)?;
    for (index, declaration) in declarations.iter().enumerate() {
        if let Some(name) = declaration.name {
            entries.push((
                NameKey {
                    owner: declaration.owner,
                    name,
                },
                DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?,
            ));
        }
    }
    NameIndex::build(entries)
}

fn build_effective_import_index<R: ResolutionReferenceFact>(
    declarations: &[Declaration],
    references: &[R],
    direct_names: &NameIndex,
    outcomes: &[ResolutionStatus],
) -> Result<NameIndex, ResolutionError> {
    let mut entries = Vec::new();
    for (index, reference) in references.iter().enumerate() {
        let ResolutionStatus::Resolved(target) = outcomes[index] else {
            continue;
        };
        let import_owner = declarations
            .get(reference.source().index())
            .ok_or(ResolutionError::InvalidStorage)?
            .owner;
        match reference.kind() {
            ReferenceKind::NamespaceImport => {
                for (name, candidates) in direct_names.entries_for_owner(Some(target)) {
                    entries.extend(candidates.iter().copied().map(|candidate| {
                        (
                            NameKey {
                                owner: import_owner,
                                name,
                            },
                            candidate,
                        )
                    }));
                }
            }
            ReferenceKind::MembershipImport => {
                let declaration = declarations
                    .get(target.index())
                    .ok_or(ResolutionError::InvalidStorage)?;
                if let Some(name) = declaration.name {
                    entries.push((
                        NameKey {
                            owner: import_owner,
                            name,
                        },
                        target,
                    ));
                }
            }
            ReferenceKind::FilterImport
            | ReferenceKind::FeatureTyping
            | ReferenceKind::Subclassification
            | ReferenceKind::Subsetting
            | ReferenceKind::Redefinition
            | ReferenceKind::References
            | ReferenceKind::Crosses
            | ReferenceKind::Intersects => {}
        }
    }
    NameIndex::build(entries)
}

fn resolve_reference<R: ResolutionReferenceFact>(
    declarations: &[Declaration],
    paths: &SymbolPathArena,
    reference: &R,
    direct_names: &NameIndex,
    effective_imports: Option<&NameIndex>,
    domain: DeclarationDomain,
    ambiguous_candidates: &mut Vec<DeclarationId>,
    candidates: &mut Vec<DeclarationId>,
    next_candidates: &mut Vec<DeclarationId>,
) -> Result<ResolutionStatus, ResolutionError> {
    let (segments, rooted) = paths
        .get(reference.path())
        .ok_or(ResolutionError::InvalidStorage)?;
    let source = declarations
        .get(reference.source().index())
        .ok_or(ResolutionError::InvalidStorage)?;
    candidates.clear();
    next_candidates.clear();
    if rooted {
        candidates.extend_from_slice(direct_names.candidates(None, segments[0]));
    } else {
        lookup_lexical_into(
            declarations,
            direct_names,
            effective_imports,
            source.owner,
            segments[0],
            candidates,
        )?;
    }

    for segment in &segments[1..] {
        next_candidates.clear();
        for candidate in candidates.iter().copied() {
            next_candidates.extend_from_slice(direct_names.candidates(Some(candidate), *segment));
        }
        next_candidates.sort_unstable();
        next_candidates.dedup();
        std::mem::swap(candidates, next_candidates);
    }
    candidates.retain(|candidate| {
        declarations
            .get(candidate.index())
            .is_some_and(|declaration| domain.accepts(declaration.kind))
    });
    status_from_candidates(candidates, ambiguous_candidates)
}

fn lookup_lexical_into(
    declarations: &[Declaration],
    direct_names: &NameIndex,
    effective_imports: Option<&NameIndex>,
    mut owner: Option<DeclarationId>,
    name: SymbolId,
    candidates: &mut Vec<DeclarationId>,
) -> Result<(), ResolutionError> {
    loop {
        let direct = direct_names.candidates(owner, name);
        if !direct.is_empty() {
            candidates.extend_from_slice(direct);
            return Ok(());
        }
        if let Some(imports) = effective_imports {
            let imported = imports.candidates(owner, name);
            if !imported.is_empty() {
                candidates.extend_from_slice(imported);
                return Ok(());
            }
        }
        let Some(current) = owner else {
            return Ok(());
        };
        owner = declarations
            .get(current.index())
            .ok_or(ResolutionError::InvalidStorage)?
            .owner;
    }
}

fn status_from_candidates(
    candidates: &[DeclarationId],
    ambiguous_candidates: &mut Vec<DeclarationId>,
) -> Result<ResolutionStatus, ResolutionError> {
    match candidates {
        [] => Ok(ResolutionStatus::Unresolved),
        [candidate] => Ok(ResolutionStatus::Resolved(*candidate)),
        _ => {
            let start = ambiguous_candidates.len();
            ambiguous_candidates.extend_from_slice(&candidates);
            Ok(ResolutionStatus::Ambiguous(CandidateRange::from_bounds(
                start,
                ambiguous_candidates.len(),
            )?))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            kind,
            span: Span::dummy(),
        }
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
        paths: SymbolPathArena,
        references: Box<[TestReference]>,
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
        ResolverFixture {
            declarations: declarations.into_boxed_slice(),
            paths: paths.freeze(),
            references: references.into_boxed_slice(),
        }
    }

    fn resolve_fixture(fixture: &ResolverFixture) -> (NameIndex, NameIndex, ResolutionResults) {
        resolve_dense(&fixture.declarations, &fixture.paths, &fixture.references).unwrap()
    }

    #[test]
    fn namespace_import_populates_index_used_by_feature_typing() {
        let fixture = cross_file_fixture(false);
        let (direct_names, effective_imports, resolution) = resolve_fixture(&fixture);
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
    fn duplicate_imported_type_is_canonically_ambiguous() {
        let fixture = cross_file_fixture(true);
        let (_, _, resolution) = resolve_fixture(&fixture);
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
    fn missing_and_filtered_references_remain_explicit() {
        let mut fixture = cross_file_fixture(false);
        fixture.references[0].kind = ReferenceKind::FilterImport;
        let (_, _, resolution) = resolve_fixture(&fixture);
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(0)),
            Some(ResolutionStatus::Unsupported)
        );
        assert_eq!(
            resolution.outcome(AuthoredReferenceId(1)),
            Some(ResolutionStatus::Unresolved)
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
}
