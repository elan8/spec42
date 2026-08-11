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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sys_ml.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 16) (end 6 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 16) (end 10 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 18 44) (end 18 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 22 39) (end 22 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 26 43) (end 26 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 47 66) (end 47 75))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 57 47) (end 57 55))
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
        (range (start 65 51) (end 65 67))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 67 49) (end 67 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 71 44) (end 71 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 99 48) (end 99 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 104 48) (end 104 68))
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
        (range (start 114 60) (end 114 69))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 116 70) (end 116 79))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 118 43) (end 118 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 126 38) (end 126 48))
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
        (range (start 182 43) (end 182 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 183 3) (end 183 68))
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
        (range (start 187 42) (end 187 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 191 55) (end 191 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 227 42) (end 227 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 239 44) (end 239 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 243 62) (end 243 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 245 52) (end 245 67))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 249 51) (end 249 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 251 47) (end 251 64))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 255 60) (end 255 65))
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
        (range (start 277 43) (end 277 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 282 64) (end 282 73))
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
        (range (start 312 59) (end 312 76))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 313 3) (end 313 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 320 3) (end 320 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 320 44) (end 320 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 321 3) (end 321 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 332 3) (end 332 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 332 44) (end 332 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 333 3) (end 333 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 362 49) (end 362 68))
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
        (range (start 381 52) (end 381 69))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 382 3) (end 382 45))
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
        (range (start 396 45) (end 396 64))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 400 63) (end 400 73))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 402 47) (end 402 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 414 55) (end 414 72))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 415 3) (end 415 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 429 55) (end 429 75))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 430 3) (end 430 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 439 33) (end 439 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 440 3) (end 440 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 441 3) (end 441 70))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 441 59) (end 441 69))
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
        (range (start 489 45) (end 489 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 509 51) (end 509 68))
      )
    )
  )
)
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
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "25da17bc983af759f12ed870af4b8880774df92277b1f98a31d688e9f7848cd7") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "SysML"))) (kind "package") (name "SysML") (declared-name "SysML") (range (start (line 0) (character 0)) (end (line 0) (character 30788))))
    (element (id (node (document "d0") (qualified-name "SysML::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 6) (character 1)) (end (line 6) (character 32))) (parent (node (document "d0") (qualified-name "SysML"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 6) (character 16)) (end (line 6) (character 28))))))
    (element (id (node (document "d0") (qualified-name "SysML::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 7) (character 1)) (end (line 7) (character 26))) (parent (node (document "d0") (qualified-name "SysML"))) (authored (membership (kind Import) (visibility "public") (import (reference "Systems::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 7) (character 15)) (end (line 7) (character 22))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems"))) (kind "package") (name "Systems") (declared-name "Systems") (range (start (line 9) (character 1)) (end (line 9) (character 30590))) (parent (node (document "d0") (qualified-name "SysML"))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 10) (character 2)) (end (line 10) (character 33))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Import) (visibility "public") (import (reference "KerML::Kernel::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 10) (character 16)) (end (line 10) (character 29))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::AcceptActionUsage"))) (kind "metadata def") (name "AcceptActionUsage") (declared-name "AcceptActionUsage") (range (start (line 12) (character 2)) (end (line 12) (character 367))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ActionUsage") (range (start (line 12) (character 45)) (end (line 12) (character 56)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::ActionDefinition"))) (kind "metadata def") (name "ActionDefinition") (declared-name "ActionDefinition") (range (start (line 18) (character 2)) (end (line 18) (character 190))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Behavior") (range (start (line 18) (character 44)) (end (line 18) (character 52)))) (specializes (reference "OccurrenceDefinition") (range (start (line 18) (character 54)) (end (line 18) (character 74)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::ActionUsage"))) (kind "metadata def") (name "ActionUsage") (declared-name "ActionUsage") (range (start (line 22) (character 2)) (end (line 22) (character 202))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Step") (range (start (line 22) (character 39)) (end (line 22) (character 43)))) (specializes (reference "OccurrenceUsage") (range (start (line 22) (character 45)) (end (line 22) (character 60)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::ActorMembership"))) (kind "metadata def") (name "ActorMembership") (declared-name "ActorMembership") (range (start (line 26) (character 2)) (end (line 26) (character 186))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ParameterMembership") (range (start (line 26) (character 43)) (end (line 26) (character 62)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::AllocationDefinition"))) (kind "metadata def") (name "AllocationDefinition") (declared-name "AllocationDefinition") (range (start (line 30) (character 2)) (end (line 30) (character 186))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ConnectionDefinition") (range (start (line 30) (character 48)) (end (line 30) (character 68)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::AllocationUsage"))) (kind "metadata def") (name "AllocationUsage") (declared-name "AllocationUsage") (range (start (line 34) (character 2)) (end (line 34) (character 206))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ConnectionUsage") (range (start (line 34) (character 43)) (end (line 34) (character 58)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::AnalysisCaseDefinition"))) (kind "metadata def") (name "AnalysisCaseDefinition") (declared-name "AnalysisCaseDefinition") (range (start (line 38) (character 2)) (end (line 38) (character 192))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "CaseDefinition") (range (start (line 38) (character 50)) (end (line 38) (character 64)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::AnalysisCaseUsage"))) (kind "metadata def") (name "AnalysisCaseUsage") (declared-name "AnalysisCaseUsage") (range (start (line 42) (character 2)) (end (line 42) (character 302))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "CaseUsage") (range (start (line 42) (character 45)) (end (line 42) (character 54)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::AssertConstraintUsage"))) (kind "metadata def") (name "AssertConstraintUsage") (declared-name "AssertConstraintUsage") (range (start (line 47) (character 2)) (end (line 47) (character 177))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ConstraintUsage") (range (start (line 47) (character 49)) (end (line 47) (character 64)))) (specializes (reference "Invariant") (range (start (line 47) (character 66)) (end (line 47) (character 75)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::AssignmentActionUsage"))) (kind "metadata def") (name "AssignmentActionUsage") (declared-name "AssignmentActionUsage") (range (start (line 51) (character 2)) (end (line 51) (character 334))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ActionUsage") (range (start (line 51) (character 49)) (end (line 51) (character 60)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::AttributeDefinition"))) (kind "metadata def") (name "AttributeDefinition") (declared-name "AttributeDefinition") (range (start (line 57) (character 2)) (end (line 57) (character 68))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "DataType") (range (start (line 57) (character 47)) (end (line 57) (character 55)))) (specializes (reference "Definition") (range (start (line 57) (character 57)) (end (line 57) (character 67)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::AttributeUsage"))) (kind "metadata def") (name "AttributeUsage") (declared-name "AttributeUsage") (range (start (line 59) (character 2)) (end (line 59) (character 248))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Usage") (range (start (line 59) (character 42)) (end (line 59) (character 47)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::AttributeUsage::isReference"))) (kind "attribute") (name "isReference") (declared-name "isReference") (range (start (line 60) (character 3)) (end (line 60) (character 71))) (parent (node (document "d0") (qualified-name "SysML::Systems::AttributeUsage"))) (authored (membership (kind Feature)) (relationships (typing (reference "Boolean") (range none)) (redefinition (reference "isReference") (range (start (line 60) (character 59)) (end (line 60) (character 70)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::BindingConnectorAsUsage"))) (kind "metadata def") (name "BindingConnectorAsUsage") (declared-name "BindingConnectorAsUsage") (range (start (line 65) (character 2)) (end (line 65) (character 86))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "BindingConnector") (range (start (line 65) (character 51)) (end (line 65) (character 67)))) (specializes (reference "ConnectorAsUsage") (range (start (line 65) (character 69)) (end (line 65) (character 85)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::CalculationDefinition"))) (kind "metadata def") (name "CalculationDefinition") (declared-name "CalculationDefinition") (range (start (line 67) (character 2)) (end (line 67) (character 208))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Function") (range (start (line 67) (character 49)) (end (line 67) (character 57)))) (specializes (reference "ActionDefinition") (range (start (line 67) (character 59)) (end (line 67) (character 75)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::CalculationUsage"))) (kind "metadata def") (name "CalculationUsage") (declared-name "CalculationUsage") (range (start (line 71) (character 2)) (end (line 71) (character 210))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Expression") (range (start (line 71) (character 44)) (end (line 71) (character 54)))) (specializes (reference "ActionUsage") (range (start (line 71) (character 56)) (end (line 71) (character 67)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::CaseDefinition"))) (kind "metadata def") (name "CaseDefinition") (declared-name "CaseDefinition") (range (start (line 75) (character 2)) (end (line 75) (character 418))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "CalculationDefinition") (range (start (line 75) (character 42)) (end (line 75) (character 63)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::CaseUsage"))) (kind "metadata def") (name "CaseUsage") (declared-name "CaseUsage") (range (start (line 81) (character 2)) (end (line 81) (character 531))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "CalculationUsage") (range (start (line 81) (character 37)) (end (line 81) (character 53)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::ConcernDefinition"))) (kind "metadata def") (name "ConcernDefinition") (declared-name "ConcernDefinition") (range (start (line 88) (character 2)) (end (line 88) (character 67))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "RequirementDefinition") (range (start (line 88) (character 45)) (end (line 88) (character 66)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::ConcernUsage"))) (kind "metadata def") (name "ConcernUsage") (declared-name "ConcernUsage") (range (start (line 90) (character 2)) (end (line 90) (character 191))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "RequirementUsage") (range (start (line 90) (character 40)) (end (line 90) (character 56)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::ConjugatedPortDefinition"))) (kind "metadata def") (name "ConjugatedPortDefinition") (declared-name "ConjugatedPortDefinition") (range (start (line 94) (character 2)) (end (line 94) (character 320))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "PortDefinition") (range (start (line 94) (character 52)) (end (line 94) (character 66)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::ConjugatedPortTyping"))) (kind "metadata def") (name "ConjugatedPortTyping") (declared-name "ConjugatedPortTyping") (range (start (line 99) (character 2)) (end (line 99) (character 276))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "FeatureTyping") (range (start (line 99) (character 48)) (end (line 99) (character 61)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::ConnectionDefinition"))) (kind "metadata def") (name "ConnectionDefinition") (declared-name "ConnectionDefinition") (range (start (line 104) (character 2)) (end (line 104) (character 274))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "AssociationStructure") (range (start (line 104) (character 48)) (end (line 104) (character 68)))) (specializes (reference "PartDefinition") (range (start (line 104) (character 70)) (end (line 104) (character 84)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::ConnectionDefinition::isSufficient"))) (kind "attribute") (name "isSufficient") (declared-name "isSufficient") (range (start (line 105) (character 3)) (end (line 105) (character 65))) (parent (node (document "d0") (qualified-name "SysML::Systems::ConnectionDefinition"))) (authored (membership (kind Feature)) (relationships (typing (reference "Boolean") (range none)) (redefinition (reference "isSufficient") (range (start (line 105) (character 52)) (end (line 105) (character 64)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::ConnectionUsage"))) (kind "metadata def") (name "ConnectionUsage") (declared-name "ConnectionUsage") (range (start (line 110) (character 2)) (end (line 110) (character 232))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ConnectorAsUsage") (range (start (line 110) (character 43)) (end (line 110) (character 59)))) (specializes (reference "PartUsage") (range (start (line 110) (character 61)) (end (line 110) (character 70)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::ConnectorAsUsage"))) (kind "metadata def") (name "ConnectorAsUsage") (declared-name "ConnectorAsUsage") (range (start (line 114) (character 2)) (end (line 114) (character 70))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Usage") (range (start (line 114) (character 53)) (end (line 114) (character 58)))) (specializes (reference "Connector") (range (start (line 114) (character 60)) (end (line 114) (character 69)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::ConstraintDefinition"))) (kind "metadata def") (name "ConstraintDefinition") (declared-name "ConstraintDefinition") (range (start (line 116) (character 2)) (end (line 116) (character 80))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "OccurrenceDefinition") (range (start (line 116) (character 48)) (end (line 116) (character 68)))) (specializes (reference "Predicate") (range (start (line 116) (character 70)) (end (line 116) (character 79)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::ConstraintUsage"))) (kind "metadata def") (name "ConstraintUsage") (declared-name "ConstraintUsage") (range (start (line 118) (character 2)) (end (line 118) (character 195))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "BooleanExpression") (range (start (line 118) (character 43)) (end (line 118) (character 60)))) (specializes (reference "OccurrenceUsage") (range (start (line 118) (character 62)) (end (line 118) (character 77)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::ControlNode"))) (kind "metadata def") (name "ControlNode") (declared-name "ControlNode") (range (start (line 122) (character 2)) (end (line 122) (character 60))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ActionUsage") (range (start (line 122) (character 48)) (end (line 122) (character 59)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::DecisionNode"))) (kind "metadata def") (name "DecisionNode") (declared-name "DecisionNode") (range (start (line 124) (character 2)) (end (line 124) (character 52))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ControlNode") (range (start (line 124) (character 40)) (end (line 124) (character 51)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::Definition"))) (kind "metadata def") (name "Definition") (declared-name "Definition") (range (start (line 126) (character 2)) (end (line 126) (character 3700))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Classifier") (range (start (line 126) (character 38)) (end (line 126) (character 48)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::Definition::isVariation"))) (kind "attribute") (name "isVariation") (declared-name "isVariation") (range (start (line 127) (character 3)) (end (line 127) (character 41))) (parent (node (document "d0") (qualified-name "SysML::Systems::Definition"))) (authored (membership (kind Feature)) (relationships (typing (reference "Boolean") (range none)))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::EnumerationDefinition"))) (kind "metadata def") (name "EnumerationDefinition") (declared-name "EnumerationDefinition") (range (start (line 162) (character 2)) (end (line 162) (character 264))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "AttributeDefinition") (range (start (line 162) (character 49)) (end (line 162) (character 68)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::EnumerationDefinition::isVariation"))) (kind "attribute") (name "isVariation") (declared-name "isVariation") (range (start (line 163) (character 3)) (end (line 163) (character 63))) (parent (node (document "d0") (qualified-name "SysML::Systems::EnumerationDefinition"))) (authored (membership (kind Feature)) (relationships (typing (reference "Boolean") (range none)) (redefinition (reference "isVariation") (range (start (line 163) (character 51)) (end (line 163) (character 62)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::EnumerationUsage"))) (kind "metadata def") (name "EnumerationUsage") (declared-name "EnumerationUsage") (range (start (line 168) (character 2)) (end (line 168) (character 199))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "AttributeUsage") (range (start (line 168) (character 44)) (end (line 168) (character 58)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::EventOccurrenceUsage"))) (kind "metadata def") (name "EventOccurrenceUsage") (declared-name "EventOccurrenceUsage") (range (start (line 172) (character 2)) (end (line 172) (character 238))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "OccurrenceUsage") (range (start (line 172) (character 48)) (end (line 172) (character 63)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::EventOccurrenceUsage::isReference"))) (kind "attribute") (name "isReference") (declared-name "isReference") (range (start (line 173) (character 3)) (end (line 173) (character 71))) (parent (node (document "d0") (qualified-name "SysML::Systems::EventOccurrenceUsage"))) (authored (membership (kind Feature)) (relationships (typing (reference "Boolean") (range none)) (redefinition (reference "isReference") (range (start (line 173) (character 59)) (end (line 173) (character 70)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::ExhibitStateUsage"))) (kind "metadata def") (name "ExhibitStateUsage") (declared-name "ExhibitStateUsage") (range (start (line 178) (character 2)) (end (line 178) (character 194))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "StateUsage") (range (start (line 178) (character 45)) (end (line 178) (character 55)))) (specializes (reference "PerformActionUsage") (range (start (line 178) (character 57)) (end (line 178) (character 75)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::Expose"))) (kind "metadata def") (name "Expose") (declared-name "Expose") (range (start (line 182) (character 2)) (end (line 182) (character 188))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Import") (range (start (line 182) (character 43)) (end (line 182) (character 49)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::Expose::isImportAll"))) (kind "attribute") (name "isImportAll") (declared-name "isImportAll") (range (start (line 184) (character 3)) (end (line 184) (character 63))) (parent (node (document "d0") (qualified-name "SysML::Systems::Expose"))) (authored (membership (kind Feature)) (relationships (typing (reference "Boolean") (range none)) (redefinition (reference "isImportAll") (range (start (line 184) (character 51)) (end (line 184) (character 62)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::Expose::visibility"))) (kind "attribute") (name "visibility") (declared-name "visibility") (range (start (line 183) (character 3)) (end (line 183) (character 68))) (parent (node (document "d0") (qualified-name "SysML::Systems::Expose"))) (authored (membership (kind Feature)) (relationships (typing (reference "VisibilityKind") (range none)) (redefinition (reference "visibility") (range (start (line 183) (character 57)) (end (line 183) (character 67)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::FlowDefinition"))) (kind "metadata def") (name "FlowDefinition") (declared-name "FlowDefinition") (range (start (line 187) (character 2)) (end (line 187) (character 177))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Interaction") (range (start (line 187) (character 42)) (end (line 187) (character 53)))) (specializes (reference "ActionDefinition") (range (start (line 187) (character 55)) (end (line 187) (character 71)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::FlowUsage"))) (kind "metadata def") (name "FlowUsage") (declared-name "FlowUsage") (range (start (line 191) (character 2)) (end (line 191) (character 214))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ConnectorAsUsage") (range (start (line 191) (character 37)) (end (line 191) (character 53)))) (specializes (reference "Flow") (range (start (line 191) (character 55)) (end (line 191) (character 59)))) (specializes (reference "ActionUsage") (range (start (line 191) (character 61)) (end (line 191) (character 72)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::ForLoopActionUsage"))) (kind "metadata def") (name "ForLoopActionUsage") (declared-name "ForLoopActionUsage") (range (start (line 195) (character 2)) (end (line 195) (character 240))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "LoopActionUsage") (range (start (line 195) (character 46)) (end (line 195) (character 61)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::ForkNode"))) (kind "metadata def") (name "ForkNode") (declared-name "ForkNode") (range (start (line 200) (character 2)) (end (line 200) (character 48))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ControlNode") (range (start (line 200) (character 36)) (end (line 200) (character 47)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::FramedConcernMembership"))) (kind "metadata def") (name "FramedConcernMembership") (declared-name "FramedConcernMembership") (range (start (line 202) (character 2)) (end (line 202) (character 392))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "RequirementConstraintMembership") (range (start (line 202) (character 51)) (end (line 202) (character 82)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::FramedConcernMembership::kind"))) (kind "attribute") (name "kind") (declared-name "kind") (range (start (line 203) (character 3)) (end (line 203) (character 67))) (parent (node (document "d0") (qualified-name "SysML::Systems::FramedConcernMembership"))) (authored (membership (kind Feature)) (relationships (typing (reference "RequirementConstraintKind") (range none)) (redefinition (reference "kind") (range (start (line 203) (character 62)) (end (line 203) (character 66)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::IfActionUsage"))) (kind "metadata def") (name "IfActionUsage") (declared-name "IfActionUsage") (range (start (line 209) (character 2)) (end (line 209) (character 309))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ActionUsage") (range (start (line 209) (character 41)) (end (line 209) (character 52)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::IncludeUseCaseUsage"))) (kind "metadata def") (name "IncludeUseCaseUsage") (declared-name "IncludeUseCaseUsage") (range (start (line 215) (character 2)) (end (line 215) (character 201))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "UseCaseUsage") (range (start (line 215) (character 47)) (end (line 215) (character 59)))) (specializes (reference "PerformActionUsage") (range (start (line 215) (character 61)) (end (line 215) (character 79)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::InterfaceDefinition"))) (kind "metadata def") (name "InterfaceDefinition") (declared-name "InterfaceDefinition") (range (start (line 219) (character 2)) (end (line 219) (character 189))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ConnectionDefinition") (range (start (line 219) (character 47)) (end (line 219) (character 67)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::InterfaceUsage"))) (kind "metadata def") (name "InterfaceUsage") (declared-name "InterfaceUsage") (range (start (line 223) (character 2)) (end (line 223) (character 195))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ConnectionUsage") (range (start (line 223) (character 42)) (end (line 223) (character 57)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::ItemDefinition"))) (kind "metadata def") (name "ItemDefinition") (declared-name "ItemDefinition") (range (start (line 227) (character 2)) (end (line 227) (character 74))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Structure") (range (start (line 227) (character 42)) (end (line 227) (character 51)))) (specializes (reference "OccurrenceDefinition") (range (start (line 227) (character 53)) (end (line 227) (character 73)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::ItemUsage"))) (kind "metadata def") (name "ItemUsage") (declared-name "ItemUsage") (range (start (line 229) (character 2)) (end (line 229) (character 213))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "OccurrenceUsage") (range (start (line 229) (character 37)) (end (line 229) (character 52)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::JoinNode"))) (kind "metadata def") (name "JoinNode") (declared-name "JoinNode") (range (start (line 233) (character 2)) (end (line 233) (character 48))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ControlNode") (range (start (line 233) (character 36)) (end (line 233) (character 47)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::LoopActionUsage"))) (kind "metadata def") (name "LoopActionUsage") (declared-name "LoopActionUsage") (range (start (line 235) (character 2)) (end (line 235) (character 153))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ActionUsage") (range (start (line 235) (character 52)) (end (line 235) (character 63)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::MembershipExpose"))) (kind "metadata def") (name "MembershipExpose") (declared-name "MembershipExpose") (range (start (line 239) (character 2)) (end (line 239) (character 69))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "MembershipImport") (range (start (line 239) (character 44)) (end (line 239) (character 60)))) (specializes (reference "Expose") (range (start (line 239) (character 62)) (end (line 239) (character 68)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::MergeNode"))) (kind "metadata def") (name "MergeNode") (declared-name "MergeNode") (range (start (line 241) (character 2)) (end (line 241) (character 49))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ControlNode") (range (start (line 241) (character 37)) (end (line 241) (character 48)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::MetadataDefinition"))) (kind "metadata def") (name "MetadataDefinition") (declared-name "MetadataDefinition") (range (start (line 243) (character 2)) (end (line 243) (character 72))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ItemDefinition") (range (start (line 243) (character 46)) (end (line 243) (character 60)))) (specializes (reference "Metaclass") (range (start (line 243) (character 62)) (end (line 243) (character 71)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::MetadataUsage"))) (kind "metadata def") (name "MetadataUsage") (declared-name "MetadataUsage") (range (start (line 245) (character 2)) (end (line 245) (character 199))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ItemUsage") (range (start (line 245) (character 41)) (end (line 245) (character 50)))) (specializes (reference "MetadataFeature") (range (start (line 245) (character 52)) (end (line 245) (character 67)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::NamespaceExpose"))) (kind "metadata def") (name "NamespaceExpose") (declared-name "NamespaceExpose") (range (start (line 249) (character 2)) (end (line 249) (character 67))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Expose") (range (start (line 249) (character 43)) (end (line 249) (character 49)))) (specializes (reference "NamespaceImport") (range (start (line 249) (character 51)) (end (line 249) (character 66)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::ObjectiveMembership"))) (kind "metadata def") (name "ObjectiveMembership") (declared-name "ObjectiveMembership") (range (start (line 251) (character 2)) (end (line 251) (character 199))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "FeatureMembership") (range (start (line 251) (character 47)) (end (line 251) (character 64)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::OccurrenceDefinition"))) (kind "metadata def") (name "OccurrenceDefinition") (declared-name "OccurrenceDefinition") (range (start (line 255) (character 2)) (end (line 255) (character 114))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Definition") (range (start (line 255) (character 48)) (end (line 255) (character 58)))) (specializes (reference "Class") (range (start (line 255) (character 60)) (end (line 255) (character 65)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::OccurrenceDefinition::isIndividual"))) (kind "attribute") (name "isIndividual") (declared-name "isIndividual") (range (start (line 256) (character 3)) (end (line 256) (character 42))) (parent (node (document "d0") (qualified-name "SysML::Systems::OccurrenceDefinition"))) (authored (membership (kind Feature)) (relationships (typing (reference "Boolean") (range none)))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::OccurrenceUsage"))) (kind "metadata def") (name "OccurrenceUsage") (declared-name "OccurrenceUsage") (range (start (line 259) (character 2)) (end (line 259) (character 396))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Usage") (range (start (line 259) (character 43)) (end (line 259) (character 48)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::OccurrenceUsage::isIndividual"))) (kind "attribute") (name "isIndividual") (declared-name "isIndividual") (range (start (line 260) (character 3)) (end (line 260) (character 42))) (parent (node (document "d0") (qualified-name "SysML::Systems::OccurrenceUsage"))) (authored (membership (kind Feature)) (relationships (typing (reference "Boolean") (range none)))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::OccurrenceUsage::portionKind"))) (kind "attribute") (name "portionKind") (declared-name "portionKind") (range (start (line 261) (character 3)) (end (line 261) (character 45))) (parent (node (document "d0") (qualified-name "SysML::Systems::OccurrenceUsage"))) (authored (membership (kind Feature)) (relationships (typing (reference "PortionKind") (range none)))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::PartDefinition"))) (kind "metadata def") (name "PartDefinition") (declared-name "PartDefinition") (range (start (line 267) (character 2)) (end (line 267) (character 57))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ItemDefinition") (range (start (line 267) (character 42)) (end (line 267) (character 56)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::PartUsage"))) (kind "metadata def") (name "PartUsage") (declared-name "PartUsage") (range (start (line 269) (character 2)) (end (line 269) (character 174))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ItemUsage") (range (start (line 269) (character 37)) (end (line 269) (character 46)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::PerformActionUsage"))) (kind "metadata def") (name "PerformActionUsage") (declared-name "PerformActionUsage") (range (start (line 273) (character 2)) (end (line 273) (character 200))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ActionUsage") (range (start (line 273) (character 46)) (end (line 273) (character 57)))) (specializes (reference "EventOccurrenceUsage") (range (start (line 273) (character 59)) (end (line 273) (character 79)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::PortConjugation"))) (kind "metadata def") (name "PortConjugation") (declared-name "PortConjugation") (range (start (line 277) (character 2)) (end (line 277) (character 306))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Conjugation") (range (start (line 277) (character 43)) (end (line 277) (character 54)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::PortDefinition"))) (kind "metadata def") (name "PortDefinition") (declared-name "PortDefinition") (range (start (line 282) (character 2)) (end (line 282) (character 210))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "OccurrenceDefinition") (range (start (line 282) (character 42)) (end (line 282) (character 62)))) (specializes (reference "Structure") (range (start (line 282) (character 64)) (end (line 282) (character 73)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::PortUsage"))) (kind "metadata def") (name "PortUsage") (declared-name "PortUsage") (range (start (line 286) (character 2)) (end (line 286) (character 188))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "OccurrenceUsage") (range (start (line 286) (character 37)) (end (line 286) (character 52)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::PortionKind"))) (kind "enum def") (name "PortionKind") (declared-name "PortionKind") (range (start (line 290) (character 2)) (end (line 290) (character 69))) (parent (node (document "d0") (qualified-name "SysML::Systems"))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::PortionKind::snapshot"))) (kind "enumerated value") (name "snapshot") (declared-name "snapshot") (range (start (line 292) (character 8)) (end (line 292) (character 18))) (parent (node (document "d0") (qualified-name "SysML::Systems::PortionKind"))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::PortionKind::timeslice"))) (kind "enumerated value") (name "timeslice") (declared-name "timeslice") (range (start (line 291) (character 8)) (end (line 291) (character 19))) (parent (node (document "d0") (qualified-name "SysML::Systems::PortionKind"))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::ReferenceUsage"))) (kind "metadata def") (name "ReferenceUsage") (declared-name "ReferenceUsage") (range (start (line 295) (character 2)) (end (line 295) (character 125))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Usage") (range (start (line 295) (character 42)) (end (line 295) (character 47)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::ReferenceUsage::isReference"))) (kind "attribute") (name "isReference") (declared-name "isReference") (range (start (line 296) (character 3)) (end (line 296) (character 71))) (parent (node (document "d0") (qualified-name "SysML::Systems::ReferenceUsage"))) (authored (membership (kind Feature)) (relationships (typing (reference "Boolean") (range none)) (redefinition (reference "isReference") (range (start (line 296) (character 59)) (end (line 296) (character 70)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::RenderingDefinition"))) (kind "metadata def") (name "RenderingDefinition") (declared-name "RenderingDefinition") (range (start (line 299) (character 2)) (end (line 299) (character 177))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "PartDefinition") (range (start (line 299) (character 47)) (end (line 299) (character 61)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::RenderingUsage"))) (kind "metadata def") (name "RenderingUsage") (declared-name "RenderingUsage") (range (start (line 303) (character 2)) (end (line 303) (character 183))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "PartUsage") (range (start (line 303) (character 42)) (end (line 303) (character 51)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::RequirementConstraintKind"))) (kind "enum def") (name "RequirementConstraintKind") (declared-name "RequirementConstraintKind") (range (start (line 307) (character 2)) (end (line 307) (character 85))) (parent (node (document "d0") (qualified-name "SysML::Systems"))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::RequirementConstraintKind::assumption"))) (kind "enumerated value") (name "assumption") (declared-name "assumption") (range (start (line 308) (character 8)) (end (line 308) (character 18))) (parent (node (document "d0") (qualified-name "SysML::Systems::RequirementConstraintKind"))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::RequirementConstraintKind::requirement"))) (kind "enumerated value") (name "requirement") (declared-name "requirement") (range (start (line 309) (character 8)) (end (line 309) (character 21))) (parent (node (document "d0") (qualified-name "SysML::Systems::RequirementConstraintKind"))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::RequirementConstraintMembership"))) (kind "metadata def") (name "RequirementConstraintMembership") (declared-name "RequirementConstraintMembership") (range (start (line 312) (character 2)) (end (line 312) (character 355))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "FeatureMembership") (range (start (line 312) (character 59)) (end (line 312) (character 76)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::RequirementConstraintMembership::kind"))) (kind "attribute") (name "kind") (declared-name "kind") (range (start (line 313) (character 3)) (end (line 313) (character 52))) (parent (node (document "d0") (qualified-name "SysML::Systems::RequirementConstraintMembership"))) (authored (membership (kind Feature)) (relationships (typing (reference "RequirementConstraintKind") (range none)))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::RequirementDefinition"))) (kind "metadata def") (name "RequirementDefinition") (declared-name "RequirementDefinition") (range (start (line 319) (character 2)) (end (line 319) (character 909))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ConstraintDefinition") (range (start (line 319) (character 49)) (end (line 319) (character 69)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::RequirementDefinition::reqId"))) (kind "attribute") (name "reqId") (declared-name "reqId") (range (start (line 320) (character 3)) (end (line 320) (character 62))) (parent (node (document "d0") (qualified-name "SysML::Systems::RequirementDefinition"))) (authored (membership (kind Feature)) (relationships (typing (reference "String") (range none)) (redefinition (reference "declaredShortName") (range (start (line 320) (character 44)) (end (line 320) (character 61)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::RequirementDefinition::text"))) (kind "attribute") (name "text") (declared-name "text") (range (start (line 321) (character 3)) (end (line 321) (character 41))) (parent (node (document "d0") (qualified-name "SysML::Systems::RequirementDefinition"))) (authored (membership (kind Feature)) (relationships (typing (reference "String") (range none)))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::RequirementUsage"))) (kind "metadata def") (name "RequirementUsage") (declared-name "RequirementUsage") (range (start (line 331) (character 2)) (end (line 331) (character 1035))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ConstraintUsage") (range (start (line 331) (character 44)) (end (line 331) (character 59)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::RequirementUsage::reqId"))) (kind "attribute") (name "reqId") (declared-name "reqId") (range (start (line 332) (character 3)) (end (line 332) (character 62))) (parent (node (document "d0") (qualified-name "SysML::Systems::RequirementUsage"))) (authored (membership (kind Feature)) (relationships (typing (reference "String") (range none)) (redefinition (reference "declaredShortName") (range (start (line 332) (character 44)) (end (line 332) (character 61)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::RequirementUsage::text"))) (kind "attribute") (name "text") (declared-name "text") (range (start (line 333) (character 3)) (end (line 333) (character 41))) (parent (node (document "d0") (qualified-name "SysML::Systems::RequirementUsage"))) (authored (membership (kind Feature)) (relationships (typing (reference "String") (range none)))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::RequirementVerificationMembership"))) (kind "metadata def") (name "RequirementVerificationMembership") (declared-name "RequirementVerificationMembership") (range (start (line 344) (character 2)) (end (line 344) (character 416))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "RequirementConstraintMembership") (range (start (line 344) (character 61)) (end (line 344) (character 92)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::RequirementVerificationMembership::kind"))) (kind "attribute") (name "kind") (declared-name "kind") (range (start (line 345) (character 3)) (end (line 345) (character 67))) (parent (node (document "d0") (qualified-name "SysML::Systems::RequirementVerificationMembership"))) (authored (membership (kind Feature)) (relationships (typing (reference "RequirementConstraintKind") (range none)) (redefinition (reference "kind") (range (start (line 345) (character 62)) (end (line 345) (character 66)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::SatisfyRequirementUsage"))) (kind "metadata def") (name "SatisfyRequirementUsage") (declared-name "SatisfyRequirementUsage") (range (start (line 351) (character 2)) (end (line 351) (character 311))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "RequirementUsage") (range (start (line 351) (character 51)) (end (line 351) (character 67)))) (specializes (reference "AssertConstraintUsage") (range (start (line 351) (character 69)) (end (line 351) (character 90)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::SendActionUsage"))) (kind "metadata def") (name "SendActionUsage") (declared-name "SendActionUsage") (range (start (line 356) (character 2)) (end (line 356) (character 324))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ActionUsage") (range (start (line 356) (character 43)) (end (line 356) (character 54)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::StakeholderMembership"))) (kind "metadata def") (name "StakeholderMembership") (declared-name "StakeholderMembership") (range (start (line 362) (character 2)) (end (line 362) (character 198))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ParameterMembership") (range (start (line 362) (character 49)) (end (line 362) (character 68)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::StateDefinition"))) (kind "metadata def") (name "StateDefinition") (declared-name "StateDefinition") (range (start (line 366) (character 2)) (end (line 366) (character 466))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ActionDefinition") (range (start (line 366) (character 43)) (end (line 366) (character 59)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::StateDefinition::isParallel"))) (kind "attribute") (name "isParallel") (declared-name "isParallel") (range (start (line 367) (character 3)) (end (line 367) (character 40))) (parent (node (document "d0") (qualified-name "SysML::Systems::StateDefinition"))) (authored (membership (kind Feature)) (relationships (typing (reference "Boolean") (range none)))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::StateSubactionKind"))) (kind "enum def") (name "StateSubactionKind") (declared-name "StateSubactionKind") (range (start (line 375) (character 2)) (end (line 375) (character 82))) (parent (node (document "d0") (qualified-name "SysML::Systems"))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::StateSubactionKind::do"))) (kind "enumerated value") (name "do") (declared-name "do") (range (start (line 377) (character 8)) (end (line 377) (character 12))) (parent (node (document "d0") (qualified-name "SysML::Systems::StateSubactionKind"))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::StateSubactionKind::entry"))) (kind "enumerated value") (name "entry") (declared-name "entry") (range (start (line 376) (character 8)) (end (line 376) (character 15))) (parent (node (document "d0") (qualified-name "SysML::Systems::StateSubactionKind"))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::StateSubactionKind::exit"))) (kind "enumerated value") (name "exit") (declared-name "exit") (range (start (line 378) (character 8)) (end (line 378) (character 14))) (parent (node (document "d0") (qualified-name "SysML::Systems::StateSubactionKind"))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::StateSubactionMembership"))) (kind "metadata def") (name "StateSubactionMembership") (declared-name "StateSubactionMembership") (range (start (line 381) (character 2)) (end (line 381) (character 232))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "FeatureMembership") (range (start (line 381) (character 52)) (end (line 381) (character 69)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::StateSubactionMembership::kind"))) (kind "attribute") (name "kind") (declared-name "kind") (range (start (line 382) (character 3)) (end (line 382) (character 45))) (parent (node (document "d0") (qualified-name "SysML::Systems::StateSubactionMembership"))) (authored (membership (kind Feature)) (relationships (typing (reference "StateSubactionKind") (range none)))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::StateUsage"))) (kind "metadata def") (name "StateUsage") (declared-name "StateUsage") (range (start (line 387) (character 2)) (end (line 387) (character 472))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ActionUsage") (range (start (line 387) (character 38)) (end (line 387) (character 49)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::StateUsage::isParallel"))) (kind "attribute") (name "isParallel") (declared-name "isParallel") (range (start (line 388) (character 3)) (end (line 388) (character 40))) (parent (node (document "d0") (qualified-name "SysML::Systems::StateUsage"))) (authored (membership (kind Feature)) (relationships (typing (reference "Boolean") (range none)))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::SubjectMembership"))) (kind "metadata def") (name "SubjectMembership") (declared-name "SubjectMembership") (range (start (line 396) (character 2)) (end (line 396) (character 186))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ParameterMembership") (range (start (line 396) (character 45)) (end (line 396) (character 64)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::SuccessionAsUsage"))) (kind "metadata def") (name "SuccessionAsUsage") (declared-name "SuccessionAsUsage") (range (start (line 400) (character 2)) (end (line 400) (character 74))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ConnectorAsUsage") (range (start (line 400) (character 45)) (end (line 400) (character 61)))) (specializes (reference "Succession") (range (start (line 400) (character 63)) (end (line 400) (character 73)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::SuccessionFlowUsage"))) (kind "metadata def") (name "SuccessionFlowUsage") (declared-name "SuccessionFlowUsage") (range (start (line 402) (character 2)) (end (line 402) (character 73))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SuccessionFlow") (range (start (line 402) (character 47)) (end (line 402) (character 61)))) (specializes (reference "FlowUsage") (range (start (line 402) (character 63)) (end (line 402) (character 72)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::TerminateActionUsage"))) (kind "metadata def") (name "TerminateActionUsage") (declared-name "TerminateActionUsage") (range (start (line 404) (character 2)) (end (line 404) (character 166))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ActionUsage") (range (start (line 404) (character 48)) (end (line 404) (character 59)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::TransitionFeatureKind"))) (kind "enum def") (name "TransitionFeatureKind") (declared-name "TransitionFeatureKind") (range (start (line 408) (character 2)) (end (line 408) (character 86))) (parent (node (document "d0") (qualified-name "SysML::Systems"))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::TransitionFeatureKind::effect"))) (kind "enumerated value") (name "effect") (declared-name "effect") (range (start (line 411) (character 8)) (end (line 411) (character 14))) (parent (node (document "d0") (qualified-name "SysML::Systems::TransitionFeatureKind"))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::TransitionFeatureKind::guard"))) (kind "enumerated value") (name "guard") (declared-name "guard") (range (start (line 410) (character 8)) (end (line 410) (character 13))) (parent (node (document "d0") (qualified-name "SysML::Systems::TransitionFeatureKind"))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::TransitionFeatureKind::trigger"))) (kind "enumerated value") (name "trigger") (declared-name "trigger") (range (start (line 409) (character 8)) (end (line 409) (character 15))) (parent (node (document "d0") (qualified-name "SysML::Systems::TransitionFeatureKind"))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::TransitionFeatureMembership"))) (kind "metadata def") (name "TransitionFeatureMembership") (declared-name "TransitionFeatureMembership") (range (start (line 414) (character 2)) (end (line 414) (character 240))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "FeatureMembership") (range (start (line 414) (character 55)) (end (line 414) (character 72)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::TransitionFeatureMembership::kind"))) (kind "attribute") (name "kind") (declared-name "kind") (range (start (line 415) (character 3)) (end (line 415) (character 48))) (parent (node (document "d0") (qualified-name "SysML::Systems::TransitionFeatureMembership"))) (authored (membership (kind Feature)) (relationships (typing (reference "TransitionFeatureKind") (range none)))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::TransitionUsage"))) (kind "metadata def") (name "TransitionUsage") (declared-name "TransitionUsage") (range (start (line 420) (character 2)) (end (line 420) (character 650))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ActionUsage") (range (start (line 420) (character 43)) (end (line 420) (character 54)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::TriggerInvocationExpression"))) (kind "metadata def") (name "TriggerInvocationExpression") (declared-name "TriggerInvocationExpression") (range (start (line 429) (character 2)) (end (line 429) (character 120))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "InvocationExpression") (range (start (line 429) (character 55)) (end (line 429) (character 75)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::TriggerInvocationExpression::kind"))) (kind "attribute") (name "kind") (declared-name "kind") (range (start (line 430) (character 3)) (end (line 430) (character 38))) (parent (node (document "d0") (qualified-name "SysML::Systems::TriggerInvocationExpression"))) (authored (membership (kind Feature)) (relationships (typing (reference "TriggerKind") (range none)))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::TriggerKind"))) (kind "enum def") (name "TriggerKind") (declared-name "TriggerKind") (range (start (line 433) (character 2)) (end (line 433) (character 75))) (parent (node (document "d0") (qualified-name "SysML::Systems"))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::TriggerKind::after"))) (kind "enumerated value") (name "after") (declared-name "after") (range (start (line 436) (character 8)) (end (line 436) (character 15))) (parent (node (document "d0") (qualified-name "SysML::Systems::TriggerKind"))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::TriggerKind::at"))) (kind "enumerated value") (name "at") (declared-name "at") (range (start (line 435) (character 8)) (end (line 435) (character 12))) (parent (node (document "d0") (qualified-name "SysML::Systems::TriggerKind"))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::TriggerKind::when"))) (kind "enumerated value") (name "when") (declared-name "when") (range (start (line 434) (character 8)) (end (line 434) (character 14))) (parent (node (document "d0") (qualified-name "SysML::Systems::TriggerKind"))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::Usage"))) (kind "metadata def") (name "Usage") (declared-name "Usage") (range (start (line 439) (character 2)) (end (line 439) (character 4178))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Feature") (range (start (line 439) (character 33)) (end (line 439) (character 40)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::Usage::isReference"))) (kind "attribute") (name "isReference") (declared-name "isReference") (range (start (line 442) (character 3)) (end (line 442) (character 49))) (parent (node (document "d0") (qualified-name "SysML::Systems::Usage"))) (authored (membership (kind Feature)) (relationships (typing (reference "Boolean") (range none)))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::Usage::isVariation"))) (kind "attribute") (name "isVariation") (declared-name "isVariation") (range (start (line 440) (character 3)) (end (line 440) (character 41))) (parent (node (document "d0") (qualified-name "SysML::Systems::Usage"))) (authored (membership (kind Feature)) (relationships (typing (reference "Boolean") (range none)))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::Usage::mayTimeVary"))) (kind "attribute") (name "mayTimeVary") (declared-name "mayTimeVary") (range (start (line 441) (character 3)) (end (line 441) (character 70))) (parent (node (document "d0") (qualified-name "SysML::Systems::Usage"))) (authored (membership (kind Feature)) (relationships (typing (reference "Boolean") (range none)) (redefinition (reference "isVariable") (range (start (line 441) (character 59)) (end (line 441) (character 69)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::UseCaseDefinition"))) (kind "metadata def") (name "UseCaseDefinition") (declared-name "UseCaseDefinition") (range (start (line 480) (character 2)) (end (line 480) (character 163))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "CaseDefinition") (range (start (line 480) (character 45)) (end (line 480) (character 59)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::UseCaseUsage"))) (kind "metadata def") (name "UseCaseUsage") (declared-name "UseCaseUsage") (range (start (line 484) (character 2)) (end (line 484) (character 275))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "CaseUsage") (range (start (line 484) (character 40)) (end (line 484) (character 49)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::VariantMembership"))) (kind "metadata def") (name "VariantMembership") (declared-name "VariantMembership") (range (start (line 489) (character 2)) (end (line 489) (character 177))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "OwningMembership") (range (start (line 489) (character 45)) (end (line 489) (character 61)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::VerificationCaseDefinition"))) (kind "metadata def") (name "VerificationCaseDefinition") (declared-name "VerificationCaseDefinition") (range (start (line 493) (character 2)) (end (line 493) (character 180))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "CaseDefinition") (range (start (line 493) (character 54)) (end (line 493) (character 68)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::VerificationCaseUsage"))) (kind "metadata def") (name "VerificationCaseUsage") (declared-name "VerificationCaseUsage") (range (start (line 497) (character 2)) (end (line 497) (character 308))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "CaseUsage") (range (start (line 497) (character 49)) (end (line 497) (character 58)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::ViewDefinition"))) (kind "metadata def") (name "ViewDefinition") (declared-name "ViewDefinition") (range (start (line 502) (character 2)) (end (line 502) (character 494))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "PartDefinition") (range (start (line 502) (character 42)) (end (line 502) (character 56)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::ViewRenderingMembership"))) (kind "metadata def") (name "ViewRenderingMembership") (declared-name "ViewRenderingMembership") (range (start (line 509) (character 2)) (end (line 509) (character 286))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "FeatureMembership") (range (start (line 509) (character 51)) (end (line 509) (character 68)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::ViewUsage"))) (kind "metadata def") (name "ViewUsage") (declared-name "ViewUsage") (range (start (line 514) (character 2)) (end (line 514) (character 608))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "PartUsage") (range (start (line 514) (character 37)) (end (line 514) (character 46)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::ViewpointDefinition"))) (kind "metadata def") (name "ViewpointDefinition") (declared-name "ViewpointDefinition") (range (start (line 522) (character 2)) (end (line 522) (character 174))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "RequirementDefinition") (range (start (line 522) (character 47)) (end (line 522) (character 68)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::ViewpointUsage"))) (kind "metadata def") (name "ViewpointUsage") (declared-name "ViewpointUsage") (range (start (line 526) (character 2)) (end (line 526) (character 297))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "RequirementUsage") (range (start (line 526) (character 42)) (end (line 526) (character 58)))))))
    (element (id (node (document "d0") (qualified-name "SysML::Systems::WhileLoopActionUsage"))) (kind "metadata def") (name "WhileLoopActionUsage") (declared-name "WhileLoopActionUsage") (range (start (line 531) (character 2)) (end (line 531) (character 241))) (parent (node (document "d0") (qualified-name "SysML::Systems"))) (authored (membership (kind Owning)) (relationships (specializes (reference "LoopActionUsage") (range (start (line 531) (character 48)) (end (line 531) (character 63)))))))
    (element (id (node (document "d0") (qualified-name "SysML::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 30788))) (parent (node (document "d0") (qualified-name "SysML"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "SysML::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 6) (character 16)) (end (line 6) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Systems::*") (range (start (line 7) (character 15)) (end (line 7) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "KerML::Kernel::*") (range (start (line 10) (character 16)) (end (line 10) (character 29))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::AcceptActionUsage"))) (kind specialization) (ordinal 0)) (authored-target "ActionUsage") (range (start (line 12) (character 45)) (end (line 12) (character 56))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::ActionUsage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::ActionDefinition"))) (kind specialization) (ordinal 0)) (authored-target "Behavior") (range (start (line 18) (character 44)) (end (line 18) (character 52))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::ActionDefinition"))) (kind specialization) (ordinal 1)) (authored-target "OccurrenceDefinition") (range (start (line 18) (character 54)) (end (line 18) (character 74))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::OccurrenceDefinition")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::ActionUsage"))) (kind specialization) (ordinal 0)) (authored-target "Step") (range (start (line 22) (character 39)) (end (line 22) (character 43))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::ActionUsage"))) (kind specialization) (ordinal 1)) (authored-target "OccurrenceUsage") (range (start (line 22) (character 45)) (end (line 22) (character 60))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::OccurrenceUsage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::ActorMembership"))) (kind specialization) (ordinal 0)) (authored-target "ParameterMembership") (range (start (line 26) (character 43)) (end (line 26) (character 62))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::AllocationDefinition"))) (kind specialization) (ordinal 0)) (authored-target "ConnectionDefinition") (range (start (line 30) (character 48)) (end (line 30) (character 68))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::ConnectionDefinition")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::AllocationUsage"))) (kind specialization) (ordinal 0)) (authored-target "ConnectionUsage") (range (start (line 34) (character 43)) (end (line 34) (character 58))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::ConnectionUsage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::AnalysisCaseDefinition"))) (kind specialization) (ordinal 0)) (authored-target "CaseDefinition") (range (start (line 38) (character 50)) (end (line 38) (character 64))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::CaseDefinition")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::AnalysisCaseUsage"))) (kind specialization) (ordinal 0)) (authored-target "CaseUsage") (range (start (line 42) (character 45)) (end (line 42) (character 54))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::CaseUsage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::AssertConstraintUsage"))) (kind specialization) (ordinal 0)) (authored-target "ConstraintUsage") (range (start (line 47) (character 49)) (end (line 47) (character 64))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::ConstraintUsage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::AssertConstraintUsage"))) (kind specialization) (ordinal 1)) (authored-target "Invariant") (range (start (line 47) (character 66)) (end (line 47) (character 75))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::AssignmentActionUsage"))) (kind specialization) (ordinal 0)) (authored-target "ActionUsage") (range (start (line 51) (character 49)) (end (line 51) (character 60))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::ActionUsage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::AttributeDefinition"))) (kind specialization) (ordinal 0)) (authored-target "DataType") (range (start (line 57) (character 47)) (end (line 57) (character 55))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::AttributeDefinition"))) (kind specialization) (ordinal 1)) (authored-target "Definition") (range (start (line 57) (character 57)) (end (line 57) (character 67))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::Definition")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::AttributeUsage"))) (kind specialization) (ordinal 0)) (authored-target "Usage") (range (start (line 59) (character 42)) (end (line 59) (character 47))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::Usage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::AttributeUsage::isReference"))) (kind featureTyping) (ordinal 0)) (authored-target "Boolean") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::AttributeUsage::isReference"))) (kind redefinition) (ordinal 0)) (authored-target "isReference") (range (start (line 60) (character 59)) (end (line 60) (character 70))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::AttributeUsage::isReference")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::BindingConnectorAsUsage"))) (kind specialization) (ordinal 0)) (authored-target "BindingConnector") (range (start (line 65) (character 51)) (end (line 65) (character 67))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::BindingConnectorAsUsage"))) (kind specialization) (ordinal 1)) (authored-target "ConnectorAsUsage") (range (start (line 65) (character 69)) (end (line 65) (character 85))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::ConnectorAsUsage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::CalculationDefinition"))) (kind specialization) (ordinal 0)) (authored-target "Function") (range (start (line 67) (character 49)) (end (line 67) (character 57))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::CalculationDefinition"))) (kind specialization) (ordinal 1)) (authored-target "ActionDefinition") (range (start (line 67) (character 59)) (end (line 67) (character 75))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::ActionDefinition")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::CalculationUsage"))) (kind specialization) (ordinal 0)) (authored-target "Expression") (range (start (line 71) (character 44)) (end (line 71) (character 54))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::CalculationUsage"))) (kind specialization) (ordinal 1)) (authored-target "ActionUsage") (range (start (line 71) (character 56)) (end (line 71) (character 67))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::ActionUsage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::CaseDefinition"))) (kind specialization) (ordinal 0)) (authored-target "CalculationDefinition") (range (start (line 75) (character 42)) (end (line 75) (character 63))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::CalculationDefinition")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::CaseUsage"))) (kind specialization) (ordinal 0)) (authored-target "CalculationUsage") (range (start (line 81) (character 37)) (end (line 81) (character 53))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::CalculationUsage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::ConcernDefinition"))) (kind specialization) (ordinal 0)) (authored-target "RequirementDefinition") (range (start (line 88) (character 45)) (end (line 88) (character 66))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::RequirementDefinition")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::ConcernUsage"))) (kind specialization) (ordinal 0)) (authored-target "RequirementUsage") (range (start (line 90) (character 40)) (end (line 90) (character 56))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::RequirementUsage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::ConjugatedPortDefinition"))) (kind specialization) (ordinal 0)) (authored-target "PortDefinition") (range (start (line 94) (character 52)) (end (line 94) (character 66))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::PortDefinition")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::ConjugatedPortTyping"))) (kind specialization) (ordinal 0)) (authored-target "FeatureTyping") (range (start (line 99) (character 48)) (end (line 99) (character 61))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::ConnectionDefinition"))) (kind specialization) (ordinal 0)) (authored-target "AssociationStructure") (range (start (line 104) (character 48)) (end (line 104) (character 68))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::ConnectionDefinition"))) (kind specialization) (ordinal 1)) (authored-target "PartDefinition") (range (start (line 104) (character 70)) (end (line 104) (character 84))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::PartDefinition")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::ConnectionDefinition::isSufficient"))) (kind featureTyping) (ordinal 0)) (authored-target "Boolean") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::ConnectionDefinition::isSufficient"))) (kind redefinition) (ordinal 0)) (authored-target "isSufficient") (range (start (line 105) (character 52)) (end (line 105) (character 64))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::ConnectionDefinition::isSufficient")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::ConnectionUsage"))) (kind specialization) (ordinal 0)) (authored-target "ConnectorAsUsage") (range (start (line 110) (character 43)) (end (line 110) (character 59))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::ConnectorAsUsage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::ConnectionUsage"))) (kind specialization) (ordinal 1)) (authored-target "PartUsage") (range (start (line 110) (character 61)) (end (line 110) (character 70))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::PartUsage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::ConnectorAsUsage"))) (kind specialization) (ordinal 0)) (authored-target "Usage") (range (start (line 114) (character 53)) (end (line 114) (character 58))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::Usage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::ConnectorAsUsage"))) (kind specialization) (ordinal 1)) (authored-target "Connector") (range (start (line 114) (character 60)) (end (line 114) (character 69))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::ConstraintDefinition"))) (kind specialization) (ordinal 0)) (authored-target "OccurrenceDefinition") (range (start (line 116) (character 48)) (end (line 116) (character 68))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::OccurrenceDefinition")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::ConstraintDefinition"))) (kind specialization) (ordinal 1)) (authored-target "Predicate") (range (start (line 116) (character 70)) (end (line 116) (character 79))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::ConstraintUsage"))) (kind specialization) (ordinal 0)) (authored-target "BooleanExpression") (range (start (line 118) (character 43)) (end (line 118) (character 60))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::ConstraintUsage"))) (kind specialization) (ordinal 1)) (authored-target "OccurrenceUsage") (range (start (line 118) (character 62)) (end (line 118) (character 77))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::OccurrenceUsage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::ControlNode"))) (kind specialization) (ordinal 0)) (authored-target "ActionUsage") (range (start (line 122) (character 48)) (end (line 122) (character 59))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::ActionUsage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::DecisionNode"))) (kind specialization) (ordinal 0)) (authored-target "ControlNode") (range (start (line 124) (character 40)) (end (line 124) (character 51))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::ControlNode")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::Definition"))) (kind specialization) (ordinal 0)) (authored-target "Classifier") (range (start (line 126) (character 38)) (end (line 126) (character 48))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::Definition::isVariation"))) (kind featureTyping) (ordinal 0)) (authored-target "Boolean") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::EnumerationDefinition"))) (kind specialization) (ordinal 0)) (authored-target "AttributeDefinition") (range (start (line 162) (character 49)) (end (line 162) (character 68))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::AttributeDefinition")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::EnumerationDefinition::isVariation"))) (kind featureTyping) (ordinal 0)) (authored-target "Boolean") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::EnumerationDefinition::isVariation"))) (kind redefinition) (ordinal 0)) (authored-target "isVariation") (range (start (line 163) (character 51)) (end (line 163) (character 62))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::EnumerationDefinition::isVariation")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::EnumerationUsage"))) (kind specialization) (ordinal 0)) (authored-target "AttributeUsage") (range (start (line 168) (character 44)) (end (line 168) (character 58))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::AttributeUsage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::EventOccurrenceUsage"))) (kind specialization) (ordinal 0)) (authored-target "OccurrenceUsage") (range (start (line 172) (character 48)) (end (line 172) (character 63))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::OccurrenceUsage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::EventOccurrenceUsage::isReference"))) (kind featureTyping) (ordinal 0)) (authored-target "Boolean") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::EventOccurrenceUsage::isReference"))) (kind redefinition) (ordinal 0)) (authored-target "isReference") (range (start (line 173) (character 59)) (end (line 173) (character 70))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::EventOccurrenceUsage::isReference")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::ExhibitStateUsage"))) (kind specialization) (ordinal 0)) (authored-target "StateUsage") (range (start (line 178) (character 45)) (end (line 178) (character 55))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::StateUsage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::ExhibitStateUsage"))) (kind specialization) (ordinal 1)) (authored-target "PerformActionUsage") (range (start (line 178) (character 57)) (end (line 178) (character 75))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::PerformActionUsage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::Expose"))) (kind specialization) (ordinal 0)) (authored-target "Import") (range (start (line 182) (character 43)) (end (line 182) (character 49))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::Expose::isImportAll"))) (kind featureTyping) (ordinal 0)) (authored-target "Boolean") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::Expose::isImportAll"))) (kind redefinition) (ordinal 0)) (authored-target "isImportAll") (range (start (line 184) (character 51)) (end (line 184) (character 62))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::Expose::isImportAll")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::Expose::visibility"))) (kind featureTyping) (ordinal 0)) (authored-target "VisibilityKind") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::Expose::visibility"))) (kind redefinition) (ordinal 0)) (authored-target "visibility") (range (start (line 183) (character 57)) (end (line 183) (character 67))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::Expose::visibility")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::FlowDefinition"))) (kind specialization) (ordinal 0)) (authored-target "Interaction") (range (start (line 187) (character 42)) (end (line 187) (character 53))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::FlowDefinition"))) (kind specialization) (ordinal 1)) (authored-target "ActionDefinition") (range (start (line 187) (character 55)) (end (line 187) (character 71))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::ActionDefinition")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::FlowUsage"))) (kind specialization) (ordinal 0)) (authored-target "ConnectorAsUsage") (range (start (line 191) (character 37)) (end (line 191) (character 53))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::ConnectorAsUsage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::FlowUsage"))) (kind specialization) (ordinal 1)) (authored-target "Flow") (range (start (line 191) (character 55)) (end (line 191) (character 59))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::FlowUsage"))) (kind specialization) (ordinal 2)) (authored-target "ActionUsage") (range (start (line 191) (character 61)) (end (line 191) (character 72))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::ActionUsage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::ForLoopActionUsage"))) (kind specialization) (ordinal 0)) (authored-target "LoopActionUsage") (range (start (line 195) (character 46)) (end (line 195) (character 61))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::LoopActionUsage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::ForkNode"))) (kind specialization) (ordinal 0)) (authored-target "ControlNode") (range (start (line 200) (character 36)) (end (line 200) (character 47))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::ControlNode")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::FramedConcernMembership"))) (kind specialization) (ordinal 0)) (authored-target "RequirementConstraintMembership") (range (start (line 202) (character 51)) (end (line 202) (character 82))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::RequirementConstraintMembership")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::FramedConcernMembership::kind"))) (kind featureTyping) (ordinal 0)) (authored-target "RequirementConstraintKind") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::RequirementConstraintKind")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::FramedConcernMembership::kind"))) (kind redefinition) (ordinal 0)) (authored-target "kind") (range (start (line 203) (character 62)) (end (line 203) (character 66))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::FramedConcernMembership::kind")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::IfActionUsage"))) (kind specialization) (ordinal 0)) (authored-target "ActionUsage") (range (start (line 209) (character 41)) (end (line 209) (character 52))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::ActionUsage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::IncludeUseCaseUsage"))) (kind specialization) (ordinal 0)) (authored-target "UseCaseUsage") (range (start (line 215) (character 47)) (end (line 215) (character 59))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::UseCaseUsage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::IncludeUseCaseUsage"))) (kind specialization) (ordinal 1)) (authored-target "PerformActionUsage") (range (start (line 215) (character 61)) (end (line 215) (character 79))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::PerformActionUsage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::InterfaceDefinition"))) (kind specialization) (ordinal 0)) (authored-target "ConnectionDefinition") (range (start (line 219) (character 47)) (end (line 219) (character 67))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::ConnectionDefinition")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::InterfaceUsage"))) (kind specialization) (ordinal 0)) (authored-target "ConnectionUsage") (range (start (line 223) (character 42)) (end (line 223) (character 57))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::ConnectionUsage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::ItemDefinition"))) (kind specialization) (ordinal 0)) (authored-target "Structure") (range (start (line 227) (character 42)) (end (line 227) (character 51))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::ItemDefinition"))) (kind specialization) (ordinal 1)) (authored-target "OccurrenceDefinition") (range (start (line 227) (character 53)) (end (line 227) (character 73))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::OccurrenceDefinition")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::ItemUsage"))) (kind specialization) (ordinal 0)) (authored-target "OccurrenceUsage") (range (start (line 229) (character 37)) (end (line 229) (character 52))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::OccurrenceUsage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::JoinNode"))) (kind specialization) (ordinal 0)) (authored-target "ControlNode") (range (start (line 233) (character 36)) (end (line 233) (character 47))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::ControlNode")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::LoopActionUsage"))) (kind specialization) (ordinal 0)) (authored-target "ActionUsage") (range (start (line 235) (character 52)) (end (line 235) (character 63))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::ActionUsage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::MembershipExpose"))) (kind specialization) (ordinal 0)) (authored-target "MembershipImport") (range (start (line 239) (character 44)) (end (line 239) (character 60))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::MembershipExpose"))) (kind specialization) (ordinal 1)) (authored-target "Expose") (range (start (line 239) (character 62)) (end (line 239) (character 68))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::Expose")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::MergeNode"))) (kind specialization) (ordinal 0)) (authored-target "ControlNode") (range (start (line 241) (character 37)) (end (line 241) (character 48))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::ControlNode")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::MetadataDefinition"))) (kind specialization) (ordinal 0)) (authored-target "ItemDefinition") (range (start (line 243) (character 46)) (end (line 243) (character 60))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::ItemDefinition")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::MetadataDefinition"))) (kind specialization) (ordinal 1)) (authored-target "Metaclass") (range (start (line 243) (character 62)) (end (line 243) (character 71))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::MetadataUsage"))) (kind specialization) (ordinal 0)) (authored-target "ItemUsage") (range (start (line 245) (character 41)) (end (line 245) (character 50))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::ItemUsage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::MetadataUsage"))) (kind specialization) (ordinal 1)) (authored-target "MetadataFeature") (range (start (line 245) (character 52)) (end (line 245) (character 67))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::NamespaceExpose"))) (kind specialization) (ordinal 0)) (authored-target "Expose") (range (start (line 249) (character 43)) (end (line 249) (character 49))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::Expose")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::NamespaceExpose"))) (kind specialization) (ordinal 1)) (authored-target "NamespaceImport") (range (start (line 249) (character 51)) (end (line 249) (character 66))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::ObjectiveMembership"))) (kind specialization) (ordinal 0)) (authored-target "FeatureMembership") (range (start (line 251) (character 47)) (end (line 251) (character 64))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::OccurrenceDefinition"))) (kind specialization) (ordinal 0)) (authored-target "Definition") (range (start (line 255) (character 48)) (end (line 255) (character 58))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::Definition")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::OccurrenceDefinition"))) (kind specialization) (ordinal 1)) (authored-target "Class") (range (start (line 255) (character 60)) (end (line 255) (character 65))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::OccurrenceDefinition::isIndividual"))) (kind featureTyping) (ordinal 0)) (authored-target "Boolean") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::OccurrenceUsage"))) (kind specialization) (ordinal 0)) (authored-target "Usage") (range (start (line 259) (character 43)) (end (line 259) (character 48))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::Usage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::OccurrenceUsage::isIndividual"))) (kind featureTyping) (ordinal 0)) (authored-target "Boolean") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::OccurrenceUsage::portionKind"))) (kind featureTyping) (ordinal 0)) (authored-target "PortionKind") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::PortionKind")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::PartDefinition"))) (kind specialization) (ordinal 0)) (authored-target "ItemDefinition") (range (start (line 267) (character 42)) (end (line 267) (character 56))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::ItemDefinition")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::PartUsage"))) (kind specialization) (ordinal 0)) (authored-target "ItemUsage") (range (start (line 269) (character 37)) (end (line 269) (character 46))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::ItemUsage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::PerformActionUsage"))) (kind specialization) (ordinal 0)) (authored-target "ActionUsage") (range (start (line 273) (character 46)) (end (line 273) (character 57))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::ActionUsage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::PerformActionUsage"))) (kind specialization) (ordinal 1)) (authored-target "EventOccurrenceUsage") (range (start (line 273) (character 59)) (end (line 273) (character 79))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::EventOccurrenceUsage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::PortConjugation"))) (kind specialization) (ordinal 0)) (authored-target "Conjugation") (range (start (line 277) (character 43)) (end (line 277) (character 54))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::PortDefinition"))) (kind specialization) (ordinal 0)) (authored-target "OccurrenceDefinition") (range (start (line 282) (character 42)) (end (line 282) (character 62))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::OccurrenceDefinition")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::PortDefinition"))) (kind specialization) (ordinal 1)) (authored-target "Structure") (range (start (line 282) (character 64)) (end (line 282) (character 73))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::PortUsage"))) (kind specialization) (ordinal 0)) (authored-target "OccurrenceUsage") (range (start (line 286) (character 37)) (end (line 286) (character 52))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::OccurrenceUsage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::ReferenceUsage"))) (kind specialization) (ordinal 0)) (authored-target "Usage") (range (start (line 295) (character 42)) (end (line 295) (character 47))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::Usage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::ReferenceUsage::isReference"))) (kind featureTyping) (ordinal 0)) (authored-target "Boolean") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::ReferenceUsage::isReference"))) (kind redefinition) (ordinal 0)) (authored-target "isReference") (range (start (line 296) (character 59)) (end (line 296) (character 70))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::ReferenceUsage::isReference")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::RenderingDefinition"))) (kind specialization) (ordinal 0)) (authored-target "PartDefinition") (range (start (line 299) (character 47)) (end (line 299) (character 61))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::PartDefinition")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::RenderingUsage"))) (kind specialization) (ordinal 0)) (authored-target "PartUsage") (range (start (line 303) (character 42)) (end (line 303) (character 51))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::PartUsage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::RequirementConstraintMembership"))) (kind specialization) (ordinal 0)) (authored-target "FeatureMembership") (range (start (line 312) (character 59)) (end (line 312) (character 76))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::RequirementConstraintMembership::kind"))) (kind featureTyping) (ordinal 0)) (authored-target "RequirementConstraintKind") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::RequirementDefinition"))) (kind specialization) (ordinal 0)) (authored-target "ConstraintDefinition") (range (start (line 319) (character 49)) (end (line 319) (character 69))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::ConstraintDefinition")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::RequirementDefinition::reqId"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::RequirementDefinition::reqId"))) (kind redefinition) (ordinal 0)) (authored-target "declaredShortName") (range (start (line 320) (character 44)) (end (line 320) (character 61))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::RequirementDefinition::text"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::RequirementUsage"))) (kind specialization) (ordinal 0)) (authored-target "ConstraintUsage") (range (start (line 331) (character 44)) (end (line 331) (character 59))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::ConstraintUsage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::RequirementUsage::reqId"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::RequirementUsage::reqId"))) (kind redefinition) (ordinal 0)) (authored-target "declaredShortName") (range (start (line 332) (character 44)) (end (line 332) (character 61))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::RequirementUsage::text"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::RequirementVerificationMembership"))) (kind specialization) (ordinal 0)) (authored-target "RequirementConstraintMembership") (range (start (line 344) (character 61)) (end (line 344) (character 92))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::RequirementConstraintMembership")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::RequirementVerificationMembership::kind"))) (kind featureTyping) (ordinal 0)) (authored-target "RequirementConstraintKind") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::RequirementConstraintKind")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::RequirementVerificationMembership::kind"))) (kind redefinition) (ordinal 0)) (authored-target "kind") (range (start (line 345) (character 62)) (end (line 345) (character 66))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::RequirementVerificationMembership::kind")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::SatisfyRequirementUsage"))) (kind specialization) (ordinal 0)) (authored-target "RequirementUsage") (range (start (line 351) (character 51)) (end (line 351) (character 67))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::RequirementUsage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::SatisfyRequirementUsage"))) (kind specialization) (ordinal 1)) (authored-target "AssertConstraintUsage") (range (start (line 351) (character 69)) (end (line 351) (character 90))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::AssertConstraintUsage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::SendActionUsage"))) (kind specialization) (ordinal 0)) (authored-target "ActionUsage") (range (start (line 356) (character 43)) (end (line 356) (character 54))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::ActionUsage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::StakeholderMembership"))) (kind specialization) (ordinal 0)) (authored-target "ParameterMembership") (range (start (line 362) (character 49)) (end (line 362) (character 68))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::StateDefinition"))) (kind specialization) (ordinal 0)) (authored-target "ActionDefinition") (range (start (line 366) (character 43)) (end (line 366) (character 59))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::ActionDefinition")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::StateDefinition::isParallel"))) (kind featureTyping) (ordinal 0)) (authored-target "Boolean") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::StateSubactionMembership"))) (kind specialization) (ordinal 0)) (authored-target "FeatureMembership") (range (start (line 381) (character 52)) (end (line 381) (character 69))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::StateSubactionMembership::kind"))) (kind featureTyping) (ordinal 0)) (authored-target "StateSubactionKind") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::StateUsage"))) (kind specialization) (ordinal 0)) (authored-target "ActionUsage") (range (start (line 387) (character 38)) (end (line 387) (character 49))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::ActionUsage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::StateUsage::isParallel"))) (kind featureTyping) (ordinal 0)) (authored-target "Boolean") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::SubjectMembership"))) (kind specialization) (ordinal 0)) (authored-target "ParameterMembership") (range (start (line 396) (character 45)) (end (line 396) (character 64))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::SuccessionAsUsage"))) (kind specialization) (ordinal 0)) (authored-target "ConnectorAsUsage") (range (start (line 400) (character 45)) (end (line 400) (character 61))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::ConnectorAsUsage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::SuccessionAsUsage"))) (kind specialization) (ordinal 1)) (authored-target "Succession") (range (start (line 400) (character 63)) (end (line 400) (character 73))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::SuccessionFlowUsage"))) (kind specialization) (ordinal 0)) (authored-target "SuccessionFlow") (range (start (line 402) (character 47)) (end (line 402) (character 61))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::SuccessionFlowUsage"))) (kind specialization) (ordinal 1)) (authored-target "FlowUsage") (range (start (line 402) (character 63)) (end (line 402) (character 72))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::FlowUsage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::TerminateActionUsage"))) (kind specialization) (ordinal 0)) (authored-target "ActionUsage") (range (start (line 404) (character 48)) (end (line 404) (character 59))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::ActionUsage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::TransitionFeatureMembership"))) (kind specialization) (ordinal 0)) (authored-target "FeatureMembership") (range (start (line 414) (character 55)) (end (line 414) (character 72))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::TransitionFeatureMembership::kind"))) (kind featureTyping) (ordinal 0)) (authored-target "TransitionFeatureKind") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::TransitionUsage"))) (kind specialization) (ordinal 0)) (authored-target "ActionUsage") (range (start (line 420) (character 43)) (end (line 420) (character 54))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::ActionUsage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::TriggerInvocationExpression"))) (kind specialization) (ordinal 0)) (authored-target "InvocationExpression") (range (start (line 429) (character 55)) (end (line 429) (character 75))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::TriggerInvocationExpression::kind"))) (kind featureTyping) (ordinal 0)) (authored-target "TriggerKind") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::Usage"))) (kind specialization) (ordinal 0)) (authored-target "Feature") (range (start (line 439) (character 33)) (end (line 439) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::Usage::isReference"))) (kind featureTyping) (ordinal 0)) (authored-target "Boolean") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::Usage::isVariation"))) (kind featureTyping) (ordinal 0)) (authored-target "Boolean") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::Usage::mayTimeVary"))) (kind featureTyping) (ordinal 0)) (authored-target "Boolean") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::Usage::mayTimeVary"))) (kind redefinition) (ordinal 0)) (authored-target "isVariable") (range (start (line 441) (character 59)) (end (line 441) (character 69))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::UseCaseDefinition"))) (kind specialization) (ordinal 0)) (authored-target "CaseDefinition") (range (start (line 480) (character 45)) (end (line 480) (character 59))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::CaseDefinition")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::UseCaseUsage"))) (kind specialization) (ordinal 0)) (authored-target "CaseUsage") (range (start (line 484) (character 40)) (end (line 484) (character 49))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::CaseUsage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::VariantMembership"))) (kind specialization) (ordinal 0)) (authored-target "OwningMembership") (range (start (line 489) (character 45)) (end (line 489) (character 61))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::VerificationCaseDefinition"))) (kind specialization) (ordinal 0)) (authored-target "CaseDefinition") (range (start (line 493) (character 54)) (end (line 493) (character 68))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::CaseDefinition")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::VerificationCaseUsage"))) (kind specialization) (ordinal 0)) (authored-target "CaseUsage") (range (start (line 497) (character 49)) (end (line 497) (character 58))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::CaseUsage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::ViewDefinition"))) (kind specialization) (ordinal 0)) (authored-target "PartDefinition") (range (start (line 502) (character 42)) (end (line 502) (character 56))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::PartDefinition")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::ViewRenderingMembership"))) (kind specialization) (ordinal 0)) (authored-target "FeatureMembership") (range (start (line 509) (character 51)) (end (line 509) (character 68))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::ViewUsage"))) (kind specialization) (ordinal 0)) (authored-target "PartUsage") (range (start (line 514) (character 37)) (end (line 514) (character 46))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::PartUsage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::ViewpointDefinition"))) (kind specialization) (ordinal 0)) (authored-target "RequirementDefinition") (range (start (line 522) (character 47)) (end (line 522) (character 68))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::RequirementDefinition")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::ViewpointUsage"))) (kind specialization) (ordinal 0)) (authored-target "RequirementUsage") (range (start (line 526) (character 42)) (end (line 526) (character 58))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::RequirementUsage")))))
    (reference (id (source (node (document "d0") (qualified-name "SysML::Systems::WhileLoopActionUsage"))) (kind specialization) (ordinal 0)) (authored-target "LoopActionUsage") (range (start (line 531) (character 48)) (end (line 531) (character 63))) (outcome (status resolved) (target (node (document "d0") (qualified-name "SysML::Systems::LoopActionUsage")))))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::AcceptActionUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::ActionUsage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::AcceptActionUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::ActionDefinition"))) (target (node (document "d0") (qualified-name "SysML::Systems::OccurrenceDefinition"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::ActionDefinition"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::ActionUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::OccurrenceUsage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::ActionUsage"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::AllocationDefinition"))) (target (node (document "d0") (qualified-name "SysML::Systems::ConnectionDefinition"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::AllocationDefinition"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::AllocationUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::ConnectionUsage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::AllocationUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::AnalysisCaseDefinition"))) (target (node (document "d0") (qualified-name "SysML::Systems::CaseDefinition"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::AnalysisCaseDefinition"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::AnalysisCaseUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::CaseUsage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::AnalysisCaseUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::AssertConstraintUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::ConstraintUsage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::AssertConstraintUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::AssignmentActionUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::ActionUsage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::AssignmentActionUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::AttributeDefinition"))) (target (node (document "d0") (qualified-name "SysML::Systems::Definition"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::AttributeDefinition"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::AttributeUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::Usage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::AttributeUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SysML::Systems::AttributeUsage::isReference"))) (target (node (document "d0") (qualified-name "SysML::Systems::AttributeUsage::isReference"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::AttributeUsage::isReference"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::BindingConnectorAsUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::ConnectorAsUsage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::BindingConnectorAsUsage"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::CalculationDefinition"))) (target (node (document "d0") (qualified-name "SysML::Systems::ActionDefinition"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::CalculationDefinition"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::CalculationUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::ActionUsage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::CalculationUsage"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::CaseDefinition"))) (target (node (document "d0") (qualified-name "SysML::Systems::CalculationDefinition"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::CaseDefinition"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::CaseUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::CalculationUsage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::CaseUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::ConcernDefinition"))) (target (node (document "d0") (qualified-name "SysML::Systems::RequirementDefinition"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::ConcernDefinition"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::ConcernUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::RequirementUsage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::ConcernUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::ConjugatedPortDefinition"))) (target (node (document "d0") (qualified-name "SysML::Systems::PortDefinition"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::ConjugatedPortDefinition"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::ConnectionDefinition"))) (target (node (document "d0") (qualified-name "SysML::Systems::PartDefinition"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::ConnectionDefinition"))) (kind specialization) (ordinal 1)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SysML::Systems::ConnectionDefinition::isSufficient"))) (target (node (document "d0") (qualified-name "SysML::Systems::ConnectionDefinition::isSufficient"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::ConnectionDefinition::isSufficient"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::ConnectionUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::ConnectorAsUsage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::ConnectionUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::ConnectionUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::PartUsage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::ConnectionUsage"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::ConnectorAsUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::Usage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::ConnectorAsUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::ConstraintDefinition"))) (target (node (document "d0") (qualified-name "SysML::Systems::OccurrenceDefinition"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::ConstraintDefinition"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::ConstraintUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::OccurrenceUsage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::ConstraintUsage"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::ControlNode"))) (target (node (document "d0") (qualified-name "SysML::Systems::ActionUsage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::ControlNode"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::DecisionNode"))) (target (node (document "d0") (qualified-name "SysML::Systems::ControlNode"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::DecisionNode"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::EnumerationDefinition"))) (target (node (document "d0") (qualified-name "SysML::Systems::AttributeDefinition"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::EnumerationDefinition"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SysML::Systems::EnumerationDefinition::isVariation"))) (target (node (document "d0") (qualified-name "SysML::Systems::EnumerationDefinition::isVariation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::EnumerationDefinition::isVariation"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::EnumerationUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::AttributeUsage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::EnumerationUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::EventOccurrenceUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::OccurrenceUsage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::EventOccurrenceUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SysML::Systems::EventOccurrenceUsage::isReference"))) (target (node (document "d0") (qualified-name "SysML::Systems::EventOccurrenceUsage::isReference"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::EventOccurrenceUsage::isReference"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::ExhibitStateUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::PerformActionUsage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::ExhibitStateUsage"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::ExhibitStateUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::StateUsage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::ExhibitStateUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SysML::Systems::Expose::isImportAll"))) (target (node (document "d0") (qualified-name "SysML::Systems::Expose::isImportAll"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::Expose::isImportAll"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SysML::Systems::Expose::visibility"))) (target (node (document "d0") (qualified-name "SysML::Systems::Expose::visibility"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::Expose::visibility"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::FlowDefinition"))) (target (node (document "d0") (qualified-name "SysML::Systems::ActionDefinition"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::FlowDefinition"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::FlowUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::ActionUsage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::FlowUsage"))) (kind specialization) (ordinal 2)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::FlowUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::ConnectorAsUsage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::FlowUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::ForLoopActionUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::LoopActionUsage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::ForLoopActionUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::ForkNode"))) (target (node (document "d0") (qualified-name "SysML::Systems::ControlNode"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::ForkNode"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::FramedConcernMembership"))) (target (node (document "d0") (qualified-name "SysML::Systems::RequirementConstraintMembership"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::FramedConcernMembership"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "SysML::Systems::FramedConcernMembership::kind"))) (target (node (document "d0") (qualified-name "SysML::Systems::RequirementConstraintKind"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::FramedConcernMembership::kind"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SysML::Systems::FramedConcernMembership::kind"))) (target (node (document "d0") (qualified-name "SysML::Systems::FramedConcernMembership::kind"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::FramedConcernMembership::kind"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::IfActionUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::ActionUsage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::IfActionUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::IncludeUseCaseUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::PerformActionUsage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::IncludeUseCaseUsage"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::IncludeUseCaseUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::UseCaseUsage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::IncludeUseCaseUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::InterfaceDefinition"))) (target (node (document "d0") (qualified-name "SysML::Systems::ConnectionDefinition"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::InterfaceDefinition"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::InterfaceUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::ConnectionUsage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::InterfaceUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::ItemDefinition"))) (target (node (document "d0") (qualified-name "SysML::Systems::OccurrenceDefinition"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::ItemDefinition"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::ItemUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::OccurrenceUsage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::ItemUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::JoinNode"))) (target (node (document "d0") (qualified-name "SysML::Systems::ControlNode"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::JoinNode"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::LoopActionUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::ActionUsage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::LoopActionUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::MembershipExpose"))) (target (node (document "d0") (qualified-name "SysML::Systems::Expose"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::MembershipExpose"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::MergeNode"))) (target (node (document "d0") (qualified-name "SysML::Systems::ControlNode"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::MergeNode"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::MetadataDefinition"))) (target (node (document "d0") (qualified-name "SysML::Systems::ItemDefinition"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::MetadataDefinition"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::MetadataUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::ItemUsage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::MetadataUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::NamespaceExpose"))) (target (node (document "d0") (qualified-name "SysML::Systems::Expose"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::NamespaceExpose"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::OccurrenceDefinition"))) (target (node (document "d0") (qualified-name "SysML::Systems::Definition"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::OccurrenceDefinition"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::OccurrenceUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::Usage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::OccurrenceUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "SysML::Systems::OccurrenceUsage::portionKind"))) (target (node (document "d0") (qualified-name "SysML::Systems::PortionKind"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::OccurrenceUsage::portionKind"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::PartDefinition"))) (target (node (document "d0") (qualified-name "SysML::Systems::ItemDefinition"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::PartDefinition"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::PartUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::ItemUsage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::PartUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::PerformActionUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::ActionUsage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::PerformActionUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::PerformActionUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::EventOccurrenceUsage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::PerformActionUsage"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::PortDefinition"))) (target (node (document "d0") (qualified-name "SysML::Systems::OccurrenceDefinition"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::PortDefinition"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::PortUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::OccurrenceUsage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::PortUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::ReferenceUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::Usage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::ReferenceUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SysML::Systems::ReferenceUsage::isReference"))) (target (node (document "d0") (qualified-name "SysML::Systems::ReferenceUsage::isReference"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::ReferenceUsage::isReference"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::RenderingDefinition"))) (target (node (document "d0") (qualified-name "SysML::Systems::PartDefinition"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::RenderingDefinition"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::RenderingUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::PartUsage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::RenderingUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::RequirementDefinition"))) (target (node (document "d0") (qualified-name "SysML::Systems::ConstraintDefinition"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::RequirementDefinition"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::RequirementUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::ConstraintUsage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::RequirementUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::RequirementVerificationMembership"))) (target (node (document "d0") (qualified-name "SysML::Systems::RequirementConstraintMembership"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::RequirementVerificationMembership"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "SysML::Systems::RequirementVerificationMembership::kind"))) (target (node (document "d0") (qualified-name "SysML::Systems::RequirementConstraintKind"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::RequirementVerificationMembership::kind"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "SysML::Systems::RequirementVerificationMembership::kind"))) (target (node (document "d0") (qualified-name "SysML::Systems::RequirementVerificationMembership::kind"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::RequirementVerificationMembership::kind"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::SatisfyRequirementUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::AssertConstraintUsage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::SatisfyRequirementUsage"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::SatisfyRequirementUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::RequirementUsage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::SatisfyRequirementUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::SendActionUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::ActionUsage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::SendActionUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::StateDefinition"))) (target (node (document "d0") (qualified-name "SysML::Systems::ActionDefinition"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::StateDefinition"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::StateUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::ActionUsage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::StateUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::SuccessionAsUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::ConnectorAsUsage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::SuccessionAsUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::SuccessionFlowUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::FlowUsage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::SuccessionFlowUsage"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::TerminateActionUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::ActionUsage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::TerminateActionUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::TransitionUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::ActionUsage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::TransitionUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::UseCaseDefinition"))) (target (node (document "d0") (qualified-name "SysML::Systems::CaseDefinition"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::UseCaseDefinition"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::UseCaseUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::CaseUsage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::UseCaseUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::VerificationCaseDefinition"))) (target (node (document "d0") (qualified-name "SysML::Systems::CaseDefinition"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::VerificationCaseDefinition"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::VerificationCaseUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::CaseUsage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::VerificationCaseUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::ViewDefinition"))) (target (node (document "d0") (qualified-name "SysML::Systems::PartDefinition"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::ViewDefinition"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::ViewUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::PartUsage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::ViewUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::ViewpointDefinition"))) (target (node (document "d0") (qualified-name "SysML::Systems::RequirementDefinition"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::ViewpointDefinition"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::ViewpointUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::RequirementUsage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::ViewpointUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "SysML::Systems::WhileLoopActionUsage"))) (target (node (document "d0") (qualified-name "SysML::Systems::LoopActionUsage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "SysML::Systems::WhileLoopActionUsage"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 22 39) (end 22 43)) (probe (position 22 39))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::ActionUsage"))
        (kind specialization) (ordinal 0) (authored-target "Step")
        (range (start 22 39) (end 22 43))
        (outcome (status unresolved))
      )
    )
    (query (range (start 191 55) (end 191 59)) (probe (position 191 55))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::FlowUsage"))
        (kind specialization) (ordinal 1) (authored-target "Flow")
        (range (start 191 55) (end 191 59))
        (outcome (status unresolved))
      )
    )
    (query (range (start 203 62) (end 203 66)) (probe (position 203 62))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::FramedConcernMembership::kind"))
        (kind redefinition) (ordinal 0) (authored-target "kind")
        (range (start 203 62) (end 203 66))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::FramedConcernMembership::kind") (range (start 203 3) (end 203 67)))
        )
      )
    )
    (query (range (start 345 62) (end 345 66)) (probe (position 345 62))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::RequirementVerificationMembership::kind"))
        (kind redefinition) (ordinal 0) (authored-target "kind")
        (range (start 345 62) (end 345 66))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::RequirementVerificationMembership::kind") (range (start 345 3) (end 345 67)))
        )
      )
    )
    (query (range (start 59 42) (end 59 47)) (probe (position 59 42))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::AttributeUsage"))
        (kind specialization) (ordinal 0) (authored-target "Usage")
        (range (start 59 42) (end 59 47))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::Usage") (range (start 439 2) (end 439 4178)))
        )
      )
    )
    (query (range (start 114 53) (end 114 58)) (probe (position 114 53))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::ConnectorAsUsage"))
        (kind specialization) (ordinal 0) (authored-target "Usage")
        (range (start 114 53) (end 114 58))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::Usage") (range (start 439 2) (end 439 4178)))
        )
      )
    )
    (query (range (start 255 60) (end 255 65)) (probe (position 255 60))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::OccurrenceDefinition"))
        (kind specialization) (ordinal 1) (authored-target "Class")
        (range (start 255 60) (end 255 65))
        (outcome (status unresolved))
      )
    )
    (query (range (start 259 43) (end 259 48)) (probe (position 259 43))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::OccurrenceUsage"))
        (kind specialization) (ordinal 0) (authored-target "Usage")
        (range (start 259 43) (end 259 48))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::Usage") (range (start 439 2) (end 439 4178)))
        )
      )
    )
    (query (range (start 295 42) (end 295 47)) (probe (position 295 42))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::ReferenceUsage"))
        (kind specialization) (ordinal 0) (authored-target "Usage")
        (range (start 295 42) (end 295 47))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::Usage") (range (start 439 2) (end 439 4178)))
        )
      )
    )
    (query (range (start 182 43) (end 182 49)) (probe (position 182 43))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::Expose"))
        (kind specialization) (ordinal 0) (authored-target "Import")
        (range (start 182 43) (end 182 49))
        (outcome (status unresolved))
      )
    )
    (query (range (start 239 62) (end 239 68)) (probe (position 239 62))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::MembershipExpose"))
        (kind specialization) (ordinal 1) (authored-target "Expose")
        (range (start 239 62) (end 239 68))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::Expose") (range (start 182 2) (end 182 188)))
        )
      )
    )
    (query (range (start 249 43) (end 249 49)) (probe (position 249 43))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::NamespaceExpose"))
        (kind specialization) (ordinal 0) (authored-target "Expose")
        (range (start 249 43) (end 249 49))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::Expose") (range (start 182 2) (end 182 188)))
        )
      )
    )
    (query (range (start 7 15) (end 7 22)) (probe (position 7 15))
      (reference
        (source (document "d0") (qualified-name "SysML::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "Systems::*")
        (range (start 7 15) (end 7 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems") (range (start 9 1) (end 9 30590)))
        )
      )
    )
    (query (range (start 439 33) (end 439 40)) (probe (position 439 33))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::Usage"))
        (kind specialization) (ordinal 0) (authored-target "Feature")
        (range (start 439 33) (end 439 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 18 44) (end 18 52)) (probe (position 18 44))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::ActionDefinition"))
        (kind specialization) (ordinal 0) (authored-target "Behavior")
        (range (start 18 44) (end 18 52))
        (outcome (status unresolved))
      )
    )
    (query (range (start 57 47) (end 57 55)) (probe (position 57 47))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::AttributeDefinition"))
        (kind specialization) (ordinal 0) (authored-target "DataType")
        (range (start 57 47) (end 57 55))
        (outcome (status unresolved))
      )
    )
    (query (range (start 67 49) (end 67 57)) (probe (position 67 49))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::CalculationDefinition"))
        (kind specialization) (ordinal 0) (authored-target "Function")
        (range (start 67 49) (end 67 57))
        (outcome (status unresolved))
      )
    )
    (query (range (start 42 45) (end 42 54)) (probe (position 42 45))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::AnalysisCaseUsage"))
        (kind specialization) (ordinal 0) (authored-target "CaseUsage")
        (range (start 42 45) (end 42 54))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::CaseUsage") (range (start 81 2) (end 81 531)))
        )
      )
    )
    (query (range (start 47 66) (end 47 75)) (probe (position 47 66))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::AssertConstraintUsage"))
        (kind specialization) (ordinal 1) (authored-target "Invariant")
        (range (start 47 66) (end 47 75))
        (outcome (status unresolved))
      )
    )
    (query (range (start 110 61) (end 110 70)) (probe (position 110 61))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::ConnectionUsage"))
        (kind specialization) (ordinal 1) (authored-target "PartUsage")
        (range (start 110 61) (end 110 70))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::PartUsage") (range (start 269 2) (end 269 174)))
        )
      )
    )
    (query (range (start 114 60) (end 114 69)) (probe (position 114 60))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::ConnectorAsUsage"))
        (kind specialization) (ordinal 1) (authored-target "Connector")
        (range (start 114 60) (end 114 69))
        (outcome (status unresolved))
      )
    )
    (query (range (start 116 70) (end 116 79)) (probe (position 116 70))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::ConstraintDefinition"))
        (kind specialization) (ordinal 1) (authored-target "Predicate")
        (range (start 116 70) (end 116 79))
        (outcome (status unresolved))
      )
    )
    (query (range (start 227 42) (end 227 51)) (probe (position 227 42))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::ItemDefinition"))
        (kind specialization) (ordinal 0) (authored-target "Structure")
        (range (start 227 42) (end 227 51))
        (outcome (status unresolved))
      )
    )
    (query (range (start 243 62) (end 243 71)) (probe (position 243 62))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::MetadataDefinition"))
        (kind specialization) (ordinal 1) (authored-target "Metaclass")
        (range (start 243 62) (end 243 71))
        (outcome (status unresolved))
      )
    )
    (query (range (start 245 41) (end 245 50)) (probe (position 245 41))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::MetadataUsage"))
        (kind specialization) (ordinal 0) (authored-target "ItemUsage")
        (range (start 245 41) (end 245 50))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::ItemUsage") (range (start 229 2) (end 229 213)))
        )
      )
    )
    (query (range (start 269 37) (end 269 46)) (probe (position 269 37))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::PartUsage"))
        (kind specialization) (ordinal 0) (authored-target "ItemUsage")
        (range (start 269 37) (end 269 46))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::ItemUsage") (range (start 229 2) (end 229 213)))
        )
      )
    )
    (query (range (start 282 64) (end 282 73)) (probe (position 282 64))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::PortDefinition"))
        (kind specialization) (ordinal 1) (authored-target "Structure")
        (range (start 282 64) (end 282 73))
        (outcome (status unresolved))
      )
    )
    (query (range (start 303 42) (end 303 51)) (probe (position 303 42))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::RenderingUsage"))
        (kind specialization) (ordinal 0) (authored-target "PartUsage")
        (range (start 303 42) (end 303 51))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::PartUsage") (range (start 269 2) (end 269 174)))
        )
      )
    )
    (query (range (start 402 63) (end 402 72)) (probe (position 402 63))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::SuccessionFlowUsage"))
        (kind specialization) (ordinal 1) (authored-target "FlowUsage")
        (range (start 402 63) (end 402 72))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::FlowUsage") (range (start 191 2) (end 191 214)))
        )
      )
    )
    (query (range (start 484 40) (end 484 49)) (probe (position 484 40))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::UseCaseUsage"))
        (kind specialization) (ordinal 0) (authored-target "CaseUsage")
        (range (start 484 40) (end 484 49))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::CaseUsage") (range (start 81 2) (end 81 531)))
        )
      )
    )
    (query (range (start 497 49) (end 497 58)) (probe (position 497 49))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::VerificationCaseUsage"))
        (kind specialization) (ordinal 0) (authored-target "CaseUsage")
        (range (start 497 49) (end 497 58))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::CaseUsage") (range (start 81 2) (end 81 531)))
        )
      )
    )
    (query (range (start 514 37) (end 514 46)) (probe (position 514 37))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::ViewUsage"))
        (kind specialization) (ordinal 0) (authored-target "PartUsage")
        (range (start 514 37) (end 514 46))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::PartUsage") (range (start 269 2) (end 269 174)))
        )
      )
    )
    (query (range (start 57 57) (end 57 67)) (probe (position 57 57))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::AttributeDefinition"))
        (kind specialization) (ordinal 1) (authored-target "Definition")
        (range (start 57 57) (end 57 67))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::Definition") (range (start 126 2) (end 126 3700)))
        )
      )
    )
    (query (range (start 71 44) (end 71 54)) (probe (position 71 44))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::CalculationUsage"))
        (kind specialization) (ordinal 0) (authored-target "Expression")
        (range (start 71 44) (end 71 54))
        (outcome (status unresolved))
      )
    )
    (query (range (start 126 38) (end 126 48)) (probe (position 126 38))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::Definition"))
        (kind specialization) (ordinal 0) (authored-target "Classifier")
        (range (start 126 38) (end 126 48))
        (outcome (status unresolved))
      )
    )
    (query (range (start 178 45) (end 178 55)) (probe (position 178 45))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::ExhibitStateUsage"))
        (kind specialization) (ordinal 0) (authored-target "StateUsage")
        (range (start 178 45) (end 178 55))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::StateUsage") (range (start 387 2) (end 387 472)))
        )
      )
    )
    (query (range (start 183 57) (end 183 67)) (probe (position 183 57))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::Expose::visibility"))
        (kind redefinition) (ordinal 0) (authored-target "visibility")
        (range (start 183 57) (end 183 67))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::Expose::visibility") (range (start 183 3) (end 183 68)))
        )
      )
    )
    (query (range (start 255 48) (end 255 58)) (probe (position 255 48))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::OccurrenceDefinition"))
        (kind specialization) (ordinal 0) (authored-target "Definition")
        (range (start 255 48) (end 255 58))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::Definition") (range (start 126 2) (end 126 3700)))
        )
      )
    )
    (query (range (start 400 63) (end 400 73)) (probe (position 400 63))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::SuccessionAsUsage"))
        (kind specialization) (ordinal 1) (authored-target "Succession")
        (range (start 400 63) (end 400 73))
        (outcome (status unresolved))
      )
    )
    (query (range (start 441 59) (end 441 69)) (probe (position 441 59))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::Usage::mayTimeVary"))
        (kind redefinition) (ordinal 0) (authored-target "isVariable")
        (range (start 441 59) (end 441 69))
        (outcome (status unresolved))
      )
    )
    (query (range (start 12 45) (end 12 56)) (probe (position 12 45))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::AcceptActionUsage"))
        (kind specialization) (ordinal 0) (authored-target "ActionUsage")
        (range (start 12 45) (end 12 56))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::ActionUsage") (range (start 22 2) (end 22 202)))
        )
      )
    )
    (query (range (start 51 49) (end 51 60)) (probe (position 51 49))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::AssignmentActionUsage"))
        (kind specialization) (ordinal 0) (authored-target "ActionUsage")
        (range (start 51 49) (end 51 60))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::ActionUsage") (range (start 22 2) (end 22 202)))
        )
      )
    )
    (query (range (start 60 59) (end 60 70)) (probe (position 60 59))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::AttributeUsage::isReference"))
        (kind redefinition) (ordinal 0) (authored-target "isReference")
        (range (start 60 59) (end 60 70))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::AttributeUsage::isReference") (range (start 60 3) (end 60 71)))
        )
      )
    )
    (query (range (start 71 56) (end 71 67)) (probe (position 71 56))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::CalculationUsage"))
        (kind specialization) (ordinal 1) (authored-target "ActionUsage")
        (range (start 71 56) (end 71 67))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::ActionUsage") (range (start 22 2) (end 22 202)))
        )
      )
    )
    (query (range (start 122 48) (end 122 59)) (probe (position 122 48))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::ControlNode"))
        (kind specialization) (ordinal 0) (authored-target "ActionUsage")
        (range (start 122 48) (end 122 59))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::ActionUsage") (range (start 22 2) (end 22 202)))
        )
      )
    )
    (query (range (start 124 40) (end 124 51)) (probe (position 124 40))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::DecisionNode"))
        (kind specialization) (ordinal 0) (authored-target "ControlNode")
        (range (start 124 40) (end 124 51))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::ControlNode") (range (start 122 2) (end 122 60)))
        )
      )
    )
    (query (range (start 163 51) (end 163 62)) (probe (position 163 51))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::EnumerationDefinition::isVariation"))
        (kind redefinition) (ordinal 0) (authored-target "isVariation")
        (range (start 163 51) (end 163 62))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::EnumerationDefinition::isVariation") (range (start 163 3) (end 163 63)))
        )
      )
    )
    (query (range (start 173 59) (end 173 70)) (probe (position 173 59))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::EventOccurrenceUsage::isReference"))
        (kind redefinition) (ordinal 0) (authored-target "isReference")
        (range (start 173 59) (end 173 70))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::EventOccurrenceUsage::isReference") (range (start 173 3) (end 173 71)))
        )
      )
    )
    (query (range (start 184 51) (end 184 62)) (probe (position 184 51))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::Expose::isImportAll"))
        (kind redefinition) (ordinal 0) (authored-target "isImportAll")
        (range (start 184 51) (end 184 62))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::Expose::isImportAll") (range (start 184 3) (end 184 63)))
        )
      )
    )
    (query (range (start 187 42) (end 187 53)) (probe (position 187 42))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::FlowDefinition"))
        (kind specialization) (ordinal 0) (authored-target "Interaction")
        (range (start 187 42) (end 187 53))
        (outcome (status unresolved))
      )
    )
    (query (range (start 191 61) (end 191 72)) (probe (position 191 61))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::FlowUsage"))
        (kind specialization) (ordinal 2) (authored-target "ActionUsage")
        (range (start 191 61) (end 191 72))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::ActionUsage") (range (start 22 2) (end 22 202)))
        )
      )
    )
    (query (range (start 200 36) (end 200 47)) (probe (position 200 36))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::ForkNode"))
        (kind specialization) (ordinal 0) (authored-target "ControlNode")
        (range (start 200 36) (end 200 47))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::ControlNode") (range (start 122 2) (end 122 60)))
        )
      )
    )
    (query (range (start 209 41) (end 209 52)) (probe (position 209 41))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::IfActionUsage"))
        (kind specialization) (ordinal 0) (authored-target "ActionUsage")
        (range (start 209 41) (end 209 52))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::ActionUsage") (range (start 22 2) (end 22 202)))
        )
      )
    )
    (query (range (start 233 36) (end 233 47)) (probe (position 233 36))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::JoinNode"))
        (kind specialization) (ordinal 0) (authored-target "ControlNode")
        (range (start 233 36) (end 233 47))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::ControlNode") (range (start 122 2) (end 122 60)))
        )
      )
    )
    (query (range (start 235 52) (end 235 63)) (probe (position 235 52))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::LoopActionUsage"))
        (kind specialization) (ordinal 0) (authored-target "ActionUsage")
        (range (start 235 52) (end 235 63))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::ActionUsage") (range (start 22 2) (end 22 202)))
        )
      )
    )
    (query (range (start 241 37) (end 241 48)) (probe (position 241 37))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::MergeNode"))
        (kind specialization) (ordinal 0) (authored-target "ControlNode")
        (range (start 241 37) (end 241 48))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::ControlNode") (range (start 122 2) (end 122 60)))
        )
      )
    )
    (query (range (start 273 46) (end 273 57)) (probe (position 273 46))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::PerformActionUsage"))
        (kind specialization) (ordinal 0) (authored-target "ActionUsage")
        (range (start 273 46) (end 273 57))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::ActionUsage") (range (start 22 2) (end 22 202)))
        )
      )
    )
    (query (range (start 277 43) (end 277 54)) (probe (position 277 43))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::PortConjugation"))
        (kind specialization) (ordinal 0) (authored-target "Conjugation")
        (range (start 277 43) (end 277 54))
        (outcome (status unresolved))
      )
    )
    (query (range (start 296 59) (end 296 70)) (probe (position 296 59))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::ReferenceUsage::isReference"))
        (kind redefinition) (ordinal 0) (authored-target "isReference")
        (range (start 296 59) (end 296 70))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::ReferenceUsage::isReference") (range (start 296 3) (end 296 71)))
        )
      )
    )
    (query (range (start 356 43) (end 356 54)) (probe (position 356 43))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::SendActionUsage"))
        (kind specialization) (ordinal 0) (authored-target "ActionUsage")
        (range (start 356 43) (end 356 54))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::ActionUsage") (range (start 22 2) (end 22 202)))
        )
      )
    )
    (query (range (start 387 38) (end 387 49)) (probe (position 387 38))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::StateUsage"))
        (kind specialization) (ordinal 0) (authored-target "ActionUsage")
        (range (start 387 38) (end 387 49))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::ActionUsage") (range (start 22 2) (end 22 202)))
        )
      )
    )
    (query (range (start 404 48) (end 404 59)) (probe (position 404 48))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::TerminateActionUsage"))
        (kind specialization) (ordinal 0) (authored-target "ActionUsage")
        (range (start 404 48) (end 404 59))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::ActionUsage") (range (start 22 2) (end 22 202)))
        )
      )
    )
    (query (range (start 420 43) (end 420 54)) (probe (position 420 43))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::TransitionUsage"))
        (kind specialization) (ordinal 0) (authored-target "ActionUsage")
        (range (start 420 43) (end 420 54))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::ActionUsage") (range (start 22 2) (end 22 202)))
        )
      )
    )
    (query (range (start 6 16) (end 6 28)) (probe (position 6 16))
      (reference
        (source (document "d0") (qualified-name "SysML::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 6 16) (end 6 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 105 52) (end 105 64)) (probe (position 105 52))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::ConnectionDefinition::isSufficient"))
        (kind redefinition) (ordinal 0) (authored-target "isSufficient")
        (range (start 105 52) (end 105 64))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::ConnectionDefinition::isSufficient") (range (start 105 3) (end 105 65)))
        )
      )
    )
    (query (range (start 215 47) (end 215 59)) (probe (position 215 47))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::IncludeUseCaseUsage"))
        (kind specialization) (ordinal 0) (authored-target "UseCaseUsage")
        (range (start 215 47) (end 215 59))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::UseCaseUsage") (range (start 484 2) (end 484 275)))
        )
      )
    )
    (query (range (start 10 16) (end 10 29)) (probe (position 10 16))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "KerML::Kernel::*")
        (range (start 10 16) (end 10 29))
        (outcome (status unresolved))
      )
    )
    (query (range (start 99 48) (end 99 61)) (probe (position 99 48))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::ConjugatedPortTyping"))
        (kind specialization) (ordinal 0) (authored-target "FeatureTyping")
        (range (start 99 48) (end 99 61))
        (outcome (status unresolved))
      )
    )
    (query (range (start 38 50) (end 38 64)) (probe (position 38 50))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::AnalysisCaseDefinition"))
        (kind specialization) (ordinal 0) (authored-target "CaseDefinition")
        (range (start 38 50) (end 38 64))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::CaseDefinition") (range (start 75 2) (end 75 418)))
        )
      )
    )
    (query (range (start 94 52) (end 94 66)) (probe (position 94 52))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::ConjugatedPortDefinition"))
        (kind specialization) (ordinal 0) (authored-target "PortDefinition")
        (range (start 94 52) (end 94 66))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::PortDefinition") (range (start 282 2) (end 282 210)))
        )
      )
    )
    (query (range (start 104 70) (end 104 84)) (probe (position 104 70))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::ConnectionDefinition"))
        (kind specialization) (ordinal 1) (authored-target "PartDefinition")
        (range (start 104 70) (end 104 84))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::PartDefinition") (range (start 267 2) (end 267 57)))
        )
      )
    )
    (query (range (start 168 44) (end 168 58)) (probe (position 168 44))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::EnumerationUsage"))
        (kind specialization) (ordinal 0) (authored-target "AttributeUsage")
        (range (start 168 44) (end 168 58))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::AttributeUsage") (range (start 59 2) (end 59 248)))
        )
      )
    )
    (query (range (start 243 46) (end 243 60)) (probe (position 243 46))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::MetadataDefinition"))
        (kind specialization) (ordinal 0) (authored-target "ItemDefinition")
        (range (start 243 46) (end 243 60))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::ItemDefinition") (range (start 227 2) (end 227 74)))
        )
      )
    )
    (query (range (start 267 42) (end 267 56)) (probe (position 267 42))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::PartDefinition"))
        (kind specialization) (ordinal 0) (authored-target "ItemDefinition")
        (range (start 267 42) (end 267 56))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::ItemDefinition") (range (start 227 2) (end 227 74)))
        )
      )
    )
    (query (range (start 299 47) (end 299 61)) (probe (position 299 47))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::RenderingDefinition"))
        (kind specialization) (ordinal 0) (authored-target "PartDefinition")
        (range (start 299 47) (end 299 61))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::PartDefinition") (range (start 267 2) (end 267 57)))
        )
      )
    )
    (query (range (start 402 47) (end 402 61)) (probe (position 402 47))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::SuccessionFlowUsage"))
        (kind specialization) (ordinal 0) (authored-target "SuccessionFlow")
        (range (start 402 47) (end 402 61))
        (outcome (status unresolved))
      )
    )
    (query (range (start 480 45) (end 480 59)) (probe (position 480 45))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::UseCaseDefinition"))
        (kind specialization) (ordinal 0) (authored-target "CaseDefinition")
        (range (start 480 45) (end 480 59))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::CaseDefinition") (range (start 75 2) (end 75 418)))
        )
      )
    )
    (query (range (start 493 54) (end 493 68)) (probe (position 493 54))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::VerificationCaseDefinition"))
        (kind specialization) (ordinal 0) (authored-target "CaseDefinition")
        (range (start 493 54) (end 493 68))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::CaseDefinition") (range (start 75 2) (end 75 418)))
        )
      )
    )
    (query (range (start 502 42) (end 502 56)) (probe (position 502 42))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::ViewDefinition"))
        (kind specialization) (ordinal 0) (authored-target "PartDefinition")
        (range (start 502 42) (end 502 56))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::PartDefinition") (range (start 267 2) (end 267 57)))
        )
      )
    )
    (query (range (start 22 45) (end 22 60)) (probe (position 22 45))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::ActionUsage"))
        (kind specialization) (ordinal 1) (authored-target "OccurrenceUsage")
        (range (start 22 45) (end 22 60))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::OccurrenceUsage") (range (start 259 2) (end 259 396)))
        )
      )
    )
    (query (range (start 34 43) (end 34 58)) (probe (position 34 43))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::AllocationUsage"))
        (kind specialization) (ordinal 0) (authored-target "ConnectionUsage")
        (range (start 34 43) (end 34 58))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::ConnectionUsage") (range (start 110 2) (end 110 232)))
        )
      )
    )
    (query (range (start 47 49) (end 47 64)) (probe (position 47 49))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::AssertConstraintUsage"))
        (kind specialization) (ordinal 0) (authored-target "ConstraintUsage")
        (range (start 47 49) (end 47 64))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::ConstraintUsage") (range (start 118 2) (end 118 195)))
        )
      )
    )
    (query (range (start 118 62) (end 118 77)) (probe (position 118 62))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::ConstraintUsage"))
        (kind specialization) (ordinal 1) (authored-target "OccurrenceUsage")
        (range (start 118 62) (end 118 77))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::OccurrenceUsage") (range (start 259 2) (end 259 396)))
        )
      )
    )
    (query (range (start 172 48) (end 172 63)) (probe (position 172 48))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::EventOccurrenceUsage"))
        (kind specialization) (ordinal 0) (authored-target "OccurrenceUsage")
        (range (start 172 48) (end 172 63))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::OccurrenceUsage") (range (start 259 2) (end 259 396)))
        )
      )
    )
    (query (range (start 195 46) (end 195 61)) (probe (position 195 46))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::ForLoopActionUsage"))
        (kind specialization) (ordinal 0) (authored-target "LoopActionUsage")
        (range (start 195 46) (end 195 61))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::LoopActionUsage") (range (start 235 2) (end 235 153)))
        )
      )
    )
    (query (range (start 223 42) (end 223 57)) (probe (position 223 42))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::InterfaceUsage"))
        (kind specialization) (ordinal 0) (authored-target "ConnectionUsage")
        (range (start 223 42) (end 223 57))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::ConnectionUsage") (range (start 110 2) (end 110 232)))
        )
      )
    )
    (query (range (start 229 37) (end 229 52)) (probe (position 229 37))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::ItemUsage"))
        (kind specialization) (ordinal 0) (authored-target "OccurrenceUsage")
        (range (start 229 37) (end 229 52))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::OccurrenceUsage") (range (start 259 2) (end 259 396)))
        )
      )
    )
    (query (range (start 245 52) (end 245 67)) (probe (position 245 52))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::MetadataUsage"))
        (kind specialization) (ordinal 1) (authored-target "MetadataFeature")
        (range (start 245 52) (end 245 67))
        (outcome (status unresolved))
      )
    )
    (query (range (start 249 51) (end 249 66)) (probe (position 249 51))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::NamespaceExpose"))
        (kind specialization) (ordinal 1) (authored-target "NamespaceImport")
        (range (start 249 51) (end 249 66))
        (outcome (status unresolved))
      )
    )
    (query (range (start 286 37) (end 286 52)) (probe (position 286 37))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::PortUsage"))
        (kind specialization) (ordinal 0) (authored-target "OccurrenceUsage")
        (range (start 286 37) (end 286 52))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::OccurrenceUsage") (range (start 259 2) (end 259 396)))
        )
      )
    )
    (query (range (start 331 44) (end 331 59)) (probe (position 331 44))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::RequirementUsage"))
        (kind specialization) (ordinal 0) (authored-target "ConstraintUsage")
        (range (start 331 44) (end 331 59))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::ConstraintUsage") (range (start 118 2) (end 118 195)))
        )
      )
    )
    (query (range (start 531 48) (end 531 63)) (probe (position 531 48))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::WhileLoopActionUsage"))
        (kind specialization) (ordinal 0) (authored-target "LoopActionUsage")
        (range (start 531 48) (end 531 63))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::LoopActionUsage") (range (start 235 2) (end 235 153)))
        )
      )
    )
    (query (range (start 65 51) (end 65 67)) (probe (position 65 51))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::BindingConnectorAsUsage"))
        (kind specialization) (ordinal 0) (authored-target "BindingConnector")
        (range (start 65 51) (end 65 67))
        (outcome (status unresolved))
      )
    )
    (query (range (start 65 69) (end 65 85)) (probe (position 65 69))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::BindingConnectorAsUsage"))
        (kind specialization) (ordinal 1) (authored-target "ConnectorAsUsage")
        (range (start 65 69) (end 65 85))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::ConnectorAsUsage") (range (start 114 2) (end 114 70)))
        )
      )
    )
    (query (range (start 67 59) (end 67 75)) (probe (position 67 59))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::CalculationDefinition"))
        (kind specialization) (ordinal 1) (authored-target "ActionDefinition")
        (range (start 67 59) (end 67 75))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::ActionDefinition") (range (start 18 2) (end 18 190)))
        )
      )
    )
    (query (range (start 81 37) (end 81 53)) (probe (position 81 37))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::CaseUsage"))
        (kind specialization) (ordinal 0) (authored-target "CalculationUsage")
        (range (start 81 37) (end 81 53))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::CalculationUsage") (range (start 71 2) (end 71 210)))
        )
      )
    )
    (query (range (start 90 40) (end 90 56)) (probe (position 90 40))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::ConcernUsage"))
        (kind specialization) (ordinal 0) (authored-target "RequirementUsage")
        (range (start 90 40) (end 90 56))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::RequirementUsage") (range (start 331 2) (end 331 1035)))
        )
      )
    )
    (query (range (start 110 43) (end 110 59)) (probe (position 110 43))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::ConnectionUsage"))
        (kind specialization) (ordinal 0) (authored-target "ConnectorAsUsage")
        (range (start 110 43) (end 110 59))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::ConnectorAsUsage") (range (start 114 2) (end 114 70)))
        )
      )
    )
    (query (range (start 187 55) (end 187 71)) (probe (position 187 55))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::FlowDefinition"))
        (kind specialization) (ordinal 1) (authored-target "ActionDefinition")
        (range (start 187 55) (end 187 71))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::ActionDefinition") (range (start 18 2) (end 18 190)))
        )
      )
    )
    (query (range (start 191 37) (end 191 53)) (probe (position 191 37))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::FlowUsage"))
        (kind specialization) (ordinal 0) (authored-target "ConnectorAsUsage")
        (range (start 191 37) (end 191 53))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::ConnectorAsUsage") (range (start 114 2) (end 114 70)))
        )
      )
    )
    (query (range (start 239 44) (end 239 60)) (probe (position 239 44))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::MembershipExpose"))
        (kind specialization) (ordinal 0) (authored-target "MembershipImport")
        (range (start 239 44) (end 239 60))
        (outcome (status unresolved))
      )
    )
    (query (range (start 351 51) (end 351 67)) (probe (position 351 51))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::SatisfyRequirementUsage"))
        (kind specialization) (ordinal 0) (authored-target "RequirementUsage")
        (range (start 351 51) (end 351 67))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::RequirementUsage") (range (start 331 2) (end 331 1035)))
        )
      )
    )
    (query (range (start 366 43) (end 366 59)) (probe (position 366 43))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::StateDefinition"))
        (kind specialization) (ordinal 0) (authored-target "ActionDefinition")
        (range (start 366 43) (end 366 59))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::ActionDefinition") (range (start 18 2) (end 18 190)))
        )
      )
    )
    (query (range (start 400 45) (end 400 61)) (probe (position 400 45))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::SuccessionAsUsage"))
        (kind specialization) (ordinal 0) (authored-target "ConnectorAsUsage")
        (range (start 400 45) (end 400 61))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::ConnectorAsUsage") (range (start 114 2) (end 114 70)))
        )
      )
    )
    (query (range (start 489 45) (end 489 61)) (probe (position 489 45))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::VariantMembership"))
        (kind specialization) (ordinal 0) (authored-target "OwningMembership")
        (range (start 489 45) (end 489 61))
        (outcome (status unresolved))
      )
    )
    (query (range (start 526 42) (end 526 58)) (probe (position 526 42))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::ViewpointUsage"))
        (kind specialization) (ordinal 0) (authored-target "RequirementUsage")
        (range (start 526 42) (end 526 58))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::RequirementUsage") (range (start 331 2) (end 331 1035)))
        )
      )
    )
    (query (range (start 118 43) (end 118 60)) (probe (position 118 43))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::ConstraintUsage"))
        (kind specialization) (ordinal 0) (authored-target "BooleanExpression")
        (range (start 118 43) (end 118 60))
        (outcome (status unresolved))
      )
    )
    (query (range (start 251 47) (end 251 64)) (probe (position 251 47))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::ObjectiveMembership"))
        (kind specialization) (ordinal 0) (authored-target "FeatureMembership")
        (range (start 251 47) (end 251 64))
        (outcome (status unresolved))
      )
    )
    (query (range (start 312 59) (end 312 76)) (probe (position 312 59))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::RequirementConstraintMembership"))
        (kind specialization) (ordinal 0) (authored-target "FeatureMembership")
        (range (start 312 59) (end 312 76))
        (outcome (status unresolved))
      )
    )
    (query (range (start 320 44) (end 320 61)) (probe (position 320 44))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::RequirementDefinition::reqId"))
        (kind redefinition) (ordinal 0) (authored-target "declaredShortName")
        (range (start 320 44) (end 320 61))
        (outcome (status unresolved))
      )
    )
    (query (range (start 332 44) (end 332 61)) (probe (position 332 44))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::RequirementUsage::reqId"))
        (kind redefinition) (ordinal 0) (authored-target "declaredShortName")
        (range (start 332 44) (end 332 61))
        (outcome (status unresolved))
      )
    )
    (query (range (start 381 52) (end 381 69)) (probe (position 381 52))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::StateSubactionMembership"))
        (kind specialization) (ordinal 0) (authored-target "FeatureMembership")
        (range (start 381 52) (end 381 69))
        (outcome (status unresolved))
      )
    )
    (query (range (start 414 55) (end 414 72)) (probe (position 414 55))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::TransitionFeatureMembership"))
        (kind specialization) (ordinal 0) (authored-target "FeatureMembership")
        (range (start 414 55) (end 414 72))
        (outcome (status unresolved))
      )
    )
    (query (range (start 509 51) (end 509 68)) (probe (position 509 51))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::ViewRenderingMembership"))
        (kind specialization) (ordinal 0) (authored-target "FeatureMembership")
        (range (start 509 51) (end 509 68))
        (outcome (status unresolved))
      )
    )
    (query (range (start 178 57) (end 178 75)) (probe (position 178 57))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::ExhibitStateUsage"))
        (kind specialization) (ordinal 1) (authored-target "PerformActionUsage")
        (range (start 178 57) (end 178 75))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::PerformActionUsage") (range (start 273 2) (end 273 200)))
        )
      )
    )
    (query (range (start 215 61) (end 215 79)) (probe (position 215 61))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::IncludeUseCaseUsage"))
        (kind specialization) (ordinal 1) (authored-target "PerformActionUsage")
        (range (start 215 61) (end 215 79))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::PerformActionUsage") (range (start 273 2) (end 273 200)))
        )
      )
    )
    (query (range (start 26 43) (end 26 62)) (probe (position 26 43))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::ActorMembership"))
        (kind specialization) (ordinal 0) (authored-target "ParameterMembership")
        (range (start 26 43) (end 26 62))
        (outcome (status unresolved))
      )
    )
    (query (range (start 162 49) (end 162 68)) (probe (position 162 49))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::EnumerationDefinition"))
        (kind specialization) (ordinal 0) (authored-target "AttributeDefinition")
        (range (start 162 49) (end 162 68))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::AttributeDefinition") (range (start 57 2) (end 57 68)))
        )
      )
    )
    (query (range (start 362 49) (end 362 68)) (probe (position 362 49))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::StakeholderMembership"))
        (kind specialization) (ordinal 0) (authored-target "ParameterMembership")
        (range (start 362 49) (end 362 68))
        (outcome (status unresolved))
      )
    )
    (query (range (start 396 45) (end 396 64)) (probe (position 396 45))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::SubjectMembership"))
        (kind specialization) (ordinal 0) (authored-target "ParameterMembership")
        (range (start 396 45) (end 396 64))
        (outcome (status unresolved))
      )
    )
    (query (range (start 18 54) (end 18 74)) (probe (position 18 54))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::ActionDefinition"))
        (kind specialization) (ordinal 1) (authored-target "OccurrenceDefinition")
        (range (start 18 54) (end 18 74))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::OccurrenceDefinition") (range (start 255 2) (end 255 114)))
        )
      )
    )
    (query (range (start 30 48) (end 30 68)) (probe (position 30 48))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::AllocationDefinition"))
        (kind specialization) (ordinal 0) (authored-target "ConnectionDefinition")
        (range (start 30 48) (end 30 68))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::ConnectionDefinition") (range (start 104 2) (end 104 274)))
        )
      )
    )
    (query (range (start 104 48) (end 104 68)) (probe (position 104 48))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::ConnectionDefinition"))
        (kind specialization) (ordinal 0) (authored-target "AssociationStructure")
        (range (start 104 48) (end 104 68))
        (outcome (status unresolved))
      )
    )
    (query (range (start 116 48) (end 116 68)) (probe (position 116 48))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::ConstraintDefinition"))
        (kind specialization) (ordinal 0) (authored-target "OccurrenceDefinition")
        (range (start 116 48) (end 116 68))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::OccurrenceDefinition") (range (start 255 2) (end 255 114)))
        )
      )
    )
    (query (range (start 219 47) (end 219 67)) (probe (position 219 47))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::InterfaceDefinition"))
        (kind specialization) (ordinal 0) (authored-target "ConnectionDefinition")
        (range (start 219 47) (end 219 67))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::ConnectionDefinition") (range (start 104 2) (end 104 274)))
        )
      )
    )
    (query (range (start 227 53) (end 227 73)) (probe (position 227 53))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::ItemDefinition"))
        (kind specialization) (ordinal 1) (authored-target "OccurrenceDefinition")
        (range (start 227 53) (end 227 73))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::OccurrenceDefinition") (range (start 255 2) (end 255 114)))
        )
      )
    )
    (query (range (start 273 59) (end 273 79)) (probe (position 273 59))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::PerformActionUsage"))
        (kind specialization) (ordinal 1) (authored-target "EventOccurrenceUsage")
        (range (start 273 59) (end 273 79))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::EventOccurrenceUsage") (range (start 172 2) (end 172 238)))
        )
      )
    )
    (query (range (start 282 42) (end 282 62)) (probe (position 282 42))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::PortDefinition"))
        (kind specialization) (ordinal 0) (authored-target "OccurrenceDefinition")
        (range (start 282 42) (end 282 62))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::OccurrenceDefinition") (range (start 255 2) (end 255 114)))
        )
      )
    )
    (query (range (start 319 49) (end 319 69)) (probe (position 319 49))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::RequirementDefinition"))
        (kind specialization) (ordinal 0) (authored-target "ConstraintDefinition")
        (range (start 319 49) (end 319 69))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::ConstraintDefinition") (range (start 116 2) (end 116 80)))
        )
      )
    )
    (query (range (start 429 55) (end 429 75)) (probe (position 429 55))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::TriggerInvocationExpression"))
        (kind specialization) (ordinal 0) (authored-target "InvocationExpression")
        (range (start 429 55) (end 429 75))
        (outcome (status unresolved))
      )
    )
    (query (range (start 75 42) (end 75 63)) (probe (position 75 42))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::CaseDefinition"))
        (kind specialization) (ordinal 0) (authored-target "CalculationDefinition")
        (range (start 75 42) (end 75 63))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::CalculationDefinition") (range (start 67 2) (end 67 208)))
        )
      )
    )
    (query (range (start 88 45) (end 88 66)) (probe (position 88 45))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::ConcernDefinition"))
        (kind specialization) (ordinal 0) (authored-target "RequirementDefinition")
        (range (start 88 45) (end 88 66))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::RequirementDefinition") (range (start 319 2) (end 319 909)))
        )
      )
    )
    (query (range (start 351 69) (end 351 90)) (probe (position 351 69))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::SatisfyRequirementUsage"))
        (kind specialization) (ordinal 1) (authored-target "AssertConstraintUsage")
        (range (start 351 69) (end 351 90))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::AssertConstraintUsage") (range (start 47 2) (end 47 177)))
        )
      )
    )
    (query (range (start 522 47) (end 522 68)) (probe (position 522 47))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::ViewpointDefinition"))
        (kind specialization) (ordinal 0) (authored-target "RequirementDefinition")
        (range (start 522 47) (end 522 68))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::RequirementDefinition") (range (start 319 2) (end 319 909)))
        )
      )
    )
    (query (range (start 202 51) (end 202 82)) (probe (position 202 51))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::FramedConcernMembership"))
        (kind specialization) (ordinal 0) (authored-target "RequirementConstraintMembership")
        (range (start 202 51) (end 202 82))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::RequirementConstraintMembership") (range (start 312 2) (end 312 355)))
        )
      )
    )
    (query (range (start 344 61) (end 344 92)) (probe (position 344 61))
      (reference
        (source (document "d0") (qualified-name "SysML::Systems::RequirementVerificationMembership"))
        (kind specialization) (ordinal 0) (authored-target "RequirementConstraintMembership")
        (range (start 344 61) (end 344 92))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SysML::Systems::RequirementConstraintMembership") (range (start 312 2) (end 312 355)))
        )
      )
    )
  )
)
~~~
