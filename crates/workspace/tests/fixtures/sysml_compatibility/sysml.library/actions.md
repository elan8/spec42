# META
~~~ini
description=Standard Library: Systems Library/Actions
type=file
~~~
# SOURCE
~~~sysml
standard library package Actions {
	doc
	/*
	 * This package defines the base types for actions and related behavioral elements in the
	 * SysML language.
	 */

	private import Base::Anything;
	private import ScalarValues::Positive;
	private import ScalarValues::Natural;
	private import SequenceFunctions::size;
	private import SequenceFunctions::isEmpty;
	private import Occurrences::Occurrence;
	private import Occurrences::HappensWhile;
	private import Performances::Performance;
	private import Performances::performances;
	private import Transfers::SendPerformance;
	private import Transfers::sendPerformances;
	private import Transfers::AcceptPerformance;
	private import Transfers::acceptPerformances;
	private import FeatureReferencingPerformances::FeatureWritePerformance;
	private import ControlPerformances::MergePerformance;
	private import ControlPerformances::DecisionPerformance;
	private import ControlPerformances::IfThenPerformance;
	private import ControlPerformances::IfThenElsePerformance;
	private import ControlPerformances::LoopPerformance;
	private import TransitionPerformances::TransitionPerformance;
	private import TransitionPerformances::NonStateTransitionPerformance;
	private import Transfers::MessageTransfer;
	private import Flows::MessageAction;
	private import OccurrenceFunctions::destroy;
	
	abstract action def Action :> Performance {
		doc
		/*
		 * Action is the most general class of Performances of ActionDefinitions in a system or 
		 * part of a system. Action is the base class of all ActionDefinitions.
		 */
	
		ref action self: Action :>> Performance::self;
		ref action incomingTransfers :>> Performance::incomingTransfers;
		
		action start: Action :>> startShot {
			doc
			/*
			 * The starting snapshot of an Action. 
			 */
		}
		
		action done: Action :>> endShot {
			doc
			/*
			 * The ending snapshot of an Action.
			 */
		}

		action subactions: Action[0..*] :> actions, subperformances {
			doc
			/*
			 * The subperformances of this Action that are Actions. 
			 */
		
			ref occurrence :>> Action::this, actions::this, subperformances::this = (that as Action).this {
				doc
				/*
				 * The "this" reference of a subaction is always the same as that of
				 * its owning Action.
				 */
			}
		}
	
		action sendSubactions: SendAction[0..*] :> subactions, sendActions {
			doc
			/*
			 * The subactions of this Action that are SendActions. 
			 */
		}
	
		action acceptSubactions: AcceptAction[0..*] :> subactions, acceptActions {
			doc
			/*
			 * The subactions of this Action that are AcceptActions. 
			 */
		}
		
		abstract action terminateSubactions : TerminateAction[0..*] :> subactions, terminateActions {
			doc
			/*
			 * The subactions of this Action that are TerminateActions.
			 */
		}
		
		abstract action controls : ControlAction[0..*] :> subactions {
			doc
			/*
			 * The subactions of this Action that are ControlActions.
			 */
		}
		
		abstract action merges : MergeAction[0..*] :> controls {
			doc
			/*
			 * The controls of this Action that are MergeActions.
			 */
		}
		
		abstract action decisions : DecisionAction :> controls {
			doc
			/*
			 * The controls of this Action that are DecisionActions.
			 */
		}
		
		abstract action joins : JoinAction :> controls {
			doc
			/*
			 * The controls of this Action that are JoinActions.
			 */
		}
		
		abstract action forks : ForkAction :> controls {
			doc
			/*
			 * The controls of this Action that are ForkActions.
			 */
		}
		
		abstract action transitions : TransitionAction[0..*] :> subactions, transitionActions {
			doc
			/*
			 * The subactions of this Action that are TransitionActions. 
			 */
		}
		
		abstract action decisionTransitions : DecisionTransitionAction[0..*] :> transitions {
			doc
			/*
			 * The subactions of this Action that are DecisionTransitionActions. 
			 */
		}
		
		abstract action assignments : AssignmentAction[0..*] :> subactions, assignmentActions {
			doc
			/*
			 * The subactions of this Action that are AssignmentActions.
			 */
			 
			 in target;
		}
		
		abstract action ifSubactions : IfThenAction[0..*] :> subactions, ifThenActions {
			doc
			/*
			 * The subactions of this Action that are IfThenActions (including IfThenElseActions).
			 */
		}
		
		abstract action loops : LoopAction[0..*] :> subactions, loopActions {
			doc
			/*
			 * The subactions of this Action that are LoopActions.
			 */
		}
		
		abstract action whileLoops : WhileLoopAction[0..*] :> loops, whileLoopActions {
			doc
			/*
			 * The loops of this Action that are WhileLoopActions.
			 */
		}
		
		abstract action forLoops : ForLoopAction[0..*] :> loops, forLoopActions {
			doc
			/*
			 * The loops of this Action that are ForLoopActions.
			 */
		}
	}
	
	abstract action actions: Action[0..*] nonunique :> performances {
		doc
		/*
		 * actions is the base feature for all ActionUsages.
		 */
	}
	
	action def SendAction :> Action, SendPerformance {
		doc
		/*
		 * A SendAction is an Action used to type SendActionUsages. It initiates an outgoingTransferFromSelf 
		 * from a designated sender Occurrence with a given payload, optionally to a designated receiver.
		 */
	
		in :>> payload [0..*];
	    ref sentMessage :>> sentTransfer: MessageTransfer, MessageAction {
	        in :>> MessageTransfer::payload, MessageAction::payload;
	    }
	}
	
	abstract action sendActions: SendAction[0..*] nonunique :> actions, sendPerformances {
		doc
		/*
		 * sendActions is the base feature for all SendActionUsages.
		 */
	}
	
	action def AcceptMessageAction :> Action, AcceptPerformance {
		doc
		/*
		 * An AcceptMessageAction is an Action that identifies an incomingTransferToSelf
		 * of a designated receiver Occurrence, providing its payload as output.
		 */
		inout :>> payload;
		ref acceptedMessage :>> acceptedTransfer: MessageTransfer, MessageAction {
            in :>> MessageTransfer::payload, MessageAction::payload;
        }
	}
	
	action def AcceptAction :> AcceptMessageAction {
		doc
		/*
		 * An AcceptAction is an AcceptMessageAction used to type AcceptActionUsages that are
		 * not accepters for TransitionActions. It waits for a payload or message of the specified 
		 * kind to be accepted by a nested state transition.
		 */
		ref :>> acceptedMessage = aState.aTransition.accepter.acceptedMessage;
		state aState  {
			transition aTransition first start accept apayload: Anything via receiver then done;
		}
		bind payload = aState.aTransition.apayload;
	}
	
	abstract action acceptActions: AcceptAction[0..*] nonunique :> actions, acceptPerformances {
		doc
		/*
		 * acceptActions is the base feature for standalone AcceptActionUsages.
		 */
	}
	
	abstract action def TerminateAction :> Action {
		doc
		/*
		 * A TerminateAction is an Action that terminates a given Occurrence, meaning 
		 * that the Occurrence ends during the performance of this Action. TerminateAction
		 * is the base type for all TerminateActionUsages.
		 */
		 
		in occurrence terminatedOccurrence[1] {
			doc
			/*
			 * The Occurrence to be terminated.
			 */
		}
		
		action terminateOccurrence : destroy[1] {
			in occ = terminatedOccurrence;
		}
	}
	
	abstract action terminateActions : TerminateAction[0..*] nonunique :> actions {
		doc
		/*
		 * terminateActions is the base feature for all TerminateActionUsages.
		 */
		 
		in occurrence terminatedOccurrence default that as Occurrence {
			doc
			/*
			 * The default terminatedOccurrence for a terminateAction is its
			 * featuring occurrence (which will generally be a containing Action).
			 */
		}
	}
	
	abstract action def ControlAction :> Action {
		doc
		/*
		 * A ControlAction is the Action of a control node, which has no inherent behavior.
		 */
	
		bind start = done {
			doc
			/*
			 * A ControlAction is instantaneous.
			 */
		}
	}
	
	action def MergeAction :> ControlAction, MergePerformance {
		doc
		/*
		 * A MergeAction is the ControlAction for a merge node.
		 * 
		 * Note: Incoming succession connectors to a MergeAction must have source multiplicity 
		 * 0..1 and subset the incomingHBLink feature inherited from MergePerformance.
		 */
	}
	
	action def DecisionAction :> ControlAction, DecisionPerformance {
		doc
		/*
		 * A DecisionAction is the ControlAction for a decision node.
		 * 
		 * Note: Outgoing succession connectors from a DecisionAction must have target multiplicity
		 * 0..1 and subset the outgoingHBLink feature inherited from DecisionPerformance.
		 * If an outgoing succession has a guard, it should have a transitionStep typed by 
		 * DecisionTransition.
		 */
	}
	
	action def JoinAction :> ControlAction {
		doc
		/*
		 * A JoinAction is the ControlAction for a JoinNode.
		 * 
		 * Note: Join behavior results from requiring that the source multiplicity of all
		 * incoming succession connectors be 1..1.
		 */
	}
	
	action def ForkAction :> ControlAction {
		doc
		/*
		 * A ForkAction is the ControlAction for a ForkNode.
		 * 
		 * Note: Fork behavior results from requiring that the target multiplicity of all
		 * outgoing succession connectors be 1..1.
		 */
	}
	
	abstract action def TransitionAction :> Action, TransitionPerformance {
		doc
		/*
		 * A TransitionAction is a TransitionPerformance with an Action as transitionLinkSource.
		 * It is the base type of all TransitionUsages.
		 */
	
		in transitionLinkSource : Action :>> TransitionPerformance::transitionLinkSource;
		ref acceptedMessage : MessageTransfer, MessageAction :>> trigger {
            in :>> MessageTransfer::payload, MessageAction::payload;
        }
		
		ref receiver :>> triggerTarget;

		action accepter : AcceptMessageAction :>> 'accept';
		
		bind receiver = accepter.receiver;
		bind acceptedMessage = accepter.acceptedMessage;
		
		action effect: Action :>> TransitionPerformance::effect;		
	}
	
	action def DecisionTransitionAction :> TransitionAction, NonStateTransitionPerformance {
		doc
		/*
		 * A DecisionTransitionAction is a TransitionAction and NonStateTransitionPerformance that has a 
		 * guard, but no trigger or effects. It is the base type of TransitionUsages used as 
		 * conditional successions in action models.
		 */
	
		ref action :>> accepter[0..0];
		ref action :>> effect[0..0];
	}

	abstract action transitionActions: TransitionAction[0..*] nonunique :> actions {
		doc
		/*
		 * transitionActions is the base feature for all TransitionUsages.
		 */
	}
	
	action def AssignmentAction :> FeatureWritePerformance, Action {
		doc
		/*
		 * An AssignmentAction is an Action, used to type an AssignmentActionUsage. It is also a
		 * FeatureWritePerformance that updates the accessedFeature of its target Occurrence with
		 * the given replacementValues.
		 */
	
		in target : Occurrence[1];
		inout replacementValues : Anything[0..*] nonunique;
	}
	
	abstract action assignmentActions : AssignmentAction[0..*] nonunique :> actions {
		doc
		/*
		 * assignmentActions is the base feature for all AssignmentActionsUsages.
		 */
		 
        in target : Occurrence[1] default that as Occurrence {
            doc
            /*
             * The default target for assignmentActions is its featuring instance (if that is 
             * an Occurrence).
             */
        }
	}
	
	action def IfThenAction :> Action, IfThenPerformance {
		doc
		/*
		 * An IfThenAction is a Kernel IfThenPerformance that is also an Action. 
		 * It is the base type for all IfActionUsages.
		 */
	
		in ifTest[1];
		in action thenClause[0..1];
	}
	
	action def IfThenElseAction :> IfThenAction, IfThenElsePerformance {
		doc
		/*
		 * An IfThenElseAction is a Kernel IfThenElsePeformance that is also an IfThenAction. 
		 * It is the base type for all IfActionUsages that have an elseAction.
		 */
	
		in ifTest[1];
		in action thenClause[0..1];
		in action elseClause[0..1];
	}
	
	abstract action ifThenActions : IfThenAction[0..*] nonunique :> actions {
		doc
		/*
		 * ifThenActions is the base feature for all IfActionUsages.
		 */
	}
	
	abstract action ifThenElseActions : IfThenElseAction[0..*] nonunique :> actions {
		doc
		/*
		 * ifThenElseActions is the base feature for all IfActionUsages that have an elseAction.
		 */
	}
	
	abstract action def LoopAction :> Action {
		doc
		/*
		 * A LoopAction is the base type for all LoopActionUsages.
		 */
	
        in ref iterator;
		
		in action body[0..*] {
			doc
			/*
			 * The action that is performed repeatedly in the loop.
			 */
		}		
	}
	
	action def WhileLoopAction :> LoopAction, LoopPerformance {
		doc
		/*
		 * A WhileLoopAction is a Kernel LoopPerformance that is also a LoopAction.
		 * It is the base type for all WhileLoopActionUsages.
		 */
	
		in whileTest default {true} {
			doc
			/*
			 * A Boolean expression that must be true for the loop to continue.
			 * It is evaluated before the body is performed and is always evaluated at 
			 * least once.
			 */
		}
		
		in action body {
			doc
			/*
			 * The action that is performed while the whileTest is true and the
			 * untilTest is false.
			 */
		}
		
		in untilTest default {false} {
			doc
			/*
			 * A Boolean expression that must be false for the loop to continue.
			 * It is evaluated after the body is performed.
			 */
		}
	}
	
	action def ForLoopAction :> LoopAction {
		doc
		/*
		 * A ForLoopAction is a LoopAction that iterates over an ordered sequence of values.
		 * It is the base type for all ForLoopActionUsages.
		 */
	
		protected ref var[0..1] :> seq {
			doc
			/*
			 * The loop variable that is assigned successive elements of seq on each
			 * iteration of the loop.
			 */
		}
		
		in ref seq {
			doc
			/*
			 * The sequence of values over which the loop iterates.
			 */
		}
		
		in action body {
			doc
			/*
			 * The action that is performed on each iteration of the loop.
			 */
		}
		
		private attribute index : Positive {
			doc
			/*
			 * The index of the element of seq assigned to var on the current iteration
			 * of the loop.
			 */
		}
		
		private action initialization
			assign index := 1;
		then private action whileLoop
			while index <= size(seq) {
				assign var := seq#(index);
				then perform body;
				then assign index := index + 1;
			}
	}
	
	abstract action loopActions : LoopAction[0..*] nonunique :> actions {
		doc
		/*
		 * loopActions is the base feature for all LoopActionUsages.
		 */
	}
	
	abstract action whileLoopActions : WhileLoopAction[0..*] nonunique :> loopActions {
		doc
		/*
		 * whileLoopActions is the base feature for all WhileLoopActionUsages.
		 */
	}
	
	abstract action forLoopActions : ForLoopAction[0..*] nonunique :> loopActions {
		doc
		/*
		 * forLoopActions is the base feature for all ForLoopActionUsages.
		 */
	}
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Performance'
semantic.unresolved_name 'Performance::self'
semantic.unresolved_name 'Performance::incomingTransfers'
semantic.unresolved_name 'startShot'
semantic.unresolved_name 'endShot'
semantic.unresolved_name 'subperformances'
semantic.unresolved_name 'Action::this'
semantic.unresolved_name 'actions::this'
semantic.unresolved_name 'subperformances::this'
semantic.unresolved_name 'performances'
semantic.unresolved_name 'SendPerformance'
semantic.unresolved_name 'payload'
semantic.unresolved_name 'sentTransfer'
semantic.unresolved_name 'MessageTransfer'
semantic.unresolved_name 'MessageAction'
semantic.unresolved_name 'MessageTransfer::payload'
semantic.unresolved_name 'MessageAction::payload'
semantic.unresolved_name 'sendPerformances'
semantic.unresolved_name 'AcceptPerformance'
semantic.unresolved_name 'payload'
semantic.unresolved_name 'acceptedTransfer'
semantic.unresolved_name 'MessageTransfer'
semantic.unresolved_name 'MessageAction'
semantic.unresolved_name 'MessageTransfer::payload'
semantic.unresolved_name 'MessageAction::payload'
semantic.unresolved_name 'acceptPerformances'
semantic.unresolved_name 'destroy'
semantic.unresolved_name 'MergePerformance'
semantic.unresolved_name 'DecisionPerformance'
semantic.unresolved_name 'TransitionPerformance'
semantic.unresolved_name 'TransitionPerformance::transitionLinkSource'
semantic.unresolved_name 'MessageTransfer'
semantic.unresolved_name 'MessageAction'
semantic.unresolved_name 'trigger'
semantic.unresolved_name 'MessageTransfer::payload'
semantic.unresolved_name 'MessageAction::payload'
semantic.unresolved_name 'triggerTarget'
semantic.unresolved_name 'accept'
semantic.unresolved_name 'TransitionPerformance::effect'
semantic.unresolved_name 'NonStateTransitionPerformance'
semantic.unresolved_name 'FeatureWritePerformance'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'IfThenPerformance'
semantic.unresolved_name 'IfThenElsePerformance'
semantic.unresolved_name 'LoopPerformance'
semantic.unresolved_name 'Positive'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Performance'
semantic.unresolved_name 'Performance::self'
semantic.unresolved_name 'Performance::incomingTransfers'
semantic.unresolved_name 'startShot'
semantic.unresolved_name 'endShot'
semantic.unresolved_name 'subperformances'
semantic.unresolved_name 'Action::this'
semantic.unresolved_name 'actions::this'
semantic.unresolved_name 'subperformances::this'
semantic.unresolved_name 'performances'
semantic.unresolved_name 'SendPerformance'
semantic.unresolved_name 'payload'
semantic.unresolved_name 'sentTransfer'
semantic.unresolved_name 'MessageTransfer'
semantic.unresolved_name 'MessageAction'
semantic.unresolved_name 'MessageTransfer::payload'
semantic.unresolved_name 'MessageAction::payload'
semantic.unresolved_name 'sendPerformances'
semantic.unresolved_name 'AcceptPerformance'
semantic.unresolved_name 'payload'
semantic.unresolved_name 'acceptedTransfer'
semantic.unresolved_name 'MessageTransfer'
semantic.unresolved_name 'MessageAction'
semantic.unresolved_name 'MessageTransfer::payload'
semantic.unresolved_name 'MessageAction::payload'
semantic.unresolved_name 'acceptPerformances'
semantic.unresolved_name 'destroy'
semantic.unresolved_name 'MergePerformance'
semantic.unresolved_name 'DecisionPerformance'
semantic.unresolved_name 'TransitionPerformance'
semantic.unresolved_name 'TransitionPerformance::transitionLinkSource'
semantic.unresolved_name 'MessageTransfer'
semantic.unresolved_name 'MessageAction'
semantic.unresolved_name 'trigger'
semantic.unresolved_name 'MessageTransfer::payload'
semantic.unresolved_name 'MessageAction::payload'
semantic.unresolved_name 'triggerTarget'
semantic.unresolved_name 'accept'
semantic.unresolved_name 'TransitionPerformance::effect'
semantic.unresolved_name 'NonStateTransitionPerformance'
semantic.unresolved_name 'FeatureWritePerformance'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'IfThenPerformance'
semantic.unresolved_name 'IfThenElsePerformance'
semantic.unresolved_name 'LoopPerformance'
semantic.unresolved_name 'Positive'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwAbstract,KwAction,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwRef,KwAction,Ident,Colon,Ident,ColonGtGt,Ident,ColonColon,Ident,Semicolon,
KwRef,KwAction,Ident,ColonGtGt,Ident,ColonColon,Ident,Semicolon,
KwAction,Ident,Colon,Ident,ColonGtGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAction,Ident,Colon,Ident,ColonGtGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAction,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
KwRef,KwOccurrence,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Eq,OpenParen,Ident,KwAs,Ident,CloseParen,Dot,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,
KwAction,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAction,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwAction,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwAction,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwAction,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwAction,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwAction,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwAction,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwAction,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwAction,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwAction,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Semicolon,
CloseCurly,
KwAbstract,KwAction,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwAction,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwAction,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwAction,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,
KwAbstract,KwAction,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAction,KwDef,Ident,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,ColonGtGt,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwRef,Ident,ColonGtGt,Ident,Colon,Ident,Comma,Ident,OpenCurly,
KwIn,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwAbstract,KwAction,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAction,KwDef,Ident,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
KwInout,ColonGtGt,Ident,Semicolon,
KwRef,Ident,ColonGtGt,Ident,Colon,Ident,Comma,Ident,OpenCurly,
KwIn,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwAction,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwRef,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwState,Ident,OpenCurly,
KwTransition,Ident,KwFirst,Ident,KwAccept,Ident,Colon,Ident,KwVia,Ident,KwThen,Ident,Semicolon,
CloseCurly,
KwBind,Ident,Eq,Ident,Dot,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwAbstract,KwAction,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwAction,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,KwOccurrence,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAction,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwIn,Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwAbstract,KwAction,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,KwOccurrence,Ident,KwDefault,Ident,KwAs,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,
KwAbstract,KwAction,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwBind,Ident,Eq,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,
KwAction,KwDef,Ident,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAction,KwDef,Ident,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAction,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAction,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwAction,KwDef,Ident,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,ColonGtGt,Ident,ColonColon,Ident,Semicolon,
KwRef,Ident,Colon,Ident,Comma,Ident,ColonGtGt,Ident,OpenCurly,
KwIn,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwRef,Ident,ColonGtGt,Ident,Semicolon,
KwAction,Ident,Colon,Ident,ColonGtGt,UnrestrictedName,Semicolon,
KwBind,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwBind,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwAction,Ident,Colon,Ident,ColonGtGt,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwAction,KwDef,Ident,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
KwRef,KwAction,ColonGtGt,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwRef,KwAction,ColonGtGt,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAbstract,KwAction,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAction,KwDef,Ident,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwInout,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,Semicolon,
CloseCurly,
KwAbstract,KwAction,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,Ident,KwAs,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,
KwAction,KwDef,Ident,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,KwAction,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAction,KwDef,Ident,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,KwAction,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwIn,KwAction,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAbstract,KwAction,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwAction,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwAction,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,KwRef,Ident,Semicolon,
KwIn,KwAction,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,
KwAction,KwDef,Ident,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,KwDefault,OpenCurly,KwTrue,CloseCurly,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwIn,KwAction,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwIn,Ident,KwDefault,OpenCurly,KwFalse,CloseCurly,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,
KwAction,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwProtected,KwRef,KwVar,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwIn,KwRef,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwIn,KwAction,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwPrivate,KwAction,Ident,
KwAssign,Ident,ColonEq,DecimalValue,Semicolon,
KwThen,KwPrivate,KwAction,Ident,
KwWhile,Ident,LtEq,Ident,OpenParen,Ident,CloseParen,OpenCurly,
KwAssign,KwVar,ColonEq,Ident,Hash,OpenParen,Ident,CloseParen,Semicolon,
KwThen,KwPerform,Ident,Semicolon,
KwThen,KwAssign,Ident,ColonEq,Ident,Plus,DecimalValue,Semicolon,
CloseCurly,
CloseCurly,
KwAbstract,KwAction,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwAction,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwAction,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'Actions'
    (documentation)
    (import_decl private 'Base::Anything')
    (import_decl private 'ScalarValues::Positive')
    (import_decl private 'ScalarValues::Natural')
    (import_decl private 'SequenceFunctions::size')
    (import_decl private 'SequenceFunctions::isEmpty')
    (import_decl private 'Occurrences::Occurrence')
    (import_decl private 'Occurrences::HappensWhile')
    (import_decl private 'Performances::Performance')
    (import_decl private 'Performances::performances')
    (import_decl private 'Transfers::SendPerformance')
    (import_decl private 'Transfers::sendPerformances')
    (import_decl private 'Transfers::AcceptPerformance')
    (import_decl private 'Transfers::acceptPerformances')
    (import_decl private 'FeatureReferencingPerformances::FeatureWritePerformance')
    (import_decl private 'ControlPerformances::MergePerformance')
    (import_decl private 'ControlPerformances::DecisionPerformance')
    (import_decl private 'ControlPerformances::IfThenPerformance')
    (import_decl private 'ControlPerformances::IfThenElsePerformance')
    (import_decl private 'ControlPerformances::LoopPerformance')
    (import_decl private 'TransitionPerformances::TransitionPerformance')
    (import_decl private 'TransitionPerformances::NonStateTransitionPerformance')
    (import_decl private 'Transfers::MessageTransfer')
    (import_decl private 'Flows::MessageAction')
    (import_decl private 'OccurrenceFunctions::destroy')
    (action_def abstract 'Action' :> 'Performance'
      (documentation)
      (action_usage ref 'self' : 'Action' :>> 'Performance::self')
      (action_usage ref 'incomingTransfers' :>> 'Performance::incomingTransfers')
      (action_usage 'start' : 'Action' :>> 'startShot'
        (documentation))
      (action_usage 'done' : 'Action' :>> 'endShot'
        (documentation))
      (action_usage 'subactions' : 'Action' multiplicity :> 'actions', 'subperformances'
        (documentation)
        (occurrence_usage ref :>> 'Action::this', 'actions::this', 'subperformances::this' value
          (documentation)))
      (action_usage 'sendSubactions' : 'SendAction' multiplicity :> 'subactions', 'sendActions'
        (documentation))
      (action_usage 'acceptSubactions' : 'AcceptAction' multiplicity :> 'subactions', 'acceptActions'
        (documentation))
      (action_usage abstract 'terminateSubactions' : 'TerminateAction' multiplicity :> 'subactions', 'terminateActions'
        (documentation))
      (action_usage abstract 'controls' : 'ControlAction' multiplicity :> 'subactions'
        (documentation))
      (action_usage abstract 'merges' : 'MergeAction' multiplicity :> 'controls'
        (documentation))
      (action_usage abstract 'decisions' : 'DecisionAction' :> 'controls'
        (documentation))
      (action_usage abstract 'joins' : 'JoinAction' :> 'controls'
        (documentation))
      (action_usage abstract 'forks' : 'ForkAction' :> 'controls'
        (documentation))
      (action_usage abstract 'transitions' : 'TransitionAction' multiplicity :> 'subactions', 'transitionActions'
        (documentation))
      (action_usage abstract 'decisionTransitions' : 'DecisionTransitionAction' multiplicity :> 'transitions'
        (documentation))
      (action_usage abstract 'assignments' : 'AssignmentAction' multiplicity :> 'subactions', 'assignmentActions'
        (documentation)
        (default_ref_usage in 'target'))
      (action_usage abstract 'ifSubactions' : 'IfThenAction' multiplicity :> 'subactions', 'ifThenActions'
        (documentation))
      (action_usage abstract 'loops' : 'LoopAction' multiplicity :> 'subactions', 'loopActions'
        (documentation))
      (action_usage abstract 'whileLoops' : 'WhileLoopAction' multiplicity :> 'loops', 'whileLoopActions'
        (documentation))
      (action_usage abstract 'forLoops' : 'ForLoopAction' multiplicity :> 'loops', 'forLoopActions'
        (documentation)))
    (action_usage abstract 'actions' : 'Action' multiplicity :> 'performances' nonunique
      (documentation))
    (action_def 'SendAction' :> 'Action', 'SendPerformance'
      (documentation)
      (default_ref_usage in :>> 'payload' multiplicity)
      (ref_usage ref 'sentMessage' :>> 'sentTransfer' : 'MessageTransfer', 'MessageAction'
        (default_ref_usage in :>> 'MessageTransfer::payload', 'MessageAction::payload')))
    (action_usage abstract 'sendActions' : 'SendAction' multiplicity :> 'actions', 'sendPerformances' nonunique
      (documentation))
    (action_def 'AcceptMessageAction' :> 'Action', 'AcceptPerformance'
      (documentation)
      (default_ref_usage inout :>> 'payload')
      (ref_usage ref 'acceptedMessage' :>> 'acceptedTransfer' : 'MessageTransfer', 'MessageAction'
        (default_ref_usage in :>> 'MessageTransfer::payload', 'MessageAction::payload')))
    (action_def 'AcceptAction' :> 'AcceptMessageAction'
      (documentation)
      (ref_usage ref :>> 'acceptedMessage' value)
      (state_usage 'aState'
        (transition_usage 'aTransition'))
      (binding_as_usage
        (connector_end)
        (connector_end)))
    (action_usage abstract 'acceptActions' : 'AcceptAction' multiplicity :> 'actions', 'acceptPerformances' nonunique
      (documentation))
    (action_def abstract 'TerminateAction' :> 'Action'
      (documentation)
      (occurrence_usage in 'terminatedOccurrence' multiplicity
        (documentation))
      (action_usage 'terminateOccurrence' : 'destroy' multiplicity
        (default_ref_usage in 'occ' value)))
    (action_usage abstract 'terminateActions' : 'TerminateAction' multiplicity :> 'actions' nonunique
      (documentation)
      (occurrence_usage in 'terminatedOccurrence' value
        (documentation)))
    (action_def abstract 'ControlAction' :> 'Action'
      (documentation)
      (binding_as_usage
        (connector_end)
        (connector_end)
        (documentation)))
    (action_def 'MergeAction' :> 'ControlAction', 'MergePerformance'
      (documentation))
    (action_def 'DecisionAction' :> 'ControlAction', 'DecisionPerformance'
      (documentation))
    (action_def 'JoinAction' :> 'ControlAction'
      (documentation))
    (action_def 'ForkAction' :> 'ControlAction'
      (documentation))
    (action_def abstract 'TransitionAction' :> 'Action', 'TransitionPerformance'
      (documentation)
      (default_ref_usage in 'transitionLinkSource' : 'Action' :>> 'TransitionPerformance::transitionLinkSource')
      (ref_usage ref 'acceptedMessage' : 'MessageTransfer', 'MessageAction' :>> 'trigger'
        (default_ref_usage in :>> 'MessageTransfer::payload', 'MessageAction::payload'))
      (ref_usage ref 'receiver' :>> 'triggerTarget')
      (action_usage 'accepter' : 'AcceptMessageAction' :>> ''accept'')
      (binding_as_usage
        (connector_end)
        (connector_end))
      (binding_as_usage
        (connector_end)
        (connector_end))
      (action_usage 'effect' : 'Action' :>> 'TransitionPerformance::effect'))
    (action_def 'DecisionTransitionAction' :> 'TransitionAction', 'NonStateTransitionPerformance'
      (documentation)
      (action_usage ref :>> 'accepter' multiplicity)
      (action_usage ref :>> 'effect' multiplicity))
    (action_usage abstract 'transitionActions' : 'TransitionAction' multiplicity :> 'actions' nonunique
      (documentation))
    (action_def 'AssignmentAction' :> 'FeatureWritePerformance', 'Action'
      (documentation)
      (default_ref_usage in 'target' : 'Occurrence' multiplicity)
      (default_ref_usage inout 'replacementValues' : 'Anything' multiplicity nonunique))
    (action_usage abstract 'assignmentActions' : 'AssignmentAction' multiplicity :> 'actions' nonunique
      (documentation)
      (default_ref_usage in 'target' : 'Occurrence' multiplicity value
        (documentation)))
    (action_def 'IfThenAction' :> 'Action', 'IfThenPerformance'
      (documentation)
      (default_ref_usage in 'ifTest' multiplicity)
      (action_usage in 'thenClause' multiplicity))
    (action_def 'IfThenElseAction' :> 'IfThenAction', 'IfThenElsePerformance'
      (documentation)
      (default_ref_usage in 'ifTest' multiplicity)
      (action_usage in 'thenClause' multiplicity)
      (action_usage in 'elseClause' multiplicity))
    (action_usage abstract 'ifThenActions' : 'IfThenAction' multiplicity :> 'actions' nonunique
      (documentation))
    (action_usage abstract 'ifThenElseActions' : 'IfThenElseAction' multiplicity :> 'actions' nonunique
      (documentation))
    (action_def abstract 'LoopAction' :> 'Action'
      (documentation)
      (ref_usage in ref 'iterator')
      (action_usage in 'body' multiplicity
        (documentation)))
    (action_def 'WhileLoopAction' :> 'LoopAction', 'LoopPerformance'
      (documentation)
      (default_ref_usage in 'whileTest' value
        (documentation))
      (action_usage in 'body'
        (documentation))
      (default_ref_usage in 'untilTest' value
        (documentation)))
    (action_def 'ForLoopAction' :> 'LoopAction'
      (documentation)
      (ref_usage protected ref 'var' :> 'seq' multiplicity
        (documentation))
      (ref_usage in ref 'seq'
        (documentation))
      (action_usage in 'body'
        (documentation))
      (attribute_usage private 'index' : 'Positive'
        (documentation))
      (action_usage private 'initialization')
      (assign_node)
      (source_succession
        (action_usage private 'whileLoop'))
      (while_loop_node))
    (action_usage abstract 'loopActions' : 'LoopAction' multiplicity :> 'actions' nonunique
      (documentation))
    (action_usage abstract 'whileLoopActions' : 'WhileLoopAction' multiplicity :> 'loopActions' nonunique
      (documentation))
    (action_usage abstract 'forLoopActions' : 'ForLoopAction' multiplicity :> 'loopActions' nonunique
      (documentation))))
~~~
# FORMAT
~~~sysml
standard library package Actions {
    doc
    /*
	 * This package defines the base types for actions and related behavioral elements in the
	 * SysML language.
	 */

    private import Base::Anything;
    private import ScalarValues::Positive;
    private import ScalarValues::Natural;
    private import SequenceFunctions::size;
    private import SequenceFunctions::isEmpty;
    private import Occurrences::Occurrence;
    private import Occurrences::HappensWhile;
    private import Performances::Performance;
    private import Performances::performances;
    private import Transfers::SendPerformance;
    private import Transfers::sendPerformances;
    private import Transfers::AcceptPerformance;
    private import Transfers::acceptPerformances;
    private import FeatureReferencingPerformances::FeatureWritePerformance;
    private import ControlPerformances::MergePerformance;
    private import ControlPerformances::DecisionPerformance;
    private import ControlPerformances::IfThenPerformance;
    private import ControlPerformances::IfThenElsePerformance;
    private import ControlPerformances::LoopPerformance;
    private import TransitionPerformances::TransitionPerformance;
    private import TransitionPerformances::NonStateTransitionPerformance;
    private import Transfers::MessageTransfer;
    private import Flows::MessageAction;
    private import OccurrenceFunctions::destroy;

    abstract action def Action :> Performance {
        doc
        /*
		 * Action is the most general class of Performances of ActionDefinitions in a system or 
		 * part of a system. Action is the base class of all ActionDefinitions.
		 */

        ref action self: Action :>> Performance::self;
        ref action incomingTransfers :>> Performance::incomingTransfers;

        action start: Action :>> startShot {
            doc
            /*
			 * The starting snapshot of an Action. 
			 */
        }

        action done: Action :>> endShot {
            doc
            /*
			 * The ending snapshot of an Action.
			 */
        }

        action subactions: Action[0..*] :> actions, subperformances {
            doc
            /*
			 * The subperformances of this Action that are Actions. 
			 */

            ref occurrence :>> Action::this, actions::this, subperformances::this = (that as Action).this {
                doc
                /*
				 * The "this" reference of a subaction is always the same as that of
				 * its owning Action.
				 */
            }
        }

        action sendSubactions: SendAction[0..*] :> subactions, sendActions {
            doc
            /*
			 * The subactions of this Action that are SendActions. 
			 */
        }

        action acceptSubactions: AcceptAction[0..*] :> subactions, acceptActions {
            doc
            /*
			 * The subactions of this Action that are AcceptActions. 
			 */
        }

        abstract action terminateSubactions : TerminateAction[0..*] :> subactions, terminateActions {
            doc
            /*
			 * The subactions of this Action that are TerminateActions.
			 */
        }

        abstract action controls : ControlAction[0..*] :> subactions {
            doc
            /*
			 * The subactions of this Action that are ControlActions.
			 */
        }

        abstract action merges : MergeAction[0..*] :> controls {
            doc
            /*
			 * The controls of this Action that are MergeActions.
			 */
        }

        abstract action decisions : DecisionAction :> controls {
            doc
            /*
			 * The controls of this Action that are DecisionActions.
			 */
        }

        abstract action joins : JoinAction :> controls {
            doc
            /*
			 * The controls of this Action that are JoinActions.
			 */
        }

        abstract action forks : ForkAction :> controls {
            doc
            /*
			 * The controls of this Action that are ForkActions.
			 */
        }

        abstract action transitions : TransitionAction[0..*] :> subactions, transitionActions {
            doc
            /*
			 * The subactions of this Action that are TransitionActions. 
			 */
        }

        abstract action decisionTransitions : DecisionTransitionAction[0..*] :> transitions {
            doc
            /*
			 * The subactions of this Action that are DecisionTransitionActions. 
			 */
        }

        abstract action assignments : AssignmentAction[0..*] :> subactions, assignmentActions {
            doc
            /*
			 * The subactions of this Action that are AssignmentActions.
			 */

            in target;
        }

        abstract action ifSubactions : IfThenAction[0..*] :> subactions, ifThenActions {
            doc
            /*
			 * The subactions of this Action that are IfThenActions (including IfThenElseActions).
			 */
        }

        abstract action loops : LoopAction[0..*] :> subactions, loopActions {
            doc
            /*
			 * The subactions of this Action that are LoopActions.
			 */
        }

        abstract action whileLoops : WhileLoopAction[0..*] :> loops, whileLoopActions {
            doc
            /*
			 * The loops of this Action that are WhileLoopActions.
			 */
        }

        abstract action forLoops : ForLoopAction[0..*] :> loops, forLoopActions {
            doc
            /*
			 * The loops of this Action that are ForLoopActions.
			 */
        }
    }

    abstract action actions: Action[0..*] nonunique :> performances {
        doc
        /*
		 * actions is the base feature for all ActionUsages.
		 */
    }

    action def SendAction :> Action, SendPerformance {
        doc
        /*
		 * A SendAction is an Action used to type SendActionUsages. It initiates an outgoingTransferFromSelf 
		 * from a designated sender Occurrence with a given payload, optionally to a designated receiver.
		 */

        in :>> payload [0..*];
        ref sentMessage :>> sentTransfer: MessageTransfer, MessageAction {
            in :>> MessageTransfer::payload, MessageAction::payload;
        }
    }

    abstract action sendActions: SendAction[0..*] nonunique :> actions, sendPerformances {
        doc
        /*
		 * sendActions is the base feature for all SendActionUsages.
		 */
    }

    action def AcceptMessageAction :> Action, AcceptPerformance {
        doc
        /*
		 * An AcceptMessageAction is an Action that identifies an incomingTransferToSelf
		 * of a designated receiver Occurrence, providing its payload as output.
		 */
        inout :>> payload;
        ref acceptedMessage :>> acceptedTransfer: MessageTransfer, MessageAction {
            in :>> MessageTransfer::payload, MessageAction::payload;
        }
    }

    action def AcceptAction :> AcceptMessageAction {
        doc
        /*
		 * An AcceptAction is an AcceptMessageAction used to type AcceptActionUsages that are
		 * not accepters for TransitionActions. It waits for a payload or message of the specified 
		 * kind to be accepted by a nested state transition.
		 */
        ref :>> acceptedMessage = aState.aTransition.accepter.acceptedMessage;
        state aState  {
            transition aTransition first start accept apayload: Anything via receiver then done;
        }
        bind payload = aState.aTransition.apayload;
    }

    abstract action acceptActions: AcceptAction[0..*] nonunique :> actions, acceptPerformances {
        doc
        /*
		 * acceptActions is the base feature for standalone AcceptActionUsages.
		 */
    }

    abstract action def TerminateAction :> Action {
        doc
        /*
		 * A TerminateAction is an Action that terminates a given Occurrence, meaning 
		 * that the Occurrence ends during the performance of this Action. TerminateAction
		 * is the base type for all TerminateActionUsages.
		 */

        in occurrence terminatedOccurrence[1] {
            doc
            /*
			 * The Occurrence to be terminated.
			 */
        }

        action terminateOccurrence : destroy[1] {
            in occ = terminatedOccurrence;
        }
    }

    abstract action terminateActions : TerminateAction[0..*] nonunique :> actions {
        doc
        /*
		 * terminateActions is the base feature for all TerminateActionUsages.
		 */

        in occurrence terminatedOccurrence default that as Occurrence {
            doc
            /*
			 * The default terminatedOccurrence for a terminateAction is its
			 * featuring occurrence (which will generally be a containing Action).
			 */
        }
    }

    abstract action def ControlAction :> Action {
        doc
        /*
		 * A ControlAction is the Action of a control node, which has no inherent behavior.
		 */

        bind start = done {
            doc
            /*
			 * A ControlAction is instantaneous.
			 */
        }
    }

    action def MergeAction :> ControlAction, MergePerformance {
        doc
        /*
		 * A MergeAction is the ControlAction for a merge node.
		 * 
		 * Note: Incoming succession connectors to a MergeAction must have source multiplicity 
		 * 0..1 and subset the incomingHBLink feature inherited from MergePerformance.
		 */
    }

    action def DecisionAction :> ControlAction, DecisionPerformance {
        doc
        /*
		 * A DecisionAction is the ControlAction for a decision node.
		 * 
		 * Note: Outgoing succession connectors from a DecisionAction must have target multiplicity
		 * 0..1 and subset the outgoingHBLink feature inherited from DecisionPerformance.
		 * If an outgoing succession has a guard, it should have a transitionStep typed by 
		 * DecisionTransition.
		 */
    }

    action def JoinAction :> ControlAction {
        doc
        /*
		 * A JoinAction is the ControlAction for a JoinNode.
		 * 
		 * Note: Join behavior results from requiring that the source multiplicity of all
		 * incoming succession connectors be 1..1.
		 */
    }

    action def ForkAction :> ControlAction {
        doc
        /*
		 * A ForkAction is the ControlAction for a ForkNode.
		 * 
		 * Note: Fork behavior results from requiring that the target multiplicity of all
		 * outgoing succession connectors be 1..1.
		 */
    }

    abstract action def TransitionAction :> Action, TransitionPerformance {
        doc
        /*
		 * A TransitionAction is a TransitionPerformance with an Action as transitionLinkSource.
		 * It is the base type of all TransitionUsages.
		 */

        in transitionLinkSource : Action :>> TransitionPerformance::transitionLinkSource;
        ref acceptedMessage : MessageTransfer, MessageAction :>> trigger {
            in :>> MessageTransfer::payload, MessageAction::payload;
        }

        ref receiver :>> triggerTarget;

        action accepter : AcceptMessageAction :>> 'accept';

        bind receiver = accepter.receiver;
        bind acceptedMessage = accepter.acceptedMessage;

        action effect: Action :>> TransitionPerformance::effect;
    }

    action def DecisionTransitionAction :> TransitionAction, NonStateTransitionPerformance {
        doc
        /*
		 * A DecisionTransitionAction is a TransitionAction and NonStateTransitionPerformance that has a 
		 * guard, but no trigger or effects. It is the base type of TransitionUsages used as 
		 * conditional successions in action models.
		 */

        ref action :>> accepter[0..0];
        ref action :>> effect[0..0];
    }

    abstract action transitionActions: TransitionAction[0..*] nonunique :> actions {
        doc
        /*
		 * transitionActions is the base feature for all TransitionUsages.
		 */
    }

    action def AssignmentAction :> FeatureWritePerformance, Action {
        doc
        /*
		 * An AssignmentAction is an Action, used to type an AssignmentActionUsage. It is also a
		 * FeatureWritePerformance that updates the accessedFeature of its target Occurrence with
		 * the given replacementValues.
		 */

        in target : Occurrence[1];
        inout replacementValues : Anything[0..*] nonunique;
    }

    abstract action assignmentActions : AssignmentAction[0..*] nonunique :> actions {
        doc
        /*
		 * assignmentActions is the base feature for all AssignmentActionsUsages.
		 */

        in target : Occurrence[1] default that as Occurrence {
            doc
            /*
             * The default target for assignmentActions is its featuring instance (if that is 
             * an Occurrence).
             */
        }
    }

    action def IfThenAction :> Action, IfThenPerformance {
        doc
        /*
		 * An IfThenAction is a Kernel IfThenPerformance that is also an Action. 
		 * It is the base type for all IfActionUsages.
		 */

        in ifTest[1];
        in action thenClause[0..1];
    }

    action def IfThenElseAction :> IfThenAction, IfThenElsePerformance {
        doc
        /*
		 * An IfThenElseAction is a Kernel IfThenElsePeformance that is also an IfThenAction. 
		 * It is the base type for all IfActionUsages that have an elseAction.
		 */

        in ifTest[1];
        in action thenClause[0..1];
        in action elseClause[0..1];
    }

    abstract action ifThenActions : IfThenAction[0..*] nonunique :> actions {
        doc
        /*
		 * ifThenActions is the base feature for all IfActionUsages.
		 */
    }

    abstract action ifThenElseActions : IfThenElseAction[0..*] nonunique :> actions {
        doc
        /*
		 * ifThenElseActions is the base feature for all IfActionUsages that have an elseAction.
		 */
    }

    abstract action def LoopAction :> Action {
        doc
        /*
		 * A LoopAction is the base type for all LoopActionUsages.
		 */

        in ref iterator;

        in action body[0..*] {
            doc
            /*
			 * The action that is performed repeatedly in the loop.
			 */
        }
    }

    action def WhileLoopAction :> LoopAction, LoopPerformance {
        doc
        /*
		 * A WhileLoopAction is a Kernel LoopPerformance that is also a LoopAction.
		 * It is the base type for all WhileLoopActionUsages.
		 */

        in whileTest default {true} {
            doc
            /*
			 * A Boolean expression that must be true for the loop to continue.
			 * It is evaluated before the body is performed and is always evaluated at 
			 * least once.
			 */
        }

        in action body {
            doc
            /*
			 * The action that is performed while the whileTest is true and the
			 * untilTest is false.
			 */
        }

        in untilTest default {false} {
            doc
            /*
			 * A Boolean expression that must be false for the loop to continue.
			 * It is evaluated after the body is performed.
			 */
        }
    }

    action def ForLoopAction :> LoopAction {
        doc
        /*
		 * A ForLoopAction is a LoopAction that iterates over an ordered sequence of values.
		 * It is the base type for all ForLoopActionUsages.
		 */

        protected ref var[0..1] :> seq {
            doc
            /*
			 * The loop variable that is assigned successive elements of seq on each
			 * iteration of the loop.
			 */
        }

        in ref seq {
            doc
            /*
			 * The sequence of values over which the loop iterates.
			 */
        }

        in action body {
            doc
            /*
			 * The action that is performed on each iteration of the loop.
			 */
        }

        private attribute index : Positive {
            doc
            /*
			 * The index of the element of seq assigned to var on the current iteration
			 * of the loop.
			 */
        }

        private action initialization
        assign index := 1;
        then private action whileLoop
        while index <= size(seq) {
            assign var := seq#(index);
            then perform body;
            then assign index := index + 1;
        }
    }

    abstract action loopActions : LoopAction[0..*] nonunique :> actions {
        doc
        /*
		 * loopActions is the base feature for all LoopActionUsages.
		 */
    }

    abstract action whileLoopActions : WhileLoopAction[0..*] nonunique :> loopActions {
        doc
        /*
		 * whileLoopActions is the base feature for all WhileLoopActionUsages.
		 */
    }

    abstract action forLoopActions : ForLoopAction[0..*] nonunique :> loopActions {
        doc
        /*
		 * forLoopActions is the base feature for all ForLoopActionUsages.
		 */
    }
}

~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Actions"))) (name "Actions") (declared-name "Actions")
      (contains
        (element (kind "action def") (id (node (document "d0") (qualified-name "Actions::AcceptAction"))) (name "AcceptAction") (declared-name "AcceptAction")
          (contains
            (element (kind "ref") (id (node (document "d0") (qualified-name "Actions::AcceptAction::"))) (name "") (declared (properties (composite false) (reference true)) (feature-value (kind bound) (expression (kind "memberAccess") (reference "acceptedMessage") (children (expression (kind "memberAccess") (reference "accepter") (children (expression (kind "memberAccess") (reference "aTransition") (children (expression (kind "featureReference") (reference "aState")))))))))) (effective (featuring-type (node (document "d0") (qualified-name "Actions::AcceptAction"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "Actions::AcceptAction::"))) (role feature-value))))
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::AcceptAction::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::AcceptAction")))))
            (element (kind "state") (id (node (document "d0") (qualified-name "Actions::AcceptAction::aState"))) (name "aState") (declared-name "aState") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "Actions::AcceptAction"))))
              (contains
                (element (kind "transition") (id (node (document "d0") (qualified-name "Actions::AcceptAction::aState::aTransition"))) (name "aTransition") (declared-name "aTransition") (effective (featuring-type (node (document "d0") (qualified-name "Actions::AcceptAction"))))
                  (contains
                    (element (kind "transition trigger") (id (node (document "d0") (qualified-name "Actions::AcceptAction::aState::aTransition::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "Actions::AcceptAction")))))
                  )
                )
              )
            )
          )
        )
        (element (kind "action def") (id (node (document "d0") (qualified-name "Actions::AcceptMessageAction"))) (name "AcceptMessageAction") (declared-name "AcceptMessageAction")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::AcceptMessageAction::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::AcceptMessageAction")))))
            (element (kind "ref") (id (node (document "d0") (qualified-name "Actions::AcceptMessageAction::acceptedMessage"))) (name "acceptedMessage") (declared-name "acceptedMessage") (declared (properties (composite false) (reference true))) (effective (featuring-type (node (document "d0") (qualified-name "Actions::AcceptMessageAction"))))
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Actions::AcceptMessageAction::acceptedMessage::MessageTransfer::payload, MessageAction::payload"))) (name "MessageTransfer::payload, MessageAction::payload") (declared-name "MessageTransfer::payload, MessageAction::payload") (effective (featuring-type (node (document "d0") (qualified-name "Actions::AcceptMessageAction")))))
              )
            )
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Actions::AcceptMessageAction::payload"))) (name "payload") (declared-name "payload") (effective (featuring-type (node (document "d0") (qualified-name "Actions::AcceptMessageAction")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Actions::AcceptPerformance"))) (name "AcceptPerformance") (declared-name "AcceptPerformance"))
        (element (kind "action def") (id (node (document "d0") (qualified-name "Actions::Action"))) (name "Action") (declared-name "Action")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::Action::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::Action")))))
            (element (kind "action") (id (node (document "d0") (qualified-name "Actions::Action::acceptSubactions"))) (name "acceptSubactions") (declared-name "acceptSubactions") (declared (properties (composite true) (reference false)) (multiplicity (lower 0) (upper unbounded) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Actions::Action"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::Action::acceptSubactions::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::AcceptAction")))))
              )
            )
            (element (kind "action") (id (node (document "d0") (qualified-name "Actions::Action::assignments"))) (name "assignments") (declared-name "assignments") (declared (properties (abstract true) (composite true) (reference false)) (multiplicity (lower 0) (upper unbounded) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Actions::Action"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::Action::assignments::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::AssignmentAction")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Actions::Action::assignments::target"))) (name "target") (declared-name "target") (effective (featuring-type (node (document "d0") (qualified-name "Actions::AssignmentAction")))))
              )
            )
            (element (kind "action") (id (node (document "d0") (qualified-name "Actions::Action::controls"))) (name "controls") (declared-name "controls") (declared (properties (abstract true) (composite true) (reference false)) (multiplicity (lower 0) (upper unbounded) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Actions::Action"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::Action::controls::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::ControlAction")))))
              )
            )
            (element (kind "action") (id (node (document "d0") (qualified-name "Actions::Action::decisionTransitions"))) (name "decisionTransitions") (declared-name "decisionTransitions") (declared (properties (abstract true) (composite true) (reference false)) (multiplicity (lower 0) (upper unbounded) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Actions::Action"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::Action::decisionTransitions::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::DecisionTransitionAction")))))
              )
            )
            (element (kind "action") (id (node (document "d0") (qualified-name "Actions::Action::decisions"))) (name "decisions") (declared-name "decisions") (declared (properties (abstract true) (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "Actions::Action"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::Action::decisions::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::DecisionAction")))))
              )
            )
            (element (kind "action") (id (node (document "d0") (qualified-name "Actions::Action::done"))) (name "done") (declared-name "done") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "Actions::Action"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::Action::done::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::Action")))))
              )
            )
            (element (kind "action") (id (node (document "d0") (qualified-name "Actions::Action::forLoops"))) (name "forLoops") (declared-name "forLoops") (declared (properties (abstract true) (composite true) (reference false)) (multiplicity (lower 0) (upper unbounded) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Actions::Action"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::Action::forLoops::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::ForLoopAction")))))
              )
            )
            (element (kind "action") (id (node (document "d0") (qualified-name "Actions::Action::forks"))) (name "forks") (declared-name "forks") (declared (properties (abstract true) (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "Actions::Action"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::Action::forks::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::ForkAction")))))
              )
            )
            (element (kind "action") (id (node (document "d0") (qualified-name "Actions::Action::ifSubactions"))) (name "ifSubactions") (declared-name "ifSubactions") (declared (properties (abstract true) (composite true) (reference false)) (multiplicity (lower 0) (upper unbounded) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Actions::Action"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::Action::ifSubactions::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::IfThenAction")))))
              )
            )
            (element (kind "ref") (id (node (document "d0") (qualified-name "Actions::Action::incomingTransfers"))) (name "incomingTransfers") (declared-name "incomingTransfers") (declared (properties (composite false) (reference true))) (effective (featuring-type (node (document "d0") (qualified-name "Actions::Action")))))
            (element (kind "action") (id (node (document "d0") (qualified-name "Actions::Action::joins"))) (name "joins") (declared-name "joins") (declared (properties (abstract true) (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "Actions::Action"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::Action::joins::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::JoinAction")))))
              )
            )
            (element (kind "action") (id (node (document "d0") (qualified-name "Actions::Action::loops"))) (name "loops") (declared-name "loops") (declared (properties (abstract true) (composite true) (reference false)) (multiplicity (lower 0) (upper unbounded) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Actions::Action"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::Action::loops::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::LoopAction")))))
              )
            )
            (element (kind "action") (id (node (document "d0") (qualified-name "Actions::Action::merges"))) (name "merges") (declared-name "merges") (declared (properties (abstract true) (composite true) (reference false)) (multiplicity (lower 0) (upper unbounded) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Actions::Action"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::Action::merges::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::MergeAction")))))
              )
            )
            (element (kind "ref") (id (node (document "d0") (qualified-name "Actions::Action::self"))) (name "self") (declared-name "self") (declared (properties (composite false) (reference true))) (effective (featuring-type (node (document "d0") (qualified-name "Actions::Action")))))
            (element (kind "action") (id (node (document "d0") (qualified-name "Actions::Action::sendSubactions"))) (name "sendSubactions") (declared-name "sendSubactions") (declared (properties (composite true) (reference false)) (multiplicity (lower 0) (upper unbounded) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Actions::Action"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::Action::sendSubactions::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::SendAction")))))
              )
            )
            (element (kind "action") (id (node (document "d0") (qualified-name "Actions::Action::start"))) (name "start") (declared-name "start") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "Actions::Action"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::Action::start::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::Action")))))
              )
            )
            (element (kind "action") (id (node (document "d0") (qualified-name "Actions::Action::subactions"))) (name "subactions") (declared-name "subactions") (declared (properties (composite true) (reference false)) (multiplicity (lower 0) (upper unbounded) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Actions::Action"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::Action::subactions::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::Action")))))
                (element (kind "ref") (id (node (document "d0") (qualified-name "Actions::Action::subactions::occurrence"))) (name "occurrence") (declared-name "occurrence") (declared (properties (composite false) (reference true))) (effective (featuring-type (node (document "d0") (qualified-name "Actions::Action"))))
                  (contains
                    (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::Action::subactions::occurrence::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::Action")))))
                  )
                )
              )
            )
            (element (kind "action") (id (node (document "d0") (qualified-name "Actions::Action::terminateSubactions"))) (name "terminateSubactions") (declared-name "terminateSubactions") (declared (properties (abstract true) (composite true) (reference false)) (multiplicity (lower 0) (upper unbounded) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Actions::Action"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::Action::terminateSubactions::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::TerminateAction")))))
              )
            )
            (element (kind "action") (id (node (document "d0") (qualified-name "Actions::Action::transitions"))) (name "transitions") (declared-name "transitions") (declared (properties (abstract true) (composite true) (reference false)) (multiplicity (lower 0) (upper unbounded) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Actions::Action"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::Action::transitions::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::TransitionAction")))))
              )
            )
            (element (kind "action") (id (node (document "d0") (qualified-name "Actions::Action::whileLoops"))) (name "whileLoops") (declared-name "whileLoops") (declared (properties (abstract true) (composite true) (reference false)) (multiplicity (lower 0) (upper unbounded) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Actions::Action"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::Action::whileLoops::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::WhileLoopAction")))))
              )
            )
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Actions::Anything"))) (name "Anything") (declared-name "Anything"))
        (element (kind "action def") (id (node (document "d0") (qualified-name "Actions::AssignmentAction"))) (name "AssignmentAction") (declared-name "AssignmentAction")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::AssignmentAction::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::AssignmentAction")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Actions::AssignmentAction::replacementValues"))) (name "replacementValues") (declared-name "replacementValues") (effective (featuring-type (node (document "d0") (qualified-name "Actions::AssignmentAction")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Actions::AssignmentAction::target"))) (name "target") (declared-name "target") (effective (featuring-type (node (document "d0") (qualified-name "Actions::AssignmentAction")))))
          )
        )
        (element (kind "action def") (id (node (document "d0") (qualified-name "Actions::ControlAction"))) (name "ControlAction") (declared-name "ControlAction")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::ControlAction::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::ControlAction")))))
          )
        )
        (element (kind "action def") (id (node (document "d0") (qualified-name "Actions::DecisionAction"))) (name "DecisionAction") (declared-name "DecisionAction")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::DecisionAction::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::DecisionAction")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Actions::DecisionPerformance"))) (name "DecisionPerformance") (declared-name "DecisionPerformance"))
        (element (kind "action def") (id (node (document "d0") (qualified-name "Actions::DecisionTransitionAction"))) (name "DecisionTransitionAction") (declared-name "DecisionTransitionAction")
          (contains
            (element (kind "ref") (id (node (document "d0") (qualified-name "Actions::DecisionTransitionAction::"))) (name "") (declared (properties (composite false) (reference true))) (effective (featuring-type (node (document "d0") (qualified-name "Actions::DecisionTransitionAction")))))
            (element (kind "ref") (id (node (document "d0") (qualified-name "Actions::DecisionTransitionAction::#ref"))) (name "") (declared (properties (composite false) (reference true))) (effective (featuring-type (node (document "d0") (qualified-name "Actions::DecisionTransitionAction")))))
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::DecisionTransitionAction::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::DecisionTransitionAction")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Actions::FeatureWritePerformance"))) (name "FeatureWritePerformance") (declared-name "FeatureWritePerformance"))
        (element (kind "action def") (id (node (document "d0") (qualified-name "Actions::ForLoopAction"))) (name "ForLoopAction") (declared-name "ForLoopAction")
          (contains
            (element (kind "assign") (id (node (document "d0") (qualified-name "Actions::ForLoopAction::_assign"))) (name "assign") (declared-name "assign") (effective (featuring-type (node (document "d0") (qualified-name "Actions::ForLoopAction")))))
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::ForLoopAction::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::ForLoopAction")))))
            (element (kind "while") (id (node (document "d0") (qualified-name "Actions::ForLoopAction::_while"))) (name "while") (declared-name "while") (effective (featuring-type (node (document "d0") (qualified-name "Actions::ForLoopAction"))))
              (contains
                (element (kind "assign") (id (node (document "d0") (qualified-name "Actions::ForLoopAction::_while::_assign"))) (name "assign") (declared-name "assign") (effective (featuring-type (node (document "d0") (qualified-name "Actions::ForLoopAction")))))
                (element (kind "assign") (id (node (document "d0") (qualified-name "Actions::ForLoopAction::_while::_assign#assign"))) (name "assign") (declared-name "assign") (effective (featuring-type (node (document "d0") (qualified-name "Actions::ForLoopAction")))))
              )
            )
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Actions::ForLoopAction::body"))) (name "body") (declared-name "body") (effective (featuring-type (node (document "d0") (qualified-name "Actions::ForLoopAction")))))
            (element (kind "action body decl") (id (node (document "d0") (qualified-name "Actions::ForLoopAction::index : Positive"))) (name "index : Positive") (declared-name "index : Positive") (effective (featuring-type (node (document "d0") (qualified-name "Actions::ForLoopAction")))))
            (element (kind "action") (id (node (document "d0") (qualified-name "Actions::ForLoopAction::initialization"))) (name "initialization") (declared-name "initialization") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "Actions::ForLoopAction")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Actions::ForLoopAction::ref"))) (name "ref") (declared-name "ref") (effective (featuring-type (node (document "d0") (qualified-name "Actions::ForLoopAction")))))
            (element (kind "ref") (id (node (document "d0") (qualified-name "Actions::ForLoopAction::var"))) (name "var") (declared-name "var") (declared (properties (composite false) (reference true))) (effective (featuring-type (node (document "d0") (qualified-name "Actions::ForLoopAction"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::ForLoopAction::var::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::ForLoopAction")))))
              )
            )
            (element (kind "action") (id (node (document "d0") (qualified-name "Actions::ForLoopAction::whileLoop"))) (name "whileLoop") (declared-name "whileLoop") (effective (featuring-type (node (document "d0") (qualified-name "Actions::ForLoopAction")))))
          )
        )
        (element (kind "action def") (id (node (document "d0") (qualified-name "Actions::ForkAction"))) (name "ForkAction") (declared-name "ForkAction")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::ForkAction::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::ForkAction")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Actions::HappensWhile"))) (name "HappensWhile") (declared-name "HappensWhile"))
        (element (kind "action def") (id (node (document "d0") (qualified-name "Actions::IfThenAction"))) (name "IfThenAction") (declared-name "IfThenAction")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::IfThenAction::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::IfThenAction")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Actions::IfThenAction::action"))) (name "action") (declared-name "action") (effective (featuring-type (node (document "d0") (qualified-name "Actions::IfThenAction")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Actions::IfThenAction::ifTest"))) (name "ifTest") (declared-name "ifTest") (effective (featuring-type (node (document "d0") (qualified-name "Actions::IfThenAction")))))
          )
        )
        (element (kind "action def") (id (node (document "d0") (qualified-name "Actions::IfThenElseAction"))) (name "IfThenElseAction") (declared-name "IfThenElseAction")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::IfThenElseAction::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::IfThenElseAction")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Actions::IfThenElseAction::action"))) (name "action") (declared-name "action") (effective (featuring-type (node (document "d0") (qualified-name "Actions::IfThenElseAction")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Actions::IfThenElseAction::action#in_out_parameter"))) (name "action") (declared-name "action") (effective (featuring-type (node (document "d0") (qualified-name "Actions::IfThenElseAction")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Actions::IfThenElseAction::ifTest"))) (name "ifTest") (declared-name "ifTest") (effective (featuring-type (node (document "d0") (qualified-name "Actions::IfThenElseAction")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Actions::IfThenElsePerformance"))) (name "IfThenElsePerformance") (declared-name "IfThenElsePerformance"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Actions::IfThenPerformance"))) (name "IfThenPerformance") (declared-name "IfThenPerformance"))
        (element (kind "action def") (id (node (document "d0") (qualified-name "Actions::JoinAction"))) (name "JoinAction") (declared-name "JoinAction")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::JoinAction::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::JoinAction")))))
          )
        )
        (element (kind "action def") (id (node (document "d0") (qualified-name "Actions::LoopAction"))) (name "LoopAction") (declared-name "LoopAction")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::LoopAction::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::LoopAction")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Actions::LoopAction::action"))) (name "action") (declared-name "action") (effective (featuring-type (node (document "d0") (qualified-name "Actions::LoopAction")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Actions::LoopAction::ref"))) (name "ref") (declared-name "ref") (effective (featuring-type (node (document "d0") (qualified-name "Actions::LoopAction")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Actions::LoopPerformance"))) (name "LoopPerformance") (declared-name "LoopPerformance"))
        (element (kind "action def") (id (node (document "d0") (qualified-name "Actions::MergeAction"))) (name "MergeAction") (declared-name "MergeAction")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::MergeAction::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::MergeAction")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Actions::MergePerformance"))) (name "MergePerformance") (declared-name "MergePerformance"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Actions::MessageAction"))) (name "MessageAction") (declared-name "MessageAction"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Actions::MessageTransfer"))) (name "MessageTransfer") (declared-name "MessageTransfer"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Actions::Natural"))) (name "Natural") (declared-name "Natural"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Actions::NonStateTransitionPerformance"))) (name "NonStateTransitionPerformance") (declared-name "NonStateTransitionPerformance"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Actions::Occurrence"))) (name "Occurrence") (declared-name "Occurrence"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Actions::Performance"))) (name "Performance") (declared-name "Performance"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Actions::Positive"))) (name "Positive") (declared-name "Positive"))
        (element (kind "action def") (id (node (document "d0") (qualified-name "Actions::SendAction"))) (name "SendAction") (declared-name "SendAction")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::SendAction::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::SendAction")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Actions::SendAction::payload"))) (name "payload") (declared-name "payload") (effective (featuring-type (node (document "d0") (qualified-name "Actions::SendAction")))))
            (element (kind "ref") (id (node (document "d0") (qualified-name "Actions::SendAction::sentMessage"))) (name "sentMessage") (declared-name "sentMessage") (declared (properties (composite false) (reference true))) (effective (featuring-type (node (document "d0") (qualified-name "Actions::SendAction"))))
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Actions::SendAction::sentMessage::MessageTransfer::payload, MessageAction::payload"))) (name "MessageTransfer::payload, MessageAction::payload") (declared-name "MessageTransfer::payload, MessageAction::payload") (effective (featuring-type (node (document "d0") (qualified-name "Actions::SendAction")))))
              )
            )
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Actions::SendPerformance"))) (name "SendPerformance") (declared-name "SendPerformance"))
        (element (kind "action def") (id (node (document "d0") (qualified-name "Actions::TerminateAction"))) (name "TerminateAction") (declared-name "TerminateAction")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::TerminateAction::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::TerminateAction")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Actions::TerminateAction::occurrence"))) (name "occurrence") (declared-name "occurrence") (effective (featuring-type (node (document "d0") (qualified-name "Actions::TerminateAction")))))
            (element (kind "action") (id (node (document "d0") (qualified-name "Actions::TerminateAction::terminateOccurrence"))) (name "terminateOccurrence") (declared-name "terminateOccurrence") (declared (properties (composite true) (reference false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Actions::TerminateAction"))))
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Actions::TerminateAction::terminateOccurrence::occ"))) (name "occ") (declared-name "occ") (effective (featuring-type (node (document "d0") (qualified-name "Actions::TerminateAction")))))
              )
            )
          )
        )
        (element (kind "action def") (id (node (document "d0") (qualified-name "Actions::TransitionAction"))) (name "TransitionAction") (declared-name "TransitionAction")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::TransitionAction::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::TransitionAction")))))
            (element (kind "ref") (id (node (document "d0") (qualified-name "Actions::TransitionAction::acceptedMessage"))) (name "acceptedMessage") (declared-name "acceptedMessage") (declared (properties (composite false) (reference true))) (effective (featuring-type (node (document "d0") (qualified-name "Actions::TransitionAction"))))
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Actions::TransitionAction::acceptedMessage::MessageTransfer::payload, MessageAction::payload"))) (name "MessageTransfer::payload, MessageAction::payload") (declared-name "MessageTransfer::payload, MessageAction::payload") (effective (featuring-type (node (document "d0") (qualified-name "Actions::TransitionAction")))))
              )
            )
            (element (kind "action") (id (node (document "d0") (qualified-name "Actions::TransitionAction::accepter"))) (name "accepter") (declared-name "accepter") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "Actions::TransitionAction")))))
            (element (kind "action") (id (node (document "d0") (qualified-name "Actions::TransitionAction::effect"))) (name "effect") (declared-name "effect") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "Actions::TransitionAction")))))
            (element (kind "ref") (id (node (document "d0") (qualified-name "Actions::TransitionAction::receiver"))) (name "receiver") (declared-name "receiver") (declared (properties (composite false) (reference true))) (effective (featuring-type (node (document "d0") (qualified-name "Actions::TransitionAction")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Actions::TransitionAction::transitionLinkSource"))) (name "transitionLinkSource") (declared-name "transitionLinkSource") (effective (featuring-type (node (document "d0") (qualified-name "Actions::TransitionAction")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Actions::TransitionPerformance"))) (name "TransitionPerformance") (declared-name "TransitionPerformance"))
        (element (kind "action def") (id (node (document "d0") (qualified-name "Actions::WhileLoopAction"))) (name "WhileLoopAction") (declared-name "WhileLoopAction")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::WhileLoopAction::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::WhileLoopAction")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Actions::WhileLoopAction::body"))) (name "body") (declared-name "body") (effective (featuring-type (node (document "d0") (qualified-name "Actions::WhileLoopAction")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Actions::WhileLoopAction::untilTest"))) (name "untilTest") (declared-name "untilTest") (effective (featuring-type (node (document "d0") (qualified-name "Actions::WhileLoopAction")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Actions::WhileLoopAction::whileTest"))) (name "whileTest") (declared-name "whileTest") (effective (featuring-type (node (document "d0") (qualified-name "Actions::WhileLoopAction")))))
          )
        )
        (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::_documentation"))) (name ""))
        (element (kind "action") (id (node (document "d0") (qualified-name "Actions::acceptActions"))) (name "acceptActions") (declared-name "acceptActions") (declared (properties (abstract true) (composite true) (reference false)) (multiplicity (lower 0) (upper unbounded) (ordered false) (provenance authored)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::acceptActions::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::AcceptAction")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Actions::acceptPerformances"))) (name "acceptPerformances") (declared-name "acceptPerformances"))
        (element (kind "action") (id (node (document "d0") (qualified-name "Actions::actions"))) (name "actions") (declared-name "actions") (declared (properties (abstract true) (composite true) (reference false)) (multiplicity (lower 0) (upper unbounded) (ordered false) (provenance authored)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::actions::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::Action")))))
          )
        )
        (element (kind "action") (id (node (document "d0") (qualified-name "Actions::assignmentActions"))) (name "assignmentActions") (declared-name "assignmentActions") (declared (properties (abstract true) (composite true) (reference false)) (multiplicity (lower 0) (upper unbounded) (ordered false) (provenance authored)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::assignmentActions::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::AssignmentAction")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Actions::assignmentActions::target"))) (name "target") (declared-name "target") (effective (featuring-type (node (document "d0") (qualified-name "Actions::AssignmentAction")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Actions::destroy"))) (name "destroy") (declared-name "destroy"))
        (element (kind "action") (id (node (document "d0") (qualified-name "Actions::forLoopActions"))) (name "forLoopActions") (declared-name "forLoopActions") (declared (properties (abstract true) (composite true) (reference false)) (multiplicity (lower 0) (upper unbounded) (ordered false) (provenance authored)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::forLoopActions::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::ForLoopAction")))))
          )
        )
        (element (kind "action") (id (node (document "d0") (qualified-name "Actions::ifThenActions"))) (name "ifThenActions") (declared-name "ifThenActions") (declared (properties (abstract true) (composite true) (reference false)) (multiplicity (lower 0) (upper unbounded) (ordered false) (provenance authored)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::ifThenActions::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::IfThenAction")))))
          )
        )
        (element (kind "action") (id (node (document "d0") (qualified-name "Actions::ifThenElseActions"))) (name "ifThenElseActions") (declared-name "ifThenElseActions") (declared (properties (abstract true) (composite true) (reference false)) (multiplicity (lower 0) (upper unbounded) (ordered false) (provenance authored)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::ifThenElseActions::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::IfThenElseAction")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Actions::isEmpty"))) (name "isEmpty") (declared-name "isEmpty"))
        (element (kind "action") (id (node (document "d0") (qualified-name "Actions::loopActions"))) (name "loopActions") (declared-name "loopActions") (declared (properties (abstract true) (composite true) (reference false)) (multiplicity (lower 0) (upper unbounded) (ordered false) (provenance authored)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::loopActions::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::LoopAction")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Actions::performances"))) (name "performances") (declared-name "performances"))
        (element (kind "action") (id (node (document "d0") (qualified-name "Actions::sendActions"))) (name "sendActions") (declared-name "sendActions") (declared (properties (abstract true) (composite true) (reference false)) (multiplicity (lower 0) (upper unbounded) (ordered false) (provenance authored)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::sendActions::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::SendAction")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Actions::sendPerformances"))) (name "sendPerformances") (declared-name "sendPerformances"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Actions::size"))) (name "size") (declared-name "size"))
        (element (kind "action") (id (node (document "d0") (qualified-name "Actions::terminateActions"))) (name "terminateActions") (declared-name "terminateActions") (declared (properties (abstract true) (composite true) (reference false)) (multiplicity (lower 0) (upper unbounded) (ordered false) (provenance authored)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::terminateActions::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::TerminateAction")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Actions::terminateActions::occurrence"))) (name "occurrence") (declared-name "occurrence") (effective (featuring-type (node (document "d0") (qualified-name "Actions::TerminateAction")))))
          )
        )
        (element (kind "action") (id (node (document "d0") (qualified-name "Actions::transitionActions"))) (name "transitionActions") (declared-name "transitionActions") (declared (properties (abstract true) (composite true) (reference false)) (multiplicity (lower 0) (upper unbounded) (ordered false) (provenance authored)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::transitionActions::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::TransitionAction")))))
          )
        )
        (element (kind "action") (id (node (document "d0") (qualified-name "Actions::whileLoopActions"))) (name "whileLoopActions") (declared-name "whileLoopActions") (declared (properties (abstract true) (composite true) (reference false)) (multiplicity (lower 0) (upper unbounded) (ordered false) (provenance authored)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Actions::whileLoopActions::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Actions::WhileLoopAction")))))
          )
        )
      )
    )
    (element (kind "diagnostic") (id (node (document "d0") (qualified-name "Actions::AcceptAction::ambiguous_connection_endpoint"))) (name "ambiguous_connection_endpoint") (declared-name "ambiguous_connection_endpoint"))
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::AcceptAction::_documentation"))) (to (node (document "d0") (qualified-name "Actions::AcceptAction"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::AcceptMessageAction::_documentation"))) (to (node (document "d0") (qualified-name "Actions::AcceptMessageAction"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::Action::_documentation"))) (to (node (document "d0") (qualified-name "Actions::Action"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::Action::acceptSubactions::_documentation"))) (to (node (document "d0") (qualified-name "Actions::Action::acceptSubactions"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::Action::assignments::_documentation"))) (to (node (document "d0") (qualified-name "Actions::Action::assignments"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::Action::controls::_documentation"))) (to (node (document "d0") (qualified-name "Actions::Action::controls"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::Action::decisionTransitions::_documentation"))) (to (node (document "d0") (qualified-name "Actions::Action::decisionTransitions"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::Action::decisions::_documentation"))) (to (node (document "d0") (qualified-name "Actions::Action::decisions"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::Action::done::_documentation"))) (to (node (document "d0") (qualified-name "Actions::Action::done"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::Action::forLoops::_documentation"))) (to (node (document "d0") (qualified-name "Actions::Action::forLoops"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::Action::forks::_documentation"))) (to (node (document "d0") (qualified-name "Actions::Action::forks"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::Action::ifSubactions::_documentation"))) (to (node (document "d0") (qualified-name "Actions::Action::ifSubactions"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::Action::joins::_documentation"))) (to (node (document "d0") (qualified-name "Actions::Action::joins"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::Action::loops::_documentation"))) (to (node (document "d0") (qualified-name "Actions::Action::loops"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::Action::merges::_documentation"))) (to (node (document "d0") (qualified-name "Actions::Action::merges"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::Action::sendSubactions::_documentation"))) (to (node (document "d0") (qualified-name "Actions::Action::sendSubactions"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::Action::start::_documentation"))) (to (node (document "d0") (qualified-name "Actions::Action::start"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::Action::subactions::_documentation"))) (to (node (document "d0") (qualified-name "Actions::Action::subactions"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::Action::subactions::occurrence::_documentation"))) (to (node (document "d0") (qualified-name "Actions::Action::subactions::occurrence"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::Action::terminateSubactions::_documentation"))) (to (node (document "d0") (qualified-name "Actions::Action::terminateSubactions"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::Action::transitions::_documentation"))) (to (node (document "d0") (qualified-name "Actions::Action::transitions"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::Action::whileLoops::_documentation"))) (to (node (document "d0") (qualified-name "Actions::Action::whileLoops"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::AssignmentAction::_documentation"))) (to (node (document "d0") (qualified-name "Actions::AssignmentAction"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::ControlAction::_documentation"))) (to (node (document "d0") (qualified-name "Actions::ControlAction"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::DecisionAction::_documentation"))) (to (node (document "d0") (qualified-name "Actions::DecisionAction"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::DecisionTransitionAction::_documentation"))) (to (node (document "d0") (qualified-name "Actions::DecisionTransitionAction"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::ForLoopAction::_documentation"))) (to (node (document "d0") (qualified-name "Actions::ForLoopAction"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::ForLoopAction::var::_documentation"))) (to (node (document "d0") (qualified-name "Actions::ForLoopAction::var"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::ForkAction::_documentation"))) (to (node (document "d0") (qualified-name "Actions::ForkAction"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::IfThenAction::_documentation"))) (to (node (document "d0") (qualified-name "Actions::IfThenAction"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::IfThenElseAction::_documentation"))) (to (node (document "d0") (qualified-name "Actions::IfThenElseAction"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::JoinAction::_documentation"))) (to (node (document "d0") (qualified-name "Actions::JoinAction"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::LoopAction::_documentation"))) (to (node (document "d0") (qualified-name "Actions::LoopAction"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::MergeAction::_documentation"))) (to (node (document "d0") (qualified-name "Actions::MergeAction"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::SendAction::_documentation"))) (to (node (document "d0") (qualified-name "Actions::SendAction"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::TerminateAction::_documentation"))) (to (node (document "d0") (qualified-name "Actions::TerminateAction"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::TransitionAction::_documentation"))) (to (node (document "d0") (qualified-name "Actions::TransitionAction"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::WhileLoopAction::_documentation"))) (to (node (document "d0") (qualified-name "Actions::WhileLoopAction"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::_documentation"))) (to (node (document "d0") (qualified-name "Actions"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::acceptActions::_documentation"))) (to (node (document "d0") (qualified-name "Actions::acceptActions"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::actions::_documentation"))) (to (node (document "d0") (qualified-name "Actions::actions"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::assignmentActions::_documentation"))) (to (node (document "d0") (qualified-name "Actions::assignmentActions"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::forLoopActions::_documentation"))) (to (node (document "d0") (qualified-name "Actions::forLoopActions"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::ifThenActions::_documentation"))) (to (node (document "d0") (qualified-name "Actions::ifThenActions"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::ifThenElseActions::_documentation"))) (to (node (document "d0") (qualified-name "Actions::ifThenElseActions"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::loopActions::_documentation"))) (to (node (document "d0") (qualified-name "Actions::loopActions"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::sendActions::_documentation"))) (to (node (document "d0") (qualified-name "Actions::sendActions"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::terminateActions::_documentation"))) (to (node (document "d0") (qualified-name "Actions::terminateActions"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::transitionActions::_documentation"))) (to (node (document "d0") (qualified-name "Actions::transitionActions"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Actions::whileLoopActions::_documentation"))) (to (node (document "d0") (qualified-name "Actions::whileLoopActions"))))
    (bind (status resolved) (from (node (document "d0") (qualified-name "Actions::Action::start"))) (to (node (document "d0") (qualified-name "Actions::Action::done"))) (connect (source-expression "start") (target-expression "done") (container-prefix "Actions::ControlAction")))
    (bind (status resolved) (from (node (document "d0") (qualified-name "Actions::TransitionAction::acceptedMessage"))) (to (node (document "d0") (qualified-name "Actions::AcceptMessageAction::acceptedMessage"))) (connect (source-expression "acceptedMessage") (target-expression "accepter::acceptedMessage") (container-prefix "Actions::TransitionAction")))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Actions::Action"))) (to (node (document "d0") (qualified-name "Actions::Action::acceptSubactions"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Actions::Action"))) (to (node (document "d0") (qualified-name "Actions::Action::assignments"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Actions::Action"))) (to (node (document "d0") (qualified-name "Actions::Action::controls"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Actions::Action"))) (to (node (document "d0") (qualified-name "Actions::Action::decisionTransitions"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Actions::Action"))) (to (node (document "d0") (qualified-name "Actions::Action::decisions"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Actions::Action"))) (to (node (document "d0") (qualified-name "Actions::Action::done"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Actions::Action"))) (to (node (document "d0") (qualified-name "Actions::Action::forLoops"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Actions::Action"))) (to (node (document "d0") (qualified-name "Actions::Action::forks"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Actions::Action"))) (to (node (document "d0") (qualified-name "Actions::Action::ifSubactions"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Actions::Action"))) (to (node (document "d0") (qualified-name "Actions::Action::joins"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Actions::Action"))) (to (node (document "d0") (qualified-name "Actions::Action::loops"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Actions::Action"))) (to (node (document "d0") (qualified-name "Actions::Action::merges"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Actions::Action"))) (to (node (document "d0") (qualified-name "Actions::Action::sendSubactions"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Actions::Action"))) (to (node (document "d0") (qualified-name "Actions::Action::start"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Actions::Action"))) (to (node (document "d0") (qualified-name "Actions::Action::subactions"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Actions::Action"))) (to (node (document "d0") (qualified-name "Actions::Action::terminateSubactions"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Actions::Action"))) (to (node (document "d0") (qualified-name "Actions::Action::transitions"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Actions::Action"))) (to (node (document "d0") (qualified-name "Actions::Action::whileLoops"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Actions::ForLoopAction"))) (to (node (document "d0") (qualified-name "Actions::ForLoopAction::initialization"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Actions::ForLoopAction"))) (to (node (document "d0") (qualified-name "Actions::ForLoopAction::whileLoop"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Actions::TerminateAction"))) (to (node (document "d0") (qualified-name "Actions::TerminateAction::terminateOccurrence"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Actions::TransitionAction"))) (to (node (document "d0") (qualified-name "Actions::TransitionAction::accepter"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Actions::TransitionAction"))) (to (node (document "d0") (qualified-name "Actions::TransitionAction::effect"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "Actions::AcceptAction::"))) (to (node (document "d0") (qualified-name "Actions::AcceptMessageAction::acceptedMessage"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "Actions::DecisionTransitionAction::"))) (to (node (document "d0") (qualified-name "Actions::TransitionAction::accepter"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "Actions::DecisionTransitionAction::#ref"))) (to (node (document "d0") (qualified-name "Actions::TransitionAction::effect"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Actions::AcceptAction"))) (to (node (document "d0") (qualified-name "Actions::AcceptMessageAction"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Actions::AcceptMessageAction"))) (to (node (document "d0") (qualified-name "Actions::Action"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Actions::AssignmentAction"))) (to (node (document "d0") (qualified-name "Actions::Action"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Actions::ControlAction"))) (to (node (document "d0") (qualified-name "Actions::Action"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Actions::DecisionAction"))) (to (node (document "d0") (qualified-name "Actions::ControlAction"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Actions::DecisionTransitionAction"))) (to (node (document "d0") (qualified-name "Actions::TransitionAction"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Actions::ForLoopAction"))) (to (node (document "d0") (qualified-name "Actions::LoopAction"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Actions::ForkAction"))) (to (node (document "d0") (qualified-name "Actions::ControlAction"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Actions::IfThenAction"))) (to (node (document "d0") (qualified-name "Actions::Action"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Actions::IfThenElseAction"))) (to (node (document "d0") (qualified-name "Actions::IfThenAction"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Actions::JoinAction"))) (to (node (document "d0") (qualified-name "Actions::ControlAction"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Actions::LoopAction"))) (to (node (document "d0") (qualified-name "Actions::Action"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Actions::MergeAction"))) (to (node (document "d0") (qualified-name "Actions::ControlAction"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Actions::SendAction"))) (to (node (document "d0") (qualified-name "Actions::Action"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Actions::TerminateAction"))) (to (node (document "d0") (qualified-name "Actions::Action"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Actions::TransitionAction"))) (to (node (document "d0") (qualified-name "Actions::Action"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Actions::WhileLoopAction"))) (to (node (document "d0") (qualified-name "Actions::LoopAction"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "Actions::Action::controls"))) (to (node (document "d0") (qualified-name "Actions::Action::subactions"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "Actions::Action::decisionTransitions"))) (to (node (document "d0") (qualified-name "Actions::Action::transitions"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "Actions::Action::decisions"))) (to (node (document "d0") (qualified-name "Actions::Action::controls"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "Actions::Action::forks"))) (to (node (document "d0") (qualified-name "Actions::Action::controls"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "Actions::Action::joins"))) (to (node (document "d0") (qualified-name "Actions::Action::controls"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "Actions::Action::merges"))) (to (node (document "d0") (qualified-name "Actions::Action::controls"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "Actions::actions"))) (to (node (document "d0") (qualified-name "Actions::performances"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "Actions::assignmentActions"))) (to (node (document "d0") (qualified-name "Actions::actions"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "Actions::forLoopActions"))) (to (node (document "d0") (qualified-name "Actions::loopActions"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "Actions::ifThenActions"))) (to (node (document "d0") (qualified-name "Actions::actions"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "Actions::ifThenElseActions"))) (to (node (document "d0") (qualified-name "Actions::actions"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "Actions::loopActions"))) (to (node (document "d0") (qualified-name "Actions::actions"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "Actions::terminateActions"))) (to (node (document "d0") (qualified-name "Actions::actions"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "Actions::transitionActions"))) (to (node (document "d0") (qualified-name "Actions::actions"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "Actions::whileLoopActions"))) (to (node (document "d0") (qualified-name "Actions::loopActions"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Actions::Action::acceptSubactions"))) (to (node (document "d0") (qualified-name "Actions::AcceptAction"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Actions::Action::assignments"))) (to (node (document "d0") (qualified-name "Actions::AssignmentAction"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Actions::Action::controls"))) (to (node (document "d0") (qualified-name "Actions::ControlAction"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Actions::Action::decisionTransitions"))) (to (node (document "d0") (qualified-name "Actions::DecisionTransitionAction"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Actions::Action::decisions"))) (to (node (document "d0") (qualified-name "Actions::DecisionAction"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Actions::Action::done"))) (to (node (document "d0") (qualified-name "Actions::Action"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Actions::Action::forLoops"))) (to (node (document "d0") (qualified-name "Actions::ForLoopAction"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Actions::Action::forks"))) (to (node (document "d0") (qualified-name "Actions::ForkAction"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Actions::Action::ifSubactions"))) (to (node (document "d0") (qualified-name "Actions::IfThenAction"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Actions::Action::joins"))) (to (node (document "d0") (qualified-name "Actions::JoinAction"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Actions::Action::loops"))) (to (node (document "d0") (qualified-name "Actions::LoopAction"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Actions::Action::merges"))) (to (node (document "d0") (qualified-name "Actions::MergeAction"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Actions::Action::self"))) (to (node (document "d0") (qualified-name "Actions::Action"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Actions::Action::sendSubactions"))) (to (node (document "d0") (qualified-name "Actions::SendAction"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Actions::Action::start"))) (to (node (document "d0") (qualified-name "Actions::Action"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Actions::Action::subactions"))) (to (node (document "d0") (qualified-name "Actions::Action"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Actions::Action::terminateSubactions"))) (to (node (document "d0") (qualified-name "Actions::TerminateAction"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Actions::Action::transitions"))) (to (node (document "d0") (qualified-name "Actions::TransitionAction"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Actions::Action::whileLoops"))) (to (node (document "d0") (qualified-name "Actions::WhileLoopAction"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Actions::TransitionAction::accepter"))) (to (node (document "d0") (qualified-name "Actions::AcceptMessageAction"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Actions::TransitionAction::effect"))) (to (node (document "d0") (qualified-name "Actions::Action"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Actions::acceptActions"))) (to (node (document "d0") (qualified-name "Actions::AcceptAction"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Actions::actions"))) (to (node (document "d0") (qualified-name "Actions::Action"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Actions::assignmentActions"))) (to (node (document "d0") (qualified-name "Actions::AssignmentAction"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Actions::forLoopActions"))) (to (node (document "d0") (qualified-name "Actions::ForLoopAction"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Actions::ifThenActions"))) (to (node (document "d0") (qualified-name "Actions::IfThenAction"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Actions::ifThenElseActions"))) (to (node (document "d0") (qualified-name "Actions::IfThenElseAction"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Actions::loopActions"))) (to (node (document "d0") (qualified-name "Actions::LoopAction"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Actions::sendActions"))) (to (node (document "d0") (qualified-name "Actions::SendAction"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Actions::terminateActions"))) (to (node (document "d0") (qualified-name "Actions::TerminateAction"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Actions::transitionActions"))) (to (node (document "d0") (qualified-name "Actions::TransitionAction"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Actions::whileLoopActions"))) (to (node (document "d0") (qualified-name "Actions::WhileLoopAction"))))
  )
  (pending-relationships
    (flow (status pending) (document "d0") (source-qualified "Actions::ForLoopAction::_while") (target-qualified "Actions::ForLoopAction::_while::body"))
    (transition (status pending) (document "d0") (source-qualified "Actions::AcceptAction::aState::start") (target-qualified "Actions::AcceptAction::aState::done"))
  )
  (pending-expression-relationships
    (bind (status pending-expression) (document "d0") (source-expression "receiver") (target-expression "accepter::receiver") (container-prefix "Actions::TransitionAction"))
  )
)
~~~
