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
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 13 3) (end 13 88))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 14 3) (end 14 127))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 15 3) (end 15 87))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 18 44) (end 18 52))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 19 3) (end 19 109))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 22 39) (end 22 43))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 23 3) (end 23 135))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 26 43) (end 26 62))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 27 3) (end 27 117))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 31 3) (end 31 111))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 35 3) (end 35 141))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 39 3) (end 39 121))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 43 3) (end 43 131))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 44 3) (end 44 109))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 47 66) (end 47 75))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 48 3) (end 48 95))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 52 3) (end 52 86))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 53 3) (end 53 87))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 54 3) (end 54 92))
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
        (range (start 60 35) (end 60 42))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 60 59) (end 60 70))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 62 3) (end 62 118))
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
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 68 3) (end 68 126))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 71 44) (end 71 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 72 3) (end 72 136))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 76 3) (end 76 120))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 77 3) (end 77 108))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 78 3) (end 78 118))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 82 3) (end 82 120))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 83 3) (end 83 122))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 84 3) (end 84 108))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 85 3) (end 85 118))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 91 3) (end 91 128))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 95 3) (end 95 124))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 96 3) (end 96 122))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 99 48) (end 99 61))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 100 3) (end 100 117))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 101 3) (end 101 90))
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
        (range (start 105 28) (end 105 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 105 52) (end 105 64))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 107 3) (end 107 113))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 111 3) (end 111 155))
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
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 119 3) (end 119 111))
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
        (range (start 127 27) (end 127 34))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 129 3) (end 129 96))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 130 3) (end 130 116))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 131 3) (end 131 96))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 132 3) (end 132 119))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 133 3) (end 133 113))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 134 3) (end 134 117))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 135 3) (end 135 117))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 136 3) (end 136 125))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 137 3) (end 137 119))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 138 3) (end 138 112))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 139 3) (end 139 106))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 140 3) (end 140 107))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 141 3) (end 141 120))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 142 3) (end 142 104))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 143 3) (end 143 122))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 144 3) (end 144 124))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 145 3) (end 145 116))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 146 3) (end 146 110))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 147 3) (end 147 111))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 148 3) (end 148 122))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 149 3) (end 149 124))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 150 3) (end 150 126))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 151 3) (end 151 111))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 152 3) (end 152 113))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 153 3) (end 153 122))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 154 3) (end 154 130))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 155 3) (end 155 112))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 156 3) (end 156 106))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 157 3) (end 157 123))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 158 3) (end 158 116))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 159 3) (end 159 114))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 163 27) (end 163 34))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 163 51) (end 163 62))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 165 3) (end 165 121))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 169 3) (end 169 134))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 173 35) (end 173 42))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 173 59) (end 173 70))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 175 3) (end 175 92))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 179 3) (end 179 112))
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
        (range (start 183 26) (end 183 40))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 183 57) (end 183 67))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 184 27) (end 184 34))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 184 51) (end 184 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 187 42) (end 187 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 188 3) (end 188 99))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 191 55) (end 191 59))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 192 3) (end 192 135))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 196 3) (end 196 83))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 197 3) (end 197 88))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 203 62) (end 203 66))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 205 3) (end 205 108))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 206 3) (end 206 122))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 210 3) (end 210 83))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 211 3) (end 211 83))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 212 3) (end 212 82))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 216 3) (end 216 115))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 220 3) (end 220 115))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 224 3) (end 224 131))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 227 42) (end 227 51))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 230 3) (end 230 154))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 236 3) (end 236 83))
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
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 246 3) (end 246 125))
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
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 252 3) (end 252 128))
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
        (range (start 256 28) (end 256 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 260 28) (end 260 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 263 3) (end 263 116))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 264 3) (end 264 131))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 270 3) (end 270 121))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 274 3) (end 274 114))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 277 43) (end 277 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 278 3) (end 278 113))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 279 3) (end 279 131))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 282 64) (end 282 73))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 283 3) (end 283 130))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 287 3) (end 287 129))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 296 35) (end 296 42))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 296 59) (end 296 70))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 300 3) (end 300 109))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 304 3) (end 304 125))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 312 59) (end 312 76))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 315 3) (end 315 117))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 316 3) (end 316 97))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 320 21) (end 320 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 320 44) (end 320 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 321 28) (end 321 34))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 323 3) (end 323 108))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 324 3) (end 324 118))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 325 3) (end 325 124))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 326 3) (end 326 123))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 327 3) (end 327 124))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 328 3) (end 328 122))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 332 21) (end 332 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 332 44) (end 332 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 333 28) (end 333 34))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 335 3) (end 335 135))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 336 3) (end 336 124))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 337 3) (end 337 123))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 338 3) (end 338 108))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 339 3) (end 339 122))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 340 3) (end 340 118))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 341 3) (end 341 124))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 345 62) (end 345 66))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 347 3) (end 347 116))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 348 3) (end 348 128))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 352 3) (end 352 127))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 353 3) (end 353 86))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 357 3) (end 357 88))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 358 3) (end 358 87))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 359 3) (end 359 86))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 362 49) (end 362 68))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 363 3) (end 363 123))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 367 26) (end 367 33))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 369 3) (end 369 104))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 370 3) (end 370 84))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 371 3) (end 371 81))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 372 3) (end 372 83))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 381 52) (end 381 69))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 384 3) (end 384 106))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 388 26) (end 388 33))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 390 3) (end 390 120))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 391 3) (end 391 84))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 392 3) (end 392 81))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 393 3) (end 393 83))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 396 45) (end 396 64))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 397 3) (end 397 115))
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
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 405 3) (end 405 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 414 55) (end 414 72))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 417 3) (end 417 108))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 421 3) (end 421 79))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 422 3) (end 422 79))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 423 3) (end 423 113))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 424 3) (end 424 108))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 425 3) (end 425 101))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 426 3) (end 426 104))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 429 55) (end 429 75))
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
        (range (start 440 27) (end 440 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 441 35) (end 441 42))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 441 59) (end 441 69))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 442 35) (end 442 42))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 444 3) (end 444 96))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 445 3) (end 445 116))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 446 3) (end 446 107))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 447 3) (end 447 97))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 448 3) (end 448 105))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 449 3) (end 449 96))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 450 3) (end 450 119))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 451 3) (end 451 114))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 452 3) (end 452 119))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 453 3) (end 453 119))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 454 3) (end 454 127))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 455 3) (end 455 121))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 456 3) (end 456 114))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 457 3) (end 457 108))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 458 3) (end 458 109))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 459 3) (end 459 122))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 460 3) (end 460 106))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 461 3) (end 461 124))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 462 3) (end 462 126))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 463 3) (end 463 118))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 464 3) (end 464 112))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 465 3) (end 465 113))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 466 3) (end 466 124))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 467 3) (end 467 126))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 468 3) (end 468 128))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 469 3) (end 469 113))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 470 3) (end 470 115))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 471 3) (end 471 124))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 472 3) (end 472 132))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 473 3) (end 473 114))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 474 3) (end 474 108))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 475 3) (end 475 125))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 476 3) (end 476 118))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 477 3) (end 477 116))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 481 3) (end 481 97))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 485 3) (end 485 121))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 486 3) (end 486 97))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 489 45) (end 489 61))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 490 3) (end 490 109))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 494 3) (end 494 105))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 498 3) (end 498 137))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 499 3) (end 499 105))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 503 3) (end 503 99))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 504 3) (end 504 127))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 505 3) (end 505 89))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 506 3) (end 506 113))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 509 51) (end 509 68))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 510 3) (end 510 115))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 511 3) (end 511 95))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 515 3) (end 515 115))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 516 3) (end 516 128))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 517 3) (end 517 106))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 518 3) (end 518 89))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 519 3) (end 519 113))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 523 3) (end 523 99))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 527 3) (end 527 132))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 528 3) (end 528 99))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 532 3) (end 532 85))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 533 3) (end 533 85))
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
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AcceptActionUsage"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ActionUsage"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionDefinition"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Behavior")) (specialization (reference "OccurrenceDefinition"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionUsage"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Step")) (specialization (reference "OccurrenceUsage"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActorMembership"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ParameterMembership"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AllocationDefinition"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ConnectionDefinition"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AllocationUsage"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ConnectionUsage"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AnalysisCaseDefinition"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "CaseDefinition"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AnalysisCaseUsage"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "CaseUsage"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AssertConstraintUsage"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ConstraintUsage")) (specialization (reference "Invariant"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AssignmentActionUsage"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ActionUsage"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AttributeDefinition"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DataType")) (specialization (reference "Definition"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AttributeUsage"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Usage"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AttributeUsage::isReference"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean")) (redefinition (reference "isReference"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::BindingConnectorAsUsage"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "BindingConnector")) (specialization (reference "ConnectorAsUsage"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::CalculationDefinition"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Function")) (specialization (reference "ActionDefinition"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::CalculationUsage"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Expression")) (specialization (reference "ActionUsage"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::CaseDefinition"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "CalculationDefinition"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::CaseUsage"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "CalculationUsage"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConcernDefinition"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "RequirementDefinition"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConcernUsage"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "RequirementUsage"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConjugatedPortDefinition"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "PortDefinition"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConjugatedPortTyping"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "FeatureTyping"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectionDefinition"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "AssociationStructure")) (specialization (reference "PartDefinition"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectionDefinition::isSufficient"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean")) (redefinition (reference "isSufficient"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectionUsage"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ConnectorAsUsage")) (specialization (reference "PartUsage"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectorAsUsage"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Usage")) (specialization (reference "Connector"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConstraintDefinition"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "OccurrenceDefinition")) (specialization (reference "Predicate"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConstraintUsage"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "BooleanExpression")) (specialization (reference "OccurrenceUsage"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ControlNode"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ActionUsage"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::DecisionNode"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ControlNode"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Definition"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Classifier"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Definition::isVariation"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::EnumerationDefinition"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "AttributeDefinition"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::EnumerationDefinition::isVariation"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean")) (redefinition (reference "isVariation"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::EnumerationUsage"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "AttributeUsage"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::EventOccurrenceUsage"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "OccurrenceUsage"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::EventOccurrenceUsage::isReference"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean")) (redefinition (reference "isReference"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ExhibitStateUsage"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "StateUsage")) (specialization (reference "PerformActionUsage"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Expose"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Import"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Expose::isImportAll"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean")) (redefinition (reference "isImportAll"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Expose::visibility"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VisibilityKind")) (redefinition (reference "visibility"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::FlowDefinition"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Interaction")) (specialization (reference "ActionDefinition"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::FlowUsage"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ConnectorAsUsage")) (specialization (reference "Flow")) (specialization (reference "ActionUsage"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ForLoopActionUsage"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "LoopActionUsage"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ForkNode"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ControlNode"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::FramedConcernMembership"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "RequirementConstraintMembership"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::FramedConcernMembership::kind"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "RequirementConstraintKind")) (redefinition (reference "kind"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::IfActionUsage"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ActionUsage"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::IncludeUseCaseUsage"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "UseCaseUsage")) (specialization (reference "PerformActionUsage"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::InterfaceDefinition"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ConnectionDefinition"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::InterfaceUsage"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ConnectionUsage"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ItemDefinition"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Structure")) (specialization (reference "OccurrenceDefinition"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ItemUsage"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "OccurrenceUsage"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::JoinNode"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ControlNode"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::LoopActionUsage"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ActionUsage"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::MembershipExpose"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "MembershipImport")) (specialization (reference "Expose"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::MergeNode"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ControlNode"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::MetadataDefinition"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ItemDefinition")) (specialization (reference "Metaclass"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::MetadataUsage"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ItemUsage")) (specialization (reference "MetadataFeature"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::NamespaceExpose"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Expose")) (specialization (reference "NamespaceImport"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ObjectiveMembership"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "FeatureMembership"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceDefinition"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Definition")) (specialization (reference "Class"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceDefinition::isIndividual"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceUsage"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Usage"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceUsage::isIndividual"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceUsage::portionKind"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PortionKind"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PartDefinition"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ItemDefinition"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PartUsage"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ItemUsage"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PerformActionUsage"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ActionUsage")) (specialization (reference "EventOccurrenceUsage"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PortConjugation"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Conjugation"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PortDefinition"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "OccurrenceDefinition")) (specialization (reference "Structure"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PortUsage"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "OccurrenceUsage"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PortionKind"))) (kind enum-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PortionKind::snapshot"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PortionKind::timeslice"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ReferenceUsage"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Usage"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ReferenceUsage::isReference"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean")) (redefinition (reference "isReference"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RenderingDefinition"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "PartDefinition"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RenderingUsage"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "PartUsage"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementConstraintKind"))) (kind enum-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementConstraintKind::assumption"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementConstraintKind::requirement"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementConstraintMembership"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "FeatureMembership"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementConstraintMembership::kind"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "RequirementConstraintKind"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementDefinition"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ConstraintDefinition"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementDefinition::reqId"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String")) (redefinition (reference "declaredShortName"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementDefinition::text"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementUsage"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ConstraintUsage"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementUsage::reqId"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String")) (redefinition (reference "declaredShortName"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementUsage::text"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementVerificationMembership"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "RequirementConstraintMembership"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementVerificationMembership::kind"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "RequirementConstraintKind")) (redefinition (reference "kind"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::SatisfyRequirementUsage"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "RequirementUsage")) (specialization (reference "AssertConstraintUsage"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::SendActionUsage"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ActionUsage"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::StakeholderMembership"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ParameterMembership"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::StateDefinition"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ActionDefinition"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::StateDefinition::isParallel"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::StateSubactionKind"))) (kind enum-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::StateSubactionKind::do"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::StateSubactionKind::entry"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::StateSubactionKind::exit"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::StateSubactionMembership"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "FeatureMembership"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::StateSubactionMembership::kind"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "StateSubactionKind"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::StateUsage"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ActionUsage"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::StateUsage::isParallel"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::SubjectMembership"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ParameterMembership"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::SuccessionAsUsage"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ConnectorAsUsage")) (specialization (reference "Succession"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::SuccessionFlowUsage"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SuccessionFlow")) (specialization (reference "FlowUsage"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::TerminateActionUsage"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ActionUsage"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::TransitionFeatureKind"))) (kind enum-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::TransitionFeatureKind::effect"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::TransitionFeatureKind::guard"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::TransitionFeatureKind::trigger"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::TransitionFeatureMembership"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "FeatureMembership"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::TransitionFeatureMembership::kind"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TransitionFeatureKind"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::TransitionUsage"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ActionUsage"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::TriggerInvocationExpression"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "InvocationExpression"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::TriggerInvocationExpression::kind"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TriggerKind"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::TriggerKind"))) (kind enum-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::TriggerKind::after"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::TriggerKind::at"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::TriggerKind::when"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Usage"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Feature"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Usage::isReference"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Usage::isVariation"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Usage::mayTimeVary"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean")) (redefinition (reference "isVariable"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::UseCaseDefinition"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "CaseDefinition"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::UseCaseUsage"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "CaseUsage"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::VariantMembership"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "OwningMembership"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::VerificationCaseDefinition"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "CaseDefinition"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::VerificationCaseUsage"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "CaseUsage"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ViewDefinition"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "PartDefinition"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ViewRenderingMembership"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "FeatureMembership"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ViewUsage"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "PartUsage"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ViewpointDefinition"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "RequirementDefinition"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ViewpointUsage"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "RequirementUsage"))))
    (declaration (id (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::WhileLoopActionUsage"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "LoopActionUsage"))))
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
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AcceptActionUsage"))) (kind specialization) (ordinal 0))
      (authored-target "ActionUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionUsage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionDefinition"))) (kind specialization) (ordinal 0))
      (authored-target "Behavior")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionDefinition"))) (kind specialization) (ordinal 1))
      (authored-target "OccurrenceDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceDefinition")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionUsage"))) (kind specialization) (ordinal 0))
      (authored-target "Step")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionUsage"))) (kind specialization) (ordinal 1))
      (authored-target "OccurrenceUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceUsage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActorMembership"))) (kind specialization) (ordinal 0))
      (authored-target "ParameterMembership")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AllocationDefinition"))) (kind specialization) (ordinal 0))
      (authored-target "ConnectionDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectionDefinition")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AllocationUsage"))) (kind specialization) (ordinal 0))
      (authored-target "ConnectionUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectionUsage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AnalysisCaseDefinition"))) (kind specialization) (ordinal 0))
      (authored-target "CaseDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::CaseDefinition")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AnalysisCaseUsage"))) (kind specialization) (ordinal 0))
      (authored-target "CaseUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::CaseUsage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AssertConstraintUsage"))) (kind specialization) (ordinal 0))
      (authored-target "ConstraintUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConstraintUsage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AssertConstraintUsage"))) (kind specialization) (ordinal 1))
      (authored-target "Invariant")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AssignmentActionUsage"))) (kind specialization) (ordinal 0))
      (authored-target "ActionUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionUsage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AttributeDefinition"))) (kind specialization) (ordinal 0))
      (authored-target "DataType")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AttributeDefinition"))) (kind specialization) (ordinal 1))
      (authored-target "Definition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Definition")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AttributeUsage"))) (kind specialization) (ordinal 0))
      (authored-target "Usage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Usage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AttributeUsage::isReference"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AttributeUsage::isReference"))) (kind redefinition) (ordinal 0))
      (authored-target "isReference")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::BindingConnectorAsUsage"))) (kind specialization) (ordinal 0))
      (authored-target "BindingConnector")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::BindingConnectorAsUsage"))) (kind specialization) (ordinal 1))
      (authored-target "ConnectorAsUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectorAsUsage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::CalculationDefinition"))) (kind specialization) (ordinal 0))
      (authored-target "Function")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::CalculationDefinition"))) (kind specialization) (ordinal 1))
      (authored-target "ActionDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionDefinition")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::CalculationUsage"))) (kind specialization) (ordinal 0))
      (authored-target "Expression")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::CalculationUsage"))) (kind specialization) (ordinal 1))
      (authored-target "ActionUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionUsage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::CaseDefinition"))) (kind specialization) (ordinal 0))
      (authored-target "CalculationDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::CalculationDefinition")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::CaseUsage"))) (kind specialization) (ordinal 0))
      (authored-target "CalculationUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::CalculationUsage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConcernDefinition"))) (kind specialization) (ordinal 0))
      (authored-target "RequirementDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementDefinition")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConcernUsage"))) (kind specialization) (ordinal 0))
      (authored-target "RequirementUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementUsage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConjugatedPortDefinition"))) (kind specialization) (ordinal 0))
      (authored-target "PortDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PortDefinition")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConjugatedPortTyping"))) (kind specialization) (ordinal 0))
      (authored-target "FeatureTyping")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectionDefinition"))) (kind specialization) (ordinal 0))
      (authored-target "AssociationStructure")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectionDefinition"))) (kind specialization) (ordinal 1))
      (authored-target "PartDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PartDefinition")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectionDefinition::isSufficient"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectionDefinition::isSufficient"))) (kind redefinition) (ordinal 0))
      (authored-target "isSufficient")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectionUsage"))) (kind specialization) (ordinal 0))
      (authored-target "ConnectorAsUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectorAsUsage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectionUsage"))) (kind specialization) (ordinal 1))
      (authored-target "PartUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PartUsage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectorAsUsage"))) (kind specialization) (ordinal 0))
      (authored-target "Usage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Usage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectorAsUsage"))) (kind specialization) (ordinal 1))
      (authored-target "Connector")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConstraintDefinition"))) (kind specialization) (ordinal 0))
      (authored-target "OccurrenceDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceDefinition")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConstraintDefinition"))) (kind specialization) (ordinal 1))
      (authored-target "Predicate")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConstraintUsage"))) (kind specialization) (ordinal 0))
      (authored-target "BooleanExpression")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConstraintUsage"))) (kind specialization) (ordinal 1))
      (authored-target "OccurrenceUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceUsage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ControlNode"))) (kind specialization) (ordinal 0))
      (authored-target "ActionUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionUsage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::DecisionNode"))) (kind specialization) (ordinal 0))
      (authored-target "ControlNode")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ControlNode")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Definition"))) (kind specialization) (ordinal 0))
      (authored-target "Classifier")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Definition::isVariation"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::EnumerationDefinition"))) (kind specialization) (ordinal 0))
      (authored-target "AttributeDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AttributeDefinition")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::EnumerationDefinition::isVariation"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::EnumerationDefinition::isVariation"))) (kind redefinition) (ordinal 0))
      (authored-target "isVariation")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::EnumerationUsage"))) (kind specialization) (ordinal 0))
      (authored-target "AttributeUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AttributeUsage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::EventOccurrenceUsage"))) (kind specialization) (ordinal 0))
      (authored-target "OccurrenceUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceUsage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::EventOccurrenceUsage::isReference"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::EventOccurrenceUsage::isReference"))) (kind redefinition) (ordinal 0))
      (authored-target "isReference")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ExhibitStateUsage"))) (kind specialization) (ordinal 0))
      (authored-target "StateUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::StateUsage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ExhibitStateUsage"))) (kind specialization) (ordinal 1))
      (authored-target "PerformActionUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PerformActionUsage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Expose"))) (kind specialization) (ordinal 0))
      (authored-target "Import")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Expose::isImportAll"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Expose::isImportAll"))) (kind redefinition) (ordinal 0))
      (authored-target "isImportAll")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Expose::visibility"))) (kind featureTyping) (ordinal 0))
      (authored-target "VisibilityKind")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Expose::visibility"))) (kind redefinition) (ordinal 0))
      (authored-target "visibility")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::FlowDefinition"))) (kind specialization) (ordinal 0))
      (authored-target "Interaction")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::FlowDefinition"))) (kind specialization) (ordinal 1))
      (authored-target "ActionDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionDefinition")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::FlowUsage"))) (kind specialization) (ordinal 0))
      (authored-target "ConnectorAsUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectorAsUsage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::FlowUsage"))) (kind specialization) (ordinal 1))
      (authored-target "Flow")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::FlowUsage"))) (kind specialization) (ordinal 2))
      (authored-target "ActionUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionUsage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ForLoopActionUsage"))) (kind specialization) (ordinal 0))
      (authored-target "LoopActionUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::LoopActionUsage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ForkNode"))) (kind specialization) (ordinal 0))
      (authored-target "ControlNode")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ControlNode")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::FramedConcernMembership"))) (kind specialization) (ordinal 0))
      (authored-target "RequirementConstraintMembership")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementConstraintMembership")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::FramedConcernMembership::kind"))) (kind featureTyping) (ordinal 0))
      (authored-target "RequirementConstraintKind")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementConstraintKind")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::FramedConcernMembership::kind"))) (kind redefinition) (ordinal 0))
      (authored-target "kind")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::IfActionUsage"))) (kind specialization) (ordinal 0))
      (authored-target "ActionUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionUsage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::IncludeUseCaseUsage"))) (kind specialization) (ordinal 0))
      (authored-target "UseCaseUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::UseCaseUsage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::IncludeUseCaseUsage"))) (kind specialization) (ordinal 1))
      (authored-target "PerformActionUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PerformActionUsage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::InterfaceDefinition"))) (kind specialization) (ordinal 0))
      (authored-target "ConnectionDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectionDefinition")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::InterfaceUsage"))) (kind specialization) (ordinal 0))
      (authored-target "ConnectionUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectionUsage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ItemDefinition"))) (kind specialization) (ordinal 0))
      (authored-target "Structure")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ItemDefinition"))) (kind specialization) (ordinal 1))
      (authored-target "OccurrenceDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceDefinition")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ItemUsage"))) (kind specialization) (ordinal 0))
      (authored-target "OccurrenceUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceUsage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::JoinNode"))) (kind specialization) (ordinal 0))
      (authored-target "ControlNode")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ControlNode")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::LoopActionUsage"))) (kind specialization) (ordinal 0))
      (authored-target "ActionUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionUsage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::MembershipExpose"))) (kind specialization) (ordinal 0))
      (authored-target "MembershipImport")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::MembershipExpose"))) (kind specialization) (ordinal 1))
      (authored-target "Expose")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Expose")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::MergeNode"))) (kind specialization) (ordinal 0))
      (authored-target "ControlNode")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ControlNode")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::MetadataDefinition"))) (kind specialization) (ordinal 0))
      (authored-target "ItemDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ItemDefinition")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::MetadataDefinition"))) (kind specialization) (ordinal 1))
      (authored-target "Metaclass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::MetadataUsage"))) (kind specialization) (ordinal 0))
      (authored-target "ItemUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ItemUsage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::MetadataUsage"))) (kind specialization) (ordinal 1))
      (authored-target "MetadataFeature")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::NamespaceExpose"))) (kind specialization) (ordinal 0))
      (authored-target "Expose")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Expose")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::NamespaceExpose"))) (kind specialization) (ordinal 1))
      (authored-target "NamespaceImport")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ObjectiveMembership"))) (kind specialization) (ordinal 0))
      (authored-target "FeatureMembership")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceDefinition"))) (kind specialization) (ordinal 0))
      (authored-target "Definition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Definition")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceDefinition"))) (kind specialization) (ordinal 1))
      (authored-target "Class")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceDefinition::isIndividual"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceUsage"))) (kind specialization) (ordinal 0))
      (authored-target "Usage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Usage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceUsage::isIndividual"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceUsage::portionKind"))) (kind featureTyping) (ordinal 0))
      (authored-target "PortionKind")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PortionKind")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PartDefinition"))) (kind specialization) (ordinal 0))
      (authored-target "ItemDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ItemDefinition")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PartUsage"))) (kind specialization) (ordinal 0))
      (authored-target "ItemUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ItemUsage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PerformActionUsage"))) (kind specialization) (ordinal 0))
      (authored-target "ActionUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionUsage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PerformActionUsage"))) (kind specialization) (ordinal 1))
      (authored-target "EventOccurrenceUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::EventOccurrenceUsage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PortConjugation"))) (kind specialization) (ordinal 0))
      (authored-target "Conjugation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PortDefinition"))) (kind specialization) (ordinal 0))
      (authored-target "OccurrenceDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceDefinition")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PortDefinition"))) (kind specialization) (ordinal 1))
      (authored-target "Structure")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PortUsage"))) (kind specialization) (ordinal 0))
      (authored-target "OccurrenceUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceUsage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ReferenceUsage"))) (kind specialization) (ordinal 0))
      (authored-target "Usage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Usage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ReferenceUsage::isReference"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ReferenceUsage::isReference"))) (kind redefinition) (ordinal 0))
      (authored-target "isReference")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RenderingDefinition"))) (kind specialization) (ordinal 0))
      (authored-target "PartDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PartDefinition")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RenderingUsage"))) (kind specialization) (ordinal 0))
      (authored-target "PartUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PartUsage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementConstraintMembership"))) (kind specialization) (ordinal 0))
      (authored-target "FeatureMembership")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementConstraintMembership::kind"))) (kind featureTyping) (ordinal 0))
      (authored-target "RequirementConstraintKind")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementConstraintKind")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementDefinition"))) (kind specialization) (ordinal 0))
      (authored-target "ConstraintDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConstraintDefinition")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementDefinition::reqId"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementDefinition::reqId"))) (kind redefinition) (ordinal 0))
      (authored-target "declaredShortName")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementDefinition::text"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementUsage"))) (kind specialization) (ordinal 0))
      (authored-target "ConstraintUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConstraintUsage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementUsage::reqId"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementUsage::reqId"))) (kind redefinition) (ordinal 0))
      (authored-target "declaredShortName")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementUsage::text"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementVerificationMembership"))) (kind specialization) (ordinal 0))
      (authored-target "RequirementConstraintMembership")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementConstraintMembership")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementVerificationMembership::kind"))) (kind featureTyping) (ordinal 0))
      (authored-target "RequirementConstraintKind")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementConstraintKind")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementVerificationMembership::kind"))) (kind redefinition) (ordinal 0))
      (authored-target "kind")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::SatisfyRequirementUsage"))) (kind specialization) (ordinal 0))
      (authored-target "RequirementUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementUsage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::SatisfyRequirementUsage"))) (kind specialization) (ordinal 1))
      (authored-target "AssertConstraintUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AssertConstraintUsage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::SendActionUsage"))) (kind specialization) (ordinal 0))
      (authored-target "ActionUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionUsage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::StakeholderMembership"))) (kind specialization) (ordinal 0))
      (authored-target "ParameterMembership")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::StateDefinition"))) (kind specialization) (ordinal 0))
      (authored-target "ActionDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionDefinition")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::StateDefinition::isParallel"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::StateSubactionMembership"))) (kind specialization) (ordinal 0))
      (authored-target "FeatureMembership")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::StateSubactionMembership::kind"))) (kind featureTyping) (ordinal 0))
      (authored-target "StateSubactionKind")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::StateSubactionKind")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::StateUsage"))) (kind specialization) (ordinal 0))
      (authored-target "ActionUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionUsage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::StateUsage::isParallel"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::SubjectMembership"))) (kind specialization) (ordinal 0))
      (authored-target "ParameterMembership")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::SuccessionAsUsage"))) (kind specialization) (ordinal 0))
      (authored-target "ConnectorAsUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectorAsUsage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::SuccessionAsUsage"))) (kind specialization) (ordinal 1))
      (authored-target "Succession")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::SuccessionFlowUsage"))) (kind specialization) (ordinal 0))
      (authored-target "SuccessionFlow")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::SuccessionFlowUsage"))) (kind specialization) (ordinal 1))
      (authored-target "FlowUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::FlowUsage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::TerminateActionUsage"))) (kind specialization) (ordinal 0))
      (authored-target "ActionUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionUsage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::TransitionFeatureMembership"))) (kind specialization) (ordinal 0))
      (authored-target "FeatureMembership")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::TransitionFeatureMembership::kind"))) (kind featureTyping) (ordinal 0))
      (authored-target "TransitionFeatureKind")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::TransitionFeatureKind")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::TransitionUsage"))) (kind specialization) (ordinal 0))
      (authored-target "ActionUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionUsage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::TriggerInvocationExpression"))) (kind specialization) (ordinal 0))
      (authored-target "InvocationExpression")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::TriggerInvocationExpression::kind"))) (kind featureTyping) (ordinal 0))
      (authored-target "TriggerKind")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::TriggerKind")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Usage"))) (kind specialization) (ordinal 0))
      (authored-target "Feature")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Usage::isReference"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Usage::isVariation"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Usage::mayTimeVary"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Usage::mayTimeVary"))) (kind redefinition) (ordinal 0))
      (authored-target "isVariable")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::UseCaseDefinition"))) (kind specialization) (ordinal 0))
      (authored-target "CaseDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::CaseDefinition")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::UseCaseUsage"))) (kind specialization) (ordinal 0))
      (authored-target "CaseUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::CaseUsage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::VariantMembership"))) (kind specialization) (ordinal 0))
      (authored-target "OwningMembership")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::VerificationCaseDefinition"))) (kind specialization) (ordinal 0))
      (authored-target "CaseDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::CaseDefinition")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::VerificationCaseUsage"))) (kind specialization) (ordinal 0))
      (authored-target "CaseUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::CaseUsage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ViewDefinition"))) (kind specialization) (ordinal 0))
      (authored-target "PartDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PartDefinition")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ViewRenderingMembership"))) (kind specialization) (ordinal 0))
      (authored-target "FeatureMembership")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ViewUsage"))) (kind specialization) (ordinal 0))
      (authored-target "PartUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PartUsage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ViewpointDefinition"))) (kind specialization) (ordinal 0))
      (authored-target "RequirementDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementDefinition")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ViewpointUsage"))) (kind specialization) (ordinal 0))
      (authored-target "RequirementUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementUsage")))))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::WhileLoopActionUsage"))) (kind specialization) (ordinal 0))
      (authored-target "LoopActionUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::LoopActionUsage")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AcceptActionUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AcceptActionUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionDefinition"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceDefinition"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionDefinition"))) (kind specialization) (ordinal 1)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionUsage"))) (kind specialization) (ordinal 1)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AllocationDefinition"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectionDefinition"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AllocationDefinition"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AllocationUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectionUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AllocationUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AnalysisCaseDefinition"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::CaseDefinition"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AnalysisCaseDefinition"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AnalysisCaseUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::CaseUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AnalysisCaseUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AssertConstraintUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConstraintUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AssertConstraintUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AssignmentActionUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AssignmentActionUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AttributeDefinition"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Definition"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AttributeDefinition"))) (kind specialization) (ordinal 1)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AttributeUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Usage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AttributeUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::BindingConnectorAsUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectorAsUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::BindingConnectorAsUsage"))) (kind specialization) (ordinal 1)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::CalculationDefinition"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionDefinition"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::CalculationDefinition"))) (kind specialization) (ordinal 1)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::CalculationUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::CalculationUsage"))) (kind specialization) (ordinal 1)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::CaseDefinition"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::CalculationDefinition"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::CaseDefinition"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::CaseUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::CalculationUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::CaseUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConcernDefinition"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementDefinition"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConcernDefinition"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConcernUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConcernUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConjugatedPortDefinition"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PortDefinition"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConjugatedPortDefinition"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectionDefinition"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PartDefinition"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectionDefinition"))) (kind specialization) (ordinal 1)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectionUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectorAsUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectionUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectionUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PartUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectionUsage"))) (kind specialization) (ordinal 1)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectorAsUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Usage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectorAsUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConstraintDefinition"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceDefinition"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConstraintDefinition"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConstraintUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConstraintUsage"))) (kind specialization) (ordinal 1)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ControlNode"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ControlNode"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::DecisionNode"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ControlNode"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::DecisionNode"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::EnumerationDefinition"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AttributeDefinition"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::EnumerationDefinition"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::EnumerationUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AttributeUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::EnumerationUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::EventOccurrenceUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::EventOccurrenceUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ExhibitStateUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::StateUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ExhibitStateUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ExhibitStateUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PerformActionUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ExhibitStateUsage"))) (kind specialization) (ordinal 1)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::FlowDefinition"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionDefinition"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::FlowDefinition"))) (kind specialization) (ordinal 1)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::FlowUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectorAsUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::FlowUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::FlowUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::FlowUsage"))) (kind specialization) (ordinal 2)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ForLoopActionUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::LoopActionUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ForLoopActionUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ForkNode"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ControlNode"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ForkNode"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::FramedConcernMembership"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementConstraintMembership"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::FramedConcernMembership"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::FramedConcernMembership::kind"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementConstraintKind"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::FramedConcernMembership::kind"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::IfActionUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::IfActionUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::IncludeUseCaseUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::UseCaseUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::IncludeUseCaseUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::IncludeUseCaseUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PerformActionUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::IncludeUseCaseUsage"))) (kind specialization) (ordinal 1)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::InterfaceDefinition"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectionDefinition"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::InterfaceDefinition"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::InterfaceUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectionUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::InterfaceUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ItemDefinition"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceDefinition"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ItemDefinition"))) (kind specialization) (ordinal 1)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ItemUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ItemUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::JoinNode"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ControlNode"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::JoinNode"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::LoopActionUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::LoopActionUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::MembershipExpose"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Expose"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::MembershipExpose"))) (kind specialization) (ordinal 1)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::MergeNode"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ControlNode"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::MergeNode"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::MetadataDefinition"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ItemDefinition"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::MetadataDefinition"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::MetadataUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ItemUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::MetadataUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::NamespaceExpose"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Expose"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::NamespaceExpose"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceDefinition"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Definition"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceDefinition"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Usage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceUsage::portionKind"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PortionKind"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceUsage::portionKind"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PartDefinition"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ItemDefinition"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PartDefinition"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PartUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ItemUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PartUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PerformActionUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PerformActionUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PerformActionUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::EventOccurrenceUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PerformActionUsage"))) (kind specialization) (ordinal 1)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PortDefinition"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceDefinition"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PortDefinition"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PortUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PortUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ReferenceUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Usage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ReferenceUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RenderingDefinition"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PartDefinition"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RenderingDefinition"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RenderingUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PartUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RenderingUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementConstraintMembership::kind"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementConstraintKind"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementConstraintMembership::kind"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementDefinition"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConstraintDefinition"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementDefinition"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConstraintUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementVerificationMembership"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementConstraintMembership"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementVerificationMembership"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementVerificationMembership::kind"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementConstraintKind"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementVerificationMembership::kind"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::SatisfyRequirementUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::SatisfyRequirementUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::SatisfyRequirementUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AssertConstraintUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::SatisfyRequirementUsage"))) (kind specialization) (ordinal 1)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::SendActionUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::SendActionUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::StateDefinition"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionDefinition"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::StateDefinition"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::StateSubactionMembership::kind"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::StateSubactionKind"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::StateSubactionMembership::kind"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::StateUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::StateUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::SuccessionAsUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectorAsUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::SuccessionAsUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::SuccessionFlowUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::FlowUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::SuccessionFlowUsage"))) (kind specialization) (ordinal 1)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::TerminateActionUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::TerminateActionUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::TransitionFeatureMembership::kind"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::TransitionFeatureKind"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::TransitionFeatureMembership::kind"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::TransitionUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::TransitionUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::TriggerInvocationExpression::kind"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::TriggerKind"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::TriggerInvocationExpression::kind"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::UseCaseDefinition"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::CaseDefinition"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::UseCaseDefinition"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::UseCaseUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::CaseUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::UseCaseUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::VerificationCaseDefinition"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::CaseDefinition"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::VerificationCaseDefinition"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::VerificationCaseUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::CaseUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::VerificationCaseUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ViewDefinition"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PartDefinition"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ViewDefinition"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ViewUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PartUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ViewUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ViewpointDefinition"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementDefinition"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ViewpointDefinition"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ViewpointUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ViewpointUsage"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::WhileLoopActionUsage"))) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::LoopActionUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::WhileLoopActionUsage"))) (kind specialization) (ordinal 0)))
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
  (query (document "memory://snapshot/sys_ml.md") (range (start 12 45) (end 12 56)) (probe (position 12 45))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AcceptActionUsage"))) (kind specialization) (ordinal 0) (authored-target "ActionUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionUsage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 18 44) (end 18 52)) (probe (position 18 44))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionDefinition"))) (kind specialization) (ordinal 0) (authored-target "Behavior")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 18 54) (end 18 74)) (probe (position 18 54))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionDefinition"))) (kind specialization) (ordinal 1) (authored-target "OccurrenceDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceDefinition")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 22 39) (end 22 43)) (probe (position 22 39))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionUsage"))) (kind specialization) (ordinal 0) (authored-target "Step")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 22 45) (end 22 60)) (probe (position 22 45))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionUsage"))) (kind specialization) (ordinal 1) (authored-target "OccurrenceUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceUsage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 26 43) (end 26 62)) (probe (position 26 43))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActorMembership"))) (kind specialization) (ordinal 0) (authored-target "ParameterMembership")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 30 48) (end 30 68)) (probe (position 30 48))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AllocationDefinition"))) (kind specialization) (ordinal 0) (authored-target "ConnectionDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectionDefinition")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 34 43) (end 34 58)) (probe (position 34 43))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AllocationUsage"))) (kind specialization) (ordinal 0) (authored-target "ConnectionUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectionUsage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 38 50) (end 38 64)) (probe (position 38 50))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AnalysisCaseDefinition"))) (kind specialization) (ordinal 0) (authored-target "CaseDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::CaseDefinition")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 42 45) (end 42 54)) (probe (position 42 45))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AnalysisCaseUsage"))) (kind specialization) (ordinal 0) (authored-target "CaseUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::CaseUsage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 47 49) (end 47 64)) (probe (position 47 49))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AssertConstraintUsage"))) (kind specialization) (ordinal 0) (authored-target "ConstraintUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConstraintUsage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 47 66) (end 47 75)) (probe (position 47 66))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AssertConstraintUsage"))) (kind specialization) (ordinal 1) (authored-target "Invariant")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 51 49) (end 51 60)) (probe (position 51 49))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AssignmentActionUsage"))) (kind specialization) (ordinal 0) (authored-target "ActionUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionUsage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 57 47) (end 57 55)) (probe (position 57 47))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AttributeDefinition"))) (kind specialization) (ordinal 0) (authored-target "DataType")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 57 57) (end 57 67)) (probe (position 57 57))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AttributeDefinition"))) (kind specialization) (ordinal 1) (authored-target "Definition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Definition")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 59 42) (end 59 47)) (probe (position 59 42))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AttributeUsage"))) (kind specialization) (ordinal 0) (authored-target "Usage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Usage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 60 35) (end 60 42)) (probe (position 60 35))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AttributeUsage::isReference"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 60 59) (end 60 70)) (probe (position 60 59))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AttributeUsage::isReference"))) (kind redefinition) (ordinal 0) (authored-target "isReference")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 65 51) (end 65 67)) (probe (position 65 51))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::BindingConnectorAsUsage"))) (kind specialization) (ordinal 0) (authored-target "BindingConnector")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 65 69) (end 65 85)) (probe (position 65 69))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::BindingConnectorAsUsage"))) (kind specialization) (ordinal 1) (authored-target "ConnectorAsUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectorAsUsage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 67 49) (end 67 57)) (probe (position 67 49))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::CalculationDefinition"))) (kind specialization) (ordinal 0) (authored-target "Function")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 67 59) (end 67 75)) (probe (position 67 59))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::CalculationDefinition"))) (kind specialization) (ordinal 1) (authored-target "ActionDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionDefinition")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 71 44) (end 71 54)) (probe (position 71 44))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::CalculationUsage"))) (kind specialization) (ordinal 0) (authored-target "Expression")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 71 56) (end 71 67)) (probe (position 71 56))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::CalculationUsage"))) (kind specialization) (ordinal 1) (authored-target "ActionUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionUsage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 75 42) (end 75 63)) (probe (position 75 42))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::CaseDefinition"))) (kind specialization) (ordinal 0) (authored-target "CalculationDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::CalculationDefinition")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 81 37) (end 81 53)) (probe (position 81 37))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::CaseUsage"))) (kind specialization) (ordinal 0) (authored-target "CalculationUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::CalculationUsage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 88 45) (end 88 66)) (probe (position 88 45))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConcernDefinition"))) (kind specialization) (ordinal 0) (authored-target "RequirementDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementDefinition")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 90 40) (end 90 56)) (probe (position 90 40))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConcernUsage"))) (kind specialization) (ordinal 0) (authored-target "RequirementUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementUsage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 94 52) (end 94 66)) (probe (position 94 52))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConjugatedPortDefinition"))) (kind specialization) (ordinal 0) (authored-target "PortDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PortDefinition")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 99 48) (end 99 61)) (probe (position 99 48))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConjugatedPortTyping"))) (kind specialization) (ordinal 0) (authored-target "FeatureTyping")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 104 48) (end 104 68)) (probe (position 104 48))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectionDefinition"))) (kind specialization) (ordinal 0) (authored-target "AssociationStructure")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 104 70) (end 104 84)) (probe (position 104 70))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectionDefinition"))) (kind specialization) (ordinal 1) (authored-target "PartDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PartDefinition")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 105 28) (end 105 35)) (probe (position 105 28))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectionDefinition::isSufficient"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 105 52) (end 105 64)) (probe (position 105 52))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectionDefinition::isSufficient"))) (kind redefinition) (ordinal 0) (authored-target "isSufficient")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 110 43) (end 110 59)) (probe (position 110 43))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectionUsage"))) (kind specialization) (ordinal 0) (authored-target "ConnectorAsUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectorAsUsage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 110 61) (end 110 70)) (probe (position 110 61))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectionUsage"))) (kind specialization) (ordinal 1) (authored-target "PartUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PartUsage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 114 53) (end 114 58)) (probe (position 114 53))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectorAsUsage"))) (kind specialization) (ordinal 0) (authored-target "Usage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Usage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 114 60) (end 114 69)) (probe (position 114 60))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectorAsUsage"))) (kind specialization) (ordinal 1) (authored-target "Connector")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 116 48) (end 116 68)) (probe (position 116 48))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConstraintDefinition"))) (kind specialization) (ordinal 0) (authored-target "OccurrenceDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceDefinition")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 116 70) (end 116 79)) (probe (position 116 70))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConstraintDefinition"))) (kind specialization) (ordinal 1) (authored-target "Predicate")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 118 43) (end 118 60)) (probe (position 118 43))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConstraintUsage"))) (kind specialization) (ordinal 0) (authored-target "BooleanExpression")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 118 62) (end 118 77)) (probe (position 118 62))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConstraintUsage"))) (kind specialization) (ordinal 1) (authored-target "OccurrenceUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceUsage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 122 48) (end 122 59)) (probe (position 122 48))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ControlNode"))) (kind specialization) (ordinal 0) (authored-target "ActionUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionUsage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 124 40) (end 124 51)) (probe (position 124 40))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::DecisionNode"))) (kind specialization) (ordinal 0) (authored-target "ControlNode")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ControlNode")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 126 38) (end 126 48)) (probe (position 126 38))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Definition"))) (kind specialization) (ordinal 0) (authored-target "Classifier")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 127 27) (end 127 34)) (probe (position 127 27))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Definition::isVariation"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 162 49) (end 162 68)) (probe (position 162 49))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::EnumerationDefinition"))) (kind specialization) (ordinal 0) (authored-target "AttributeDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AttributeDefinition")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 163 27) (end 163 34)) (probe (position 163 27))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::EnumerationDefinition::isVariation"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 163 51) (end 163 62)) (probe (position 163 51))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::EnumerationDefinition::isVariation"))) (kind redefinition) (ordinal 0) (authored-target "isVariation")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 168 44) (end 168 58)) (probe (position 168 44))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::EnumerationUsage"))) (kind specialization) (ordinal 0) (authored-target "AttributeUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AttributeUsage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 172 48) (end 172 63)) (probe (position 172 48))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::EventOccurrenceUsage"))) (kind specialization) (ordinal 0) (authored-target "OccurrenceUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceUsage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 173 35) (end 173 42)) (probe (position 173 35))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::EventOccurrenceUsage::isReference"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 173 59) (end 173 70)) (probe (position 173 59))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::EventOccurrenceUsage::isReference"))) (kind redefinition) (ordinal 0) (authored-target "isReference")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 178 45) (end 178 55)) (probe (position 178 45))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ExhibitStateUsage"))) (kind specialization) (ordinal 0) (authored-target "StateUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::StateUsage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 178 57) (end 178 75)) (probe (position 178 57))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ExhibitStateUsage"))) (kind specialization) (ordinal 1) (authored-target "PerformActionUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PerformActionUsage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 182 43) (end 182 49)) (probe (position 182 43))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Expose"))) (kind specialization) (ordinal 0) (authored-target "Import")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 184 27) (end 184 34)) (probe (position 184 27))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Expose::isImportAll"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 184 51) (end 184 62)) (probe (position 184 51))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Expose::isImportAll"))) (kind redefinition) (ordinal 0) (authored-target "isImportAll")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 183 26) (end 183 40)) (probe (position 183 26))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Expose::visibility"))) (kind featureTyping) (ordinal 0) (authored-target "VisibilityKind")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 183 57) (end 183 67)) (probe (position 183 57))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Expose::visibility"))) (kind redefinition) (ordinal 0) (authored-target "visibility")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 187 42) (end 187 53)) (probe (position 187 42))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::FlowDefinition"))) (kind specialization) (ordinal 0) (authored-target "Interaction")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 187 55) (end 187 71)) (probe (position 187 55))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::FlowDefinition"))) (kind specialization) (ordinal 1) (authored-target "ActionDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionDefinition")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 191 37) (end 191 53)) (probe (position 191 37))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::FlowUsage"))) (kind specialization) (ordinal 0) (authored-target "ConnectorAsUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectorAsUsage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 191 55) (end 191 59)) (probe (position 191 55))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::FlowUsage"))) (kind specialization) (ordinal 1) (authored-target "Flow")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 191 61) (end 191 72)) (probe (position 191 61))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::FlowUsage"))) (kind specialization) (ordinal 2) (authored-target "ActionUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionUsage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 195 46) (end 195 61)) (probe (position 195 46))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ForLoopActionUsage"))) (kind specialization) (ordinal 0) (authored-target "LoopActionUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::LoopActionUsage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 200 36) (end 200 47)) (probe (position 200 36))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ForkNode"))) (kind specialization) (ordinal 0) (authored-target "ControlNode")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ControlNode")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 202 51) (end 202 82)) (probe (position 202 51))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::FramedConcernMembership"))) (kind specialization) (ordinal 0) (authored-target "RequirementConstraintMembership")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementConstraintMembership")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 203 20) (end 203 45)) (probe (position 203 20))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::FramedConcernMembership::kind"))) (kind featureTyping) (ordinal 0) (authored-target "RequirementConstraintKind")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementConstraintKind")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 203 62) (end 203 66)) (probe (position 203 62))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::FramedConcernMembership::kind"))) (kind redefinition) (ordinal 0) (authored-target "kind")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 209 41) (end 209 52)) (probe (position 209 41))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::IfActionUsage"))) (kind specialization) (ordinal 0) (authored-target "ActionUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionUsage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 215 47) (end 215 59)) (probe (position 215 47))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::IncludeUseCaseUsage"))) (kind specialization) (ordinal 0) (authored-target "UseCaseUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::UseCaseUsage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 215 61) (end 215 79)) (probe (position 215 61))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::IncludeUseCaseUsage"))) (kind specialization) (ordinal 1) (authored-target "PerformActionUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PerformActionUsage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 219 47) (end 219 67)) (probe (position 219 47))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::InterfaceDefinition"))) (kind specialization) (ordinal 0) (authored-target "ConnectionDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectionDefinition")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 223 42) (end 223 57)) (probe (position 223 42))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::InterfaceUsage"))) (kind specialization) (ordinal 0) (authored-target "ConnectionUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectionUsage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 227 42) (end 227 51)) (probe (position 227 42))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ItemDefinition"))) (kind specialization) (ordinal 0) (authored-target "Structure")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 227 53) (end 227 73)) (probe (position 227 53))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ItemDefinition"))) (kind specialization) (ordinal 1) (authored-target "OccurrenceDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceDefinition")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 229 37) (end 229 52)) (probe (position 229 37))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ItemUsage"))) (kind specialization) (ordinal 0) (authored-target "OccurrenceUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceUsage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 233 36) (end 233 47)) (probe (position 233 36))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::JoinNode"))) (kind specialization) (ordinal 0) (authored-target "ControlNode")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ControlNode")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 235 52) (end 235 63)) (probe (position 235 52))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::LoopActionUsage"))) (kind specialization) (ordinal 0) (authored-target "ActionUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionUsage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 239 44) (end 239 60)) (probe (position 239 44))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::MembershipExpose"))) (kind specialization) (ordinal 0) (authored-target "MembershipImport")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 239 62) (end 239 68)) (probe (position 239 62))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::MembershipExpose"))) (kind specialization) (ordinal 1) (authored-target "Expose")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Expose")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 241 37) (end 241 48)) (probe (position 241 37))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::MergeNode"))) (kind specialization) (ordinal 0) (authored-target "ControlNode")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ControlNode")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 243 46) (end 243 60)) (probe (position 243 46))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::MetadataDefinition"))) (kind specialization) (ordinal 0) (authored-target "ItemDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ItemDefinition")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 243 62) (end 243 71)) (probe (position 243 62))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::MetadataDefinition"))) (kind specialization) (ordinal 1) (authored-target "Metaclass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 245 41) (end 245 50)) (probe (position 245 41))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::MetadataUsage"))) (kind specialization) (ordinal 0) (authored-target "ItemUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ItemUsage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 245 52) (end 245 67)) (probe (position 245 52))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::MetadataUsage"))) (kind specialization) (ordinal 1) (authored-target "MetadataFeature")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 249 43) (end 249 49)) (probe (position 249 43))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::NamespaceExpose"))) (kind specialization) (ordinal 0) (authored-target "Expose")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Expose")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 249 51) (end 249 66)) (probe (position 249 51))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::NamespaceExpose"))) (kind specialization) (ordinal 1) (authored-target "NamespaceImport")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 251 47) (end 251 64)) (probe (position 251 47))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ObjectiveMembership"))) (kind specialization) (ordinal 0) (authored-target "FeatureMembership")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 255 48) (end 255 58)) (probe (position 255 48))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceDefinition"))) (kind specialization) (ordinal 0) (authored-target "Definition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Definition")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 255 60) (end 255 65)) (probe (position 255 60))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceDefinition"))) (kind specialization) (ordinal 1) (authored-target "Class")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 256 28) (end 256 35)) (probe (position 256 28))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceDefinition::isIndividual"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 259 43) (end 259 48)) (probe (position 259 43))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceUsage"))) (kind specialization) (ordinal 0) (authored-target "Usage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Usage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 260 28) (end 260 35)) (probe (position 260 28))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceUsage::isIndividual"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 261 27) (end 261 38)) (probe (position 261 27))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceUsage::portionKind"))) (kind featureTyping) (ordinal 0) (authored-target "PortionKind")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PortionKind")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 267 42) (end 267 56)) (probe (position 267 42))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PartDefinition"))) (kind specialization) (ordinal 0) (authored-target "ItemDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ItemDefinition")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 269 37) (end 269 46)) (probe (position 269 37))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PartUsage"))) (kind specialization) (ordinal 0) (authored-target "ItemUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ItemUsage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 273 46) (end 273 57)) (probe (position 273 46))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PerformActionUsage"))) (kind specialization) (ordinal 0) (authored-target "ActionUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionUsage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 273 59) (end 273 79)) (probe (position 273 59))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PerformActionUsage"))) (kind specialization) (ordinal 1) (authored-target "EventOccurrenceUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::EventOccurrenceUsage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 277 43) (end 277 54)) (probe (position 277 43))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PortConjugation"))) (kind specialization) (ordinal 0) (authored-target "Conjugation")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 282 42) (end 282 62)) (probe (position 282 42))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PortDefinition"))) (kind specialization) (ordinal 0) (authored-target "OccurrenceDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceDefinition")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 282 64) (end 282 73)) (probe (position 282 64))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PortDefinition"))) (kind specialization) (ordinal 1) (authored-target "Structure")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 286 37) (end 286 52)) (probe (position 286 37))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PortUsage"))) (kind specialization) (ordinal 0) (authored-target "OccurrenceUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::OccurrenceUsage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 295 42) (end 295 47)) (probe (position 295 42))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ReferenceUsage"))) (kind specialization) (ordinal 0) (authored-target "Usage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Usage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 296 35) (end 296 42)) (probe (position 296 35))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ReferenceUsage::isReference"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 296 59) (end 296 70)) (probe (position 296 59))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ReferenceUsage::isReference"))) (kind redefinition) (ordinal 0) (authored-target "isReference")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 299 47) (end 299 61)) (probe (position 299 47))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RenderingDefinition"))) (kind specialization) (ordinal 0) (authored-target "PartDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PartDefinition")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 303 42) (end 303 51)) (probe (position 303 42))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RenderingUsage"))) (kind specialization) (ordinal 0) (authored-target "PartUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PartUsage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 312 59) (end 312 76)) (probe (position 312 59))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementConstraintMembership"))) (kind specialization) (ordinal 0) (authored-target "FeatureMembership")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 313 20) (end 313 45)) (probe (position 313 20))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementConstraintMembership::kind"))) (kind featureTyping) (ordinal 0) (authored-target "RequirementConstraintKind")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementConstraintKind")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 319 49) (end 319 69)) (probe (position 319 49))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementDefinition"))) (kind specialization) (ordinal 0) (authored-target "ConstraintDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConstraintDefinition")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 320 21) (end 320 27)) (probe (position 320 21))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementDefinition::reqId"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 320 44) (end 320 61)) (probe (position 320 44))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementDefinition::reqId"))) (kind redefinition) (ordinal 0) (authored-target "declaredShortName")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 321 28) (end 321 34)) (probe (position 321 28))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementDefinition::text"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 331 44) (end 331 59)) (probe (position 331 44))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementUsage"))) (kind specialization) (ordinal 0) (authored-target "ConstraintUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConstraintUsage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 332 21) (end 332 27)) (probe (position 332 21))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementUsage::reqId"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 332 44) (end 332 61)) (probe (position 332 44))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementUsage::reqId"))) (kind redefinition) (ordinal 0) (authored-target "declaredShortName")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 333 28) (end 333 34)) (probe (position 333 28))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementUsage::text"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 344 61) (end 344 92)) (probe (position 344 61))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementVerificationMembership"))) (kind specialization) (ordinal 0) (authored-target "RequirementConstraintMembership")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementConstraintMembership")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 345 20) (end 345 45)) (probe (position 345 20))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementVerificationMembership::kind"))) (kind featureTyping) (ordinal 0) (authored-target "RequirementConstraintKind")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementConstraintKind")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 345 62) (end 345 66)) (probe (position 345 62))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementVerificationMembership::kind"))) (kind redefinition) (ordinal 0) (authored-target "kind")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 351 51) (end 351 67)) (probe (position 351 51))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::SatisfyRequirementUsage"))) (kind specialization) (ordinal 0) (authored-target "RequirementUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementUsage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 351 69) (end 351 90)) (probe (position 351 69))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::SatisfyRequirementUsage"))) (kind specialization) (ordinal 1) (authored-target "AssertConstraintUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::AssertConstraintUsage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 356 43) (end 356 54)) (probe (position 356 43))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::SendActionUsage"))) (kind specialization) (ordinal 0) (authored-target "ActionUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionUsage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 362 49) (end 362 68)) (probe (position 362 49))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::StakeholderMembership"))) (kind specialization) (ordinal 0) (authored-target "ParameterMembership")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 366 43) (end 366 59)) (probe (position 366 43))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::StateDefinition"))) (kind specialization) (ordinal 0) (authored-target "ActionDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionDefinition")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 367 26) (end 367 33)) (probe (position 367 26))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::StateDefinition::isParallel"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 381 52) (end 381 69)) (probe (position 381 52))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::StateSubactionMembership"))) (kind specialization) (ordinal 0) (authored-target "FeatureMembership")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 382 20) (end 382 38)) (probe (position 382 20))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::StateSubactionMembership::kind"))) (kind featureTyping) (ordinal 0) (authored-target "StateSubactionKind")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::StateSubactionKind")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 387 38) (end 387 49)) (probe (position 387 38))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::StateUsage"))) (kind specialization) (ordinal 0) (authored-target "ActionUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionUsage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 388 26) (end 388 33)) (probe (position 388 26))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::StateUsage::isParallel"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 396 45) (end 396 64)) (probe (position 396 45))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::SubjectMembership"))) (kind specialization) (ordinal 0) (authored-target "ParameterMembership")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 400 45) (end 400 61)) (probe (position 400 45))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::SuccessionAsUsage"))) (kind specialization) (ordinal 0) (authored-target "ConnectorAsUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ConnectorAsUsage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 400 63) (end 400 73)) (probe (position 400 63))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::SuccessionAsUsage"))) (kind specialization) (ordinal 1) (authored-target "Succession")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 402 47) (end 402 61)) (probe (position 402 47))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::SuccessionFlowUsage"))) (kind specialization) (ordinal 0) (authored-target "SuccessionFlow")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 402 63) (end 402 72)) (probe (position 402 63))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::SuccessionFlowUsage"))) (kind specialization) (ordinal 1) (authored-target "FlowUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::FlowUsage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 404 48) (end 404 59)) (probe (position 404 48))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::TerminateActionUsage"))) (kind specialization) (ordinal 0) (authored-target "ActionUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionUsage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 414 55) (end 414 72)) (probe (position 414 55))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::TransitionFeatureMembership"))) (kind specialization) (ordinal 0) (authored-target "FeatureMembership")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 415 20) (end 415 41)) (probe (position 415 20))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::TransitionFeatureMembership::kind"))) (kind featureTyping) (ordinal 0) (authored-target "TransitionFeatureKind")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::TransitionFeatureKind")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 420 43) (end 420 54)) (probe (position 420 43))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::TransitionUsage"))) (kind specialization) (ordinal 0) (authored-target "ActionUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ActionUsage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 429 55) (end 429 75)) (probe (position 429 55))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::TriggerInvocationExpression"))) (kind specialization) (ordinal 0) (authored-target "InvocationExpression")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 430 20) (end 430 31)) (probe (position 430 20))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::TriggerInvocationExpression::kind"))) (kind featureTyping) (ordinal 0) (authored-target "TriggerKind")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::TriggerKind")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 439 33) (end 439 40)) (probe (position 439 33))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Usage"))) (kind specialization) (ordinal 0) (authored-target "Feature")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 442 35) (end 442 42)) (probe (position 442 35))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Usage::isReference"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 440 27) (end 440 34)) (probe (position 440 27))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Usage::isVariation"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 441 35) (end 441 42)) (probe (position 441 35))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Usage::mayTimeVary"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 441 59) (end 441 69)) (probe (position 441 59))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::Usage::mayTimeVary"))) (kind redefinition) (ordinal 0) (authored-target "isVariable")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 480 45) (end 480 59)) (probe (position 480 45))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::UseCaseDefinition"))) (kind specialization) (ordinal 0) (authored-target "CaseDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::CaseDefinition")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 484 40) (end 484 49)) (probe (position 484 40))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::UseCaseUsage"))) (kind specialization) (ordinal 0) (authored-target "CaseUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::CaseUsage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 489 45) (end 489 61)) (probe (position 489 45))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::VariantMembership"))) (kind specialization) (ordinal 0) (authored-target "OwningMembership")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 493 54) (end 493 68)) (probe (position 493 54))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::VerificationCaseDefinition"))) (kind specialization) (ordinal 0) (authored-target "CaseDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::CaseDefinition")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 497 49) (end 497 58)) (probe (position 497 49))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::VerificationCaseUsage"))) (kind specialization) (ordinal 0) (authored-target "CaseUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::CaseUsage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 502 42) (end 502 56)) (probe (position 502 42))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ViewDefinition"))) (kind specialization) (ordinal 0) (authored-target "PartDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PartDefinition")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 509 51) (end 509 68)) (probe (position 509 51))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ViewRenderingMembership"))) (kind specialization) (ordinal 0) (authored-target "FeatureMembership")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 514 37) (end 514 46)) (probe (position 514 37))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ViewUsage"))) (kind specialization) (ordinal 0) (authored-target "PartUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::PartUsage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 522 47) (end 522 68)) (probe (position 522 47))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ViewpointDefinition"))) (kind specialization) (ordinal 0) (authored-target "RequirementDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementDefinition")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 526 42) (end 526 58)) (probe (position 526 42))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::ViewpointUsage"))) (kind specialization) (ordinal 0) (authored-target "RequirementUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::RequirementUsage")))))
  )
  (query (document "memory://snapshot/sys_ml.md") (range (start 531 48) (end 531 63)) (probe (position 531 48))
    (reference (id (source (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::WhileLoopActionUsage"))) (kind specialization) (ordinal 0) (authored-target "LoopActionUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sys_ml.md") (qualified-name "SysML::Systems::LoopActionUsage")))))
  )
)
~~~
