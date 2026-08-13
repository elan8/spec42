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
  (document "memory://snapshot/sys_ml.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 16) (end 6 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 16) (end 10 32))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 12 2) (end 16 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 18 2) (end 20 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 22 2) (end 24 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 26 2) (end 28 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 30 2) (end 32 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 34 2) (end 36 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 38 2) (end 40 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 42 2) (end 45 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 47 2) (end 49 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 51 2) (end 55 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 57 2) (end 57 68))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 59 2) (end 63 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 65 2) (end 65 86))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 67 2) (end 69 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 71 2) (end 73 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 75 2) (end 79 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 81 2) (end 86 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 88 2) (end 88 67))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 90 2) (end 92 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 94 2) (end 97 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 99 2) (end 102 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 104 2) (end 108 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 110 2) (end 112 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 114 2) (end 114 70))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 116 2) (end 116 80))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 118 2) (end 120 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 122 2) (end 122 60))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 124 2) (end 124 52))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 126 2) (end 160 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 162 2) (end 166 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 168 2) (end 170 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 172 2) (end 176 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 178 2) (end 180 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 182 2) (end 185 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 187 2) (end 189 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 191 2) (end 193 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 195 2) (end 198 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 200 2) (end 200 48))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 202 2) (end 207 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 209 2) (end 213 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 215 2) (end 217 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 219 2) (end 221 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 223 2) (end 225 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 227 2) (end 227 74))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 229 2) (end 231 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 233 2) (end 233 48))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 235 2) (end 237 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 239 2) (end 239 69))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 241 2) (end 241 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 243 2) (end 243 72))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 245 2) (end 247 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 249 2) (end 249 67))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 251 2) (end 253 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 255 2) (end 257 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 259 2) (end 265 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 267 2) (end 267 57))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 269 2) (end 271 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 273 2) (end 275 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 277 2) (end 280 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 282 2) (end 284 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 286 2) (end 288 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 290 2) (end 293 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 295 2) (end 297 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 299 2) (end 301 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 303 2) (end 305 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 307 2) (end 310 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 312 2) (end 317 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 319 2) (end 329 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 331 2) (end 342 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 344 2) (end 349 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 351 2) (end 354 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 356 2) (end 360 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 362 2) (end 364 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 366 2) (end 373 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 375 2) (end 379 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 381 2) (end 385 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 387 2) (end 394 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 396 2) (end 398 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 400 2) (end 400 74))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 402 2) (end 402 73))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 404 2) (end 406 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 408 2) (end 412 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 414 2) (end 418 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 420 2) (end 427 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 429 2) (end 431 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 433 2) (end 437 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 439 2) (end 478 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 480 2) (end 482 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 484 2) (end 487 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 489 2) (end 491 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 493 2) (end 495 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 497 2) (end 500 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 502 2) (end 507 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 509 2) (end 512 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 514 2) (end 520 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 522 2) (end 524 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 526 2) (end 529 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 531 2) (end 534 3))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:1aa7c1ab3e9cf268e133ef4f9ba7e93c050c2b6f0191aa10e3369505cb966516") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "Systems") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "KerML::Kernel") (import (shape namespace) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Systems")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "KerML::Kernel")
      (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sys_ml.md") (range (start 6 16) (end 6 31)) (probe (position 6 16))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 7 15) (end 7 25)) (probe (position 7 15))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "Systems")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 10 16) (end 10 32)) (probe (position 10 16))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "KerML::Kernel")
      (outcome (status unresolved)))
  )
)
~~~
