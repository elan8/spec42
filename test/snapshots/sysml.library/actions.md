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
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 39 2) (end 39 48))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 40 2) (end 40 66))
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
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 62 3) (end 68 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 147 4) (end 147 14))
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
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 193 2) (end 193 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 194 5) (end 196 6))
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
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 212 2) (end 212 20))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 213 2) (end 215 9))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 225 2) (end 225 72))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 227 3) (end 227 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 229 2) (end 229 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 232 73) (end 232 91))
      )
      (diagnostic
        (severity error)
        (code "recovered_action_body_element")
        (source "parser")
        (range (start 247 2) (end 254 2))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 247 2) (end 254 2))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 254 31) (end 254 38))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 255 3) (end 255 33))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 280 2) (end 285 3))
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
        (severity error)
        (code "recovered_action_body_element")
        (source "parser")
        (range (start 337 2) (end 338 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 338 2) (end 340 9))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 342 2) (end 342 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 344 44) (end 344 52))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 346 2) (end 346 36))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 347 2) (end 347 50))
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
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 360 2) (end 360 32))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 361 2) (end 361 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 371 32) (end 371 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 379 2) (end 379 28))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 380 2) (end 381 1))
      )
      (diagnostic
        (severity error)
        (code "recovered_action_body_element")
        (source "parser")
        (range (start 389 8) (end 396 1))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 398 36) (end 398 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 405 2) (end 405 15))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 406 2) (end 406 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 409 46) (end 409 67))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 416 2) (end 416 15))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 417 2) (end 417 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 418 2) (end 418 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 441 8) (end 441 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 443 2) (end 448 3))
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
        (range (start 458 2) (end 465 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 467 2) (end 473 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 475 2) (end 481 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 491 2) (end 497 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 499 2) (end 504 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 506 2) (end 511 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 513 2) (end 519 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 522 3) (end 522 21))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 523 2) (end 524 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 524 3) (end 528 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:0683383b61e47daf5fc3d06f372c78670abac17c7ed407cc15dcc1a8429a1ac8") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Base::Anything") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Positive") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Natural") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::size") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::isEmpty") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 5))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Occurrences::Occurrence") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 6))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Occurrences::HappensWhile") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 7))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Performances::Performance") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 8))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Performances::performances") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 9))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Transfers::SendPerformance") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 10))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Transfers::sendPerformances") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 11))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Transfers::AcceptPerformance") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 12))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Transfers::acceptPerformances") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 13))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "FeatureReferencingPerformances::FeatureWritePerformance") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 14))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ControlPerformances::MergePerformance") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 15))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ControlPerformances::DecisionPerformance") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 16))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ControlPerformances::IfThenPerformance") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 17))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ControlPerformances::IfThenElsePerformance") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 18))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ControlPerformances::LoopPerformance") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 19))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "TransitionPerformances::TransitionPerformance") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 20))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "TransitionPerformances::NonStateTransitionPerformance") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 21))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Transfers::MessageTransfer") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 22))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Flows::MessageAction") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 23))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "OccurrenceFunctions::destroy") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptAction"))) (kind action-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "AcceptMessageAction"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptAction::aState"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptMessageAction"))) (kind action-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Action")) (specialization (reference "AcceptPerformance"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action"))) (kind action-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Performance"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::acceptSubactions"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AcceptAction")) (subsetting (reference "subactions")) (subsetting (reference "acceptActions"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::assignments"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AssignmentAction")) (subsetting (reference "subactions")) (subsetting (reference "assignmentActions"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::controls"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ControlAction")) (subsetting (reference "subactions"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::decisionTransitions"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DecisionTransitionAction")) (subsetting (reference "transitions"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::decisions"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DecisionAction")) (subsetting (reference "controls"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::done"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Action")) (redefinition (reference "endShot"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::forLoops"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ForLoopAction")) (subsetting (reference "loops")) (subsetting (reference "forLoopActions"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::forks"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ForkAction")) (subsetting (reference "controls"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::ifSubactions"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "IfThenAction")) (subsetting (reference "subactions")) (subsetting (reference "ifThenActions"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::joins"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "JoinAction")) (subsetting (reference "controls"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::loops"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LoopAction")) (subsetting (reference "subactions")) (subsetting (reference "loopActions"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::merges"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MergeAction")) (subsetting (reference "controls"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::sendSubactions"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SendAction")) (subsetting (reference "subactions")) (subsetting (reference "sendActions"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::start"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Action")) (redefinition (reference "startShot"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Action")) (subsetting (reference "actions")) (subsetting (reference "subperformances"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::terminateSubactions"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TerminateAction")) (subsetting (reference "subactions")) (subsetting (reference "terminateActions"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::transitions"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TransitionAction")) (subsetting (reference "subactions")) (subsetting (reference "transitionActions"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::whileLoops"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "WhileLoopAction")) (subsetting (reference "loops")) (subsetting (reference "whileLoopActions"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AssignmentAction"))) (kind action-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "FeatureWritePerformance")) (specialization (reference "Action"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ControlAction"))) (kind action-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Action"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::DecisionAction"))) (kind action-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ControlAction")) (specialization (reference "DecisionPerformance"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::DecisionTransitionAction"))) (kind action-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "TransitionAction")) (specialization (reference "NonStateTransitionPerformance"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction"))) (kind action-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "LoopAction"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction::initialization"))) (kind action) (membership (kind feature) (visibility private)))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForkAction"))) (kind action-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ControlAction"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenAction"))) (kind action-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Action")) (specialization (reference "IfThenPerformance"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenElseAction"))) (kind action-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "IfThenAction")) (specialization (reference "IfThenElsePerformance"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::JoinAction"))) (kind action-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ControlAction"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction"))) (kind action-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Action"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::MergeAction"))) (kind action-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ControlAction")) (specialization (reference "MergePerformance"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::SendAction"))) (kind action-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Action")) (specialization (reference "SendPerformance"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TerminateAction"))) (kind action-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Action"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TerminateAction::terminateOccurrence"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "destroy"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction"))) (kind action-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Action")) (specialization (reference "TransitionPerformance"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::accepter"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AcceptMessageAction")) (redefinition (reference "accept"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::effect"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Action")) (redefinition (reference "TransitionPerformance::effect"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::WhileLoopAction"))) (kind action-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "LoopAction")) (specialization (reference "LoopPerformance"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::acceptActions"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AcceptAction")) (subsetting (reference "actions")) (subsetting (reference "acceptPerformances"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Action")) (subsetting (reference "performances"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::assignmentActions"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AssignmentAction")) (subsetting (reference "actions"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::forLoopActions"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ForLoopAction")) (subsetting (reference "loopActions"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ifThenActions"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "IfThenAction")) (subsetting (reference "actions"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ifThenElseActions"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "IfThenElseAction")) (subsetting (reference "actions"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::loopActions"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LoopAction")) (subsetting (reference "actions"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::sendActions"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SendAction")) (subsetting (reference "actions")) (subsetting (reference "sendPerformances"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::terminateActions"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TerminateAction")) (subsetting (reference "actions"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::transitionActions"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TransitionAction")) (subsetting (reference "actions"))))
    (declaration (id (node (document "memory://snapshot/actions.md") (qualified-name "Actions::whileLoopActions"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "WhileLoopAction")) (subsetting (reference "loopActions"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Base::Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Positive")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Natural")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::size")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::isEmpty")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0))
      (authored-target "Occurrences::Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0))
      (authored-target "Occurrences::HappensWhile")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0))
      (authored-target "Performances::Performance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 8))))) (kind membershipImport) (ordinal 0))
      (authored-target "Performances::performances")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 9))))) (kind membershipImport) (ordinal 0))
      (authored-target "Transfers::SendPerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 10))))) (kind membershipImport) (ordinal 0))
      (authored-target "Transfers::sendPerformances")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 11))))) (kind membershipImport) (ordinal 0))
      (authored-target "Transfers::AcceptPerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 12))))) (kind membershipImport) (ordinal 0))
      (authored-target "Transfers::acceptPerformances")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 13))))) (kind membershipImport) (ordinal 0))
      (authored-target "FeatureReferencingPerformances::FeatureWritePerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 14))))) (kind membershipImport) (ordinal 0))
      (authored-target "ControlPerformances::MergePerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 15))))) (kind membershipImport) (ordinal 0))
      (authored-target "ControlPerformances::DecisionPerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 16))))) (kind membershipImport) (ordinal 0))
      (authored-target "ControlPerformances::IfThenPerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 17))))) (kind membershipImport) (ordinal 0))
      (authored-target "ControlPerformances::IfThenElsePerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 18))))) (kind membershipImport) (ordinal 0))
      (authored-target "ControlPerformances::LoopPerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 19))))) (kind membershipImport) (ordinal 0))
      (authored-target "TransitionPerformances::TransitionPerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 20))))) (kind membershipImport) (ordinal 0))
      (authored-target "TransitionPerformances::NonStateTransitionPerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 21))))) (kind membershipImport) (ordinal 0))
      (authored-target "Transfers::MessageTransfer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 22))))) (kind membershipImport) (ordinal 0))
      (authored-target "Flows::MessageAction")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 23))))) (kind membershipImport) (ordinal 0))
      (authored-target "OccurrenceFunctions::destroy")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptAction"))) (kind specialization) (ordinal 0))
      (authored-target "AcceptMessageAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptMessageAction")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptMessageAction"))) (kind specialization) (ordinal 0))
      (authored-target "Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptMessageAction"))) (kind specialization) (ordinal 1))
      (authored-target "AcceptPerformance")
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
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ControlAction"))) (kind specialization) (ordinal 0))
      (authored-target "Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")))))
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
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction"))) (kind specialization) (ordinal 0))
      (authored-target "LoopAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction")))))
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
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TerminateAction"))) (kind specialization) (ordinal 0))
      (authored-target "Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TerminateAction::terminateOccurrence"))) (kind featureTyping) (ordinal 0))
      (authored-target "destroy")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction"))) (kind specialization) (ordinal 0))
      (authored-target "Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction"))) (kind specialization) (ordinal 1))
      (authored-target "TransitionPerformance")
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
    (relationship (kind specialization) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForkAction"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ControlAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForkAction"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenAction"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenAction"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenElseAction"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenElseAction"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::JoinAction"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ControlAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::JoinAction"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::MergeAction"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ControlAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::MergeAction"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::SendAction"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::SendAction"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TerminateAction"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TerminateAction"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::accepter"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptMessageAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::accepter"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::effect"))) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::effect"))) (kind featureTyping) (ordinal 0)))
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
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/actions.md") (range (start 7 16) (end 7 30)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Base::Anything")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/actions.md") (range (start 8 16) (end 8 38)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Positive")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/actions.md") (range (start 9 16) (end 9 37)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Natural")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/actions.md") (range (start 10 16) (end 10 39)) (probe (position 10 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::size")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/actions.md") (range (start 11 16) (end 11 42)) (probe (position 11 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::isEmpty")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/actions.md") (range (start 12 16) (end 12 39)) (probe (position 12 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0) (authored-target "Occurrences::Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/actions.md") (range (start 13 16) (end 13 41)) (probe (position 13 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0) (authored-target "Occurrences::HappensWhile")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/actions.md") (range (start 14 16) (end 14 41)) (probe (position 14 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0) (authored-target "Performances::Performance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/actions.md") (range (start 15 16) (end 15 42)) (probe (position 15 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 8))))) (kind membershipImport) (ordinal 0) (authored-target "Performances::performances")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/actions.md") (range (start 16 16) (end 16 42)) (probe (position 16 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 9))))) (kind membershipImport) (ordinal 0) (authored-target "Transfers::SendPerformance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/actions.md") (range (start 17 16) (end 17 43)) (probe (position 17 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 10))))) (kind membershipImport) (ordinal 0) (authored-target "Transfers::sendPerformances")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/actions.md") (range (start 18 16) (end 18 44)) (probe (position 18 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 11))))) (kind membershipImport) (ordinal 0) (authored-target "Transfers::AcceptPerformance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/actions.md") (range (start 19 16) (end 19 45)) (probe (position 19 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 12))))) (kind membershipImport) (ordinal 0) (authored-target "Transfers::acceptPerformances")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/actions.md") (range (start 20 16) (end 20 71)) (probe (position 20 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 13))))) (kind membershipImport) (ordinal 0) (authored-target "FeatureReferencingPerformances::FeatureWritePerformance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/actions.md") (range (start 21 16) (end 21 53)) (probe (position 21 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 14))))) (kind membershipImport) (ordinal 0) (authored-target "ControlPerformances::MergePerformance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/actions.md") (range (start 22 16) (end 22 56)) (probe (position 22 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 15))))) (kind membershipImport) (ordinal 0) (authored-target "ControlPerformances::DecisionPerformance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/actions.md") (range (start 23 16) (end 23 54)) (probe (position 23 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 16))))) (kind membershipImport) (ordinal 0) (authored-target "ControlPerformances::IfThenPerformance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/actions.md") (range (start 24 16) (end 24 58)) (probe (position 24 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 17))))) (kind membershipImport) (ordinal 0) (authored-target "ControlPerformances::IfThenElsePerformance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/actions.md") (range (start 25 16) (end 25 52)) (probe (position 25 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 18))))) (kind membershipImport) (ordinal 0) (authored-target "ControlPerformances::LoopPerformance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/actions.md") (range (start 26 16) (end 26 61)) (probe (position 26 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 19))))) (kind membershipImport) (ordinal 0) (authored-target "TransitionPerformances::TransitionPerformance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/actions.md") (range (start 27 16) (end 27 69)) (probe (position 27 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 20))))) (kind membershipImport) (ordinal 0) (authored-target "TransitionPerformances::NonStateTransitionPerformance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/actions.md") (range (start 28 16) (end 28 42)) (probe (position 28 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 21))))) (kind membershipImport) (ordinal 0) (authored-target "Transfers::MessageTransfer")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/actions.md") (range (start 29 16) (end 29 36)) (probe (position 29 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 22))))) (kind membershipImport) (ordinal 0) (authored-target "Flows::MessageAction")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/actions.md") (range (start 30 16) (end 30 44)) (probe (position 30 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (anonymous (kind import) (ordinal 23))))) (kind membershipImport) (ordinal 0) (authored-target "OccurrenceFunctions::destroy")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/actions.md") (range (start 218 28) (end 218 47)) (probe (position 218 28))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptAction"))) (kind specialization) (ordinal 0) (authored-target "AcceptMessageAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptMessageAction")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 206 35) (end 206 41)) (probe (position 206 35))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptMessageAction"))) (kind specialization) (ordinal 0) (authored-target "Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 206 43) (end 206 60)) (probe (position 206 43))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptMessageAction"))) (kind specialization) (ordinal 1) (authored-target "AcceptPerformance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/actions.md") (range (start 32 31) (end 32 42)) (probe (position 32 31))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action"))) (kind specialization) (ordinal 0) (authored-target "Performance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/actions.md") (range (start 78 27) (end 78 39)) (probe (position 78 27))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::acceptSubactions"))) (kind featureTyping) (ordinal 0) (authored-target "AcceptAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptAction")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 78 49) (end 78 59)) (probe (position 78 49))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::acceptSubactions"))) (kind subsetting) (ordinal 0) (authored-target "subactions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 78 61) (end 78 74)) (probe (position 78 61))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::acceptSubactions"))) (kind subsetting) (ordinal 1) (authored-target "acceptActions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::acceptActions")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 141 32) (end 141 48)) (probe (position 141 32))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::assignments"))) (kind featureTyping) (ordinal 0) (authored-target "AssignmentAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AssignmentAction")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 141 58) (end 141 68)) (probe (position 141 58))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::assignments"))) (kind subsetting) (ordinal 0) (authored-target "subactions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 141 70) (end 141 87)) (probe (position 141 70))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::assignments"))) (kind subsetting) (ordinal 1) (authored-target "assignmentActions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::assignmentActions")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 92 29) (end 92 42)) (probe (position 92 29))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::controls"))) (kind featureTyping) (ordinal 0) (authored-target "ControlAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ControlAction")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 92 52) (end 92 62)) (probe (position 92 52))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::controls"))) (kind subsetting) (ordinal 0) (authored-target "subactions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 134 40) (end 134 64)) (probe (position 134 40))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::decisionTransitions"))) (kind featureTyping) (ordinal 0) (authored-target "DecisionTransitionAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::DecisionTransitionAction")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 134 74) (end 134 85)) (probe (position 134 74))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::decisionTransitions"))) (kind subsetting) (ordinal 0) (authored-target "transitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::transitions")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 106 30) (end 106 44)) (probe (position 106 30))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::decisions"))) (kind featureTyping) (ordinal 0) (authored-target "DecisionAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::DecisionAction")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 106 48) (end 106 56)) (probe (position 106 48))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::decisions"))) (kind subsetting) (ordinal 0) (authored-target "controls")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::controls")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 49 15) (end 49 21)) (probe (position 49 15))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::done"))) (kind featureTyping) (ordinal 0) (authored-target "Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 49 26) (end 49 33)) (probe (position 49 26))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::done"))) (kind redefinition) (ordinal 0) (authored-target "endShot")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/actions.md") (range (start 171 29) (end 171 42)) (probe (position 171 29))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::forLoops"))) (kind featureTyping) (ordinal 0) (authored-target "ForLoopAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 171 52) (end 171 57)) (probe (position 171 52))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::forLoops"))) (kind subsetting) (ordinal 0) (authored-target "loops")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::loops")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 171 59) (end 171 73)) (probe (position 171 59))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::forLoops"))) (kind subsetting) (ordinal 1) (authored-target "forLoopActions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::forLoopActions")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 120 26) (end 120 36)) (probe (position 120 26))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::forks"))) (kind featureTyping) (ordinal 0) (authored-target "ForkAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForkAction")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 120 40) (end 120 48)) (probe (position 120 40))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::forks"))) (kind subsetting) (ordinal 0) (authored-target "controls")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::controls")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 150 33) (end 150 45)) (probe (position 150 33))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::ifSubactions"))) (kind featureTyping) (ordinal 0) (authored-target "IfThenAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenAction")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 150 55) (end 150 65)) (probe (position 150 55))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::ifSubactions"))) (kind subsetting) (ordinal 0) (authored-target "subactions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 150 67) (end 150 80)) (probe (position 150 67))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::ifSubactions"))) (kind subsetting) (ordinal 1) (authored-target "ifThenActions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ifThenActions")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 113 26) (end 113 36)) (probe (position 113 26))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::joins"))) (kind featureTyping) (ordinal 0) (authored-target "JoinAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::JoinAction")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 113 40) (end 113 48)) (probe (position 113 40))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::joins"))) (kind subsetting) (ordinal 0) (authored-target "controls")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::controls")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 157 26) (end 157 36)) (probe (position 157 26))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::loops"))) (kind featureTyping) (ordinal 0) (authored-target "LoopAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 157 46) (end 157 56)) (probe (position 157 46))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::loops"))) (kind subsetting) (ordinal 0) (authored-target "subactions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 157 58) (end 157 69)) (probe (position 157 58))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::loops"))) (kind subsetting) (ordinal 1) (authored-target "loopActions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::loopActions")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 99 27) (end 99 38)) (probe (position 99 27))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::merges"))) (kind featureTyping) (ordinal 0) (authored-target "MergeAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::MergeAction")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 99 48) (end 99 56)) (probe (position 99 48))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::merges"))) (kind subsetting) (ordinal 0) (authored-target "controls")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::controls")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 71 25) (end 71 35)) (probe (position 71 25))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::sendSubactions"))) (kind featureTyping) (ordinal 0) (authored-target "SendAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::SendAction")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 71 45) (end 71 55)) (probe (position 71 45))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::sendSubactions"))) (kind subsetting) (ordinal 0) (authored-target "subactions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 71 57) (end 71 68)) (probe (position 71 57))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::sendSubactions"))) (kind subsetting) (ordinal 1) (authored-target "sendActions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::sendActions")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 42 16) (end 42 22)) (probe (position 42 16))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::start"))) (kind featureTyping) (ordinal 0) (authored-target "Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 42 27) (end 42 36)) (probe (position 42 27))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::start"))) (kind redefinition) (ordinal 0) (authored-target "startShot")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/actions.md") (range (start 56 21) (end 56 27)) (probe (position 56 21))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions"))) (kind featureTyping) (ordinal 0) (authored-target "Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 56 37) (end 56 44)) (probe (position 56 37))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions"))) (kind subsetting) (ordinal 0) (authored-target "actions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 56 46) (end 56 61)) (probe (position 56 46))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions"))) (kind subsetting) (ordinal 1) (authored-target "subperformances")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/actions.md") (range (start 85 40) (end 85 55)) (probe (position 85 40))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::terminateSubactions"))) (kind featureTyping) (ordinal 0) (authored-target "TerminateAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TerminateAction")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 85 65) (end 85 75)) (probe (position 85 65))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::terminateSubactions"))) (kind subsetting) (ordinal 0) (authored-target "subactions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 85 77) (end 85 93)) (probe (position 85 77))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::terminateSubactions"))) (kind subsetting) (ordinal 1) (authored-target "terminateActions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::terminateActions")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 127 32) (end 127 48)) (probe (position 127 32))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::transitions"))) (kind featureTyping) (ordinal 0) (authored-target "TransitionAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 127 58) (end 127 68)) (probe (position 127 58))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::transitions"))) (kind subsetting) (ordinal 0) (authored-target "subactions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::subactions")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 127 70) (end 127 87)) (probe (position 127 70))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::transitions"))) (kind subsetting) (ordinal 1) (authored-target "transitionActions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::transitionActions")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 164 31) (end 164 46)) (probe (position 164 31))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::whileLoops"))) (kind featureTyping) (ordinal 0) (authored-target "WhileLoopAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::WhileLoopAction")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 164 56) (end 164 61)) (probe (position 164 56))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::whileLoops"))) (kind subsetting) (ordinal 0) (authored-target "loops")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::loops")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 164 63) (end 164 79)) (probe (position 164 63))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action::whileLoops"))) (kind subsetting) (ordinal 1) (authored-target "whileLoopActions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::whileLoopActions")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 371 32) (end 371 55)) (probe (position 371 32))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AssignmentAction"))) (kind specialization) (ordinal 0) (authored-target "FeatureWritePerformance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/actions.md") (range (start 371 57) (end 371 63)) (probe (position 371 57))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AssignmentAction"))) (kind specialization) (ordinal 1) (authored-target "Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 274 38) (end 274 44)) (probe (position 274 38))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ControlAction"))) (kind specialization) (ordinal 0) (authored-target "Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 298 30) (end 298 43)) (probe (position 298 30))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::DecisionAction"))) (kind specialization) (ordinal 0) (authored-target "ControlAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ControlAction")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 298 45) (end 298 64)) (probe (position 298 45))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::DecisionAction"))) (kind specialization) (ordinal 1) (authored-target "DecisionPerformance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/actions.md") (range (start 352 40) (end 352 56)) (probe (position 352 40))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::DecisionTransitionAction"))) (kind specialization) (ordinal 0) (authored-target "TransitionAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 352 58) (end 352 87)) (probe (position 352 58))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::DecisionTransitionAction"))) (kind specialization) (ordinal 1) (authored-target "NonStateTransitionPerformance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/actions.md") (range (start 484 29) (end 484 39)) (probe (position 484 29))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction"))) (kind specialization) (ordinal 0) (authored-target "LoopAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 320 26) (end 320 39)) (probe (position 320 26))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForkAction"))) (kind specialization) (ordinal 0) (authored-target "ControlAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ControlAction")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 398 28) (end 398 34)) (probe (position 398 28))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenAction"))) (kind specialization) (ordinal 0) (authored-target "Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 398 36) (end 398 53)) (probe (position 398 36))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenAction"))) (kind specialization) (ordinal 1) (authored-target "IfThenPerformance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/actions.md") (range (start 409 32) (end 409 44)) (probe (position 409 32))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenElseAction"))) (kind specialization) (ordinal 0) (authored-target "IfThenAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenAction")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 409 46) (end 409 67)) (probe (position 409 46))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenElseAction"))) (kind specialization) (ordinal 1) (authored-target "IfThenElsePerformance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/actions.md") (range (start 310 26) (end 310 39)) (probe (position 310 26))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::JoinAction"))) (kind specialization) (ordinal 0) (authored-target "ControlAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ControlAction")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 435 35) (end 435 41)) (probe (position 435 35))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction"))) (kind specialization) (ordinal 0) (authored-target "Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 288 27) (end 288 40)) (probe (position 288 27))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::MergeAction"))) (kind specialization) (ordinal 0) (authored-target "ControlAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ControlAction")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 288 42) (end 288 58)) (probe (position 288 42))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::MergeAction"))) (kind specialization) (ordinal 1) (authored-target "MergePerformance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/actions.md") (range (start 186 26) (end 186 32)) (probe (position 186 26))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::SendAction"))) (kind specialization) (ordinal 0) (authored-target "Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 186 34) (end 186 49)) (probe (position 186 34))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::SendAction"))) (kind specialization) (ordinal 1) (authored-target "SendPerformance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/actions.md") (range (start 239 40) (end 239 46)) (probe (position 239 40))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TerminateAction"))) (kind specialization) (ordinal 0) (authored-target "Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 254 31) (end 254 38)) (probe (position 254 31))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TerminateAction::terminateOccurrence"))) (kind featureTyping) (ordinal 0) (authored-target "destroy")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/actions.md") (range (start 330 41) (end 330 47)) (probe (position 330 41))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction"))) (kind specialization) (ordinal 0) (authored-target "Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 330 49) (end 330 70)) (probe (position 330 49))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction"))) (kind specialization) (ordinal 1) (authored-target "TransitionPerformance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/actions.md") (range (start 344 20) (end 344 39)) (probe (position 344 20))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::accepter"))) (kind featureTyping) (ordinal 0) (authored-target "AcceptMessageAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptMessageAction")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 344 44) (end 344 52)) (probe (position 344 44))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::accepter"))) (kind redefinition) (ordinal 0) (authored-target "accept")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/actions.md") (range (start 349 17) (end 349 23)) (probe (position 349 17))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::effect"))) (kind featureTyping) (ordinal 0) (authored-target "Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 349 28) (end 349 57)) (probe (position 349 28))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction::effect"))) (kind redefinition) (ordinal 0) (authored-target "TransitionPerformance::effect")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/actions.md") (range (start 451 31) (end 451 41)) (probe (position 451 31))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::WhileLoopAction"))) (kind specialization) (ordinal 0) (authored-target "LoopAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 451 43) (end 451 58)) (probe (position 451 43))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::WhileLoopAction"))) (kind specialization) (ordinal 1) (authored-target "LoopPerformance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/actions.md") (range (start 232 32) (end 232 44)) (probe (position 232 32))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::acceptActions"))) (kind featureTyping) (ordinal 0) (authored-target "AcceptAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AcceptAction")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 232 64) (end 232 71)) (probe (position 232 64))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::acceptActions"))) (kind subsetting) (ordinal 0) (authored-target "actions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 232 73) (end 232 91)) (probe (position 232 73))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::acceptActions"))) (kind subsetting) (ordinal 1) (authored-target "acceptPerformances")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/actions.md") (range (start 179 26) (end 179 32)) (probe (position 179 26))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions"))) (kind featureTyping) (ordinal 0) (authored-target "Action")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::Action")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 179 52) (end 179 64)) (probe (position 179 52))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions"))) (kind subsetting) (ordinal 0) (authored-target "performances")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/actions.md") (range (start 383 37) (end 383 53)) (probe (position 383 37))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::assignmentActions"))) (kind featureTyping) (ordinal 0) (authored-target "AssignmentAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::AssignmentAction")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 383 73) (end 383 80)) (probe (position 383 73))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::assignmentActions"))) (kind subsetting) (ordinal 0) (authored-target "actions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 545 34) (end 545 47)) (probe (position 545 34))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::forLoopActions"))) (kind featureTyping) (ordinal 0) (authored-target "ForLoopAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ForLoopAction")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 545 67) (end 545 78)) (probe (position 545 67))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::forLoopActions"))) (kind subsetting) (ordinal 0) (authored-target "loopActions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::loopActions")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 421 33) (end 421 45)) (probe (position 421 33))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ifThenActions"))) (kind featureTyping) (ordinal 0) (authored-target "IfThenAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenAction")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 421 65) (end 421 72)) (probe (position 421 65))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ifThenActions"))) (kind subsetting) (ordinal 0) (authored-target "actions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 428 37) (end 428 53)) (probe (position 428 37))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ifThenElseActions"))) (kind featureTyping) (ordinal 0) (authored-target "IfThenElseAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::IfThenElseAction")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 428 73) (end 428 80)) (probe (position 428 73))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::ifThenElseActions"))) (kind subsetting) (ordinal 0) (authored-target "actions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 531 31) (end 531 41)) (probe (position 531 31))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::loopActions"))) (kind featureTyping) (ordinal 0) (authored-target "LoopAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::LoopAction")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 531 61) (end 531 68)) (probe (position 531 61))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::loopActions"))) (kind subsetting) (ordinal 0) (authored-target "actions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 199 30) (end 199 40)) (probe (position 199 30))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::sendActions"))) (kind featureTyping) (ordinal 0) (authored-target "SendAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::SendAction")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 199 60) (end 199 67)) (probe (position 199 60))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::sendActions"))) (kind subsetting) (ordinal 0) (authored-target "actions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 199 69) (end 199 85)) (probe (position 199 69))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::sendActions"))) (kind subsetting) (ordinal 1) (authored-target "sendPerformances")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/actions.md") (range (start 259 36) (end 259 51)) (probe (position 259 36))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::terminateActions"))) (kind featureTyping) (ordinal 0) (authored-target "TerminateAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TerminateAction")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 259 71) (end 259 78)) (probe (position 259 71))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::terminateActions"))) (kind subsetting) (ordinal 0) (authored-target "actions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 364 36) (end 364 52)) (probe (position 364 36))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::transitionActions"))) (kind featureTyping) (ordinal 0) (authored-target "TransitionAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::TransitionAction")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 364 72) (end 364 79)) (probe (position 364 72))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::transitionActions"))) (kind subsetting) (ordinal 0) (authored-target "actions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::actions")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 538 36) (end 538 51)) (probe (position 538 36))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::whileLoopActions"))) (kind featureTyping) (ordinal 0) (authored-target "WhileLoopAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::WhileLoopAction")))))
  )
  (query (document "memory://snapshot/actions.md") (range (start 538 71) (end 538 82)) (probe (position 538 71))
    (reference (id (source (node (document "memory://snapshot/actions.md") (qualified-name "Actions::whileLoopActions"))) (kind subsetting) (ordinal 0) (authored-target "loopActions")
      (outcome (status resolved) (target (node (document "memory://snapshot/actions.md") (qualified-name "Actions::loopActions")))))
  )
)
~~~
