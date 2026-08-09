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
    doc /*
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
        doc /*
		 * Action is the most general class of Performances of ActionDefinitions in a system or 
		 * part of a system. Action is the base class of all ActionDefinitions.
		 */

        ref action self : Action :>> Performance::self;
        ref action incomingTransfers :>> Performance::incomingTransfers;

        action start : Action :>> startShot {
            doc /*
			 * The starting snapshot of an Action. 
			 */
        }

        action done : Action :>> endShot {
            doc /*
			 * The ending snapshot of an Action.
			 */
        }

        action subactions : Action [0..*] :> actions, subperformances {
            doc /*
			 * The subperformances of this Action that are Actions. 
			 */

            ref occurrence :>> Action::this, actions::this, subperformances::this = (that as Action).this {
                doc /*
				 * The "this" reference of a subaction is always the same as that of
				 * its owning Action.
				 */
            }
        }

        action sendSubactions : SendAction [0..*] :> subactions, sendActions {
            doc /*
			 * The subactions of this Action that are SendActions. 
			 */
        }

        action acceptSubactions : AcceptAction [0..*] :> subactions, acceptActions {
            doc /*
			 * The subactions of this Action that are AcceptActions. 
			 */
        }

        abstract action terminateSubactions : TerminateAction [0..*] :> subactions, terminateActions {
            doc /*
			 * The subactions of this Action that are TerminateActions.
			 */
        }

        abstract action controls : ControlAction [0..*] :> subactions {
            doc /*
			 * The subactions of this Action that are ControlActions.
			 */
        }

        abstract action merges : MergeAction [0..*] :> controls {
            doc /*
			 * The controls of this Action that are MergeActions.
			 */
        }

        abstract action decisions : DecisionAction :> controls {
            doc /*
			 * The controls of this Action that are DecisionActions.
			 */
        }

        abstract action joins : JoinAction :> controls {
            doc /*
			 * The controls of this Action that are JoinActions.
			 */
        }

        abstract action forks : ForkAction :> controls {
            doc /*
			 * The controls of this Action that are ForkActions.
			 */
        }

        abstract action transitions : TransitionAction [0..*] :> subactions, transitionActions {
            doc /*
			 * The subactions of this Action that are TransitionActions. 
			 */
        }

        abstract action decisionTransitions : DecisionTransitionAction [0..*] :> transitions {
            doc /*
			 * The subactions of this Action that are DecisionTransitionActions. 
			 */
        }

        abstract action assignments : AssignmentAction [0..*] :> subactions, assignmentActions {
            doc /*
			 * The subactions of this Action that are AssignmentActions.
			 */

            in target;
        }

        abstract action ifSubactions : IfThenAction [0..*] :> subactions, ifThenActions {
            doc /*
			 * The subactions of this Action that are IfThenActions (including IfThenElseActions).
			 */
        }

        abstract action loops : LoopAction [0..*] :> subactions, loopActions {
            doc /*
			 * The subactions of this Action that are LoopActions.
			 */
        }

        abstract action whileLoops : WhileLoopAction [0..*] :> loops, whileLoopActions {
            doc /*
			 * The loops of this Action that are WhileLoopActions.
			 */
        }

        abstract action forLoops : ForLoopAction [0..*] :> loops, forLoopActions {
            doc /*
			 * The loops of this Action that are ForLoopActions.
			 */
        }
    }

    abstract action actions : Action [0..*] :> performances nonunique {
        doc /*
		 * actions is the base feature for all ActionUsages.
		 */
    }

    action def SendAction :> Action, SendPerformance {
        doc /*
		 * A SendAction is an Action used to type SendActionUsages. It initiates an outgoingTransferFromSelf 
		 * from a designated sender Occurrence with a given payload, optionally to a designated receiver.
		 */

        in :>> payload [0..*];
        ref sentMessage :>> sentTransfer : MessageTransfer, MessageAction {
            in :>> MessageTransfer::payload, MessageAction::payload;
        }
    }

    abstract action sendActions : SendAction [0..*] :> actions, sendPerformances nonunique {
        doc /*
		 * sendActions is the base feature for all SendActionUsages.
		 */
    }

    action def AcceptMessageAction :> Action, AcceptPerformance {
        doc /*
		 * An AcceptMessageAction is an Action that identifies an incomingTransferToSelf
		 * of a designated receiver Occurrence, providing its payload as output.
		 */
        inout :>> payload;
        ref acceptedMessage :>> acceptedTransfer : MessageTransfer, MessageAction {
            in :>> MessageTransfer::payload, MessageAction::payload;
        }
    }

    action def AcceptAction :> AcceptMessageAction {
        doc /*
		 * An AcceptAction is an AcceptMessageAction used to type AcceptActionUsages that are
		 * not accepters for TransitionActions. It waits for a payload or message of the specified 
		 * kind to be accepted by a nested state transition.
		 */
        ref :>> acceptedMessage = aState.aTransition.accepter.acceptedMessage;
        state aState {
            transition aTransition first start accept apayload : Anything via receiver then done;
        }
        bind payload = aState.aTransition.apayload;
    }

    abstract action acceptActions : AcceptAction [0..*] :> actions, acceptPerformances nonunique {
        doc /*
		 * acceptActions is the base feature for standalone AcceptActionUsages.
		 */
    }

    abstract action def TerminateAction :> Action {
        doc /*
		 * A TerminateAction is an Action that terminates a given Occurrence, meaning 
		 * that the Occurrence ends during the performance of this Action. TerminateAction
		 * is the base type for all TerminateActionUsages.
		 */

        in occurrence terminatedOccurrence [1] {
            doc /*
			 * The Occurrence to be terminated.
			 */
        }

        action terminateOccurrence : destroy [1] {
            in occ = terminatedOccurrence;
        }
    }

    abstract action terminateActions : TerminateAction [0..*] :> actions nonunique {
        doc /*
		 * terminateActions is the base feature for all TerminateActionUsages.
		 */

        in occurrence terminatedOccurrence default = that as Occurrence {
            doc /*
			 * The default terminatedOccurrence for a terminateAction is its
			 * featuring occurrence (which will generally be a containing Action).
			 */
        }
    }

    abstract action def ControlAction :> Action {
        doc /*
		 * A ControlAction is the Action of a control node, which has no inherent behavior.
		 */

        bind start = done {
            doc /*
			 * A ControlAction is instantaneous.
			 */
        }
    }

    action def MergeAction :> ControlAction, MergePerformance {
        doc /*
		 * A MergeAction is the ControlAction for a merge node.
		 * 
		 * Note: Incoming succession connectors to a MergeAction must have source multiplicity 
		 * 0..1 and subset the incomingHBLink feature inherited from MergePerformance.
		 */
    }

    action def DecisionAction :> ControlAction, DecisionPerformance {
        doc /*
		 * A DecisionAction is the ControlAction for a decision node.
		 * 
		 * Note: Outgoing succession connectors from a DecisionAction must have target multiplicity
		 * 0..1 and subset the outgoingHBLink feature inherited from DecisionPerformance.
		 * If an outgoing succession has a guard, it should have a transitionStep typed by 
		 * DecisionTransition.
		 */
    }

    action def JoinAction :> ControlAction {
        doc /*
		 * A JoinAction is the ControlAction for a JoinNode.
		 * 
		 * Note: Join behavior results from requiring that the source multiplicity of all
		 * incoming succession connectors be 1..1.
		 */
    }

    action def ForkAction :> ControlAction {
        doc /*
		 * A ForkAction is the ControlAction for a ForkNode.
		 * 
		 * Note: Fork behavior results from requiring that the target multiplicity of all
		 * outgoing succession connectors be 1..1.
		 */
    }

    abstract action def TransitionAction :> Action, TransitionPerformance {
        doc /*
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

        action effect : Action :>> TransitionPerformance::effect;
    }

    action def DecisionTransitionAction :> TransitionAction, NonStateTransitionPerformance {
        doc /*
		 * A DecisionTransitionAction is a TransitionAction and NonStateTransitionPerformance that has a 
		 * guard, but no trigger or effects. It is the base type of TransitionUsages used as 
		 * conditional successions in action models.
		 */

        ref action :>> accepter [0..0];
        ref action :>> effect [0..0];
    }

    abstract action transitionActions : TransitionAction [0..*] :> actions nonunique {
        doc /*
		 * transitionActions is the base feature for all TransitionUsages.
		 */
    }

    action def AssignmentAction :> FeatureWritePerformance, Action {
        doc /*
		 * An AssignmentAction is an Action, used to type an AssignmentActionUsage. It is also a
		 * FeatureWritePerformance that updates the accessedFeature of its target Occurrence with
		 * the given replacementValues.
		 */

        in target : Occurrence [1];
        inout replacementValues : Anything [0..*] nonunique;
    }

    abstract action assignmentActions : AssignmentAction [0..*] :> actions nonunique {
        doc /*
		 * assignmentActions is the base feature for all AssignmentActionsUsages.
		 */

        in target : Occurrence [1] default = that as Occurrence {
            doc /*
             * The default target for assignmentActions is its featuring instance (if that is 
             * an Occurrence).
             */
        }
    }

    action def IfThenAction :> Action, IfThenPerformance {
        doc /*
		 * An IfThenAction is a Kernel IfThenPerformance that is also an Action. 
		 * It is the base type for all IfActionUsages.
		 */

        in ifTest [1];
        in action thenClause[0..1];
    }

    action def IfThenElseAction :> IfThenAction, IfThenElsePerformance {
        doc /*
		 * An IfThenElseAction is a Kernel IfThenElsePeformance that is also an IfThenAction. 
		 * It is the base type for all IfActionUsages that have an elseAction.
		 */

        in ifTest [1];
        in action thenClause[0..1];
        in action elseClause[0..1];
    }

    abstract action ifThenActions : IfThenAction [0..*] :> actions nonunique {
        doc /*
		 * ifThenActions is the base feature for all IfActionUsages.
		 */
    }

    abstract action ifThenElseActions : IfThenElseAction [0..*] :> actions nonunique {
        doc /*
		 * ifThenElseActions is the base feature for all IfActionUsages that have an elseAction.
		 */
    }

    abstract action def LoopAction :> Action {
        doc /*
		 * A LoopAction is the base type for all LoopActionUsages.
		 */

        in ref iterator;

        in action body[0..*] {
            doc /*
			 * The action that is performed repeatedly in the loop.
			 */
        }
    }

    action def WhileLoopAction :> LoopAction, LoopPerformance {
        doc /*
		 * A WhileLoopAction is a Kernel LoopPerformance that is also a LoopAction.
		 * It is the base type for all WhileLoopActionUsages.
		 */

        in whileTest default = {true} {
            doc /*
			 * A Boolean expression that must be true for the loop to continue.
			 * It is evaluated before the body is performed and is always evaluated at 
			 * least once.
			 */
        }

        in action body {
            doc /*
			 * The action that is performed while the whileTest is true and the
			 * untilTest is false.
			 */
        }

        in untilTest default = {false} {
            doc /*
			 * A Boolean expression that must be false for the loop to continue.
			 * It is evaluated after the body is performed.
			 */
        }
    }

    action def ForLoopAction :> LoopAction {
        doc /*
		 * A ForLoopAction is a LoopAction that iterates over an ordered sequence of values.
		 * It is the base type for all ForLoopActionUsages.
		 */

        protected ref var :> seq [0..1] {
            doc /*
			 * The loop variable that is assigned successive elements of seq on each
			 * iteration of the loop.
			 */
        }

        in ref seq {
            doc /*
			 * The sequence of values over which the loop iterates.
			 */
        }

        in action body {
            doc /*
			 * The action that is performed on each iteration of the loop.
			 */
        }

        private attribute index : Positive {
            doc /*
			 * The index of the element of seq assigned to var on the current iteration
			 * of the loop.
			 */
        }

        private action initialization;
        assign index := 1;
        then private action whileLoop
        while index <= size(seq) {
            assign :=;
            var  := seq#(index);
            then perform body;
            then assign index := index + 1;
        }
    }

    abstract action loopActions : LoopAction [0..*] :> actions nonunique {
        doc /*
		 * loopActions is the base feature for all LoopActionUsages.
		 */
    }

    abstract action whileLoopActions : WhileLoopAction [0..*] :> loopActions nonunique {
        doc /*
		 * whileLoopActions is the base feature for all WhileLoopActionUsages.
		 */
    }

    abstract action forLoopActions : ForLoopAction [0..*] :> loopActions nonunique {
        doc /*
		 * forLoopActions is the base feature for all ForLoopActionUsages.
		 */
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'Actions'
      (documentation)
      (membership_import private -> 'Base::Anything'[unresolved])
      (membership_import private -> 'ScalarValues::Positive'[unresolved])
      (membership_import private -> 'ScalarValues::Natural'[unresolved])
      (membership_import private -> 'SequenceFunctions::size'[unresolved])
      (membership_import private -> 'SequenceFunctions::isEmpty'[unresolved])
      (membership_import private -> 'Occurrences::Occurrence'[unresolved])
      (membership_import private -> 'Occurrences::HappensWhile'[unresolved])
      (membership_import private -> 'Performances::Performance'[unresolved])
      (membership_import private -> 'Performances::performances'[unresolved])
      (membership_import private -> 'Transfers::SendPerformance'[unresolved])
      (membership_import private -> 'Transfers::sendPerformances'[unresolved])
      (membership_import private -> 'Transfers::AcceptPerformance'[unresolved])
      (membership_import private -> 'Transfers::acceptPerformances'[unresolved])
      (membership_import private -> 'FeatureReferencingPerformances::FeatureWritePerformance'[unresolved])
      (membership_import private -> 'ControlPerformances::MergePerformance'[unresolved])
      (membership_import private -> 'ControlPerformances::DecisionPerformance'[unresolved])
      (membership_import private -> 'ControlPerformances::IfThenPerformance'[unresolved])
      (membership_import private -> 'ControlPerformances::IfThenElsePerformance'[unresolved])
      (membership_import private -> 'ControlPerformances::LoopPerformance'[unresolved])
      (membership_import private -> 'TransitionPerformances::TransitionPerformance'[unresolved])
      (membership_import private -> 'TransitionPerformances::NonStateTransitionPerformance'[unresolved])
      (membership_import private -> 'Transfers::MessageTransfer'[unresolved])
      (membership_import private -> 'Flows::MessageAction'[unresolved])
      (membership_import private -> 'OccurrenceFunctions::destroy'[unresolved])
      (action_def abstract 'Action' :> 'Performance'[unresolved]
        (documentation)
        (action_usage reference 'self' : 'Actions::Action'[action_def] :>> 'Performance::self'[unresolved] :> 'Actions::actions'[action_usage][implied])
        (action_usage reference 'incomingTransfers' :>> 'Performance::incomingTransfers'[unresolved] :> 'Actions::actions'[action_usage][implied])
        (action_usage composite 'start' : 'Actions::Action'[action_def] :>> 'startShot'[unresolved] :> 'Actions::Action::subactions'[action_usage][implied]
          (documentation))
        (action_usage composite 'done' : 'Actions::Action'[action_def] :>> 'endShot'[unresolved] :> 'Actions::Action::subactions'[action_usage][implied]
          (documentation))
        (action_usage composite 'subactions' : 'Actions::Action'[action_def] :> 'Actions::actions'[action_usage] :> 'subperformances'[unresolved]
          (multiplicity_range [0..*])
          (documentation)
          (occurrence_usage reference :>> 'Action::this'[unresolved] :>> 'actions::this'[unresolved] :>> 'subperformances::this'[unresolved]
            (feature_value (=))
            (documentation)))
        (action_usage composite 'sendSubactions' : 'Actions::SendAction'[action_def] :> 'Actions::Action::subactions'[action_usage] :> 'Actions::sendActions'[action_usage]
          (multiplicity_range [0..*])
          (documentation))
        (action_usage composite 'acceptSubactions' : 'Actions::AcceptAction'[action_def] :> 'Actions::Action::subactions'[action_usage] :> 'Actions::acceptActions'[action_usage]
          (multiplicity_range [0..*])
          (documentation))
        (action_usage abstract composite 'terminateSubactions' : 'Actions::TerminateAction'[action_def] :> 'Actions::Action::subactions'[action_usage] :> 'Actions::terminateActions'[action_usage]
          (multiplicity_range [0..*])
          (documentation))
        (action_usage abstract composite 'controls' : 'Actions::ControlAction'[action_def] :> 'Actions::Action::subactions'[action_usage]
          (multiplicity_range [0..*])
          (documentation))
        (action_usage abstract composite 'merges' : 'Actions::MergeAction'[action_def] :> 'Actions::Action::controls'[action_usage]
          (multiplicity_range [0..*])
          (documentation))
        (action_usage abstract composite 'decisions' : 'Actions::DecisionAction'[action_def] :> 'Actions::Action::controls'[action_usage]
          (documentation))
        (action_usage abstract composite 'joins' : 'Actions::JoinAction'[action_def] :> 'Actions::Action::controls'[action_usage]
          (documentation))
        (action_usage abstract composite 'forks' : 'Actions::ForkAction'[action_def] :> 'Actions::Action::controls'[action_usage]
          (documentation))
        (action_usage abstract composite 'transitions' : 'Actions::TransitionAction'[action_def] :> 'Actions::Action::subactions'[action_usage] :> 'Actions::transitionActions'[action_usage]
          (multiplicity_range [0..*])
          (documentation))
        (action_usage abstract composite 'decisionTransitions' : 'Actions::DecisionTransitionAction'[action_def] :> 'Actions::Action::transitions'[action_usage]
          (multiplicity_range [0..*])
          (documentation))
        (action_usage abstract composite 'assignments' : 'Actions::AssignmentAction'[action_def] :> 'Actions::Action::subactions'[action_usage] :> 'Actions::assignmentActions'[action_usage]
          (multiplicity_range [0..*])
          (documentation)
          (reference_usage in reference 'target' :>> 'Actions::assignmentActions::target'[reference_usage][implied]))
        (action_usage abstract composite 'ifSubactions' : 'Actions::IfThenAction'[action_def] :> 'Actions::Action::subactions'[action_usage] :> 'Actions::ifThenActions'[action_usage]
          (multiplicity_range [0..*])
          (documentation))
        (action_usage abstract composite 'loops' : 'Actions::LoopAction'[action_def] :> 'Actions::Action::subactions'[action_usage] :> 'Actions::loopActions'[action_usage]
          (multiplicity_range [0..*])
          (documentation))
        (action_usage abstract composite 'whileLoops' : 'Actions::WhileLoopAction'[action_def] :> 'Actions::Action::loops'[action_usage] :> 'Actions::whileLoopActions'[action_usage]
          (multiplicity_range [0..*])
          (documentation))
        (action_usage abstract composite 'forLoops' : 'Actions::ForLoopAction'[action_def] :> 'Actions::Action::loops'[action_usage] :> 'Actions::forLoopActions'[action_usage]
          (multiplicity_range [0..*])
          (documentation)))
      (action_usage abstract 'actions' : 'Actions::Action'[action_def] :> 'performances'[unresolved]
        (multiplicity_range [0..*])
        (documentation))
      (action_def 'SendAction' :> 'Actions::Action'[action_def] :> 'SendPerformance'[unresolved]
        (documentation)
        (reference_usage in reference :>> 'payload'[unresolved]
          (multiplicity_range [0..*]))
        (reference_usage reference 'sentMessage' :>> 'sentTransfer'[unresolved] : 'MessageTransfer'[unresolved] : 'MessageAction'[unresolved]
          (reference_usage in reference :>> 'MessageTransfer::payload'[unresolved] :>> 'MessageAction::payload'[unresolved])))
      (action_usage abstract 'sendActions' : 'Actions::SendAction'[action_def] :> 'Actions::actions'[action_usage] :> 'sendPerformances'[unresolved]
        (multiplicity_range [0..*])
        (documentation))
      (action_def 'AcceptMessageAction' :> 'Actions::Action'[action_def] :> 'AcceptPerformance'[unresolved]
        (documentation)
        (reference_usage inout reference :>> 'payload'[unresolved])
        (reference_usage reference 'acceptedMessage' :>> 'acceptedTransfer'[unresolved] : 'MessageTransfer'[unresolved] : 'MessageAction'[unresolved]
          (reference_usage in reference :>> 'MessageTransfer::payload'[unresolved] :>> 'MessageAction::payload'[unresolved])))
      (action_def 'AcceptAction' :> 'Actions::AcceptMessageAction'[action_def]
        (documentation)
        (reference_usage reference :>> 'Actions::AcceptMessageAction::acceptedMessage'[reference_usage]
          (feature_value (=)))
        (state_usage composite 'aState'
          (transition_usage 'aTransition'))
        (binding_connector_def
          (connector_end 'payload')
          (connector_end 'aState.aTransition.apayload')))
      (action_usage abstract 'acceptActions' : 'Actions::AcceptAction'[action_def] :> 'Actions::actions'[action_usage] :> 'acceptPerformances'[unresolved]
        (multiplicity_range [0..*])
        (documentation))
      (action_def abstract 'TerminateAction' :> 'Actions::Action'[action_def]
        (documentation)
        (occurrence_usage in 'terminatedOccurrence'
          (multiplicity_range [1])
          (documentation))
        (action_usage composite 'terminateOccurrence' : 'destroy'[unresolved] :> 'Actions::Action::subactions'[action_usage][implied]
          (multiplicity_range [1])
          (reference_usage in reference 'occ'
            (feature_value (=)))))
      (action_usage abstract 'terminateActions' : 'Actions::TerminateAction'[action_def] :> 'Actions::actions'[action_usage]
        (multiplicity_range [0..*])
        (documentation)
        (occurrence_usage in 'terminatedOccurrence'
          (feature_value (default =))
          (documentation)))
      (action_def abstract 'ControlAction' :> 'Actions::Action'[action_def]
        (documentation)
        (binding_connector_def
          (connector_end 'start')
          (connector_end 'done')
          (documentation)))
      (action_def 'MergeAction' :> 'Actions::ControlAction'[action_def] :> 'MergePerformance'[unresolved]
        (documentation))
      (action_def 'DecisionAction' :> 'Actions::ControlAction'[action_def] :> 'DecisionPerformance'[unresolved]
        (documentation))
      (action_def 'JoinAction' :> 'Actions::ControlAction'[action_def]
        (documentation))
      (action_def 'ForkAction' :> 'Actions::ControlAction'[action_def]
        (documentation))
      (action_def abstract 'TransitionAction' :> 'Actions::Action'[action_def] :> 'TransitionPerformance'[unresolved]
        (documentation)
        (reference_usage in reference 'transitionLinkSource' : 'Actions::Action'[action_def] :>> 'TransitionPerformance::transitionLinkSource'[unresolved])
        (reference_usage reference 'acceptedMessage' : 'MessageTransfer'[unresolved] : 'MessageAction'[unresolved] :>> 'trigger'[unresolved]
          (reference_usage in reference :>> 'MessageTransfer::payload'[unresolved] :>> 'MessageAction::payload'[unresolved]))
        (reference_usage reference 'receiver' :>> 'triggerTarget'[unresolved])
        (action_usage composite 'accepter' : 'Actions::AcceptMessageAction'[action_def] :>> 'accept'[unresolved] :> 'Actions::Action::subactions'[action_usage][implied])
        (binding_connector_def
          (connector_end 'receiver')
          (connector_end 'accepter.receiver'))
        (binding_connector_def
          (connector_end 'acceptedMessage')
          (connector_end 'accepter.acceptedMessage'))
        (action_usage composite 'effect' : 'Actions::Action'[action_def] :>> 'TransitionPerformance::effect'[unresolved] :> 'Actions::Action::subactions'[action_usage][implied]))
      (action_def 'DecisionTransitionAction' :> 'Actions::TransitionAction'[action_def] :> 'NonStateTransitionPerformance'[unresolved]
        (documentation)
        (action_usage reference :>> 'Actions::TransitionAction::accepter'[action_usage]
          (multiplicity_range [0..0]))
        (action_usage reference :>> 'Actions::TransitionAction::effect'[action_usage]
          (multiplicity_range [0..0])))
      (action_usage abstract 'transitionActions' : 'Actions::TransitionAction'[action_def] :> 'Actions::actions'[action_usage]
        (multiplicity_range [0..*])
        (documentation))
      (action_def 'AssignmentAction' :> 'FeatureWritePerformance'[unresolved] :> 'Actions::Action'[action_def]
        (documentation)
        (reference_usage in reference 'target' : 'Occurrence'[unresolved]
          (multiplicity_range [1]))
        (reference_usage inout reference 'replacementValues' : 'Anything'[unresolved]
          (multiplicity_range [0..*])))
      (action_usage abstract 'assignmentActions' : 'Actions::AssignmentAction'[action_def] :> 'Actions::actions'[action_usage]
        (multiplicity_range [0..*])
        (documentation)
        (reference_usage in reference 'target' : 'Occurrence'[unresolved]
          (multiplicity_range [1])
          (feature_value (default =))
          (documentation)))
      (action_def 'IfThenAction' :> 'Actions::Action'[action_def] :> 'IfThenPerformance'[unresolved]
        (documentation)
        (reference_usage in reference 'ifTest'
          (multiplicity_range [1]))
        (action_usage in 'thenClause' :> 'Actions::actions'[action_usage][implied]
          (multiplicity_range [0..1])))
      (action_def 'IfThenElseAction' :> 'Actions::IfThenAction'[action_def] :> 'IfThenElsePerformance'[unresolved]
        (documentation)
        (reference_usage in reference 'ifTest' :>> 'Actions::IfThenAction::ifTest'[reference_usage][implied]
          (multiplicity_range [1]))
        (action_usage in 'thenClause' :>> 'Actions::IfThenAction::thenClause'[action_usage][implied]
          (multiplicity_range [0..1]))
        (action_usage in 'elseClause' :> 'Actions::actions'[action_usage][implied]
          (multiplicity_range [0..1])))
      (action_usage abstract 'ifThenActions' : 'Actions::IfThenAction'[action_def] :> 'Actions::actions'[action_usage]
        (multiplicity_range [0..*])
        (documentation))
      (action_usage abstract 'ifThenElseActions' : 'Actions::IfThenElseAction'[action_def] :> 'Actions::actions'[action_usage]
        (multiplicity_range [0..*])
        (documentation))
      (action_def abstract 'LoopAction' :> 'Actions::Action'[action_def]
        (documentation)
        (reference_usage in reference 'iterator')
        (action_usage in 'body' :> 'Actions::actions'[action_usage][implied]
          (multiplicity_range [0..*])
          (documentation)))
      (action_def 'WhileLoopAction' :> 'Actions::LoopAction'[action_def] :> 'LoopPerformance'[unresolved]
        (documentation)
        (reference_usage in reference 'whileTest' :>> 'Actions::LoopAction::iterator'[reference_usage][implied]
          (feature_value (default =))
          (documentation))
        (action_usage in 'body' :>> 'Actions::LoopAction::body'[action_usage][implied]
          (documentation))
        (reference_usage in reference 'untilTest'
          (feature_value (default =))
          (documentation)))
      (action_def 'ForLoopAction' :> 'Actions::LoopAction'[action_def]
        (documentation)
        (reference_usage reference 'var' :> 'Actions::ForLoopAction::seq'[reference_usage]
          (multiplicity_range [0..1])
          (documentation))
        (reference_usage in reference 'seq' :>> 'Actions::LoopAction::iterator'[reference_usage][implied]
          (documentation))
        (action_usage in 'body' :>> 'Actions::LoopAction::body'[action_usage][implied]
          (documentation))
        (attribute_usage composite 'index' : 'Positive'[unresolved]
          (documentation))
        (action_usage composite 'initialization' :> 'Actions::Action::subactions'[action_usage][implied])
        (assignment_action_usage)
        (source_succession
          (action_usage 'whileLoop' :> 'Actions::actions'[action_usage][implied]))
        (while_loop_action_usage
          (assignment_action_usage)
          (feature_def
            (feature_value (:=)))
          (source_succession
            (perform_action_usage :>> 'Actions::ForLoopAction::body'[action_usage]))
          (source_succession
            (assignment_action_usage))))
      (action_usage abstract 'loopActions' : 'Actions::LoopAction'[action_def] :> 'Actions::actions'[action_usage]
        (multiplicity_range [0..*])
        (documentation))
      (action_usage abstract 'whileLoopActions' : 'Actions::WhileLoopAction'[action_def] :> 'Actions::loopActions'[action_usage]
        (multiplicity_range [0..*])
        (documentation))
      (action_usage abstract 'forLoopActions' : 'Actions::ForLoopAction'[action_def] :> 'Actions::loopActions'[action_usage]
        (multiplicity_range [0..*])
        (documentation)))))
~~~
