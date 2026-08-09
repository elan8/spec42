# META
~~~ini
description=Standard Library: Systems Library/SysML
type=file
~~~
# SOURCE
~~~sysml
standard library package SysML {
	doc 
	/*
	 * This package contains a reflective KerML model of the KerML abstract syntax.
	 */
	 
	private import ScalarValues::*;
	public import Systems::*;
	
	package Systems {
		public import KerML::Kernel::*;
		
		metadata def AcceptActionUsage specializes ActionUsage {
			derived ref item receiverArgument : Expression[0..1] subsets Metadata::metadataItems;
			derived ref item payloadParameter : ReferenceUsage[1..1] subsets nestedReference, parameter subsets Metadata::metadataItems;
			derived ref item payloadArgument : Expression[0..1] subsets Metadata::metadataItems;
		}		
		
		metadata def ActionDefinition specializes Behavior, OccurrenceDefinition {
			derived ref item 'action' : ActionUsage[0..*] ordered subsets step, usage subsets Metadata::metadataItems;
		}		
		
		metadata def ActionUsage specializes Step, OccurrenceUsage {
			derived ref item actionDefinition : Behavior[0..*] ordered redefines behavior, occurrenceDefinition subsets Metadata::metadataItems;
		}		
		
		metadata def ActorMembership specializes ParameterMembership {
			derived item ownedActorParameter : PartUsage[1..1] redefines ownedMemberParameter subsets Metadata::metadataItems;
		}		
		
		metadata def AllocationDefinition specializes ConnectionDefinition {
			derived ref item 'allocation' : AllocationUsage[0..*] ordered subsets usage subsets Metadata::metadataItems;
		}		
		
		metadata def AllocationUsage specializes ConnectionUsage {
			derived ref item allocationDefinition : AllocationDefinition[0..*] ordered redefines connectionDefinition subsets Metadata::metadataItems;
		}		
		
		metadata def AnalysisCaseDefinition specializes CaseDefinition {
			derived ref item resultExpression : Expression[0..1] subsets expression, ownedFeature subsets Metadata::metadataItems;
		}		
		
		metadata def AnalysisCaseUsage specializes CaseUsage {
			derived ref item analysisCaseDefinition : AnalysisCaseDefinition[0..1] redefines caseDefinition subsets Metadata::metadataItems;
			derived ref item resultExpression : Expression[0..1] subsets ownedFeature subsets Metadata::metadataItems;
		}		
		
		metadata def AssertConstraintUsage specializes ConstraintUsage, Invariant {
			derived ref item assertedConstraint : ConstraintUsage[1..1] subsets Metadata::metadataItems;
		}		
		
		metadata def AssignmentActionUsage specializes ActionUsage {
			derived ref item targetArgument : Expression[0..1] subsets Metadata::metadataItems;
			derived ref item valueExpression : Expression[0..1] subsets Metadata::metadataItems;
			derived ref item referent : Feature[1..1] subsets member subsets Metadata::metadataItems;
		}		
		
		metadata def AttributeDefinition specializes DataType, Definition;		
		
		metadata def AttributeUsage specializes Usage {
			derived attribute isReference : Boolean[1..1] redefines isReference;
			
			derived ref item attributeDefinition : DataType[0..*] ordered redefines definition subsets Metadata::metadataItems;
		}		
		
		metadata def BindingConnectorAsUsage specializes BindingConnector, ConnectorAsUsage;		
		
		metadata def CalculationDefinition specializes Function, ActionDefinition {
			derived ref item calculation : CalculationUsage[0..*] ordered subsets 'action', expression subsets Metadata::metadataItems;
		}		
		
		metadata def CalculationUsage specializes Expression, ActionUsage {
			derived ref item calculationDefinition : Function[0..1] ordered redefines function, actionDefinition subsets Metadata::metadataItems;
		}		
		
		metadata def CaseDefinition specializes CalculationDefinition {
			derived ref item objectiveRequirement : RequirementUsage[0..1] ordered subsets usage subsets Metadata::metadataItems;
			derived ref item subjectParameter : Usage[1..1] subsets parameter, usage subsets Metadata::metadataItems;
			derived ref item actorParameter : PartUsage[0..*] ordered subsets parameter, usage subsets Metadata::metadataItems;
		}		
		
		metadata def CaseUsage specializes CalculationUsage {
			derived ref item objectiveRequirement : RequirementUsage[0..1] ordered subsets usage subsets Metadata::metadataItems;
			derived ref item caseDefinition : CaseDefinition[0..1] redefines calculationDefinition subsets Metadata::metadataItems;
			derived ref item subjectParameter : Usage[1..1] subsets parameter, usage subsets Metadata::metadataItems;
			derived ref item actorParameter : PartUsage[0..*] ordered subsets parameter, usage subsets Metadata::metadataItems;
		}		
		
		metadata def ConcernDefinition specializes RequirementDefinition;		
		
		metadata def ConcernUsage specializes RequirementUsage {
			derived ref item concernDefinition : ConcernDefinition[0..1] redefines requirementDefinition subsets Metadata::metadataItems;
		}		
		
		metadata def ConjugatedPortDefinition specializes PortDefinition {
			derived ref item originalPortDefinition : PortDefinition[1..1] redefines owningNamespace subsets Metadata::metadataItems;
			derived ref item ownedPortConjugator : PortConjugation[1..1] redefines ownedConjugator subsets Metadata::metadataItems;
		}		
		
		metadata def ConjugatedPortTyping specializes FeatureTyping {
			ref item conjugatedPortDefinition : ConjugatedPortDefinition[1..1] redefines type subsets Metadata::metadataItems;
			derived ref item portDefinition : PortDefinition[1..1] subsets Metadata::metadataItems;
		}		
		
		metadata def ConnectionDefinition specializes AssociationStructure, PartDefinition {
			attribute isSufficient : Boolean[1..1] redefines isSufficient;
			
			derived ref item connectionEnd : Usage[0..*] ordered redefines associationEnd subsets Metadata::metadataItems;
		}		
		
		metadata def ConnectionUsage specializes ConnectorAsUsage, PartUsage {
			derived ref item connectionDefinition : AssociationStructure[0..*] ordered subsets itemDefinition redefines association subsets Metadata::metadataItems;
		}		
		
		abstract metadata def ConnectorAsUsage specializes Usage, Connector;		
		
		metadata def ConstraintDefinition specializes OccurrenceDefinition, Predicate;		
		
		metadata def ConstraintUsage specializes BooleanExpression, OccurrenceUsage {
			derived ref item constraintDefinition : Predicate[0..1] redefines predicate subsets Metadata::metadataItems;
		}		
		
		abstract metadata def ControlNode specializes ActionUsage;		
		
		metadata def DecisionNode specializes ControlNode;		
		
		metadata def Definition specializes Classifier {
			attribute isVariation : Boolean[1..1];
			
			derived ref item 'variant' : Usage[0..*] subsets ownedMember subsets Metadata::metadataItems;
			derived item variantMembership : VariantMembership[0..*] subsets ownedMembership subsets Metadata::metadataItems;
			derived ref item usage : Usage[0..*] ordered subsets feature subsets Metadata::metadataItems;
			derived ref item directedUsage : Usage[0..*] ordered subsets directedFeature, usage subsets Metadata::metadataItems;
			derived ref item ownedUsage : Usage[0..*] ordered subsets ownedFeature, usage subsets Metadata::metadataItems;
			derived ref item ownedReference : ReferenceUsage[0..*] ordered subsets ownedUsage subsets Metadata::metadataItems;
			derived ref item ownedAttribute : AttributeUsage[0..*] ordered subsets ownedUsage subsets Metadata::metadataItems;
			derived ref item ownedEnumeration : EnumerationUsage[0..*] ordered subsets ownedAttribute subsets Metadata::metadataItems;
			derived ref item ownedOccurrence : OccurrenceUsage[0..*] ordered subsets ownedUsage subsets Metadata::metadataItems;
			derived ref item ownedItem : ItemUsage[0..*] ordered subsets ownedOccurrence subsets Metadata::metadataItems;
			derived ref item ownedPart : PartUsage[0..*] ordered subsets ownedItem subsets Metadata::metadataItems;
			derived ref item ownedPort : PortUsage[0..*] ordered subsets ownedUsage subsets Metadata::metadataItems;
			derived ref item ownedConnection : ConnectorAsUsage[0..*] ordered subsets ownedUsage subsets Metadata::metadataItems;
			derived ref item ownedFlow : FlowUsage[0..*] subsets ownedConnection subsets Metadata::metadataItems;
			derived ref item ownedInterface : InterfaceUsage[0..*] ordered subsets ownedConnection subsets Metadata::metadataItems;
			derived ref item ownedAllocation : AllocationUsage[0..*] ordered subsets ownedConnection subsets Metadata::metadataItems;
			derived ref item ownedAction : ActionUsage[0..*] ordered subsets ownedOccurrence subsets Metadata::metadataItems;
			derived ref item ownedState : StateUsage[0..*] ordered subsets ownedAction subsets Metadata::metadataItems;
			derived ref item ownedTransition : TransitionUsage[0..*] subsets ownedUsage subsets Metadata::metadataItems;
			derived ref item ownedCalculation : CalculationUsage[0..*] ordered subsets ownedAction subsets Metadata::metadataItems;
			derived ref item ownedConstraint : ConstraintUsage[0..*] ordered subsets ownedOccurrence subsets Metadata::metadataItems;
			derived ref item ownedRequirement : RequirementUsage[0..*] ordered subsets ownedConstraint subsets Metadata::metadataItems;
			derived ref item ownedConcern : ConcernUsage[0..*] subsets ownedRequirement subsets Metadata::metadataItems;
			derived ref item ownedCase : CaseUsage[0..*] ordered subsets ownedCalculation subsets Metadata::metadataItems;
			derived ref item ownedAnalysisCase : AnalysisCaseUsage[0..*] ordered subsets ownedCase subsets Metadata::metadataItems;
			derived ref item ownedVerificationCase : VerificationCaseUsage[0..*] ordered subsets ownedCase subsets Metadata::metadataItems;
			derived ref item ownedUseCase : UseCaseUsage[0..*] ordered subsets ownedCase subsets Metadata::metadataItems;
			derived ref item ownedView : ViewUsage[0..*] ordered subsets ownedPart subsets Metadata::metadataItems;
			derived ref item ownedViewpoint : ViewpointUsage[0..*] ordered subsets ownedRequirement subsets Metadata::metadataItems;
			derived ref item ownedRendering : RenderingUsage[0..*] ordered subsets ownedPart subsets Metadata::metadataItems;
			derived ref item ownedMetadata : MetadataUsage[0..*] ordered subsets ownedItem subsets Metadata::metadataItems;
		}		
		
		metadata def EnumerationDefinition specializes AttributeDefinition {
			attribute isVariation : Boolean[1..1] redefines isVariation;
			
			derived ref item enumeratedValue : EnumerationUsage[0..*] ordered redefines 'variant' subsets Metadata::metadataItems;
		}		
		
		metadata def EnumerationUsage specializes AttributeUsage {
			derived ref item enumerationDefinition : EnumerationDefinition[1..1] redefines attributeDefinition subsets Metadata::metadataItems;
		}		
		
		metadata def EventOccurrenceUsage specializes OccurrenceUsage {
			derived attribute isReference : Boolean[1..1] redefines isReference;
			
			derived ref item eventOccurrence : OccurrenceUsage[1..1] subsets Metadata::metadataItems;
		}		
		
		metadata def ExhibitStateUsage specializes StateUsage, PerformActionUsage {
			derived ref item exhibitedState : StateUsage[1..1] redefines performedAction subsets Metadata::metadataItems;
		}		
		
		abstract metadata def Expose specializes Import {
			attribute visibility : VisibilityKind[1..1] redefines visibility;
			attribute isImportAll : Boolean[1..1] redefines isImportAll;
		}		
		
		metadata def FlowDefinition specializes Interaction, ActionDefinition {
			derived ref item flowEnd : Usage[0..*] redefines associationEnd subsets Metadata::metadataItems;
		}		
		
		metadata def FlowUsage specializes ConnectorAsUsage, Flow, ActionUsage {
			derived ref item flowDefinition : Interaction[0..*] ordered redefines actionDefinition, interaction subsets Metadata::metadataItems;
		}		
		
		metadata def ForLoopActionUsage specializes LoopActionUsage {
			derived ref item seqArgument : Expression[1..1] subsets Metadata::metadataItems;
			derived ref item loopVariable : ReferenceUsage[1..1] subsets Metadata::metadataItems;
		}		
		
		metadata def ForkNode specializes ControlNode;		
		
		metadata def FramedConcernMembership specializes RequirementConstraintMembership {
			attribute kind : RequirementConstraintKind[1..1] redefines kind;
			
			derived item ownedConcern : ConcernUsage[1..1] redefines ownedConstraint subsets Metadata::metadataItems;
			derived ref item referencedConcern : ConcernUsage[1..1] redefines referencedConstraint subsets Metadata::metadataItems;
		}		
		
		metadata def IfActionUsage specializes ActionUsage {
			derived ref item elseAction : ActionUsage[0..1] subsets Metadata::metadataItems;
			derived ref item thenAction : ActionUsage[1..1] subsets Metadata::metadataItems;
			derived ref item ifArgument : Expression[1..1] subsets Metadata::metadataItems;
		}		
		
		metadata def IncludeUseCaseUsage specializes UseCaseUsage, PerformActionUsage {
			derived ref item useCaseIncluded : UseCaseUsage[1..1] redefines performedAction subsets Metadata::metadataItems;
		}		
		
		metadata def InterfaceDefinition specializes ConnectionDefinition {
			derived ref item interfaceEnd : PortUsage[0..*] ordered redefines connectionEnd subsets Metadata::metadataItems;
		}		
		
		metadata def InterfaceUsage specializes ConnectionUsage {
			derived ref item interfaceDefinition : InterfaceDefinition[0..*] redefines connectionDefinition subsets Metadata::metadataItems;
		}		
		
		metadata def ItemDefinition specializes Structure, OccurrenceDefinition;		
		
		metadata def ItemUsage specializes OccurrenceUsage {
			derived ref item itemDefinition : Structure[0..*] ordered subsets occurrenceDefinition subsets Metadata::metadataItems subsets Metadata::metadataItems;
		}		
		
		metadata def JoinNode specializes ControlNode;		
		
		abstract metadata def LoopActionUsage specializes ActionUsage {
			derived ref item bodyAction : ActionUsage[1..1] subsets Metadata::metadataItems;
		}		
		
		metadata def MembershipExpose specializes MembershipImport, Expose;		
		
		metadata def MergeNode specializes ControlNode;		
		
		metadata def MetadataDefinition specializes ItemDefinition, Metaclass;		
		
		metadata def MetadataUsage specializes ItemUsage, MetadataFeature {
			derived ref item metadataDefinition : Metaclass[0..1] redefines itemDefinition, metaclass subsets Metadata::metadataItems;
		}		
		
		metadata def NamespaceExpose specializes Expose, NamespaceImport;		
		
		metadata def ObjectiveMembership specializes FeatureMembership {
			derived item ownedObjectiveRequirement : RequirementUsage[1..1] redefines ownedMemberFeature subsets Metadata::metadataItems;
		}		
		
		metadata def OccurrenceDefinition specializes Definition, Class {
			attribute isIndividual : Boolean[1..1];
		}		
		
		metadata def OccurrenceUsage specializes Usage {
			attribute isIndividual : Boolean[1..1];
			attribute portionKind : PortionKind[0..1];
			
			derived ref item occurrenceDefinition : Class[0..*] ordered redefines definition subsets Metadata::metadataItems;
			derived ref item individualDefinition : OccurrenceDefinition[0..1] subsets occurrenceDefinition subsets Metadata::metadataItems;
		}		
		
		metadata def PartDefinition specializes ItemDefinition;		
		
		metadata def PartUsage specializes ItemUsage {
			derived ref item partDefinition : PartDefinition[0..*] ordered subsets itemDefinition subsets Metadata::metadataItems;
		}		
		
		metadata def PerformActionUsage specializes ActionUsage, EventOccurrenceUsage {
			derived ref item performedAction : ActionUsage[1..1] redefines eventOccurrence subsets Metadata::metadataItems;
		}		
		
		metadata def PortConjugation specializes Conjugation {
			ref item originalPortDefinition : PortDefinition[1..1] redefines originalType subsets Metadata::metadataItems;
			derived ref item conjugatedPortDefinition : ConjugatedPortDefinition[1..1] redefines owningType subsets Metadata::metadataItems;
		}		
		
		metadata def PortDefinition specializes OccurrenceDefinition, Structure {
			derived ref item conjugatedPortDefinition : ConjugatedPortDefinition[0..1] subsets ownedMember subsets Metadata::metadataItems;
		}		
		
		metadata def PortUsage specializes OccurrenceUsage {
			derived ref item portDefinition : PortDefinition[0..*] ordered redefines occurrenceDefinition subsets Metadata::metadataItems;
		}		
		
		enum def PortionKind {
			enum 'timeslice';
			enum 'snapshot';
		}
		
		metadata def ReferenceUsage specializes Usage {
			derived attribute isReference : Boolean[1..1] redefines isReference;
		}		
		
		metadata def RenderingDefinition specializes PartDefinition {
			derived ref item 'rendering' : RenderingUsage[0..*] ordered subsets usage subsets Metadata::metadataItems;
		}		
		
		metadata def RenderingUsage specializes PartUsage {
			derived ref item renderingDefinition : RenderingDefinition[0..1] redefines partDefinition subsets Metadata::metadataItems;
		}		
		
		enum def RequirementConstraintKind {
			enum assumption;
			enum 'requirement';
		}
		
		metadata def RequirementConstraintMembership specializes FeatureMembership {
			attribute kind : RequirementConstraintKind[1..1];
			
			derived item ownedConstraint : ConstraintUsage[1..1] redefines ownedMemberFeature subsets Metadata::metadataItems;
			derived ref item referencedConstraint : ConstraintUsage[1..1] subsets Metadata::metadataItems;
		}		
		
		metadata def RequirementDefinition specializes ConstraintDefinition {
			attribute reqId : String[0..1] redefines declaredShortName;
			derived attribute text : String[0..*];
			
			derived ref item subjectParameter : Usage[1..1] subsets parameter, usage subsets Metadata::metadataItems;
			derived ref item actorParameter : PartUsage[0..*] ordered subsets parameter, usage subsets Metadata::metadataItems;
			derived ref item stakeholderParameter : PartUsage[0..*] ordered subsets parameter, usage subsets Metadata::metadataItems;
			derived ref item assumedConstraint : ConstraintUsage[0..*] ordered subsets ownedFeature subsets Metadata::metadataItems;
			derived ref item requiredConstraint : ConstraintUsage[0..*] ordered subsets ownedFeature subsets Metadata::metadataItems;
			derived ref item framedConcern : ConcernUsage[0..*] ordered subsets requiredConstraint subsets Metadata::metadataItems;
		}		
		
		metadata def RequirementUsage specializes ConstraintUsage {
			attribute reqId : String[0..1] redefines declaredShortName;
			derived attribute text : String[0..*];
			
			derived ref item requirementDefinition : RequirementDefinition[0..1] redefines constraintDefinition subsets Metadata::metadataItems;
			derived ref item requiredConstraint : ConstraintUsage[0..*] ordered subsets ownedFeature subsets Metadata::metadataItems;
			derived ref item assumedConstraint : ConstraintUsage[0..*] ordered subsets ownedFeature subsets Metadata::metadataItems;
			derived ref item subjectParameter : Usage[1..1] subsets parameter, usage subsets Metadata::metadataItems;
			derived ref item framedConcern : ConcernUsage[0..*] ordered subsets requiredConstraint subsets Metadata::metadataItems;
			derived ref item actorParameter : PartUsage[0..*] ordered subsets parameter, usage subsets Metadata::metadataItems;
			derived ref item stakeholderParameter : PartUsage[0..*] ordered subsets parameter, usage subsets Metadata::metadataItems;
		}		
		
		metadata def RequirementVerificationMembership specializes RequirementConstraintMembership {
			attribute kind : RequirementConstraintKind[1..1] redefines kind;
			
			derived item ownedRequirement : RequirementUsage[1..1] redefines ownedConstraint subsets Metadata::metadataItems;
			derived ref item verifiedRequirement : RequirementUsage[1..1] redefines referencedConstraint subsets Metadata::metadataItems;
		}		
		
		metadata def SatisfyRequirementUsage specializes RequirementUsage, AssertConstraintUsage {
			derived ref item satisfiedRequirement : RequirementUsage[1..1] redefines assertedConstraint subsets Metadata::metadataItems;
			derived ref item satisfyingFeature : Feature[1..1] subsets Metadata::metadataItems;
		}		
		
		metadata def SendActionUsage specializes ActionUsage {
			derived ref item receiverArgument : Expression[0..1] subsets Metadata::metadataItems;
			derived ref item payloadArgument : Expression[1..1] subsets Metadata::metadataItems;
			derived ref item senderArgument : Expression[0..1] subsets Metadata::metadataItems;
		}		
		
		metadata def StakeholderMembership specializes ParameterMembership {
			derived item ownedStakeholderParameter : PartUsage[1..1] redefines ownedMemberParameter subsets Metadata::metadataItems;
		}		
		
		metadata def StateDefinition specializes ActionDefinition {
			attribute isParallel : Boolean[1..1];
			
			derived ref item 'state' : StateUsage[0..*] ordered subsets 'action' subsets Metadata::metadataItems;
			derived ref item entryAction : ActionUsage[0..1] subsets Metadata::metadataItems;
			derived ref item doAction : ActionUsage[0..1] subsets Metadata::metadataItems;
			derived ref item exitAction : ActionUsage[0..1] subsets Metadata::metadataItems;
		}		
		
		enum def StateSubactionKind {
			enum 'entry';
			enum 'do';
			enum 'exit';
		}
		
		metadata def StateSubactionMembership specializes FeatureMembership {
			attribute kind : StateSubactionKind[1..1];
			
			derived item 'action' : ActionUsage[1..1] redefines ownedMemberFeature subsets Metadata::metadataItems;
		}		
		
		metadata def StateUsage specializes ActionUsage {
			attribute isParallel : Boolean[1..1];
			
			derived ref item stateDefinition : Behavior[0..*] ordered redefines actionDefinition subsets Metadata::metadataItems;
			derived ref item entryAction : ActionUsage[0..1] subsets Metadata::metadataItems;
			derived ref item doAction : ActionUsage[0..1] subsets Metadata::metadataItems;
			derived ref item exitAction : ActionUsage[0..1] subsets Metadata::metadataItems;
		}		
		
		metadata def SubjectMembership specializes ParameterMembership {
			derived item ownedSubjectParameter : Usage[1..1] redefines ownedMemberParameter subsets Metadata::metadataItems;
		}		
		
		metadata def SuccessionAsUsage specializes ConnectorAsUsage, Succession;		
		
		metadata def SuccessionFlowUsage specializes SuccessionFlow, FlowUsage;		
		
		metadata def TerminateActionUsage specializes ActionUsage {
			derived ref item terminatedOccurrenceArgument : Expression[0..1] subsets Metadata::metadataItems;
		}		
		
		enum def TransitionFeatureKind {
			enum trigger;
			enum guard;
			enum effect;
		}
		
		metadata def TransitionFeatureMembership specializes FeatureMembership {
			attribute kind : TransitionFeatureKind[1..1];
			
			derived item transitionFeature : Step[1..1] redefines ownedMemberFeature subsets Metadata::metadataItems;
		}		
		
		metadata def TransitionUsage specializes ActionUsage {
			derived ref item source : ActionUsage[1..1] subsets Metadata::metadataItems;
			derived ref item target : ActionUsage[1..1] subsets Metadata::metadataItems;
			derived ref item triggerAction : AcceptActionUsage[0..*] subsets ownedFeature subsets Metadata::metadataItems;
			derived ref item guardExpression : Expression[0..*] subsets ownedFeature subsets Metadata::metadataItems;
			derived ref item effectAction : ActionUsage[0..*] subsets feature subsets Metadata::metadataItems;
			derived ref item 'succession' : Succession[1..1] subsets ownedMember subsets Metadata::metadataItems;
		}		
		
		metadata def TriggerInvocationExpression specializes InvocationExpression {
			attribute kind : TriggerKind[1..1];
		}		
		
		enum def TriggerKind {
			enum 'when';
			enum 'at';
			enum 'after';
		}
		
		metadata def Usage specializes Feature {
			attribute isVariation : Boolean[1..1];
			derived attribute mayTimeVary : Boolean[1..1] redefines isVariable;
			derived attribute isReference : Boolean[1..1];
			
			derived ref item 'variant' : Usage[0..*] subsets ownedMember subsets Metadata::metadataItems;
			derived item variantMembership : VariantMembership[0..*] subsets ownedMembership subsets Metadata::metadataItems;
			derived ref item owningDefinition : Definition[0..1] subsets owningType subsets Metadata::metadataItems;
			derived ref item owningUsage : Usage[0..1] subsets owningType subsets Metadata::metadataItems;
			derived ref item definition : Classifier[0..*] ordered redefines type subsets Metadata::metadataItems;
			derived ref item usage : Usage[0..*] ordered subsets feature subsets Metadata::metadataItems;
			derived ref item directedUsage : Usage[0..*] ordered subsets directedFeature, usage subsets Metadata::metadataItems;
			derived ref item nestedUsage : Usage[0..*] ordered subsets ownedFeature, usage subsets Metadata::metadataItems;
			derived ref item nestedReference : ReferenceUsage[0..*] ordered subsets nestedUsage subsets Metadata::metadataItems;
			derived ref item nestedAttribute : AttributeUsage[0..*] ordered subsets nestedUsage subsets Metadata::metadataItems;
			derived ref item nestedEnumeration : EnumerationUsage[0..*] ordered subsets nestedAttribute subsets Metadata::metadataItems;
			derived ref item nestedOccurrence : OccurrenceUsage[0..*] ordered subsets nestedUsage subsets Metadata::metadataItems;
			derived ref item nestedItem : ItemUsage[0..*] ordered subsets nestedOccurrence subsets Metadata::metadataItems;
			derived ref item nestedPart : PartUsage[0..*] ordered subsets nestedItem subsets Metadata::metadataItems;
			derived ref item nestedPort : PortUsage[0..*] ordered subsets nestedUsage subsets Metadata::metadataItems;
			derived ref item nestedConnection : ConnectorAsUsage[0..*] ordered subsets nestedUsage subsets Metadata::metadataItems;
			derived ref item nestedFlow : FlowUsage[0..*] subsets nestedConnection subsets Metadata::metadataItems;
			derived ref item nestedInterface : InterfaceUsage[0..*] ordered subsets nestedConnection subsets Metadata::metadataItems;
			derived ref item nestedAllocation : AllocationUsage[0..*] ordered subsets nestedConnection subsets Metadata::metadataItems;
			derived ref item nestedAction : ActionUsage[0..*] ordered subsets nestedOccurrence subsets Metadata::metadataItems;
			derived ref item nestedState : StateUsage[0..*] ordered subsets nestedAction subsets Metadata::metadataItems;
			derived ref item nestedTransition : TransitionUsage[0..*] subsets nestedUsage subsets Metadata::metadataItems;
			derived ref item nestedCalculation : CalculationUsage[0..*] ordered subsets nestedAction subsets Metadata::metadataItems;
			derived ref item nestedConstraint : ConstraintUsage[0..*] ordered subsets nestedOccurrence subsets Metadata::metadataItems;
			derived ref item nestedRequirement : RequirementUsage[0..*] ordered subsets nestedConstraint subsets Metadata::metadataItems;
			derived ref item nestedConcern : ConcernUsage[0..*] subsets nestedRequirement subsets Metadata::metadataItems;
			derived ref item nestedCase : CaseUsage[0..*] ordered subsets nestedCalculation subsets Metadata::metadataItems;
			derived ref item nestedAnalysisCase : AnalysisCaseUsage[0..*] ordered subsets nestedCase subsets Metadata::metadataItems;
			derived ref item nestedVerificationCase : VerificationCaseUsage[0..*] ordered subsets nestedCase subsets Metadata::metadataItems;
			derived ref item nestedUseCase : UseCaseUsage[0..*] ordered subsets nestedCase subsets Metadata::metadataItems;
			derived ref item nestedView : ViewUsage[0..*] ordered subsets nestedPart subsets Metadata::metadataItems;
			derived ref item nestedViewpoint : ViewpointUsage[0..*] ordered subsets nestedRequirement subsets Metadata::metadataItems;
			derived ref item nestedRendering : RenderingUsage[0..*] ordered subsets nestedPart subsets Metadata::metadataItems;
			derived ref item nestedMetadata : MetadataUsage[0..*] ordered subsets nestedItem subsets Metadata::metadataItems;
		}		
		
		metadata def UseCaseDefinition specializes CaseDefinition {
			derived ref item includedUseCase : UseCaseUsage[0..*] ordered subsets Metadata::metadataItems;
		}		
		
		metadata def UseCaseUsage specializes CaseUsage {
			derived ref item useCaseDefinition : UseCaseDefinition[0..1] redefines caseDefinition subsets Metadata::metadataItems;
			derived ref item includedUseCase : UseCaseUsage[0..*] ordered subsets Metadata::metadataItems;
		}		
		
		metadata def VariantMembership specializes OwningMembership {
			derived item ownedVariantUsage : Usage[1..1] redefines ownedMemberElement subsets Metadata::metadataItems;
		}		
		
		metadata def VerificationCaseDefinition specializes CaseDefinition {
			derived ref item verifiedRequirement : RequirementUsage[0..*] ordered subsets Metadata::metadataItems;
		}		
		
		metadata def VerificationCaseUsage specializes CaseUsage {
			derived ref item verificationCaseDefinition : VerificationCaseDefinition[0..1] subsets caseDefinition subsets Metadata::metadataItems;
			derived ref item verifiedRequirement : RequirementUsage[0..*] ordered subsets Metadata::metadataItems;
		}		
		
		metadata def ViewDefinition specializes PartDefinition {
			derived ref item 'view' : ViewUsage[0..*] ordered subsets usage subsets Metadata::metadataItems;
			derived ref item satisfiedViewpoint : ViewpointUsage[0..*] ordered subsets ownedRequirement subsets Metadata::metadataItems;
			derived ref item viewRendering : RenderingUsage[0..1] subsets Metadata::metadataItems;
			derived ref item viewCondition : Expression[0..*] ordered subsets ownedMember subsets Metadata::metadataItems;
		}		
		
		metadata def ViewRenderingMembership specializes FeatureMembership {
			derived item ownedRendering : RenderingUsage[1..1] redefines ownedMemberFeature subsets Metadata::metadataItems;
			derived ref item referencedRendering : RenderingUsage[1..1] subsets Metadata::metadataItems;
		}		
		
		metadata def ViewUsage specializes PartUsage {
			derived ref item viewDefinition : ViewDefinition[0..1] redefines partDefinition subsets Metadata::metadataItems;
			derived ref item satisfiedViewpoint : ViewpointUsage[0..*] ordered subsets nestedRequirement subsets Metadata::metadataItems;
			derived ref item exposedElement : Element[0..*] ordered subsets member subsets Metadata::metadataItems;
			derived ref item viewRendering : RenderingUsage[0..1] subsets Metadata::metadataItems;
			derived ref item viewCondition : Expression[0..*] ordered subsets ownedMember subsets Metadata::metadataItems;
		}		
		
		metadata def ViewpointDefinition specializes RequirementDefinition {
			derived ref item viewpointStakeholder : PartUsage[0..*] ordered subsets Metadata::metadataItems;
		}		
		
		metadata def ViewpointUsage specializes RequirementUsage {
			derived ref item viewpointDefinition : ViewpointDefinition[0..1] redefines requirementDefinition subsets Metadata::metadataItems;
			derived ref item viewpointStakeholder : PartUsage[0..*] ordered subsets Metadata::metadataItems;
		}		
		
		metadata def WhileLoopActionUsage specializes LoopActionUsage {
			derived ref item whileArgument : Expression[1..1] subsets Metadata::metadataItems;
			derived ref item untilArgument : Expression[0..1] subsets Metadata::metadataItems;
		}		
		
	}
	
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Expression'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'parameter'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Expression'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Behavior'
semantic.unresolved_name 'step'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Step'
semantic.unresolved_name 'Behavior'
semantic.unresolved_name 'behavior'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'ParameterMembership'
semantic.unresolved_name 'ownedMemberParameter'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Expression'
semantic.unresolved_name 'expression'
semantic.unresolved_name 'ownedFeature'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Expression'
semantic.unresolved_name 'ownedFeature'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Invariant'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Expression'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Expression'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Feature'
semantic.unresolved_name 'member'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'DataType'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'DataType'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'BindingConnector'
semantic.unresolved_name 'Function'
semantic.unresolved_name 'expression'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Expression'
semantic.unresolved_name 'Function'
semantic.unresolved_name 'function'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'parameter'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'parameter'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'parameter'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'parameter'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'owningNamespace'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'ownedConjugator'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'FeatureTyping'
semantic.unresolved_name 'type'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'AssociationStructure'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'isSufficient'
semantic.unresolved_name 'associationEnd'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'AssociationStructure'
semantic.unresolved_name 'association'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Connector'
semantic.unresolved_name 'Predicate'
semantic.unresolved_name 'BooleanExpression'
semantic.unresolved_name 'Predicate'
semantic.unresolved_name 'predicate'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Classifier'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'ownedMember'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'ownedMembership'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'feature'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'directedFeature'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'ownedFeature'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Import'
semantic.unresolved_name 'VisibilityKind'
semantic.unresolved_name 'visibility'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'isImportAll'
semantic.unresolved_name 'Interaction'
semantic.unresolved_name 'associationEnd'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Flow'
semantic.unresolved_name 'Interaction'
semantic.unresolved_name 'interaction'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Expression'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Expression'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Structure'
semantic.unresolved_name 'Structure'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'MembershipImport'
semantic.unresolved_name 'Metaclass'
semantic.unresolved_name 'MetadataFeature'
semantic.unresolved_name 'Metaclass'
semantic.unresolved_name 'metaclass'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'NamespaceImport'
semantic.unresolved_name 'FeatureMembership'
semantic.unresolved_name 'ownedMemberFeature'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Class'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Class'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Conjugation'
semantic.unresolved_name 'originalType'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'owningType'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Structure'
semantic.unresolved_name 'ownedMember'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'FeatureMembership'
semantic.unresolved_name 'ownedMemberFeature'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'String'
semantic.unresolved_name 'declaredShortName'
semantic.unresolved_name 'String'
semantic.unresolved_name 'parameter'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'parameter'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'parameter'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'ownedFeature'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'ownedFeature'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'String'
semantic.unresolved_name 'declaredShortName'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'ownedFeature'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'ownedFeature'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'parameter'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'parameter'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'parameter'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Feature'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Expression'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Expression'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Expression'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'ParameterMembership'
semantic.unresolved_name 'ownedMemberParameter'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'FeatureMembership'
semantic.unresolved_name 'ownedMemberFeature'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Behavior'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'ParameterMembership'
semantic.unresolved_name 'ownedMemberParameter'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Succession'
semantic.unresolved_name 'SuccessionFlow'
semantic.unresolved_name 'Expression'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'FeatureMembership'
semantic.unresolved_name 'Step'
semantic.unresolved_name 'ownedMemberFeature'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'ownedFeature'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Expression'
semantic.unresolved_name 'ownedFeature'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'feature'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Succession'
semantic.unresolved_name 'ownedMember'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'InvocationExpression'
semantic.unresolved_name 'Feature'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'isVariable'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'ownedMember'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'ownedMembership'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'owningType'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'owningType'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Classifier'
semantic.unresolved_name 'type'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'feature'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'directedFeature'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'ownedFeature'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'OwningMembership'
semantic.unresolved_name 'ownedMemberElement'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Expression'
semantic.unresolved_name 'ownedMember'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'FeatureMembership'
semantic.unresolved_name 'ownedMemberFeature'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Element'
semantic.unresolved_name 'member'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Expression'
semantic.unresolved_name 'ownedMember'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Expression'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Expression'
semantic.unresolved_name 'Metadata::metadataItems'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Expression'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'parameter'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Expression'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Behavior'
semantic.unresolved_name 'step'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Step'
semantic.unresolved_name 'Behavior'
semantic.unresolved_name 'behavior'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'ParameterMembership'
semantic.unresolved_name 'ownedMemberParameter'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Expression'
semantic.unresolved_name 'expression'
semantic.unresolved_name 'ownedFeature'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Expression'
semantic.unresolved_name 'ownedFeature'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Invariant'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Expression'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Expression'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Feature'
semantic.unresolved_name 'member'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'DataType'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'DataType'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'BindingConnector'
semantic.unresolved_name 'Function'
semantic.unresolved_name 'expression'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Expression'
semantic.unresolved_name 'Function'
semantic.unresolved_name 'function'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'parameter'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'parameter'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'parameter'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'parameter'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'owningNamespace'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'ownedConjugator'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'FeatureTyping'
semantic.unresolved_name 'type'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'AssociationStructure'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'isSufficient'
semantic.unresolved_name 'associationEnd'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'AssociationStructure'
semantic.unresolved_name 'association'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Connector'
semantic.unresolved_name 'Predicate'
semantic.unresolved_name 'BooleanExpression'
semantic.unresolved_name 'Predicate'
semantic.unresolved_name 'predicate'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Classifier'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'ownedMember'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'ownedMembership'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'feature'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'directedFeature'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'ownedFeature'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Import'
semantic.unresolved_name 'VisibilityKind'
semantic.unresolved_name 'visibility'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'isImportAll'
semantic.unresolved_name 'Interaction'
semantic.unresolved_name 'associationEnd'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Flow'
semantic.unresolved_name 'Interaction'
semantic.unresolved_name 'interaction'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Expression'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Expression'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Structure'
semantic.unresolved_name 'Structure'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'MembershipImport'
semantic.unresolved_name 'Metaclass'
semantic.unresolved_name 'MetadataFeature'
semantic.unresolved_name 'Metaclass'
semantic.unresolved_name 'metaclass'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'NamespaceImport'
semantic.unresolved_name 'FeatureMembership'
semantic.unresolved_name 'ownedMemberFeature'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Class'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Class'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Conjugation'
semantic.unresolved_name 'originalType'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'owningType'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Structure'
semantic.unresolved_name 'ownedMember'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'FeatureMembership'
semantic.unresolved_name 'ownedMemberFeature'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'String'
semantic.unresolved_name 'declaredShortName'
semantic.unresolved_name 'String'
semantic.unresolved_name 'parameter'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'parameter'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'parameter'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'ownedFeature'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'ownedFeature'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'String'
semantic.unresolved_name 'declaredShortName'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'ownedFeature'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'ownedFeature'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'parameter'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'parameter'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'parameter'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Feature'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Expression'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Expression'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Expression'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'ParameterMembership'
semantic.unresolved_name 'ownedMemberParameter'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'FeatureMembership'
semantic.unresolved_name 'ownedMemberFeature'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Behavior'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'ParameterMembership'
semantic.unresolved_name 'ownedMemberParameter'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Succession'
semantic.unresolved_name 'SuccessionFlow'
semantic.unresolved_name 'Expression'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'FeatureMembership'
semantic.unresolved_name 'Step'
semantic.unresolved_name 'ownedMemberFeature'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'ownedFeature'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Expression'
semantic.unresolved_name 'ownedFeature'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'feature'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Succession'
semantic.unresolved_name 'ownedMember'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'InvocationExpression'
semantic.unresolved_name 'Feature'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'isVariable'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'ownedMember'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'ownedMembership'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'owningType'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'owningType'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Classifier'
semantic.unresolved_name 'type'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'feature'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'directedFeature'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'ownedFeature'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'OwningMembership'
semantic.unresolved_name 'ownedMemberElement'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Expression'
semantic.unresolved_name 'ownedMember'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'FeatureMembership'
semantic.unresolved_name 'ownedMemberFeature'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Element'
semantic.unresolved_name 'member'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Expression'
semantic.unresolved_name 'ownedMember'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Expression'
semantic.unresolved_name 'Metadata::metadataItems'
semantic.unresolved_name 'Expression'
semantic.unresolved_name 'Metadata::metadataItems'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPackage,Ident,OpenCurly,
KwPublic,KwImport,Ident,ColonColon,Ident,ColonColon,Star,Semicolon,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,Comma,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,Comma,Ident,OpenCurly,
KwDerived,KwRef,KwItem,UnrestrictedName,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,KwStep,Comma,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,Comma,Ident,OpenCurly,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwRedefines,KwBehavior,Comma,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwRef,KwItem,UnrestrictedName,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwRedefines,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,Comma,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,Comma,Ident,OpenCurly,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,KwMember,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,Comma,Ident,Semicolon,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwRedefines,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,Comma,Ident,Semicolon,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,Comma,Ident,OpenCurly,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,UnrestrictedName,Comma,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,Comma,Ident,OpenCurly,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwOrdered,KwRedefines,KwFunction,Comma,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,Comma,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,Comma,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,Comma,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,Comma,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,Semicolon,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,KwType,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,Comma,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwRedefines,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,Comma,Ident,OpenCurly,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwRedefines,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwAbstract,KwMetadata,KwDef,Ident,KwSpecializes,Ident,Comma,Ident,Semicolon,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,Comma,Ident,Semicolon,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,Comma,Ident,OpenCurly,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,KwPredicate,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwAbstract,KwMetadata,KwDef,Ident,KwSpecializes,Ident,Semicolon,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,Semicolon,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwDerived,KwRef,KwItem,UnrestrictedName,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,KwFeature,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,Comma,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,Comma,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwRedefines,UnrestrictedName,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,Comma,Ident,OpenCurly,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwAbstract,KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,Comma,Ident,OpenCurly,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwRedefines,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,Comma,Ident,Comma,Ident,OpenCurly,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwRedefines,Ident,Comma,KwInteraction,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,Semicolon,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
KwDerived,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,Comma,Ident,OpenCurly,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwRedefines,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwRedefines,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,Comma,Ident,Semicolon,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,Semicolon,
KwAbstract,KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,Comma,Ident,Semicolon,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,Semicolon,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,Comma,Ident,Semicolon,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,Comma,Ident,OpenCurly,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Comma,KwMetaclass,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,Comma,Ident,Semicolon,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,Comma,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwRedefines,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,Semicolon,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,Comma,Ident,OpenCurly,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,Comma,Ident,OpenCurly,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwRedefines,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwEnum,KwDef,Ident,OpenCurly,
KwEnum,UnrestrictedName,Semicolon,
KwEnum,UnrestrictedName,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwRef,KwItem,UnrestrictedName,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwEnum,KwDef,Ident,OpenCurly,
KwEnum,Ident,Semicolon,
KwEnum,UnrestrictedName,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwDerived,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
KwDerived,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,Comma,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,Comma,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,Comma,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
KwDerived,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,Comma,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,Comma,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,Comma,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
KwDerived,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,Comma,Ident,OpenCurly,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwDerived,KwRef,KwItem,UnrestrictedName,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,UnrestrictedName,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwEnum,KwDef,Ident,OpenCurly,
KwEnum,UnrestrictedName,Semicolon,
KwEnum,UnrestrictedName,Semicolon,
KwEnum,UnrestrictedName,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwDerived,KwItem,UnrestrictedName,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwRedefines,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,Comma,Ident,Semicolon,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,Comma,Ident,Semicolon,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwEnum,KwDef,Ident,OpenCurly,
KwEnum,Ident,Semicolon,
KwEnum,Ident,Semicolon,
KwEnum,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwDerived,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,KwFeature,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,UnrestrictedName,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwEnum,KwDef,Ident,OpenCurly,
KwEnum,UnrestrictedName,Semicolon,
KwEnum,UnrestrictedName,Semicolon,
KwEnum,UnrestrictedName,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwDerived,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
KwDerived,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwDerived,KwRef,KwItem,UnrestrictedName,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwRedefines,KwType,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,KwFeature,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,Comma,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,Comma,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwRef,KwItem,UnrestrictedName,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,KwMember,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwRedefines,Ident,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,ColonColon,Ident,Semicolon,
KwDerived,KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'SysML'
    (documentation)
    (import_decl private 'ScalarValues::*')
    (import_decl public 'Systems::*')
    (package_def 'Systems'
      (import_decl public 'KerML::Kernel::*')
      (metadata_def 'AcceptActionUsage' :> 'ActionUsage'
        (item_usage derived ref 'receiverArgument' : 'Expression' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'payloadParameter' : 'ReferenceUsage' :> 'nestedReference', 'parameter' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'payloadArgument' : 'Expression' :> 'Metadata::metadataItems' multiplicity))
      (metadata_def 'ActionDefinition' :> 'Behavior', 'OccurrenceDefinition'
        (item_usage derived ref ''action'' : 'ActionUsage' :> 'step', 'usage' :> 'Metadata::metadataItems' multiplicity ordered))
      (metadata_def 'ActionUsage' :> 'Step', 'OccurrenceUsage'
        (item_usage derived ref 'actionDefinition' : 'Behavior' :>> 'behavior', 'occurrenceDefinition' :> 'Metadata::metadataItems' multiplicity ordered))
      (metadata_def 'ActorMembership' :> 'ParameterMembership'
        (item_usage derived 'ownedActorParameter' : 'PartUsage' :>> 'ownedMemberParameter' :> 'Metadata::metadataItems' multiplicity))
      (metadata_def 'AllocationDefinition' :> 'ConnectionDefinition'
        (item_usage derived ref ''allocation'' : 'AllocationUsage' :> 'usage' :> 'Metadata::metadataItems' multiplicity ordered))
      (metadata_def 'AllocationUsage' :> 'ConnectionUsage'
        (item_usage derived ref 'allocationDefinition' : 'AllocationDefinition' :>> 'connectionDefinition' :> 'Metadata::metadataItems' multiplicity ordered))
      (metadata_def 'AnalysisCaseDefinition' :> 'CaseDefinition'
        (item_usage derived ref 'resultExpression' : 'Expression' :> 'expression', 'ownedFeature' :> 'Metadata::metadataItems' multiplicity))
      (metadata_def 'AnalysisCaseUsage' :> 'CaseUsage'
        (item_usage derived ref 'analysisCaseDefinition' : 'AnalysisCaseDefinition' :>> 'caseDefinition' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'resultExpression' : 'Expression' :> 'ownedFeature' :> 'Metadata::metadataItems' multiplicity))
      (metadata_def 'AssertConstraintUsage' :> 'ConstraintUsage', 'Invariant'
        (item_usage derived ref 'assertedConstraint' : 'ConstraintUsage' :> 'Metadata::metadataItems' multiplicity))
      (metadata_def 'AssignmentActionUsage' :> 'ActionUsage'
        (item_usage derived ref 'targetArgument' : 'Expression' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'valueExpression' : 'Expression' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'referent' : 'Feature' :> 'member' :> 'Metadata::metadataItems' multiplicity))
      (metadata_def 'AttributeDefinition' :> 'DataType', 'Definition')
      (metadata_def 'AttributeUsage' :> 'Usage'
        (attribute_usage derived 'isReference' : 'Boolean' :>> 'isReference' multiplicity)
        (item_usage derived ref 'attributeDefinition' : 'DataType' :>> 'definition' :> 'Metadata::metadataItems' multiplicity ordered))
      (metadata_def 'BindingConnectorAsUsage' :> 'BindingConnector', 'ConnectorAsUsage')
      (metadata_def 'CalculationDefinition' :> 'Function', 'ActionDefinition'
        (item_usage derived ref 'calculation' : 'CalculationUsage' :> ''action'', 'expression' :> 'Metadata::metadataItems' multiplicity ordered))
      (metadata_def 'CalculationUsage' :> 'Expression', 'ActionUsage'
        (item_usage derived ref 'calculationDefinition' : 'Function' :>> 'function', 'actionDefinition' :> 'Metadata::metadataItems' multiplicity ordered))
      (metadata_def 'CaseDefinition' :> 'CalculationDefinition'
        (item_usage derived ref 'objectiveRequirement' : 'RequirementUsage' :> 'usage' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'subjectParameter' : 'Usage' :> 'parameter', 'usage' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'actorParameter' : 'PartUsage' :> 'parameter', 'usage' :> 'Metadata::metadataItems' multiplicity ordered))
      (metadata_def 'CaseUsage' :> 'CalculationUsage'
        (item_usage derived ref 'objectiveRequirement' : 'RequirementUsage' :> 'usage' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'caseDefinition' : 'CaseDefinition' :>> 'calculationDefinition' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'subjectParameter' : 'Usage' :> 'parameter', 'usage' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'actorParameter' : 'PartUsage' :> 'parameter', 'usage' :> 'Metadata::metadataItems' multiplicity ordered))
      (metadata_def 'ConcernDefinition' :> 'RequirementDefinition')
      (metadata_def 'ConcernUsage' :> 'RequirementUsage'
        (item_usage derived ref 'concernDefinition' : 'ConcernDefinition' :>> 'requirementDefinition' :> 'Metadata::metadataItems' multiplicity))
      (metadata_def 'ConjugatedPortDefinition' :> 'PortDefinition'
        (item_usage derived ref 'originalPortDefinition' : 'PortDefinition' :>> 'owningNamespace' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'ownedPortConjugator' : 'PortConjugation' :>> 'ownedConjugator' :> 'Metadata::metadataItems' multiplicity))
      (metadata_def 'ConjugatedPortTyping' :> 'FeatureTyping'
        (item_usage ref 'conjugatedPortDefinition' : 'ConjugatedPortDefinition' :>> 'type' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'portDefinition' : 'PortDefinition' :> 'Metadata::metadataItems' multiplicity))
      (metadata_def 'ConnectionDefinition' :> 'AssociationStructure', 'PartDefinition'
        (attribute_usage 'isSufficient' : 'Boolean' :>> 'isSufficient' multiplicity)
        (item_usage derived ref 'connectionEnd' : 'Usage' :>> 'associationEnd' :> 'Metadata::metadataItems' multiplicity ordered))
      (metadata_def 'ConnectionUsage' :> 'ConnectorAsUsage', 'PartUsage'
        (item_usage derived ref 'connectionDefinition' : 'AssociationStructure' :> 'itemDefinition' :>> 'association' :> 'Metadata::metadataItems' multiplicity ordered))
      (metadata_def abstract 'ConnectorAsUsage' :> 'Usage', 'Connector')
      (metadata_def 'ConstraintDefinition' :> 'OccurrenceDefinition', 'Predicate')
      (metadata_def 'ConstraintUsage' :> 'BooleanExpression', 'OccurrenceUsage'
        (item_usage derived ref 'constraintDefinition' : 'Predicate' :>> 'predicate' :> 'Metadata::metadataItems' multiplicity))
      (metadata_def abstract 'ControlNode' :> 'ActionUsage')
      (metadata_def 'DecisionNode' :> 'ControlNode')
      (metadata_def 'Definition' :> 'Classifier'
        (attribute_usage 'isVariation' : 'Boolean' multiplicity)
        (item_usage derived ref ''variant'' : 'Usage' :> 'ownedMember' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived 'variantMembership' : 'VariantMembership' :> 'ownedMembership' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'usage' : 'Usage' :> 'feature' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'directedUsage' : 'Usage' :> 'directedFeature', 'usage' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'ownedUsage' : 'Usage' :> 'ownedFeature', 'usage' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'ownedReference' : 'ReferenceUsage' :> 'ownedUsage' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'ownedAttribute' : 'AttributeUsage' :> 'ownedUsage' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'ownedEnumeration' : 'EnumerationUsage' :> 'ownedAttribute' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'ownedOccurrence' : 'OccurrenceUsage' :> 'ownedUsage' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'ownedItem' : 'ItemUsage' :> 'ownedOccurrence' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'ownedPart' : 'PartUsage' :> 'ownedItem' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'ownedPort' : 'PortUsage' :> 'ownedUsage' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'ownedConnection' : 'ConnectorAsUsage' :> 'ownedUsage' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'ownedFlow' : 'FlowUsage' :> 'ownedConnection' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'ownedInterface' : 'InterfaceUsage' :> 'ownedConnection' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'ownedAllocation' : 'AllocationUsage' :> 'ownedConnection' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'ownedAction' : 'ActionUsage' :> 'ownedOccurrence' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'ownedState' : 'StateUsage' :> 'ownedAction' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'ownedTransition' : 'TransitionUsage' :> 'ownedUsage' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'ownedCalculation' : 'CalculationUsage' :> 'ownedAction' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'ownedConstraint' : 'ConstraintUsage' :> 'ownedOccurrence' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'ownedRequirement' : 'RequirementUsage' :> 'ownedConstraint' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'ownedConcern' : 'ConcernUsage' :> 'ownedRequirement' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'ownedCase' : 'CaseUsage' :> 'ownedCalculation' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'ownedAnalysisCase' : 'AnalysisCaseUsage' :> 'ownedCase' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'ownedVerificationCase' : 'VerificationCaseUsage' :> 'ownedCase' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'ownedUseCase' : 'UseCaseUsage' :> 'ownedCase' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'ownedView' : 'ViewUsage' :> 'ownedPart' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'ownedViewpoint' : 'ViewpointUsage' :> 'ownedRequirement' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'ownedRendering' : 'RenderingUsage' :> 'ownedPart' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'ownedMetadata' : 'MetadataUsage' :> 'ownedItem' :> 'Metadata::metadataItems' multiplicity ordered))
      (metadata_def 'EnumerationDefinition' :> 'AttributeDefinition'
        (attribute_usage 'isVariation' : 'Boolean' :>> 'isVariation' multiplicity)
        (item_usage derived ref 'enumeratedValue' : 'EnumerationUsage' :>> ''variant'' :> 'Metadata::metadataItems' multiplicity ordered))
      (metadata_def 'EnumerationUsage' :> 'AttributeUsage'
        (item_usage derived ref 'enumerationDefinition' : 'EnumerationDefinition' :>> 'attributeDefinition' :> 'Metadata::metadataItems' multiplicity))
      (metadata_def 'EventOccurrenceUsage' :> 'OccurrenceUsage'
        (attribute_usage derived 'isReference' : 'Boolean' :>> 'isReference' multiplicity)
        (item_usage derived ref 'eventOccurrence' : 'OccurrenceUsage' :> 'Metadata::metadataItems' multiplicity))
      (metadata_def 'ExhibitStateUsage' :> 'StateUsage', 'PerformActionUsage'
        (item_usage derived ref 'exhibitedState' : 'StateUsage' :>> 'performedAction' :> 'Metadata::metadataItems' multiplicity))
      (metadata_def abstract 'Expose' :> 'Import'
        (attribute_usage 'visibility' : 'VisibilityKind' :>> 'visibility' multiplicity)
        (attribute_usage 'isImportAll' : 'Boolean' :>> 'isImportAll' multiplicity))
      (metadata_def 'FlowDefinition' :> 'Interaction', 'ActionDefinition'
        (item_usage derived ref 'flowEnd' : 'Usage' :>> 'associationEnd' :> 'Metadata::metadataItems' multiplicity))
      (metadata_def 'FlowUsage' :> 'ConnectorAsUsage', 'Flow', 'ActionUsage'
        (item_usage derived ref 'flowDefinition' : 'Interaction' :>> 'actionDefinition', 'interaction' :> 'Metadata::metadataItems' multiplicity ordered))
      (metadata_def 'ForLoopActionUsage' :> 'LoopActionUsage'
        (item_usage derived ref 'seqArgument' : 'Expression' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'loopVariable' : 'ReferenceUsage' :> 'Metadata::metadataItems' multiplicity))
      (metadata_def 'ForkNode' :> 'ControlNode')
      (metadata_def 'FramedConcernMembership' :> 'RequirementConstraintMembership'
        (attribute_usage 'kind' : 'RequirementConstraintKind' :>> 'kind' multiplicity)
        (item_usage derived 'ownedConcern' : 'ConcernUsage' :>> 'ownedConstraint' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'referencedConcern' : 'ConcernUsage' :>> 'referencedConstraint' :> 'Metadata::metadataItems' multiplicity))
      (metadata_def 'IfActionUsage' :> 'ActionUsage'
        (item_usage derived ref 'elseAction' : 'ActionUsage' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'thenAction' : 'ActionUsage' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'ifArgument' : 'Expression' :> 'Metadata::metadataItems' multiplicity))
      (metadata_def 'IncludeUseCaseUsage' :> 'UseCaseUsage', 'PerformActionUsage'
        (item_usage derived ref 'useCaseIncluded' : 'UseCaseUsage' :>> 'performedAction' :> 'Metadata::metadataItems' multiplicity))
      (metadata_def 'InterfaceDefinition' :> 'ConnectionDefinition'
        (item_usage derived ref 'interfaceEnd' : 'PortUsage' :>> 'connectionEnd' :> 'Metadata::metadataItems' multiplicity ordered))
      (metadata_def 'InterfaceUsage' :> 'ConnectionUsage'
        (item_usage derived ref 'interfaceDefinition' : 'InterfaceDefinition' :>> 'connectionDefinition' :> 'Metadata::metadataItems' multiplicity))
      (metadata_def 'ItemDefinition' :> 'Structure', 'OccurrenceDefinition')
      (metadata_def 'ItemUsage' :> 'OccurrenceUsage'
        (item_usage derived ref 'itemDefinition' : 'Structure' :> 'occurrenceDefinition' :> 'Metadata::metadataItems' :> 'Metadata::metadataItems' multiplicity ordered))
      (metadata_def 'JoinNode' :> 'ControlNode')
      (metadata_def abstract 'LoopActionUsage' :> 'ActionUsage'
        (item_usage derived ref 'bodyAction' : 'ActionUsage' :> 'Metadata::metadataItems' multiplicity))
      (metadata_def 'MembershipExpose' :> 'MembershipImport', 'Expose')
      (metadata_def 'MergeNode' :> 'ControlNode')
      (metadata_def 'MetadataDefinition' :> 'ItemDefinition', 'Metaclass')
      (metadata_def 'MetadataUsage' :> 'ItemUsage', 'MetadataFeature'
        (item_usage derived ref 'metadataDefinition' : 'Metaclass' :>> 'itemDefinition', 'metaclass' :> 'Metadata::metadataItems' multiplicity))
      (metadata_def 'NamespaceExpose' :> 'Expose', 'NamespaceImport')
      (metadata_def 'ObjectiveMembership' :> 'FeatureMembership'
        (item_usage derived 'ownedObjectiveRequirement' : 'RequirementUsage' :>> 'ownedMemberFeature' :> 'Metadata::metadataItems' multiplicity))
      (metadata_def 'OccurrenceDefinition' :> 'Definition', 'Class'
        (attribute_usage 'isIndividual' : 'Boolean' multiplicity))
      (metadata_def 'OccurrenceUsage' :> 'Usage'
        (attribute_usage 'isIndividual' : 'Boolean' multiplicity)
        (attribute_usage 'portionKind' : 'PortionKind' multiplicity)
        (item_usage derived ref 'occurrenceDefinition' : 'Class' :>> 'definition' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'individualDefinition' : 'OccurrenceDefinition' :> 'occurrenceDefinition' :> 'Metadata::metadataItems' multiplicity))
      (metadata_def 'PartDefinition' :> 'ItemDefinition')
      (metadata_def 'PartUsage' :> 'ItemUsage'
        (item_usage derived ref 'partDefinition' : 'PartDefinition' :> 'itemDefinition' :> 'Metadata::metadataItems' multiplicity ordered))
      (metadata_def 'PerformActionUsage' :> 'ActionUsage', 'EventOccurrenceUsage'
        (item_usage derived ref 'performedAction' : 'ActionUsage' :>> 'eventOccurrence' :> 'Metadata::metadataItems' multiplicity))
      (metadata_def 'PortConjugation' :> 'Conjugation'
        (item_usage ref 'originalPortDefinition' : 'PortDefinition' :>> 'originalType' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'conjugatedPortDefinition' : 'ConjugatedPortDefinition' :>> 'owningType' :> 'Metadata::metadataItems' multiplicity))
      (metadata_def 'PortDefinition' :> 'OccurrenceDefinition', 'Structure'
        (item_usage derived ref 'conjugatedPortDefinition' : 'ConjugatedPortDefinition' :> 'ownedMember' :> 'Metadata::metadataItems' multiplicity))
      (metadata_def 'PortUsage' :> 'OccurrenceUsage'
        (item_usage derived ref 'portDefinition' : 'PortDefinition' :>> 'occurrenceDefinition' :> 'Metadata::metadataItems' multiplicity ordered))
      (enum_def 'PortionKind'
        (enum_value ''timeslice'')
        (enum_value ''snapshot''))
      (metadata_def 'ReferenceUsage' :> 'Usage'
        (attribute_usage derived 'isReference' : 'Boolean' :>> 'isReference' multiplicity))
      (metadata_def 'RenderingDefinition' :> 'PartDefinition'
        (item_usage derived ref ''rendering'' : 'RenderingUsage' :> 'usage' :> 'Metadata::metadataItems' multiplicity ordered))
      (metadata_def 'RenderingUsage' :> 'PartUsage'
        (item_usage derived ref 'renderingDefinition' : 'RenderingDefinition' :>> 'partDefinition' :> 'Metadata::metadataItems' multiplicity))
      (enum_def 'RequirementConstraintKind'
        (enum_value 'assumption')
        (enum_value ''requirement''))
      (metadata_def 'RequirementConstraintMembership' :> 'FeatureMembership'
        (attribute_usage 'kind' : 'RequirementConstraintKind' multiplicity)
        (item_usage derived 'ownedConstraint' : 'ConstraintUsage' :>> 'ownedMemberFeature' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'referencedConstraint' : 'ConstraintUsage' :> 'Metadata::metadataItems' multiplicity))
      (metadata_def 'RequirementDefinition' :> 'ConstraintDefinition'
        (attribute_usage 'reqId' : 'String' :>> 'declaredShortName' multiplicity)
        (attribute_usage derived 'text' : 'String' multiplicity)
        (item_usage derived ref 'subjectParameter' : 'Usage' :> 'parameter', 'usage' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'actorParameter' : 'PartUsage' :> 'parameter', 'usage' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'stakeholderParameter' : 'PartUsage' :> 'parameter', 'usage' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'assumedConstraint' : 'ConstraintUsage' :> 'ownedFeature' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'requiredConstraint' : 'ConstraintUsage' :> 'ownedFeature' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'framedConcern' : 'ConcernUsage' :> 'requiredConstraint' :> 'Metadata::metadataItems' multiplicity ordered))
      (metadata_def 'RequirementUsage' :> 'ConstraintUsage'
        (attribute_usage 'reqId' : 'String' :>> 'declaredShortName' multiplicity)
        (attribute_usage derived 'text' : 'String' multiplicity)
        (item_usage derived ref 'requirementDefinition' : 'RequirementDefinition' :>> 'constraintDefinition' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'requiredConstraint' : 'ConstraintUsage' :> 'ownedFeature' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'assumedConstraint' : 'ConstraintUsage' :> 'ownedFeature' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'subjectParameter' : 'Usage' :> 'parameter', 'usage' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'framedConcern' : 'ConcernUsage' :> 'requiredConstraint' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'actorParameter' : 'PartUsage' :> 'parameter', 'usage' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'stakeholderParameter' : 'PartUsage' :> 'parameter', 'usage' :> 'Metadata::metadataItems' multiplicity ordered))
      (metadata_def 'RequirementVerificationMembership' :> 'RequirementConstraintMembership'
        (attribute_usage 'kind' : 'RequirementConstraintKind' :>> 'kind' multiplicity)
        (item_usage derived 'ownedRequirement' : 'RequirementUsage' :>> 'ownedConstraint' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'verifiedRequirement' : 'RequirementUsage' :>> 'referencedConstraint' :> 'Metadata::metadataItems' multiplicity))
      (metadata_def 'SatisfyRequirementUsage' :> 'RequirementUsage', 'AssertConstraintUsage'
        (item_usage derived ref 'satisfiedRequirement' : 'RequirementUsage' :>> 'assertedConstraint' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'satisfyingFeature' : 'Feature' :> 'Metadata::metadataItems' multiplicity))
      (metadata_def 'SendActionUsage' :> 'ActionUsage'
        (item_usage derived ref 'receiverArgument' : 'Expression' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'payloadArgument' : 'Expression' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'senderArgument' : 'Expression' :> 'Metadata::metadataItems' multiplicity))
      (metadata_def 'StakeholderMembership' :> 'ParameterMembership'
        (item_usage derived 'ownedStakeholderParameter' : 'PartUsage' :>> 'ownedMemberParameter' :> 'Metadata::metadataItems' multiplicity))
      (metadata_def 'StateDefinition' :> 'ActionDefinition'
        (attribute_usage 'isParallel' : 'Boolean' multiplicity)
        (item_usage derived ref ''state'' : 'StateUsage' :> ''action'' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'entryAction' : 'ActionUsage' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'doAction' : 'ActionUsage' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'exitAction' : 'ActionUsage' :> 'Metadata::metadataItems' multiplicity))
      (enum_def 'StateSubactionKind'
        (enum_value ''entry'')
        (enum_value ''do'')
        (enum_value ''exit''))
      (metadata_def 'StateSubactionMembership' :> 'FeatureMembership'
        (attribute_usage 'kind' : 'StateSubactionKind' multiplicity)
        (item_usage derived ''action'' : 'ActionUsage' :>> 'ownedMemberFeature' :> 'Metadata::metadataItems' multiplicity))
      (metadata_def 'StateUsage' :> 'ActionUsage'
        (attribute_usage 'isParallel' : 'Boolean' multiplicity)
        (item_usage derived ref 'stateDefinition' : 'Behavior' :>> 'actionDefinition' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'entryAction' : 'ActionUsage' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'doAction' : 'ActionUsage' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'exitAction' : 'ActionUsage' :> 'Metadata::metadataItems' multiplicity))
      (metadata_def 'SubjectMembership' :> 'ParameterMembership'
        (item_usage derived 'ownedSubjectParameter' : 'Usage' :>> 'ownedMemberParameter' :> 'Metadata::metadataItems' multiplicity))
      (metadata_def 'SuccessionAsUsage' :> 'ConnectorAsUsage', 'Succession')
      (metadata_def 'SuccessionFlowUsage' :> 'SuccessionFlow', 'FlowUsage')
      (metadata_def 'TerminateActionUsage' :> 'ActionUsage'
        (item_usage derived ref 'terminatedOccurrenceArgument' : 'Expression' :> 'Metadata::metadataItems' multiplicity))
      (enum_def 'TransitionFeatureKind'
        (enum_value 'trigger')
        (enum_value 'guard')
        (enum_value 'effect'))
      (metadata_def 'TransitionFeatureMembership' :> 'FeatureMembership'
        (attribute_usage 'kind' : 'TransitionFeatureKind' multiplicity)
        (item_usage derived 'transitionFeature' : 'Step' :>> 'ownedMemberFeature' :> 'Metadata::metadataItems' multiplicity))
      (metadata_def 'TransitionUsage' :> 'ActionUsage'
        (item_usage derived ref 'source' : 'ActionUsage' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'target' : 'ActionUsage' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'triggerAction' : 'AcceptActionUsage' :> 'ownedFeature' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'guardExpression' : 'Expression' :> 'ownedFeature' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'effectAction' : 'ActionUsage' :> 'feature' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref ''succession'' : 'Succession' :> 'ownedMember' :> 'Metadata::metadataItems' multiplicity))
      (metadata_def 'TriggerInvocationExpression' :> 'InvocationExpression'
        (attribute_usage 'kind' : 'TriggerKind' multiplicity))
      (enum_def 'TriggerKind'
        (enum_value ''when'')
        (enum_value ''at'')
        (enum_value ''after''))
      (metadata_def 'Usage' :> 'Feature'
        (attribute_usage 'isVariation' : 'Boolean' multiplicity)
        (attribute_usage derived 'mayTimeVary' : 'Boolean' :>> 'isVariable' multiplicity)
        (attribute_usage derived 'isReference' : 'Boolean' multiplicity)
        (item_usage derived ref ''variant'' : 'Usage' :> 'ownedMember' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived 'variantMembership' : 'VariantMembership' :> 'ownedMembership' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'owningDefinition' : 'Definition' :> 'owningType' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'owningUsage' : 'Usage' :> 'owningType' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'definition' : 'Classifier' :>> 'type' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'usage' : 'Usage' :> 'feature' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'directedUsage' : 'Usage' :> 'directedFeature', 'usage' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'nestedUsage' : 'Usage' :> 'ownedFeature', 'usage' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'nestedReference' : 'ReferenceUsage' :> 'nestedUsage' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'nestedAttribute' : 'AttributeUsage' :> 'nestedUsage' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'nestedEnumeration' : 'EnumerationUsage' :> 'nestedAttribute' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'nestedOccurrence' : 'OccurrenceUsage' :> 'nestedUsage' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'nestedItem' : 'ItemUsage' :> 'nestedOccurrence' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'nestedPart' : 'PartUsage' :> 'nestedItem' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'nestedPort' : 'PortUsage' :> 'nestedUsage' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'nestedConnection' : 'ConnectorAsUsage' :> 'nestedUsage' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'nestedFlow' : 'FlowUsage' :> 'nestedConnection' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'nestedInterface' : 'InterfaceUsage' :> 'nestedConnection' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'nestedAllocation' : 'AllocationUsage' :> 'nestedConnection' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'nestedAction' : 'ActionUsage' :> 'nestedOccurrence' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'nestedState' : 'StateUsage' :> 'nestedAction' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'nestedTransition' : 'TransitionUsage' :> 'nestedUsage' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'nestedCalculation' : 'CalculationUsage' :> 'nestedAction' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'nestedConstraint' : 'ConstraintUsage' :> 'nestedOccurrence' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'nestedRequirement' : 'RequirementUsage' :> 'nestedConstraint' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'nestedConcern' : 'ConcernUsage' :> 'nestedRequirement' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'nestedCase' : 'CaseUsage' :> 'nestedCalculation' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'nestedAnalysisCase' : 'AnalysisCaseUsage' :> 'nestedCase' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'nestedVerificationCase' : 'VerificationCaseUsage' :> 'nestedCase' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'nestedUseCase' : 'UseCaseUsage' :> 'nestedCase' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'nestedView' : 'ViewUsage' :> 'nestedPart' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'nestedViewpoint' : 'ViewpointUsage' :> 'nestedRequirement' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'nestedRendering' : 'RenderingUsage' :> 'nestedPart' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'nestedMetadata' : 'MetadataUsage' :> 'nestedItem' :> 'Metadata::metadataItems' multiplicity ordered))
      (metadata_def 'UseCaseDefinition' :> 'CaseDefinition'
        (item_usage derived ref 'includedUseCase' : 'UseCaseUsage' :> 'Metadata::metadataItems' multiplicity ordered))
      (metadata_def 'UseCaseUsage' :> 'CaseUsage'
        (item_usage derived ref 'useCaseDefinition' : 'UseCaseDefinition' :>> 'caseDefinition' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'includedUseCase' : 'UseCaseUsage' :> 'Metadata::metadataItems' multiplicity ordered))
      (metadata_def 'VariantMembership' :> 'OwningMembership'
        (item_usage derived 'ownedVariantUsage' : 'Usage' :>> 'ownedMemberElement' :> 'Metadata::metadataItems' multiplicity))
      (metadata_def 'VerificationCaseDefinition' :> 'CaseDefinition'
        (item_usage derived ref 'verifiedRequirement' : 'RequirementUsage' :> 'Metadata::metadataItems' multiplicity ordered))
      (metadata_def 'VerificationCaseUsage' :> 'CaseUsage'
        (item_usage derived ref 'verificationCaseDefinition' : 'VerificationCaseDefinition' :> 'caseDefinition' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'verifiedRequirement' : 'RequirementUsage' :> 'Metadata::metadataItems' multiplicity ordered))
      (metadata_def 'ViewDefinition' :> 'PartDefinition'
        (item_usage derived ref ''view'' : 'ViewUsage' :> 'usage' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'satisfiedViewpoint' : 'ViewpointUsage' :> 'ownedRequirement' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'viewRendering' : 'RenderingUsage' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'viewCondition' : 'Expression' :> 'ownedMember' :> 'Metadata::metadataItems' multiplicity ordered))
      (metadata_def 'ViewRenderingMembership' :> 'FeatureMembership'
        (item_usage derived 'ownedRendering' : 'RenderingUsage' :>> 'ownedMemberFeature' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'referencedRendering' : 'RenderingUsage' :> 'Metadata::metadataItems' multiplicity))
      (metadata_def 'ViewUsage' :> 'PartUsage'
        (item_usage derived ref 'viewDefinition' : 'ViewDefinition' :>> 'partDefinition' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'satisfiedViewpoint' : 'ViewpointUsage' :> 'nestedRequirement' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'exposedElement' : 'Element' :> 'member' :> 'Metadata::metadataItems' multiplicity ordered)
        (item_usage derived ref 'viewRendering' : 'RenderingUsage' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'viewCondition' : 'Expression' :> 'ownedMember' :> 'Metadata::metadataItems' multiplicity ordered))
      (metadata_def 'ViewpointDefinition' :> 'RequirementDefinition'
        (item_usage derived ref 'viewpointStakeholder' : 'PartUsage' :> 'Metadata::metadataItems' multiplicity ordered))
      (metadata_def 'ViewpointUsage' :> 'RequirementUsage'
        (item_usage derived ref 'viewpointDefinition' : 'ViewpointDefinition' :>> 'requirementDefinition' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'viewpointStakeholder' : 'PartUsage' :> 'Metadata::metadataItems' multiplicity ordered))
      (metadata_def 'WhileLoopActionUsage' :> 'LoopActionUsage'
        (item_usage derived ref 'whileArgument' : 'Expression' :> 'Metadata::metadataItems' multiplicity)
        (item_usage derived ref 'untilArgument' : 'Expression' :> 'Metadata::metadataItems' multiplicity)))))
~~~
# FORMAT
~~~sysml
standard library package SysML {
    doc /*
	 * This package contains a reflective KerML model of the KerML abstract syntax.
	 */

    private import ScalarValues::*;
    public import Systems::*;

    package Systems {
        public import KerML::Kernel::*;

        metadata def AcceptActionUsage specializes ActionUsage {
            derived ref item receiverArgument : Expression subsets Metadata::metadataItems [0..1];
            derived ref item payloadParameter : ReferenceUsage subsets nestedReference, parameter subsets Metadata::metadataItems [1..1];
            derived ref item payloadArgument : Expression subsets Metadata::metadataItems [0..1];
        }

        metadata def ActionDefinition specializes Behavior, OccurrenceDefinition {
            derived ref item 'action' : ActionUsage subsets step, usage subsets Metadata::metadataItems [0..*] ordered;
        }

        metadata def ActionUsage specializes Step, OccurrenceUsage {
            derived ref item actionDefinition : Behavior redefines behavior, occurrenceDefinition subsets Metadata::metadataItems [0..*] ordered;
        }

        metadata def ActorMembership specializes ParameterMembership {
            derived item ownedActorParameter : PartUsage redefines ownedMemberParameter subsets Metadata::metadataItems [1..1];
        }

        metadata def AllocationDefinition specializes ConnectionDefinition {
            derived ref item 'allocation' : AllocationUsage subsets usage subsets Metadata::metadataItems [0..*] ordered;
        }

        metadata def AllocationUsage specializes ConnectionUsage {
            derived ref item allocationDefinition : AllocationDefinition redefines connectionDefinition subsets Metadata::metadataItems [0..*] ordered;
        }

        metadata def AnalysisCaseDefinition specializes CaseDefinition {
            derived ref item resultExpression : Expression subsets expression, ownedFeature subsets Metadata::metadataItems [0..1];
        }

        metadata def AnalysisCaseUsage specializes CaseUsage {
            derived ref item analysisCaseDefinition : AnalysisCaseDefinition redefines caseDefinition subsets Metadata::metadataItems [0..1];
            derived ref item resultExpression : Expression subsets ownedFeature subsets Metadata::metadataItems [0..1];
        }

        metadata def AssertConstraintUsage specializes ConstraintUsage, Invariant {
            derived ref item assertedConstraint : ConstraintUsage subsets Metadata::metadataItems [1..1];
        }

        metadata def AssignmentActionUsage specializes ActionUsage {
            derived ref item targetArgument : Expression subsets Metadata::metadataItems [0..1];
            derived ref item valueExpression : Expression subsets Metadata::metadataItems [0..1];
            derived ref item referent : Feature subsets member subsets Metadata::metadataItems [1..1];
        }

        metadata def AttributeDefinition specializes DataType, Definition;

        metadata def AttributeUsage specializes Usage {
            derived attribute isReference : Boolean redefines isReference [1..1];

            derived ref item attributeDefinition : DataType redefines definition subsets Metadata::metadataItems [0..*] ordered;
        }

        metadata def BindingConnectorAsUsage specializes BindingConnector, ConnectorAsUsage;

        metadata def CalculationDefinition specializes Function, ActionDefinition {
            derived ref item calculation : CalculationUsage subsets 'action', expression subsets Metadata::metadataItems [0..*] ordered;
        }

        metadata def CalculationUsage specializes Expression, ActionUsage {
            derived ref item calculationDefinition : Function redefines function, actionDefinition subsets Metadata::metadataItems [0..1] ordered;
        }

        metadata def CaseDefinition specializes CalculationDefinition {
            derived ref item objectiveRequirement : RequirementUsage subsets usage subsets Metadata::metadataItems [0..1] ordered;
            derived ref item subjectParameter : Usage subsets parameter, usage subsets Metadata::metadataItems [1..1];
            derived ref item actorParameter : PartUsage subsets parameter, usage subsets Metadata::metadataItems [0..*] ordered;
        }

        metadata def CaseUsage specializes CalculationUsage {
            derived ref item objectiveRequirement : RequirementUsage subsets usage subsets Metadata::metadataItems [0..1] ordered;
            derived ref item caseDefinition : CaseDefinition redefines calculationDefinition subsets Metadata::metadataItems [0..1];
            derived ref item subjectParameter : Usage subsets parameter, usage subsets Metadata::metadataItems [1..1];
            derived ref item actorParameter : PartUsage subsets parameter, usage subsets Metadata::metadataItems [0..*] ordered;
        }

        metadata def ConcernDefinition specializes RequirementDefinition;

        metadata def ConcernUsage specializes RequirementUsage {
            derived ref item concernDefinition : ConcernDefinition redefines requirementDefinition subsets Metadata::metadataItems [0..1];
        }

        metadata def ConjugatedPortDefinition specializes PortDefinition {
            derived ref item originalPortDefinition : PortDefinition redefines owningNamespace subsets Metadata::metadataItems [1..1];
            derived ref item ownedPortConjugator : PortConjugation redefines ownedConjugator subsets Metadata::metadataItems [1..1];
        }

        metadata def ConjugatedPortTyping specializes FeatureTyping {
            ref item conjugatedPortDefinition : ConjugatedPortDefinition redefines type subsets Metadata::metadataItems [1..1];
            derived ref item portDefinition : PortDefinition subsets Metadata::metadataItems [1..1];
        }

        metadata def ConnectionDefinition specializes AssociationStructure, PartDefinition {
            attribute isSufficient : Boolean redefines isSufficient [1..1];

            derived ref item connectionEnd : Usage redefines associationEnd subsets Metadata::metadataItems [0..*] ordered;
        }

        metadata def ConnectionUsage specializes ConnectorAsUsage, PartUsage {
            derived ref item connectionDefinition : AssociationStructure subsets itemDefinition redefines association subsets Metadata::metadataItems [0..*] ordered;
        }

        abstract metadata def ConnectorAsUsage specializes Usage, Connector;

        metadata def ConstraintDefinition specializes OccurrenceDefinition, Predicate;

        metadata def ConstraintUsage specializes BooleanExpression, OccurrenceUsage {
            derived ref item constraintDefinition : Predicate redefines predicate subsets Metadata::metadataItems [0..1];
        }

        abstract metadata def ControlNode specializes ActionUsage;

        metadata def DecisionNode specializes ControlNode;

        metadata def Definition specializes Classifier {
            attribute isVariation : Boolean [1..1];

            derived ref item 'variant' : Usage subsets ownedMember subsets Metadata::metadataItems [0..*];
            derived item variantMembership : VariantMembership subsets ownedMembership subsets Metadata::metadataItems [0..*];
            derived ref item usage : Usage subsets feature subsets Metadata::metadataItems [0..*] ordered;
            derived ref item directedUsage : Usage subsets directedFeature, usage subsets Metadata::metadataItems [0..*] ordered;
            derived ref item ownedUsage : Usage subsets ownedFeature, usage subsets Metadata::metadataItems [0..*] ordered;
            derived ref item ownedReference : ReferenceUsage subsets ownedUsage subsets Metadata::metadataItems [0..*] ordered;
            derived ref item ownedAttribute : AttributeUsage subsets ownedUsage subsets Metadata::metadataItems [0..*] ordered;
            derived ref item ownedEnumeration : EnumerationUsage subsets ownedAttribute subsets Metadata::metadataItems [0..*] ordered;
            derived ref item ownedOccurrence : OccurrenceUsage subsets ownedUsage subsets Metadata::metadataItems [0..*] ordered;
            derived ref item ownedItem : ItemUsage subsets ownedOccurrence subsets Metadata::metadataItems [0..*] ordered;
            derived ref item ownedPart : PartUsage subsets ownedItem subsets Metadata::metadataItems [0..*] ordered;
            derived ref item ownedPort : PortUsage subsets ownedUsage subsets Metadata::metadataItems [0..*] ordered;
            derived ref item ownedConnection : ConnectorAsUsage subsets ownedUsage subsets Metadata::metadataItems [0..*] ordered;
            derived ref item ownedFlow : FlowUsage subsets ownedConnection subsets Metadata::metadataItems [0..*];
            derived ref item ownedInterface : InterfaceUsage subsets ownedConnection subsets Metadata::metadataItems [0..*] ordered;
            derived ref item ownedAllocation : AllocationUsage subsets ownedConnection subsets Metadata::metadataItems [0..*] ordered;
            derived ref item ownedAction : ActionUsage subsets ownedOccurrence subsets Metadata::metadataItems [0..*] ordered;
            derived ref item ownedState : StateUsage subsets ownedAction subsets Metadata::metadataItems [0..*] ordered;
            derived ref item ownedTransition : TransitionUsage subsets ownedUsage subsets Metadata::metadataItems [0..*];
            derived ref item ownedCalculation : CalculationUsage subsets ownedAction subsets Metadata::metadataItems [0..*] ordered;
            derived ref item ownedConstraint : ConstraintUsage subsets ownedOccurrence subsets Metadata::metadataItems [0..*] ordered;
            derived ref item ownedRequirement : RequirementUsage subsets ownedConstraint subsets Metadata::metadataItems [0..*] ordered;
            derived ref item ownedConcern : ConcernUsage subsets ownedRequirement subsets Metadata::metadataItems [0..*];
            derived ref item ownedCase : CaseUsage subsets ownedCalculation subsets Metadata::metadataItems [0..*] ordered;
            derived ref item ownedAnalysisCase : AnalysisCaseUsage subsets ownedCase subsets Metadata::metadataItems [0..*] ordered;
            derived ref item ownedVerificationCase : VerificationCaseUsage subsets ownedCase subsets Metadata::metadataItems [0..*] ordered;
            derived ref item ownedUseCase : UseCaseUsage subsets ownedCase subsets Metadata::metadataItems [0..*] ordered;
            derived ref item ownedView : ViewUsage subsets ownedPart subsets Metadata::metadataItems [0..*] ordered;
            derived ref item ownedViewpoint : ViewpointUsage subsets ownedRequirement subsets Metadata::metadataItems [0..*] ordered;
            derived ref item ownedRendering : RenderingUsage subsets ownedPart subsets Metadata::metadataItems [0..*] ordered;
            derived ref item ownedMetadata : MetadataUsage subsets ownedItem subsets Metadata::metadataItems [0..*] ordered;
        }

        metadata def EnumerationDefinition specializes AttributeDefinition {
            attribute isVariation : Boolean redefines isVariation [1..1];

            derived ref item enumeratedValue : EnumerationUsage redefines 'variant' subsets Metadata::metadataItems [0..*] ordered;
        }

        metadata def EnumerationUsage specializes AttributeUsage {
            derived ref item enumerationDefinition : EnumerationDefinition redefines attributeDefinition subsets Metadata::metadataItems [1..1];
        }

        metadata def EventOccurrenceUsage specializes OccurrenceUsage {
            derived attribute isReference : Boolean redefines isReference [1..1];

            derived ref item eventOccurrence : OccurrenceUsage subsets Metadata::metadataItems [1..1];
        }

        metadata def ExhibitStateUsage specializes StateUsage, PerformActionUsage {
            derived ref item exhibitedState : StateUsage redefines performedAction subsets Metadata::metadataItems [1..1];
        }

        abstract metadata def Expose specializes Import {
            attribute visibility : VisibilityKind redefines visibility [1..1];
            attribute isImportAll : Boolean redefines isImportAll [1..1];
        }

        metadata def FlowDefinition specializes Interaction, ActionDefinition {
            derived ref item flowEnd : Usage redefines associationEnd subsets Metadata::metadataItems [0..*];
        }

        metadata def FlowUsage specializes ConnectorAsUsage, Flow, ActionUsage {
            derived ref item flowDefinition : Interaction redefines actionDefinition, interaction subsets Metadata::metadataItems [0..*] ordered;
        }

        metadata def ForLoopActionUsage specializes LoopActionUsage {
            derived ref item seqArgument : Expression subsets Metadata::metadataItems [1..1];
            derived ref item loopVariable : ReferenceUsage subsets Metadata::metadataItems [1..1];
        }

        metadata def ForkNode specializes ControlNode;

        metadata def FramedConcernMembership specializes RequirementConstraintMembership {
            attribute kind : RequirementConstraintKind redefines kind [1..1];

            derived item ownedConcern : ConcernUsage redefines ownedConstraint subsets Metadata::metadataItems [1..1];
            derived ref item referencedConcern : ConcernUsage redefines referencedConstraint subsets Metadata::metadataItems [1..1];
        }

        metadata def IfActionUsage specializes ActionUsage {
            derived ref item elseAction : ActionUsage subsets Metadata::metadataItems [0..1];
            derived ref item thenAction : ActionUsage subsets Metadata::metadataItems [1..1];
            derived ref item ifArgument : Expression subsets Metadata::metadataItems [1..1];
        }

        metadata def IncludeUseCaseUsage specializes UseCaseUsage, PerformActionUsage {
            derived ref item useCaseIncluded : UseCaseUsage redefines performedAction subsets Metadata::metadataItems [1..1];
        }

        metadata def InterfaceDefinition specializes ConnectionDefinition {
            derived ref item interfaceEnd : PortUsage redefines connectionEnd subsets Metadata::metadataItems [0..*] ordered;
        }

        metadata def InterfaceUsage specializes ConnectionUsage {
            derived ref item interfaceDefinition : InterfaceDefinition redefines connectionDefinition subsets Metadata::metadataItems [0..*];
        }

        metadata def ItemDefinition specializes Structure, OccurrenceDefinition;

        metadata def ItemUsage specializes OccurrenceUsage {
            derived ref item itemDefinition : Structure subsets occurrenceDefinition subsets Metadata::metadataItems subsets Metadata::metadataItems [0..*] ordered;
        }

        metadata def JoinNode specializes ControlNode;

        abstract metadata def LoopActionUsage specializes ActionUsage {
            derived ref item bodyAction : ActionUsage subsets Metadata::metadataItems [1..1];
        }

        metadata def MembershipExpose specializes MembershipImport, Expose;

        metadata def MergeNode specializes ControlNode;

        metadata def MetadataDefinition specializes ItemDefinition, Metaclass;

        metadata def MetadataUsage specializes ItemUsage, MetadataFeature {
            derived ref item metadataDefinition : Metaclass redefines itemDefinition, metaclass subsets Metadata::metadataItems [0..1];
        }

        metadata def NamespaceExpose specializes Expose, NamespaceImport;

        metadata def ObjectiveMembership specializes FeatureMembership {
            derived item ownedObjectiveRequirement : RequirementUsage redefines ownedMemberFeature subsets Metadata::metadataItems [1..1];
        }

        metadata def OccurrenceDefinition specializes Definition, Class {
            attribute isIndividual : Boolean [1..1];
        }

        metadata def OccurrenceUsage specializes Usage {
            attribute isIndividual : Boolean [1..1];
            attribute portionKind : PortionKind [0..1];

            derived ref item occurrenceDefinition : Class redefines definition subsets Metadata::metadataItems [0..*] ordered;
            derived ref item individualDefinition : OccurrenceDefinition subsets occurrenceDefinition subsets Metadata::metadataItems [0..1];
        }

        metadata def PartDefinition specializes ItemDefinition;

        metadata def PartUsage specializes ItemUsage {
            derived ref item partDefinition : PartDefinition subsets itemDefinition subsets Metadata::metadataItems [0..*] ordered;
        }

        metadata def PerformActionUsage specializes ActionUsage, EventOccurrenceUsage {
            derived ref item performedAction : ActionUsage redefines eventOccurrence subsets Metadata::metadataItems [1..1];
        }

        metadata def PortConjugation specializes Conjugation {
            ref item originalPortDefinition : PortDefinition redefines originalType subsets Metadata::metadataItems [1..1];
            derived ref item conjugatedPortDefinition : ConjugatedPortDefinition redefines owningType subsets Metadata::metadataItems [1..1];
        }

        metadata def PortDefinition specializes OccurrenceDefinition, Structure {
            derived ref item conjugatedPortDefinition : ConjugatedPortDefinition subsets ownedMember subsets Metadata::metadataItems [0..1];
        }

        metadata def PortUsage specializes OccurrenceUsage {
            derived ref item portDefinition : PortDefinition redefines occurrenceDefinition subsets Metadata::metadataItems [0..*] ordered;
        }

        enum def PortionKind {
            enum 'timeslice';
            enum 'snapshot';
        }

        metadata def ReferenceUsage specializes Usage {
            derived attribute isReference : Boolean redefines isReference [1..1];
        }

        metadata def RenderingDefinition specializes PartDefinition {
            derived ref item 'rendering' : RenderingUsage subsets usage subsets Metadata::metadataItems [0..*] ordered;
        }

        metadata def RenderingUsage specializes PartUsage {
            derived ref item renderingDefinition : RenderingDefinition redefines partDefinition subsets Metadata::metadataItems [0..1];
        }

        enum def RequirementConstraintKind {
            enum assumption;
            enum 'requirement';
        }

        metadata def RequirementConstraintMembership specializes FeatureMembership {
            attribute kind : RequirementConstraintKind [1..1];

            derived item ownedConstraint : ConstraintUsage redefines ownedMemberFeature subsets Metadata::metadataItems [1..1];
            derived ref item referencedConstraint : ConstraintUsage subsets Metadata::metadataItems [1..1];
        }

        metadata def RequirementDefinition specializes ConstraintDefinition {
            attribute reqId : String redefines declaredShortName [0..1];
            derived attribute text : String [0..*];

            derived ref item subjectParameter : Usage subsets parameter, usage subsets Metadata::metadataItems [1..1];
            derived ref item actorParameter : PartUsage subsets parameter, usage subsets Metadata::metadataItems [0..*] ordered;
            derived ref item stakeholderParameter : PartUsage subsets parameter, usage subsets Metadata::metadataItems [0..*] ordered;
            derived ref item assumedConstraint : ConstraintUsage subsets ownedFeature subsets Metadata::metadataItems [0..*] ordered;
            derived ref item requiredConstraint : ConstraintUsage subsets ownedFeature subsets Metadata::metadataItems [0..*] ordered;
            derived ref item framedConcern : ConcernUsage subsets requiredConstraint subsets Metadata::metadataItems [0..*] ordered;
        }

        metadata def RequirementUsage specializes ConstraintUsage {
            attribute reqId : String redefines declaredShortName [0..1];
            derived attribute text : String [0..*];

            derived ref item requirementDefinition : RequirementDefinition redefines constraintDefinition subsets Metadata::metadataItems [0..1];
            derived ref item requiredConstraint : ConstraintUsage subsets ownedFeature subsets Metadata::metadataItems [0..*] ordered;
            derived ref item assumedConstraint : ConstraintUsage subsets ownedFeature subsets Metadata::metadataItems [0..*] ordered;
            derived ref item subjectParameter : Usage subsets parameter, usage subsets Metadata::metadataItems [1..1];
            derived ref item framedConcern : ConcernUsage subsets requiredConstraint subsets Metadata::metadataItems [0..*] ordered;
            derived ref item actorParameter : PartUsage subsets parameter, usage subsets Metadata::metadataItems [0..*] ordered;
            derived ref item stakeholderParameter : PartUsage subsets parameter, usage subsets Metadata::metadataItems [0..*] ordered;
        }

        metadata def RequirementVerificationMembership specializes RequirementConstraintMembership {
            attribute kind : RequirementConstraintKind redefines kind [1..1];

            derived item ownedRequirement : RequirementUsage redefines ownedConstraint subsets Metadata::metadataItems [1..1];
            derived ref item verifiedRequirement : RequirementUsage redefines referencedConstraint subsets Metadata::metadataItems [1..1];
        }

        metadata def SatisfyRequirementUsage specializes RequirementUsage, AssertConstraintUsage {
            derived ref item satisfiedRequirement : RequirementUsage redefines assertedConstraint subsets Metadata::metadataItems [1..1];
            derived ref item satisfyingFeature : Feature subsets Metadata::metadataItems [1..1];
        }

        metadata def SendActionUsage specializes ActionUsage {
            derived ref item receiverArgument : Expression subsets Metadata::metadataItems [0..1];
            derived ref item payloadArgument : Expression subsets Metadata::metadataItems [1..1];
            derived ref item senderArgument : Expression subsets Metadata::metadataItems [0..1];
        }

        metadata def StakeholderMembership specializes ParameterMembership {
            derived item ownedStakeholderParameter : PartUsage redefines ownedMemberParameter subsets Metadata::metadataItems [1..1];
        }

        metadata def StateDefinition specializes ActionDefinition {
            attribute isParallel : Boolean [1..1];

            derived ref item 'state' : StateUsage subsets 'action' subsets Metadata::metadataItems [0..*] ordered;
            derived ref item entryAction : ActionUsage subsets Metadata::metadataItems [0..1];
            derived ref item doAction : ActionUsage subsets Metadata::metadataItems [0..1];
            derived ref item exitAction : ActionUsage subsets Metadata::metadataItems [0..1];
        }

        enum def StateSubactionKind {
            enum 'entry';
            enum 'do';
            enum 'exit';
        }

        metadata def StateSubactionMembership specializes FeatureMembership {
            attribute kind : StateSubactionKind [1..1];

            derived item 'action' : ActionUsage redefines ownedMemberFeature subsets Metadata::metadataItems [1..1];
        }

        metadata def StateUsage specializes ActionUsage {
            attribute isParallel : Boolean [1..1];

            derived ref item stateDefinition : Behavior redefines actionDefinition subsets Metadata::metadataItems [0..*] ordered;
            derived ref item entryAction : ActionUsage subsets Metadata::metadataItems [0..1];
            derived ref item doAction : ActionUsage subsets Metadata::metadataItems [0..1];
            derived ref item exitAction : ActionUsage subsets Metadata::metadataItems [0..1];
        }

        metadata def SubjectMembership specializes ParameterMembership {
            derived item ownedSubjectParameter : Usage redefines ownedMemberParameter subsets Metadata::metadataItems [1..1];
        }

        metadata def SuccessionAsUsage specializes ConnectorAsUsage, Succession;

        metadata def SuccessionFlowUsage specializes SuccessionFlow, FlowUsage;

        metadata def TerminateActionUsage specializes ActionUsage {
            derived ref item terminatedOccurrenceArgument : Expression subsets Metadata::metadataItems [0..1];
        }

        enum def TransitionFeatureKind {
            enum trigger;
            enum guard;
            enum effect;
        }

        metadata def TransitionFeatureMembership specializes FeatureMembership {
            attribute kind : TransitionFeatureKind [1..1];

            derived item transitionFeature : Step redefines ownedMemberFeature subsets Metadata::metadataItems [1..1];
        }

        metadata def TransitionUsage specializes ActionUsage {
            derived ref item source : ActionUsage subsets Metadata::metadataItems [1..1];
            derived ref item target : ActionUsage subsets Metadata::metadataItems [1..1];
            derived ref item triggerAction : AcceptActionUsage subsets ownedFeature subsets Metadata::metadataItems [0..*];
            derived ref item guardExpression : Expression subsets ownedFeature subsets Metadata::metadataItems [0..*];
            derived ref item effectAction : ActionUsage subsets feature subsets Metadata::metadataItems [0..*];
            derived ref item 'succession' : Succession subsets ownedMember subsets Metadata::metadataItems [1..1];
        }

        metadata def TriggerInvocationExpression specializes InvocationExpression {
            attribute kind : TriggerKind [1..1];
        }

        enum def TriggerKind {
            enum 'when';
            enum 'at';
            enum 'after';
        }

        metadata def Usage specializes Feature {
            attribute isVariation : Boolean [1..1];
            derived attribute mayTimeVary : Boolean redefines isVariable [1..1];
            derived attribute isReference : Boolean [1..1];

            derived ref item 'variant' : Usage subsets ownedMember subsets Metadata::metadataItems [0..*];
            derived item variantMembership : VariantMembership subsets ownedMembership subsets Metadata::metadataItems [0..*];
            derived ref item owningDefinition : Definition subsets owningType subsets Metadata::metadataItems [0..1];
            derived ref item owningUsage : Usage subsets owningType subsets Metadata::metadataItems [0..1];
            derived ref item definition : Classifier redefines type subsets Metadata::metadataItems [0..*] ordered;
            derived ref item usage : Usage subsets feature subsets Metadata::metadataItems [0..*] ordered;
            derived ref item directedUsage : Usage subsets directedFeature, usage subsets Metadata::metadataItems [0..*] ordered;
            derived ref item nestedUsage : Usage subsets ownedFeature, usage subsets Metadata::metadataItems [0..*] ordered;
            derived ref item nestedReference : ReferenceUsage subsets nestedUsage subsets Metadata::metadataItems [0..*] ordered;
            derived ref item nestedAttribute : AttributeUsage subsets nestedUsage subsets Metadata::metadataItems [0..*] ordered;
            derived ref item nestedEnumeration : EnumerationUsage subsets nestedAttribute subsets Metadata::metadataItems [0..*] ordered;
            derived ref item nestedOccurrence : OccurrenceUsage subsets nestedUsage subsets Metadata::metadataItems [0..*] ordered;
            derived ref item nestedItem : ItemUsage subsets nestedOccurrence subsets Metadata::metadataItems [0..*] ordered;
            derived ref item nestedPart : PartUsage subsets nestedItem subsets Metadata::metadataItems [0..*] ordered;
            derived ref item nestedPort : PortUsage subsets nestedUsage subsets Metadata::metadataItems [0..*] ordered;
            derived ref item nestedConnection : ConnectorAsUsage subsets nestedUsage subsets Metadata::metadataItems [0..*] ordered;
            derived ref item nestedFlow : FlowUsage subsets nestedConnection subsets Metadata::metadataItems [0..*];
            derived ref item nestedInterface : InterfaceUsage subsets nestedConnection subsets Metadata::metadataItems [0..*] ordered;
            derived ref item nestedAllocation : AllocationUsage subsets nestedConnection subsets Metadata::metadataItems [0..*] ordered;
            derived ref item nestedAction : ActionUsage subsets nestedOccurrence subsets Metadata::metadataItems [0..*] ordered;
            derived ref item nestedState : StateUsage subsets nestedAction subsets Metadata::metadataItems [0..*] ordered;
            derived ref item nestedTransition : TransitionUsage subsets nestedUsage subsets Metadata::metadataItems [0..*];
            derived ref item nestedCalculation : CalculationUsage subsets nestedAction subsets Metadata::metadataItems [0..*] ordered;
            derived ref item nestedConstraint : ConstraintUsage subsets nestedOccurrence subsets Metadata::metadataItems [0..*] ordered;
            derived ref item nestedRequirement : RequirementUsage subsets nestedConstraint subsets Metadata::metadataItems [0..*] ordered;
            derived ref item nestedConcern : ConcernUsage subsets nestedRequirement subsets Metadata::metadataItems [0..*];
            derived ref item nestedCase : CaseUsage subsets nestedCalculation subsets Metadata::metadataItems [0..*] ordered;
            derived ref item nestedAnalysisCase : AnalysisCaseUsage subsets nestedCase subsets Metadata::metadataItems [0..*] ordered;
            derived ref item nestedVerificationCase : VerificationCaseUsage subsets nestedCase subsets Metadata::metadataItems [0..*] ordered;
            derived ref item nestedUseCase : UseCaseUsage subsets nestedCase subsets Metadata::metadataItems [0..*] ordered;
            derived ref item nestedView : ViewUsage subsets nestedPart subsets Metadata::metadataItems [0..*] ordered;
            derived ref item nestedViewpoint : ViewpointUsage subsets nestedRequirement subsets Metadata::metadataItems [0..*] ordered;
            derived ref item nestedRendering : RenderingUsage subsets nestedPart subsets Metadata::metadataItems [0..*] ordered;
            derived ref item nestedMetadata : MetadataUsage subsets nestedItem subsets Metadata::metadataItems [0..*] ordered;
        }

        metadata def UseCaseDefinition specializes CaseDefinition {
            derived ref item includedUseCase : UseCaseUsage subsets Metadata::metadataItems [0..*] ordered;
        }

        metadata def UseCaseUsage specializes CaseUsage {
            derived ref item useCaseDefinition : UseCaseDefinition redefines caseDefinition subsets Metadata::metadataItems [0..1];
            derived ref item includedUseCase : UseCaseUsage subsets Metadata::metadataItems [0..*] ordered;
        }

        metadata def VariantMembership specializes OwningMembership {
            derived item ownedVariantUsage : Usage redefines ownedMemberElement subsets Metadata::metadataItems [1..1];
        }

        metadata def VerificationCaseDefinition specializes CaseDefinition {
            derived ref item verifiedRequirement : RequirementUsage subsets Metadata::metadataItems [0..*] ordered;
        }

        metadata def VerificationCaseUsage specializes CaseUsage {
            derived ref item verificationCaseDefinition : VerificationCaseDefinition subsets caseDefinition subsets Metadata::metadataItems [0..1];
            derived ref item verifiedRequirement : RequirementUsage subsets Metadata::metadataItems [0..*] ordered;
        }

        metadata def ViewDefinition specializes PartDefinition {
            derived ref item 'view' : ViewUsage subsets usage subsets Metadata::metadataItems [0..*] ordered;
            derived ref item satisfiedViewpoint : ViewpointUsage subsets ownedRequirement subsets Metadata::metadataItems [0..*] ordered;
            derived ref item viewRendering : RenderingUsage subsets Metadata::metadataItems [0..1];
            derived ref item viewCondition : Expression subsets ownedMember subsets Metadata::metadataItems [0..*] ordered;
        }

        metadata def ViewRenderingMembership specializes FeatureMembership {
            derived item ownedRendering : RenderingUsage redefines ownedMemberFeature subsets Metadata::metadataItems [1..1];
            derived ref item referencedRendering : RenderingUsage subsets Metadata::metadataItems [1..1];
        }

        metadata def ViewUsage specializes PartUsage {
            derived ref item viewDefinition : ViewDefinition redefines partDefinition subsets Metadata::metadataItems [0..1];
            derived ref item satisfiedViewpoint : ViewpointUsage subsets nestedRequirement subsets Metadata::metadataItems [0..*] ordered;
            derived ref item exposedElement : Element subsets member subsets Metadata::metadataItems [0..*] ordered;
            derived ref item viewRendering : RenderingUsage subsets Metadata::metadataItems [0..1];
            derived ref item viewCondition : Expression subsets ownedMember subsets Metadata::metadataItems [0..*] ordered;
        }

        metadata def ViewpointDefinition specializes RequirementDefinition {
            derived ref item viewpointStakeholder : PartUsage subsets Metadata::metadataItems [0..*] ordered;
        }

        metadata def ViewpointUsage specializes RequirementUsage {
            derived ref item viewpointDefinition : ViewpointDefinition redefines requirementDefinition subsets Metadata::metadataItems [0..1];
            derived ref item viewpointStakeholder : PartUsage subsets Metadata::metadataItems [0..*] ordered;
        }

        metadata def WhileLoopActionUsage specializes LoopActionUsage {
            derived ref item whileArgument : Expression subsets Metadata::metadataItems [1..1];
            derived ref item untilArgument : Expression subsets Metadata::metadataItems [0..1];
        }
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'SysML'
      (documentation)
      (namespace_import private -> 'ScalarValues'[unresolved])
      (namespace_import public -> 'SysML::Systems'[package])
      (package 'Systems'
        (namespace_import public -> 'KerML::Kernel'[unresolved])
        (metadata_def 'AcceptActionUsage' :> 'SysML::Systems::ActionUsage'[metadata_def]
          (item_usage derived reference 'receiverArgument' : 'Expression'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..1]))
          (item_usage derived reference 'payloadParameter' : 'SysML::Systems::ReferenceUsage'[metadata_def] :> 'SysML::Systems::Usage::nestedReference'[item_usage] :> 'parameter'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [1..1]))
          (item_usage derived reference 'payloadArgument' : 'Expression'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..1])))
        (metadata_def 'ActionDefinition' :> 'Behavior'[unresolved] :> 'SysML::Systems::OccurrenceDefinition'[metadata_def]
          (item_usage derived reference ordered 'action' : 'SysML::Systems::ActionUsage'[metadata_def] :> 'step'[unresolved] :> 'SysML::Systems::Usage::usage'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*])))
        (metadata_def 'ActionUsage' :> 'Step'[unresolved] :> 'SysML::Systems::OccurrenceUsage'[metadata_def]
          (item_usage derived reference ordered 'actionDefinition' : 'Behavior'[unresolved] :>> 'behavior'[unresolved] :>> 'SysML::Systems::OccurrenceUsage::occurrenceDefinition'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*])))
        (metadata_def 'ActorMembership' :> 'ParameterMembership'[unresolved]
          (item_usage derived composite 'ownedActorParameter' : 'SysML::Systems::PartUsage'[metadata_def] :>> 'ownedMemberParameter'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [1..1])))
        (metadata_def 'AllocationDefinition' :> 'SysML::Systems::ConnectionDefinition'[metadata_def]
          (item_usage derived reference ordered 'allocation' : 'SysML::Systems::AllocationUsage'[metadata_def] :> 'SysML::Systems::Usage::usage'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*])))
        (metadata_def 'AllocationUsage' :> 'SysML::Systems::ConnectionUsage'[metadata_def]
          (item_usage derived reference ordered 'allocationDefinition' : 'SysML::Systems::AllocationDefinition'[metadata_def] :>> 'SysML::Systems::ConnectionUsage::connectionDefinition'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*])))
        (metadata_def 'AnalysisCaseDefinition' :> 'SysML::Systems::CaseDefinition'[metadata_def]
          (item_usage derived reference 'resultExpression' : 'Expression'[unresolved] :> 'expression'[unresolved] :> 'ownedFeature'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..1])))
        (metadata_def 'AnalysisCaseUsage' :> 'SysML::Systems::CaseUsage'[metadata_def]
          (item_usage derived reference 'analysisCaseDefinition' : 'SysML::Systems::AnalysisCaseDefinition'[metadata_def] :>> 'SysML::Systems::CaseUsage::caseDefinition'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..1]))
          (item_usage derived reference 'resultExpression' : 'Expression'[unresolved] :> 'ownedFeature'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..1])))
        (metadata_def 'AssertConstraintUsage' :> 'SysML::Systems::ConstraintUsage'[metadata_def] :> 'Invariant'[unresolved]
          (item_usage derived reference 'assertedConstraint' : 'SysML::Systems::ConstraintUsage'[metadata_def] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [1..1])))
        (metadata_def 'AssignmentActionUsage' :> 'SysML::Systems::ActionUsage'[metadata_def]
          (item_usage derived reference 'targetArgument' : 'Expression'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..1]))
          (item_usage derived reference 'valueExpression' : 'Expression'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..1]))
          (item_usage derived reference 'referent' : 'Feature'[unresolved] :> 'member'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [1..1])))
        (metadata_def 'AttributeDefinition' :> 'DataType'[unresolved] :> 'SysML::Systems::Definition'[metadata_def])
        (metadata_def 'AttributeUsage' :> 'SysML::Systems::Usage'[metadata_def]
          (attribute_usage derived composite 'isReference' : 'Boolean'[unresolved] :>> 'SysML::Systems::Usage::isReference'[attribute_usage]
            (multiplicity_range [1..1]))
          (item_usage derived reference ordered 'attributeDefinition' : 'DataType'[unresolved] :>> 'SysML::Systems::Usage::definition'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*])))
        (metadata_def 'BindingConnectorAsUsage' :> 'BindingConnector'[unresolved] :> 'SysML::Systems::ConnectorAsUsage'[metadata_def])
        (metadata_def 'CalculationDefinition' :> 'Function'[unresolved] :> 'SysML::Systems::ActionDefinition'[metadata_def]
          (item_usage derived reference ordered 'calculation' : 'SysML::Systems::CalculationUsage'[metadata_def] :> 'SysML::Systems::ActionDefinition::action'[item_usage] :> 'expression'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*])))
        (metadata_def 'CalculationUsage' :> 'Expression'[unresolved] :> 'SysML::Systems::ActionUsage'[metadata_def]
          (item_usage derived reference ordered 'calculationDefinition' : 'Function'[unresolved] :>> 'function'[unresolved] :>> 'SysML::Systems::ActionUsage::actionDefinition'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..1])))
        (metadata_def 'CaseDefinition' :> 'SysML::Systems::CalculationDefinition'[metadata_def]
          (item_usage derived reference ordered 'objectiveRequirement' : 'SysML::Systems::RequirementUsage'[metadata_def] :> 'SysML::Systems::Usage::usage'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..1]))
          (item_usage derived reference 'subjectParameter' : 'SysML::Systems::Usage'[metadata_def] :> 'parameter'[unresolved] :> 'SysML::Systems::Usage::usage'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [1..1]))
          (item_usage derived reference ordered 'actorParameter' : 'SysML::Systems::PartUsage'[metadata_def] :> 'parameter'[unresolved] :> 'SysML::Systems::Usage::usage'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*])))
        (metadata_def 'CaseUsage' :> 'SysML::Systems::CalculationUsage'[metadata_def]
          (item_usage derived reference ordered 'objectiveRequirement' : 'SysML::Systems::RequirementUsage'[metadata_def] :> 'SysML::Systems::Usage::usage'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..1]))
          (item_usage derived reference 'caseDefinition' : 'SysML::Systems::CaseDefinition'[metadata_def] :>> 'SysML::Systems::CalculationUsage::calculationDefinition'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..1]))
          (item_usage derived reference 'subjectParameter' : 'SysML::Systems::Usage'[metadata_def] :> 'parameter'[unresolved] :> 'SysML::Systems::Usage::usage'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [1..1]))
          (item_usage derived reference ordered 'actorParameter' : 'SysML::Systems::PartUsage'[metadata_def] :> 'parameter'[unresolved] :> 'SysML::Systems::Usage::usage'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*])))
        (metadata_def 'ConcernDefinition' :> 'SysML::Systems::RequirementDefinition'[metadata_def])
        (metadata_def 'ConcernUsage' :> 'SysML::Systems::RequirementUsage'[metadata_def]
          (item_usage derived reference 'concernDefinition' : 'SysML::Systems::ConcernDefinition'[metadata_def] :>> 'SysML::Systems::RequirementUsage::requirementDefinition'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..1])))
        (metadata_def 'ConjugatedPortDefinition' :> 'SysML::Systems::PortDefinition'[metadata_def]
          (item_usage derived reference 'originalPortDefinition' : 'SysML::Systems::PortDefinition'[metadata_def] :>> 'owningNamespace'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [1..1]))
          (item_usage derived reference 'ownedPortConjugator' : 'SysML::Systems::PortConjugation'[metadata_def] :>> 'ownedConjugator'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [1..1])))
        (metadata_def 'ConjugatedPortTyping' :> 'FeatureTyping'[unresolved]
          (item_usage reference 'conjugatedPortDefinition' : 'SysML::Systems::ConjugatedPortDefinition'[metadata_def] :>> 'type'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [1..1]))
          (item_usage derived reference 'portDefinition' : 'SysML::Systems::PortDefinition'[metadata_def] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [1..1])))
        (metadata_def 'ConnectionDefinition' :> 'AssociationStructure'[unresolved] :> 'SysML::Systems::PartDefinition'[metadata_def]
          (attribute_usage composite 'isSufficient' : 'Boolean'[unresolved] :>> 'isSufficient'[unresolved]
            (multiplicity_range [1..1]))
          (item_usage derived reference ordered 'connectionEnd' : 'SysML::Systems::Usage'[metadata_def] :>> 'associationEnd'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*])))
        (metadata_def 'ConnectionUsage' :> 'SysML::Systems::ConnectorAsUsage'[metadata_def] :> 'SysML::Systems::PartUsage'[metadata_def]
          (item_usage derived reference ordered 'connectionDefinition' : 'AssociationStructure'[unresolved] :> 'SysML::Systems::ItemUsage::itemDefinition'[item_usage] :>> 'association'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*])))
        (metadata_def abstract 'ConnectorAsUsage' :> 'SysML::Systems::Usage'[metadata_def] :> 'Connector'[unresolved])
        (metadata_def 'ConstraintDefinition' :> 'SysML::Systems::OccurrenceDefinition'[metadata_def] :> 'Predicate'[unresolved])
        (metadata_def 'ConstraintUsage' :> 'BooleanExpression'[unresolved] :> 'SysML::Systems::OccurrenceUsage'[metadata_def]
          (item_usage derived reference 'constraintDefinition' : 'Predicate'[unresolved] :>> 'predicate'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..1])))
        (metadata_def abstract 'ControlNode' :> 'SysML::Systems::ActionUsage'[metadata_def])
        (metadata_def 'DecisionNode' :> 'SysML::Systems::ControlNode'[metadata_def])
        (metadata_def 'Definition' :> 'Classifier'[unresolved]
          (attribute_usage composite 'isVariation' : 'Boolean'[unresolved]
            (multiplicity_range [1..1]))
          (item_usage derived reference 'variant' : 'SysML::Systems::Usage'[metadata_def] :> 'ownedMember'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived composite 'variantMembership' : 'SysML::Systems::VariantMembership'[metadata_def] :> 'ownedMembership'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'usage' : 'SysML::Systems::Usage'[metadata_def] :> 'feature'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'directedUsage' : 'SysML::Systems::Usage'[metadata_def] :> 'directedFeature'[unresolved] :> 'SysML::Systems::Usage::usage'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'ownedUsage' : 'SysML::Systems::Usage'[metadata_def] :> 'ownedFeature'[unresolved] :> 'SysML::Systems::Usage::usage'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'ownedReference' : 'SysML::Systems::ReferenceUsage'[metadata_def] :> 'SysML::Systems::Definition::ownedUsage'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'ownedAttribute' : 'SysML::Systems::AttributeUsage'[metadata_def] :> 'SysML::Systems::Definition::ownedUsage'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'ownedEnumeration' : 'SysML::Systems::EnumerationUsage'[metadata_def] :> 'SysML::Systems::Definition::ownedAttribute'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'ownedOccurrence' : 'SysML::Systems::OccurrenceUsage'[metadata_def] :> 'SysML::Systems::Definition::ownedUsage'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'ownedItem' : 'SysML::Systems::ItemUsage'[metadata_def] :> 'SysML::Systems::Definition::ownedOccurrence'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'ownedPart' : 'SysML::Systems::PartUsage'[metadata_def] :> 'SysML::Systems::Definition::ownedItem'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'ownedPort' : 'SysML::Systems::PortUsage'[metadata_def] :> 'SysML::Systems::Definition::ownedUsage'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'ownedConnection' : 'SysML::Systems::ConnectorAsUsage'[metadata_def] :> 'SysML::Systems::Definition::ownedUsage'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference 'ownedFlow' : 'SysML::Systems::FlowUsage'[metadata_def] :> 'SysML::Systems::Definition::ownedConnection'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'ownedInterface' : 'SysML::Systems::InterfaceUsage'[metadata_def] :> 'SysML::Systems::Definition::ownedConnection'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'ownedAllocation' : 'SysML::Systems::AllocationUsage'[metadata_def] :> 'SysML::Systems::Definition::ownedConnection'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'ownedAction' : 'SysML::Systems::ActionUsage'[metadata_def] :> 'SysML::Systems::Definition::ownedOccurrence'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'ownedState' : 'SysML::Systems::StateUsage'[metadata_def] :> 'SysML::Systems::Definition::ownedAction'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference 'ownedTransition' : 'SysML::Systems::TransitionUsage'[metadata_def] :> 'SysML::Systems::Definition::ownedUsage'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'ownedCalculation' : 'SysML::Systems::CalculationUsage'[metadata_def] :> 'SysML::Systems::Definition::ownedAction'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'ownedConstraint' : 'SysML::Systems::ConstraintUsage'[metadata_def] :> 'SysML::Systems::Definition::ownedOccurrence'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'ownedRequirement' : 'SysML::Systems::RequirementUsage'[metadata_def] :> 'SysML::Systems::Definition::ownedConstraint'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference 'ownedConcern' : 'SysML::Systems::ConcernUsage'[metadata_def] :> 'SysML::Systems::Definition::ownedRequirement'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'ownedCase' : 'SysML::Systems::CaseUsage'[metadata_def] :> 'SysML::Systems::Definition::ownedCalculation'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'ownedAnalysisCase' : 'SysML::Systems::AnalysisCaseUsage'[metadata_def] :> 'SysML::Systems::Definition::ownedCase'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'ownedVerificationCase' : 'SysML::Systems::VerificationCaseUsage'[metadata_def] :> 'SysML::Systems::Definition::ownedCase'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'ownedUseCase' : 'SysML::Systems::UseCaseUsage'[metadata_def] :> 'SysML::Systems::Definition::ownedCase'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'ownedView' : 'SysML::Systems::ViewUsage'[metadata_def] :> 'SysML::Systems::Definition::ownedPart'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'ownedViewpoint' : 'SysML::Systems::ViewpointUsage'[metadata_def] :> 'SysML::Systems::Definition::ownedRequirement'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'ownedRendering' : 'SysML::Systems::RenderingUsage'[metadata_def] :> 'SysML::Systems::Definition::ownedPart'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'ownedMetadata' : 'SysML::Systems::MetadataUsage'[metadata_def] :> 'SysML::Systems::Definition::ownedItem'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*])))
        (metadata_def 'EnumerationDefinition' :> 'SysML::Systems::AttributeDefinition'[metadata_def]
          (attribute_usage composite 'isVariation' : 'Boolean'[unresolved] :>> 'SysML::Systems::Definition::isVariation'[attribute_usage]
            (multiplicity_range [1..1]))
          (item_usage derived reference ordered 'enumeratedValue' : 'SysML::Systems::EnumerationUsage'[metadata_def] :>> 'SysML::Systems::Definition::variant'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*])))
        (metadata_def 'EnumerationUsage' :> 'SysML::Systems::AttributeUsage'[metadata_def]
          (item_usage derived reference 'enumerationDefinition' : 'SysML::Systems::EnumerationDefinition'[metadata_def] :>> 'SysML::Systems::AttributeUsage::attributeDefinition'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [1..1])))
        (metadata_def 'EventOccurrenceUsage' :> 'SysML::Systems::OccurrenceUsage'[metadata_def]
          (attribute_usage derived composite 'isReference' : 'Boolean'[unresolved] :>> 'SysML::Systems::Usage::isReference'[attribute_usage]
            (multiplicity_range [1..1]))
          (item_usage derived reference 'eventOccurrence' : 'SysML::Systems::OccurrenceUsage'[metadata_def] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [1..1])))
        (metadata_def 'ExhibitStateUsage' :> 'SysML::Systems::StateUsage'[metadata_def] :> 'SysML::Systems::PerformActionUsage'[metadata_def]
          (item_usage derived reference 'exhibitedState' : 'SysML::Systems::StateUsage'[metadata_def] :>> 'SysML::Systems::PerformActionUsage::performedAction'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [1..1])))
        (metadata_def abstract 'Expose' :> 'Import'[unresolved]
          (attribute_usage composite 'visibility' : 'VisibilityKind'[unresolved] :>> 'visibility'[unresolved]
            (multiplicity_range [1..1]))
          (attribute_usage composite 'isImportAll' : 'Boolean'[unresolved] :>> 'isImportAll'[unresolved]
            (multiplicity_range [1..1])))
        (metadata_def 'FlowDefinition' :> 'Interaction'[unresolved] :> 'SysML::Systems::ActionDefinition'[metadata_def]
          (item_usage derived reference 'flowEnd' : 'SysML::Systems::Usage'[metadata_def] :>> 'associationEnd'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*])))
        (metadata_def 'FlowUsage' :> 'SysML::Systems::ConnectorAsUsage'[metadata_def] :> 'Flow'[unresolved] :> 'SysML::Systems::ActionUsage'[metadata_def]
          (item_usage derived reference ordered 'flowDefinition' : 'Interaction'[unresolved] :>> 'SysML::Systems::ActionUsage::actionDefinition'[item_usage] :>> 'interaction'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*])))
        (metadata_def 'ForLoopActionUsage' :> 'SysML::Systems::LoopActionUsage'[metadata_def]
          (item_usage derived reference 'seqArgument' : 'Expression'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [1..1]))
          (item_usage derived reference 'loopVariable' : 'SysML::Systems::ReferenceUsage'[metadata_def] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [1..1])))
        (metadata_def 'ForkNode' :> 'SysML::Systems::ControlNode'[metadata_def])
        (metadata_def 'FramedConcernMembership' :> 'SysML::Systems::RequirementConstraintMembership'[metadata_def]
          (attribute_usage composite 'kind' : 'SysML::Systems::RequirementConstraintKind'[enum_def] :>> 'SysML::Systems::RequirementConstraintMembership::kind'[attribute_usage]
            (multiplicity_range [1..1]))
          (item_usage derived composite 'ownedConcern' : 'SysML::Systems::ConcernUsage'[metadata_def] :>> 'SysML::Systems::RequirementConstraintMembership::ownedConstraint'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [1..1]))
          (item_usage derived reference 'referencedConcern' : 'SysML::Systems::ConcernUsage'[metadata_def] :>> 'SysML::Systems::RequirementConstraintMembership::referencedConstraint'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [1..1])))
        (metadata_def 'IfActionUsage' :> 'SysML::Systems::ActionUsage'[metadata_def]
          (item_usage derived reference 'elseAction' : 'SysML::Systems::ActionUsage'[metadata_def] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..1]))
          (item_usage derived reference 'thenAction' : 'SysML::Systems::ActionUsage'[metadata_def] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [1..1]))
          (item_usage derived reference 'ifArgument' : 'Expression'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [1..1])))
        (metadata_def 'IncludeUseCaseUsage' :> 'SysML::Systems::UseCaseUsage'[metadata_def] :> 'SysML::Systems::PerformActionUsage'[metadata_def]
          (item_usage derived reference 'useCaseIncluded' : 'SysML::Systems::UseCaseUsage'[metadata_def] :>> 'SysML::Systems::PerformActionUsage::performedAction'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [1..1])))
        (metadata_def 'InterfaceDefinition' :> 'SysML::Systems::ConnectionDefinition'[metadata_def]
          (item_usage derived reference ordered 'interfaceEnd' : 'SysML::Systems::PortUsage'[metadata_def] :>> 'SysML::Systems::ConnectionDefinition::connectionEnd'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*])))
        (metadata_def 'InterfaceUsage' :> 'SysML::Systems::ConnectionUsage'[metadata_def]
          (item_usage derived reference 'interfaceDefinition' : 'SysML::Systems::InterfaceDefinition'[metadata_def] :>> 'SysML::Systems::ConnectionUsage::connectionDefinition'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*])))
        (metadata_def 'ItemDefinition' :> 'Structure'[unresolved] :> 'SysML::Systems::OccurrenceDefinition'[metadata_def])
        (metadata_def 'ItemUsage' :> 'SysML::Systems::OccurrenceUsage'[metadata_def]
          (item_usage derived reference ordered 'itemDefinition' : 'Structure'[unresolved] :> 'SysML::Systems::OccurrenceUsage::occurrenceDefinition'[item_usage] :> 'Metadata::metadataItems'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*])))
        (metadata_def 'JoinNode' :> 'SysML::Systems::ControlNode'[metadata_def])
        (metadata_def abstract 'LoopActionUsage' :> 'SysML::Systems::ActionUsage'[metadata_def]
          (item_usage derived reference 'bodyAction' : 'SysML::Systems::ActionUsage'[metadata_def] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [1..1])))
        (metadata_def 'MembershipExpose' :> 'MembershipImport'[unresolved] :> 'SysML::Systems::Expose'[metadata_def])
        (metadata_def 'MergeNode' :> 'SysML::Systems::ControlNode'[metadata_def])
        (metadata_def 'MetadataDefinition' :> 'SysML::Systems::ItemDefinition'[metadata_def] :> 'Metaclass'[unresolved])
        (metadata_def 'MetadataUsage' :> 'SysML::Systems::ItemUsage'[metadata_def] :> 'MetadataFeature'[unresolved]
          (item_usage derived reference 'metadataDefinition' : 'Metaclass'[unresolved] :>> 'SysML::Systems::ItemUsage::itemDefinition'[item_usage] :>> 'metaclass'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..1])))
        (metadata_def 'NamespaceExpose' :> 'SysML::Systems::Expose'[metadata_def] :> 'NamespaceImport'[unresolved])
        (metadata_def 'ObjectiveMembership' :> 'FeatureMembership'[unresolved]
          (item_usage derived composite 'ownedObjectiveRequirement' : 'SysML::Systems::RequirementUsage'[metadata_def] :>> 'ownedMemberFeature'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [1..1])))
        (metadata_def 'OccurrenceDefinition' :> 'SysML::Systems::Definition'[metadata_def] :> 'Class'[unresolved]
          (attribute_usage composite 'isIndividual' : 'Boolean'[unresolved]
            (multiplicity_range [1..1])))
        (metadata_def 'OccurrenceUsage' :> 'SysML::Systems::Usage'[metadata_def]
          (attribute_usage composite 'isIndividual' : 'Boolean'[unresolved]
            (multiplicity_range [1..1]))
          (attribute_usage composite 'portionKind' : 'SysML::Systems::PortionKind'[enum_def]
            (multiplicity_range [0..1]))
          (item_usage derived reference ordered 'occurrenceDefinition' : 'Class'[unresolved] :>> 'SysML::Systems::Usage::definition'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference 'individualDefinition' : 'SysML::Systems::OccurrenceDefinition'[metadata_def] :> 'SysML::Systems::OccurrenceUsage::occurrenceDefinition'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..1])))
        (metadata_def 'PartDefinition' :> 'SysML::Systems::ItemDefinition'[metadata_def])
        (metadata_def 'PartUsage' :> 'SysML::Systems::ItemUsage'[metadata_def]
          (item_usage derived reference ordered 'partDefinition' : 'SysML::Systems::PartDefinition'[metadata_def] :> 'SysML::Systems::ItemUsage::itemDefinition'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*])))
        (metadata_def 'PerformActionUsage' :> 'SysML::Systems::ActionUsage'[metadata_def] :> 'SysML::Systems::EventOccurrenceUsage'[metadata_def]
          (item_usage derived reference 'performedAction' : 'SysML::Systems::ActionUsage'[metadata_def] :>> 'SysML::Systems::EventOccurrenceUsage::eventOccurrence'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [1..1])))
        (metadata_def 'PortConjugation' :> 'Conjugation'[unresolved]
          (item_usage reference 'originalPortDefinition' : 'SysML::Systems::PortDefinition'[metadata_def] :>> 'originalType'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [1..1]))
          (item_usage derived reference 'conjugatedPortDefinition' : 'SysML::Systems::ConjugatedPortDefinition'[metadata_def] :>> 'owningType'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [1..1])))
        (metadata_def 'PortDefinition' :> 'SysML::Systems::OccurrenceDefinition'[metadata_def] :> 'Structure'[unresolved]
          (item_usage derived reference 'conjugatedPortDefinition' : 'SysML::Systems::ConjugatedPortDefinition'[metadata_def] :> 'ownedMember'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..1])))
        (metadata_def 'PortUsage' :> 'SysML::Systems::OccurrenceUsage'[metadata_def]
          (item_usage derived reference ordered 'portDefinition' : 'SysML::Systems::PortDefinition'[metadata_def] :>> 'SysML::Systems::OccurrenceUsage::occurrenceDefinition'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*])))
        (enum_def 'PortionKind'
          (enum_usage composite 'timeslice')
          (enum_usage composite 'snapshot'))
        (metadata_def 'ReferenceUsage' :> 'SysML::Systems::Usage'[metadata_def]
          (attribute_usage derived composite 'isReference' : 'Boolean'[unresolved] :>> 'SysML::Systems::Usage::isReference'[attribute_usage]
            (multiplicity_range [1..1])))
        (metadata_def 'RenderingDefinition' :> 'SysML::Systems::PartDefinition'[metadata_def]
          (item_usage derived reference ordered 'rendering' : 'SysML::Systems::RenderingUsage'[metadata_def] :> 'SysML::Systems::Usage::usage'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*])))
        (metadata_def 'RenderingUsage' :> 'SysML::Systems::PartUsage'[metadata_def]
          (item_usage derived reference 'renderingDefinition' : 'SysML::Systems::RenderingDefinition'[metadata_def] :>> 'SysML::Systems::PartUsage::partDefinition'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..1])))
        (enum_def 'RequirementConstraintKind'
          (enum_usage composite 'assumption')
          (enum_usage composite 'requirement'))
        (metadata_def 'RequirementConstraintMembership' :> 'FeatureMembership'[unresolved]
          (attribute_usage composite 'kind' : 'SysML::Systems::RequirementConstraintKind'[enum_def]
            (multiplicity_range [1..1]))
          (item_usage derived composite 'ownedConstraint' : 'SysML::Systems::ConstraintUsage'[metadata_def] :>> 'ownedMemberFeature'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [1..1]))
          (item_usage derived reference 'referencedConstraint' : 'SysML::Systems::ConstraintUsage'[metadata_def] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [1..1])))
        (metadata_def 'RequirementDefinition' :> 'SysML::Systems::ConstraintDefinition'[metadata_def]
          (attribute_usage composite 'reqId' : 'String'[unresolved] :>> 'declaredShortName'[unresolved]
            (multiplicity_range [0..1]))
          (attribute_usage derived composite 'text' : 'String'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference 'subjectParameter' : 'SysML::Systems::Usage'[metadata_def] :> 'parameter'[unresolved] :> 'SysML::Systems::Usage::usage'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [1..1]))
          (item_usage derived reference ordered 'actorParameter' : 'SysML::Systems::PartUsage'[metadata_def] :> 'parameter'[unresolved] :> 'SysML::Systems::Usage::usage'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'stakeholderParameter' : 'SysML::Systems::PartUsage'[metadata_def] :> 'parameter'[unresolved] :> 'SysML::Systems::Usage::usage'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'assumedConstraint' : 'SysML::Systems::ConstraintUsage'[metadata_def] :> 'ownedFeature'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'requiredConstraint' : 'SysML::Systems::ConstraintUsage'[metadata_def] :> 'ownedFeature'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'framedConcern' : 'SysML::Systems::ConcernUsage'[metadata_def] :> 'SysML::Systems::RequirementUsage::requiredConstraint'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*])))
        (metadata_def 'RequirementUsage' :> 'SysML::Systems::ConstraintUsage'[metadata_def]
          (attribute_usage composite 'reqId' : 'String'[unresolved] :>> 'declaredShortName'[unresolved]
            (multiplicity_range [0..1]))
          (attribute_usage derived composite 'text' : 'String'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference 'requirementDefinition' : 'SysML::Systems::RequirementDefinition'[metadata_def] :>> 'SysML::Systems::ConstraintUsage::constraintDefinition'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..1]))
          (item_usage derived reference ordered 'requiredConstraint' : 'SysML::Systems::ConstraintUsage'[metadata_def] :> 'ownedFeature'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'assumedConstraint' : 'SysML::Systems::ConstraintUsage'[metadata_def] :> 'ownedFeature'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference 'subjectParameter' : 'SysML::Systems::Usage'[metadata_def] :> 'parameter'[unresolved] :> 'SysML::Systems::Usage::usage'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [1..1]))
          (item_usage derived reference ordered 'framedConcern' : 'SysML::Systems::ConcernUsage'[metadata_def] :> 'SysML::Systems::RequirementUsage::requiredConstraint'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'actorParameter' : 'SysML::Systems::PartUsage'[metadata_def] :> 'parameter'[unresolved] :> 'SysML::Systems::Usage::usage'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'stakeholderParameter' : 'SysML::Systems::PartUsage'[metadata_def] :> 'parameter'[unresolved] :> 'SysML::Systems::Usage::usage'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*])))
        (metadata_def 'RequirementVerificationMembership' :> 'SysML::Systems::RequirementConstraintMembership'[metadata_def]
          (attribute_usage composite 'kind' : 'SysML::Systems::RequirementConstraintKind'[enum_def] :>> 'SysML::Systems::RequirementConstraintMembership::kind'[attribute_usage]
            (multiplicity_range [1..1]))
          (item_usage derived composite 'ownedRequirement' : 'SysML::Systems::RequirementUsage'[metadata_def] :>> 'SysML::Systems::RequirementConstraintMembership::ownedConstraint'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [1..1]))
          (item_usage derived reference 'verifiedRequirement' : 'SysML::Systems::RequirementUsage'[metadata_def] :>> 'SysML::Systems::RequirementConstraintMembership::referencedConstraint'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [1..1])))
        (metadata_def 'SatisfyRequirementUsage' :> 'SysML::Systems::RequirementUsage'[metadata_def] :> 'SysML::Systems::AssertConstraintUsage'[metadata_def]
          (item_usage derived reference 'satisfiedRequirement' : 'SysML::Systems::RequirementUsage'[metadata_def] :>> 'SysML::Systems::AssertConstraintUsage::assertedConstraint'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [1..1]))
          (item_usage derived reference 'satisfyingFeature' : 'Feature'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [1..1])))
        (metadata_def 'SendActionUsage' :> 'SysML::Systems::ActionUsage'[metadata_def]
          (item_usage derived reference 'receiverArgument' : 'Expression'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..1]))
          (item_usage derived reference 'payloadArgument' : 'Expression'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [1..1]))
          (item_usage derived reference 'senderArgument' : 'Expression'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..1])))
        (metadata_def 'StakeholderMembership' :> 'ParameterMembership'[unresolved]
          (item_usage derived composite 'ownedStakeholderParameter' : 'SysML::Systems::PartUsage'[metadata_def] :>> 'ownedMemberParameter'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [1..1])))
        (metadata_def 'StateDefinition' :> 'SysML::Systems::ActionDefinition'[metadata_def]
          (attribute_usage composite 'isParallel' : 'Boolean'[unresolved]
            (multiplicity_range [1..1]))
          (item_usage derived reference ordered 'state' : 'SysML::Systems::StateUsage'[metadata_def] :> 'SysML::Systems::ActionDefinition::action'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference 'entryAction' : 'SysML::Systems::ActionUsage'[metadata_def] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..1]))
          (item_usage derived reference 'doAction' : 'SysML::Systems::ActionUsage'[metadata_def] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..1]))
          (item_usage derived reference 'exitAction' : 'SysML::Systems::ActionUsage'[metadata_def] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..1])))
        (enum_def 'StateSubactionKind'
          (enum_usage composite 'entry')
          (enum_usage composite 'do')
          (enum_usage composite 'exit'))
        (metadata_def 'StateSubactionMembership' :> 'FeatureMembership'[unresolved]
          (attribute_usage composite 'kind' : 'SysML::Systems::StateSubactionKind'[enum_def]
            (multiplicity_range [1..1]))
          (item_usage derived composite 'action' : 'SysML::Systems::ActionUsage'[metadata_def] :>> 'ownedMemberFeature'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [1..1])))
        (metadata_def 'StateUsage' :> 'SysML::Systems::ActionUsage'[metadata_def]
          (attribute_usage composite 'isParallel' : 'Boolean'[unresolved]
            (multiplicity_range [1..1]))
          (item_usage derived reference ordered 'stateDefinition' : 'Behavior'[unresolved] :>> 'SysML::Systems::ActionUsage::actionDefinition'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference 'entryAction' : 'SysML::Systems::ActionUsage'[metadata_def] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..1]))
          (item_usage derived reference 'doAction' : 'SysML::Systems::ActionUsage'[metadata_def] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..1]))
          (item_usage derived reference 'exitAction' : 'SysML::Systems::ActionUsage'[metadata_def] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..1])))
        (metadata_def 'SubjectMembership' :> 'ParameterMembership'[unresolved]
          (item_usage derived composite 'ownedSubjectParameter' : 'SysML::Systems::Usage'[metadata_def] :>> 'ownedMemberParameter'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [1..1])))
        (metadata_def 'SuccessionAsUsage' :> 'SysML::Systems::ConnectorAsUsage'[metadata_def] :> 'Succession'[unresolved])
        (metadata_def 'SuccessionFlowUsage' :> 'SuccessionFlow'[unresolved] :> 'SysML::Systems::FlowUsage'[metadata_def])
        (metadata_def 'TerminateActionUsage' :> 'SysML::Systems::ActionUsage'[metadata_def]
          (item_usage derived reference 'terminatedOccurrenceArgument' : 'Expression'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..1])))
        (enum_def 'TransitionFeatureKind'
          (enum_usage composite 'trigger')
          (enum_usage composite 'guard')
          (enum_usage composite 'effect'))
        (metadata_def 'TransitionFeatureMembership' :> 'FeatureMembership'[unresolved]
          (attribute_usage composite 'kind' : 'SysML::Systems::TransitionFeatureKind'[enum_def]
            (multiplicity_range [1..1]))
          (item_usage derived composite 'transitionFeature' : 'Step'[unresolved] :>> 'ownedMemberFeature'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [1..1])))
        (metadata_def 'TransitionUsage' :> 'SysML::Systems::ActionUsage'[metadata_def]
          (item_usage derived reference 'source' : 'SysML::Systems::ActionUsage'[metadata_def] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [1..1]))
          (item_usage derived reference 'target' : 'SysML::Systems::ActionUsage'[metadata_def] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [1..1]))
          (item_usage derived reference 'triggerAction' : 'SysML::Systems::AcceptActionUsage'[metadata_def] :> 'ownedFeature'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference 'guardExpression' : 'Expression'[unresolved] :> 'ownedFeature'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference 'effectAction' : 'SysML::Systems::ActionUsage'[metadata_def] :> 'feature'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference 'succession' : 'Succession'[unresolved] :> 'ownedMember'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [1..1])))
        (metadata_def 'TriggerInvocationExpression' :> 'InvocationExpression'[unresolved]
          (attribute_usage composite 'kind' : 'SysML::Systems::TriggerKind'[enum_def]
            (multiplicity_range [1..1])))
        (enum_def 'TriggerKind'
          (enum_usage composite 'when')
          (enum_usage composite 'at')
          (enum_usage composite 'after'))
        (metadata_def 'Usage' :> 'Feature'[unresolved]
          (attribute_usage composite 'isVariation' : 'Boolean'[unresolved]
            (multiplicity_range [1..1]))
          (attribute_usage derived composite 'mayTimeVary' : 'Boolean'[unresolved] :>> 'isVariable'[unresolved]
            (multiplicity_range [1..1]))
          (attribute_usage derived composite 'isReference' : 'Boolean'[unresolved]
            (multiplicity_range [1..1]))
          (item_usage derived reference 'variant' : 'SysML::Systems::Usage'[metadata_def] :> 'ownedMember'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived composite 'variantMembership' : 'SysML::Systems::VariantMembership'[metadata_def] :> 'ownedMembership'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference 'owningDefinition' : 'SysML::Systems::Definition'[metadata_def] :> 'owningType'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..1]))
          (item_usage derived reference 'owningUsage' : 'SysML::Systems::Usage'[metadata_def] :> 'owningType'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..1]))
          (item_usage derived reference ordered 'definition' : 'Classifier'[unresolved] :>> 'type'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'usage' : 'SysML::Systems::Usage'[metadata_def] :> 'feature'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'directedUsage' : 'SysML::Systems::Usage'[metadata_def] :> 'directedFeature'[unresolved] :> 'SysML::Systems::Usage::usage'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'nestedUsage' : 'SysML::Systems::Usage'[metadata_def] :> 'ownedFeature'[unresolved] :> 'SysML::Systems::Usage::usage'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'nestedReference' : 'SysML::Systems::ReferenceUsage'[metadata_def] :> 'SysML::Systems::Usage::nestedUsage'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'nestedAttribute' : 'SysML::Systems::AttributeUsage'[metadata_def] :> 'SysML::Systems::Usage::nestedUsage'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'nestedEnumeration' : 'SysML::Systems::EnumerationUsage'[metadata_def] :> 'SysML::Systems::Usage::nestedAttribute'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'nestedOccurrence' : 'SysML::Systems::OccurrenceUsage'[metadata_def] :> 'SysML::Systems::Usage::nestedUsage'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'nestedItem' : 'SysML::Systems::ItemUsage'[metadata_def] :> 'SysML::Systems::Usage::nestedOccurrence'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'nestedPart' : 'SysML::Systems::PartUsage'[metadata_def] :> 'SysML::Systems::Usage::nestedItem'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'nestedPort' : 'SysML::Systems::PortUsage'[metadata_def] :> 'SysML::Systems::Usage::nestedUsage'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'nestedConnection' : 'SysML::Systems::ConnectorAsUsage'[metadata_def] :> 'SysML::Systems::Usage::nestedUsage'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference 'nestedFlow' : 'SysML::Systems::FlowUsage'[metadata_def] :> 'SysML::Systems::Usage::nestedConnection'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'nestedInterface' : 'SysML::Systems::InterfaceUsage'[metadata_def] :> 'SysML::Systems::Usage::nestedConnection'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'nestedAllocation' : 'SysML::Systems::AllocationUsage'[metadata_def] :> 'SysML::Systems::Usage::nestedConnection'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'nestedAction' : 'SysML::Systems::ActionUsage'[metadata_def] :> 'SysML::Systems::Usage::nestedOccurrence'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'nestedState' : 'SysML::Systems::StateUsage'[metadata_def] :> 'SysML::Systems::Usage::nestedAction'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference 'nestedTransition' : 'SysML::Systems::TransitionUsage'[metadata_def] :> 'SysML::Systems::Usage::nestedUsage'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'nestedCalculation' : 'SysML::Systems::CalculationUsage'[metadata_def] :> 'SysML::Systems::Usage::nestedAction'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'nestedConstraint' : 'SysML::Systems::ConstraintUsage'[metadata_def] :> 'SysML::Systems::Usage::nestedOccurrence'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'nestedRequirement' : 'SysML::Systems::RequirementUsage'[metadata_def] :> 'SysML::Systems::Usage::nestedConstraint'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference 'nestedConcern' : 'SysML::Systems::ConcernUsage'[metadata_def] :> 'SysML::Systems::Usage::nestedRequirement'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'nestedCase' : 'SysML::Systems::CaseUsage'[metadata_def] :> 'SysML::Systems::Usage::nestedCalculation'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'nestedAnalysisCase' : 'SysML::Systems::AnalysisCaseUsage'[metadata_def] :> 'SysML::Systems::Usage::nestedCase'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'nestedVerificationCase' : 'SysML::Systems::VerificationCaseUsage'[metadata_def] :> 'SysML::Systems::Usage::nestedCase'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'nestedUseCase' : 'SysML::Systems::UseCaseUsage'[metadata_def] :> 'SysML::Systems::Usage::nestedCase'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'nestedView' : 'SysML::Systems::ViewUsage'[metadata_def] :> 'SysML::Systems::Usage::nestedPart'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'nestedViewpoint' : 'SysML::Systems::ViewpointUsage'[metadata_def] :> 'SysML::Systems::Usage::nestedRequirement'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'nestedRendering' : 'SysML::Systems::RenderingUsage'[metadata_def] :> 'SysML::Systems::Usage::nestedPart'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'nestedMetadata' : 'SysML::Systems::MetadataUsage'[metadata_def] :> 'SysML::Systems::Usage::nestedItem'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*])))
        (metadata_def 'UseCaseDefinition' :> 'SysML::Systems::CaseDefinition'[metadata_def]
          (item_usage derived reference ordered 'includedUseCase' : 'SysML::Systems::UseCaseUsage'[metadata_def] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*])))
        (metadata_def 'UseCaseUsage' :> 'SysML::Systems::CaseUsage'[metadata_def]
          (item_usage derived reference 'useCaseDefinition' : 'SysML::Systems::UseCaseDefinition'[metadata_def] :>> 'SysML::Systems::CaseUsage::caseDefinition'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..1]))
          (item_usage derived reference ordered 'includedUseCase' : 'SysML::Systems::UseCaseUsage'[metadata_def] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*])))
        (metadata_def 'VariantMembership' :> 'OwningMembership'[unresolved]
          (item_usage derived composite 'ownedVariantUsage' : 'SysML::Systems::Usage'[metadata_def] :>> 'ownedMemberElement'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [1..1])))
        (metadata_def 'VerificationCaseDefinition' :> 'SysML::Systems::CaseDefinition'[metadata_def]
          (item_usage derived reference ordered 'verifiedRequirement' : 'SysML::Systems::RequirementUsage'[metadata_def] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*])))
        (metadata_def 'VerificationCaseUsage' :> 'SysML::Systems::CaseUsage'[metadata_def]
          (item_usage derived reference 'verificationCaseDefinition' : 'SysML::Systems::VerificationCaseDefinition'[metadata_def] :> 'SysML::Systems::CaseUsage::caseDefinition'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..1]))
          (item_usage derived reference ordered 'verifiedRequirement' : 'SysML::Systems::RequirementUsage'[metadata_def] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*])))
        (metadata_def 'ViewDefinition' :> 'SysML::Systems::PartDefinition'[metadata_def]
          (item_usage derived reference ordered 'view' : 'SysML::Systems::ViewUsage'[metadata_def] :> 'SysML::Systems::Usage::usage'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'satisfiedViewpoint' : 'SysML::Systems::ViewpointUsage'[metadata_def] :> 'SysML::Systems::Definition::ownedRequirement'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference 'viewRendering' : 'SysML::Systems::RenderingUsage'[metadata_def] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..1]))
          (item_usage derived reference ordered 'viewCondition' : 'Expression'[unresolved] :> 'ownedMember'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*])))
        (metadata_def 'ViewRenderingMembership' :> 'FeatureMembership'[unresolved]
          (item_usage derived composite 'ownedRendering' : 'SysML::Systems::RenderingUsage'[metadata_def] :>> 'ownedMemberFeature'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [1..1]))
          (item_usage derived reference 'referencedRendering' : 'SysML::Systems::RenderingUsage'[metadata_def] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [1..1])))
        (metadata_def 'ViewUsage' :> 'SysML::Systems::PartUsage'[metadata_def]
          (item_usage derived reference 'viewDefinition' : 'SysML::Systems::ViewDefinition'[metadata_def] :>> 'SysML::Systems::PartUsage::partDefinition'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..1]))
          (item_usage derived reference ordered 'satisfiedViewpoint' : 'SysML::Systems::ViewpointUsage'[metadata_def] :> 'SysML::Systems::Usage::nestedRequirement'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference ordered 'exposedElement' : 'Element'[unresolved] :> 'member'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*]))
          (item_usage derived reference 'viewRendering' : 'SysML::Systems::RenderingUsage'[metadata_def] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..1]))
          (item_usage derived reference ordered 'viewCondition' : 'Expression'[unresolved] :> 'ownedMember'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*])))
        (metadata_def 'ViewpointDefinition' :> 'SysML::Systems::RequirementDefinition'[metadata_def]
          (item_usage derived reference ordered 'viewpointStakeholder' : 'SysML::Systems::PartUsage'[metadata_def] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*])))
        (metadata_def 'ViewpointUsage' :> 'SysML::Systems::RequirementUsage'[metadata_def]
          (item_usage derived reference 'viewpointDefinition' : 'SysML::Systems::ViewpointDefinition'[metadata_def] :>> 'SysML::Systems::RequirementUsage::requirementDefinition'[item_usage] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..1]))
          (item_usage derived reference ordered 'viewpointStakeholder' : 'SysML::Systems::PartUsage'[metadata_def] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..*])))
        (metadata_def 'WhileLoopActionUsage' :> 'SysML::Systems::LoopActionUsage'[metadata_def]
          (item_usage derived reference 'whileArgument' : 'Expression'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [1..1]))
          (item_usage derived reference 'untilArgument' : 'Expression'[unresolved] :> 'Metadata::metadataItems'[unresolved]
            (multiplicity_range [0..1])))))))
~~~
