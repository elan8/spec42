# Normative validation-rule inventory

Active work list for encoding every normative KerML and SysML validation constraint as a snapshot
fixture under `tests/snapshots/validation`. The snapshot corpus is the durable coverage record;
this file holds only unprocessed rules, ambiguities, and blockers. Remove an entry when its
fixture lands.

Sources of the inventory below are the normative constraint listings of the abstract-syntax
clauses:

- OMG KerML 1.0 (formal/26-03-01), <https://www.omg.org/spec/KerML/1.0/PDF> -- 88 constraints,
  all covered.
- OMG SysML 2.0 Language (formal/26-03-02), <https://www.omg.org/spec/SysML/2.0/Language/PDF> --
  92 constraints.

Every constraint the specifications name is listed; the abstract-syntax constraint name is the
stable inventory identity, qualified by its clause number.

## Active blockers

- `8.3.2.3.3 validateAnnotationAnnotatedElementOwnership`: the publication records a
  `comment about Thing` annotation as documentation of the owning package rather than of `Thing`
  (see `tests/snapshots/validation/kerml_annotation_annotating_element.md`). The constraint itself
  has no textual violating form, so this does not block its fixture, but the annotatedElement
  question is open for the annotation clauses generally.

## Remaining rules




































### SysML 2.0 Language (formal/26-03-02)






























- 8.3.18.2 ExhibitStateUsage
  - `validateExhibitStateUsageReference`

- 8.3.18.4 StateSubactionMembership
  - `validateStateSubactionMembershipOwningType`

- 8.3.18.5 StateDefinition
  - `validateStateDefinitionParallelSubactions`
  - `validateStateDefinitionStateSubactionKind`

- 8.3.18.6 StateUsage
  - `validateStateUsageParallelSubactions`
  - `validateStateUsageStateSubactionKind`

- 8.3.18.8 TransitionFeatureMembership
  - `validateTransitionFeatureMembershipEffectAction`
  - `validateTransitionFeatureMembershipGuardExpression`
  - `validateTransitionFeatureMembershipOwningType`
  - `validateTransitionFeatureMembershipTriggerAction`

- 8.3.18.9 TransitionUsage
  - `validateTransitionUsageParameters`
  - `validateTransitionUsageSuccession`
  - `validateTransitionUsageTriggerActions`

- 8.3.20.2 AssertConstraintUsage
  - `validateAssertConstraintUsageReference`

- 8.3.21.2 ActorMembership
  - `validateActorMembershipOwningType`

- 8.3.21.5 FramedConcernMembership
  - `validateFramedConcernMembershipConstraintKind`

- 8.3.21.7 RequirementConstraintMembership
  - `validateRequirementConstraintMembershipIsComposite`
  - `validateRequirementConstraintMembershipOwningType`

- 8.3.21.8 RequirementDefinition
  - `validateRequirementDefinitionOnlyOneSubject`
  - `validateRequirementDefinitionSubjectParameterPosition`

- 8.3.21.9 RequirementUsage
  - `validateRequirementUsageOnlyOneSubject`
  - `validateRequirementUsageSubjectParameterPosition`

- 8.3.21.10 SatisfyRequirementUsage
  - `validateSatisfyRequirementUsageReference`

- 8.3.21.11 SubjectMembership
  - `validateSubjectMembershipOwningType`

- 8.3.21.12 StakeholderMembership
  - `validateStakeholderMembershipOwningType`

- 8.3.22.2 CaseDefinition
  - `validateCaseDefinitionOnlyOneObjective`
  - `validateCaseDefinitionOnlyOneSubject`
  - `validateCaseDefinitionSubjectParameterPosition`

- 8.3.22.3 CaseUsage
  - `validateCaseUsageOnlyOneObjective`
  - `validateCaseUsageOnlyOneSubject`
  - `validateCaseUsageSubjectParameterPosition`

- 8.3.22.4 ObjectiveMembership
  - `validateObjectiveMembershipIsComposite`
  - `validateObjectiveMembershipOwningType`

- 8.3.24.2 RequirementVerificationMembership
  - `validateRequirementVerificationMembershipKind`
  - `validateRequirementVerificationMembershipOwningType`

- 8.3.25.2 IncludeUseCaseUsage
  - `validateIncludeUseCaseUsageReference`

- 8.3.26.2 Expose
  - `validateExposeIsImportAll`
  - `validateExposeOwningNamespace`
  - `validateExposeVisibility`

- 8.3.26.7 ViewDefinition
  - `validateViewDefinitionOnlyOneViewRendering`

- 8.3.26.10 ViewRenderingMembership
  - `validateViewRenderingMembershipOwningType`

- 8.3.26.11 ViewUsage
  - `validateViewUsageOnlyOneViewRendering`

