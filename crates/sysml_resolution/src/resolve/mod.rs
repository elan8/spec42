//! Phase 3: name resolution, run to convergence under an explicit bound.

pub(crate) mod effective_types;
pub(crate) mod implied;
pub(crate) mod library_seed;
pub(crate) mod names;
pub(crate) mod results;

use crate::lower::facts::AuthoredReference;
use crate::lower::facts::Declaration;
use crate::lower::facts::DeclarationFacts;
use crate::lower::facts::MembershipRecord;
use crate::lower::facts::RelationshipFlags;
use crate::lower::intern::SymbolPathArena;
use crate::model::DeclarationId;
use crate::model::DeclarationKind;
use crate::model::ReferenceKind;
use crate::model::SymbolPathId;
use crate::requirement_query::RequirementDerivedFactCollection;
use crate::resolve::implied::detect_cyclic_alias_bindings;
use crate::resolve::implied::synthesize_implied_alias_bindings;
use crate::resolve::implied::synthesize_implied_redefinitions;
use crate::resolve::implied::LibrarySpecializationAnchorFacts;
use crate::resolve::names::build_direct_name_index;
use crate::resolve::names::build_effective_import_indexes;
use crate::resolve::names::build_inherited_name_index;
use crate::resolve::names::extend_inherited_names_with_effective_types;
use crate::resolve::names::lookup_lexical_into;
use crate::resolve::names::CandidateRange;
use crate::resolve::names::LookupTarget;
use crate::resolve::names::MembershipIndex;
use crate::resolve::names::NameIndex;
use crate::resolve::results::ResolutionError;
use crate::resolve::results::ResolutionResults;
use crate::resolve::results::ResolutionStatus;
use crate::resolve::results::ResolutionWork;
use crate::resolve::results::SolverStatus;
use crate::DefinitionUsageDerivedKind;
use crate::MembershipRole;
use crate::RequirementConstraintKind;

pub(crate) trait ResolutionReferenceFact {
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

pub(crate) fn resolve_dense<R: ResolutionReferenceFact>(
    declarations: &[Declaration],
    declaration_facts: Option<&[DeclarationFacts]>,
    memberships: &[MembershipRecord],
    paths: &SymbolPathArena,
    references: &[R],
    seed: Option<&[ResolutionStatus]>,
) -> Result<(NameIndex, NameIndex, MembershipIndex, ResolutionResults), ResolutionError> {
    let supported_import_count = references
        .iter()
        .filter(|reference| supported_import_domain(*reference).is_some())
        .count();
    // Each effective edge is drawn from a finite declaration/import product. The deliberately
    // conservative bound makes failure explicit without assuming acyclic imports or relying on a
    // machine-dependent timeout.
    let pass_limit = declarations
        .len()
        .checked_mul(supported_import_count.saturating_add(1))
        .and_then(|limit| limit.checked_add(1))
        .ok_or(ResolutionError::Capacity)?
        .max(1);
    resolve_dense_with_limit(
        declarations,
        declaration_facts,
        memberships,
        paths,
        references,
        pass_limit,
        seed,
    )
}

pub(crate) fn resolve_dense_with_limit<R: ResolutionReferenceFact>(
    declarations: &[Declaration],
    declaration_facts: Option<&[DeclarationFacts]>,
    memberships: &[MembershipRecord],
    paths: &SymbolPathArena,
    references: &[R],
    pass_limit: usize,
    seed: Option<&[ResolutionStatus]>,
) -> Result<(NameIndex, NameIndex, MembershipIndex, ResolutionResults), ResolutionError> {
    let membership_records = memberships;
    let memberships = MembershipIndex::build(declarations, memberships)?;
    let direct_names = build_direct_name_index(declarations, declaration_facts, None)?;
    let exported_names =
        build_direct_name_index(declarations, declaration_facts, Some(&memberships))?;
    let mut outcomes = vec![ResolutionStatus::Unsupported; references.len()];
    // A settled library's outcomes are installed before the first pass and its references are then
    // left out of every slot list below, so no pass re-evaluates them. They are still *read* --
    // by the name, import and inheritance indexes each pass rebuilds -- which is what makes the
    // workspace resolve against a library that is already settled rather than against nothing.
    let settled = seed.map_or(0, <[ResolutionStatus]>::len);
    if let Some(seed) = seed {
        outcomes[..settled].copy_from_slice(seed);
    }
    let all_import_slots: Vec<usize> = references
        .iter()
        .enumerate()
        .filter_map(|(index, reference)| supported_import_domain(reference).map(|_| index))
        .collect();
    let import_slots: Vec<usize> = all_import_slots
        .iter()
        .copied()
        .filter(|index| *index >= settled)
        .collect();
    // Subclassification is resolved first because the ancestor-scoped inherited-member lookup used
    // by FeatureTyping is built directly from settled Subclassification outcomes; splitting the two
    // kinds avoids depending on source order between an owned specialization and an owned typing
    // reference within the same document.
    let subclass_slots: Vec<usize> = references
        .iter()
        .enumerate()
        .filter(|(index, _)| *index >= settled)
        .filter_map(|(index, reference)| {
            // The four KerML type-relationship kinds join this pass rather than getting their own:
            // each names a `Type` through the same lexical lookup a `specializes` clause uses, and
            // none of them reads inherited scope. They stay distinct `ReferenceKind`s so the
            // published relationship never collapses into a specialization.
            matches!(
                reference.kind(),
                ReferenceKind::Subclassification
                    | ReferenceKind::Conjugation
                    | ReferenceKind::Unioning
                    | ReferenceKind::Intersecting
                    | ReferenceKind::Differencing
                    | ReferenceKind::Disjoining
            )
            .then_some(index)
        })
        .collect();
    let typing_slots: Vec<usize> = references
        .iter()
        .enumerate()
        .filter(|(index, _)| *index >= settled)
        .filter_map(|(index, reference)| {
            matches!(
                reference.kind(),
                ReferenceKind::FeatureTyping | ReferenceKind::TypeFeaturing
            )
            .then_some(index)
        })
        .collect();
    // A metadata annotation's target (`@Safety{...}`'s `Safety`) must be a type -- specifically a
    // metadata def -- exactly like `FeatureTyping`, so `MetadataAnnotation` joins the same
    // `DeclarationDomain::Type`/ancestor-scoped lexical lookup as `typing_slots` below, kept as its
    // own slot list (and its own `ReferenceKind`) purely so the annotation relationship stays
    // distinct from ordinary typing/specialization in query output.
    // A filter condition's `@Name` metadata-classification test (`ReferenceKind::
    // FilterMetadataTest`, e.g. `filter @Safety;`'s `Safety`) names a metadata def exactly like a
    // `MetadataAnnotation` target, so it joins the same `DeclarationDomain::Type` slot list rather
    // than a separate one -- both resolve through the identical lexical lookup fixed point, kept
    // as a distinct `ReferenceKind` purely so filter and annotation relationships stay distinct in
    // query output.
    let metadata_annotation_slots: Vec<usize> = references
        .iter()
        .enumerate()
        .filter(|(index, _)| *index >= settled)
        .filter_map(|(index, reference)| {
            matches!(
                reference.kind(),
                ReferenceKind::MetadataAnnotation
                    | ReferenceKind::FilterMetadataTest
                    | ReferenceKind::AcceptPayloadType
                    | ReferenceKind::TypeCheckTarget
                    | ReferenceKind::MetaCastTarget
                    | ReferenceKind::FlowPayloadType
            )
            .then_some(index)
        })
        .collect();
    // `Subsetting` (`:>`) and `Redefinition` (`:>>`) targets are always features, never types, so
    // they resolve against `DeclarationDomain::Any` -- like `AliasBinding`/`ConnectorEnd` -- rather
    // than joining the Subclassification/FeatureTyping `Type` domain pass. They do read the same
    // ancestor-scoped `inherited_names` index as `FeatureTyping` (built from settled
    // Subclassification outcomes below), because both explicit redefinition (`:>> status = ...;`)
    // and subsetting of an inherited feature must be able to reach a same-named or differently-named
    // feature owned by an ancestor, not just a directly owned member.
    // `ReferenceSubsetting` (`::>`/`references`), `CrossSubsetting` (`crosses`), a feature's
    // `intersects` clause and `FeatureInverting` (`inverse of`) all name a feature through the
    // same lexical lookup a `subsets` clause uses, so they share this pass; each keeps its own
    // `ReferenceKind` so the published relationship never collapses into a plain subsetting.
    let subsetting_slots: Vec<usize> = references
        .iter()
        .enumerate()
        .filter(|(index, _)| *index >= settled)
        .filter_map(|(index, reference)| {
            (matches!(
                reference.kind(),
                ReferenceKind::Subsetting
                    | ReferenceKind::References
                    | ReferenceKind::Crosses
                    | ReferenceKind::Intersects
                    | ReferenceKind::FeatureInverting
            ) && !reference.flags().dotted)
                .then_some(index)
        })
        .collect();
    let redefinition_slots: Vec<usize> = references
        .iter()
        .enumerate()
        .filter(|(index, _)| *index >= settled)
        .filter_map(|(index, reference)| {
            (reference.kind() == ReferenceKind::Redefinition && !reference.flags().dotted)
                .then_some(index)
        })
        .collect();
    // An alias target can be any element (not just a Type), so `AliasBinding` resolves against
    // `DeclarationDomain::Any` rather than joining the Subclassification/FeatureTyping `Type`
    // domain passes; it does not read inherited scope either, so it can settle alongside
    // Subclassification, independently of the ancestor closures built below.
    let alias_slots: Vec<usize> = references
        .iter()
        .enumerate()
        .filter(|(index, _)| *index >= settled)
        .filter_map(|(index, reference)| {
            (reference.kind() == ReferenceKind::AliasBinding).then_some(index)
        })
        .collect();
    // A connector end can reference any feature (not just a Type), exactly like an alias target,
    // so `ConnectorEnd` resolves against `DeclarationDomain::Any` alongside `AliasBinding` rather
    // than joining the Subclassification/FeatureTyping `Type` domain passes; it does not read
    // inherited scope either.
    let connector_end_slots: Vec<usize> = references
        .iter()
        .enumerate()
        .filter(|(index, _)| *index >= settled)
        .filter_map(|(index, reference)| {
            (reference.kind() == ReferenceKind::ConnectorEnd).then_some(index)
        })
        .collect();
    // A succession end (`first`/`then` in a `FirstStmt`) can reference any owned action feature
    // (not just a Type), exactly like a connector end, so `Succession` resolves against
    // `DeclarationDomain::Any` alongside `ConnectorEnd` rather than joining the Subclassification/
    // FeatureTyping `Type` domain passes; it does not read inherited scope either.
    let succession_slots: Vec<usize> = references
        .iter()
        .enumerate()
        .filter(|(index, _)| *index >= settled)
        .filter_map(|(index, reference)| {
            (reference.kind() == ReferenceKind::Succession).then_some(index)
        })
        .collect();
    // Entry/do/exit action bindings, a state's initial-state (`then`) target, and a constraint/
    // calc expression's feature-reference operands (`ExpressionOperand`) can each reference any
    // owned feature (not just a Type), exactly like `Succession`, so they resolve against
    // `DeclarationDomain::Any` alongside it rather than joining the Subclassification/FeatureTyping
    // `Type` domain passes; none of them read inherited scope either.
    // A `satisfy <requirement> by <element>;` statement's source/target operands can each
    // reference any owned feature (not just a Type), exactly like `Succession`/`TransitionSource`,
    // so they join `state_binding_slots`'s `DeclarationDomain::Any` pass rather than the
    // Subclassification/FeatureTyping `Type` domain passes.
    // An `Expression::Invocation`/`Constructor` callee (`InvocationCallee`) can likewise name any
    // owned feature (a calc/function) or a type (a constructor), not just a Type, so it joins this
    // same `DeclarationDomain::Any` pass.
    let state_binding_slots: Vec<usize> = references
        .iter()
        .enumerate()
        .filter(|(index, _)| *index >= settled)
        .filter_map(|(index, reference)| {
            matches!(
                reference.kind(),
                ReferenceKind::EntryActionBinding
                    | ReferenceKind::DoActionBinding
                    | ReferenceKind::ExitActionBinding
                    | ReferenceKind::InitialState
                    | ReferenceKind::ExpressionOperand
                    | ReferenceKind::TransitionSource
                    | ReferenceKind::TransitionTarget
                    | ReferenceKind::TransitionTrigger
                    | ReferenceKind::TransitionEffect
                    | ReferenceKind::SatisfySource
                    | ReferenceKind::SatisfyTarget
                    | ReferenceKind::BindSource
                    | ReferenceKind::BindTarget
                    | ReferenceKind::IncludeUseCase
                    | ReferenceKind::ViewExpose
                    | ReferenceKind::InvocationCallee
                    | ReferenceKind::ThenTarget
                    | ReferenceKind::AcceptVia
                    | ReferenceKind::SendTarget
                    | ReferenceKind::TerminateTarget
                    | ReferenceKind::FlowSource
                    | ReferenceKind::FlowTarget
                    | ReferenceKind::StakeholderTarget
                    | ReferenceKind::PurposeTarget
                    | ReferenceKind::VerifyRequirementTarget
                    | ReferenceKind::AssignTarget
                    | ReferenceKind::DependencyClient
                    | ReferenceKind::DependencySupplier
                    | ReferenceKind::PerformParameterTarget
                    | ReferenceKind::FeatureChaining
            )
            // A dotted `chains a.b` is a `FeatureChain`, resolved hop by hop below.
            .then(|| {
                !(reference.kind() == ReferenceKind::FeatureChaining && reference.flags().dotted)
            })
            .unwrap_or(false)
            .then_some(index)
        })
        .collect();
    // `MemberAccessOperand` (a dotted feature-chain access, e.g. `t.bead`, `f.a`) must run after
    // effective typing has settled and `inherited_names` has been extended with effective-type
    // entries below (`extend_inherited_names_with_effective_types`), because its interior segments are looked
    // up as members of each hop's resolved *type*, not the hop's own declaration -- exactly the
    // index that extension builds. It cannot join `state_binding_slots` above, which runs before
    // that extension exists.
    let member_access_slots: Vec<usize> = references
        .iter()
        .enumerate()
        .filter(|(index, _)| *index >= settled)
        .filter_map(|(index, reference)| {
            // A dotted subsetting-family target (`subsets a.b`, `crosses a.b`, `inverse of
            // a.b`, `chains a.b`) is a KerML `FeatureChain`, walked hop by hop exactly like a
            // member access; its `ReferenceKind` stays the authored relationship.
            (matches!(
                reference.kind(),
                ReferenceKind::MemberAccessOperand
                    | ReferenceKind::AllocateSource
                    | ReferenceKind::AllocateTarget
            ) || (reference.flags().dotted
                && matches!(
                    reference.kind(),
                    ReferenceKind::Subsetting
                        | ReferenceKind::References
                        | ReferenceKind::Crosses
                        | ReferenceKind::Redefinition
                        | ReferenceKind::FeatureInverting
                        | ReferenceKind::FeatureChaining
                )))
            .then_some(index)
        })
        .collect();
    let mut work = ResolutionWork {
        direct_index_entries: u64::try_from(direct_names.candidates.len())
            .map_err(|_| ResolutionError::Capacity)?,
        ..ResolutionWork::default()
    };
    let mut effective_imports = NameIndex::build(Vec::new())?;
    let mut exported_imports = NameIndex::build(Vec::new())?;
    let mut ambiguous_candidates = Vec::new();
    let mut candidates = Vec::new();
    let mut next_candidates = Vec::new();
    let mut converged = false;

    for _ in 0..pass_limit {
        work.passes = work
            .passes
            .checked_add(1)
            .ok_or(ResolutionError::Capacity)?;
        ambiguous_candidates.clear();
        for index in import_slots.iter().copied() {
            work.import_evaluations = work
                .import_evaluations
                .checked_add(1)
                .ok_or(ResolutionError::Capacity)?;
            let reference = &references[index];
            outcomes[index] = resolve_reference(
                declarations,
                paths,
                reference,
                supported_import_domain(reference).ok_or(ResolutionError::InvalidStorage)?,
                ResolutionIndexes {
                    direct_names: &direct_names,
                    exported_names: &exported_names,
                    effective_imports: Some(&effective_imports),
                    exported_imports: Some(&exported_imports),
                    inherited_names: None,
                },
                ResolutionScratch {
                    ambiguous_candidates: &mut ambiguous_candidates,
                    candidates: &mut candidates,
                    next_candidates: &mut next_candidates,
                    work: &mut work,
                },
            )?;
        }
        let (next_effective_imports, next_exported_imports) = build_effective_import_indexes(
            declarations,
            &memberships,
            references,
            &all_import_slots,
            &exported_names,
            &exported_imports,
            &outcomes,
        )?;
        if next_effective_imports == effective_imports && next_exported_imports == exported_imports
        {
            effective_imports = next_effective_imports;
            exported_imports = next_exported_imports;
            converged = true;
            break;
        }
        effective_imports = next_effective_imports;
        exported_imports = next_exported_imports;
    }

    let mut solver_status = if converged {
        SolverStatus::Converged
    } else {
        for index in import_slots
            .iter()
            .chain(&subclass_slots)
            .chain(&typing_slots)
            .chain(&state_binding_slots)
            .chain(&subsetting_slots)
            .chain(&redefinition_slots)
            .chain(&alias_slots)
            .chain(&connector_end_slots)
            .chain(&succession_slots)
            .chain(&metadata_annotation_slots)
            .chain(&member_access_slots)
            .copied()
        {
            outcomes[index] = ResolutionStatus::NonConverged;
        }
        ambiguous_candidates.clear();
        SolverStatus::NonConverged
    };

    let mut inherited_names = NameIndex::build(Vec::new())?;
    if converged {
        for index in subclass_slots.iter().copied() {
            work.downstream_evaluations = work
                .downstream_evaluations
                .checked_add(1)
                .ok_or(ResolutionError::Capacity)?;
            outcomes[index] = resolve_reference(
                declarations,
                paths,
                &references[index],
                DeclarationDomain::Type,
                ResolutionIndexes {
                    direct_names: &direct_names,
                    exported_names: &exported_names,
                    effective_imports: Some(&effective_imports),
                    exported_imports: Some(&exported_imports),
                    inherited_names: None,
                },
                ResolutionScratch {
                    ambiguous_candidates: &mut ambiguous_candidates,
                    candidates: &mut candidates,
                    next_candidates: &mut next_candidates,
                    work: &mut work,
                },
            )?;
        }

        for index in alias_slots.iter().copied() {
            work.downstream_evaluations = work
                .downstream_evaluations
                .checked_add(1)
                .ok_or(ResolutionError::Capacity)?;
            outcomes[index] = resolve_reference(
                declarations,
                paths,
                &references[index],
                DeclarationDomain::Any,
                ResolutionIndexes {
                    direct_names: &direct_names,
                    exported_names: &exported_names,
                    effective_imports: Some(&effective_imports),
                    exported_imports: Some(&exported_imports),
                    inherited_names: None,
                },
                ResolutionScratch {
                    ambiguous_candidates: &mut ambiguous_candidates,
                    candidates: &mut candidates,
                    next_candidates: &mut next_candidates,
                    work: &mut work,
                },
            )?;
        }

        for index in connector_end_slots.iter().copied() {
            work.downstream_evaluations = work
                .downstream_evaluations
                .checked_add(1)
                .ok_or(ResolutionError::Capacity)?;
            outcomes[index] = resolve_reference(
                declarations,
                paths,
                &references[index],
                DeclarationDomain::Any,
                ResolutionIndexes {
                    direct_names: &direct_names,
                    exported_names: &exported_names,
                    effective_imports: Some(&effective_imports),
                    exported_imports: Some(&exported_imports),
                    inherited_names: None,
                },
                ResolutionScratch {
                    ambiguous_candidates: &mut ambiguous_candidates,
                    candidates: &mut candidates,
                    next_candidates: &mut next_candidates,
                    work: &mut work,
                },
            )?;
        }

        for index in succession_slots.iter().copied() {
            work.downstream_evaluations = work
                .downstream_evaluations
                .checked_add(1)
                .ok_or(ResolutionError::Capacity)?;
            outcomes[index] = resolve_reference(
                declarations,
                paths,
                &references[index],
                DeclarationDomain::Any,
                ResolutionIndexes {
                    direct_names: &direct_names,
                    exported_names: &exported_names,
                    effective_imports: Some(&effective_imports),
                    exported_imports: Some(&exported_imports),
                    inherited_names: None,
                },
                ResolutionScratch {
                    ambiguous_candidates: &mut ambiguous_candidates,
                    candidates: &mut candidates,
                    next_candidates: &mut next_candidates,
                    work: &mut work,
                },
            )?;
        }

        for index in state_binding_slots.iter().copied() {
            work.downstream_evaluations = work
                .downstream_evaluations
                .checked_add(1)
                .ok_or(ResolutionError::Capacity)?;
            outcomes[index] = resolve_reference(
                declarations,
                paths,
                &references[index],
                DeclarationDomain::Any,
                ResolutionIndexes {
                    direct_names: &direct_names,
                    exported_names: &exported_names,
                    effective_imports: Some(&effective_imports),
                    exported_imports: Some(&exported_imports),
                    inherited_names: None,
                },
                ResolutionScratch {
                    ambiguous_candidates: &mut ambiguous_candidates,
                    candidates: &mut candidates,
                    next_candidates: &mut next_candidates,
                    work: &mut work,
                },
            )?;
        }

        // Alias bindings form a functional graph (each alias has at most one outgoing edge, its
        // own resolved target). A cycle (`alias A for B; alias B for A;`) is detected explicitly,
        // bounded by declaration count, and published as a typed `NonConverged` outcome on each
        // implicated alias's own `AliasBinding` reference -- mirroring the Subclassification
        // ancestor-closure cycle handling above -- rather than looping or panicking.
        let cyclic_alias_sources =
            detect_cyclic_alias_bindings(declarations, references, &outcomes)?;
        for index in alias_slots.iter().copied() {
            if cyclic_alias_sources.contains(&references[index].source()) {
                outcomes[index] = ResolutionStatus::NonConverged;
            }
        }

        // Ancestor-scoped inherited-member lookup is built once here, over the now-settled
        // Subclassification outcomes above, as its own bounded fixed point: diamond ancestry
        // (Left -> Base and Right -> Base) is visited once per declaration because the closure is a
        // set, and a specialization cycle is detected explicitly rather than looped forever.
        let (ancestor_closures, cyclic_ancestry) =
            build_ancestor_closures(declarations, references, &outcomes)?;
        inherited_names =
            build_inherited_name_index(declarations, &direct_names, &ancestor_closures)?;

        for index in typing_slots.iter().copied() {
            work.downstream_evaluations = work
                .downstream_evaluations
                .checked_add(1)
                .ok_or(ResolutionError::Capacity)?;
            let reference = &references[index];
            if owner_chain_is_cyclic(declarations, reference.source(), &cyclic_ancestry)? {
                outcomes[index] = ResolutionStatus::NonConverged;
                continue;
            }
            outcomes[index] = resolve_reference(
                declarations,
                paths,
                reference,
                DeclarationDomain::Type,
                ResolutionIndexes {
                    direct_names: &direct_names,
                    exported_names: &exported_names,
                    effective_imports: Some(&effective_imports),
                    exported_imports: Some(&exported_imports),
                    inherited_names: Some(&inherited_names),
                },
                ResolutionScratch {
                    ambiguous_candidates: &mut ambiguous_candidates,
                    candidates: &mut candidates,
                    next_candidates: &mut next_candidates,
                    work: &mut work,
                },
            )?;
        }

        for index in metadata_annotation_slots.iter().copied() {
            work.downstream_evaluations = work
                .downstream_evaluations
                .checked_add(1)
                .ok_or(ResolutionError::Capacity)?;
            let reference = &references[index];
            if owner_chain_is_cyclic(declarations, reference.source(), &cyclic_ancestry)? {
                outcomes[index] = ResolutionStatus::NonConverged;
                continue;
            }
            outcomes[index] = resolve_reference(
                declarations,
                paths,
                reference,
                DeclarationDomain::Type,
                ResolutionIndexes {
                    direct_names: &direct_names,
                    exported_names: &exported_names,
                    effective_imports: Some(&effective_imports),
                    exported_imports: Some(&exported_imports),
                    inherited_names: Some(&inherited_names),
                },
                ResolutionScratch {
                    ambiguous_candidates: &mut ambiguous_candidates,
                    candidates: &mut candidates,
                    next_candidates: &mut next_candidates,
                    work: &mut work,
                },
            )?;
        }

        // Effective Feature typing includes types inherited through Subsetting and Redefinition,
        // not only directly-authored FeatureTyping. Since those relationships themselves use the
        // effective member scope, settle the two together at a deterministic pass barrier. This
        // is required for nested redefinitions such as `:>> quantityDimension { :>>
        // quantityPowerFactors = ...; }`: the outer redefinition first inherits
        // `QuantityDimension`, which makes the inner redefinition's target visible on the next
        // pass. The canonical effective-type derivation is shared with the published type index.
        let relationship_pass_limit = declarations
            .len()
            .checked_add(1)
            .ok_or(ResolutionError::Capacity)?;
        let mut relationships_converged = false;
        let mut relationship_scopes = std::collections::BTreeSet::new();
        for index in subsetting_slots.iter().chain(&redefinition_slots) {
            let mut owner = declarations
                .get(references[*index].source().index())
                .ok_or(ResolutionError::InvalidStorage)?
                .owner;
            while let Some(current) = owner {
                relationship_scopes.insert(current);
                owner = declarations
                    .get(current.index())
                    .ok_or(ResolutionError::InvalidStorage)?
                    .owner;
            }
        }
        // Qualified relationship targets can traverse an intermediate Feature's effective member
        // scope (`Shape::faces::edges`). Include every declaration whose authored long or short
        // name occurs in a non-final segment. This is the dependency-complete subset needed by
        // these relationship queries; building effective member entries for every declaration on
        // every settlement pass would make unrelated model width part of their cost.
        let mut relationship_intermediate_names = std::collections::BTreeSet::new();
        for index in subsetting_slots.iter().chain(&redefinition_slots) {
            let (segments, _) = paths
                .get(references[*index].path())
                .ok_or(ResolutionError::InvalidStorage)?;
            relationship_intermediate_names.extend(
                segments
                    .get(..segments.len().saturating_sub(1))
                    .unwrap_or_default()
                    .iter()
                    .copied(),
            );
        }
        for (index, declaration) in declarations.iter().enumerate() {
            let long_name_matches = declaration
                .name
                .is_some_and(|name| relationship_intermediate_names.contains(&name));
            let short_name_matches = declaration_facts
                .and_then(|facts| facts.get(index))
                .and_then(|facts| facts.short_name)
                .is_some_and(|name| relationship_intermediate_names.contains(&name));
            if long_name_matches || short_name_matches {
                relationship_scopes.insert(
                    DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?,
                );
            }
        }
        let mut settled_effective_types = None;
        for _ in 0..relationship_pass_limit {
            let previous = subsetting_slots
                .iter()
                .chain(&redefinition_slots)
                .filter_map(|index| match outcomes[*index] {
                    ResolutionStatus::Resolved(target) => Some((
                        references[*index].source(),
                        target,
                        references[*index].kind(),
                    )),
                    _ => None,
                })
                .collect::<Vec<_>>();
            let edges = references
                .iter()
                .enumerate()
                .filter_map(|(index, reference)| {
                    let ResolutionStatus::Resolved(target) = outcomes[index] else {
                        return None;
                    };
                    Some((reference.source(), target, reference.kind()))
                });
            let effective_types =
                crate::resolve::effective_types::derive_effective_types_from_edges(
                    declarations.len(),
                    edges,
                )?;
            inherited_names = extend_inherited_names_with_effective_types(
                &direct_names,
                build_inherited_name_index(declarations, &direct_names, &ancestor_closures)?,
                &effective_types,
                declarations.len(),
                Some(&relationship_scopes),
            )?;
            settled_effective_types = Some(effective_types);

            for index in subsetting_slots.iter().chain(&redefinition_slots).copied() {
                work.downstream_evaluations = work
                    .downstream_evaluations
                    .checked_add(1)
                    .ok_or(ResolutionError::Capacity)?;
                let reference = &references[index];
                if owner_chain_is_cyclic(declarations, reference.source(), &cyclic_ancestry)? {
                    outcomes[index] = ResolutionStatus::NonConverged;
                    continue;
                }
                outcomes[index] = resolve_reference(
                    declarations,
                    paths,
                    reference,
                    DeclarationDomain::Any,
                    ResolutionIndexes {
                        direct_names: &direct_names,
                        exported_names: &exported_names,
                        effective_imports: Some(&effective_imports),
                        exported_imports: Some(&exported_imports),
                        inherited_names: Some(&inherited_names),
                    },
                    ResolutionScratch {
                        ambiguous_candidates: &mut ambiguous_candidates,
                        candidates: &mut candidates,
                        next_candidates: &mut next_candidates,
                        work: &mut work,
                    },
                )?;
            }
            let current = subsetting_slots
                .iter()
                .chain(&redefinition_slots)
                .filter_map(|index| match outcomes[*index] {
                    ResolutionStatus::Resolved(target) => Some((
                        references[*index].source(),
                        target,
                        references[*index].kind(),
                    )),
                    _ => None,
                })
                .collect::<Vec<_>>();
            if current == previous {
                relationships_converged = true;
                break;
            }
        }
        if !relationships_converged {
            for index in subsetting_slots
                .iter()
                .chain(&redefinition_slots)
                .chain(&member_access_slots)
                .copied()
            {
                outcomes[index] = ResolutionStatus::NonConverged;
            }
            converged = false;
            solver_status = SolverStatus::NonConverged;
        } else {
            inherited_names = extend_inherited_names_with_effective_types(
                &direct_names,
                build_inherited_name_index(declarations, &direct_names, &ancestor_closures)?,
                settled_effective_types
                    .as_ref()
                    .ok_or(ResolutionError::InvalidStorage)?,
                declarations.len(),
                None,
            )?;
            // Dotted member access consumes the final effective member scope.
            for index in member_access_slots.iter().copied() {
                work.downstream_evaluations = work
                    .downstream_evaluations
                    .checked_add(1)
                    .ok_or(ResolutionError::Capacity)?;
                outcomes[index] = resolve_member_access_reference(
                    declarations,
                    paths,
                    &references[index],
                    ResolutionIndexes {
                        direct_names: &direct_names,
                        exported_names: &exported_names,
                        effective_imports: Some(&effective_imports),
                        exported_imports: Some(&exported_imports),
                        inherited_names: Some(&inherited_names),
                    },
                    ResolutionScratch {
                        ambiguous_candidates: &mut ambiguous_candidates,
                        candidates: &mut candidates,
                        next_candidates: &mut next_candidates,
                        work: &mut work,
                    },
                )?;
            }
        }
    }
    #[cfg(test)]
    {
        work.effective_index_entries = u64::try_from(effective_imports.candidates.len())
            .map_err(|_| ResolutionError::Capacity)?;
    }

    // The implied-redefinition family reads only settled Subclassification outcomes and the
    // already-built owner-scoped direct-name index above; it is not mutually recursive with the
    // import/typing fixed point above and therefore runs once, after that fixed point settles,
    // rather than joining it as another per-pass family.
    let implied_relationships = if converged {
        let mut implied = synthesize_implied_redefinitions(
            declarations,
            membership_records,
            references,
            &direct_names,
            &outcomes,
        )?
        .into_vec();
        let cyclic_alias_sources =
            detect_cyclic_alias_bindings(declarations, references, &outcomes)?;
        implied.extend(
            synthesize_implied_alias_bindings(
                declarations,
                references,
                &outcomes,
                &cyclic_alias_sources,
            )?
            .into_vec(),
        );
        implied.into_boxed_slice()
    } else {
        Box::default()
    };

    Ok((
        direct_names,
        effective_imports,
        memberships,
        ResolutionResults {
            outcomes: outcomes.into_boxed_slice(),
            ambiguous_candidates: ambiguous_candidates.into_boxed_slice(),
            inherited_names,
            solver_status,
            implied_relationships,
            library_specialization_anchors: LibrarySpecializationAnchorFacts::default(),
            semantic_metadata_projections: Box::default(),
            semantic_metadata_projection_status: Default::default(),
            select_expression_projection_status: Default::default(),
            index_expression_projection_status: Default::default(),
            index_expression_array_anchor: None,
            constructor_expression_projection_status: Default::default(),
            constructor_expression_projections: Box::default(),
            constructor_expression_specialization_status: Default::default(),
            constructor_expression_anchor: None,
            feature_chain_expression_specialization_status: Default::default(),
            feature_chain_expression_projections: Box::default(),
            feature_reference_expression_status: Default::default(),
            feature_reference_expression_projections: Box::default(),
            invocation_expression_projection_status: Default::default(),
            invocation_expression_projections: Box::default(),
            #[cfg(test)]
            work,
        },
    ))
}

/// Computes, for every declaration, the transitive set of Subclassification ancestors reached
/// through resolved Subclassification outcomes, as a bounded fixed point over the immutable
/// previous-pass state (mirroring the effective-import fixed point above): each pass reads the
/// prior complete closure array and writes a fresh next array, swapped only at the pass barrier.
///
/// A diamond (`Diamond :> Left, Right` where both specialize `Base`) naturally dedups to a single
/// `Base` entry because each declaration's closure is a set. A specialization cycle is detected
/// explicitly: if a declaration's own closure would come to include itself, that declaration is
/// reported as cyclic instead of being handed an ever-growing or self-referential ancestor list.
/// Because each pass only ever unions in previously-discovered ancestors, the closure array is
/// bounded by the total declaration count and this loop is guaranteed to reach a fixed point well
/// inside `declarations.len() + 1` passes even in the presence of a cycle; it never spins forever.
pub(crate) type AncestorClosures = (
    Vec<Box<[DeclarationId]>>,
    std::collections::BTreeSet<DeclarationId>,
);

pub(crate) fn build_ancestor_closures<R: ResolutionReferenceFact>(
    declarations: &[Declaration],
    references: &[R],
    outcomes: &[ResolutionStatus],
) -> Result<AncestorClosures, ResolutionError> {
    let mut direct_parents: Vec<std::collections::BTreeSet<DeclarationId>> =
        vec![Default::default(); declarations.len()];
    for (index, reference) in references.iter().enumerate() {
        if reference.kind() != ReferenceKind::Subclassification {
            continue;
        }
        if let ResolutionStatus::Resolved(target) = outcomes[index] {
            let slot = direct_parents
                .get_mut(reference.source().index())
                .ok_or(ResolutionError::InvalidStorage)?;
            slot.insert(target);
        }
    }

    let mut closure = direct_parents.clone();
    let pass_limit = declarations
        .len()
        .checked_add(1)
        .ok_or(ResolutionError::Capacity)?;
    for _ in 0..pass_limit {
        let mut next = closure.clone();
        let mut changed = false;
        for (index, parents) in direct_parents.iter().enumerate() {
            for parent in parents {
                for ancestor in
                    std::iter::once(*parent).chain(closure[parent.index()].iter().copied())
                {
                    if next[index].insert(ancestor) {
                        changed = true;
                    }
                }
            }
        }
        closure = next;
        if !changed {
            break;
        }
    }

    let mut cyclic = std::collections::BTreeSet::new();
    let mut closures = Vec::with_capacity(declarations.len());
    for (index, ancestors) in closure.into_iter().enumerate() {
        let id = DeclarationId::from_index(index).map_err(|_| ResolutionError::Capacity)?;
        if ancestors.contains(&id) {
            cyclic.insert(id);
            closures.push(Box::default());
        } else {
            closures.push(ancestors.into_iter().collect::<Vec<_>>().into_boxed_slice());
        }
    }
    Ok((closures, cyclic))
}

/// True when `source`'s owning namespace chain passes through a declaration whose Subclassification
/// ancestry was found to be cyclic. A FeatureTyping reference owned (directly or via an enclosing
/// scope) by such a declaration cannot have its inherited scope computed and is published as an
/// explicit `NonConverged` outcome rather than silently falling back to local/import-only lookup or
/// looping.
pub(crate) fn owner_chain_is_cyclic(
    declarations: &[Declaration],
    source: DeclarationId,
    cyclic_ancestry: &std::collections::BTreeSet<DeclarationId>,
) -> Result<bool, ResolutionError> {
    let mut owner = declarations
        .get(source.index())
        .ok_or(ResolutionError::InvalidStorage)?
        .owner;
    while let Some(current) = owner {
        if cyclic_ancestry.contains(&current) {
            return Ok(true);
        }
        owner = declarations
            .get(current.index())
            .ok_or(ResolutionError::InvalidStorage)?
            .owner;
    }
    Ok(false)
}

pub(crate) fn supported_import_domain(
    reference: &impl ResolutionReferenceFact,
) -> Option<DeclarationDomain> {
    match reference.kind() {
        // A view's `expose` names any member, so it resolves through the same lookup an ordinary
        // reference does rather than the import domains.
        ReferenceKind::ViewExpose => Some(DeclarationDomain::Any),
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
        | ReferenceKind::FilterImport
        | ReferenceKind::FeatureTyping
        | ReferenceKind::TypeFeaturing
        | ReferenceKind::FeatureChaining
        | ReferenceKind::Subclassification
        | ReferenceKind::Conjugation
        | ReferenceKind::FeatureInverting
        | ReferenceKind::Subsetting
        | ReferenceKind::Redefinition
        | ReferenceKind::References
        | ReferenceKind::Crosses
        | ReferenceKind::Intersects
        | ReferenceKind::Unioning
        | ReferenceKind::Intersecting
        | ReferenceKind::Differencing
        | ReferenceKind::Disjoining
        | ReferenceKind::AliasBinding
        | ReferenceKind::ConnectorEnd
        | ReferenceKind::Succession
        | ReferenceKind::EntryActionBinding
        | ReferenceKind::DoActionBinding
        | ReferenceKind::ExitActionBinding
        | ReferenceKind::InitialState
        | ReferenceKind::ExpressionOperand
        | ReferenceKind::TransitionSource
        | ReferenceKind::TransitionTarget
        | ReferenceKind::TransitionTrigger
        | ReferenceKind::TransitionEffect
        | ReferenceKind::MetadataAnnotation
        | ReferenceKind::FilterMetadataTest
        | ReferenceKind::SatisfySource
        | ReferenceKind::SatisfyTarget
        | ReferenceKind::AllocateSource
        | ReferenceKind::AllocateTarget
        | ReferenceKind::BindSource
        | ReferenceKind::BindTarget
        | ReferenceKind::IncludeUseCase
        | ReferenceKind::MemberAccessOperand
        | ReferenceKind::InvocationCallee
        | ReferenceKind::ThenTarget
        | ReferenceKind::AcceptVia
        | ReferenceKind::SendTarget
        | ReferenceKind::AcceptPayloadType
        | ReferenceKind::TerminateTarget
        | ReferenceKind::FlowSource
        | ReferenceKind::FlowTarget
        | ReferenceKind::TypeCheckTarget
        | ReferenceKind::MetaCastTarget
        | ReferenceKind::StakeholderTarget
        | ReferenceKind::PurposeTarget
        | ReferenceKind::VerifyRequirementTarget
        | ReferenceKind::AssignTarget
        | ReferenceKind::DependencyClient
        | ReferenceKind::DependencySupplier
        | ReferenceKind::PerformParameterTarget
        | ReferenceKind::FlowPayloadType => None,
    }
}

/// Exact Definition/Usage metaclass applicability over the lowering's canonical kind fact.
///
/// These are deliberately not display-name checks. A new lowered declaration kind makes this
/// policy reviewable here rather than silently appearing in a broad derived collection.
pub(crate) fn definition_usage_source_matches(metaclass: &str, kind: DeclarationKind) -> bool {
    match metaclass {
        "Definition" => matches!(
            kind,
            DeclarationKind::PartDefinition
                | DeclarationKind::AttributeDefinition
                | DeclarationKind::EnumerationDefinition
                | DeclarationKind::RequirementDefinition
                | DeclarationKind::PortDefinition
                | DeclarationKind::ItemDefinition
                | DeclarationKind::ActionDefinition
                | DeclarationKind::StateDefinition
                | DeclarationKind::MetadataDefinition
                | DeclarationKind::ConnectionDefinition
                | DeclarationKind::OccurrenceDefinition
                | DeclarationKind::AnalysisCaseDefinition
                | DeclarationKind::InterfaceDefinition
                | DeclarationKind::ViewDefinition
                | DeclarationKind::CaseDefinition
                | DeclarationKind::VerificationCaseDefinition
                | DeclarationKind::UseCaseDefinition
                | DeclarationKind::ViewpointDefinition
                | DeclarationKind::RenderingDefinition
                | DeclarationKind::AllocationDefinition
                | DeclarationKind::FlowDefinition
                | DeclarationKind::ConstraintDefinition
                | DeclarationKind::ConcernDefinition
                | DeclarationKind::CalcDefinition
                | DeclarationKind::ClassDefinition
                | DeclarationKind::ExtendedDefinition
                | DeclarationKind::IndividualDefinition
        ),
        "Usage" => is_usage_declaration(kind),
        _ => false,
    }
}

pub(crate) fn requirement_derived_source_matches(metaclass: &str, kind: DeclarationKind) -> bool {
    matches!(
        (metaclass, kind),
        (
            "RequirementDefinition",
            DeclarationKind::RequirementDefinition
        ) | ("RequirementUsage", DeclarationKind::RequirementUsage)
    )
}

pub(crate) fn requirement_derived_membership_role(
    collection: RequirementDerivedFactCollection,
) -> Option<MembershipRole> {
    use RequirementDerivedFactCollection as Collection;
    match collection {
        Collection::DefinitionActorParameter | Collection::UsageActorParameter => {
            Some(MembershipRole::Actor)
        }
        Collection::DefinitionSubjectParameter | Collection::UsageSubjectParameter => {
            Some(MembershipRole::Subject)
        }
        Collection::DefinitionRequiredConstraint | Collection::UsageRequiredConstraint => Some(
            MembershipRole::RequirementConstraint(RequirementConstraintKind::Requirement),
        ),
        Collection::DefinitionAssumedConstraint | Collection::UsageAssumedConstraint => Some(
            MembershipRole::RequirementConstraint(RequirementConstraintKind::Assumption),
        ),
        Collection::DefinitionFramedConcern | Collection::UsageFramedConcern => {
            Some(MembershipRole::FramedConcern)
        }
        Collection::DefinitionText | Collection::UsageText => None,
    }
}

/// The currently lowered SysML `Usage` subtypes. This is a semantic applicability table, not a
/// fallback: the private declaration kind is the canonical lowering fact and the match remains
/// exhaustive enough to force review when a new usage form is admitted.
pub(crate) fn is_usage_declaration(kind: DeclarationKind) -> bool {
    matches!(
        kind,
        DeclarationKind::PartUsage
            | DeclarationKind::AttributeUsage
            | DeclarationKind::EnumerationUsage
            | DeclarationKind::EnumerationLiteral
            | DeclarationKind::RequirementUsage
            | DeclarationKind::PortUsage
            | DeclarationKind::ItemUsage
            | DeclarationKind::ActionUsage
            | DeclarationKind::AcceptActionUsage
            | DeclarationKind::SendActionUsage
            | DeclarationKind::TerminateActionUsage
            | DeclarationKind::StateUsage
            | DeclarationKind::MetadataUsage
            | DeclarationKind::ConnectionUsage
            | DeclarationKind::OccurrenceUsage
            | DeclarationKind::AnalysisCaseUsage
            | DeclarationKind::ViewUsage
            | DeclarationKind::CaseUsage
            | DeclarationKind::VerificationCaseUsage
            | DeclarationKind::UseCaseUsage
            | DeclarationKind::ViewpointUsage
            | DeclarationKind::RenderingUsage
            | DeclarationKind::InterfaceUsage
            | DeclarationKind::ConstraintUsage
            | DeclarationKind::AssertConstraintUsage
            | DeclarationKind::AssumeConstraintUsage
            | DeclarationKind::RequireConstraintUsage
            | DeclarationKind::ConcernUsage
            | DeclarationKind::CalcUsage
            | DeclarationKind::ReferenceUsage
            | DeclarationKind::DefaultReferenceUsage
            | DeclarationKind::ExtendedUsage
            | DeclarationKind::ParameterUsage
            | DeclarationKind::SubjectUsage
            | DeclarationKind::PerformActionUsage
            | DeclarationKind::Transition
            | DeclarationKind::Satisfy
            | DeclarationKind::Allocate
            | DeclarationKind::Bind
            | DeclarationKind::Assign
            | DeclarationKind::While
            | DeclarationKind::Loop
            | DeclarationKind::If
            | DeclarationKind::ForLoop
            | DeclarationKind::ForLoopVariable
            | DeclarationKind::Decide
            | DeclarationKind::Merge
            | DeclarationKind::Fork
            | DeclarationKind::Join
            | DeclarationKind::ThenContinuation
            | DeclarationKind::Flow
            | DeclarationKind::StakeholderUsage
            | DeclarationKind::RequirementActor
            | DeclarationKind::CaseActor
            | DeclarationKind::Frame
            | DeclarationKind::VerifyRequirement
            | DeclarationKind::BareConnect
            | DeclarationKind::EntryActionBinding
            | DeclarationKind::DoActionBinding
            | DeclarationKind::ExitActionBinding
            | DeclarationKind::InitialState
            | DeclarationKind::FinalState
            | DeclarationKind::PerformParameterBinding
    )
}

/// Whether a lowered declaration is a KerML `Feature` (including every SysML usage). This is the
/// canonical metamodel-category predicate used by both resolution synthesis and structural checks.
pub(crate) fn is_feature_declaration(kind: DeclarationKind) -> bool {
    is_usage_declaration(kind)
        || matches!(
            crate::model::element_kind::element_kind(kind),
            sysml_contract::ElementKind::Feature
                | sysml_contract::ElementKind::Step
                | sysml_contract::ElementKind::Expression
                | sysml_contract::ElementKind::BooleanExpression
                | sysml_contract::ElementKind::Connector
                | sysml_contract::ElementKind::BindingConnector
                | sysml_contract::ElementKind::Invariant
        )
}

/// Whether a lowered Usage is an `ActionUsage` or one of its concrete SysML subtypes.
///
/// This is a metamodel predicate over canonical declaration kinds. It deliberately includes the
/// state and case families (`StateUsage :> ActionUsage`, `CaseUsage :> ActionUsage`) and the
/// syntax-specific action nodes that do not pass through the ordinary `action` production.
pub(crate) fn is_action_usage_declaration(kind: DeclarationKind) -> bool {
    kind.is_action_usage()
}

/// Whether a direct canonical member satisfies the exact selected `selectByKind` result.
pub(crate) fn definition_usage_candidate_matches(
    collection: DefinitionUsageDerivedKind,
    kind: DeclarationKind,
) -> bool {
    use DefinitionUsageDerivedKind as Collection;
    match collection {
        Collection::DefinitionOwnedAction | Collection::UsageNestedAction => matches!(
            kind,
            DeclarationKind::ActionUsage
                | DeclarationKind::AcceptActionUsage
                | DeclarationKind::SendActionUsage
                | DeclarationKind::TerminateActionUsage
                | DeclarationKind::PerformActionUsage
                | DeclarationKind::EntryActionBinding
                | DeclarationKind::DoActionBinding
                | DeclarationKind::ExitActionBinding
        ),
        Collection::DefinitionOwnedAllocation | Collection::UsageNestedAllocation => {
            matches!(kind, DeclarationKind::Allocate)
        }
        Collection::DefinitionOwnedAnalysisCase | Collection::UsageNestedAnalysisCase => {
            matches!(kind, DeclarationKind::AnalysisCaseUsage)
        }
        Collection::DefinitionOwnedAttribute | Collection::UsageNestedAttribute => {
            matches!(kind, DeclarationKind::AttributeUsage)
        }
        Collection::DefinitionOwnedCalculation | Collection::UsageNestedCalculation => {
            matches!(kind, DeclarationKind::CalcUsage)
        }
        Collection::DefinitionOwnedCase | Collection::UsageNestedCase => {
            matches!(kind, DeclarationKind::CaseUsage)
        }
        Collection::DefinitionOwnedConcern | Collection::UsageNestedConcern => {
            matches!(kind, DeclarationKind::ConcernUsage | DeclarationKind::Frame)
        }
        Collection::DefinitionOwnedConnection | Collection::UsageNestedConnection => matches!(
            kind,
            DeclarationKind::ConnectionUsage | DeclarationKind::BareConnect | DeclarationKind::Bind
        ),
        Collection::DefinitionOwnedConstraint | Collection::UsageNestedConstraint => matches!(
            kind,
            DeclarationKind::ConstraintUsage
                | DeclarationKind::AssertConstraintUsage
                | DeclarationKind::AssumeConstraintUsage
                | DeclarationKind::RequireConstraintUsage
        ),
        Collection::DefinitionOwnedEnumeration | Collection::UsageNestedEnumeration => matches!(
            kind,
            DeclarationKind::EnumerationUsage | DeclarationKind::EnumerationLiteral
        ),
        Collection::DefinitionOwnedFlow | Collection::UsageNestedFlow => {
            matches!(kind, DeclarationKind::Flow)
        }
        // The pinned XMI body selects ReferenceUsage for both `ownedInterface` and
        // `nestedInterface`; the exact body, not the property suffix, is authoritative.
        Collection::DefinitionOwnedInterface
        | Collection::UsageNestedInterface
        | Collection::DefinitionOwnedReference
        | Collection::UsageNestedReference => matches!(
            kind,
            DeclarationKind::ReferenceUsage
                | DeclarationKind::DefaultReferenceUsage
                | DeclarationKind::ParameterUsage
                | DeclarationKind::SubjectUsage
                | DeclarationKind::PerformParameterBinding
        ),
        Collection::DefinitionOwnedItem | Collection::UsageNestedItem => {
            matches!(kind, DeclarationKind::ItemUsage)
        }
        Collection::DefinitionOwnedMetadata | Collection::UsageNestedMetadata => {
            matches!(kind, DeclarationKind::MetadataUsage)
        }
        Collection::DefinitionOwnedOccurrence | Collection::UsageNestedOccurrence => {
            matches!(kind, DeclarationKind::OccurrenceUsage)
        }
        Collection::DefinitionOwnedPart | Collection::UsageNestedPart => matches!(
            kind,
            DeclarationKind::PartUsage
                | DeclarationKind::StakeholderUsage
                | DeclarationKind::RequirementActor
                | DeclarationKind::CaseActor
        ),
        Collection::DefinitionOwnedPort | Collection::UsageNestedPort => {
            matches!(kind, DeclarationKind::PortUsage)
        }
        Collection::DefinitionOwnedRendering | Collection::UsageNestedRendering => {
            matches!(kind, DeclarationKind::RenderingUsage)
        }
        Collection::DefinitionOwnedRequirement | Collection::UsageNestedRequirement => matches!(
            kind,
            DeclarationKind::RequirementUsage | DeclarationKind::VerifyRequirement
        ),
        Collection::DefinitionOwnedState | Collection::UsageNestedState => {
            matches!(
                kind,
                DeclarationKind::StateUsage | DeclarationKind::FinalState
            )
        }
        Collection::DefinitionOwnedTransition | Collection::UsageNestedTransition => {
            matches!(kind, DeclarationKind::Transition)
        }
        Collection::DefinitionOwnedUsage | Collection::UsageNestedUsage => {
            is_usage_declaration(kind)
        }
        Collection::DefinitionOwnedUseCase | Collection::UsageNestedUseCase => {
            matches!(kind, DeclarationKind::UseCaseUsage)
        }
        Collection::DefinitionOwnedVerificationCase | Collection::UsageNestedVerificationCase => {
            matches!(kind, DeclarationKind::VerificationCaseUsage)
        }
        Collection::DefinitionOwnedView | Collection::UsageNestedView => {
            matches!(kind, DeclarationKind::ViewUsage)
        }
        Collection::DefinitionOwnedViewpoint | Collection::UsageNestedViewpoint => {
            matches!(kind, DeclarationKind::ViewpointUsage)
        }
        // These variants are returned before candidate selection by the rule-scoped method.
        Collection::DefinitionDirectedUsage
        | Collection::DefinitionUsage
        | Collection::DefinitionVariant
        | Collection::DefinitionVariantMembership
        | Collection::UsageDirectedUsage
        | Collection::UsageIsReference
        | Collection::UsageMayTimeVary
        | Collection::UsageUsage
        | Collection::UsageVariant
        | Collection::UsageVariantMembership => false,
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum DeclarationDomain {
    Any,
    Namespace,
    Type,
}

impl DeclarationDomain {
    pub(crate) fn accepts(self, kind: DeclarationKind) -> bool {
        match self {
            Self::Any => true,
            Self::Namespace => matches!(
                kind,
                DeclarationKind::Namespace
                    | DeclarationKind::Package
                    | DeclarationKind::LibraryPackage
            ),
            // An alias is a transparent proxy: whether it is Type-domain-compatible is a property
            // of its (possibly not-yet-resolved) ultimate target, not of the alias declaration
            // itself, so it is provisionally accepted here. `synthesize_implied_alias_bindings`
            // below chases the alias's own resolved `AliasBinding` reference to publish the real
            // (implied) typing/specialization fact against the ultimate non-alias target.
            Self::Type => matches!(
                kind,
                DeclarationKind::PartDefinition
                    | DeclarationKind::AttributeDefinition
                    | DeclarationKind::EnumerationDefinition
                    | DeclarationKind::RequirementDefinition
                    | DeclarationKind::PortDefinition
                    | DeclarationKind::ItemDefinition
                    | DeclarationKind::ActionDefinition
                    | DeclarationKind::StateDefinition
                    | DeclarationKind::MetadataDefinition
                    | DeclarationKind::ConnectionDefinition
                    | DeclarationKind::InterfaceDefinition
                    | DeclarationKind::OccurrenceDefinition
                    | DeclarationKind::AnalysisCaseDefinition
                    | DeclarationKind::ViewDefinition
                    | DeclarationKind::CaseDefinition
                    | DeclarationKind::VerificationCaseDefinition
                    | DeclarationKind::UseCaseDefinition
                    | DeclarationKind::ViewpointDefinition
                    | DeclarationKind::RenderingDefinition
                    | DeclarationKind::AllocationDefinition
                    | DeclarationKind::FlowDefinition
                    | DeclarationKind::ConstraintDefinition
                    | DeclarationKind::ConcernDefinition
                    | DeclarationKind::CalcDefinition
                    | DeclarationKind::ClassDefinition
                    // The KerML type metaclasses. Every one of these is a `Type` in the
                    // metamodel, so each is a legitimate FeatureTyping/Subclassification target;
                    // without them no reference into the KerML kernel libraries can resolve, and
                    // `attribute mass : ScalarValues::Real` fails against a `datatype` that is
                    // right there in the admitted standard library. `KermlMultiplicity` is
                    // deliberately absent: `Multiplicity <: Feature`, and this domain admits
                    // definition-like types only, exactly as it already excludes SysML usages.
                    | DeclarationKind::KermlClassifier
                    | DeclarationKind::KermlStructure
                    | DeclarationKind::KermlAssociation
                    | DeclarationKind::KermlAssociationStructure
                    | DeclarationKind::KermlDataType
                    | DeclarationKind::KermlMetaclass
                    | DeclarationKind::KermlBehavior
                    | DeclarationKind::KermlFunction
                    | DeclarationKind::KermlPredicate
                    | DeclarationKind::KermlInteraction
                    | DeclarationKind::KermlType
                    | DeclarationKind::Alias
            ),
        }
    }
}

pub(crate) struct ResolutionIndexes<'a> {
    pub(crate) direct_names: &'a NameIndex,
    pub(crate) exported_names: &'a NameIndex,
    pub(crate) effective_imports: Option<&'a NameIndex>,
    pub(crate) exported_imports: Option<&'a NameIndex>,
    /// Ancestor-scoped inherited-member lookup, keyed by `(child type declaration, name)`. Absent
    /// for the Subclassification pass itself (it is built from Subclassification's own settled
    /// outcomes) and present for reference kinds resolved afterward, such as FeatureTyping.
    pub(crate) inherited_names: Option<&'a NameIndex>,
}

pub(crate) struct ResolutionScratch<'a> {
    pub(crate) ambiguous_candidates: &'a mut Vec<DeclarationId>,
    pub(crate) candidates: &'a mut Vec<DeclarationId>,
    pub(crate) next_candidates: &'a mut Vec<DeclarationId>,
    pub(crate) work: &'a mut ResolutionWork,
}

pub(crate) fn resolve_reference<R: ResolutionReferenceFact>(
    declarations: &[Declaration],
    paths: &SymbolPathArena,
    reference: &R,
    domain: DeclarationDomain,
    indexes: ResolutionIndexes<'_>,
    scratch: ResolutionScratch<'_>,
) -> Result<ResolutionStatus, ResolutionError> {
    let (segments, rooted) = paths
        .get(reference.path())
        .ok_or(ResolutionError::InvalidStorage)?;
    let source = declarations
        .get(reference.source().index())
        .ok_or(ResolutionError::InvalidStorage)?;
    // KerML Redefinition relates a feature to a *different* feature, so the redefining feature is
    // not in its own redefinition scope: the Pilot's `KerMLScope` excludes it, and
    // Redefinition excludes owned
    // first-scope candidates".
    //
    // Without this, `feature annotatedElement : Element[1..*] redefines annotatedElement;` -- the
    // shape the KerML abstract-syntax library uses throughout to narrow an inherited feature --
    // finds itself in its owner's owned tier, and that tier shadows the inherited one the author
    // meant. The feature then specializes itself, which makes its whole conformance hierarchy
    // cyclic and every conformance question about it unanswerable.
    //
    // Deliberately Redefinition alone. The anonymous subsetting shorthand `:> annotatedElement :
    // T;` has the same shape, but a metadata definition commonly authors several of them, and the
    // lowering gives each the subsetted feature's name as its *declared* name. Excluding only the
    // reference's own source there does not reach the inherited feature; it reaches the sibling
    // shorthand, so two of them resolve to each other and the self-loop becomes a two-cycle. That
    // is a lowering defect -- an unnamed member must not acquire a declared name -- and it is
    // recorded in planning/UPSTREAM_PARSER_GAPS.md rather than compensated for here.
    let excluded = (reference.kind() == ReferenceKind::Redefinition).then(|| reference.source());
    scratch.candidates.clear();
    scratch.next_candidates.clear();
    if rooted {
        record_lookup(scratch.work)?;
        scratch
            .candidates
            .extend_from_slice(indexes.exported_names.candidates(None, segments[0]));
    } else {
        // When the first segment is also the last (a plain unqualified reference), the winning
        // precedence tier's candidates *are* the final resolution target, so domain compatibility
        // is applied per tier below (an incompatible-domain local binding still shadows a
        // compatible outer/imported one. When more
        // segments follow, this first segment denotes an intermediate namespace/type owner, not
        // the reference's final target, so no domain filtering applies here: `Any` accepts
        // everything and the tier logic degrades to plain name-presence shadowing.
        let first_segment_domain = if segments.len() == 1 {
            domain
        } else {
            DeclarationDomain::Any
        };
        let lexical_scope = if reference.kind() == ReferenceKind::ExpressionOperand {
            Some(reference.source())
        } else {
            source.owner
        };
        lookup_lexical_into(
            declarations,
            &indexes,
            lexical_scope,
            segments[0],
            LookupTarget {
                domain: first_segment_domain,
                excluded,
            },
            scratch.candidates,
            scratch.work,
        )?;
    }

    for segment in &segments[1..] {
        scratch.next_candidates.clear();
        for candidate in scratch.candidates.iter().copied() {
            record_lookup(scratch.work)?;
            // KerML qualified-name traversal continues through the previous segment's visible
            // memberships. For a Type, those are its directly owned memberships followed by its
            // inherited memberships; a direct name shadows the inherited tier. The canonical
            // `inherited_names` index also carries effective-type members for Features, so paths
            // such as `ConeOrCylinder::faces::edges` traverse `faces` to the members of its type.
            // Import fallback remains last for owners reached through an imported namespace.
            let direct = indexes.direct_names.candidates(Some(candidate), *segment);
            if !direct.is_empty() {
                scratch.next_candidates.extend_from_slice(direct);
                continue;
            }
            let inherited = indexes
                .inherited_names
                .map_or(&[][..], |names| names.candidates(Some(candidate), *segment));
            if !inherited.is_empty() {
                scratch.next_candidates.extend_from_slice(inherited);
                continue;
            }
            if let Some(imports) = indexes.exported_imports {
                record_lookup(scratch.work)?;
                scratch
                    .next_candidates
                    .extend_from_slice(imports.candidates(Some(candidate), *segment));
            }
        }
        scratch.next_candidates.sort_unstable();
        scratch.next_candidates.dedup();
        std::mem::swap(scratch.candidates, scratch.next_candidates);
    }
    // The non-rooted, single-segment case already applied domain-aware tier selection inside
    // `lookup_lexical_into` above (including the incompatible-domain shadow rule), so re-filtering
    // here would silently discard a shadowed candidate and regress it to `Unresolved`. Every other
    // case (rooted lookups, and the final segment of a multi-segment qualified name) still needs
    // this domain check applied to its result.
    if rooted || segments.len() > 1 {
        scratch.candidates.retain(|candidate| {
            declarations
                .get(candidate.index())
                .is_some_and(|declaration| domain.accepts(declaration.kind))
        });
    }
    // A qualified or absolute path can name the specializing feature just as an unqualified one
    // can, and it is no more well-formed for having been spelled out.
    if let Some(excluded) = excluded {
        scratch
            .candidates
            .retain(|candidate| *candidate != excluded);
    }
    status_from_candidates(scratch.candidates, scratch.ambiguous_candidates)
}

/// Resolves a `ReferenceKind::MemberAccessOperand` reference (a dotted feature-chain access, e.g.
/// `t.bead`, `f.a`, chained `a.b.c`): the reference's `path()` is the flattened chain built by
/// `SemanticModelBuilder::push_member_access_reference` -- the root segment(s) followed by each
/// subsequent dotted member segment, all in one `SymbolPathId`.
///
/// The root segment resolves through `DeclarationDomain::Any` lexical lookup, beginning in the
/// source declaration's owned scope and then walking its enclosing-namespace chain. This lets a
/// constraint/calc expression reach its own parameters without changing the enclosing-scope result
/// for sources that own no declarations. Each subsequent segment first uses the previous segment's
/// directly owned members, then its effective-type members when no direct member shadows them.
/// The latter reuses `inherited_names`, which by the time this runs has already been extended from
/// canonical effective typing
/// (`extend_inherited_names_with_effective_types`): `inherited_names.candidates(Some(usage), name)`
/// contains every name owned (directly or by inheritance) by every effective type of `usage`.
///
/// If the root segment does not resolve to exactly one candidate, or any subsequent segment finds
/// no direct-or-typed member, the whole chain publishes `Unresolved`; if an intermediate or final
/// segment is ambiguous, the whole chain publishes `Ambiguous`. The chain never partially resolves
/// -- there is no way to publish "the first two segments resolved but the third didn't."
pub(crate) fn resolve_member_access_reference<R: ResolutionReferenceFact>(
    declarations: &[Declaration],
    paths: &SymbolPathArena,
    reference: &R,
    indexes: ResolutionIndexes<'_>,
    scratch: ResolutionScratch<'_>,
) -> Result<ResolutionStatus, ResolutionError> {
    let (segments, rooted) = paths
        .get(reference.path())
        .ok_or(ResolutionError::InvalidStorage)?;
    if rooted {
        // A dotted member-access chain is never `::`-absolute; defensive, not reachable from the
        // lowering side today.
        return Ok(ResolutionStatus::Unsupported);
    }
    let _source = declarations
        .get(reference.source().index())
        .ok_or(ResolutionError::InvalidStorage)?;
    scratch.candidates.clear();
    scratch.next_candidates.clear();
    lookup_lexical_into(
        declarations,
        &indexes,
        Some(reference.source()),
        segments[0],
        // A member-access chain is never a Redefinition reference.
        LookupTarget {
            domain: DeclarationDomain::Any,
            excluded: None,
        },
        scratch.candidates,
        scratch.work,
    )?;
    for segment in &segments[1..] {
        if scratch.candidates.len() != 1 {
            // Zero candidates (Unresolved) or more than one (Ambiguous) -- either way the chain
            // cannot continue past this hop; publish that outcome directly rather than silently
            // dropping the remaining segments.
            return status_from_candidates(scratch.candidates, scratch.ambiguous_candidates);
        }
        let candidate = scratch.candidates[0];
        scratch.next_candidates.clear();
        record_lookup(scratch.work)?;
        let direct = indexes.direct_names.candidates(Some(candidate), *segment);
        if !direct.is_empty() {
            scratch.next_candidates.extend_from_slice(direct);
        } else if let Some(inherited) = indexes.inherited_names {
            scratch
                .next_candidates
                .extend_from_slice(inherited.candidates(Some(candidate), *segment));
        }
        scratch.next_candidates.sort_unstable();
        scratch.next_candidates.dedup();
        std::mem::swap(scratch.candidates, scratch.next_candidates);
    }
    status_from_candidates(scratch.candidates, scratch.ambiguous_candidates)
}

pub(crate) fn record_lookup(work: &mut ResolutionWork) -> Result<(), ResolutionError> {
    work.indexed_name_lookups = work
        .indexed_name_lookups
        .checked_add(1)
        .ok_or(ResolutionError::Capacity)?;
    Ok(())
}

pub(crate) fn status_from_candidates(
    candidates: &[DeclarationId],
    ambiguous_candidates: &mut Vec<DeclarationId>,
) -> Result<ResolutionStatus, ResolutionError> {
    match candidates {
        [] => Ok(ResolutionStatus::Unresolved),
        [candidate] => Ok(ResolutionStatus::Resolved(*candidate)),
        _ => {
            let start = ambiguous_candidates.len();
            ambiguous_candidates.extend_from_slice(candidates);
            Ok(ResolutionStatus::Ambiguous(CandidateRange::from_bounds(
                start,
                ambiguous_candidates.len(),
            )?))
        }
    }
}
