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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/actions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 16) (end 9 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 16) (end 10 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 16) (end 11 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 12 16) (end 12 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 13 16) (end 13 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 14 16) (end 14 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 15 16) (end 15 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 16 16) (end 16 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 17 16) (end 17 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 18 16) (end 18 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 19 16) (end 19 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 20 16) (end 20 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 21 16) (end 21 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 22 16) (end 22 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 23 16) (end 23 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 24 16) (end 24 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 25 16) (end 25 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 26 16) (end 26 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 27 16) (end 27 69))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 28 16) (end 28 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 29 16) (end 29 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 30 16) (end 30 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 32 31) (end 32 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 40 35) (end 40 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 42 27) (end 42 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 49 26) (end 49 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 56 46) (end 56 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 62 22) (end 62 34))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference_usage_member")
        (source "semantic")
        (range (start 63 4) (end 67 7))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 179 52) (end 179 64))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 186 34) (end 186 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 193 9) (end 193 16))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 194 25) (end 194 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 194 39) (end 194 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 194 56) (end 194 69))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference_usage_member")
        (source "semantic")
        (range (start 195 9) (end 195 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 199 69) (end 199 85))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 206 43) (end 206 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 212 12) (end 212 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 213 26) (end 213 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 213 44) (end 213 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 213 61) (end 213 74))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference_usage_member")
        (source "semantic")
        (range (start 214 12) (end 214 68))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 227 3) (end 227 87))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 227 32) (end 227 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 227 82) (end 227 86))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 229 7) (end 229 14))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 229 17) (end 229 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 232 73) (end 232 91))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 254 31) (end 254 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 280 7) (end 280 12))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 280 15) (end 280 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 288 42) (end 288 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 298 45) (end 298 64))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 330 49) (end 330 70))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 337 39) (end 337 82))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 338 24) (end 338 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 338 41) (end 338 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference_usage_member")
        (source "semantic")
        (range (start 339 12) (end 339 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 342 19) (end 342 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 344 44) (end 344 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 346 18) (end 346 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 349 28) (end 349 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 352 58) (end 352 87))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 371 32) (end 371 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 379 14) (end 379 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 380 28) (end 380 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 389 20) (end 389 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 389 42) (end 389 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 389 50) (end 389 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 398 36) (end 398 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 409 46) (end 409 67))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 451 43) (end 451 58))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 458 23) (end 458 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 475 23) (end 475 30))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference_usage_member")
        (source "semantic")
        (range (start 492 3) (end 496 6))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 513 2) (end 519 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 522 10) (end 522 15))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 524 9) (end 524 14))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 524 18) (end 524 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 525 23) (end 525 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 527 16) (end 527 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 527 25) (end 527 30))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:0683383b61e47daf5fc3d06f372c78670abac17c7ed407cc15dcc1a8429a1ac8") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions"))) (kind library-package) (membership (kind owning) (visibility default)) (facts (modifiers standard)) (documentation (doc (text "\n\t * This package defines the base types for actions and related behavioral elements in the\n\t * SysML language.\n\t "))))
    (declaration (id (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Base::Anything") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Positive") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Natural") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::size") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::isEmpty") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 5))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Occurrences::Occurrence") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 6))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Occurrences::HappensWhile") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 7))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Performances::Performance") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 8))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Performances::performances") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 9))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Transfers::SendPerformance") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 10))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Transfers::sendPerformances") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 11))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Transfers::AcceptPerformance") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 12))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Transfers::acceptPerformances") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 13))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "FeatureReferencingPerformances::FeatureWritePerformance") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 14))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ControlPerformances::MergePerformance") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 15))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ControlPerformances::DecisionPerformance") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 16))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ControlPerformances::IfThenPerformance") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 17))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ControlPerformances::IfThenElsePerformance") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 18))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ControlPerformances::LoopPerformance") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 19))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "TransitionPerformances::TransitionPerformance") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 20))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "TransitionPerformances::NonStateTransitionPerformance") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 21))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Transfers::MessageTransfer") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 22))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Flows::MessageAction") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 23))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "OccurrenceFunctions::destroy") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptAction"))) (kind action-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * An AcceptAction is an AcceptMessageAction used to type AcceptActionUsages that are\n\t\t * not accepters for TransitionActions. It waits for a payload or message of the specified \n\t\t * kind to be accepted by a nested state transition.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "AcceptMessageAction")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptAction")) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "acceptedMessage")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptAction")) (anonymous (kind bind) (ordinal 0))))) (kind bind) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindSource (reference "payload")) (memberAccessOperand (reference "aState::aTransition::apayload")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptAction::aState"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptAction::aState::aTransition"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "start")) (transitionTarget (reference "done")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptMessageAction"))) (kind action-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * An AcceptMessageAction is an Action that identifies an incomingTransferToSelf\n\t\t * of a designated receiver Occurrence, providing its payload as output.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Action")) (specialization (reference "AcceptPerformance")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction inout)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "payload")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptMessageAction::acceptedMessage"))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MessageTransfer")) (featureTyping (reference "MessageAction")) (redefinition (reference "acceptedTransfer")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action"))) (kind action-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * Action is the most general class of Performances of ActionDefinitions in a system or \n\t\t * part of a system. Action is the base class of all ActionDefinitions.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Performance")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::acceptSubactions"))) (kind action) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n\t\t\t * The subactions of this Action that are AcceptActions. \n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AcceptAction")) (subsetting (reference "subactions")) (subsetting (reference "acceptActions")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::assignments"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers abstract) (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n\t\t\t * The subactions of this Action that are AssignmentActions.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AssignmentAction")) (subsetting (reference "subactions")) (subsetting (reference "assignmentActions")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::assignments::target"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::controls"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers abstract) (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n\t\t\t * The subactions of this Action that are ControlActions.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ControlAction")) (subsetting (reference "subactions")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::decisionTransitions"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers abstract) (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n\t\t\t * The subactions of this Action that are DecisionTransitionActions. \n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DecisionTransitionAction")) (subsetting (reference "transitions")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::decisions"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers abstract)) (documentation (doc (text "\n\t\t\t * The controls of this Action that are DecisionActions.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DecisionAction")) (subsetting (reference "controls")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::done"))) (kind action) (membership (kind feature) (visibility default)) (documentation (doc (text "\n\t\t\t * The ending snapshot of an Action.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Action")) (redefinition (reference "endShot")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::forLoops"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers abstract) (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n\t\t\t * The loops of this Action that are ForLoopActions.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ForLoopAction")) (subsetting (reference "loops")) (subsetting (reference "forLoopActions")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::forks"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers abstract)) (documentation (doc (text "\n\t\t\t * The controls of this Action that are ForkActions.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ForkAction")) (subsetting (reference "controls")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::ifSubactions"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers abstract) (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n\t\t\t * The subactions of this Action that are IfThenActions (including IfThenElseActions).\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "IfThenAction")) (subsetting (reference "subactions")) (subsetting (reference "ifThenActions")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::incomingTransfers"))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "Performance::incomingTransfers")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::joins"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers abstract)) (documentation (doc (text "\n\t\t\t * The controls of this Action that are JoinActions.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "JoinAction")) (subsetting (reference "controls")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::loops"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers abstract) (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n\t\t\t * The subactions of this Action that are LoopActions.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LoopAction")) (subsetting (reference "subactions")) (subsetting (reference "loopActions")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::merges"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers abstract) (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n\t\t\t * The controls of this Action that are MergeActions.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MergeAction")) (subsetting (reference "controls")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::self"))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Action")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::sendSubactions"))) (kind action) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n\t\t\t * The subactions of this Action that are SendActions. \n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SendAction")) (subsetting (reference "subactions")) (subsetting (reference "sendActions")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::start"))) (kind action) (membership (kind feature) (visibility default)) (documentation (doc (text "\n\t\t\t * The starting snapshot of an Action. \n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Action")) (redefinition (reference "startShot")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions"))) (kind action) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n\t\t\t * The subperformances of this Action that are Actions. \n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Action")) (subsetting (reference "actions")) (subsetting (reference "subperformances")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions::occurrence"))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "Action::this")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::terminateSubactions"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers abstract) (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n\t\t\t * The subactions of this Action that are TerminateActions.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TerminateAction")) (subsetting (reference "subactions")) (subsetting (reference "terminateActions")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::transitions"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers abstract) (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n\t\t\t * The subactions of this Action that are TransitionActions. \n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TransitionAction")) (subsetting (reference "subactions")) (subsetting (reference "transitionActions")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::whileLoops"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers abstract) (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n\t\t\t * The loops of this Action that are WhileLoopActions.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "WhileLoopAction")) (subsetting (reference "loops")) (subsetting (reference "whileLoopActions")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AssignmentAction"))) (kind action-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * An AssignmentAction is an Action, used to type an AssignmentActionUsage. It is also a\n\t\t * FeatureWritePerformance that updates the accessedFeature of its target Occurrence with\n\t\t * the given replacementValues.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "FeatureWritePerformance")) (specialization (reference "Action")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AssignmentAction::replacementValues"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (modifiers nonunique) (direction inout) (multiplicity (lower 0) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Anything") (direction inout)))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AssignmentAction::target"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence") (direction in)))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ControlAction"))) (kind action-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * A ControlAction is the Action of a control node, which has no inherent behavior.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Action")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "ControlAction")) (anonymous (kind bind) (ordinal 0))))) (kind bind) (membership (kind feature) (visibility default)) (documentation (doc (text "\n\t\t\t * A ControlAction is instantaneous.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (bindSource (reference "start")) (bindTarget (reference "done")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::DecisionAction"))) (kind action-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * A DecisionAction is the ControlAction for a decision node.\n\t\t * \n\t\t * Note: Outgoing succession connectors from a DecisionAction must have target multiplicity\n\t\t * 0..1 and subset the outgoingHBLink feature inherited from DecisionPerformance.\n\t\t * If an outgoing succession has a guard, it should have a transitionStep typed by \n\t\t * DecisionTransition.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ControlAction")) (specialization (reference "DecisionPerformance")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::DecisionTransitionAction"))) (kind action-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * A DecisionTransitionAction is a TransitionAction and NonStateTransitionPerformance that has a \n\t\t * guard, but no trigger or effects. It is the base type of TransitionUsages used as \n\t\t * conditional successions in action models.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "TransitionAction")) (specialization (reference "NonStateTransitionPerformance")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "DecisionTransitionAction")) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "accepter")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "DecisionTransitionAction")) (anonymous (kind ref) (ordinal 1))))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "effect")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction"))) (kind action-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * A ForLoopAction is a LoopAction that iterates over an ordered sequence of values.\n\t\t * It is the base type for all ForLoopActionUsages.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "LoopAction")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "ForLoopAction")) (anonymous (kind assign) (ordinal 0))))) (kind assign) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (assignTarget (reference "index")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "ForLoopAction")) (anonymous (kind while) (ordinal 0))))) (kind while) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "index")) (expressionOperand (reference "seq")) (invocationCallee (reference "size")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "ForLoopAction")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (kind assign) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "seq")) (expressionOperand (reference "index")) (assignTarget (reference "var")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "ForLoopAction")) (anonymous (kind while) (ordinal 0)) (anonymous (kind perform-action) (ordinal 0))))) (kind perform-action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "ForLoopAction")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 1))))) (kind assign) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "index")) (assignTarget (reference "index")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction::body"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction::initialization"))) (kind action) (membership (kind feature) (visibility private)))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction::seq"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (modifiers reference) (direction in)))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction::var"))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "seq")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction::whileLoop"))) (kind action) (membership (kind feature) (visibility private)))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForkAction"))) (kind action-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * A ForkAction is the ControlAction for a ForkNode.\n\t\t * \n\t\t * Note: Fork behavior results from requiring that the target multiplicity of all\n\t\t * outgoing succession connectors be 1..1.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ControlAction")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenAction"))) (kind action-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * An IfThenAction is a Kernel IfThenPerformance that is also an Action. \n\t\t * It is the base type for all IfActionUsages.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Action")) (specialization (reference "IfThenPerformance")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenAction::ifTest"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in) (multiplicity (lower 1) (upper 1))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenAction::thenClause"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in) (multiplicity (lower 0) (upper 1))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenElseAction"))) (kind action-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * An IfThenElseAction is a Kernel IfThenElsePeformance that is also an IfThenAction. \n\t\t * It is the base type for all IfActionUsages that have an elseAction.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "IfThenAction")) (specialization (reference "IfThenElsePerformance")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenElseAction::elseClause"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in) (multiplicity (lower 0) (upper 1))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenElseAction::ifTest"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in) (multiplicity (lower 1) (upper 1))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenElseAction::thenClause"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in) (multiplicity (lower 0) (upper 1))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::JoinAction"))) (kind action-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * A JoinAction is the ControlAction for a JoinNode.\n\t\t * \n\t\t * Note: Join behavior results from requiring that the source multiplicity of all\n\t\t * incoming succession connectors be 1..1.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ControlAction")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction"))) (kind action-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * A LoopAction is the base type for all LoopActionUsages.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Action")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction::body"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in) (multiplicity (lower 0) (upper unbounded))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction::iterator"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (modifiers reference) (direction in)))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::MergeAction"))) (kind action-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * A MergeAction is the ControlAction for a merge node.\n\t\t * \n\t\t * Note: Incoming succession connectors to a MergeAction must have source multiplicity \n\t\t * 0..1 and subset the incomingHBLink feature inherited from MergePerformance.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ControlAction")) (specialization (reference "MergePerformance")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::SendAction"))) (kind action-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * A SendAction is an Action used to type SendActionUsages. It initiates an outgoingTransferFromSelf \n\t\t * from a designated sender Occurrence with a given payload, optionally to a designated receiver.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Action")) (specialization (reference "SendPerformance")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "SendAction")) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in) (multiplicity (lower 0) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "payload")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::SendAction::sentMessage"))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MessageTransfer")) (featureTyping (reference "MessageAction")) (redefinition (reference "sentTransfer")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TerminateAction"))) (kind action-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * A TerminateAction is an Action that terminates a given Occurrence, meaning \n\t\t * that the Occurrence ends during the performance of this Action. TerminateAction\n\t\t * is the base type for all TerminateActionUsages.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Action")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TerminateAction::terminateOccurrence"))) (kind action) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "destroy")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TerminateAction::terminateOccurrence::occ"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "terminatedOccurrence")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TerminateAction::terminatedOccurrence"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (direction in) (multiplicity (lower 1) (upper 1))) (documentation (doc (text "\n\t\t\t * The Occurrence to be terminated.\n\t\t\t "))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction"))) (kind action-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * A TransitionAction is a TransitionPerformance with an Action as transitionLinkSource.\n\t\t * It is the base type of all TransitionUsages.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Action")) (specialization (reference "TransitionPerformance")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "TransitionAction")) (anonymous (kind bind) (ordinal 0))))) (kind bind) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindSource (reference "receiver")) (memberAccessOperand (reference "accepter::receiver")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "TransitionAction")) (anonymous (kind bind) (ordinal 1))))) (kind bind) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindSource (reference "acceptedMessage")) (memberAccessOperand (reference "accepter::acceptedMessage")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::acceptedMessage"))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MessageTransfer")) (featureTyping (reference "MessageAction")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::accepter"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AcceptMessageAction")) (redefinition (reference "accept")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::effect"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Action")) (redefinition (reference "TransitionPerformance::effect")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::receiver"))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "triggerTarget")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::transitionLinkSource"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Action") (direction in)) (redefinition (reference "TransitionPerformance::transitionLinkSource")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::WhileLoopAction"))) (kind action-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * A WhileLoopAction is a Kernel LoopPerformance that is also a LoopAction.\n\t\t * It is the base type for all WhileLoopActionUsages.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "LoopAction")) (specialization (reference "LoopPerformance")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::WhileLoopAction::body"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::WhileLoopAction::untilTest"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (feature-value (kind bind) (default true) (operator false)))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::WhileLoopAction::whileTest"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (feature-value (kind bind) (default true) (operator false)))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::acceptActions"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers abstract) (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n\t\t * acceptActions is the base feature for standalone AcceptActionUsages.\n\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AcceptAction")) (subsetting (reference "actions")) (subsetting (reference "acceptPerformances")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers abstract) (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n\t\t * actions is the base feature for all ActionUsages.\n\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Action")) (subsetting (reference "performances")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::assignmentActions"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers abstract) (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n\t\t * assignmentActions is the base feature for all AssignmentActionsUsages.\n\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AssignmentAction")) (subsetting (reference "actions")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::assignmentActions::target"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in) (multiplicity (lower 1) (upper 1))) (feature-value (kind bind) (default true) (operator false)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence") (direction in)) (expressionOperand (reference "that")) (typeCheckTarget (reference "Occurrence")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::forLoopActions"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers abstract) (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n\t\t * forLoopActions is the base feature for all ForLoopActionUsages.\n\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ForLoopAction")) (subsetting (reference "loopActions")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ifThenActions"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers abstract) (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n\t\t * ifThenActions is the base feature for all IfActionUsages.\n\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "IfThenAction")) (subsetting (reference "actions")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ifThenElseActions"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers abstract) (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n\t\t * ifThenElseActions is the base feature for all IfActionUsages that have an elseAction.\n\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "IfThenElseAction")) (subsetting (reference "actions")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::loopActions"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers abstract) (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n\t\t * loopActions is the base feature for all LoopActionUsages.\n\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LoopAction")) (subsetting (reference "actions")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::sendActions"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers abstract) (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n\t\t * sendActions is the base feature for all SendActionUsages.\n\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SendAction")) (subsetting (reference "actions")) (subsetting (reference "sendPerformances")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::terminateActions"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers abstract) (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n\t\t * terminateActions is the base feature for all TerminateActionUsages.\n\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TerminateAction")) (subsetting (reference "actions")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::terminateActions::terminatedOccurrence"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (direction in)) (documentation (doc (text "\n\t\t\t * The default terminatedOccurrence for a terminateAction is its\n\t\t\t * featuring occurrence (which will generally be a containing Action).\n\t\t\t "))) (feature-value (kind bind) (default true) (operator false)))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::transitionActions"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers abstract) (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n\t\t * transitionActions is the base feature for all TransitionUsages.\n\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TransitionAction")) (subsetting (reference "actions")))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::whileLoopActions"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers abstract) (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n\t\t * whileLoopActions is the base feature for all WhileLoopActionUsages.\n\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "WhileLoopAction")) (subsetting (reference "loopActions")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Base::Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Positive")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Natural")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::size")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::isEmpty")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0))
      (authored-target "Occurrences::Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0))
      (authored-target "Occurrences::HappensWhile")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0))
      (authored-target "Performances::Performance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 8))))) (kind membershipImport) (ordinal 0))
      (authored-target "Performances::performances")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 9))))) (kind membershipImport) (ordinal 0))
      (authored-target "Transfers::SendPerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 10))))) (kind membershipImport) (ordinal 0))
      (authored-target "Transfers::sendPerformances")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 11))))) (kind membershipImport) (ordinal 0))
      (authored-target "Transfers::AcceptPerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 12))))) (kind membershipImport) (ordinal 0))
      (authored-target "Transfers::acceptPerformances")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 13))))) (kind membershipImport) (ordinal 0))
      (authored-target "FeatureReferencingPerformances::FeatureWritePerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 14))))) (kind membershipImport) (ordinal 0))
      (authored-target "ControlPerformances::MergePerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 15))))) (kind membershipImport) (ordinal 0))
      (authored-target "ControlPerformances::DecisionPerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 16))))) (kind membershipImport) (ordinal 0))
      (authored-target "ControlPerformances::IfThenPerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 17))))) (kind membershipImport) (ordinal 0))
      (authored-target "ControlPerformances::IfThenElsePerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 18))))) (kind membershipImport) (ordinal 0))
      (authored-target "ControlPerformances::LoopPerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 19))))) (kind membershipImport) (ordinal 0))
      (authored-target "TransitionPerformances::TransitionPerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 20))))) (kind membershipImport) (ordinal 0))
      (authored-target "TransitionPerformances::NonStateTransitionPerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 21))))) (kind membershipImport) (ordinal 0))
      (authored-target "Transfers::MessageTransfer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 22))))) (kind membershipImport) (ordinal 0))
      (authored-target "Flows::MessageAction")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 23))))) (kind membershipImport) (ordinal 0))
      (authored-target "OccurrenceFunctions::destroy")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptAction"))) (kind specialization) (ordinal 0))
      (authored-target "AcceptMessageAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptMessageAction")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptAction")) (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "acceptedMessage")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptMessageAction::acceptedMessage")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptAction")) (anonymous (kind bind) (ordinal 0))))) (kind bindSource) (ordinal 0))
      (authored-target "payload")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptAction")) (anonymous (kind bind) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "aState::aTransition::apayload")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptAction::aState::aTransition"))) (kind transitionSource) (ordinal 0))
      (authored-target "start")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptAction::aState::aTransition"))) (kind transitionTarget) (ordinal 0))
      (authored-target "done")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptMessageAction"))) (kind specialization) (ordinal 0))
      (authored-target "Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptMessageAction"))) (kind specialization) (ordinal 1))
      (authored-target "AcceptPerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "payload")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptMessageAction::acceptedMessage"))) (kind featureTyping) (ordinal 0))
      (authored-target "MessageTransfer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptMessageAction::acceptedMessage"))) (kind featureTyping) (ordinal 1))
      (authored-target "MessageAction")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptMessageAction::acceptedMessage"))) (kind redefinition) (ordinal 0))
      (authored-target "acceptedTransfer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action"))) (kind specialization) (ordinal 0))
      (authored-target "Performance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::acceptSubactions"))) (kind featureTyping) (ordinal 0))
      (authored-target "AcceptAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptAction")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::acceptSubactions"))) (kind subsetting) (ordinal 0))
      (authored-target "subactions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::acceptSubactions"))) (kind subsetting) (ordinal 1))
      (authored-target "acceptActions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::acceptActions")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::assignments"))) (kind featureTyping) (ordinal 0))
      (authored-target "AssignmentAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AssignmentAction")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::assignments"))) (kind subsetting) (ordinal 0))
      (authored-target "subactions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::assignments"))) (kind subsetting) (ordinal 1))
      (authored-target "assignmentActions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::assignmentActions")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::controls"))) (kind featureTyping) (ordinal 0))
      (authored-target "ControlAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ControlAction")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::controls"))) (kind subsetting) (ordinal 0))
      (authored-target "subactions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::decisionTransitions"))) (kind featureTyping) (ordinal 0))
      (authored-target "DecisionTransitionAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::DecisionTransitionAction")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::decisionTransitions"))) (kind subsetting) (ordinal 0))
      (authored-target "transitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::transitions")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::decisions"))) (kind featureTyping) (ordinal 0))
      (authored-target "DecisionAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::DecisionAction")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::decisions"))) (kind subsetting) (ordinal 0))
      (authored-target "controls")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::controls")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::done"))) (kind featureTyping) (ordinal 0))
      (authored-target "Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::done"))) (kind redefinition) (ordinal 0))
      (authored-target "endShot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::forLoops"))) (kind featureTyping) (ordinal 0))
      (authored-target "ForLoopAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::forLoops"))) (kind subsetting) (ordinal 0))
      (authored-target "loops")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::loops")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::forLoops"))) (kind subsetting) (ordinal 1))
      (authored-target "forLoopActions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::forLoopActions")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::forks"))) (kind featureTyping) (ordinal 0))
      (authored-target "ForkAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForkAction")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::forks"))) (kind subsetting) (ordinal 0))
      (authored-target "controls")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::controls")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::ifSubactions"))) (kind featureTyping) (ordinal 0))
      (authored-target "IfThenAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenAction")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::ifSubactions"))) (kind subsetting) (ordinal 0))
      (authored-target "subactions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::ifSubactions"))) (kind subsetting) (ordinal 1))
      (authored-target "ifThenActions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ifThenActions")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::incomingTransfers"))) (kind redefinition) (ordinal 0))
      (authored-target "Performance::incomingTransfers")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::joins"))) (kind featureTyping) (ordinal 0))
      (authored-target "JoinAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::JoinAction")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::joins"))) (kind subsetting) (ordinal 0))
      (authored-target "controls")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::controls")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::loops"))) (kind featureTyping) (ordinal 0))
      (authored-target "LoopAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::loops"))) (kind subsetting) (ordinal 0))
      (authored-target "subactions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::loops"))) (kind subsetting) (ordinal 1))
      (authored-target "loopActions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::loopActions")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::merges"))) (kind featureTyping) (ordinal 0))
      (authored-target "MergeAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::MergeAction")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::merges"))) (kind subsetting) (ordinal 0))
      (authored-target "controls")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::controls")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::self"))) (kind featureTyping) (ordinal 0))
      (authored-target "Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::sendSubactions"))) (kind featureTyping) (ordinal 0))
      (authored-target "SendAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::SendAction")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::sendSubactions"))) (kind subsetting) (ordinal 0))
      (authored-target "subactions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::sendSubactions"))) (kind subsetting) (ordinal 1))
      (authored-target "sendActions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::sendActions")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::start"))) (kind featureTyping) (ordinal 0))
      (authored-target "Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::start"))) (kind redefinition) (ordinal 0))
      (authored-target "startShot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions"))) (kind featureTyping) (ordinal 0))
      (authored-target "Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions"))) (kind subsetting) (ordinal 0))
      (authored-target "actions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions"))) (kind subsetting) (ordinal 1))
      (authored-target "subperformances")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions::occurrence"))) (kind redefinition) (ordinal 0))
      (authored-target "Action::this")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::terminateSubactions"))) (kind featureTyping) (ordinal 0))
      (authored-target "TerminateAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TerminateAction")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::terminateSubactions"))) (kind subsetting) (ordinal 0))
      (authored-target "subactions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::terminateSubactions"))) (kind subsetting) (ordinal 1))
      (authored-target "terminateActions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::terminateActions")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::transitions"))) (kind featureTyping) (ordinal 0))
      (authored-target "TransitionAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::transitions"))) (kind subsetting) (ordinal 0))
      (authored-target "subactions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::transitions"))) (kind subsetting) (ordinal 1))
      (authored-target "transitionActions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::transitionActions")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::whileLoops"))) (kind featureTyping) (ordinal 0))
      (authored-target "WhileLoopAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::WhileLoopAction")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::whileLoops"))) (kind subsetting) (ordinal 0))
      (authored-target "loops")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::loops")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::whileLoops"))) (kind subsetting) (ordinal 1))
      (authored-target "whileLoopActions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::whileLoopActions")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AssignmentAction"))) (kind specialization) (ordinal 0))
      (authored-target "FeatureWritePerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AssignmentAction"))) (kind specialization) (ordinal 1))
      (authored-target "Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AssignmentAction::replacementValues"))) (kind featureTyping) (ordinal 0))
      (authored-target "Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AssignmentAction::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ControlAction"))) (kind specialization) (ordinal 0))
      (authored-target "Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "ControlAction")) (anonymous (kind bind) (ordinal 0))))) (kind bindSource) (ordinal 0))
      (authored-target "start")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "ControlAction")) (anonymous (kind bind) (ordinal 0))))) (kind bindTarget) (ordinal 0))
      (authored-target "done")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::DecisionAction"))) (kind specialization) (ordinal 0))
      (authored-target "ControlAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ControlAction")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::DecisionAction"))) (kind specialization) (ordinal 1))
      (authored-target "DecisionPerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::DecisionTransitionAction"))) (kind specialization) (ordinal 0))
      (authored-target "TransitionAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::DecisionTransitionAction"))) (kind specialization) (ordinal 1))
      (authored-target "NonStateTransitionPerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "DecisionTransitionAction")) (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "accepter")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::accepter")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "DecisionTransitionAction")) (anonymous (kind ref) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "effect")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::effect")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction"))) (kind specialization) (ordinal 0))
      (authored-target "LoopAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "ForLoopAction")) (anonymous (kind while) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "index")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "ForLoopAction")) (anonymous (kind while) (ordinal 0))))) (kind expressionOperand) (ordinal 1))
      (authored-target "seq")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction::seq")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "ForLoopAction")) (anonymous (kind while) (ordinal 0))))) (kind invocationCallee) (ordinal 0))
      (authored-target "size")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "ForLoopAction")) (anonymous (kind assign) (ordinal 0))))) (kind assignTarget) (ordinal 0))
      (authored-target "index")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "ForLoopAction")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "seq")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction::seq")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "ForLoopAction")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 1))))) (kind expressionOperand) (ordinal 0))
      (authored-target "index")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "ForLoopAction")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (kind expressionOperand) (ordinal 1))
      (authored-target "index")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "ForLoopAction")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (kind assignTarget) (ordinal 0))
      (authored-target "var")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction::var")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "ForLoopAction")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 1))))) (kind assignTarget) (ordinal 0))
      (authored-target "index")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction::var"))) (kind featureTyping) (ordinal 0))
      (authored-target "seq")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction::seq")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForkAction"))) (kind specialization) (ordinal 0))
      (authored-target "ControlAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ControlAction")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenAction"))) (kind specialization) (ordinal 0))
      (authored-target "Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenAction"))) (kind specialization) (ordinal 1))
      (authored-target "IfThenPerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenElseAction"))) (kind specialization) (ordinal 0))
      (authored-target "IfThenAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenAction")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenElseAction"))) (kind specialization) (ordinal 1))
      (authored-target "IfThenElsePerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::JoinAction"))) (kind specialization) (ordinal 0))
      (authored-target "ControlAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ControlAction")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction"))) (kind specialization) (ordinal 0))
      (authored-target "Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::MergeAction"))) (kind specialization) (ordinal 0))
      (authored-target "ControlAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ControlAction")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::MergeAction"))) (kind specialization) (ordinal 1))
      (authored-target "MergePerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::SendAction"))) (kind specialization) (ordinal 0))
      (authored-target "Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::SendAction"))) (kind specialization) (ordinal 1))
      (authored-target "SendPerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "SendAction")) (anonymous (kind parameter) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "payload")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::SendAction::sentMessage"))) (kind featureTyping) (ordinal 0))
      (authored-target "MessageTransfer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::SendAction::sentMessage"))) (kind featureTyping) (ordinal 1))
      (authored-target "MessageAction")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::SendAction::sentMessage"))) (kind redefinition) (ordinal 0))
      (authored-target "sentTransfer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TerminateAction"))) (kind specialization) (ordinal 0))
      (authored-target "Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TerminateAction::terminateOccurrence"))) (kind featureTyping) (ordinal 0))
      (authored-target "destroy")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TerminateAction::terminateOccurrence::occ"))) (kind expressionOperand) (ordinal 0))
      (authored-target "terminatedOccurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TerminateAction::terminatedOccurrence")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction"))) (kind specialization) (ordinal 0))
      (authored-target "Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction"))) (kind specialization) (ordinal 1))
      (authored-target "TransitionPerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "TransitionAction")) (anonymous (kind bind) (ordinal 0))))) (kind bindSource) (ordinal 0))
      (authored-target "receiver")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::receiver")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "TransitionAction")) (anonymous (kind bind) (ordinal 1))))) (kind bindSource) (ordinal 0))
      (authored-target "acceptedMessage")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::acceptedMessage")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "TransitionAction")) (anonymous (kind bind) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "accepter::receiver")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "TransitionAction")) (anonymous (kind bind) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "accepter::acceptedMessage")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptMessageAction::acceptedMessage")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::acceptedMessage"))) (kind featureTyping) (ordinal 0))
      (authored-target "MessageTransfer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::acceptedMessage"))) (kind featureTyping) (ordinal 1))
      (authored-target "MessageAction")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::accepter"))) (kind featureTyping) (ordinal 0))
      (authored-target "AcceptMessageAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptMessageAction")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::accepter"))) (kind redefinition) (ordinal 0))
      (authored-target "accept")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::effect"))) (kind featureTyping) (ordinal 0))
      (authored-target "Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::effect"))) (kind redefinition) (ordinal 0))
      (authored-target "TransitionPerformance::effect")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::receiver"))) (kind redefinition) (ordinal 0))
      (authored-target "triggerTarget")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::transitionLinkSource"))) (kind featureTyping) (ordinal 0))
      (authored-target "Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::transitionLinkSource"))) (kind redefinition) (ordinal 0))
      (authored-target "TransitionPerformance::transitionLinkSource")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::WhileLoopAction"))) (kind specialization) (ordinal 0))
      (authored-target "LoopAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::WhileLoopAction"))) (kind specialization) (ordinal 1))
      (authored-target "LoopPerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::acceptActions"))) (kind featureTyping) (ordinal 0))
      (authored-target "AcceptAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptAction")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::acceptActions"))) (kind subsetting) (ordinal 0))
      (authored-target "actions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::acceptActions"))) (kind subsetting) (ordinal 1))
      (authored-target "acceptPerformances")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions"))) (kind featureTyping) (ordinal 0))
      (authored-target "Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions"))) (kind subsetting) (ordinal 0))
      (authored-target "performances")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::assignmentActions"))) (kind featureTyping) (ordinal 0))
      (authored-target "AssignmentAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AssignmentAction")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::assignmentActions"))) (kind subsetting) (ordinal 0))
      (authored-target "actions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::assignmentActions::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::assignmentActions::target"))) (kind expressionOperand) (ordinal 0))
      (authored-target "that")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::assignmentActions::target"))) (kind typeCheckTarget) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::forLoopActions"))) (kind featureTyping) (ordinal 0))
      (authored-target "ForLoopAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::forLoopActions"))) (kind subsetting) (ordinal 0))
      (authored-target "loopActions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::loopActions")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ifThenActions"))) (kind featureTyping) (ordinal 0))
      (authored-target "IfThenAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenAction")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ifThenActions"))) (kind subsetting) (ordinal 0))
      (authored-target "actions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ifThenElseActions"))) (kind featureTyping) (ordinal 0))
      (authored-target "IfThenElseAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenElseAction")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ifThenElseActions"))) (kind subsetting) (ordinal 0))
      (authored-target "actions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::loopActions"))) (kind featureTyping) (ordinal 0))
      (authored-target "LoopAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::loopActions"))) (kind subsetting) (ordinal 0))
      (authored-target "actions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::sendActions"))) (kind featureTyping) (ordinal 0))
      (authored-target "SendAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::SendAction")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::sendActions"))) (kind subsetting) (ordinal 0))
      (authored-target "actions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::sendActions"))) (kind subsetting) (ordinal 1))
      (authored-target "sendPerformances")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::terminateActions"))) (kind featureTyping) (ordinal 0))
      (authored-target "TerminateAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TerminateAction")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::terminateActions"))) (kind subsetting) (ordinal 0))
      (authored-target "actions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::transitionActions"))) (kind featureTyping) (ordinal 0))
      (authored-target "TransitionAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::transitionActions"))) (kind subsetting) (ordinal 0))
      (authored-target "actions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::whileLoopActions"))) (kind featureTyping) (ordinal 0))
      (authored-target "WhileLoopAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::WhileLoopAction")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::whileLoopActions"))) (kind subsetting) (ordinal 0))
      (authored-target "loopActions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::loopActions")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptAction"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptMessageAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptAction"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptAction")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptMessageAction::acceptedMessage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptAction")) (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptMessageAction"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptMessageAction"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::acceptSubactions"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::acceptSubactions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::acceptSubactions"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::acceptSubactions"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::acceptSubactions"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::acceptActions"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::acceptSubactions"))) (kind subsetting) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::assignments"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AssignmentAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::assignments"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::assignments"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::assignments"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::assignments"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::assignmentActions"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::assignments"))) (kind subsetting) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::controls"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ControlAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::controls"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::controls"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::controls"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::decisionTransitions"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::DecisionTransitionAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::decisionTransitions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::decisionTransitions"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::transitions"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::decisionTransitions"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::decisions"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::DecisionAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::decisions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::decisions"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::controls"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::decisions"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::done"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::done"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::forLoops"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::forLoops"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::forLoops"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::loops"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::forLoops"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::forLoops"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::forLoopActions"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::forLoops"))) (kind subsetting) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::forks"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForkAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::forks"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::forks"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::controls"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::forks"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::ifSubactions"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::ifSubactions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::ifSubactions"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::ifSubactions"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::ifSubactions"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ifThenActions"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::ifSubactions"))) (kind subsetting) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::joins"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::JoinAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::joins"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::joins"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::controls"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::joins"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::loops"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::loops"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::loops"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::loops"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::loops"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::loopActions"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::loops"))) (kind subsetting) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::merges"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::MergeAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::merges"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::merges"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::controls"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::merges"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::self"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::self"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::sendSubactions"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::SendAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::sendSubactions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::sendSubactions"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::sendSubactions"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::sendSubactions"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::sendActions"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::sendSubactions"))) (kind subsetting) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::start"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::start"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::terminateSubactions"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TerminateAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::terminateSubactions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::terminateSubactions"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::terminateSubactions"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::terminateSubactions"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::terminateActions"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::terminateSubactions"))) (kind subsetting) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::transitions"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::transitions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::transitions"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::transitions"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::transitions"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::transitionActions"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::transitions"))) (kind subsetting) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::whileLoops"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::WhileLoopAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::whileLoops"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::whileLoops"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::loops"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::whileLoops"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::whileLoops"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::whileLoopActions"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::whileLoops"))) (kind subsetting) (ordinal 1)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AssignmentAction"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AssignmentAction"))) (kind specialization) (ordinal 1)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ControlAction"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ControlAction"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::DecisionAction"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ControlAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::DecisionAction"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::DecisionTransitionAction"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::DecisionTransitionAction"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "DecisionTransitionAction")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::accepter"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "DecisionTransitionAction")) (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "DecisionTransitionAction")) (anonymous (kind ref) (ordinal 1))))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::effect"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "DecisionTransitionAction")) (anonymous (kind ref) (ordinal 1))))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction"))) (kind specialization) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "ForLoopAction")) (anonymous (kind while) (ordinal 0))))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction::seq"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "ForLoopAction")) (anonymous (kind while) (ordinal 0))))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "ForLoopAction")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction::seq"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "ForLoopAction")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind assignTarget) (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "ForLoopAction")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction::var"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "ForLoopAction")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (kind assignTarget) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction::var"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction::seq"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction::var"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForkAction"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ControlAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForkAction"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenAction"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenAction"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenElseAction"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenElseAction"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::JoinAction"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ControlAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::JoinAction"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::MergeAction"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ControlAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::MergeAction"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::SendAction"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::SendAction"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TerminateAction"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TerminateAction"))) (kind specialization) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TerminateAction::terminateOccurrence::occ"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TerminateAction::terminatedOccurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TerminateAction::terminateOccurrence::occ"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction"))) (kind specialization) (ordinal 0)))
    (relationship (kind bindSource) (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "TransitionAction")) (anonymous (kind bind) (ordinal 0))))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::receiver"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "TransitionAction")) (anonymous (kind bind) (ordinal 0))))) (kind bindSource) (ordinal 0)))
    (relationship (kind bindSource) (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "TransitionAction")) (anonymous (kind bind) (ordinal 1))))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::acceptedMessage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "TransitionAction")) (anonymous (kind bind) (ordinal 1))))) (kind bindSource) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "TransitionAction")) (anonymous (kind bind) (ordinal 1))))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptMessageAction::acceptedMessage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "TransitionAction")) (anonymous (kind bind) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::accepter"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptMessageAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::accepter"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::effect"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::effect"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::transitionLinkSource"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::transitionLinkSource"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::WhileLoopAction"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::WhileLoopAction"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::acceptActions"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::acceptActions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::acceptActions"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::acceptActions"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::assignmentActions"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AssignmentAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::assignmentActions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::assignmentActions"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::assignmentActions"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::forLoopActions"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::forLoopActions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::forLoopActions"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::loopActions"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::forLoopActions"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ifThenActions"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ifThenActions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ifThenActions"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ifThenActions"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ifThenElseActions"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenElseAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ifThenElseActions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ifThenElseActions"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ifThenElseActions"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::loopActions"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::loopActions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::loopActions"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::loopActions"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::sendActions"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::SendAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::sendActions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::sendActions"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::sendActions"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::terminateActions"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TerminateAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::terminateActions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::terminateActions"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::terminateActions"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::transitionActions"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::transitionActions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::transitionActions"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::transitionActions"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::whileLoopActions"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::WhileLoopAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::whileLoopActions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::whileLoopActions"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::loopActions"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::whileLoopActions"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction::body"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction::body"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenElseAction::ifTest"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenAction::ifTest"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenElseAction::thenClause"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenAction::thenClause"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::WhileLoopAction::body"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction::body"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "ForLoopAction")) (anonymous (kind assign) (ordinal 0))))) (state literal) (value (kind integer) (integer 1)))
    (evaluated (declaration (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "ForLoopAction")) (anonymous (kind while) (ordinal 0))))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "ForLoopAction")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 1))))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TerminateAction::terminateOccurrence::occ"))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/actions.md") (qualified-name "Actions::assignmentActions::target"))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptAction")))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptMessageAction")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::acceptSubactions")) (scopes any))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::acceptActions")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptAction")) (anonymous (kind ref) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptMessageAction::acceptedMessage")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptMessageAction")))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptAction")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::accepter")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptMessageAction::acceptedMessage")))
      (subtype (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptAction")) (anonymous (kind ref) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptMessageAction")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::done")) (scopes any))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::self")) (scopes any))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::start")) (scopes any))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AssignmentAction")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ControlAction")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenAction")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::SendAction")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TerminateAction")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::effect")) (scopes any))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::transitionLinkSource")) (scopes any))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::acceptSubactions")))
      (type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptAction")) (provenance authored))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptAction")) (source direct))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptAction")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::acceptActions"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions"))))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptAction")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptMessageAction")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::acceptActions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::assignments")))
      (type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AssignmentAction")) (provenance authored))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AssignmentAction")) (source direct))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AssignmentAction")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::assignmentActions"))))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AssignmentAction")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::assignmentActions")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::controls")))
      (type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ControlAction")) (provenance authored))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ControlAction")) (source direct))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ControlAction")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::decisions")) (scopes any feature))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::forks")) (scopes any feature))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::joins")) (scopes any feature))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::merges")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::decisionTransitions")))
      (type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::DecisionTransitionAction")) (provenance authored))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::DecisionTransitionAction")) (source direct))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::transitions"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::transitionActions"))))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::transitions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::DecisionTransitionAction")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::transitionActions")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::decisions")))
      (type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::DecisionAction")) (provenance authored))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ControlAction")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::controls"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::DecisionAction")) (source direct))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::controls")) (scopes any feature))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ControlAction")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::DecisionAction")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::done")))
      (type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (provenance authored))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source direct))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::forLoops")))
      (type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction")) (provenance authored))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction")) (source direct))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::forLoopActions"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::loops"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::loopActions"))))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::loops")) (scopes any feature))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::forLoopActions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::loopActions")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::forks")))
      (type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForkAction")) (provenance authored))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ControlAction")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::controls"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForkAction")) (source direct))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::controls")) (scopes any feature))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ControlAction")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForkAction")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::ifSubactions")))
      (type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenAction")) (provenance authored))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenAction")) (source direct))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenAction")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ifThenActions"))))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenAction")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ifThenActions")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::joins")))
      (type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::JoinAction")) (provenance authored))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ControlAction")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::controls"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::JoinAction")) (source direct))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::controls")) (scopes any feature))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ControlAction")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::JoinAction")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::loops")))
      (type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction")) (provenance authored))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction")) (source direct))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::loopActions"))))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::loopActions")) (scopes any feature))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::forLoops")) (scopes any feature))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::whileLoops")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::merges")))
      (type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::MergeAction")) (provenance authored))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ControlAction")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::controls"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::MergeAction")) (source direct))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::controls")) (scopes any feature))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ControlAction")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::MergeAction")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::self")))
      (type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (provenance authored))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source direct))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::sendSubactions")))
      (type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::SendAction")) (provenance authored))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::SendAction")) (source direct))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::SendAction")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::sendActions"))))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::SendAction")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::sendActions")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::start")))
      (type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (provenance authored))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source direct))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions")))
      (type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (provenance authored))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source direct))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions"))))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::acceptSubactions")) (scopes any feature))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::assignments")) (scopes any feature))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::controls")) (scopes any feature))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::ifSubactions")) (scopes any feature))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::loops")) (scopes any feature))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::sendSubactions")) (scopes any feature))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::terminateSubactions")) (scopes any feature))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::transitions")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::terminateSubactions")))
      (type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TerminateAction")) (provenance authored))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TerminateAction")) (source direct))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TerminateAction")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::terminateActions"))))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TerminateAction")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::terminateActions")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::transitions")))
      (type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction")) (provenance authored))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction")) (source direct))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::transitionActions"))))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::transitionActions")) (scopes any feature))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::decisionTransitions")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::whileLoops")))
      (type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::WhileLoopAction")) (provenance authored))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::loops"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::loopActions"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::WhileLoopAction")) (source direct))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::WhileLoopAction")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::whileLoopActions"))))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::loops")) (scopes any feature))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::WhileLoopAction")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::loopActions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::whileLoopActions")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AssignmentAction")))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::assignments")) (scopes any))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::assignmentActions")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ControlAction")))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::controls")) (scopes any))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::DecisionAction")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForkAction")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::JoinAction")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::MergeAction")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::DecisionAction")))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ControlAction")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::decisions")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::DecisionTransitionAction")))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::decisionTransitions")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "DecisionTransitionAction")) (anonymous (kind ref) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptMessageAction")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::accepter"))))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptMessageAction")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::accepter")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "DecisionTransitionAction")) (anonymous (kind ref) (ordinal 1)))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::effect"))))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::effect")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction")))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::forLoops")) (scopes any))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::forLoopActions")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction::body")))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction::body")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction::seq")))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction::var")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction::var")))
      (type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction::seq")) (provenance authored))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction::seq")) (source direct))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction::seq")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForkAction")))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ControlAction")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::forks")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenAction")))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::ifSubactions")) (scopes any))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenElseAction")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ifThenActions")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenAction::ifTest")))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenElseAction::ifTest")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenAction::thenClause")))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenElseAction::thenClause")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenElseAction")))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenAction")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ifThenElseActions")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenElseAction::ifTest")))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenAction::ifTest")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenElseAction::thenClause")))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenAction::thenClause")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::JoinAction")))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ControlAction")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::joins")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction")))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::loops")) (scopes any))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::WhileLoopAction")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::loopActions")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction::body")))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction::body")) (scopes any feature))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::WhileLoopAction::body")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::MergeAction")))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ControlAction")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::merges")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::SendAction")))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::sendSubactions")) (scopes any))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::sendActions")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TerminateAction")))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::terminateSubactions")) (scopes any))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::terminateActions")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction")))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::transitions")) (scopes any))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::DecisionTransitionAction")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::transitionActions")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::accepter")))
      (type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptMessageAction")) (provenance authored))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptMessageAction")) (source direct))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptMessageAction")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (subtype (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "DecisionTransitionAction")) (anonymous (kind ref) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::effect")))
      (type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (provenance authored))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source direct))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (subtype (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "DecisionTransitionAction")) (anonymous (kind ref) (ordinal 1)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::transitionLinkSource")))
      (type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (provenance authored))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source direct))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::WhileLoopAction")))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::whileLoops")) (scopes any))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::whileLoopActions")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::WhileLoopAction::body")))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction::body")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::acceptActions")))
      (type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptAction")) (provenance authored))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptAction")) (source direct))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions"))))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptAction")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptMessageAction")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::acceptSubactions")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")))
      (type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (provenance authored))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source direct))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::acceptActions")) (scopes any feature))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::assignmentActions")) (scopes any feature))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ifThenActions")) (scopes any feature))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ifThenElseActions")) (scopes any feature))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::loopActions")) (scopes any feature))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::sendActions")) (scopes any feature))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::terminateActions")) (scopes any feature))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::transitionActions")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::assignmentActions")))
      (type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AssignmentAction")) (provenance authored))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AssignmentAction")) (source direct))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AssignmentAction")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::assignments")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::forLoopActions")))
      (type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction")) (provenance authored))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction")) (source direct))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::loopActions"))))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::loopActions")) (scopes any feature))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::forLoops")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ifThenActions")))
      (type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenAction")) (provenance authored))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenAction")) (source direct))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenAction")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::ifSubactions")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ifThenElseActions")))
      (type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenElseAction")) (provenance authored))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenElseAction")) (source direct))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenAction")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenElseAction")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::loopActions")))
      (type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction")) (provenance authored))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction")) (source direct))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::loops")) (scopes any feature))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::forLoopActions")) (scopes any feature))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::whileLoopActions")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::sendActions")))
      (type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::SendAction")) (provenance authored))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::SendAction")) (source direct))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::SendAction")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::sendSubactions")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::terminateActions")))
      (type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TerminateAction")) (provenance authored))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TerminateAction")) (source direct))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TerminateAction")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::terminateSubactions")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::transitionActions")))
      (type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction")) (provenance authored))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction")) (source direct))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::transitions")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::whileLoopActions")))
      (type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::WhileLoopAction")) (provenance authored))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction")) (source inherited) (from (node (document "memory://snapshot/actions.md") (qualified-name "Actions::loopActions"))))
      (effective-type (node (document "memory://snapshot/actions.md") (qualified-name "Actions::WhileLoopAction")) (source direct))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::WhileLoopAction")) (scopes any))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::loopActions")) (scopes any feature))
      (subtype (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::whileLoops")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/actions.md") (range (start 7 16) (end 7 30)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Base::Anything")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 8 16) (end 8 38)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Positive")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 9 16) (end 9 37)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Natural")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 10 16) (end 10 39)) (probe (position 10 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::size")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 11 16) (end 11 42)) (probe (position 11 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::isEmpty")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 12 16) (end 12 39)) (probe (position 12 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0) (authored-target "Occurrences::Occurrence")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 13 16) (end 13 41)) (probe (position 13 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0) (authored-target "Occurrences::HappensWhile")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 14 16) (end 14 41)) (probe (position 14 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0) (authored-target "Performances::Performance")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 15 16) (end 15 42)) (probe (position 15 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 8))))) (kind membershipImport) (ordinal 0) (authored-target "Performances::performances")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 16 16) (end 16 42)) (probe (position 16 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 9))))) (kind membershipImport) (ordinal 0) (authored-target "Transfers::SendPerformance")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 17 16) (end 17 43)) (probe (position 17 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 10))))) (kind membershipImport) (ordinal 0) (authored-target "Transfers::sendPerformances")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 18 16) (end 18 44)) (probe (position 18 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 11))))) (kind membershipImport) (ordinal 0) (authored-target "Transfers::AcceptPerformance")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 19 16) (end 19 45)) (probe (position 19 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 12))))) (kind membershipImport) (ordinal 0) (authored-target "Transfers::acceptPerformances")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 20 16) (end 20 71)) (probe (position 20 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 13))))) (kind membershipImport) (ordinal 0) (authored-target "FeatureReferencingPerformances::FeatureWritePerformance")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 21 16) (end 21 53)) (probe (position 21 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 14))))) (kind membershipImport) (ordinal 0) (authored-target "ControlPerformances::MergePerformance")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 22 16) (end 22 56)) (probe (position 22 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 15))))) (kind membershipImport) (ordinal 0) (authored-target "ControlPerformances::DecisionPerformance")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 23 16) (end 23 54)) (probe (position 23 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 16))))) (kind membershipImport) (ordinal 0) (authored-target "ControlPerformances::IfThenPerformance")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 24 16) (end 24 58)) (probe (position 24 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 17))))) (kind membershipImport) (ordinal 0) (authored-target "ControlPerformances::IfThenElsePerformance")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 25 16) (end 25 52)) (probe (position 25 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 18))))) (kind membershipImport) (ordinal 0) (authored-target "ControlPerformances::LoopPerformance")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 26 16) (end 26 61)) (probe (position 26 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 19))))) (kind membershipImport) (ordinal 0) (authored-target "TransitionPerformances::TransitionPerformance")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 27 16) (end 27 69)) (probe (position 27 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 20))))) (kind membershipImport) (ordinal 0) (authored-target "TransitionPerformances::NonStateTransitionPerformance")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 28 16) (end 28 42)) (probe (position 28 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 21))))) (kind membershipImport) (ordinal 0) (authored-target "Transfers::MessageTransfer")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 29 16) (end 29 36)) (probe (position 29 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 22))))) (kind membershipImport) (ordinal 0) (authored-target "Flows::MessageAction")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 30 16) (end 30 44)) (probe (position 30 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (anonymous (kind import) (ordinal 23))))) (kind membershipImport) (ordinal 0) (authored-target "OccurrenceFunctions::destroy")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 218 28) (end 218 47)) (probe (position 218 28))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptAction"))) (kind specialization) (ordinal 0) (authored-target "AcceptMessageAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptMessageAction")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 225 10) (end 225 25)) (probe (position 225 10))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptAction")) (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "acceptedMessage")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptMessageAction::acceptedMessage")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 229 7) (end 229 14)) (probe (position 229 7))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptAction")) (anonymous (kind bind) (ordinal 0))))) (kind bindSource) (ordinal 0) (authored-target "payload")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 229 17) (end 229 44)) (probe (position 229 17))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptAction")) (anonymous (kind bind) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "aState::aTransition::apayload")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 227 32) (end 227 37)) (probe (position 227 32))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptAction::aState::aTransition"))) (kind transitionSource) (ordinal 0) (authored-target "start")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 227 82) (end 227 86)) (probe (position 227 82))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptAction::aState::aTransition"))) (kind transitionTarget) (ordinal 0) (authored-target "done")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 206 35) (end 206 41)) (probe (position 206 35))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptMessageAction"))) (kind specialization) (ordinal 0) (authored-target "Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 206 43) (end 206 60)) (probe (position 206 43))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptMessageAction"))) (kind specialization) (ordinal 1) (authored-target "AcceptPerformance")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 212 12) (end 212 19)) (probe (position 212 12))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "payload")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 213 44) (end 213 59)) (probe (position 213 44))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptMessageAction::acceptedMessage"))) (kind featureTyping) (ordinal 0) (authored-target "MessageTransfer")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 213 61) (end 213 74)) (probe (position 213 61))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptMessageAction::acceptedMessage"))) (kind featureTyping) (ordinal 1) (authored-target "MessageAction")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 213 26) (end 213 42)) (probe (position 213 26))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptMessageAction::acceptedMessage"))) (kind redefinition) (ordinal 0) (authored-target "acceptedTransfer")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 32 31) (end 32 42)) (probe (position 32 31))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action"))) (kind specialization) (ordinal 0) (authored-target "Performance")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 78 27) (end 78 39)) (probe (position 78 27))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::acceptSubactions"))) (kind featureTyping) (ordinal 0) (authored-target "AcceptAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptAction")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 78 49) (end 78 59)) (probe (position 78 49))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::acceptSubactions"))) (kind subsetting) (ordinal 0) (authored-target "subactions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 78 61) (end 78 74)) (probe (position 78 61))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::acceptSubactions"))) (kind subsetting) (ordinal 1) (authored-target "acceptActions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::acceptActions")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 141 32) (end 141 48)) (probe (position 141 32))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::assignments"))) (kind featureTyping) (ordinal 0) (authored-target "AssignmentAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AssignmentAction")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 141 58) (end 141 68)) (probe (position 141 58))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::assignments"))) (kind subsetting) (ordinal 0) (authored-target "subactions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 141 70) (end 141 87)) (probe (position 141 70))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::assignments"))) (kind subsetting) (ordinal 1) (authored-target "assignmentActions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::assignmentActions")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 92 29) (end 92 42)) (probe (position 92 29))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::controls"))) (kind featureTyping) (ordinal 0) (authored-target "ControlAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ControlAction")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 92 52) (end 92 62)) (probe (position 92 52))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::controls"))) (kind subsetting) (ordinal 0) (authored-target "subactions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 134 40) (end 134 64)) (probe (position 134 40))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::decisionTransitions"))) (kind featureTyping) (ordinal 0) (authored-target "DecisionTransitionAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::DecisionTransitionAction")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 134 74) (end 134 85)) (probe (position 134 74))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::decisionTransitions"))) (kind subsetting) (ordinal 0) (authored-target "transitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::transitions")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 106 30) (end 106 44)) (probe (position 106 30))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::decisions"))) (kind featureTyping) (ordinal 0) (authored-target "DecisionAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::DecisionAction")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 106 48) (end 106 56)) (probe (position 106 48))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::decisions"))) (kind subsetting) (ordinal 0) (authored-target "controls")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::controls")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 49 15) (end 49 21)) (probe (position 49 15))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::done"))) (kind featureTyping) (ordinal 0) (authored-target "Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 49 26) (end 49 33)) (probe (position 49 26))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::done"))) (kind redefinition) (ordinal 0) (authored-target "endShot")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 171 29) (end 171 42)) (probe (position 171 29))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::forLoops"))) (kind featureTyping) (ordinal 0) (authored-target "ForLoopAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 171 52) (end 171 57)) (probe (position 171 52))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::forLoops"))) (kind subsetting) (ordinal 0) (authored-target "loops")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::loops")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 171 59) (end 171 73)) (probe (position 171 59))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::forLoops"))) (kind subsetting) (ordinal 1) (authored-target "forLoopActions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::forLoopActions")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 120 26) (end 120 36)) (probe (position 120 26))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::forks"))) (kind featureTyping) (ordinal 0) (authored-target "ForkAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForkAction")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 120 40) (end 120 48)) (probe (position 120 40))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::forks"))) (kind subsetting) (ordinal 0) (authored-target "controls")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::controls")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 150 33) (end 150 45)) (probe (position 150 33))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::ifSubactions"))) (kind featureTyping) (ordinal 0) (authored-target "IfThenAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenAction")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 150 55) (end 150 65)) (probe (position 150 55))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::ifSubactions"))) (kind subsetting) (ordinal 0) (authored-target "subactions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 150 67) (end 150 80)) (probe (position 150 67))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::ifSubactions"))) (kind subsetting) (ordinal 1) (authored-target "ifThenActions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ifThenActions")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 40 35) (end 40 65)) (probe (position 40 35))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::incomingTransfers"))) (kind redefinition) (ordinal 0) (authored-target "Performance::incomingTransfers")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 113 26) (end 113 36)) (probe (position 113 26))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::joins"))) (kind featureTyping) (ordinal 0) (authored-target "JoinAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::JoinAction")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 113 40) (end 113 48)) (probe (position 113 40))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::joins"))) (kind subsetting) (ordinal 0) (authored-target "controls")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::controls")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 157 26) (end 157 36)) (probe (position 157 26))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::loops"))) (kind featureTyping) (ordinal 0) (authored-target "LoopAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 157 46) (end 157 56)) (probe (position 157 46))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::loops"))) (kind subsetting) (ordinal 0) (authored-target "subactions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 157 58) (end 157 69)) (probe (position 157 58))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::loops"))) (kind subsetting) (ordinal 1) (authored-target "loopActions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::loopActions")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 99 27) (end 99 38)) (probe (position 99 27))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::merges"))) (kind featureTyping) (ordinal 0) (authored-target "MergeAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::MergeAction")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 99 48) (end 99 56)) (probe (position 99 48))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::merges"))) (kind subsetting) (ordinal 0) (authored-target "controls")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::controls")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 39 19) (end 39 25)) (probe (position 39 19))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::self"))) (kind featureTyping) (ordinal 0) (authored-target "Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 71 25) (end 71 35)) (probe (position 71 25))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::sendSubactions"))) (kind featureTyping) (ordinal 0) (authored-target "SendAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::SendAction")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 71 45) (end 71 55)) (probe (position 71 45))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::sendSubactions"))) (kind subsetting) (ordinal 0) (authored-target "subactions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 71 57) (end 71 68)) (probe (position 71 57))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::sendSubactions"))) (kind subsetting) (ordinal 1) (authored-target "sendActions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::sendActions")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 42 16) (end 42 22)) (probe (position 42 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::start"))) (kind featureTyping) (ordinal 0) (authored-target "Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 42 27) (end 42 36)) (probe (position 42 27))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::start"))) (kind redefinition) (ordinal 0) (authored-target "startShot")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 56 21) (end 56 27)) (probe (position 56 21))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions"))) (kind featureTyping) (ordinal 0) (authored-target "Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 56 37) (end 56 44)) (probe (position 56 37))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions"))) (kind subsetting) (ordinal 0) (authored-target "actions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 56 46) (end 56 61)) (probe (position 56 46))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions"))) (kind subsetting) (ordinal 1) (authored-target "subperformances")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 62 22) (end 62 34)) (probe (position 62 22))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions::occurrence"))) (kind redefinition) (ordinal 0) (authored-target "Action::this")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 85 40) (end 85 55)) (probe (position 85 40))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::terminateSubactions"))) (kind featureTyping) (ordinal 0) (authored-target "TerminateAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TerminateAction")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 85 65) (end 85 75)) (probe (position 85 65))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::terminateSubactions"))) (kind subsetting) (ordinal 0) (authored-target "subactions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 85 77) (end 85 93)) (probe (position 85 77))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::terminateSubactions"))) (kind subsetting) (ordinal 1) (authored-target "terminateActions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::terminateActions")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 127 32) (end 127 48)) (probe (position 127 32))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::transitions"))) (kind featureTyping) (ordinal 0) (authored-target "TransitionAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 127 58) (end 127 68)) (probe (position 127 58))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::transitions"))) (kind subsetting) (ordinal 0) (authored-target "subactions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 127 70) (end 127 87)) (probe (position 127 70))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::transitions"))) (kind subsetting) (ordinal 1) (authored-target "transitionActions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::transitionActions")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 164 31) (end 164 46)) (probe (position 164 31))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::whileLoops"))) (kind featureTyping) (ordinal 0) (authored-target "WhileLoopAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::WhileLoopAction")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 164 56) (end 164 61)) (probe (position 164 56))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::whileLoops"))) (kind subsetting) (ordinal 0) (authored-target "loops")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::loops")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 164 63) (end 164 79)) (probe (position 164 63))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::whileLoops"))) (kind subsetting) (ordinal 1) (authored-target "whileLoopActions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::whileLoopActions")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 371 32) (end 371 55)) (probe (position 371 32))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AssignmentAction"))) (kind specialization) (ordinal 0) (authored-target "FeatureWritePerformance")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 371 57) (end 371 63)) (probe (position 371 57))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AssignmentAction"))) (kind specialization) (ordinal 1) (authored-target "Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 380 28) (end 380 36)) (probe (position 380 28))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AssignmentAction::replacementValues"))) (kind featureTyping) (ordinal 0) (authored-target "Anything")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 379 14) (end 379 24)) (probe (position 379 14))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AssignmentAction::target"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 274 38) (end 274 44)) (probe (position 274 38))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ControlAction"))) (kind specialization) (ordinal 0) (authored-target "Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 280 7) (end 280 12)) (probe (position 280 7))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "ControlAction")) (anonymous (kind bind) (ordinal 0))))) (kind bindSource) (ordinal 0) (authored-target "start")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 280 15) (end 280 19)) (probe (position 280 15))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "ControlAction")) (anonymous (kind bind) (ordinal 0))))) (kind bindTarget) (ordinal 0) (authored-target "done")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 298 30) (end 298 43)) (probe (position 298 30))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::DecisionAction"))) (kind specialization) (ordinal 0) (authored-target "ControlAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ControlAction")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 298 45) (end 298 64)) (probe (position 298 45))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::DecisionAction"))) (kind specialization) (ordinal 1) (authored-target "DecisionPerformance")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 352 40) (end 352 56)) (probe (position 352 40))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::DecisionTransitionAction"))) (kind specialization) (ordinal 0) (authored-target "TransitionAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 352 58) (end 352 87)) (probe (position 352 58))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::DecisionTransitionAction"))) (kind specialization) (ordinal 1) (authored-target "NonStateTransitionPerformance")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 360 17) (end 360 25)) (probe (position 360 17))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "DecisionTransitionAction")) (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "accepter")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::accepter")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 361 17) (end 361 23)) (probe (position 361 17))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "DecisionTransitionAction")) (anonymous (kind ref) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "effect")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::effect")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 484 29) (end 484 39)) (probe (position 484 29))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction"))) (kind specialization) (ordinal 0) (authored-target "LoopAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 524 9) (end 524 14)) (probe (position 524 9))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "ForLoopAction")) (anonymous (kind while) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "index")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 524 23) (end 524 26)) (probe (position 524 23))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "ForLoopAction")) (anonymous (kind while) (ordinal 0))))) (kind expressionOperand) (ordinal 1) (authored-target "seq")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction::seq")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 524 18) (end 524 22)) (probe (position 524 18))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "ForLoopAction")) (anonymous (kind while) (ordinal 0))))) (kind invocationCallee) (ordinal 0) (authored-target "size")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 522 10) (end 522 15)) (probe (position 522 10))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "ForLoopAction")) (anonymous (kind assign) (ordinal 0))))) (kind assignTarget) (ordinal 0) (authored-target "index")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 525 18) (end 525 21)) (probe (position 525 18))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "ForLoopAction")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "seq")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction::seq")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 527 25) (end 527 30)) (probe (position 527 25))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "ForLoopAction")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 1))))) (kind expressionOperand) (ordinal 0) (authored-target "index")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 525 23) (end 525 28)) (probe (position 525 23))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "ForLoopAction")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (kind expressionOperand) (ordinal 1) (authored-target "index")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 525 11) (end 525 14)) (probe (position 525 11))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "ForLoopAction")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (kind assignTarget) (ordinal 0) (authored-target "var")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction::var")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 527 16) (end 527 21)) (probe (position 527 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "ForLoopAction")) (anonymous (kind while) (ordinal 0)) (anonymous (kind assign) (ordinal 1))))) (kind assignTarget) (ordinal 0) (authored-target "index")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 491 29) (end 491 32)) (probe (position 491 29))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction::var"))) (kind featureTyping) (ordinal 0) (authored-target "seq")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction::seq")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 320 26) (end 320 39)) (probe (position 320 26))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForkAction"))) (kind specialization) (ordinal 0) (authored-target "ControlAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ControlAction")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 398 28) (end 398 34)) (probe (position 398 28))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenAction"))) (kind specialization) (ordinal 0) (authored-target "Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 398 36) (end 398 53)) (probe (position 398 36))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenAction"))) (kind specialization) (ordinal 1) (authored-target "IfThenPerformance")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 409 32) (end 409 44)) (probe (position 409 32))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenElseAction"))) (kind specialization) (ordinal 0) (authored-target "IfThenAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenAction")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 409 46) (end 409 67)) (probe (position 409 46))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenElseAction"))) (kind specialization) (ordinal 1) (authored-target "IfThenElsePerformance")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 310 26) (end 310 39)) (probe (position 310 26))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::JoinAction"))) (kind specialization) (ordinal 0) (authored-target "ControlAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ControlAction")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 435 35) (end 435 41)) (probe (position 435 35))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction"))) (kind specialization) (ordinal 0) (authored-target "Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 288 27) (end 288 40)) (probe (position 288 27))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::MergeAction"))) (kind specialization) (ordinal 0) (authored-target "ControlAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ControlAction")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 288 42) (end 288 58)) (probe (position 288 42))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::MergeAction"))) (kind specialization) (ordinal 1) (authored-target "MergePerformance")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 186 26) (end 186 32)) (probe (position 186 26))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::SendAction"))) (kind specialization) (ordinal 0) (authored-target "Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 186 34) (end 186 49)) (probe (position 186 34))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::SendAction"))) (kind specialization) (ordinal 1) (authored-target "SendPerformance")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 193 9) (end 193 16)) (probe (position 193 9))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "SendAction")) (anonymous (kind parameter) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "payload")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 194 39) (end 194 54)) (probe (position 194 39))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::SendAction::sentMessage"))) (kind featureTyping) (ordinal 0) (authored-target "MessageTransfer")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 194 56) (end 194 69)) (probe (position 194 56))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::SendAction::sentMessage"))) (kind featureTyping) (ordinal 1) (authored-target "MessageAction")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 194 25) (end 194 37)) (probe (position 194 25))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::SendAction::sentMessage"))) (kind redefinition) (ordinal 0) (authored-target "sentTransfer")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 239 40) (end 239 46)) (probe (position 239 40))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TerminateAction"))) (kind specialization) (ordinal 0) (authored-target "Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 254 31) (end 254 38)) (probe (position 254 31))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TerminateAction::terminateOccurrence"))) (kind featureTyping) (ordinal 0) (authored-target "destroy")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 255 12) (end 255 32)) (probe (position 255 12))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TerminateAction::terminateOccurrence::occ"))) (kind expressionOperand) (ordinal 0) (authored-target "terminatedOccurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TerminateAction::terminatedOccurrence")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 330 41) (end 330 47)) (probe (position 330 41))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction"))) (kind specialization) (ordinal 0) (authored-target "Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 330 49) (end 330 70)) (probe (position 330 49))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction"))) (kind specialization) (ordinal 1) (authored-target "TransitionPerformance")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 346 7) (end 346 15)) (probe (position 346 7))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "TransitionAction")) (anonymous (kind bind) (ordinal 0))))) (kind bindSource) (ordinal 0) (authored-target "receiver")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::receiver")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 347 7) (end 347 22)) (probe (position 347 7))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "TransitionAction")) (anonymous (kind bind) (ordinal 1))))) (kind bindSource) (ordinal 0) (authored-target "acceptedMessage")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::acceptedMessage")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 346 18) (end 346 35)) (probe (position 346 18))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "TransitionAction")) (anonymous (kind bind) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "accepter::receiver")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 347 25) (end 347 49)) (probe (position 347 25))
    (reference (id (source (node (document "memory://snapshot/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "TransitionAction")) (anonymous (kind bind) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0) (authored-target "accepter::acceptedMessage")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptMessageAction::acceptedMessage")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 338 24) (end 338 39)) (probe (position 338 24))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::acceptedMessage"))) (kind featureTyping) (ordinal 0) (authored-target "MessageTransfer")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 338 41) (end 338 54)) (probe (position 338 41))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::acceptedMessage"))) (kind featureTyping) (ordinal 1) (authored-target "MessageAction")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 344 20) (end 344 39)) (probe (position 344 20))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::accepter"))) (kind featureTyping) (ordinal 0) (authored-target "AcceptMessageAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptMessageAction")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 344 44) (end 344 52)) (probe (position 344 44))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::accepter"))) (kind redefinition) (ordinal 0) (authored-target "accept")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 349 17) (end 349 23)) (probe (position 349 17))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::effect"))) (kind featureTyping) (ordinal 0) (authored-target "Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 349 28) (end 349 57)) (probe (position 349 28))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::effect"))) (kind redefinition) (ordinal 0) (authored-target "TransitionPerformance::effect")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 342 19) (end 342 32)) (probe (position 342 19))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::receiver"))) (kind redefinition) (ordinal 0) (authored-target "triggerTarget")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 337 28) (end 337 34)) (probe (position 337 28))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::transitionLinkSource"))) (kind featureTyping) (ordinal 0) (authored-target "Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 337 39) (end 337 82)) (probe (position 337 39))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::transitionLinkSource"))) (kind redefinition) (ordinal 0) (authored-target "TransitionPerformance::transitionLinkSource")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 451 31) (end 451 41)) (probe (position 451 31))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::WhileLoopAction"))) (kind specialization) (ordinal 0) (authored-target "LoopAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 451 43) (end 451 58)) (probe (position 451 43))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::WhileLoopAction"))) (kind specialization) (ordinal 1) (authored-target "LoopPerformance")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 232 32) (end 232 44)) (probe (position 232 32))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::acceptActions"))) (kind featureTyping) (ordinal 0) (authored-target "AcceptAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptAction")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 232 64) (end 232 71)) (probe (position 232 64))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::acceptActions"))) (kind subsetting) (ordinal 0) (authored-target "actions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 232 73) (end 232 91)) (probe (position 232 73))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::acceptActions"))) (kind subsetting) (ordinal 1) (authored-target "acceptPerformances")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 179 26) (end 179 32)) (probe (position 179 26))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions"))) (kind featureTyping) (ordinal 0) (authored-target "Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 179 52) (end 179 64)) (probe (position 179 52))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions"))) (kind subsetting) (ordinal 0) (authored-target "performances")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 383 37) (end 383 53)) (probe (position 383 37))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::assignmentActions"))) (kind featureTyping) (ordinal 0) (authored-target "AssignmentAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AssignmentAction")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 383 73) (end 383 80)) (probe (position 383 73))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::assignmentActions"))) (kind subsetting) (ordinal 0) (authored-target "actions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 389 20) (end 389 30)) (probe (position 389 20))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::assignmentActions::target"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 389 42) (end 389 46)) (probe (position 389 42))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::assignmentActions::target"))) (kind expressionOperand) (ordinal 0) (authored-target "that")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 389 50) (end 389 60)) (probe (position 389 50))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::assignmentActions::target"))) (kind typeCheckTarget) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 545 34) (end 545 47)) (probe (position 545 34))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::forLoopActions"))) (kind featureTyping) (ordinal 0) (authored-target "ForLoopAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 545 67) (end 545 78)) (probe (position 545 67))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::forLoopActions"))) (kind subsetting) (ordinal 0) (authored-target "loopActions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::loopActions")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 421 33) (end 421 45)) (probe (position 421 33))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ifThenActions"))) (kind featureTyping) (ordinal 0) (authored-target "IfThenAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenAction")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 421 65) (end 421 72)) (probe (position 421 65))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ifThenActions"))) (kind subsetting) (ordinal 0) (authored-target "actions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 428 37) (end 428 53)) (probe (position 428 37))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ifThenElseActions"))) (kind featureTyping) (ordinal 0) (authored-target "IfThenElseAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenElseAction")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 428 73) (end 428 80)) (probe (position 428 73))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ifThenElseActions"))) (kind subsetting) (ordinal 0) (authored-target "actions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 531 31) (end 531 41)) (probe (position 531 31))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::loopActions"))) (kind featureTyping) (ordinal 0) (authored-target "LoopAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 531 61) (end 531 68)) (probe (position 531 61))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::loopActions"))) (kind subsetting) (ordinal 0) (authored-target "actions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 199 30) (end 199 40)) (probe (position 199 30))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::sendActions"))) (kind featureTyping) (ordinal 0) (authored-target "SendAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::SendAction")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 199 60) (end 199 67)) (probe (position 199 60))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::sendActions"))) (kind subsetting) (ordinal 0) (authored-target "actions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 199 69) (end 199 85)) (probe (position 199 69))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::sendActions"))) (kind subsetting) (ordinal 1) (authored-target "sendPerformances")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 259 36) (end 259 51)) (probe (position 259 36))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::terminateActions"))) (kind featureTyping) (ordinal 0) (authored-target "TerminateAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TerminateAction")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 259 71) (end 259 78)) (probe (position 259 71))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::terminateActions"))) (kind subsetting) (ordinal 0) (authored-target "actions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 364 36) (end 364 52)) (probe (position 364 36))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::transitionActions"))) (kind featureTyping) (ordinal 0) (authored-target "TransitionAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 364 72) (end 364 79)) (probe (position 364 72))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::transitionActions"))) (kind subsetting) (ordinal 0) (authored-target "actions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 538 36) (end 538 51)) (probe (position 538 36))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::whileLoopActions"))) (kind featureTyping) (ordinal 0) (authored-target "WhileLoopAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::WhileLoopAction")))))
    )
  )
  (query (document "memory://snapshot/actions.md") (range (start 538 71) (end 538 82)) (probe (position 538 71))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::whileLoopActions"))) (kind subsetting) (ordinal 0) (authored-target "loopActions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::loopActions")))))
    )
  )
)
~~~
