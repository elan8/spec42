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






- 8.3.7.2 AttributeDefinition
  - `validateAttributeDefinitionFeatures`

- 8.3.7.3 AttributeUsage
  - `validateAttributeUsageFeatures`
  - `validateAttributeUsageIsReference`

- 8.3.8.2 EnumerationDefinition
  - `validateEnumerationDefinitionIsVariation`

- 8.3.9.2 EventOccurrenceUsage
  - `validateEventOccurrenceUsageIsReference`
  - `validateEventOccurrenceUsageReference`

- 8.3.9.4 OccurrenceUsage
  - `validateOccurrenceUsageIndividualDefinition`
  - `validateOccurrenceUsageIndividualUsage`
  - `validateOccurrenceUsageIsPortion`
  - `validateOccurrenceUsagePortionKind`

- 8.3.11.3 PartUsage
  - `validatePartUsagePartDefinition`

- 8.3.12.2 ConjugatedPortDefinition
  - `validateConjugatedPortDefinitionConjugatedPortDefinitionIsEmpty`
  - `validateConjugatedPortDefinitionOriginalPortDefinition`

- 8.3.12.5 PortDefinition
  - `validatePortDefinitionConjugatedPortDefinition`
  - `validatePortDefinitionOwnedUsagesNotComposite`

- 8.3.12.6 PortUsage
  - `validatePortUsageIsReference`
  - `validatePortUsageNestedUsagesNotComposite`

- 8.3.13.3 ConnectionDefinition
  - `validateConnectionDefinitionIsSufficient`

- 8.3.16.2 FlowDefinition
  - `validateFlowDefinitionFlowEnds`

- 8.3.17.2 AcceptActionUsage
  - `validateAcceptActionUsageParameters`

- 8.3.17.5 AssignmentActionUsage
  - `validateAssignmentActionUsage`
  - `validateAssignmentActionUsageReferent`

- 8.3.17.6 ControlNode
  - `validateControlNodeIncomingSuccessions`
  - `validateControlNodeIsComposite`
  - `validateControlNodeOutgoingSuccessions`
  - `validateControlNodeOwningType`

- 8.3.17.7 DecisionNode
  - `validateDecisionNodeIncomingSuccessions`
  - `validateDecisionNodeOutgoingSuccessions`

- 8.3.17.8 ForkNode
  - `validateForkNodeIncomingSuccessions`

- 8.3.17.9 ForLoopActionUsage
  - `validateForLoopActionUsageLoopVariable`
  - `validateForLoopActionUsageParameters`

- 8.3.17.10 IfActionUsage
  - `validateIfActionUsageParameters`

- 8.3.17.11 JoinNode
  - `validateJoinNodeOutgoingSuccessions`

- 8.3.17.13 MergeNode
  - `validateMergeNodeIncomingSuccessions`
  - `validateMergeNodeOutgoingSuccessions`

- 8.3.17.14 PerformActionUsage
  - `validatePerformActionUsageReference`

- 8.3.17.15 SendActionUsage
  - `validateSendActionParameters`

- 8.3.17.17 TriggerInvocationExpression
  - `validateTriggerInvocationExpressionAfterArgument`
  - `validateTriggerInvocationExpressionAtArgument`
  - `validateTriggerInvocationExpressionWhenArgument`

- 8.3.17.19 WhileLoopActionUsage
  - `validateWhileLoopActionUsage`

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

