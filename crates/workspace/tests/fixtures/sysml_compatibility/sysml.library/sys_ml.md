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
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "SysML"))) (name "SysML") (declared-name "SysML")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "SysML::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "SysML::*#import"))) (name "*") (declared-name "*"))
        (element (kind "package") (id (node (document "d0") (qualified-name "SysML::Systems"))) (name "Systems") (declared-name "Systems")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "SysML::Systems::*"))) (name "*") (declared-name "*"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::AcceptActionUsage"))) (name "AcceptActionUsage") (declared-name "AcceptActionUsage"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::ActionDefinition"))) (name "ActionDefinition") (declared-name "ActionDefinition"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::ActionUsage"))) (name "ActionUsage") (declared-name "ActionUsage"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::ActorMembership"))) (name "ActorMembership") (declared-name "ActorMembership"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::AllocationDefinition"))) (name "AllocationDefinition") (declared-name "AllocationDefinition"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::AllocationUsage"))) (name "AllocationUsage") (declared-name "AllocationUsage"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::AnalysisCaseDefinition"))) (name "AnalysisCaseDefinition") (declared-name "AnalysisCaseDefinition"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::AnalysisCaseUsage"))) (name "AnalysisCaseUsage") (declared-name "AnalysisCaseUsage"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::AssertConstraintUsage"))) (name "AssertConstraintUsage") (declared-name "AssertConstraintUsage"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::AssignmentActionUsage"))) (name "AssignmentActionUsage") (declared-name "AssignmentActionUsage"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::AttributeDefinition"))) (name "AttributeDefinition") (declared-name "AttributeDefinition"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::AttributeUsage"))) (name "AttributeUsage") (declared-name "AttributeUsage")
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "SysML::Systems::AttributeUsage::isReference"))) (name "isReference") (declared-name "isReference") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SysML::Systems::AttributeUsage")))))
              )
            )
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::BindingConnectorAsUsage"))) (name "BindingConnectorAsUsage") (declared-name "BindingConnectorAsUsage"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::CalculationDefinition"))) (name "CalculationDefinition") (declared-name "CalculationDefinition"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::CalculationUsage"))) (name "CalculationUsage") (declared-name "CalculationUsage"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::CaseDefinition"))) (name "CaseDefinition") (declared-name "CaseDefinition"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::CaseUsage"))) (name "CaseUsage") (declared-name "CaseUsage"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::ConcernDefinition"))) (name "ConcernDefinition") (declared-name "ConcernDefinition"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::ConcernUsage"))) (name "ConcernUsage") (declared-name "ConcernUsage"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::ConjugatedPortDefinition"))) (name "ConjugatedPortDefinition") (declared-name "ConjugatedPortDefinition"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::ConjugatedPortTyping"))) (name "ConjugatedPortTyping") (declared-name "ConjugatedPortTyping"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::ConnectionDefinition"))) (name "ConnectionDefinition") (declared-name "ConnectionDefinition")
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "SysML::Systems::ConnectionDefinition::isSufficient"))) (name "isSufficient") (declared-name "isSufficient") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SysML::Systems::ConnectionDefinition")))))
              )
            )
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::ConnectionUsage"))) (name "ConnectionUsage") (declared-name "ConnectionUsage"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::ConnectorAsUsage"))) (name "ConnectorAsUsage") (declared-name "ConnectorAsUsage"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::ConstraintDefinition"))) (name "ConstraintDefinition") (declared-name "ConstraintDefinition"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::ConstraintUsage"))) (name "ConstraintUsage") (declared-name "ConstraintUsage"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::ControlNode"))) (name "ControlNode") (declared-name "ControlNode"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::DecisionNode"))) (name "DecisionNode") (declared-name "DecisionNode"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::Definition"))) (name "Definition") (declared-name "Definition")
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "SysML::Systems::Definition::isVariation"))) (name "isVariation") (declared-name "isVariation") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SysML::Systems::Definition")))))
              )
            )
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::EnumerationDefinition"))) (name "EnumerationDefinition") (declared-name "EnumerationDefinition")
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "SysML::Systems::EnumerationDefinition::isVariation"))) (name "isVariation") (declared-name "isVariation") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SysML::Systems::EnumerationDefinition")))))
              )
            )
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::EnumerationUsage"))) (name "EnumerationUsage") (declared-name "EnumerationUsage"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::EventOccurrenceUsage"))) (name "EventOccurrenceUsage") (declared-name "EventOccurrenceUsage")
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "SysML::Systems::EventOccurrenceUsage::isReference"))) (name "isReference") (declared-name "isReference") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SysML::Systems::EventOccurrenceUsage")))))
              )
            )
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::ExhibitStateUsage"))) (name "ExhibitStateUsage") (declared-name "ExhibitStateUsage"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::Expose"))) (name "Expose") (declared-name "Expose")
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "SysML::Systems::Expose::isImportAll"))) (name "isImportAll") (declared-name "isImportAll") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SysML::Systems::Expose")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "SysML::Systems::Expose::visibility"))) (name "visibility") (declared-name "visibility") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SysML::Systems::Expose")))))
              )
            )
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::FlowDefinition"))) (name "FlowDefinition") (declared-name "FlowDefinition"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::FlowUsage"))) (name "FlowUsage") (declared-name "FlowUsage"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::ForLoopActionUsage"))) (name "ForLoopActionUsage") (declared-name "ForLoopActionUsage"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::ForkNode"))) (name "ForkNode") (declared-name "ForkNode"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::FramedConcernMembership"))) (name "FramedConcernMembership") (declared-name "FramedConcernMembership")
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "SysML::Systems::FramedConcernMembership::kind"))) (name "kind") (declared-name "kind") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SysML::Systems::FramedConcernMembership")))))
              )
            )
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::IfActionUsage"))) (name "IfActionUsage") (declared-name "IfActionUsage"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::IncludeUseCaseUsage"))) (name "IncludeUseCaseUsage") (declared-name "IncludeUseCaseUsage"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::InterfaceDefinition"))) (name "InterfaceDefinition") (declared-name "InterfaceDefinition"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::InterfaceUsage"))) (name "InterfaceUsage") (declared-name "InterfaceUsage"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::ItemDefinition"))) (name "ItemDefinition") (declared-name "ItemDefinition"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::ItemUsage"))) (name "ItemUsage") (declared-name "ItemUsage"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::JoinNode"))) (name "JoinNode") (declared-name "JoinNode"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::LoopActionUsage"))) (name "LoopActionUsage") (declared-name "LoopActionUsage"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::MembershipExpose"))) (name "MembershipExpose") (declared-name "MembershipExpose"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::MergeNode"))) (name "MergeNode") (declared-name "MergeNode"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::MetadataDefinition"))) (name "MetadataDefinition") (declared-name "MetadataDefinition"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::MetadataUsage"))) (name "MetadataUsage") (declared-name "MetadataUsage"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::NamespaceExpose"))) (name "NamespaceExpose") (declared-name "NamespaceExpose"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::ObjectiveMembership"))) (name "ObjectiveMembership") (declared-name "ObjectiveMembership"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::OccurrenceDefinition"))) (name "OccurrenceDefinition") (declared-name "OccurrenceDefinition")
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "SysML::Systems::OccurrenceDefinition::isIndividual"))) (name "isIndividual") (declared-name "isIndividual") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SysML::Systems::OccurrenceDefinition")))))
              )
            )
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::OccurrenceUsage"))) (name "OccurrenceUsage") (declared-name "OccurrenceUsage")
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "SysML::Systems::OccurrenceUsage::isIndividual"))) (name "isIndividual") (declared-name "isIndividual") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SysML::Systems::OccurrenceUsage")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "SysML::Systems::OccurrenceUsage::portionKind"))) (name "portionKind") (declared-name "portionKind") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SysML::Systems::OccurrenceUsage")))))
              )
            )
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::PartDefinition"))) (name "PartDefinition") (declared-name "PartDefinition"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::PartUsage"))) (name "PartUsage") (declared-name "PartUsage"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::PerformActionUsage"))) (name "PerformActionUsage") (declared-name "PerformActionUsage"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::PortConjugation"))) (name "PortConjugation") (declared-name "PortConjugation"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::PortDefinition"))) (name "PortDefinition") (declared-name "PortDefinition"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::PortUsage"))) (name "PortUsage") (declared-name "PortUsage"))
            (element (kind "enum def") (id (node (document "d0") (qualified-name "SysML::Systems::PortionKind"))) (name "PortionKind") (declared-name "PortionKind")
              (contains
                (element (kind "enumerated value") (id (node (document "d0") (qualified-name "SysML::Systems::PortionKind::snapshot"))) (name "snapshot") (declared-name "snapshot") (effective (featuring-type (node (document "d0") (qualified-name "SysML::Systems::PortionKind")))))
                (element (kind "enumerated value") (id (node (document "d0") (qualified-name "SysML::Systems::PortionKind::timeslice"))) (name "timeslice") (declared-name "timeslice") (effective (featuring-type (node (document "d0") (qualified-name "SysML::Systems::PortionKind")))))
              )
            )
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::ReferenceUsage"))) (name "ReferenceUsage") (declared-name "ReferenceUsage")
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "SysML::Systems::ReferenceUsage::isReference"))) (name "isReference") (declared-name "isReference") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SysML::Systems::ReferenceUsage")))))
              )
            )
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::RenderingDefinition"))) (name "RenderingDefinition") (declared-name "RenderingDefinition"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::RenderingUsage"))) (name "RenderingUsage") (declared-name "RenderingUsage"))
            (element (kind "enum def") (id (node (document "d0") (qualified-name "SysML::Systems::RequirementConstraintKind"))) (name "RequirementConstraintKind") (declared-name "RequirementConstraintKind")
              (contains
                (element (kind "enumerated value") (id (node (document "d0") (qualified-name "SysML::Systems::RequirementConstraintKind::assumption"))) (name "assumption") (declared-name "assumption") (effective (featuring-type (node (document "d0") (qualified-name "SysML::Systems::RequirementConstraintKind")))))
                (element (kind "enumerated value") (id (node (document "d0") (qualified-name "SysML::Systems::RequirementConstraintKind::requirement"))) (name "requirement") (declared-name "requirement") (effective (featuring-type (node (document "d0") (qualified-name "SysML::Systems::RequirementConstraintKind")))))
              )
            )
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::RequirementConstraintMembership"))) (name "RequirementConstraintMembership") (declared-name "RequirementConstraintMembership")
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "SysML::Systems::RequirementConstraintMembership::kind"))) (name "kind") (declared-name "kind") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SysML::Systems::RequirementConstraintMembership")))))
              )
            )
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::RequirementDefinition"))) (name "RequirementDefinition") (declared-name "RequirementDefinition")
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "SysML::Systems::RequirementDefinition::reqId"))) (name "reqId") (declared-name "reqId") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SysML::Systems::RequirementDefinition")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "SysML::Systems::RequirementDefinition::text"))) (name "text") (declared-name "text") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SysML::Systems::RequirementDefinition")))))
              )
            )
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::RequirementUsage"))) (name "RequirementUsage") (declared-name "RequirementUsage")
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "SysML::Systems::RequirementUsage::reqId"))) (name "reqId") (declared-name "reqId") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SysML::Systems::RequirementUsage")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "SysML::Systems::RequirementUsage::text"))) (name "text") (declared-name "text") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SysML::Systems::RequirementUsage")))))
              )
            )
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::RequirementVerificationMembership"))) (name "RequirementVerificationMembership") (declared-name "RequirementVerificationMembership")
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "SysML::Systems::RequirementVerificationMembership::kind"))) (name "kind") (declared-name "kind") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SysML::Systems::RequirementVerificationMembership")))))
              )
            )
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::SatisfyRequirementUsage"))) (name "SatisfyRequirementUsage") (declared-name "SatisfyRequirementUsage"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::SendActionUsage"))) (name "SendActionUsage") (declared-name "SendActionUsage"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::StakeholderMembership"))) (name "StakeholderMembership") (declared-name "StakeholderMembership"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::StateDefinition"))) (name "StateDefinition") (declared-name "StateDefinition")
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "SysML::Systems::StateDefinition::isParallel"))) (name "isParallel") (declared-name "isParallel") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SysML::Systems::StateDefinition")))))
              )
            )
            (element (kind "enum def") (id (node (document "d0") (qualified-name "SysML::Systems::StateSubactionKind"))) (name "StateSubactionKind") (declared-name "StateSubactionKind")
              (contains
                (element (kind "enumerated value") (id (node (document "d0") (qualified-name "SysML::Systems::StateSubactionKind::do"))) (name "do") (declared-name "do") (effective (featuring-type (node (document "d0") (qualified-name "SysML::Systems::StateSubactionKind")))))
                (element (kind "enumerated value") (id (node (document "d0") (qualified-name "SysML::Systems::StateSubactionKind::entry"))) (name "entry") (declared-name "entry") (effective (featuring-type (node (document "d0") (qualified-name "SysML::Systems::StateSubactionKind")))))
                (element (kind "enumerated value") (id (node (document "d0") (qualified-name "SysML::Systems::StateSubactionKind::exit"))) (name "exit") (declared-name "exit") (effective (featuring-type (node (document "d0") (qualified-name "SysML::Systems::StateSubactionKind")))))
              )
            )
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::StateSubactionMembership"))) (name "StateSubactionMembership") (declared-name "StateSubactionMembership")
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "SysML::Systems::StateSubactionMembership::kind"))) (name "kind") (declared-name "kind") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SysML::Systems::StateSubactionMembership")))))
              )
            )
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::StateUsage"))) (name "StateUsage") (declared-name "StateUsage")
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "SysML::Systems::StateUsage::isParallel"))) (name "isParallel") (declared-name "isParallel") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SysML::Systems::StateUsage")))))
              )
            )
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::SubjectMembership"))) (name "SubjectMembership") (declared-name "SubjectMembership"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::SuccessionAsUsage"))) (name "SuccessionAsUsage") (declared-name "SuccessionAsUsage"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::SuccessionFlowUsage"))) (name "SuccessionFlowUsage") (declared-name "SuccessionFlowUsage"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::TerminateActionUsage"))) (name "TerminateActionUsage") (declared-name "TerminateActionUsage"))
            (element (kind "enum def") (id (node (document "d0") (qualified-name "SysML::Systems::TransitionFeatureKind"))) (name "TransitionFeatureKind") (declared-name "TransitionFeatureKind")
              (contains
                (element (kind "enumerated value") (id (node (document "d0") (qualified-name "SysML::Systems::TransitionFeatureKind::effect"))) (name "effect") (declared-name "effect") (effective (featuring-type (node (document "d0") (qualified-name "SysML::Systems::TransitionFeatureKind")))))
                (element (kind "enumerated value") (id (node (document "d0") (qualified-name "SysML::Systems::TransitionFeatureKind::guard"))) (name "guard") (declared-name "guard") (effective (featuring-type (node (document "d0") (qualified-name "SysML::Systems::TransitionFeatureKind")))))
                (element (kind "enumerated value") (id (node (document "d0") (qualified-name "SysML::Systems::TransitionFeatureKind::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "SysML::Systems::TransitionFeatureKind")))))
              )
            )
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::TransitionFeatureMembership"))) (name "TransitionFeatureMembership") (declared-name "TransitionFeatureMembership")
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "SysML::Systems::TransitionFeatureMembership::kind"))) (name "kind") (declared-name "kind") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SysML::Systems::TransitionFeatureMembership")))))
              )
            )
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::TransitionUsage"))) (name "TransitionUsage") (declared-name "TransitionUsage"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::TriggerInvocationExpression"))) (name "TriggerInvocationExpression") (declared-name "TriggerInvocationExpression")
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "SysML::Systems::TriggerInvocationExpression::kind"))) (name "kind") (declared-name "kind") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SysML::Systems::TriggerInvocationExpression")))))
              )
            )
            (element (kind "enum def") (id (node (document "d0") (qualified-name "SysML::Systems::TriggerKind"))) (name "TriggerKind") (declared-name "TriggerKind")
              (contains
                (element (kind "enumerated value") (id (node (document "d0") (qualified-name "SysML::Systems::TriggerKind::after"))) (name "after") (declared-name "after") (effective (featuring-type (node (document "d0") (qualified-name "SysML::Systems::TriggerKind")))))
                (element (kind "enumerated value") (id (node (document "d0") (qualified-name "SysML::Systems::TriggerKind::at"))) (name "at") (declared-name "at") (effective (featuring-type (node (document "d0") (qualified-name "SysML::Systems::TriggerKind")))))
                (element (kind "enumerated value") (id (node (document "d0") (qualified-name "SysML::Systems::TriggerKind::when"))) (name "when") (declared-name "when") (effective (featuring-type (node (document "d0") (qualified-name "SysML::Systems::TriggerKind")))))
              )
            )
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::Usage"))) (name "Usage") (declared-name "Usage")
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "SysML::Systems::Usage::isReference"))) (name "isReference") (declared-name "isReference") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SysML::Systems::Usage")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "SysML::Systems::Usage::isVariation"))) (name "isVariation") (declared-name "isVariation") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SysML::Systems::Usage")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "SysML::Systems::Usage::mayTimeVary"))) (name "mayTimeVary") (declared-name "mayTimeVary") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "SysML::Systems::Usage")))))
              )
            )
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::UseCaseDefinition"))) (name "UseCaseDefinition") (declared-name "UseCaseDefinition"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::UseCaseUsage"))) (name "UseCaseUsage") (declared-name "UseCaseUsage"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::VariantMembership"))) (name "VariantMembership") (declared-name "VariantMembership"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::VerificationCaseDefinition"))) (name "VerificationCaseDefinition") (declared-name "VerificationCaseDefinition"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::VerificationCaseUsage"))) (name "VerificationCaseUsage") (declared-name "VerificationCaseUsage"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::ViewDefinition"))) (name "ViewDefinition") (declared-name "ViewDefinition"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::ViewRenderingMembership"))) (name "ViewRenderingMembership") (declared-name "ViewRenderingMembership"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::ViewUsage"))) (name "ViewUsage") (declared-name "ViewUsage"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::ViewpointDefinition"))) (name "ViewpointDefinition") (declared-name "ViewpointDefinition"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::ViewpointUsage"))) (name "ViewpointUsage") (declared-name "ViewpointUsage"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "SysML::Systems::WhileLoopActionUsage"))) (name "WhileLoopActionUsage") (declared-name "WhileLoopActionUsage"))
          )
        )
        (element (kind "documentation") (id (node (document "d0") (qualified-name "SysML::_documentation"))) (name ""))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "SysML::_documentation"))) (to (node (document "d0") (qualified-name "SysML"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::AttributeUsage::isReference"))) (to (node (document "d0") (qualified-name "SysML::Systems::Usage::isReference"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::EnumerationDefinition::isVariation"))) (to (node (document "d0") (qualified-name "SysML::Systems::Definition::isVariation"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::EventOccurrenceUsage::isReference"))) (to (node (document "d0") (qualified-name "SysML::Systems::Usage::isReference"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::FramedConcernMembership::kind"))) (to (node (document "d0") (qualified-name "SysML::Systems::RequirementConstraintMembership::kind"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::ReferenceUsage::isReference"))) (to (node (document "d0") (qualified-name "SysML::Systems::Usage::isReference"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::RequirementVerificationMembership::kind"))) (to (node (document "d0") (qualified-name "SysML::Systems::RequirementConstraintMembership::kind"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::AcceptActionUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::ActionUsage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::ActionDefinition"))) (to (node (document "d0") (qualified-name "SysML::Systems::OccurrenceDefinition"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::ActionUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::OccurrenceUsage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::AllocationDefinition"))) (to (node (document "d0") (qualified-name "SysML::Systems::ConnectionDefinition"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::AllocationUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::ConnectionUsage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::AnalysisCaseDefinition"))) (to (node (document "d0") (qualified-name "SysML::Systems::CaseDefinition"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::AnalysisCaseUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::CaseUsage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::AssertConstraintUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::ConstraintUsage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::AssignmentActionUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::ActionUsage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::AttributeDefinition"))) (to (node (document "d0") (qualified-name "SysML::Systems::Definition"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::AttributeUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::Usage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::BindingConnectorAsUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::ConnectorAsUsage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::CalculationDefinition"))) (to (node (document "d0") (qualified-name "SysML::Systems::ActionDefinition"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::CalculationUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::ActionUsage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::CaseDefinition"))) (to (node (document "d0") (qualified-name "SysML::Systems::CalculationDefinition"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::CaseUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::CalculationUsage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::ConcernDefinition"))) (to (node (document "d0") (qualified-name "SysML::Systems::RequirementDefinition"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::ConcernUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::RequirementUsage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::ConjugatedPortDefinition"))) (to (node (document "d0") (qualified-name "SysML::Systems::PortDefinition"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::ConnectionDefinition"))) (to (node (document "d0") (qualified-name "SysML::Systems::PartDefinition"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::ConnectionUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::ConnectorAsUsage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::ConnectionUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::PartUsage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::ConnectorAsUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::Usage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::ConstraintDefinition"))) (to (node (document "d0") (qualified-name "SysML::Systems::OccurrenceDefinition"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::ConstraintUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::OccurrenceUsage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::ControlNode"))) (to (node (document "d0") (qualified-name "SysML::Systems::ActionUsage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::DecisionNode"))) (to (node (document "d0") (qualified-name "SysML::Systems::ControlNode"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::EnumerationDefinition"))) (to (node (document "d0") (qualified-name "SysML::Systems::AttributeDefinition"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::EnumerationUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::AttributeUsage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::EventOccurrenceUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::OccurrenceUsage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::ExhibitStateUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::PerformActionUsage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::ExhibitStateUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::StateUsage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::FlowDefinition"))) (to (node (document "d0") (qualified-name "SysML::Systems::ActionDefinition"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::FlowUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::ActionUsage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::FlowUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::ConnectorAsUsage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::ForLoopActionUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::LoopActionUsage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::ForkNode"))) (to (node (document "d0") (qualified-name "SysML::Systems::ControlNode"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::FramedConcernMembership"))) (to (node (document "d0") (qualified-name "SysML::Systems::RequirementConstraintMembership"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::IfActionUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::ActionUsage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::IncludeUseCaseUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::PerformActionUsage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::IncludeUseCaseUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::UseCaseUsage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::InterfaceDefinition"))) (to (node (document "d0") (qualified-name "SysML::Systems::ConnectionDefinition"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::InterfaceUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::ConnectionUsage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::ItemDefinition"))) (to (node (document "d0") (qualified-name "SysML::Systems::OccurrenceDefinition"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::ItemUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::OccurrenceUsage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::JoinNode"))) (to (node (document "d0") (qualified-name "SysML::Systems::ControlNode"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::LoopActionUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::ActionUsage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::MembershipExpose"))) (to (node (document "d0") (qualified-name "SysML::Systems::Expose"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::MergeNode"))) (to (node (document "d0") (qualified-name "SysML::Systems::ControlNode"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::MetadataDefinition"))) (to (node (document "d0") (qualified-name "SysML::Systems::ItemDefinition"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::MetadataUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::ItemUsage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::NamespaceExpose"))) (to (node (document "d0") (qualified-name "SysML::Systems::Expose"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::OccurrenceDefinition"))) (to (node (document "d0") (qualified-name "SysML::Systems::Definition"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::OccurrenceUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::Usage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::PartDefinition"))) (to (node (document "d0") (qualified-name "SysML::Systems::ItemDefinition"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::PartUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::ItemUsage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::PerformActionUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::ActionUsage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::PerformActionUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::EventOccurrenceUsage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::PortDefinition"))) (to (node (document "d0") (qualified-name "SysML::Systems::OccurrenceDefinition"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::PortUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::OccurrenceUsage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::ReferenceUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::Usage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::RenderingDefinition"))) (to (node (document "d0") (qualified-name "SysML::Systems::PartDefinition"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::RenderingUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::PartUsage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::RequirementDefinition"))) (to (node (document "d0") (qualified-name "SysML::Systems::ConstraintDefinition"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::RequirementUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::ConstraintUsage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::RequirementVerificationMembership"))) (to (node (document "d0") (qualified-name "SysML::Systems::RequirementConstraintMembership"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::SatisfyRequirementUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::AssertConstraintUsage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::SatisfyRequirementUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::RequirementUsage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::SendActionUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::ActionUsage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::StateDefinition"))) (to (node (document "d0") (qualified-name "SysML::Systems::ActionDefinition"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::StateUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::ActionUsage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::SuccessionAsUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::ConnectorAsUsage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::SuccessionFlowUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::FlowUsage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::TerminateActionUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::ActionUsage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::TransitionUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::ActionUsage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::UseCaseDefinition"))) (to (node (document "d0") (qualified-name "SysML::Systems::CaseDefinition"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::UseCaseUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::CaseUsage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::VerificationCaseDefinition"))) (to (node (document "d0") (qualified-name "SysML::Systems::CaseDefinition"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::VerificationCaseUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::CaseUsage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::ViewDefinition"))) (to (node (document "d0") (qualified-name "SysML::Systems::PartDefinition"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::ViewUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::PartUsage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::ViewpointDefinition"))) (to (node (document "d0") (qualified-name "SysML::Systems::RequirementDefinition"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::ViewpointUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::RequirementUsage"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::WhileLoopActionUsage"))) (to (node (document "d0") (qualified-name "SysML::Systems::LoopActionUsage"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::FramedConcernMembership::kind"))) (to (node (document "d0") (qualified-name "SysML::Systems::RequirementConstraintKind"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::OccurrenceUsage::portionKind"))) (to (node (document "d0") (qualified-name "SysML::Systems::PortionKind"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::RequirementConstraintMembership::kind"))) (to (node (document "d0") (qualified-name "SysML::Systems::RequirementConstraintKind"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::RequirementVerificationMembership::kind"))) (to (node (document "d0") (qualified-name "SysML::Systems::RequirementConstraintKind"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::StateSubactionMembership::kind"))) (to (node (document "d0") (qualified-name "SysML::Systems::StateSubactionKind"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::TransitionFeatureMembership::kind"))) (to (node (document "d0") (qualified-name "SysML::Systems::TransitionFeatureKind"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "SysML::Systems::TriggerInvocationExpression::kind"))) (to (node (document "d0") (qualified-name "SysML::Systems::TriggerKind"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml.library/sys_ml.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 1) (end 6 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 2) (end 10 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 18 2) (end 18 190))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 22 2) (end 22 202))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 26 2) (end 26 186))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 47 2) (end 47 177))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 57 2) (end 57 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 60 3) (end 60 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 65 2) (end 65 86))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 67 2) (end 67 208))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 71 2) (end 71 210))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 99 2) (end 99 276))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 104 2) (end 104 274))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 105 3) (end 105 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 105 3) (end 105 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 114 2) (end 114 70))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 116 2) (end 116 80))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 118 2) (end 118 195))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 126 2) (end 126 3700))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 127 3) (end 127 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 163 3) (end 163 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 173 3) (end 173 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 182 2) (end 182 188))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 183 3) (end 183 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 183 3) (end 183 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 184 3) (end 184 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 184 3) (end 184 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 187 2) (end 187 177))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 191 2) (end 191 214))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 227 2) (end 227 74))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 239 2) (end 239 69))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 243 2) (end 243 72))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 245 2) (end 245 199))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 249 2) (end 249 67))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 251 2) (end 251 199))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 255 2) (end 255 114))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 256 3) (end 256 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 260 3) (end 260 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 277 2) (end 277 306))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 282 2) (end 282 210))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 296 3) (end 296 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 312 2) (end 312 355))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 320 3) (end 320 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 332 3) (end 332 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 362 2) (end 362 198))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 367 3) (end 367 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 381 2) (end 381 232))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 388 3) (end 388 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 396 2) (end 396 186))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 400 2) (end 400 74))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 402 2) (end 402 73))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 414 2) (end 414 240))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 429 2) (end 429 120))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 439 2) (end 439 4178))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 440 3) (end 440 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 441 3) (end 441 70))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 441 3) (end 441 70))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 442 3) (end 442 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 489 2) (end 489 177))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 509 2) (end 509 286))
      )
    )
  )
)
~~~
