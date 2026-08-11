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
  (document "actions.md"
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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 194 25) (end 194 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 213 26) (end 213 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 229 17) (end 229 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 247 2) (end 247 104))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 265 2) (end 265 231))
      )
      (diagnostic
        (severity warning)
        (code "connection_context_invalid")
        (source "semantic")
        (range (start 280 7) (end 280 12))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 337 2) (end 337 83))
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
        (range (start 347 25) (end 347 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 349 28) (end 349 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 379 2) (end 379 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 380 2) (end 380 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 389 8) (end 389 245))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 405 2) (end 405 15))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 406 2) (end 406 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 416 2) (end 416 15))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 417 2) (end 417 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 418 2) (end 418 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 441 8) (end 441 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 443 2) (end 443 107))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 491 29) (end 491 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 499 2) (end 499 97))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 524 3) (end 524 124))
      )
    )
  )
)
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
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "5e7f567998c3514a84521c656b80b5f6e136a042587305fc83fbbeb0af44ab97") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Actions"))) (kind "package") (name "Actions") (declared-name "Actions"))
    (element (id (node (document "d0") (qualified-name "Actions::AcceptAction"))) (kind "action def") (name "AcceptAction") (declared-name "AcceptAction") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "AcceptMessageAction")) (specializes (reference "AcceptMessageAction")) (specializes (reference "AcceptMessageAction")))))
    (element (id (node (document "d0") (qualified-name "Actions::AcceptAction::"))) (kind "ref") (name "") (parent (node (document "d0") (qualified-name "Actions::AcceptAction"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "acceptedMessage")))))
    (element (id (node (document "d0") (qualified-name "Actions::AcceptAction::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::AcceptAction"))))
    (element (id (node (document "d0") (qualified-name "Actions::AcceptAction::aState"))) (kind "state") (name "aState") (declared-name "aState") (parent (node (document "d0") (qualified-name "Actions::AcceptAction"))))
    (element (id (node (document "d0") (qualified-name "Actions::AcceptAction::aState::aTransition"))) (kind "transition") (name "aTransition") (declared-name "aTransition") (parent (node (document "d0") (qualified-name "Actions::AcceptAction::aState"))))
    (element (id (node (document "d0") (qualified-name "Actions::AcceptAction::aState::aTransition::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "Actions::AcceptAction::aState::aTransition"))))
    (element (id (node (document "d0") (qualified-name "Actions::AcceptMessageAction"))) (kind "action def") (name "AcceptMessageAction") (declared-name "AcceptMessageAction") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Action")) (specializes (reference "AcceptPerformance")) (specializes (reference "Action")) (specializes (reference "AcceptPerformance")) (specializes (reference "Action")) (specializes (reference "AcceptPerformance")))))
    (element (id (node (document "d0") (qualified-name "Actions::AcceptMessageAction::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::AcceptMessageAction"))))
    (element (id (node (document "d0") (qualified-name "Actions::AcceptMessageAction::acceptedMessage"))) (kind "ref") (name "acceptedMessage") (declared-name "acceptedMessage") (parent (node (document "d0") (qualified-name "Actions::AcceptMessageAction"))) (authored (membership (kind Feature)) (relationships (typing (reference "MessageTransfer")) (typing (reference "MessageAction")) (redefinition (reference "acceptedTransfer")))))
    (element (id (node (document "d0") (qualified-name "Actions::AcceptMessageAction::acceptedMessage::MessageTransfer::payload, MessageAction::payload"))) (kind "in out parameter") (name "MessageTransfer::payload, MessageAction::payload") (declared-name "MessageTransfer::payload, MessageAction::payload") (parent (node (document "d0") (qualified-name "Actions::AcceptMessageAction::acceptedMessage"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Actions::AcceptMessageAction::payload"))) (kind "in out parameter") (name "payload") (declared-name "payload") (parent (node (document "d0") (qualified-name "Actions::AcceptMessageAction"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Actions::AcceptPerformance"))) (kind "import") (name "AcceptPerformance") (declared-name "AcceptPerformance") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::AcceptPerformance") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Actions::Action"))) (kind "action def") (name "Action") (declared-name "Action") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Performance")) (specializes (reference "Performance")) (specializes (reference "Performance")) (perform (reference "Actions::Action::start")) (perform (reference "Actions::Action::done")) (perform (reference "Actions::Action::subactions")) (perform (reference "Actions::Action::sendSubactions")) (perform (reference "Actions::Action::acceptSubactions")) (perform (reference "Actions::Action::terminateSubactions")) (perform (reference "Actions::Action::controls")) (perform (reference "Actions::Action::merges")) (perform (reference "Actions::Action::decisions")) (perform (reference "Actions::Action::joins")) (perform (reference "Actions::Action::forks")) (perform (reference "Actions::Action::transitions")) (perform (reference "Actions::Action::decisionTransitions")) (perform (reference "Actions::Action::assignments")) (perform (reference "Actions::Action::ifSubactions")) (perform (reference "Actions::Action::loops")) (perform (reference "Actions::Action::whileLoops")) (perform (reference "Actions::Action::forLoops")))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::Action"))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::acceptSubactions"))) (kind "action") (name "acceptSubactions") (declared-name "acceptSubactions") (parent (node (document "d0") (qualified-name "Actions::Action"))) (authored (membership (kind Feature)) (relationships (typing (reference "AcceptAction")) (subsetting (reference "subactions")) (subsetting (reference "acceptActions")))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::acceptSubactions::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::Action::acceptSubactions"))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::assignments"))) (kind "action") (name "assignments") (declared-name "assignments") (parent (node (document "d0") (qualified-name "Actions::Action"))) (authored (membership (kind Feature)) (relationships (typing (reference "AssignmentAction")) (subsetting (reference "subactions")) (subsetting (reference "assignmentActions")))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::assignments::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::Action::assignments"))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::assignments::target"))) (kind "in out parameter") (name "target") (declared-name "target") (parent (node (document "d0") (qualified-name "Actions::Action::assignments"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::controls"))) (kind "action") (name "controls") (declared-name "controls") (parent (node (document "d0") (qualified-name "Actions::Action"))) (authored (membership (kind Feature)) (relationships (typing (reference "ControlAction")) (subsetting (reference "subactions")))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::controls::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::Action::controls"))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::decisionTransitions"))) (kind "action") (name "decisionTransitions") (declared-name "decisionTransitions") (parent (node (document "d0") (qualified-name "Actions::Action"))) (authored (membership (kind Feature)) (relationships (typing (reference "DecisionTransitionAction")) (subsetting (reference "transitions")))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::decisionTransitions::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::Action::decisionTransitions"))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::decisions"))) (kind "action") (name "decisions") (declared-name "decisions") (parent (node (document "d0") (qualified-name "Actions::Action"))) (authored (membership (kind Feature)) (relationships (typing (reference "DecisionAction")) (subsetting (reference "controls")))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::decisions::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::Action::decisions"))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::done"))) (kind "action") (name "done") (declared-name "done") (parent (node (document "d0") (qualified-name "Actions::Action"))) (authored (membership (kind Feature)) (relationships (typing (reference "Action")) (redefinition (reference "endShot")))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::done::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::Action::done"))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::forLoops"))) (kind "action") (name "forLoops") (declared-name "forLoops") (parent (node (document "d0") (qualified-name "Actions::Action"))) (authored (membership (kind Feature)) (relationships (typing (reference "ForLoopAction")) (subsetting (reference "loops")) (subsetting (reference "forLoopActions")))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::forLoops::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::Action::forLoops"))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::forks"))) (kind "action") (name "forks") (declared-name "forks") (parent (node (document "d0") (qualified-name "Actions::Action"))) (authored (membership (kind Feature)) (relationships (typing (reference "ForkAction")) (subsetting (reference "controls")))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::forks::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::Action::forks"))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::ifSubactions"))) (kind "action") (name "ifSubactions") (declared-name "ifSubactions") (parent (node (document "d0") (qualified-name "Actions::Action"))) (authored (membership (kind Feature)) (relationships (typing (reference "IfThenAction")) (subsetting (reference "subactions")) (subsetting (reference "ifThenActions")))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::ifSubactions::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::Action::ifSubactions"))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::incomingTransfers"))) (kind "ref") (name "incomingTransfers") (declared-name "incomingTransfers") (parent (node (document "d0") (qualified-name "Actions::Action"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "Performance::incomingTransfers")))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::joins"))) (kind "action") (name "joins") (declared-name "joins") (parent (node (document "d0") (qualified-name "Actions::Action"))) (authored (membership (kind Feature)) (relationships (typing (reference "JoinAction")) (subsetting (reference "controls")))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::joins::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::Action::joins"))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::loops"))) (kind "action") (name "loops") (declared-name "loops") (parent (node (document "d0") (qualified-name "Actions::Action"))) (authored (membership (kind Feature)) (relationships (typing (reference "LoopAction")) (subsetting (reference "subactions")) (subsetting (reference "loopActions")))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::loops::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::Action::loops"))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::merges"))) (kind "action") (name "merges") (declared-name "merges") (parent (node (document "d0") (qualified-name "Actions::Action"))) (authored (membership (kind Feature)) (relationships (typing (reference "MergeAction")) (subsetting (reference "controls")))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::merges::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::Action::merges"))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::self"))) (kind "ref") (name "self") (declared-name "self") (parent (node (document "d0") (qualified-name "Actions::Action"))) (authored (membership (kind Feature)) (relationships (typing (reference "Action")))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::sendSubactions"))) (kind "action") (name "sendSubactions") (declared-name "sendSubactions") (parent (node (document "d0") (qualified-name "Actions::Action"))) (authored (membership (kind Feature)) (relationships (typing (reference "SendAction")) (subsetting (reference "subactions")) (subsetting (reference "sendActions")))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::sendSubactions::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::Action::sendSubactions"))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::start"))) (kind "action") (name "start") (declared-name "start") (parent (node (document "d0") (qualified-name "Actions::Action"))) (authored (membership (kind Feature)) (relationships (typing (reference "Action")) (redefinition (reference "startShot")))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::start::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::Action::start"))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::subactions"))) (kind "action") (name "subactions") (declared-name "subactions") (parent (node (document "d0") (qualified-name "Actions::Action"))) (authored (membership (kind Feature)) (relationships (typing (reference "Action")) (subsetting (reference "actions")) (subsetting (reference "subperformances")))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::subactions::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::Action::subactions"))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::subactions::occurrence"))) (kind "ref") (name "occurrence") (declared-name "occurrence") (parent (node (document "d0") (qualified-name "Actions::Action::subactions"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "Action::this")))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::subactions::occurrence::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::Action::subactions::occurrence"))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::terminateSubactions"))) (kind "action") (name "terminateSubactions") (declared-name "terminateSubactions") (parent (node (document "d0") (qualified-name "Actions::Action"))) (authored (membership (kind Feature)) (relationships (typing (reference "TerminateAction")) (subsetting (reference "subactions")) (subsetting (reference "terminateActions")))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::terminateSubactions::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::Action::terminateSubactions"))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::transitions"))) (kind "action") (name "transitions") (declared-name "transitions") (parent (node (document "d0") (qualified-name "Actions::Action"))) (authored (membership (kind Feature)) (relationships (typing (reference "TransitionAction")) (subsetting (reference "subactions")) (subsetting (reference "transitionActions")))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::transitions::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::Action::transitions"))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::whileLoops"))) (kind "action") (name "whileLoops") (declared-name "whileLoops") (parent (node (document "d0") (qualified-name "Actions::Action"))) (authored (membership (kind Feature)) (relationships (typing (reference "WhileLoopAction")) (subsetting (reference "loops")) (subsetting (reference "whileLoopActions")))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::whileLoops::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::Action::whileLoops"))))
    (element (id (node (document "d0") (qualified-name "Actions::Anything"))) (kind "import") (name "Anything") (declared-name "Anything") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Import) (visibility "private") (import (reference "Base::Anything") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Actions::AssignmentAction"))) (kind "action def") (name "AssignmentAction") (declared-name "AssignmentAction") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "FeatureWritePerformance")) (specializes (reference "Action")) (specializes (reference "FeatureWritePerformance")) (specializes (reference "Action")) (specializes (reference "FeatureWritePerformance")) (specializes (reference "Action")))))
    (element (id (node (document "d0") (qualified-name "Actions::AssignmentAction::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::AssignmentAction"))))
    (element (id (node (document "d0") (qualified-name "Actions::AssignmentAction::replacementValues"))) (kind "in out parameter") (name "replacementValues") (declared-name "replacementValues") (parent (node (document "d0") (qualified-name "Actions::AssignmentAction"))) (authored (relationships (typing (reference "replacementValues : Anything[0..*] nonunique")))))
    (element (id (node (document "d0") (qualified-name "Actions::AssignmentAction::target"))) (kind "in out parameter") (name "target") (declared-name "target") (parent (node (document "d0") (qualified-name "Actions::AssignmentAction"))) (authored (relationships (typing (reference "target : Occurrence[1]")))))
    (element (id (node (document "d0") (qualified-name "Actions::ControlAction"))) (kind "action def") (name "ControlAction") (declared-name "ControlAction") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Action")) (specializes (reference "Action")) (specializes (reference "Action")))))
    (element (id (node (document "d0") (qualified-name "Actions::ControlAction::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::ControlAction"))))
    (element (id (node (document "d0") (qualified-name "Actions::DecisionAction"))) (kind "action def") (name "DecisionAction") (declared-name "DecisionAction") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ControlAction")) (specializes (reference "DecisionPerformance")) (specializes (reference "ControlAction")) (specializes (reference "DecisionPerformance")) (specializes (reference "ControlAction")) (specializes (reference "DecisionPerformance")))))
    (element (id (node (document "d0") (qualified-name "Actions::DecisionAction::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::DecisionAction"))))
    (element (id (node (document "d0") (qualified-name "Actions::DecisionPerformance"))) (kind "import") (name "DecisionPerformance") (declared-name "DecisionPerformance") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlPerformances::DecisionPerformance") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Actions::DecisionTransitionAction"))) (kind "action def") (name "DecisionTransitionAction") (declared-name "DecisionTransitionAction") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "TransitionAction")) (specializes (reference "NonStateTransitionPerformance")) (specializes (reference "TransitionAction")) (specializes (reference "NonStateTransitionPerformance")) (specializes (reference "TransitionAction")) (specializes (reference "NonStateTransitionPerformance")))))
    (element (id (node (document "d0") (qualified-name "Actions::DecisionTransitionAction::"))) (kind "ref") (name "") (parent (node (document "d0") (qualified-name "Actions::DecisionTransitionAction"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "accepter")))))
    (element (id (node (document "d0") (qualified-name "Actions::DecisionTransitionAction::#ref"))) (kind "ref") (name "") (parent (node (document "d0") (qualified-name "Actions::DecisionTransitionAction"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "effect")))))
    (element (id (node (document "d0") (qualified-name "Actions::DecisionTransitionAction::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::DecisionTransitionAction"))))
    (element (id (node (document "d0") (qualified-name "Actions::FeatureWritePerformance"))) (kind "import") (name "FeatureWritePerformance") (declared-name "FeatureWritePerformance") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Import) (visibility "private") (import (reference "FeatureReferencingPerformances::FeatureWritePerformance") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Actions::ForLoopAction"))) (kind "action def") (name "ForLoopAction") (declared-name "ForLoopAction") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "LoopAction")) (specializes (reference "LoopAction")) (specializes (reference "LoopAction")) (perform (reference "Actions::ForLoopAction::initialization")) (perform (reference "Actions::ForLoopAction::whileLoop")))))
    (element (id (node (document "d0") (qualified-name "Actions::ForLoopAction::_assign"))) (kind "assign") (name "assign") (declared-name "assign") (parent (node (document "d0") (qualified-name "Actions::ForLoopAction"))))
    (element (id (node (document "d0") (qualified-name "Actions::ForLoopAction::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::ForLoopAction"))))
    (element (id (node (document "d0") (qualified-name "Actions::ForLoopAction::_while"))) (kind "while") (name "while") (declared-name "while") (parent (node (document "d0") (qualified-name "Actions::ForLoopAction"))) (authored (relationships (flow (reference "Actions::ForLoopAction::_while::body")))))
    (element (id (node (document "d0") (qualified-name "Actions::ForLoopAction::_while::_assign"))) (kind "assign") (name "assign") (declared-name "assign") (parent (node (document "d0") (qualified-name "Actions::ForLoopAction::_while"))))
    (element (id (node (document "d0") (qualified-name "Actions::ForLoopAction::_while::_assign#assign"))) (kind "assign") (name "assign") (declared-name "assign") (parent (node (document "d0") (qualified-name "Actions::ForLoopAction::_while"))))
    (element (id (node (document "d0") (qualified-name "Actions::ForLoopAction::body"))) (kind "in out parameter") (name "body") (declared-name "body") (parent (node (document "d0") (qualified-name "Actions::ForLoopAction"))) (authored (relationships (typing (reference "action")))))
    (element (id (node (document "d0") (qualified-name "Actions::ForLoopAction::index : Positive"))) (kind "action body decl") (name "index : Positive") (declared-name "index : Positive") (parent (node (document "d0") (qualified-name "Actions::ForLoopAction"))))
    (element (id (node (document "d0") (qualified-name "Actions::ForLoopAction::initialization"))) (kind "action") (name "initialization") (declared-name "initialization") (parent (node (document "d0") (qualified-name "Actions::ForLoopAction"))))
    (element (id (node (document "d0") (qualified-name "Actions::ForLoopAction::ref"))) (kind "in out parameter") (name "ref") (declared-name "ref") (parent (node (document "d0") (qualified-name "Actions::ForLoopAction"))) (authored (relationships (typing (reference "ref seq")))))
    (element (id (node (document "d0") (qualified-name "Actions::ForLoopAction::var"))) (kind "ref") (name "var") (declared-name "var") (parent (node (document "d0") (qualified-name "Actions::ForLoopAction"))) (authored (membership (kind Feature)) (relationships (typing (reference "seq")))))
    (element (id (node (document "d0") (qualified-name "Actions::ForLoopAction::var::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::ForLoopAction::var"))))
    (element (id (node (document "d0") (qualified-name "Actions::ForLoopAction::whileLoop"))) (kind "action") (name "whileLoop") (declared-name "whileLoop") (parent (node (document "d0") (qualified-name "Actions::ForLoopAction"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Actions::ForkAction"))) (kind "action def") (name "ForkAction") (declared-name "ForkAction") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ControlAction")) (specializes (reference "ControlAction")) (specializes (reference "ControlAction")))))
    (element (id (node (document "d0") (qualified-name "Actions::ForkAction::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::ForkAction"))))
    (element (id (node (document "d0") (qualified-name "Actions::HappensWhile"))) (kind "import") (name "HappensWhile") (declared-name "HappensWhile") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::HappensWhile") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Actions::IfThenAction"))) (kind "action def") (name "IfThenAction") (declared-name "IfThenAction") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Action")) (specializes (reference "IfThenPerformance")) (specializes (reference "Action")) (specializes (reference "IfThenPerformance")) (specializes (reference "Action")) (specializes (reference "IfThenPerformance")))))
    (element (id (node (document "d0") (qualified-name "Actions::IfThenAction::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::IfThenAction"))))
    (element (id (node (document "d0") (qualified-name "Actions::IfThenAction::action"))) (kind "in out parameter") (name "action") (declared-name "action") (parent (node (document "d0") (qualified-name "Actions::IfThenAction"))) (authored (relationships (typing (reference "action thenClause[0..1]")))))
    (element (id (node (document "d0") (qualified-name "Actions::IfThenAction::ifTest"))) (kind "in out parameter") (name "ifTest") (declared-name "ifTest") (parent (node (document "d0") (qualified-name "Actions::IfThenAction"))) (authored (relationships (typing (reference "ifTest[1]")))))
    (element (id (node (document "d0") (qualified-name "Actions::IfThenElseAction"))) (kind "action def") (name "IfThenElseAction") (declared-name "IfThenElseAction") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "IfThenAction")) (specializes (reference "IfThenElsePerformance")) (specializes (reference "IfThenAction")) (specializes (reference "IfThenElsePerformance")) (specializes (reference "IfThenAction")) (specializes (reference "IfThenElsePerformance")))))
    (element (id (node (document "d0") (qualified-name "Actions::IfThenElseAction::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::IfThenElseAction"))))
    (element (id (node (document "d0") (qualified-name "Actions::IfThenElseAction::action"))) (kind "in out parameter") (name "action") (declared-name "action") (parent (node (document "d0") (qualified-name "Actions::IfThenElseAction"))) (authored (relationships (typing (reference "action thenClause[0..1]")))))
    (element (id (node (document "d0") (qualified-name "Actions::IfThenElseAction::action#in_out_parameter"))) (kind "in out parameter") (name "action") (declared-name "action") (parent (node (document "d0") (qualified-name "Actions::IfThenElseAction"))) (authored (relationships (typing (reference "action elseClause[0..1]")))))
    (element (id (node (document "d0") (qualified-name "Actions::IfThenElseAction::ifTest"))) (kind "in out parameter") (name "ifTest") (declared-name "ifTest") (parent (node (document "d0") (qualified-name "Actions::IfThenElseAction"))) (authored (relationships (typing (reference "ifTest[1]")))))
    (element (id (node (document "d0") (qualified-name "Actions::IfThenElsePerformance"))) (kind "import") (name "IfThenElsePerformance") (declared-name "IfThenElsePerformance") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlPerformances::IfThenElsePerformance") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Actions::IfThenPerformance"))) (kind "import") (name "IfThenPerformance") (declared-name "IfThenPerformance") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlPerformances::IfThenPerformance") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Actions::JoinAction"))) (kind "action def") (name "JoinAction") (declared-name "JoinAction") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ControlAction")) (specializes (reference "ControlAction")) (specializes (reference "ControlAction")))))
    (element (id (node (document "d0") (qualified-name "Actions::JoinAction::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::JoinAction"))))
    (element (id (node (document "d0") (qualified-name "Actions::LoopAction"))) (kind "action def") (name "LoopAction") (declared-name "LoopAction") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Action")) (specializes (reference "Action")) (specializes (reference "Action")))))
    (element (id (node (document "d0") (qualified-name "Actions::LoopAction::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::LoopAction"))))
    (element (id (node (document "d0") (qualified-name "Actions::LoopAction::action"))) (kind "in out parameter") (name "action") (declared-name "action") (parent (node (document "d0") (qualified-name "Actions::LoopAction"))) (authored (relationships (typing (reference "action body[0..*]")))))
    (element (id (node (document "d0") (qualified-name "Actions::LoopAction::ref"))) (kind "in out parameter") (name "ref") (declared-name "ref") (parent (node (document "d0") (qualified-name "Actions::LoopAction"))) (authored (relationships (typing (reference "ref iterator")))))
    (element (id (node (document "d0") (qualified-name "Actions::LoopPerformance"))) (kind "import") (name "LoopPerformance") (declared-name "LoopPerformance") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlPerformances::LoopPerformance") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Actions::MergeAction"))) (kind "action def") (name "MergeAction") (declared-name "MergeAction") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ControlAction")) (specializes (reference "MergePerformance")) (specializes (reference "ControlAction")) (specializes (reference "MergePerformance")) (specializes (reference "ControlAction")) (specializes (reference "MergePerformance")))))
    (element (id (node (document "d0") (qualified-name "Actions::MergeAction::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::MergeAction"))))
    (element (id (node (document "d0") (qualified-name "Actions::MergePerformance"))) (kind "import") (name "MergePerformance") (declared-name "MergePerformance") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlPerformances::MergePerformance") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Actions::MessageAction"))) (kind "import") (name "MessageAction") (declared-name "MessageAction") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Import) (visibility "private") (import (reference "Flows::MessageAction") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Actions::MessageTransfer"))) (kind "import") (name "MessageTransfer") (declared-name "MessageTransfer") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::MessageTransfer") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Actions::Natural"))) (kind "import") (name "Natural") (declared-name "Natural") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Natural") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Actions::NonStateTransitionPerformance"))) (kind "import") (name "NonStateTransitionPerformance") (declared-name "NonStateTransitionPerformance") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Import) (visibility "private") (import (reference "TransitionPerformances::NonStateTransitionPerformance") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Actions::Occurrence"))) (kind "import") (name "Occurrence") (declared-name "Occurrence") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::Occurrence") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Actions::Performance"))) (kind "import") (name "Performance") (declared-name "Performance") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Import) (visibility "private") (import (reference "Performances::Performance") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Actions::Positive"))) (kind "import") (name "Positive") (declared-name "Positive") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Positive") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Actions::SendAction"))) (kind "action def") (name "SendAction") (declared-name "SendAction") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Action")) (specializes (reference "SendPerformance")) (specializes (reference "Action")) (specializes (reference "SendPerformance")) (specializes (reference "Action")) (specializes (reference "SendPerformance")))))
    (element (id (node (document "d0") (qualified-name "Actions::SendAction::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::SendAction"))))
    (element (id (node (document "d0") (qualified-name "Actions::SendAction::payload"))) (kind "in out parameter") (name "payload") (declared-name "payload") (parent (node (document "d0") (qualified-name "Actions::SendAction"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Actions::SendAction::sentMessage"))) (kind "ref") (name "sentMessage") (declared-name "sentMessage") (parent (node (document "d0") (qualified-name "Actions::SendAction"))) (authored (membership (kind Feature)) (relationships (typing (reference "MessageTransfer")) (typing (reference "MessageAction")) (redefinition (reference "sentTransfer")))))
    (element (id (node (document "d0") (qualified-name "Actions::SendAction::sentMessage::MessageTransfer::payload, MessageAction::payload"))) (kind "in out parameter") (name "MessageTransfer::payload, MessageAction::payload") (declared-name "MessageTransfer::payload, MessageAction::payload") (parent (node (document "d0") (qualified-name "Actions::SendAction::sentMessage"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Actions::SendPerformance"))) (kind "import") (name "SendPerformance") (declared-name "SendPerformance") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::SendPerformance") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Actions::TerminateAction"))) (kind "action def") (name "TerminateAction") (declared-name "TerminateAction") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Action")) (specializes (reference "Action")) (specializes (reference "Action")) (perform (reference "Actions::TerminateAction::terminateOccurrence")))))
    (element (id (node (document "d0") (qualified-name "Actions::TerminateAction::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::TerminateAction"))))
    (element (id (node (document "d0") (qualified-name "Actions::TerminateAction::occurrence"))) (kind "in out parameter") (name "occurrence") (declared-name "occurrence") (parent (node (document "d0") (qualified-name "Actions::TerminateAction"))) (authored (relationships (typing (reference "occurrence terminatedOccurrence[1]")))))
    (element (id (node (document "d0") (qualified-name "Actions::TerminateAction::terminateOccurrence"))) (kind "action") (name "terminateOccurrence") (declared-name "terminateOccurrence") (parent (node (document "d0") (qualified-name "Actions::TerminateAction"))) (authored (membership (kind Feature)) (relationships (typing (reference "destroy")))))
    (element (id (node (document "d0") (qualified-name "Actions::TerminateAction::terminateOccurrence::occ"))) (kind "in out parameter") (name "occ") (declared-name "occ") (parent (node (document "d0") (qualified-name "Actions::TerminateAction::terminateOccurrence"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Actions::TransitionAction"))) (kind "action def") (name "TransitionAction") (declared-name "TransitionAction") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Action")) (specializes (reference "TransitionPerformance")) (specializes (reference "Action")) (specializes (reference "TransitionPerformance")) (specializes (reference "Action")) (specializes (reference "TransitionPerformance")) (perform (reference "Actions::TransitionAction::accepter")) (perform (reference "Actions::TransitionAction::effect")))))
    (element (id (node (document "d0") (qualified-name "Actions::TransitionAction::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::TransitionAction"))))
    (element (id (node (document "d0") (qualified-name "Actions::TransitionAction::acceptedMessage"))) (kind "ref") (name "acceptedMessage") (declared-name "acceptedMessage") (parent (node (document "d0") (qualified-name "Actions::TransitionAction"))) (authored (membership (kind Feature)) (relationships (typing (reference "MessageTransfer")) (typing (reference "MessageAction")))))
    (element (id (node (document "d0") (qualified-name "Actions::TransitionAction::acceptedMessage::MessageTransfer::payload, MessageAction::payload"))) (kind "in out parameter") (name "MessageTransfer::payload, MessageAction::payload") (declared-name "MessageTransfer::payload, MessageAction::payload") (parent (node (document "d0") (qualified-name "Actions::TransitionAction::acceptedMessage"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Actions::TransitionAction::accepter"))) (kind "action") (name "accepter") (declared-name "accepter") (parent (node (document "d0") (qualified-name "Actions::TransitionAction"))) (authored (membership (kind Feature)) (relationships (typing (reference "AcceptMessageAction")) (redefinition (reference "accept")))))
    (element (id (node (document "d0") (qualified-name "Actions::TransitionAction::effect"))) (kind "action") (name "effect") (declared-name "effect") (parent (node (document "d0") (qualified-name "Actions::TransitionAction"))) (authored (membership (kind Feature)) (relationships (typing (reference "Action")) (redefinition (reference "TransitionPerformance::effect")))))
    (element (id (node (document "d0") (qualified-name "Actions::TransitionAction::receiver"))) (kind "ref") (name "receiver") (declared-name "receiver") (parent (node (document "d0") (qualified-name "Actions::TransitionAction"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "triggerTarget")))))
    (element (id (node (document "d0") (qualified-name "Actions::TransitionAction::transitionLinkSource"))) (kind "in out parameter") (name "transitionLinkSource") (declared-name "transitionLinkSource") (parent (node (document "d0") (qualified-name "Actions::TransitionAction"))) (authored (relationships (typing (reference "transitionLinkSource : Action :>> TransitionPerformance::transitionLinkSource")))))
    (element (id (node (document "d0") (qualified-name "Actions::TransitionPerformance"))) (kind "import") (name "TransitionPerformance") (declared-name "TransitionPerformance") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Import) (visibility "private") (import (reference "TransitionPerformances::TransitionPerformance") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Actions::WhileLoopAction"))) (kind "action def") (name "WhileLoopAction") (declared-name "WhileLoopAction") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "LoopAction")) (specializes (reference "LoopPerformance")) (specializes (reference "LoopAction")) (specializes (reference "LoopPerformance")) (specializes (reference "LoopAction")) (specializes (reference "LoopPerformance")))))
    (element (id (node (document "d0") (qualified-name "Actions::WhileLoopAction::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::WhileLoopAction"))))
    (element (id (node (document "d0") (qualified-name "Actions::WhileLoopAction::body"))) (kind "in out parameter") (name "body") (declared-name "body") (parent (node (document "d0") (qualified-name "Actions::WhileLoopAction"))) (authored (relationships (typing (reference "action")))))
    (element (id (node (document "d0") (qualified-name "Actions::WhileLoopAction::untilTest"))) (kind "in out parameter") (name "untilTest") (declared-name "untilTest") (parent (node (document "d0") (qualified-name "Actions::WhileLoopAction"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Actions::WhileLoopAction::whileTest"))) (kind "in out parameter") (name "whileTest") (declared-name "whileTest") (parent (node (document "d0") (qualified-name "Actions::WhileLoopAction"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Actions::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions"))))
    (element (id (node (document "d0") (qualified-name "Actions::acceptActions"))) (kind "action") (name "acceptActions") (declared-name "acceptActions") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Feature)) (relationships (typing (reference "AcceptAction")) (subsetting (reference "actions")) (subsetting (reference "acceptPerformances")))))
    (element (id (node (document "d0") (qualified-name "Actions::acceptActions::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::acceptActions"))))
    (element (id (node (document "d0") (qualified-name "Actions::acceptPerformances"))) (kind "import") (name "acceptPerformances") (declared-name "acceptPerformances") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::acceptPerformances") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Actions::actions"))) (kind "action") (name "actions") (declared-name "actions") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Feature)) (relationships (typing (reference "Action")) (subsetting (reference "performances")))))
    (element (id (node (document "d0") (qualified-name "Actions::actions::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::actions"))))
    (element (id (node (document "d0") (qualified-name "Actions::assignmentActions"))) (kind "action") (name "assignmentActions") (declared-name "assignmentActions") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Feature)) (relationships (typing (reference "AssignmentAction")) (subsetting (reference "actions")))))
    (element (id (node (document "d0") (qualified-name "Actions::assignmentActions::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::assignmentActions"))))
    (element (id (node (document "d0") (qualified-name "Actions::assignmentActions::target"))) (kind "in out parameter") (name "target") (declared-name "target") (parent (node (document "d0") (qualified-name "Actions::assignmentActions"))) (authored (relationships (typing (reference "target : Occurrence[1] default that as Occurrence")))))
    (element (id (node (document "d0") (qualified-name "Actions::destroy"))) (kind "import") (name "destroy") (declared-name "destroy") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Import) (visibility "private") (import (reference "OccurrenceFunctions::destroy") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Actions::forLoopActions"))) (kind "action") (name "forLoopActions") (declared-name "forLoopActions") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Feature)) (relationships (typing (reference "ForLoopAction")) (subsetting (reference "loopActions")))))
    (element (id (node (document "d0") (qualified-name "Actions::forLoopActions::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::forLoopActions"))))
    (element (id (node (document "d0") (qualified-name "Actions::ifThenActions"))) (kind "action") (name "ifThenActions") (declared-name "ifThenActions") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Feature)) (relationships (typing (reference "IfThenAction")) (subsetting (reference "actions")))))
    (element (id (node (document "d0") (qualified-name "Actions::ifThenActions::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::ifThenActions"))))
    (element (id (node (document "d0") (qualified-name "Actions::ifThenElseActions"))) (kind "action") (name "ifThenElseActions") (declared-name "ifThenElseActions") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Feature)) (relationships (typing (reference "IfThenElseAction")) (subsetting (reference "actions")))))
    (element (id (node (document "d0") (qualified-name "Actions::ifThenElseActions::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::ifThenElseActions"))))
    (element (id (node (document "d0") (qualified-name "Actions::isEmpty"))) (kind "import") (name "isEmpty") (declared-name "isEmpty") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::isEmpty") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Actions::loopActions"))) (kind "action") (name "loopActions") (declared-name "loopActions") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Feature)) (relationships (typing (reference "LoopAction")) (subsetting (reference "actions")))))
    (element (id (node (document "d0") (qualified-name "Actions::loopActions::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::loopActions"))))
    (element (id (node (document "d0") (qualified-name "Actions::performances"))) (kind "import") (name "performances") (declared-name "performances") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Import) (visibility "private") (import (reference "Performances::performances") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Actions::sendActions"))) (kind "action") (name "sendActions") (declared-name "sendActions") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Feature)) (relationships (typing (reference "SendAction")) (subsetting (reference "actions")) (subsetting (reference "sendPerformances")))))
    (element (id (node (document "d0") (qualified-name "Actions::sendActions::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::sendActions"))))
    (element (id (node (document "d0") (qualified-name "Actions::sendPerformances"))) (kind "import") (name "sendPerformances") (declared-name "sendPerformances") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::sendPerformances") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Actions::size"))) (kind "import") (name "size") (declared-name "size") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::size") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Actions::terminateActions"))) (kind "action") (name "terminateActions") (declared-name "terminateActions") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Feature)) (relationships (typing (reference "TerminateAction")) (subsetting (reference "actions")))))
    (element (id (node (document "d0") (qualified-name "Actions::terminateActions::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::terminateActions"))))
    (element (id (node (document "d0") (qualified-name "Actions::terminateActions::occurrence"))) (kind "in out parameter") (name "occurrence") (declared-name "occurrence") (parent (node (document "d0") (qualified-name "Actions::terminateActions"))) (authored (relationships (typing (reference "occurrence terminatedOccurrence default that as Occurrence")))))
    (element (id (node (document "d0") (qualified-name "Actions::transitionActions"))) (kind "action") (name "transitionActions") (declared-name "transitionActions") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Feature)) (relationships (typing (reference "TransitionAction")) (subsetting (reference "actions")))))
    (element (id (node (document "d0") (qualified-name "Actions::transitionActions::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::transitionActions"))))
    (element (id (node (document "d0") (qualified-name "Actions::whileLoopActions"))) (kind "action") (name "whileLoopActions") (declared-name "whileLoopActions") (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Feature)) (relationships (typing (reference "WhileLoopAction")) (subsetting (reference "loopActions")))))
    (element (id (node (document "d0") (qualified-name "Actions::whileLoopActions::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Actions::whileLoopActions"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Actions::AcceptAction"))) (kind specialization) (ordinal 0)) (authored-target "AcceptMessageAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::AcceptMessageAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AcceptAction"))) (kind specialization) (ordinal 1)) (authored-target "AcceptMessageAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::AcceptMessageAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AcceptAction"))) (kind specialization) (ordinal 2)) (authored-target "AcceptMessageAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::AcceptMessageAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AcceptAction"))) (kind bindSource) (ordinal 0)) (authored-target "payload") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::AcceptMessageAction::payload")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AcceptAction"))) (kind bindTarget) (ordinal 0)) (authored-target "aState::aTransition::apayload") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AcceptAction::"))) (kind redefinition) (ordinal 0)) (authored-target "acceptedMessage") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::AcceptMessageAction::acceptedMessage")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AcceptMessageAction"))) (kind specialization) (ordinal 0)) (authored-target "Action") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AcceptMessageAction"))) (kind specialization) (ordinal 1)) (authored-target "AcceptPerformance") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::AcceptPerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AcceptMessageAction"))) (kind specialization) (ordinal 2)) (authored-target "Action") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AcceptMessageAction"))) (kind specialization) (ordinal 3)) (authored-target "AcceptPerformance") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::AcceptPerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AcceptMessageAction"))) (kind specialization) (ordinal 4)) (authored-target "Action") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AcceptMessageAction"))) (kind specialization) (ordinal 5)) (authored-target "AcceptPerformance") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::AcceptPerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AcceptMessageAction::acceptedMessage"))) (kind featureTyping) (ordinal 0)) (authored-target "MessageTransfer") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::MessageTransfer")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AcceptMessageAction::acceptedMessage"))) (kind featureTyping) (ordinal 1)) (authored-target "MessageAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::MessageAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AcceptMessageAction::acceptedMessage"))) (kind redefinition) (ordinal 0)) (authored-target "acceptedTransfer") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AcceptMessageAction::acceptedMessage::MessageTransfer::payload, MessageAction::payload"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::AcceptMessageAction::_documentation")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AcceptMessageAction::payload"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::AcceptMessageAction::_documentation")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AcceptPerformance"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::AcceptPerformance") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action"))) (kind specialization) (ordinal 0)) (authored-target "Performance") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Performance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action"))) (kind specialization) (ordinal 1)) (authored-target "Performance") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Performance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action"))) (kind specialization) (ordinal 2)) (authored-target "Performance") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Performance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 0)) (authored-target "Actions::Action::start") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::start")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 1)) (authored-target "Actions::Action::done") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::done")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 2)) (authored-target "Actions::Action::subactions") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::subactions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 3)) (authored-target "Actions::Action::sendSubactions") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::sendSubactions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 4)) (authored-target "Actions::Action::acceptSubactions") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::acceptSubactions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 5)) (authored-target "Actions::Action::terminateSubactions") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::terminateSubactions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 6)) (authored-target "Actions::Action::controls") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::controls")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 7)) (authored-target "Actions::Action::merges") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::merges")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 8)) (authored-target "Actions::Action::decisions") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::decisions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 9)) (authored-target "Actions::Action::joins") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::joins")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 10)) (authored-target "Actions::Action::forks") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::forks")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 11)) (authored-target "Actions::Action::transitions") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::transitions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 12)) (authored-target "Actions::Action::decisionTransitions") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::decisionTransitions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 13)) (authored-target "Actions::Action::assignments") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::assignments")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 14)) (authored-target "Actions::Action::ifSubactions") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::ifSubactions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 15)) (authored-target "Actions::Action::loops") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::loops")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 16)) (authored-target "Actions::Action::whileLoops") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::whileLoops")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 17)) (authored-target "Actions::Action::forLoops") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::forLoops")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::acceptSubactions"))) (kind featureTyping) (ordinal 0)) (authored-target "AcceptAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::AcceptAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::acceptSubactions"))) (kind subsetting) (ordinal 0)) (authored-target "subactions") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::subactions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::acceptSubactions"))) (kind subsetting) (ordinal 1)) (authored-target "acceptActions") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::acceptActions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::assignments"))) (kind featureTyping) (ordinal 0)) (authored-target "AssignmentAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::AssignmentAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::assignments"))) (kind subsetting) (ordinal 0)) (authored-target "subactions") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::subactions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::assignments"))) (kind subsetting) (ordinal 1)) (authored-target "assignmentActions") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::assignmentActions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::assignments::target"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::assignments::_documentation")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::controls"))) (kind featureTyping) (ordinal 0)) (authored-target "ControlAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::ControlAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::controls"))) (kind subsetting) (ordinal 0)) (authored-target "subactions") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::subactions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::decisionTransitions"))) (kind featureTyping) (ordinal 0)) (authored-target "DecisionTransitionAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::DecisionTransitionAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::decisionTransitions"))) (kind subsetting) (ordinal 0)) (authored-target "transitions") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::transitions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::decisions"))) (kind featureTyping) (ordinal 0)) (authored-target "DecisionAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::DecisionAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::decisions"))) (kind subsetting) (ordinal 0)) (authored-target "controls") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::controls")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::done"))) (kind featureTyping) (ordinal 0)) (authored-target "Action") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::done"))) (kind redefinition) (ordinal 0)) (authored-target "endShot") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::forLoops"))) (kind featureTyping) (ordinal 0)) (authored-target "ForLoopAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::ForLoopAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::forLoops"))) (kind subsetting) (ordinal 0)) (authored-target "loops") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::loops")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::forLoops"))) (kind subsetting) (ordinal 1)) (authored-target "forLoopActions") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::forLoopActions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::forks"))) (kind featureTyping) (ordinal 0)) (authored-target "ForkAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::ForkAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::forks"))) (kind subsetting) (ordinal 0)) (authored-target "controls") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::controls")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::ifSubactions"))) (kind featureTyping) (ordinal 0)) (authored-target "IfThenAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::IfThenAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::ifSubactions"))) (kind subsetting) (ordinal 0)) (authored-target "subactions") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::subactions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::ifSubactions"))) (kind subsetting) (ordinal 1)) (authored-target "ifThenActions") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::ifThenActions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::incomingTransfers"))) (kind redefinition) (ordinal 0)) (authored-target "Performance::incomingTransfers") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::joins"))) (kind featureTyping) (ordinal 0)) (authored-target "JoinAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::JoinAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::joins"))) (kind subsetting) (ordinal 0)) (authored-target "controls") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::controls")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::loops"))) (kind featureTyping) (ordinal 0)) (authored-target "LoopAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::LoopAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::loops"))) (kind subsetting) (ordinal 0)) (authored-target "subactions") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::subactions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::loops"))) (kind subsetting) (ordinal 1)) (authored-target "loopActions") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::loopActions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::merges"))) (kind featureTyping) (ordinal 0)) (authored-target "MergeAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::MergeAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::merges"))) (kind subsetting) (ordinal 0)) (authored-target "controls") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::controls")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::self"))) (kind featureTyping) (ordinal 0)) (authored-target "Action") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::sendSubactions"))) (kind featureTyping) (ordinal 0)) (authored-target "SendAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::SendAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::sendSubactions"))) (kind subsetting) (ordinal 0)) (authored-target "subactions") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::subactions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::sendSubactions"))) (kind subsetting) (ordinal 1)) (authored-target "sendActions") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::sendActions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::start"))) (kind featureTyping) (ordinal 0)) (authored-target "Action") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::start"))) (kind redefinition) (ordinal 0)) (authored-target "startShot") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::subactions"))) (kind featureTyping) (ordinal 0)) (authored-target "Action") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::subactions"))) (kind subsetting) (ordinal 0)) (authored-target "actions") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::actions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::subactions"))) (kind subsetting) (ordinal 1)) (authored-target "subperformances") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::subactions::occurrence"))) (kind redefinition) (ordinal 0)) (authored-target "Action::this") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::terminateSubactions"))) (kind featureTyping) (ordinal 0)) (authored-target "TerminateAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::TerminateAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::terminateSubactions"))) (kind subsetting) (ordinal 0)) (authored-target "subactions") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::subactions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::terminateSubactions"))) (kind subsetting) (ordinal 1)) (authored-target "terminateActions") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::terminateActions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::transitions"))) (kind featureTyping) (ordinal 0)) (authored-target "TransitionAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::TransitionAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::transitions"))) (kind subsetting) (ordinal 0)) (authored-target "subactions") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::subactions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::transitions"))) (kind subsetting) (ordinal 1)) (authored-target "transitionActions") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::transitionActions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::whileLoops"))) (kind featureTyping) (ordinal 0)) (authored-target "WhileLoopAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::WhileLoopAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::whileLoops"))) (kind subsetting) (ordinal 0)) (authored-target "loops") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::loops")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::whileLoops"))) (kind subsetting) (ordinal 1)) (authored-target "whileLoopActions") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::whileLoopActions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Anything"))) (kind membershipImport) (ordinal 0)) (authored-target "Base::Anything") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AssignmentAction"))) (kind specialization) (ordinal 0)) (authored-target "FeatureWritePerformance") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::FeatureWritePerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AssignmentAction"))) (kind specialization) (ordinal 1)) (authored-target "Action") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AssignmentAction"))) (kind specialization) (ordinal 2)) (authored-target "FeatureWritePerformance") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::FeatureWritePerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AssignmentAction"))) (kind specialization) (ordinal 3)) (authored-target "Action") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AssignmentAction"))) (kind specialization) (ordinal 4)) (authored-target "FeatureWritePerformance") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::FeatureWritePerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AssignmentAction"))) (kind specialization) (ordinal 5)) (authored-target "Action") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AssignmentAction::replacementValues"))) (kind featureTyping) (ordinal 0)) (authored-target "replacementValues : Anything[0..*] nonunique") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AssignmentAction::target"))) (kind featureTyping) (ordinal 0)) (authored-target "target : Occurrence[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::ControlAction"))) (kind specialization) (ordinal 0)) (authored-target "Action") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::ControlAction"))) (kind specialization) (ordinal 1)) (authored-target "Action") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::ControlAction"))) (kind specialization) (ordinal 2)) (authored-target "Action") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::ControlAction"))) (kind bindSource) (ordinal 0)) (authored-target "start") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::start")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::ControlAction"))) (kind bindTarget) (ordinal 0)) (authored-target "done") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::done")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::DecisionAction"))) (kind specialization) (ordinal 0)) (authored-target "ControlAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::ControlAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::DecisionAction"))) (kind specialization) (ordinal 1)) (authored-target "DecisionPerformance") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::DecisionPerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::DecisionAction"))) (kind specialization) (ordinal 2)) (authored-target "ControlAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::ControlAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::DecisionAction"))) (kind specialization) (ordinal 3)) (authored-target "DecisionPerformance") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::DecisionPerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::DecisionAction"))) (kind specialization) (ordinal 4)) (authored-target "ControlAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::ControlAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::DecisionAction"))) (kind specialization) (ordinal 5)) (authored-target "DecisionPerformance") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::DecisionPerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::DecisionPerformance"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlPerformances::DecisionPerformance") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::DecisionTransitionAction"))) (kind specialization) (ordinal 0)) (authored-target "TransitionAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::TransitionAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::DecisionTransitionAction"))) (kind specialization) (ordinal 1)) (authored-target "NonStateTransitionPerformance") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::NonStateTransitionPerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::DecisionTransitionAction"))) (kind specialization) (ordinal 2)) (authored-target "TransitionAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::TransitionAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::DecisionTransitionAction"))) (kind specialization) (ordinal 3)) (authored-target "NonStateTransitionPerformance") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::NonStateTransitionPerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::DecisionTransitionAction"))) (kind specialization) (ordinal 4)) (authored-target "TransitionAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::TransitionAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::DecisionTransitionAction"))) (kind specialization) (ordinal 5)) (authored-target "NonStateTransitionPerformance") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::NonStateTransitionPerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::DecisionTransitionAction::"))) (kind redefinition) (ordinal 0)) (authored-target "accepter") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::TransitionAction::accepter")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::DecisionTransitionAction::#ref"))) (kind redefinition) (ordinal 0)) (authored-target "effect") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::TransitionAction::effect")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::FeatureWritePerformance"))) (kind membershipImport) (ordinal 0)) (authored-target "FeatureReferencingPerformances::FeatureWritePerformance") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::ForLoopAction"))) (kind specialization) (ordinal 0)) (authored-target "LoopAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::LoopAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::ForLoopAction"))) (kind specialization) (ordinal 1)) (authored-target "LoopAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::LoopAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::ForLoopAction"))) (kind specialization) (ordinal 2)) (authored-target "LoopAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::LoopAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::ForLoopAction"))) (kind performSource) (ordinal 0)) (authored-target "Actions::ForLoopAction::initialization") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::ForLoopAction::initialization")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::ForLoopAction"))) (kind performSource) (ordinal 1)) (authored-target "Actions::ForLoopAction::whileLoop") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::ForLoopAction::whileLoop")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::ForLoopAction::_while"))) (kind flowSource) (ordinal 0)) (authored-target "Actions::ForLoopAction::_while::body") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::ForLoopAction::body"))) (kind featureTyping) (ordinal 0)) (authored-target "action") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::LoopAction::action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::ForLoopAction::ref"))) (kind featureTyping) (ordinal 0)) (authored-target "ref seq") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::ForLoopAction::var"))) (kind featureTyping) (ordinal 0)) (authored-target "seq") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::ForLoopAction::whileLoop"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::ForLoopAction::_documentation")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::ForkAction"))) (kind specialization) (ordinal 0)) (authored-target "ControlAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::ControlAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::ForkAction"))) (kind specialization) (ordinal 1)) (authored-target "ControlAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::ControlAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::ForkAction"))) (kind specialization) (ordinal 2)) (authored-target "ControlAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::ControlAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::HappensWhile"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::HappensWhile") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::IfThenAction"))) (kind specialization) (ordinal 0)) (authored-target "Action") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::IfThenAction"))) (kind specialization) (ordinal 1)) (authored-target "IfThenPerformance") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::IfThenPerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::IfThenAction"))) (kind specialization) (ordinal 2)) (authored-target "Action") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::IfThenAction"))) (kind specialization) (ordinal 3)) (authored-target "IfThenPerformance") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::IfThenPerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::IfThenAction"))) (kind specialization) (ordinal 4)) (authored-target "Action") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::IfThenAction"))) (kind specialization) (ordinal 5)) (authored-target "IfThenPerformance") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::IfThenPerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::IfThenAction::action"))) (kind featureTyping) (ordinal 0)) (authored-target "action thenClause[0..1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::IfThenAction::ifTest"))) (kind featureTyping) (ordinal 0)) (authored-target "ifTest[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::IfThenElseAction"))) (kind specialization) (ordinal 0)) (authored-target "IfThenAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::IfThenAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::IfThenElseAction"))) (kind specialization) (ordinal 1)) (authored-target "IfThenElsePerformance") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::IfThenElsePerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::IfThenElseAction"))) (kind specialization) (ordinal 2)) (authored-target "IfThenAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::IfThenAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::IfThenElseAction"))) (kind specialization) (ordinal 3)) (authored-target "IfThenElsePerformance") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::IfThenElsePerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::IfThenElseAction"))) (kind specialization) (ordinal 4)) (authored-target "IfThenAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::IfThenAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::IfThenElseAction"))) (kind specialization) (ordinal 5)) (authored-target "IfThenElsePerformance") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::IfThenElsePerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::IfThenElseAction::action"))) (kind featureTyping) (ordinal 0)) (authored-target "action thenClause[0..1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::IfThenElseAction::action#in_out_parameter"))) (kind featureTyping) (ordinal 0)) (authored-target "action elseClause[0..1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::IfThenElseAction::ifTest"))) (kind featureTyping) (ordinal 0)) (authored-target "ifTest[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::IfThenElsePerformance"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlPerformances::IfThenElsePerformance") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::IfThenPerformance"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlPerformances::IfThenPerformance") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::JoinAction"))) (kind specialization) (ordinal 0)) (authored-target "ControlAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::ControlAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::JoinAction"))) (kind specialization) (ordinal 1)) (authored-target "ControlAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::ControlAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::JoinAction"))) (kind specialization) (ordinal 2)) (authored-target "ControlAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::ControlAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::LoopAction"))) (kind specialization) (ordinal 0)) (authored-target "Action") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::LoopAction"))) (kind specialization) (ordinal 1)) (authored-target "Action") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::LoopAction"))) (kind specialization) (ordinal 2)) (authored-target "Action") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::LoopAction::action"))) (kind featureTyping) (ordinal 0)) (authored-target "action body[0..*]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::LoopAction::ref"))) (kind featureTyping) (ordinal 0)) (authored-target "ref iterator") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::LoopPerformance"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlPerformances::LoopPerformance") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::MergeAction"))) (kind specialization) (ordinal 0)) (authored-target "ControlAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::ControlAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::MergeAction"))) (kind specialization) (ordinal 1)) (authored-target "MergePerformance") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::MergePerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::MergeAction"))) (kind specialization) (ordinal 2)) (authored-target "ControlAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::ControlAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::MergeAction"))) (kind specialization) (ordinal 3)) (authored-target "MergePerformance") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::MergePerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::MergeAction"))) (kind specialization) (ordinal 4)) (authored-target "ControlAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::ControlAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::MergeAction"))) (kind specialization) (ordinal 5)) (authored-target "MergePerformance") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::MergePerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::MergePerformance"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlPerformances::MergePerformance") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::MessageAction"))) (kind membershipImport) (ordinal 0)) (authored-target "Flows::MessageAction") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::MessageTransfer"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::MessageTransfer") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Natural"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Natural") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::NonStateTransitionPerformance"))) (kind membershipImport) (ordinal 0)) (authored-target "TransitionPerformances::NonStateTransitionPerformance") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Occurrence"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::Occurrence") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Performance"))) (kind membershipImport) (ordinal 0)) (authored-target "Performances::Performance") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Positive"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Positive") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::SendAction"))) (kind specialization) (ordinal 0)) (authored-target "Action") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::SendAction"))) (kind specialization) (ordinal 1)) (authored-target "SendPerformance") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::SendPerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::SendAction"))) (kind specialization) (ordinal 2)) (authored-target "Action") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::SendAction"))) (kind specialization) (ordinal 3)) (authored-target "SendPerformance") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::SendPerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::SendAction"))) (kind specialization) (ordinal 4)) (authored-target "Action") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::SendAction"))) (kind specialization) (ordinal 5)) (authored-target "SendPerformance") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::SendPerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::SendAction::payload"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::SendAction::_documentation")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::SendAction::sentMessage"))) (kind featureTyping) (ordinal 0)) (authored-target "MessageTransfer") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::MessageTransfer")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::SendAction::sentMessage"))) (kind featureTyping) (ordinal 1)) (authored-target "MessageAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::MessageAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::SendAction::sentMessage"))) (kind redefinition) (ordinal 0)) (authored-target "sentTransfer") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::SendAction::sentMessage::MessageTransfer::payload, MessageAction::payload"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::SendAction::_documentation")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::SendPerformance"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::SendPerformance") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TerminateAction"))) (kind specialization) (ordinal 0)) (authored-target "Action") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TerminateAction"))) (kind specialization) (ordinal 1)) (authored-target "Action") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TerminateAction"))) (kind specialization) (ordinal 2)) (authored-target "Action") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TerminateAction"))) (kind performSource) (ordinal 0)) (authored-target "Actions::TerminateAction::terminateOccurrence") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::TerminateAction::terminateOccurrence")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TerminateAction::occurrence"))) (kind featureTyping) (ordinal 0)) (authored-target "occurrence terminatedOccurrence[1]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TerminateAction::terminateOccurrence"))) (kind featureTyping) (ordinal 0)) (authored-target "destroy") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::destroy")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TerminateAction::terminateOccurrence::occ"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::TerminateAction::_documentation")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TransitionAction"))) (kind specialization) (ordinal 0)) (authored-target "Action") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TransitionAction"))) (kind specialization) (ordinal 1)) (authored-target "TransitionPerformance") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::TransitionPerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TransitionAction"))) (kind specialization) (ordinal 2)) (authored-target "Action") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TransitionAction"))) (kind specialization) (ordinal 3)) (authored-target "TransitionPerformance") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::TransitionPerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TransitionAction"))) (kind specialization) (ordinal 4)) (authored-target "Action") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TransitionAction"))) (kind specialization) (ordinal 5)) (authored-target "TransitionPerformance") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::TransitionPerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TransitionAction"))) (kind bindSource) (ordinal 0)) (authored-target "receiver") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::TransitionAction::receiver")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TransitionAction"))) (kind bindSource) (ordinal 1)) (authored-target "acceptedMessage") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::TransitionAction::acceptedMessage")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TransitionAction"))) (kind bindTarget) (ordinal 0)) (authored-target "accepter::receiver") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TransitionAction"))) (kind bindTarget) (ordinal 1)) (authored-target "accepter::acceptedMessage") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TransitionAction"))) (kind performSource) (ordinal 0)) (authored-target "Actions::TransitionAction::accepter") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::TransitionAction::accepter")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TransitionAction"))) (kind performSource) (ordinal 1)) (authored-target "Actions::TransitionAction::effect") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::TransitionAction::effect")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TransitionAction::acceptedMessage"))) (kind featureTyping) (ordinal 0)) (authored-target "MessageTransfer") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::MessageTransfer")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TransitionAction::acceptedMessage"))) (kind featureTyping) (ordinal 1)) (authored-target "MessageAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::MessageAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TransitionAction::acceptedMessage::MessageTransfer::payload, MessageAction::payload"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::TransitionAction::_documentation")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TransitionAction::accepter"))) (kind featureTyping) (ordinal 0)) (authored-target "AcceptMessageAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::AcceptMessageAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TransitionAction::accepter"))) (kind redefinition) (ordinal 0)) (authored-target "accept") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TransitionAction::effect"))) (kind featureTyping) (ordinal 0)) (authored-target "Action") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TransitionAction::effect"))) (kind redefinition) (ordinal 0)) (authored-target "TransitionPerformance::effect") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TransitionAction::receiver"))) (kind redefinition) (ordinal 0)) (authored-target "triggerTarget") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TransitionAction::transitionLinkSource"))) (kind featureTyping) (ordinal 0)) (authored-target "transitionLinkSource : Action :>> TransitionPerformance::transitionLinkSource") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TransitionPerformance"))) (kind membershipImport) (ordinal 0)) (authored-target "TransitionPerformances::TransitionPerformance") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::WhileLoopAction"))) (kind specialization) (ordinal 0)) (authored-target "LoopAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::LoopAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::WhileLoopAction"))) (kind specialization) (ordinal 1)) (authored-target "LoopPerformance") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::LoopPerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::WhileLoopAction"))) (kind specialization) (ordinal 2)) (authored-target "LoopAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::LoopAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::WhileLoopAction"))) (kind specialization) (ordinal 3)) (authored-target "LoopPerformance") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::LoopPerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::WhileLoopAction"))) (kind specialization) (ordinal 4)) (authored-target "LoopAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::LoopAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::WhileLoopAction"))) (kind specialization) (ordinal 5)) (authored-target "LoopPerformance") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::LoopPerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::WhileLoopAction::body"))) (kind featureTyping) (ordinal 0)) (authored-target "action") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::LoopAction::action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::WhileLoopAction::untilTest"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::WhileLoopAction::_documentation")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::WhileLoopAction::whileTest"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::WhileLoopAction::_documentation")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::acceptActions"))) (kind featureTyping) (ordinal 0)) (authored-target "AcceptAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::AcceptAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::acceptActions"))) (kind subsetting) (ordinal 0)) (authored-target "actions") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::actions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::acceptActions"))) (kind subsetting) (ordinal 1)) (authored-target "acceptPerformances") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::acceptPerformances")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::acceptPerformances"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::acceptPerformances") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::actions"))) (kind featureTyping) (ordinal 0)) (authored-target "Action") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::actions"))) (kind subsetting) (ordinal 0)) (authored-target "performances") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::performances")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::assignmentActions"))) (kind featureTyping) (ordinal 0)) (authored-target "AssignmentAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::AssignmentAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::assignmentActions"))) (kind subsetting) (ordinal 0)) (authored-target "actions") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::actions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::assignmentActions::target"))) (kind featureTyping) (ordinal 0)) (authored-target "target : Occurrence[1] default that as Occurrence") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::destroy"))) (kind membershipImport) (ordinal 0)) (authored-target "OccurrenceFunctions::destroy") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::forLoopActions"))) (kind featureTyping) (ordinal 0)) (authored-target "ForLoopAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::ForLoopAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::forLoopActions"))) (kind subsetting) (ordinal 0)) (authored-target "loopActions") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::loopActions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::ifThenActions"))) (kind featureTyping) (ordinal 0)) (authored-target "IfThenAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::IfThenAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::ifThenActions"))) (kind subsetting) (ordinal 0)) (authored-target "actions") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::actions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::ifThenElseActions"))) (kind featureTyping) (ordinal 0)) (authored-target "IfThenElseAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::IfThenElseAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::ifThenElseActions"))) (kind subsetting) (ordinal 0)) (authored-target "actions") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::actions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::isEmpty"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::isEmpty") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::loopActions"))) (kind featureTyping) (ordinal 0)) (authored-target "LoopAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::LoopAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::loopActions"))) (kind subsetting) (ordinal 0)) (authored-target "actions") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::actions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::performances"))) (kind membershipImport) (ordinal 0)) (authored-target "Performances::performances") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::sendActions"))) (kind featureTyping) (ordinal 0)) (authored-target "SendAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::SendAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::sendActions"))) (kind subsetting) (ordinal 0)) (authored-target "actions") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::actions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::sendActions"))) (kind subsetting) (ordinal 1)) (authored-target "sendPerformances") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::sendPerformances")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::sendPerformances"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::sendPerformances") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::size"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::size") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::terminateActions"))) (kind featureTyping) (ordinal 0)) (authored-target "TerminateAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::TerminateAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::terminateActions"))) (kind subsetting) (ordinal 0)) (authored-target "actions") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::actions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::terminateActions::occurrence"))) (kind featureTyping) (ordinal 0)) (authored-target "occurrence terminatedOccurrence default that as Occurrence") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::transitionActions"))) (kind featureTyping) (ordinal 0)) (authored-target "TransitionAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::TransitionAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::transitionActions"))) (kind subsetting) (ordinal 0)) (authored-target "actions") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::actions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::whileLoopActions"))) (kind featureTyping) (ordinal 0)) (authored-target "WhileLoopAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::WhileLoopAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::whileLoopActions"))) (kind subsetting) (ordinal 0)) (authored-target "loopActions") (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::loopActions")))))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::AcceptAction"))) (target (node (document "d0") (qualified-name "Actions::AcceptMessageAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::AcceptAction"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::AcceptAction"))) (target (node (document "d0") (qualified-name "Actions::AcceptMessageAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::AcceptAction"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::AcceptAction"))) (target (node (document "d0") (qualified-name "Actions::AcceptMessageAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::AcceptAction"))) (kind specialization) (ordinal 2)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Actions::AcceptAction::"))) (target (node (document "d0") (qualified-name "Actions::AcceptMessageAction::acceptedMessage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::AcceptAction::"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::AcceptMessageAction"))) (target (node (document "d0") (qualified-name "Actions::AcceptPerformance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::AcceptMessageAction"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::AcceptMessageAction"))) (target (node (document "d0") (qualified-name "Actions::AcceptPerformance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::AcceptMessageAction"))) (kind specialization) (ordinal 3)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::AcceptMessageAction"))) (target (node (document "d0") (qualified-name "Actions::AcceptPerformance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::AcceptMessageAction"))) (kind specialization) (ordinal 5)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::AcceptMessageAction"))) (target (node (document "d0") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::AcceptMessageAction"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::AcceptMessageAction"))) (target (node (document "d0") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::AcceptMessageAction"))) (kind specialization) (ordinal 2)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::AcceptMessageAction"))) (target (node (document "d0") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::AcceptMessageAction"))) (kind specialization) (ordinal 4)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::AcceptMessageAction::acceptedMessage"))) (target (node (document "d0") (qualified-name "Actions::MessageAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::AcceptMessageAction::acceptedMessage"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::AcceptMessageAction::acceptedMessage"))) (target (node (document "d0") (qualified-name "Actions::MessageTransfer"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::AcceptMessageAction::acceptedMessage"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::AcceptMessageAction::acceptedMessage::MessageTransfer::payload, MessageAction::payload"))) (target (node (document "d0") (qualified-name "Actions::AcceptMessageAction::_documentation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::AcceptMessageAction::acceptedMessage::MessageTransfer::payload, MessageAction::payload"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::AcceptMessageAction::payload"))) (target (node (document "d0") (qualified-name "Actions::AcceptMessageAction::_documentation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::AcceptMessageAction::payload"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::Action"))) (target (node (document "d0") (qualified-name "Actions::Performance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::Action"))) (target (node (document "d0") (qualified-name "Actions::Performance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::Action"))) (target (node (document "d0") (qualified-name "Actions::Performance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action"))) (kind specialization) (ordinal 2)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Actions::Action"))) (target (node (document "d0") (qualified-name "Actions::Action::acceptSubactions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 4)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Actions::Action"))) (target (node (document "d0") (qualified-name "Actions::Action::assignments"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 13)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Actions::Action"))) (target (node (document "d0") (qualified-name "Actions::Action::controls"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 6)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Actions::Action"))) (target (node (document "d0") (qualified-name "Actions::Action::decisionTransitions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 12)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Actions::Action"))) (target (node (document "d0") (qualified-name "Actions::Action::decisions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 8)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Actions::Action"))) (target (node (document "d0") (qualified-name "Actions::Action::done"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 1)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Actions::Action"))) (target (node (document "d0") (qualified-name "Actions::Action::forLoops"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 17)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Actions::Action"))) (target (node (document "d0") (qualified-name "Actions::Action::forks"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 10)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Actions::Action"))) (target (node (document "d0") (qualified-name "Actions::Action::ifSubactions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 14)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Actions::Action"))) (target (node (document "d0") (qualified-name "Actions::Action::joins"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 9)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Actions::Action"))) (target (node (document "d0") (qualified-name "Actions::Action::loops"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 15)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Actions::Action"))) (target (node (document "d0") (qualified-name "Actions::Action::merges"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 7)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Actions::Action"))) (target (node (document "d0") (qualified-name "Actions::Action::sendSubactions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 3)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Actions::Action"))) (target (node (document "d0") (qualified-name "Actions::Action::start"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Actions::Action"))) (target (node (document "d0") (qualified-name "Actions::Action::subactions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 2)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Actions::Action"))) (target (node (document "d0") (qualified-name "Actions::Action::terminateSubactions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 5)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Actions::Action"))) (target (node (document "d0") (qualified-name "Actions::Action::transitions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 11)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Actions::Action"))) (target (node (document "d0") (qualified-name "Actions::Action::whileLoops"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 16)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::Action::acceptSubactions"))) (target (node (document "d0") (qualified-name "Actions::AcceptAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action::acceptSubactions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Actions::Action::acceptSubactions"))) (target (node (document "d0") (qualified-name "Actions::Action::subactions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action::acceptSubactions"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Actions::Action::acceptSubactions"))) (target (node (document "d0") (qualified-name "Actions::acceptActions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action::acceptSubactions"))) (kind subsetting) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::Action::assignments"))) (target (node (document "d0") (qualified-name "Actions::AssignmentAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action::assignments"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Actions::Action::assignments"))) (target (node (document "d0") (qualified-name "Actions::Action::subactions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action::assignments"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Actions::Action::assignments"))) (target (node (document "d0") (qualified-name "Actions::assignmentActions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action::assignments"))) (kind subsetting) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::Action::assignments::target"))) (target (node (document "d0") (qualified-name "Actions::Action::assignments::_documentation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action::assignments::target"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::Action::controls"))) (target (node (document "d0") (qualified-name "Actions::ControlAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action::controls"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Actions::Action::controls"))) (target (node (document "d0") (qualified-name "Actions::Action::subactions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action::controls"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::Action::decisionTransitions"))) (target (node (document "d0") (qualified-name "Actions::DecisionTransitionAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action::decisionTransitions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Actions::Action::decisionTransitions"))) (target (node (document "d0") (qualified-name "Actions::Action::transitions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action::decisionTransitions"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::Action::decisions"))) (target (node (document "d0") (qualified-name "Actions::DecisionAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action::decisions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Actions::Action::decisions"))) (target (node (document "d0") (qualified-name "Actions::Action::controls"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action::decisions"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::Action::done"))) (target (node (document "d0") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action::done"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::Action::forLoops"))) (target (node (document "d0") (qualified-name "Actions::ForLoopAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action::forLoops"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Actions::Action::forLoops"))) (target (node (document "d0") (qualified-name "Actions::Action::loops"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action::forLoops"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Actions::Action::forLoops"))) (target (node (document "d0") (qualified-name "Actions::forLoopActions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action::forLoops"))) (kind subsetting) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::Action::forks"))) (target (node (document "d0") (qualified-name "Actions::ForkAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action::forks"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Actions::Action::forks"))) (target (node (document "d0") (qualified-name "Actions::Action::controls"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action::forks"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::Action::ifSubactions"))) (target (node (document "d0") (qualified-name "Actions::IfThenAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action::ifSubactions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Actions::Action::ifSubactions"))) (target (node (document "d0") (qualified-name "Actions::Action::subactions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action::ifSubactions"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Actions::Action::ifSubactions"))) (target (node (document "d0") (qualified-name "Actions::ifThenActions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action::ifSubactions"))) (kind subsetting) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::Action::joins"))) (target (node (document "d0") (qualified-name "Actions::JoinAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action::joins"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Actions::Action::joins"))) (target (node (document "d0") (qualified-name "Actions::Action::controls"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action::joins"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::Action::loops"))) (target (node (document "d0") (qualified-name "Actions::LoopAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action::loops"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Actions::Action::loops"))) (target (node (document "d0") (qualified-name "Actions::Action::subactions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action::loops"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Actions::Action::loops"))) (target (node (document "d0") (qualified-name "Actions::loopActions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action::loops"))) (kind subsetting) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::Action::merges"))) (target (node (document "d0") (qualified-name "Actions::MergeAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action::merges"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Actions::Action::merges"))) (target (node (document "d0") (qualified-name "Actions::Action::controls"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action::merges"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::Action::self"))) (target (node (document "d0") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action::self"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::Action::sendSubactions"))) (target (node (document "d0") (qualified-name "Actions::SendAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action::sendSubactions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Actions::Action::sendSubactions"))) (target (node (document "d0") (qualified-name "Actions::Action::subactions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action::sendSubactions"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Actions::Action::sendSubactions"))) (target (node (document "d0") (qualified-name "Actions::sendActions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action::sendSubactions"))) (kind subsetting) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::Action::start"))) (target (node (document "d0") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action::start"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind bind) (source (node (document "d0") (qualified-name "Actions::Action::start"))) (target (node (document "d0") (qualified-name "Actions::Action::done"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::ControlAction"))) (kind bindSource) (ordinal 0)) (expression (kind bind) (source "start") (target "done")))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::Action::subactions"))) (target (node (document "d0") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action::subactions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Actions::Action::subactions"))) (target (node (document "d0") (qualified-name "Actions::actions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action::subactions"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::Action::terminateSubactions"))) (target (node (document "d0") (qualified-name "Actions::TerminateAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action::terminateSubactions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Actions::Action::terminateSubactions"))) (target (node (document "d0") (qualified-name "Actions::Action::subactions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action::terminateSubactions"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Actions::Action::terminateSubactions"))) (target (node (document "d0") (qualified-name "Actions::terminateActions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action::terminateSubactions"))) (kind subsetting) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::Action::transitions"))) (target (node (document "d0") (qualified-name "Actions::TransitionAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action::transitions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Actions::Action::transitions"))) (target (node (document "d0") (qualified-name "Actions::Action::subactions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action::transitions"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Actions::Action::transitions"))) (target (node (document "d0") (qualified-name "Actions::transitionActions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action::transitions"))) (kind subsetting) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::Action::whileLoops"))) (target (node (document "d0") (qualified-name "Actions::WhileLoopAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action::whileLoops"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Actions::Action::whileLoops"))) (target (node (document "d0") (qualified-name "Actions::Action::loops"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action::whileLoops"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Actions::Action::whileLoops"))) (target (node (document "d0") (qualified-name "Actions::whileLoopActions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::Action::whileLoops"))) (kind subsetting) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::AssignmentAction"))) (target (node (document "d0") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::AssignmentAction"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::AssignmentAction"))) (target (node (document "d0") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::AssignmentAction"))) (kind specialization) (ordinal 3)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::AssignmentAction"))) (target (node (document "d0") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::AssignmentAction"))) (kind specialization) (ordinal 5)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::AssignmentAction"))) (target (node (document "d0") (qualified-name "Actions::FeatureWritePerformance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::AssignmentAction"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::AssignmentAction"))) (target (node (document "d0") (qualified-name "Actions::FeatureWritePerformance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::AssignmentAction"))) (kind specialization) (ordinal 2)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::AssignmentAction"))) (target (node (document "d0") (qualified-name "Actions::FeatureWritePerformance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::AssignmentAction"))) (kind specialization) (ordinal 4)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::ControlAction"))) (target (node (document "d0") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::ControlAction"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::ControlAction"))) (target (node (document "d0") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::ControlAction"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::ControlAction"))) (target (node (document "d0") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::ControlAction"))) (kind specialization) (ordinal 2)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::DecisionAction"))) (target (node (document "d0") (qualified-name "Actions::ControlAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::DecisionAction"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::DecisionAction"))) (target (node (document "d0") (qualified-name "Actions::ControlAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::DecisionAction"))) (kind specialization) (ordinal 2)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::DecisionAction"))) (target (node (document "d0") (qualified-name "Actions::ControlAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::DecisionAction"))) (kind specialization) (ordinal 4)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::DecisionAction"))) (target (node (document "d0") (qualified-name "Actions::DecisionPerformance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::DecisionAction"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::DecisionAction"))) (target (node (document "d0") (qualified-name "Actions::DecisionPerformance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::DecisionAction"))) (kind specialization) (ordinal 3)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::DecisionAction"))) (target (node (document "d0") (qualified-name "Actions::DecisionPerformance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::DecisionAction"))) (kind specialization) (ordinal 5)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::DecisionTransitionAction"))) (target (node (document "d0") (qualified-name "Actions::NonStateTransitionPerformance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::DecisionTransitionAction"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::DecisionTransitionAction"))) (target (node (document "d0") (qualified-name "Actions::NonStateTransitionPerformance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::DecisionTransitionAction"))) (kind specialization) (ordinal 3)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::DecisionTransitionAction"))) (target (node (document "d0") (qualified-name "Actions::NonStateTransitionPerformance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::DecisionTransitionAction"))) (kind specialization) (ordinal 5)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::DecisionTransitionAction"))) (target (node (document "d0") (qualified-name "Actions::TransitionAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::DecisionTransitionAction"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::DecisionTransitionAction"))) (target (node (document "d0") (qualified-name "Actions::TransitionAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::DecisionTransitionAction"))) (kind specialization) (ordinal 2)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::DecisionTransitionAction"))) (target (node (document "d0") (qualified-name "Actions::TransitionAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::DecisionTransitionAction"))) (kind specialization) (ordinal 4)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Actions::DecisionTransitionAction::"))) (target (node (document "d0") (qualified-name "Actions::TransitionAction::accepter"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::DecisionTransitionAction::"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Actions::DecisionTransitionAction::#ref"))) (target (node (document "d0") (qualified-name "Actions::TransitionAction::effect"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::DecisionTransitionAction::#ref"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::ForLoopAction"))) (target (node (document "d0") (qualified-name "Actions::LoopAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::ForLoopAction"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::ForLoopAction"))) (target (node (document "d0") (qualified-name "Actions::LoopAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::ForLoopAction"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::ForLoopAction"))) (target (node (document "d0") (qualified-name "Actions::LoopAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::ForLoopAction"))) (kind specialization) (ordinal 2)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Actions::ForLoopAction"))) (target (node (document "d0") (qualified-name "Actions::ForLoopAction::initialization"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::ForLoopAction"))) (kind performSource) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Actions::ForLoopAction"))) (target (node (document "d0") (qualified-name "Actions::ForLoopAction::whileLoop"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::ForLoopAction"))) (kind performSource) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::ForLoopAction::body"))) (target (node (document "d0") (qualified-name "Actions::LoopAction::action"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::ForLoopAction::body"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::ForLoopAction::whileLoop"))) (target (node (document "d0") (qualified-name "Actions::ForLoopAction::_documentation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::ForLoopAction::whileLoop"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::ForkAction"))) (target (node (document "d0") (qualified-name "Actions::ControlAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::ForkAction"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::ForkAction"))) (target (node (document "d0") (qualified-name "Actions::ControlAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::ForkAction"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::ForkAction"))) (target (node (document "d0") (qualified-name "Actions::ControlAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::ForkAction"))) (kind specialization) (ordinal 2)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::IfThenAction"))) (target (node (document "d0") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::IfThenAction"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::IfThenAction"))) (target (node (document "d0") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::IfThenAction"))) (kind specialization) (ordinal 2)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::IfThenAction"))) (target (node (document "d0") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::IfThenAction"))) (kind specialization) (ordinal 4)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::IfThenAction"))) (target (node (document "d0") (qualified-name "Actions::IfThenPerformance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::IfThenAction"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::IfThenAction"))) (target (node (document "d0") (qualified-name "Actions::IfThenPerformance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::IfThenAction"))) (kind specialization) (ordinal 3)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::IfThenAction"))) (target (node (document "d0") (qualified-name "Actions::IfThenPerformance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::IfThenAction"))) (kind specialization) (ordinal 5)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::IfThenElseAction"))) (target (node (document "d0") (qualified-name "Actions::IfThenAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::IfThenElseAction"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::IfThenElseAction"))) (target (node (document "d0") (qualified-name "Actions::IfThenAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::IfThenElseAction"))) (kind specialization) (ordinal 2)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::IfThenElseAction"))) (target (node (document "d0") (qualified-name "Actions::IfThenAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::IfThenElseAction"))) (kind specialization) (ordinal 4)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::IfThenElseAction"))) (target (node (document "d0") (qualified-name "Actions::IfThenElsePerformance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::IfThenElseAction"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::IfThenElseAction"))) (target (node (document "d0") (qualified-name "Actions::IfThenElsePerformance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::IfThenElseAction"))) (kind specialization) (ordinal 3)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::IfThenElseAction"))) (target (node (document "d0") (qualified-name "Actions::IfThenElsePerformance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::IfThenElseAction"))) (kind specialization) (ordinal 5)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::JoinAction"))) (target (node (document "d0") (qualified-name "Actions::ControlAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::JoinAction"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::JoinAction"))) (target (node (document "d0") (qualified-name "Actions::ControlAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::JoinAction"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::JoinAction"))) (target (node (document "d0") (qualified-name "Actions::ControlAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::JoinAction"))) (kind specialization) (ordinal 2)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::LoopAction"))) (target (node (document "d0") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::LoopAction"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::LoopAction"))) (target (node (document "d0") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::LoopAction"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::LoopAction"))) (target (node (document "d0") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::LoopAction"))) (kind specialization) (ordinal 2)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::MergeAction"))) (target (node (document "d0") (qualified-name "Actions::ControlAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::MergeAction"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::MergeAction"))) (target (node (document "d0") (qualified-name "Actions::ControlAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::MergeAction"))) (kind specialization) (ordinal 2)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::MergeAction"))) (target (node (document "d0") (qualified-name "Actions::ControlAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::MergeAction"))) (kind specialization) (ordinal 4)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::MergeAction"))) (target (node (document "d0") (qualified-name "Actions::MergePerformance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::MergeAction"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::MergeAction"))) (target (node (document "d0") (qualified-name "Actions::MergePerformance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::MergeAction"))) (kind specialization) (ordinal 3)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::MergeAction"))) (target (node (document "d0") (qualified-name "Actions::MergePerformance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::MergeAction"))) (kind specialization) (ordinal 5)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::SendAction"))) (target (node (document "d0") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::SendAction"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::SendAction"))) (target (node (document "d0") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::SendAction"))) (kind specialization) (ordinal 2)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::SendAction"))) (target (node (document "d0") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::SendAction"))) (kind specialization) (ordinal 4)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::SendAction"))) (target (node (document "d0") (qualified-name "Actions::SendPerformance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::SendAction"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::SendAction"))) (target (node (document "d0") (qualified-name "Actions::SendPerformance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::SendAction"))) (kind specialization) (ordinal 3)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::SendAction"))) (target (node (document "d0") (qualified-name "Actions::SendPerformance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::SendAction"))) (kind specialization) (ordinal 5)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::SendAction::payload"))) (target (node (document "d0") (qualified-name "Actions::SendAction::_documentation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::SendAction::payload"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::SendAction::sentMessage"))) (target (node (document "d0") (qualified-name "Actions::MessageAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::SendAction::sentMessage"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::SendAction::sentMessage"))) (target (node (document "d0") (qualified-name "Actions::MessageTransfer"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::SendAction::sentMessage"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::SendAction::sentMessage::MessageTransfer::payload, MessageAction::payload"))) (target (node (document "d0") (qualified-name "Actions::SendAction::_documentation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::SendAction::sentMessage::MessageTransfer::payload, MessageAction::payload"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::TerminateAction"))) (target (node (document "d0") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::TerminateAction"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::TerminateAction"))) (target (node (document "d0") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::TerminateAction"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::TerminateAction"))) (target (node (document "d0") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::TerminateAction"))) (kind specialization) (ordinal 2)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Actions::TerminateAction"))) (target (node (document "d0") (qualified-name "Actions::TerminateAction::terminateOccurrence"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::TerminateAction"))) (kind performSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::TerminateAction::terminateOccurrence"))) (target (node (document "d0") (qualified-name "Actions::destroy"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::TerminateAction::terminateOccurrence"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::TerminateAction::terminateOccurrence::occ"))) (target (node (document "d0") (qualified-name "Actions::TerminateAction::_documentation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::TerminateAction::terminateOccurrence::occ"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::TransitionAction"))) (target (node (document "d0") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::TransitionAction"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::TransitionAction"))) (target (node (document "d0") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::TransitionAction"))) (kind specialization) (ordinal 2)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::TransitionAction"))) (target (node (document "d0") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::TransitionAction"))) (kind specialization) (ordinal 4)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::TransitionAction"))) (target (node (document "d0") (qualified-name "Actions::TransitionPerformance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::TransitionAction"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::TransitionAction"))) (target (node (document "d0") (qualified-name "Actions::TransitionPerformance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::TransitionAction"))) (kind specialization) (ordinal 3)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::TransitionAction"))) (target (node (document "d0") (qualified-name "Actions::TransitionPerformance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::TransitionAction"))) (kind specialization) (ordinal 5)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Actions::TransitionAction"))) (target (node (document "d0") (qualified-name "Actions::TransitionAction::accepter"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::TransitionAction"))) (kind performSource) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Actions::TransitionAction"))) (target (node (document "d0") (qualified-name "Actions::TransitionAction::effect"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::TransitionAction"))) (kind performSource) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::TransitionAction::acceptedMessage"))) (target (node (document "d0") (qualified-name "Actions::MessageAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::TransitionAction::acceptedMessage"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::TransitionAction::acceptedMessage"))) (target (node (document "d0") (qualified-name "Actions::MessageTransfer"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::TransitionAction::acceptedMessage"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::TransitionAction::acceptedMessage::MessageTransfer::payload, MessageAction::payload"))) (target (node (document "d0") (qualified-name "Actions::TransitionAction::_documentation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::TransitionAction::acceptedMessage::MessageTransfer::payload, MessageAction::payload"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::TransitionAction::accepter"))) (target (node (document "d0") (qualified-name "Actions::AcceptMessageAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::TransitionAction::accepter"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::TransitionAction::effect"))) (target (node (document "d0") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::TransitionAction::effect"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::WhileLoopAction"))) (target (node (document "d0") (qualified-name "Actions::LoopAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::WhileLoopAction"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::WhileLoopAction"))) (target (node (document "d0") (qualified-name "Actions::LoopAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::WhileLoopAction"))) (kind specialization) (ordinal 2)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::WhileLoopAction"))) (target (node (document "d0") (qualified-name "Actions::LoopAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::WhileLoopAction"))) (kind specialization) (ordinal 4)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::WhileLoopAction"))) (target (node (document "d0") (qualified-name "Actions::LoopPerformance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::WhileLoopAction"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::WhileLoopAction"))) (target (node (document "d0") (qualified-name "Actions::LoopPerformance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::WhileLoopAction"))) (kind specialization) (ordinal 3)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Actions::WhileLoopAction"))) (target (node (document "d0") (qualified-name "Actions::LoopPerformance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::WhileLoopAction"))) (kind specialization) (ordinal 5)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::WhileLoopAction::body"))) (target (node (document "d0") (qualified-name "Actions::LoopAction::action"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::WhileLoopAction::body"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::WhileLoopAction::untilTest"))) (target (node (document "d0") (qualified-name "Actions::WhileLoopAction::_documentation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::WhileLoopAction::untilTest"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::WhileLoopAction::whileTest"))) (target (node (document "d0") (qualified-name "Actions::WhileLoopAction::_documentation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::WhileLoopAction::whileTest"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::acceptActions"))) (target (node (document "d0") (qualified-name "Actions::AcceptAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::acceptActions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Actions::acceptActions"))) (target (node (document "d0") (qualified-name "Actions::acceptPerformances"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::acceptActions"))) (kind subsetting) (ordinal 1)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Actions::acceptActions"))) (target (node (document "d0") (qualified-name "Actions::actions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::acceptActions"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::actions"))) (target (node (document "d0") (qualified-name "Actions::Action"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::actions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Actions::actions"))) (target (node (document "d0") (qualified-name "Actions::performances"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::actions"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::assignmentActions"))) (target (node (document "d0") (qualified-name "Actions::AssignmentAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::assignmentActions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Actions::assignmentActions"))) (target (node (document "d0") (qualified-name "Actions::actions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::assignmentActions"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::forLoopActions"))) (target (node (document "d0") (qualified-name "Actions::ForLoopAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::forLoopActions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Actions::forLoopActions"))) (target (node (document "d0") (qualified-name "Actions::loopActions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::forLoopActions"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::ifThenActions"))) (target (node (document "d0") (qualified-name "Actions::IfThenAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::ifThenActions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Actions::ifThenActions"))) (target (node (document "d0") (qualified-name "Actions::actions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::ifThenActions"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::ifThenElseActions"))) (target (node (document "d0") (qualified-name "Actions::IfThenElseAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::ifThenElseActions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Actions::ifThenElseActions"))) (target (node (document "d0") (qualified-name "Actions::actions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::ifThenElseActions"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::loopActions"))) (target (node (document "d0") (qualified-name "Actions::LoopAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::loopActions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Actions::loopActions"))) (target (node (document "d0") (qualified-name "Actions::actions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::loopActions"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::sendActions"))) (target (node (document "d0") (qualified-name "Actions::SendAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::sendActions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Actions::sendActions"))) (target (node (document "d0") (qualified-name "Actions::actions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::sendActions"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Actions::sendActions"))) (target (node (document "d0") (qualified-name "Actions::sendPerformances"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::sendActions"))) (kind subsetting) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::terminateActions"))) (target (node (document "d0") (qualified-name "Actions::TerminateAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::terminateActions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Actions::terminateActions"))) (target (node (document "d0") (qualified-name "Actions::actions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::terminateActions"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::transitionActions"))) (target (node (document "d0") (qualified-name "Actions::TransitionAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::transitionActions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Actions::transitionActions"))) (target (node (document "d0") (qualified-name "Actions::actions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::transitionActions"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Actions::whileLoopActions"))) (target (node (document "d0") (qualified-name "Actions::WhileLoopAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::whileLoopActions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Actions::whileLoopActions"))) (target (node (document "d0") (qualified-name "Actions::loopActions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::whileLoopActions"))) (kind subsetting) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Actions::AcceptAction::")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "Actions::TerminateAction::terminateOccurrence::occ")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 491 29) (end 491 32)) (probe (position 491 29))
      (reference
        (source (document "d0") (qualified-name "Actions::ForLoopAction::var"))
        (kind featureTyping) (ordinal 0) (authored-target "seq")
        (range (start 491 29) (end 491 32))
        (outcome (status unresolved))
      )
    )
    (query (range (start 280 15) (end 280 19)) (probe (position 280 15))
      (reference
        (source (document "d0") (qualified-name "Actions::ControlAction"))
        (kind bindTarget) (ordinal 0) (authored-target "done")
        (range (start 280 15) (end 280 19))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::Action::done") (range (start 49 2) (end 49 99)))
        )
      )
    )
    (query (range (start 164 56) (end 164 61)) (probe (position 164 56))
      (reference
        (source (document "d0") (qualified-name "Actions::Action::whileLoops"))
        (kind subsetting) (ordinal 0) (authored-target "loops")
        (range (start 164 56) (end 164 61))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::Action::loops") (range (start 157 2) (end 157 153)))
        )
      )
    )
    (query (range (start 171 52) (end 171 57)) (probe (position 171 52))
      (reference
        (source (document "d0") (qualified-name "Actions::Action::forLoops"))
        (kind subsetting) (ordinal 0) (authored-target "loops")
        (range (start 171 52) (end 171 57))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::Action::loops") (range (start 157 2) (end 157 153)))
        )
      )
    )
    (query (range (start 280 7) (end 280 12)) (probe (position 280 7))
      (reference
        (source (document "d0") (qualified-name "Actions::ControlAction"))
        (kind bindSource) (ordinal 0) (authored-target "start")
        (range (start 280 7) (end 280 12))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::Action::start") (range (start 42 2) (end 42 105)))
        )
      )
    )
    (query (range (start 39 19) (end 39 25)) (probe (position 39 19))
      (reference
        (source (document "d0") (qualified-name "Actions::Action::self"))
        (kind featureTyping) (ordinal 0) (authored-target "Action")
        (range (start 39 19) (end 39 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::Action") (range (start 32 1) (end 32 3407)))
        )
      )
    )
    (query (range (start 186 26) (end 186 32)) (probe (position 186 26))
      (reference
        (source (document "d0") (qualified-name "Actions::SendAction"))
        (kind specialization) (ordinal 4) (authored-target "Action")
        (range (start 186 26) (end 186 32))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::Action") (range (start 32 1) (end 32 3407)))
        )
      )
    )
    (query (range (start 206 35) (end 206 41)) (probe (position 206 35))
      (reference
        (source (document "d0") (qualified-name "Actions::AcceptMessageAction"))
        (kind specialization) (ordinal 4) (authored-target "Action")
        (range (start 206 35) (end 206 41))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::Action") (range (start 32 1) (end 32 3407)))
        )
      )
    )
    (query (range (start 239 40) (end 239 46)) (probe (position 239 40))
      (reference
        (source (document "d0") (qualified-name "Actions::TerminateAction"))
        (kind specialization) (ordinal 2) (authored-target "Action")
        (range (start 239 40) (end 239 46))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::Action") (range (start 32 1) (end 32 3407)))
        )
      )
    )
    (query (range (start 274 38) (end 274 44)) (probe (position 274 38))
      (reference
        (source (document "d0") (qualified-name "Actions::ControlAction"))
        (kind specialization) (ordinal 2) (authored-target "Action")
        (range (start 274 38) (end 274 44))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::Action") (range (start 32 1) (end 32 3407)))
        )
      )
    )
    (query (range (start 330 41) (end 330 47)) (probe (position 330 41))
      (reference
        (source (document "d0") (qualified-name "Actions::TransitionAction"))
        (kind specialization) (ordinal 4) (authored-target "Action")
        (range (start 330 41) (end 330 47))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::Action") (range (start 32 1) (end 32 3407)))
        )
      )
    )
    (query (range (start 361 17) (end 361 23)) (probe (position 361 17))
      (reference
        (source (document "d0") (qualified-name "Actions::DecisionTransitionAction::#ref"))
        (kind redefinition) (ordinal 0) (authored-target "effect")
        (range (start 361 17) (end 361 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::TransitionAction::effect") (range (start 349 2) (end 349 58)))
        )
      )
    )
    (query (range (start 371 57) (end 371 63)) (probe (position 371 57))
      (reference
        (source (document "d0") (qualified-name "Actions::AssignmentAction"))
        (kind specialization) (ordinal 5) (authored-target "Action")
        (range (start 371 57) (end 371 63))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::Action") (range (start 32 1) (end 32 3407)))
        )
      )
    )
    (query (range (start 398 28) (end 398 34)) (probe (position 398 28))
      (reference
        (source (document "d0") (qualified-name "Actions::IfThenAction"))
        (kind specialization) (ordinal 4) (authored-target "Action")
        (range (start 398 28) (end 398 34))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::Action") (range (start 32 1) (end 32 3407)))
        )
      )
    )
    (query (range (start 435 35) (end 435 41)) (probe (position 435 35))
      (reference
        (source (document "d0") (qualified-name "Actions::LoopAction"))
        (kind specialization) (ordinal 2) (authored-target "Action")
        (range (start 435 35) (end 435 41))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::Action") (range (start 32 1) (end 32 3407)))
        )
      )
    )
    (query (range (start 49 26) (end 49 33)) (probe (position 49 26))
      (reference
        (source (document "d0") (qualified-name "Actions::Action::done"))
        (kind redefinition) (ordinal 0) (authored-target "endShot")
        (range (start 49 26) (end 49 33))
        (outcome (status unresolved))
      )
    )
    (query (range (start 56 37) (end 56 44)) (probe (position 56 37))
      (reference
        (source (document "d0") (qualified-name "Actions::Action::subactions"))
        (kind subsetting) (ordinal 0) (authored-target "actions")
        (range (start 56 37) (end 56 44))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::actions") (range (start 179 1) (end 179 141)))
        )
      )
    )
    (query (range (start 199 60) (end 199 67)) (probe (position 199 60))
      (reference
        (source (document "d0") (qualified-name "Actions::sendActions"))
        (kind subsetting) (ordinal 0) (authored-target "actions")
        (range (start 199 60) (end 199 67))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::actions") (range (start 179 1) (end 179 141)))
        )
      )
    )
    (query (range (start 229 7) (end 229 14)) (probe (position 229 7))
      (reference
        (source (document "d0") (qualified-name "Actions::AcceptAction"))
        (kind bindSource) (ordinal 0) (authored-target "payload")
        (range (start 229 7) (end 229 14))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::AcceptMessageAction::payload") (range (start 212 2) (end 212 20)))
        )
      )
    )
    (query (range (start 232 64) (end 232 71)) (probe (position 232 64))
      (reference
        (source (document "d0") (qualified-name "Actions::acceptActions"))
        (kind subsetting) (ordinal 0) (authored-target "actions")
        (range (start 232 64) (end 232 71))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::actions") (range (start 179 1) (end 179 141)))
        )
      )
    )
    (query (range (start 259 71) (end 259 78)) (probe (position 259 71))
      (reference
        (source (document "d0") (qualified-name "Actions::terminateActions"))
        (kind subsetting) (ordinal 0) (authored-target "actions")
        (range (start 259 71) (end 259 78))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::actions") (range (start 179 1) (end 179 141)))
        )
      )
    )
    (query (range (start 364 72) (end 364 79)) (probe (position 364 72))
      (reference
        (source (document "d0") (qualified-name "Actions::transitionActions"))
        (kind subsetting) (ordinal 0) (authored-target "actions")
        (range (start 364 72) (end 364 79))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::actions") (range (start 179 1) (end 179 141)))
        )
      )
    )
    (query (range (start 383 73) (end 383 80)) (probe (position 383 73))
      (reference
        (source (document "d0") (qualified-name "Actions::assignmentActions"))
        (kind subsetting) (ordinal 0) (authored-target "actions")
        (range (start 383 73) (end 383 80))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::actions") (range (start 179 1) (end 179 141)))
        )
      )
    )
    (query (range (start 421 65) (end 421 72)) (probe (position 421 65))
      (reference
        (source (document "d0") (qualified-name "Actions::ifThenActions"))
        (kind subsetting) (ordinal 0) (authored-target "actions")
        (range (start 421 65) (end 421 72))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::actions") (range (start 179 1) (end 179 141)))
        )
      )
    )
    (query (range (start 428 73) (end 428 80)) (probe (position 428 73))
      (reference
        (source (document "d0") (qualified-name "Actions::ifThenElseActions"))
        (kind subsetting) (ordinal 0) (authored-target "actions")
        (range (start 428 73) (end 428 80))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::actions") (range (start 179 1) (end 179 141)))
        )
      )
    )
    (query (range (start 531 61) (end 531 68)) (probe (position 531 61))
      (reference
        (source (document "d0") (qualified-name "Actions::loopActions"))
        (kind subsetting) (ordinal 0) (authored-target "actions")
        (range (start 531 61) (end 531 68))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::actions") (range (start 179 1) (end 179 141)))
        )
      )
    )
    (query (range (start 99 48) (end 99 56)) (probe (position 99 48))
      (reference
        (source (document "d0") (qualified-name "Actions::Action::merges"))
        (kind subsetting) (ordinal 0) (authored-target "controls")
        (range (start 99 48) (end 99 56))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::Action::controls") (range (start 92 2) (end 92 149)))
        )
      )
    )
    (query (range (start 106 48) (end 106 56)) (probe (position 106 48))
      (reference
        (source (document "d0") (qualified-name "Actions::Action::decisions"))
        (kind subsetting) (ordinal 0) (authored-target "controls")
        (range (start 106 48) (end 106 56))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::Action::controls") (range (start 92 2) (end 92 149)))
        )
      )
    )
    (query (range (start 113 40) (end 113 48)) (probe (position 113 40))
      (reference
        (source (document "d0") (qualified-name "Actions::Action::joins"))
        (kind subsetting) (ordinal 0) (authored-target "controls")
        (range (start 113 40) (end 113 48))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::Action::controls") (range (start 92 2) (end 92 149)))
        )
      )
    )
    (query (range (start 120 40) (end 120 48)) (probe (position 120 40))
      (reference
        (source (document "d0") (qualified-name "Actions::Action::forks"))
        (kind subsetting) (ordinal 0) (authored-target "controls")
        (range (start 120 40) (end 120 48))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::Action::controls") (range (start 92 2) (end 92 149)))
        )
      )
    )
    (query (range (start 344 44) (end 344 52)) (probe (position 344 44))
      (reference
        (source (document "d0") (qualified-name "Actions::TransitionAction::accepter"))
        (kind redefinition) (ordinal 0) (authored-target "accept")
        (range (start 344 44) (end 344 52))
        (outcome (status unresolved))
      )
    )
    (query (range (start 346 7) (end 346 15)) (probe (position 346 7))
      (reference
        (source (document "d0") (qualified-name "Actions::TransitionAction"))
        (kind bindSource) (ordinal 0) (authored-target "receiver")
        (range (start 346 7) (end 346 15))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::TransitionAction::receiver") (range (start 342 2) (end 342 33)))
        )
      )
    )
    (query (range (start 360 17) (end 360 25)) (probe (position 360 17))
      (reference
        (source (document "d0") (qualified-name "Actions::DecisionTransitionAction::"))
        (kind redefinition) (ordinal 0) (authored-target "accepter")
        (range (start 360 17) (end 360 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::TransitionAction::accepter") (range (start 344 2) (end 344 53)))
        )
      )
    )
    (query (range (start 42 27) (end 42 36)) (probe (position 42 27))
      (reference
        (source (document "d0") (qualified-name "Actions::Action::start"))
        (kind redefinition) (ordinal 0) (authored-target "startShot")
        (range (start 42 27) (end 42 36))
        (outcome (status unresolved))
      )
    )
    (query (range (start 71 45) (end 71 55)) (probe (position 71 45))
      (reference
        (source (document "d0") (qualified-name "Actions::Action::sendSubactions"))
        (kind subsetting) (ordinal 0) (authored-target "subactions")
        (range (start 71 45) (end 71 55))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::Action::subactions") (range (start 56 2) (end 56 376)))
        )
      )
    )
    (query (range (start 78 49) (end 78 59)) (probe (position 78 49))
      (reference
        (source (document "d0") (qualified-name "Actions::Action::acceptSubactions"))
        (kind subsetting) (ordinal 0) (authored-target "subactions")
        (range (start 78 49) (end 78 59))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::Action::subactions") (range (start 56 2) (end 56 376)))
        )
      )
    )
    (query (range (start 85 65) (end 85 75)) (probe (position 85 65))
      (reference
        (source (document "d0") (qualified-name "Actions::Action::terminateSubactions"))
        (kind subsetting) (ordinal 0) (authored-target "subactions")
        (range (start 85 65) (end 85 75))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::Action::subactions") (range (start 56 2) (end 56 376)))
        )
      )
    )
    (query (range (start 92 52) (end 92 62)) (probe (position 92 52))
      (reference
        (source (document "d0") (qualified-name "Actions::Action::controls"))
        (kind subsetting) (ordinal 0) (authored-target "subactions")
        (range (start 92 52) (end 92 62))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::Action::subactions") (range (start 56 2) (end 56 376)))
        )
      )
    )
    (query (range (start 127 58) (end 127 68)) (probe (position 127 58))
      (reference
        (source (document "d0") (qualified-name "Actions::Action::transitions"))
        (kind subsetting) (ordinal 0) (authored-target "subactions")
        (range (start 127 58) (end 127 68))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::Action::subactions") (range (start 56 2) (end 56 376)))
        )
      )
    )
    (query (range (start 141 58) (end 141 68)) (probe (position 141 58))
      (reference
        (source (document "d0") (qualified-name "Actions::Action::assignments"))
        (kind subsetting) (ordinal 0) (authored-target "subactions")
        (range (start 141 58) (end 141 68))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::Action::subactions") (range (start 56 2) (end 56 376)))
        )
      )
    )
    (query (range (start 150 55) (end 150 65)) (probe (position 150 55))
      (reference
        (source (document "d0") (qualified-name "Actions::Action::ifSubactions"))
        (kind subsetting) (ordinal 0) (authored-target "subactions")
        (range (start 150 55) (end 150 65))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::Action::subactions") (range (start 56 2) (end 56 376)))
        )
      )
    )
    (query (range (start 157 46) (end 157 56)) (probe (position 157 46))
      (reference
        (source (document "d0") (qualified-name "Actions::Action::loops"))
        (kind subsetting) (ordinal 0) (authored-target "subactions")
        (range (start 157 46) (end 157 56))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::Action::subactions") (range (start 56 2) (end 56 376)))
        )
      )
    )
    (query (range (start 451 31) (end 451 41)) (probe (position 451 31))
      (reference
        (source (document "d0") (qualified-name "Actions::WhileLoopAction"))
        (kind specialization) (ordinal 4) (authored-target "LoopAction")
        (range (start 451 31) (end 451 41))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::LoopAction") (range (start 435 1) (end 435 264)))
        )
      )
    )
    (query (range (start 484 29) (end 484 39)) (probe (position 484 29))
      (reference
        (source (document "d0") (qualified-name "Actions::ForLoopAction"))
        (kind specialization) (ordinal 2) (authored-target "LoopAction")
        (range (start 484 29) (end 484 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::LoopAction") (range (start 435 1) (end 435 264)))
        )
      )
    )
    (query (range (start 32 31) (end 32 42)) (probe (position 32 31))
      (reference
        (source (document "d0") (qualified-name "Actions::Action"))
        (kind specialization) (ordinal 2) (authored-target "Performance")
        (range (start 32 31) (end 32 42))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::Performance") (range (start 14 1) (end 14 42)))
        )
      )
    )
    (query (range (start 71 57) (end 71 68)) (probe (position 71 57))
      (reference
        (source (document "d0") (qualified-name "Actions::Action::sendSubactions"))
        (kind subsetting) (ordinal 1) (authored-target "sendActions")
        (range (start 71 57) (end 71 68))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::sendActions") (range (start 199 1) (end 199 170)))
        )
      )
    )
    (query (range (start 134 74) (end 134 85)) (probe (position 134 74))
      (reference
        (source (document "d0") (qualified-name "Actions::Action::decisionTransitions"))
        (kind subsetting) (ordinal 0) (authored-target "transitions")
        (range (start 134 74) (end 134 85))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::Action::transitions") (range (start 127 2) (end 127 178)))
        )
      )
    )
    (query (range (start 157 58) (end 157 69)) (probe (position 157 58))
      (reference
        (source (document "d0") (qualified-name "Actions::Action::loops"))
        (kind subsetting) (ordinal 1) (authored-target "loopActions")
        (range (start 157 58) (end 157 69))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::loopActions") (range (start 531 1) (end 531 153)))
        )
      )
    )
    (query (range (start 538 71) (end 538 82)) (probe (position 538 71))
      (reference
        (source (document "d0") (qualified-name "Actions::whileLoopActions"))
        (kind subsetting) (ordinal 0) (authored-target "loopActions")
        (range (start 538 71) (end 538 82))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::loopActions") (range (start 531 1) (end 531 153)))
        )
      )
    )
    (query (range (start 545 67) (end 545 78)) (probe (position 545 67))
      (reference
        (source (document "d0") (qualified-name "Actions::forLoopActions"))
        (kind subsetting) (ordinal 0) (authored-target "loopActions")
        (range (start 545 67) (end 545 78))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::loopActions") (range (start 531 1) (end 531 153)))
        )
      )
    )
    (query (range (start 62 22) (end 62 34)) (probe (position 62 22))
      (reference
        (source (document "d0") (qualified-name "Actions::Action::subactions::occurrence"))
        (kind redefinition) (ordinal 0) (authored-target "Action::this")
        (range (start 62 22) (end 62 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 179 52) (end 179 64)) (probe (position 179 52))
      (reference
        (source (document "d0") (qualified-name "Actions::actions"))
        (kind subsetting) (ordinal 0) (authored-target "performances")
        (range (start 179 52) (end 179 64))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::performances") (range (start 15 1) (end 15 43)))
        )
      )
    )
    (query (range (start 194 25) (end 194 37)) (probe (position 194 25))
      (reference
        (source (document "d0") (qualified-name "Actions::SendAction::sentMessage"))
        (kind redefinition) (ordinal 0) (authored-target "sentTransfer")
        (range (start 194 25) (end 194 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 409 32) (end 409 44)) (probe (position 409 32))
      (reference
        (source (document "d0") (qualified-name "Actions::IfThenElseAction"))
        (kind specialization) (ordinal 4) (authored-target "IfThenAction")
        (range (start 409 32) (end 409 44))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::IfThenAction") (range (start 398 1) (end 398 248)))
        )
      )
    )
    (query (range (start 78 61) (end 78 74)) (probe (position 78 61))
      (reference
        (source (document "d0") (qualified-name "Actions::Action::acceptSubactions"))
        (kind subsetting) (ordinal 1) (authored-target "acceptActions")
        (range (start 78 61) (end 78 74))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::acceptActions") (range (start 232 1) (end 232 187)))
        )
      )
    )
    (query (range (start 150 67) (end 150 80)) (probe (position 150 67))
      (reference
        (source (document "d0") (qualified-name "Actions::Action::ifSubactions"))
        (kind subsetting) (ordinal 1) (authored-target "ifThenActions")
        (range (start 150 67) (end 150 80))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::ifThenActions") (range (start 421 1) (end 421 157)))
        )
      )
    )
    (query (range (start 194 56) (end 194 69)) (probe (position 194 56))
      (reference
        (source (document "d0") (qualified-name "Actions::SendAction::sentMessage"))
        (kind featureTyping) (ordinal 1) (authored-target "MessageAction")
        (range (start 194 56) (end 194 69))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::MessageAction") (range (start 29 1) (end 29 37)))
        )
      )
    )
    (query (range (start 213 61) (end 213 74)) (probe (position 213 61))
      (reference
        (source (document "d0") (qualified-name "Actions::AcceptMessageAction::acceptedMessage"))
        (kind featureTyping) (ordinal 1) (authored-target "MessageAction")
        (range (start 213 61) (end 213 74))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::MessageAction") (range (start 29 1) (end 29 37)))
        )
      )
    )
    (query (range (start 288 27) (end 288 40)) (probe (position 288 27))
      (reference
        (source (document "d0") (qualified-name "Actions::MergeAction"))
        (kind specialization) (ordinal 4) (authored-target "ControlAction")
        (range (start 288 27) (end 288 40))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::ControlAction") (range (start 274 1) (end 274 240)))
        )
      )
    )
    (query (range (start 298 30) (end 298 43)) (probe (position 298 30))
      (reference
        (source (document "d0") (qualified-name "Actions::DecisionAction"))
        (kind specialization) (ordinal 4) (authored-target "ControlAction")
        (range (start 298 30) (end 298 43))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::ControlAction") (range (start 274 1) (end 274 240)))
        )
      )
    )
    (query (range (start 310 26) (end 310 39)) (probe (position 310 26))
      (reference
        (source (document "d0") (qualified-name "Actions::JoinAction"))
        (kind specialization) (ordinal 2) (authored-target "ControlAction")
        (range (start 310 26) (end 310 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::ControlAction") (range (start 274 1) (end 274 240)))
        )
      )
    )
    (query (range (start 320 26) (end 320 39)) (probe (position 320 26))
      (reference
        (source (document "d0") (qualified-name "Actions::ForkAction"))
        (kind specialization) (ordinal 2) (authored-target "ControlAction")
        (range (start 320 26) (end 320 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::ControlAction") (range (start 274 1) (end 274 240)))
        )
      )
    )
    (query (range (start 338 41) (end 338 54)) (probe (position 338 41))
      (reference
        (source (document "d0") (qualified-name "Actions::TransitionAction::acceptedMessage"))
        (kind featureTyping) (ordinal 1) (authored-target "MessageAction")
        (range (start 338 41) (end 338 54))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::MessageAction") (range (start 29 1) (end 29 37)))
        )
      )
    )
    (query (range (start 342 19) (end 342 32)) (probe (position 342 19))
      (reference
        (source (document "d0") (qualified-name "Actions::TransitionAction::receiver"))
        (kind redefinition) (ordinal 0) (authored-target "triggerTarget")
        (range (start 342 19) (end 342 32))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 16) (end 7 30)) (probe (position 7 16))
      (reference
        (source (document "d0") (qualified-name "Actions::Anything"))
        (kind membershipImport) (ordinal 0) (authored-target "Base::Anything")
        (range (start 7 16) (end 7 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 171 59) (end 171 73)) (probe (position 171 59))
      (reference
        (source (document "d0") (qualified-name "Actions::Action::forLoops"))
        (kind subsetting) (ordinal 1) (authored-target "forLoopActions")
        (range (start 171 59) (end 171 73))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::forLoopActions") (range (start 545 1) (end 545 169)))
        )
      )
    )
    (query (range (start 56 46) (end 56 61)) (probe (position 56 46))
      (reference
        (source (document "d0") (qualified-name "Actions::Action::subactions"))
        (kind subsetting) (ordinal 1) (authored-target "subperformances")
        (range (start 56 46) (end 56 61))
        (outcome (status unresolved))
      )
    )
    (query (range (start 186 34) (end 186 49)) (probe (position 186 34))
      (reference
        (source (document "d0") (qualified-name "Actions::SendAction"))
        (kind specialization) (ordinal 5) (authored-target "SendPerformance")
        (range (start 186 34) (end 186 49))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::SendPerformance") (range (start 16 1) (end 16 43)))
        )
      )
    )
    (query (range (start 194 39) (end 194 54)) (probe (position 194 39))
      (reference
        (source (document "d0") (qualified-name "Actions::SendAction::sentMessage"))
        (kind featureTyping) (ordinal 0) (authored-target "MessageTransfer")
        (range (start 194 39) (end 194 54))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::MessageTransfer") (range (start 28 1) (end 28 43)))
        )
      )
    )
    (query (range (start 213 44) (end 213 59)) (probe (position 213 44))
      (reference
        (source (document "d0") (qualified-name "Actions::AcceptMessageAction::acceptedMessage"))
        (kind featureTyping) (ordinal 0) (authored-target "MessageTransfer")
        (range (start 213 44) (end 213 59))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::MessageTransfer") (range (start 28 1) (end 28 43)))
        )
      )
    )
    (query (range (start 225 10) (end 225 25)) (probe (position 225 10))
      (reference
        (source (document "d0") (qualified-name "Actions::AcceptAction::"))
        (kind redefinition) (ordinal 0) (authored-target "acceptedMessage")
        (range (start 225 10) (end 225 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::AcceptMessageAction::acceptedMessage") (range (start 213 2) (end 213 155)))
        )
      )
    )
    (query (range (start 338 24) (end 338 39)) (probe (position 338 24))
      (reference
        (source (document "d0") (qualified-name "Actions::TransitionAction::acceptedMessage"))
        (kind featureTyping) (ordinal 0) (authored-target "MessageTransfer")
        (range (start 338 24) (end 338 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::MessageTransfer") (range (start 28 1) (end 28 43)))
        )
      )
    )
    (query (range (start 347 7) (end 347 22)) (probe (position 347 7))
      (reference
        (source (document "d0") (qualified-name "Actions::TransitionAction"))
        (kind bindSource) (ordinal 1) (authored-target "acceptedMessage")
        (range (start 347 7) (end 347 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::TransitionAction::acceptedMessage") (range (start 338 2) (end 338 147)))
        )
      )
    )
    (query (range (start 451 43) (end 451 58)) (probe (position 451 43))
      (reference
        (source (document "d0") (qualified-name "Actions::WhileLoopAction"))
        (kind specialization) (ordinal 5) (authored-target "LoopPerformance")
        (range (start 451 43) (end 451 58))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::LoopPerformance") (range (start 25 1) (end 25 53)))
        )
      )
    )
    (query (range (start 85 77) (end 85 93)) (probe (position 85 77))
      (reference
        (source (document "d0") (qualified-name "Actions::Action::terminateSubactions"))
        (kind subsetting) (ordinal 1) (authored-target "terminateActions")
        (range (start 85 77) (end 85 93))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::terminateActions") (range (start 259 1) (end 259 409)))
        )
      )
    )
    (query (range (start 164 63) (end 164 79)) (probe (position 164 63))
      (reference
        (source (document "d0") (qualified-name "Actions::Action::whileLoops"))
        (kind subsetting) (ordinal 1) (authored-target "whileLoopActions")
        (range (start 164 63) (end 164 79))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::whileLoopActions") (range (start 538 1) (end 538 177)))
        )
      )
    )
    (query (range (start 199 69) (end 199 85)) (probe (position 199 69))
      (reference
        (source (document "d0") (qualified-name "Actions::sendActions"))
        (kind subsetting) (ordinal 1) (authored-target "sendPerformances")
        (range (start 199 69) (end 199 85))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::sendPerformances") (range (start 17 1) (end 17 44)))
        )
      )
    )
    (query (range (start 213 26) (end 213 42)) (probe (position 213 26))
      (reference
        (source (document "d0") (qualified-name "Actions::AcceptMessageAction::acceptedMessage"))
        (kind redefinition) (ordinal 0) (authored-target "acceptedTransfer")
        (range (start 213 26) (end 213 42))
        (outcome (status unresolved))
      )
    )
    (query (range (start 288 42) (end 288 58)) (probe (position 288 42))
      (reference
        (source (document "d0") (qualified-name "Actions::MergeAction"))
        (kind specialization) (ordinal 5) (authored-target "MergePerformance")
        (range (start 288 42) (end 288 58))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::MergePerformance") (range (start 21 1) (end 21 54)))
        )
      )
    )
    (query (range (start 352 40) (end 352 56)) (probe (position 352 40))
      (reference
        (source (document "d0") (qualified-name "Actions::DecisionTransitionAction"))
        (kind specialization) (ordinal 4) (authored-target "TransitionAction")
        (range (start 352 40) (end 352 56))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::TransitionAction") (range (start 330 1) (end 330 714)))
        )
      )
    )
    (query (range (start 127 70) (end 127 87)) (probe (position 127 70))
      (reference
        (source (document "d0") (qualified-name "Actions::Action::transitions"))
        (kind subsetting) (ordinal 1) (authored-target "transitionActions")
        (range (start 127 70) (end 127 87))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::transitionActions") (range (start 364 1) (end 364 170)))
        )
      )
    )
    (query (range (start 141 70) (end 141 87)) (probe (position 141 70))
      (reference
        (source (document "d0") (qualified-name "Actions::Action::assignments"))
        (kind subsetting) (ordinal 1) (authored-target "assignmentActions")
        (range (start 141 70) (end 141 87))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::assignmentActions") (range (start 383 1) (end 383 428)))
        )
      )
    )
    (query (range (start 206 43) (end 206 60)) (probe (position 206 43))
      (reference
        (source (document "d0") (qualified-name "Actions::AcceptMessageAction"))
        (kind specialization) (ordinal 5) (authored-target "AcceptPerformance")
        (range (start 206 43) (end 206 60))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::AcceptPerformance") (range (start 18 1) (end 18 45)))
        )
      )
    )
    (query (range (start 346 18) (end 346 35)) (probe (position 346 18))
      (reference
        (source (document "d0") (qualified-name "Actions::TransitionAction"))
        (kind bindTarget) (ordinal 0) (authored-target "accepter::receiver")
        (range (start 346 18) (end 346 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 398 36) (end 398 53)) (probe (position 398 36))
      (reference
        (source (document "d0") (qualified-name "Actions::IfThenAction"))
        (kind specialization) (ordinal 5) (authored-target "IfThenPerformance")
        (range (start 398 36) (end 398 53))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::IfThenPerformance") (range (start 23 1) (end 23 55)))
        )
      )
    )
    (query (range (start 232 73) (end 232 91)) (probe (position 232 73))
      (reference
        (source (document "d0") (qualified-name "Actions::acceptActions"))
        (kind subsetting) (ordinal 1) (authored-target "acceptPerformances")
        (range (start 232 73) (end 232 91))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::acceptPerformances") (range (start 19 1) (end 19 46)))
        )
      )
    )
    (query (range (start 218 28) (end 218 47)) (probe (position 218 28))
      (reference
        (source (document "d0") (qualified-name "Actions::AcceptAction"))
        (kind specialization) (ordinal 2) (authored-target "AcceptMessageAction")
        (range (start 218 28) (end 218 47))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::AcceptMessageAction") (range (start 206 1) (end 206 417)))
        )
      )
    )
    (query (range (start 298 45) (end 298 64)) (probe (position 298 45))
      (reference
        (source (document "d0") (qualified-name "Actions::DecisionAction"))
        (kind specialization) (ordinal 5) (authored-target "DecisionPerformance")
        (range (start 298 45) (end 298 64))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::DecisionPerformance") (range (start 22 1) (end 22 57)))
        )
      )
    )
    (query (range (start 29 16) (end 29 36)) (probe (position 29 16))
      (reference
        (source (document "d0") (qualified-name "Actions::MessageAction"))
        (kind membershipImport) (ordinal 0) (authored-target "Flows::MessageAction")
        (range (start 29 16) (end 29 36))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 16) (end 9 37)) (probe (position 9 16))
      (reference
        (source (document "d0") (qualified-name "Actions::Natural"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Natural")
        (range (start 9 16) (end 9 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 330 49) (end 330 70)) (probe (position 330 49))
      (reference
        (source (document "d0") (qualified-name "Actions::TransitionAction"))
        (kind specialization) (ordinal 5) (authored-target "TransitionPerformance")
        (range (start 330 49) (end 330 70))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::TransitionPerformance") (range (start 26 1) (end 26 62)))
        )
      )
    )
    (query (range (start 409 46) (end 409 67)) (probe (position 409 46))
      (reference
        (source (document "d0") (qualified-name "Actions::IfThenElseAction"))
        (kind specialization) (ordinal 5) (authored-target "IfThenElsePerformance")
        (range (start 409 46) (end 409 67))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::IfThenElsePerformance") (range (start 24 1) (end 24 59)))
        )
      )
    )
    (query (range (start 8 16) (end 8 38)) (probe (position 8 16))
      (reference
        (source (document "d0") (qualified-name "Actions::Positive"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Positive")
        (range (start 8 16) (end 8 38))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 16) (end 10 39)) (probe (position 10 16))
      (reference
        (source (document "d0") (qualified-name "Actions::size"))
        (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::size")
        (range (start 10 16) (end 10 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 12 16) (end 12 39)) (probe (position 12 16))
      (reference
        (source (document "d0") (qualified-name "Actions::Occurrence"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::Occurrence")
        (range (start 12 16) (end 12 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 371 32) (end 371 55)) (probe (position 371 32))
      (reference
        (source (document "d0") (qualified-name "Actions::AssignmentAction"))
        (kind specialization) (ordinal 4) (authored-target "FeatureWritePerformance")
        (range (start 371 32) (end 371 55))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::FeatureWritePerformance") (range (start 20 1) (end 20 72)))
        )
      )
    )
    (query (range (start 347 25) (end 347 49)) (probe (position 347 25))
      (reference
        (source (document "d0") (qualified-name "Actions::TransitionAction"))
        (kind bindTarget) (ordinal 1) (authored-target "accepter::acceptedMessage")
        (range (start 347 25) (end 347 49))
        (outcome (status unresolved))
      )
    )
    (query (range (start 13 16) (end 13 41)) (probe (position 13 16))
      (reference
        (source (document "d0") (qualified-name "Actions::HappensWhile"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::HappensWhile")
        (range (start 13 16) (end 13 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 14 16) (end 14 41)) (probe (position 14 16))
      (reference
        (source (document "d0") (qualified-name "Actions::Performance"))
        (kind membershipImport) (ordinal 0) (authored-target "Performances::Performance")
        (range (start 14 16) (end 14 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 11 16) (end 11 42)) (probe (position 11 16))
      (reference
        (source (document "d0") (qualified-name "Actions::isEmpty"))
        (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::isEmpty")
        (range (start 11 16) (end 11 42))
        (outcome (status unresolved))
      )
    )
    (query (range (start 15 16) (end 15 42)) (probe (position 15 16))
      (reference
        (source (document "d0") (qualified-name "Actions::performances"))
        (kind membershipImport) (ordinal 0) (authored-target "Performances::performances")
        (range (start 15 16) (end 15 42))
        (outcome (status unresolved))
      )
    )
    (query (range (start 16 16) (end 16 42)) (probe (position 16 16))
      (reference
        (source (document "d0") (qualified-name "Actions::SendPerformance"))
        (kind membershipImport) (ordinal 0) (authored-target "Transfers::SendPerformance")
        (range (start 16 16) (end 16 42))
        (outcome (status unresolved))
      )
    )
    (query (range (start 28 16) (end 28 42)) (probe (position 28 16))
      (reference
        (source (document "d0") (qualified-name "Actions::MessageTransfer"))
        (kind membershipImport) (ordinal 0) (authored-target "Transfers::MessageTransfer")
        (range (start 28 16) (end 28 42))
        (outcome (status unresolved))
      )
    )
    (query (range (start 17 16) (end 17 43)) (probe (position 17 16))
      (reference
        (source (document "d0") (qualified-name "Actions::sendPerformances"))
        (kind membershipImport) (ordinal 0) (authored-target "Transfers::sendPerformances")
        (range (start 17 16) (end 17 43))
        (outcome (status unresolved))
      )
    )
    (query (range (start 229 17) (end 229 44)) (probe (position 229 17))
      (reference
        (source (document "d0") (qualified-name "Actions::AcceptAction"))
        (kind bindTarget) (ordinal 0) (authored-target "aState::aTransition::apayload")
        (range (start 229 17) (end 229 44))
        (outcome (status unresolved))
      )
    )
    (query (range (start 18 16) (end 18 44)) (probe (position 18 16))
      (reference
        (source (document "d0") (qualified-name "Actions::AcceptPerformance"))
        (kind membershipImport) (ordinal 0) (authored-target "Transfers::AcceptPerformance")
        (range (start 18 16) (end 18 44))
        (outcome (status unresolved))
      )
    )
    (query (range (start 30 16) (end 30 44)) (probe (position 30 16))
      (reference
        (source (document "d0") (qualified-name "Actions::destroy"))
        (kind membershipImport) (ordinal 0) (authored-target "OccurrenceFunctions::destroy")
        (range (start 30 16) (end 30 44))
        (outcome (status unresolved))
      )
    )
    (query (range (start 19 16) (end 19 45)) (probe (position 19 16))
      (reference
        (source (document "d0") (qualified-name "Actions::acceptPerformances"))
        (kind membershipImport) (ordinal 0) (authored-target "Transfers::acceptPerformances")
        (range (start 19 16) (end 19 45))
        (outcome (status unresolved))
      )
    )
    (query (range (start 349 28) (end 349 57)) (probe (position 349 28))
      (reference
        (source (document "d0") (qualified-name "Actions::TransitionAction::effect"))
        (kind redefinition) (ordinal 0) (authored-target "TransitionPerformance::effect")
        (range (start 349 28) (end 349 57))
        (outcome (status unresolved))
      )
    )
    (query (range (start 352 58) (end 352 87)) (probe (position 352 58))
      (reference
        (source (document "d0") (qualified-name "Actions::DecisionTransitionAction"))
        (kind specialization) (ordinal 5) (authored-target "NonStateTransitionPerformance")
        (range (start 352 58) (end 352 87))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Actions::NonStateTransitionPerformance") (range (start 27 1) (end 27 70)))
        )
      )
    )
    (query (range (start 40 35) (end 40 65)) (probe (position 40 35))
      (reference
        (source (document "d0") (qualified-name "Actions::Action::incomingTransfers"))
        (kind redefinition) (ordinal 0) (authored-target "Performance::incomingTransfers")
        (range (start 40 35) (end 40 65))
        (outcome (status unresolved))
      )
    )
    (query (range (start 25 16) (end 25 52)) (probe (position 25 16))
      (reference
        (source (document "d0") (qualified-name "Actions::LoopPerformance"))
        (kind membershipImport) (ordinal 0) (authored-target "ControlPerformances::LoopPerformance")
        (range (start 25 16) (end 25 52))
        (outcome (status unresolved))
      )
    )
    (query (range (start 21 16) (end 21 53)) (probe (position 21 16))
      (reference
        (source (document "d0") (qualified-name "Actions::MergePerformance"))
        (kind membershipImport) (ordinal 0) (authored-target "ControlPerformances::MergePerformance")
        (range (start 21 16) (end 21 53))
        (outcome (status unresolved))
      )
    )
    (query (range (start 23 16) (end 23 54)) (probe (position 23 16))
      (reference
        (source (document "d0") (qualified-name "Actions::IfThenPerformance"))
        (kind membershipImport) (ordinal 0) (authored-target "ControlPerformances::IfThenPerformance")
        (range (start 23 16) (end 23 54))
        (outcome (status unresolved))
      )
    )
    (query (range (start 22 16) (end 22 56)) (probe (position 22 16))
      (reference
        (source (document "d0") (qualified-name "Actions::DecisionPerformance"))
        (kind membershipImport) (ordinal 0) (authored-target "ControlPerformances::DecisionPerformance")
        (range (start 22 16) (end 22 56))
        (outcome (status unresolved))
      )
    )
    (query (range (start 24 16) (end 24 58)) (probe (position 24 16))
      (reference
        (source (document "d0") (qualified-name "Actions::IfThenElsePerformance"))
        (kind membershipImport) (ordinal 0) (authored-target "ControlPerformances::IfThenElsePerformance")
        (range (start 24 16) (end 24 58))
        (outcome (status unresolved))
      )
    )
    (query (range (start 26 16) (end 26 61)) (probe (position 26 16))
      (reference
        (source (document "d0") (qualified-name "Actions::TransitionPerformance"))
        (kind membershipImport) (ordinal 0) (authored-target "TransitionPerformances::TransitionPerformance")
        (range (start 26 16) (end 26 61))
        (outcome (status unresolved))
      )
    )
    (query (range (start 27 16) (end 27 69)) (probe (position 27 16))
      (reference
        (source (document "d0") (qualified-name "Actions::NonStateTransitionPerformance"))
        (kind membershipImport) (ordinal 0) (authored-target "TransitionPerformances::NonStateTransitionPerformance")
        (range (start 27 16) (end 27 69))
        (outcome (status unresolved))
      )
    )
    (query (range (start 20 16) (end 20 71)) (probe (position 20 16))
      (reference
        (source (document "d0") (qualified-name "Actions::FeatureWritePerformance"))
        (kind membershipImport) (ordinal 0) (authored-target "FeatureReferencingPerformances::FeatureWritePerformance")
        (range (start 20 16) (end 20 71))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
