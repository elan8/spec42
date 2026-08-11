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
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "1a5e449c917f6f8a1f5ba9e60eccfaf7fa82683803e2dc8d4930f906cfc14cd0") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Actions"))) (kind "package") (name "Actions") (declared-name "Actions") (range (start (line 0) (character 0)) (end (line 0) (character 14635))))
    (element (id (node (document "d0") (qualified-name "Actions::AcceptAction"))) (kind "action def") (name "AcceptAction") (declared-name "AcceptAction") (range (start (line 218) (character 1)) (end (line 218) (character 535))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "AcceptMessageAction") (range none)) (specializes (reference "AcceptMessageAction") (range none)) (specializes (reference "AcceptMessageAction") (range (start (line 218) (character 28)) (end (line 218) (character 47)))))))
    (element (id (node (document "d0") (qualified-name "Actions::AcceptAction::"))) (kind "ref") (name "") (range (start (line 225) (character 2)) (end (line 225) (character 72))) (parent (node (document "d0") (qualified-name "Actions::AcceptAction"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "acceptedMessage") (range (start (line 225) (character 10)) (end (line 225) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "Actions::AcceptAction::_documentation"))) (kind "documentation") (name "") (range (start (line 218) (character 1)) (end (line 218) (character 535))) (parent (node (document "d0") (qualified-name "Actions::AcceptAction"))))
    (element (id (node (document "d0") (qualified-name "Actions::AcceptAction::aState"))) (kind "state") (name "aState") (declared-name "aState") (range (start (line 226) (character 2)) (end (line 226) (character 109))) (parent (node (document "d0") (qualified-name "Actions::AcceptAction"))))
    (element (id (node (document "d0") (qualified-name "Actions::AcceptAction::aState::aTransition"))) (kind "transition") (name "aTransition") (declared-name "aTransition") (range (start (line 227) (character 3)) (end (line 227) (character 87))) (parent (node (document "d0") (qualified-name "Actions::AcceptAction::aState"))))
    (element (id (node (document "d0") (qualified-name "Actions::AcceptAction::aState::aTransition::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 227) (character 3)) (end (line 227) (character 87))) (parent (node (document "d0") (qualified-name "Actions::AcceptAction::aState::aTransition"))))
    (element (id (node (document "d0") (qualified-name "Actions::AcceptMessageAction"))) (kind "action def") (name "AcceptMessageAction") (declared-name "AcceptMessageAction") (range (start (line 206) (character 1)) (end (line 206) (character 417))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Action") (range none)) (specializes (reference "AcceptPerformance") (range none)) (specializes (reference "Action") (range none)) (specializes (reference "AcceptPerformance") (range none)) (specializes (reference "Action") (range (start (line 206) (character 35)) (end (line 206) (character 41)))) (specializes (reference "AcceptPerformance") (range (start (line 206) (character 43)) (end (line 206) (character 60)))))))
    (element (id (node (document "d0") (qualified-name "Actions::AcceptMessageAction::_documentation"))) (kind "documentation") (name "") (range (start (line 206) (character 1)) (end (line 206) (character 417))) (parent (node (document "d0") (qualified-name "Actions::AcceptMessageAction"))))
    (element (id (node (document "d0") (qualified-name "Actions::AcceptMessageAction::acceptedMessage"))) (kind "ref") (name "acceptedMessage") (declared-name "acceptedMessage") (range (start (line 213) (character 2)) (end (line 213) (character 155))) (parent (node (document "d0") (qualified-name "Actions::AcceptMessageAction"))) (authored (membership (kind Feature)) (relationships (typing (reference "MessageTransfer") (range (start (line 213) (character 44)) (end (line 213) (character 59)))) (typing (reference "MessageAction") (range (start (line 213) (character 61)) (end (line 213) (character 74)))) (redefinition (reference "acceptedTransfer") (range (start (line 213) (character 26)) (end (line 213) (character 42)))))))
    (element (id (node (document "d0") (qualified-name "Actions::AcceptMessageAction::acceptedMessage::MessageTransfer::payload, MessageAction::payload"))) (kind "in out parameter") (name "MessageTransfer::payload, MessageAction::payload") (declared-name "MessageTransfer::payload, MessageAction::payload") (range (start (line 214) (character 12)) (end (line 214) (character 68))) (parent (node (document "d0") (qualified-name "Actions::AcceptMessageAction::acceptedMessage"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Actions::AcceptMessageAction::payload"))) (kind "in out parameter") (name "payload") (declared-name "payload") (range (start (line 212) (character 2)) (end (line 212) (character 20))) (parent (node (document "d0") (qualified-name "Actions::AcceptMessageAction"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Actions::AcceptPerformance"))) (kind "import") (name "AcceptPerformance") (declared-name "AcceptPerformance") (range (start (line 18) (character 1)) (end (line 18) (character 45))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::AcceptPerformance") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 18) (character 16)) (end (line 18) (character 44))))))
    (element (id (node (document "d0") (qualified-name "Actions::Action"))) (kind "action def") (name "Action") (declared-name "Action") (range (start (line 32) (character 1)) (end (line 32) (character 3407))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Performance") (range none)) (specializes (reference "Performance") (range none)) (specializes (reference "Performance") (range (start (line 32) (character 31)) (end (line 32) (character 42)))) (perform (reference "Actions::Action::start") (range none)) (perform (reference "Actions::Action::done") (range none)) (perform (reference "Actions::Action::subactions") (range none)) (perform (reference "Actions::Action::sendSubactions") (range none)) (perform (reference "Actions::Action::acceptSubactions") (range none)) (perform (reference "Actions::Action::terminateSubactions") (range none)) (perform (reference "Actions::Action::controls") (range none)) (perform (reference "Actions::Action::merges") (range none)) (perform (reference "Actions::Action::decisions") (range none)) (perform (reference "Actions::Action::joins") (range none)) (perform (reference "Actions::Action::forks") (range none)) (perform (reference "Actions::Action::transitions") (range none)) (perform (reference "Actions::Action::decisionTransitions") (range none)) (perform (reference "Actions::Action::assignments") (range none)) (perform (reference "Actions::Action::ifSubactions") (range none)) (perform (reference "Actions::Action::loops") (range none)) (perform (reference "Actions::Action::whileLoops") (range none)) (perform (reference "Actions::Action::forLoops") (range none)))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::_documentation"))) (kind "documentation") (name "") (range (start (line 32) (character 1)) (end (line 32) (character 3407))) (parent (node (document "d0") (qualified-name "Actions::Action"))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::acceptSubactions"))) (kind "action") (name "acceptSubactions") (declared-name "acceptSubactions") (range (start (line 78) (character 2)) (end (line 78) (character 161))) (parent (node (document "d0") (qualified-name "Actions::Action"))) (authored (membership (kind Feature)) (relationships (typing (reference "AcceptAction") (range none)) (subsetting (reference "subactions") (range (start (line 78) (character 49)) (end (line 78) (character 59)))) (subsetting (reference "acceptActions") (range (start (line 78) (character 61)) (end (line 78) (character 74)))))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::acceptSubactions::_documentation"))) (kind "documentation") (name "") (range (start (line 78) (character 2)) (end (line 78) (character 161))) (parent (node (document "d0") (qualified-name "Actions::Action::acceptSubactions"))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::assignments"))) (kind "action") (name "assignments") (declared-name "assignments") (range (start (line 141) (character 2)) (end (line 141) (character 197))) (parent (node (document "d0") (qualified-name "Actions::Action"))) (authored (membership (kind Feature)) (relationships (typing (reference "AssignmentAction") (range none)) (subsetting (reference "subactions") (range (start (line 141) (character 58)) (end (line 141) (character 68)))) (subsetting (reference "assignmentActions") (range (start (line 141) (character 70)) (end (line 141) (character 87)))))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::assignments::_documentation"))) (kind "documentation") (name "") (range (start (line 141) (character 2)) (end (line 141) (character 197))) (parent (node (document "d0") (qualified-name "Actions::Action::assignments"))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::assignments::target"))) (kind "in out parameter") (name "target") (declared-name "target") (range (start (line 147) (character 4)) (end (line 147) (character 14))) (parent (node (document "d0") (qualified-name "Actions::Action::assignments"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::controls"))) (kind "action") (name "controls") (declared-name "controls") (range (start (line 92) (character 2)) (end (line 92) (character 149))) (parent (node (document "d0") (qualified-name "Actions::Action"))) (authored (membership (kind Feature)) (relationships (typing (reference "ControlAction") (range none)) (subsetting (reference "subactions") (range (start (line 92) (character 52)) (end (line 92) (character 62)))))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::controls::_documentation"))) (kind "documentation") (name "") (range (start (line 92) (character 2)) (end (line 92) (character 149))) (parent (node (document "d0") (qualified-name "Actions::Action::controls"))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::decisionTransitions"))) (kind "action") (name "decisionTransitions") (declared-name "decisionTransitions") (range (start (line 134) (character 2)) (end (line 134) (character 184))) (parent (node (document "d0") (qualified-name "Actions::Action"))) (authored (membership (kind Feature)) (relationships (typing (reference "DecisionTransitionAction") (range none)) (subsetting (reference "transitions") (range (start (line 134) (character 74)) (end (line 134) (character 85)))))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::decisionTransitions::_documentation"))) (kind "documentation") (name "") (range (start (line 134) (character 2)) (end (line 134) (character 184))) (parent (node (document "d0") (qualified-name "Actions::Action::decisionTransitions"))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::decisions"))) (kind "action") (name "decisions") (declared-name "decisions") (range (start (line 106) (character 2)) (end (line 106) (character 142))) (parent (node (document "d0") (qualified-name "Actions::Action"))) (authored (membership (kind Feature)) (relationships (typing (reference "DecisionAction") (range none)) (subsetting (reference "controls") (range (start (line 106) (character 48)) (end (line 106) (character 56)))))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::decisions::_documentation"))) (kind "documentation") (name "") (range (start (line 106) (character 2)) (end (line 106) (character 142))) (parent (node (document "d0") (qualified-name "Actions::Action::decisions"))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::done"))) (kind "action") (name "done") (declared-name "done") (range (start (line 49) (character 2)) (end (line 49) (character 99))) (parent (node (document "d0") (qualified-name "Actions::Action"))) (authored (membership (kind Feature)) (relationships (typing (reference "Action") (range none)) (redefinition (reference "endShot") (range (start (line 49) (character 26)) (end (line 49) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::done::_documentation"))) (kind "documentation") (name "") (range (start (line 49) (character 2)) (end (line 49) (character 99))) (parent (node (document "d0") (qualified-name "Actions::Action::done"))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::forLoops"))) (kind "action") (name "forLoops") (declared-name "forLoops") (range (start (line 171) (character 2)) (end (line 171) (character 155))) (parent (node (document "d0") (qualified-name "Actions::Action"))) (authored (membership (kind Feature)) (relationships (typing (reference "ForLoopAction") (range none)) (subsetting (reference "loops") (range (start (line 171) (character 52)) (end (line 171) (character 57)))) (subsetting (reference "forLoopActions") (range (start (line 171) (character 59)) (end (line 171) (character 73)))))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::forLoops::_documentation"))) (kind "documentation") (name "") (range (start (line 171) (character 2)) (end (line 171) (character 155))) (parent (node (document "d0") (qualified-name "Actions::Action::forLoops"))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::forks"))) (kind "action") (name "forks") (declared-name "forks") (range (start (line 120) (character 2)) (end (line 120) (character 130))) (parent (node (document "d0") (qualified-name "Actions::Action"))) (authored (membership (kind Feature)) (relationships (typing (reference "ForkAction") (range none)) (subsetting (reference "controls") (range (start (line 120) (character 40)) (end (line 120) (character 48)))))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::forks::_documentation"))) (kind "documentation") (name "") (range (start (line 120) (character 2)) (end (line 120) (character 130))) (parent (node (document "d0") (qualified-name "Actions::Action::forks"))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::ifSubactions"))) (kind "action") (name "ifSubactions") (declared-name "ifSubactions") (range (start (line 150) (character 2)) (end (line 150) (character 196))) (parent (node (document "d0") (qualified-name "Actions::Action"))) (authored (membership (kind Feature)) (relationships (typing (reference "IfThenAction") (range none)) (subsetting (reference "subactions") (range (start (line 150) (character 55)) (end (line 150) (character 65)))) (subsetting (reference "ifThenActions") (range (start (line 150) (character 67)) (end (line 150) (character 80)))))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::ifSubactions::_documentation"))) (kind "documentation") (name "") (range (start (line 150) (character 2)) (end (line 150) (character 196))) (parent (node (document "d0") (qualified-name "Actions::Action::ifSubactions"))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::incomingTransfers"))) (kind "ref") (name "incomingTransfers") (declared-name "incomingTransfers") (range (start (line 40) (character 2)) (end (line 40) (character 66))) (parent (node (document "d0") (qualified-name "Actions::Action"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "Performance::incomingTransfers") (range (start (line 40) (character 35)) (end (line 40) (character 65)))))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::joins"))) (kind "action") (name "joins") (declared-name "joins") (range (start (line 113) (character 2)) (end (line 113) (character 130))) (parent (node (document "d0") (qualified-name "Actions::Action"))) (authored (membership (kind Feature)) (relationships (typing (reference "JoinAction") (range none)) (subsetting (reference "controls") (range (start (line 113) (character 40)) (end (line 113) (character 48)))))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::joins::_documentation"))) (kind "documentation") (name "") (range (start (line 113) (character 2)) (end (line 113) (character 130))) (parent (node (document "d0") (qualified-name "Actions::Action::joins"))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::loops"))) (kind "action") (name "loops") (declared-name "loops") (range (start (line 157) (character 2)) (end (line 157) (character 153))) (parent (node (document "d0") (qualified-name "Actions::Action"))) (authored (membership (kind Feature)) (relationships (typing (reference "LoopAction") (range none)) (subsetting (reference "subactions") (range (start (line 157) (character 46)) (end (line 157) (character 56)))) (subsetting (reference "loopActions") (range (start (line 157) (character 58)) (end (line 157) (character 69)))))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::loops::_documentation"))) (kind "documentation") (name "") (range (start (line 157) (character 2)) (end (line 157) (character 153))) (parent (node (document "d0") (qualified-name "Actions::Action::loops"))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::merges"))) (kind "action") (name "merges") (declared-name "merges") (range (start (line 99) (character 2)) (end (line 99) (character 139))) (parent (node (document "d0") (qualified-name "Actions::Action"))) (authored (membership (kind Feature)) (relationships (typing (reference "MergeAction") (range none)) (subsetting (reference "controls") (range (start (line 99) (character 48)) (end (line 99) (character 56)))))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::merges::_documentation"))) (kind "documentation") (name "") (range (start (line 99) (character 2)) (end (line 99) (character 139))) (parent (node (document "d0") (qualified-name "Actions::Action::merges"))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::self"))) (kind "ref") (name "self") (declared-name "self") (range (start (line 39) (character 2)) (end (line 39) (character 48))) (parent (node (document "d0") (qualified-name "Actions::Action"))) (authored (membership (kind Feature)) (relationships (typing (reference "Action") (range (start (line 39) (character 19)) (end (line 39) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::sendSubactions"))) (kind "action") (name "sendSubactions") (declared-name "sendSubactions") (range (start (line 71) (character 2)) (end (line 71) (character 153))) (parent (node (document "d0") (qualified-name "Actions::Action"))) (authored (membership (kind Feature)) (relationships (typing (reference "SendAction") (range none)) (subsetting (reference "subactions") (range (start (line 71) (character 45)) (end (line 71) (character 55)))) (subsetting (reference "sendActions") (range (start (line 71) (character 57)) (end (line 71) (character 68)))))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::sendSubactions::_documentation"))) (kind "documentation") (name "") (range (start (line 71) (character 2)) (end (line 71) (character 153))) (parent (node (document "d0") (qualified-name "Actions::Action::sendSubactions"))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::start"))) (kind "action") (name "start") (declared-name "start") (range (start (line 42) (character 2)) (end (line 42) (character 105))) (parent (node (document "d0") (qualified-name "Actions::Action"))) (authored (membership (kind Feature)) (relationships (typing (reference "Action") (range none)) (redefinition (reference "startShot") (range (start (line 42) (character 27)) (end (line 42) (character 36)))))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::start::_documentation"))) (kind "documentation") (name "") (range (start (line 42) (character 2)) (end (line 42) (character 105))) (parent (node (document "d0") (qualified-name "Actions::Action::start"))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::subactions"))) (kind "action") (name "subactions") (declared-name "subactions") (range (start (line 56) (character 2)) (end (line 56) (character 376))) (parent (node (document "d0") (qualified-name "Actions::Action"))) (authored (membership (kind Feature)) (relationships (typing (reference "Action") (range none)) (subsetting (reference "actions") (range (start (line 56) (character 37)) (end (line 56) (character 44)))) (subsetting (reference "subperformances") (range (start (line 56) (character 46)) (end (line 56) (character 61)))))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::subactions::_documentation"))) (kind "documentation") (name "") (range (start (line 56) (character 2)) (end (line 56) (character 376))) (parent (node (document "d0") (qualified-name "Actions::Action::subactions"))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::subactions::occurrence"))) (kind "ref") (name "occurrence") (declared-name "occurrence") (range (start (line 62) (character 3)) (end (line 62) (character 225))) (parent (node (document "d0") (qualified-name "Actions::Action::subactions"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "Action::this") (range (start (line 62) (character 22)) (end (line 62) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::subactions::occurrence::_documentation"))) (kind "documentation") (name "") (range (start (line 62) (character 3)) (end (line 62) (character 225))) (parent (node (document "d0") (qualified-name "Actions::Action::subactions::occurrence"))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::terminateSubactions"))) (kind "action") (name "terminateSubactions") (declared-name "terminateSubactions") (range (start (line 85) (character 2)) (end (line 85) (character 182))) (parent (node (document "d0") (qualified-name "Actions::Action"))) (authored (membership (kind Feature)) (relationships (typing (reference "TerminateAction") (range none)) (subsetting (reference "subactions") (range (start (line 85) (character 65)) (end (line 85) (character 75)))) (subsetting (reference "terminateActions") (range (start (line 85) (character 77)) (end (line 85) (character 93)))))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::terminateSubactions::_documentation"))) (kind "documentation") (name "") (range (start (line 85) (character 2)) (end (line 85) (character 182))) (parent (node (document "d0") (qualified-name "Actions::Action::terminateSubactions"))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::transitions"))) (kind "action") (name "transitions") (declared-name "transitions") (range (start (line 127) (character 2)) (end (line 127) (character 178))) (parent (node (document "d0") (qualified-name "Actions::Action"))) (authored (membership (kind Feature)) (relationships (typing (reference "TransitionAction") (range none)) (subsetting (reference "subactions") (range (start (line 127) (character 58)) (end (line 127) (character 68)))) (subsetting (reference "transitionActions") (range (start (line 127) (character 70)) (end (line 127) (character 87)))))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::transitions::_documentation"))) (kind "documentation") (name "") (range (start (line 127) (character 2)) (end (line 127) (character 178))) (parent (node (document "d0") (qualified-name "Actions::Action::transitions"))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::whileLoops"))) (kind "action") (name "whileLoops") (declared-name "whileLoops") (range (start (line 164) (character 2)) (end (line 164) (character 163))) (parent (node (document "d0") (qualified-name "Actions::Action"))) (authored (membership (kind Feature)) (relationships (typing (reference "WhileLoopAction") (range none)) (subsetting (reference "loops") (range (start (line 164) (character 56)) (end (line 164) (character 61)))) (subsetting (reference "whileLoopActions") (range (start (line 164) (character 63)) (end (line 164) (character 79)))))))
    (element (id (node (document "d0") (qualified-name "Actions::Action::whileLoops::_documentation"))) (kind "documentation") (name "") (range (start (line 164) (character 2)) (end (line 164) (character 163))) (parent (node (document "d0") (qualified-name "Actions::Action::whileLoops"))))
    (element (id (node (document "d0") (qualified-name "Actions::Anything"))) (kind "import") (name "Anything") (declared-name "Anything") (range (start (line 7) (character 1)) (end (line 7) (character 31))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Import) (visibility "private") (import (reference "Base::Anything") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 16)) (end (line 7) (character 30))))))
    (element (id (node (document "d0") (qualified-name "Actions::AssignmentAction"))) (kind "action def") (name "AssignmentAction") (declared-name "AssignmentAction") (range (start (line 371) (character 1)) (end (line 371) (character 387))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "FeatureWritePerformance") (range none)) (specializes (reference "Action") (range none)) (specializes (reference "FeatureWritePerformance") (range none)) (specializes (reference "Action") (range none)) (specializes (reference "FeatureWritePerformance") (range (start (line 371) (character 32)) (end (line 371) (character 55)))) (specializes (reference "Action") (range (start (line 371) (character 57)) (end (line 371) (character 63)))))))
    (element (id (node (document "d0") (qualified-name "Actions::AssignmentAction::_documentation"))) (kind "documentation") (name "") (range (start (line 371) (character 1)) (end (line 371) (character 387))) (parent (node (document "d0") (qualified-name "Actions::AssignmentAction"))))
    (element (id (node (document "d0") (qualified-name "Actions::AssignmentAction::replacementValues"))) (kind "in out parameter") (name "replacementValues") (declared-name "replacementValues") (range (start (line 380) (character 2)) (end (line 380) (character 53))) (parent (node (document "d0") (qualified-name "Actions::AssignmentAction"))) (authored (relationships (typing (reference "replacementValues : Anything[0..*] nonunique") (range none)))))
    (element (id (node (document "d0") (qualified-name "Actions::AssignmentAction::target"))) (kind "in out parameter") (name "target") (declared-name "target") (range (start (line 379) (character 2)) (end (line 379) (character 28))) (parent (node (document "d0") (qualified-name "Actions::AssignmentAction"))) (authored (relationships (typing (reference "target : Occurrence[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "Actions::ControlAction"))) (kind "action def") (name "ControlAction") (declared-name "ControlAction") (range (start (line 274) (character 1)) (end (line 274) (character 240))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Action") (range none)) (specializes (reference "Action") (range none)) (specializes (reference "Action") (range (start (line 274) (character 38)) (end (line 274) (character 44)))))))
    (element (id (node (document "d0") (qualified-name "Actions::ControlAction::_documentation"))) (kind "documentation") (name "") (range (start (line 274) (character 1)) (end (line 274) (character 240))) (parent (node (document "d0") (qualified-name "Actions::ControlAction"))))
    (element (id (node (document "d0") (qualified-name "Actions::DecisionAction"))) (kind "action def") (name "DecisionAction") (declared-name "DecisionAction") (range (start (line 298) (character 1)) (end (line 298) (character 445))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ControlAction") (range none)) (specializes (reference "DecisionPerformance") (range none)) (specializes (reference "ControlAction") (range none)) (specializes (reference "DecisionPerformance") (range none)) (specializes (reference "ControlAction") (range (start (line 298) (character 30)) (end (line 298) (character 43)))) (specializes (reference "DecisionPerformance") (range (start (line 298) (character 45)) (end (line 298) (character 64)))))))
    (element (id (node (document "d0") (qualified-name "Actions::DecisionAction::_documentation"))) (kind "documentation") (name "") (range (start (line 298) (character 1)) (end (line 298) (character 445))) (parent (node (document "d0") (qualified-name "Actions::DecisionAction"))))
    (element (id (node (document "d0") (qualified-name "Actions::DecisionPerformance"))) (kind "import") (name "DecisionPerformance") (declared-name "DecisionPerformance") (range (start (line 22) (character 1)) (end (line 22) (character 57))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlPerformances::DecisionPerformance") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 22) (character 16)) (end (line 22) (character 56))))))
    (element (id (node (document "d0") (qualified-name "Actions::DecisionTransitionAction"))) (kind "action def") (name "DecisionTransitionAction") (declared-name "DecisionTransitionAction") (range (start (line 352) (character 1)) (end (line 352) (character 410))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "TransitionAction") (range none)) (specializes (reference "NonStateTransitionPerformance") (range none)) (specializes (reference "TransitionAction") (range none)) (specializes (reference "NonStateTransitionPerformance") (range none)) (specializes (reference "TransitionAction") (range (start (line 352) (character 40)) (end (line 352) (character 56)))) (specializes (reference "NonStateTransitionPerformance") (range (start (line 352) (character 58)) (end (line 352) (character 87)))))))
    (element (id (node (document "d0") (qualified-name "Actions::DecisionTransitionAction::"))) (kind "ref") (name "") (range (start (line 360) (character 2)) (end (line 360) (character 32))) (parent (node (document "d0") (qualified-name "Actions::DecisionTransitionAction"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "accepter") (range (start (line 360) (character 17)) (end (line 360) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "Actions::DecisionTransitionAction::#ref"))) (kind "ref") (name "") (range (start (line 361) (character 2)) (end (line 361) (character 30))) (parent (node (document "d0") (qualified-name "Actions::DecisionTransitionAction"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "effect") (range (start (line 361) (character 17)) (end (line 361) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "Actions::DecisionTransitionAction::_documentation"))) (kind "documentation") (name "") (range (start (line 352) (character 1)) (end (line 352) (character 410))) (parent (node (document "d0") (qualified-name "Actions::DecisionTransitionAction"))))
    (element (id (node (document "d0") (qualified-name "Actions::FeatureWritePerformance"))) (kind "import") (name "FeatureWritePerformance") (declared-name "FeatureWritePerformance") (range (start (line 20) (character 1)) (end (line 20) (character 72))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Import) (visibility "private") (import (reference "FeatureReferencingPerformances::FeatureWritePerformance") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 20) (character 16)) (end (line 20) (character 71))))))
    (element (id (node (document "d0") (qualified-name "Actions::ForLoopAction"))) (kind "action def") (name "ForLoopAction") (declared-name "ForLoopAction") (range (start (line 484) (character 1)) (end (line 484) (character 959))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "LoopAction") (range none)) (specializes (reference "LoopAction") (range none)) (specializes (reference "LoopAction") (range (start (line 484) (character 29)) (end (line 484) (character 39)))) (perform (reference "Actions::ForLoopAction::initialization") (range none)) (perform (reference "Actions::ForLoopAction::whileLoop") (range none)))))
    (element (id (node (document "d0") (qualified-name "Actions::ForLoopAction::_assign"))) (kind "assign") (name "assign") (declared-name "assign") (range (start (line 522) (character 3)) (end (line 522) (character 21))) (parent (node (document "d0") (qualified-name "Actions::ForLoopAction"))))
    (element (id (node (document "d0") (qualified-name "Actions::ForLoopAction::_documentation"))) (kind "documentation") (name "") (range (start (line 484) (character 1)) (end (line 484) (character 959))) (parent (node (document "d0") (qualified-name "Actions::ForLoopAction"))))
    (element (id (node (document "d0") (qualified-name "Actions::ForLoopAction::_while"))) (kind "while") (name "while") (declared-name "while") (range (start (line 524) (character 3)) (end (line 524) (character 124))) (parent (node (document "d0") (qualified-name "Actions::ForLoopAction"))) (authored (relationships (flow (reference "Actions::ForLoopAction::_while::body") (range none)))))
    (element (id (node (document "d0") (qualified-name "Actions::ForLoopAction::_while::_assign"))) (kind "assign") (name "assign") (declared-name "assign") (range (start (line 525) (character 4)) (end (line 525) (character 30))) (parent (node (document "d0") (qualified-name "Actions::ForLoopAction::_while"))))
    (element (id (node (document "d0") (qualified-name "Actions::ForLoopAction::_while::_assign#assign"))) (kind "assign") (name "assign") (declared-name "assign") (range (start (line 527) (character 4)) (end (line 527) (character 35))) (parent (node (document "d0") (qualified-name "Actions::ForLoopAction::_while"))))
    (element (id (node (document "d0") (qualified-name "Actions::ForLoopAction::body"))) (kind "in out parameter") (name "body") (declared-name "body") (range (start (line 506) (character 2)) (end (line 506) (character 108))) (parent (node (document "d0") (qualified-name "Actions::ForLoopAction"))) (authored (relationships (typing (reference "action") (range none)))))
    (element (id (node (document "d0") (qualified-name "Actions::ForLoopAction::index : Positive"))) (kind "action body decl") (name "index : Positive") (declared-name "index : Positive") (range (start (line 513) (character 2)) (end (line 513) (character 160))) (parent (node (document "d0") (qualified-name "Actions::ForLoopAction"))))
    (element (id (node (document "d0") (qualified-name "Actions::ForLoopAction::initialization"))) (kind "action") (name "initialization") (declared-name "initialization") (range (start (line 521) (character 2)) (end (line 521) (character 35))) (parent (node (document "d0") (qualified-name "Actions::ForLoopAction"))))
    (element (id (node (document "d0") (qualified-name "Actions::ForLoopAction::ref"))) (kind "in out parameter") (name "ref") (declared-name "ref") (range (start (line 499) (character 2)) (end (line 499) (character 97))) (parent (node (document "d0") (qualified-name "Actions::ForLoopAction"))) (authored (relationships (typing (reference "ref seq") (range none)))))
    (element (id (node (document "d0") (qualified-name "Actions::ForLoopAction::var"))) (kind "ref") (name "var") (declared-name "var") (range (start (line 491) (character 2)) (end (line 491) (character 163))) (parent (node (document "d0") (qualified-name "Actions::ForLoopAction"))) (authored (membership (kind Feature)) (relationships (typing (reference "seq") (range (start (line 491) (character 29)) (end (line 491) (character 32)))))))
    (element (id (node (document "d0") (qualified-name "Actions::ForLoopAction::var::_documentation"))) (kind "documentation") (name "") (range (start (line 491) (character 2)) (end (line 491) (character 163))) (parent (node (document "d0") (qualified-name "Actions::ForLoopAction::var"))))
    (element (id (node (document "d0") (qualified-name "Actions::ForLoopAction::whileLoop"))) (kind "action") (name "whileLoop") (declared-name "whileLoop") (range (start (line 523) (character 2)) (end (line 523) (character 35))) (parent (node (document "d0") (qualified-name "Actions::ForLoopAction"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Actions::ForkAction"))) (kind "action def") (name "ForkAction") (declared-name "ForkAction") (range (start (line 320) (character 1)) (end (line 320) (character 251))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ControlAction") (range none)) (specializes (reference "ControlAction") (range none)) (specializes (reference "ControlAction") (range (start (line 320) (character 26)) (end (line 320) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "Actions::ForkAction::_documentation"))) (kind "documentation") (name "") (range (start (line 320) (character 1)) (end (line 320) (character 251))) (parent (node (document "d0") (qualified-name "Actions::ForkAction"))))
    (element (id (node (document "d0") (qualified-name "Actions::HappensWhile"))) (kind "import") (name "HappensWhile") (declared-name "HappensWhile") (range (start (line 13) (character 1)) (end (line 13) (character 42))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::HappensWhile") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 13) (character 16)) (end (line 13) (character 41))))))
    (element (id (node (document "d0") (qualified-name "Actions::IfThenAction"))) (kind "action def") (name "IfThenAction") (declared-name "IfThenAction") (range (start (line 398) (character 1)) (end (line 398) (character 248))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Action") (range none)) (specializes (reference "IfThenPerformance") (range none)) (specializes (reference "Action") (range none)) (specializes (reference "IfThenPerformance") (range none)) (specializes (reference "Action") (range (start (line 398) (character 28)) (end (line 398) (character 34)))) (specializes (reference "IfThenPerformance") (range (start (line 398) (character 36)) (end (line 398) (character 53)))))))
    (element (id (node (document "d0") (qualified-name "Actions::IfThenAction::_documentation"))) (kind "documentation") (name "") (range (start (line 398) (character 1)) (end (line 398) (character 248))) (parent (node (document "d0") (qualified-name "Actions::IfThenAction"))))
    (element (id (node (document "d0") (qualified-name "Actions::IfThenAction::action"))) (kind "in out parameter") (name "action") (declared-name "action") (range (start (line 406) (character 2)) (end (line 406) (character 29))) (parent (node (document "d0") (qualified-name "Actions::IfThenAction"))) (authored (relationships (typing (reference "action thenClause[0..1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "Actions::IfThenAction::ifTest"))) (kind "in out parameter") (name "ifTest") (declared-name "ifTest") (range (start (line 405) (character 2)) (end (line 405) (character 15))) (parent (node (document "d0") (qualified-name "Actions::IfThenAction"))) (authored (relationships (typing (reference "ifTest[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "Actions::IfThenElseAction"))) (kind "action def") (name "IfThenElseAction") (declared-name "IfThenElseAction") (range (start (line 409) (character 1)) (end (line 409) (character 329))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "IfThenAction") (range none)) (specializes (reference "IfThenElsePerformance") (range none)) (specializes (reference "IfThenAction") (range none)) (specializes (reference "IfThenElsePerformance") (range none)) (specializes (reference "IfThenAction") (range (start (line 409) (character 32)) (end (line 409) (character 44)))) (specializes (reference "IfThenElsePerformance") (range (start (line 409) (character 46)) (end (line 409) (character 67)))))))
    (element (id (node (document "d0") (qualified-name "Actions::IfThenElseAction::_documentation"))) (kind "documentation") (name "") (range (start (line 409) (character 1)) (end (line 409) (character 329))) (parent (node (document "d0") (qualified-name "Actions::IfThenElseAction"))))
    (element (id (node (document "d0") (qualified-name "Actions::IfThenElseAction::action"))) (kind "in out parameter") (name "action") (declared-name "action") (range (start (line 417) (character 2)) (end (line 417) (character 29))) (parent (node (document "d0") (qualified-name "Actions::IfThenElseAction"))) (authored (relationships (typing (reference "action thenClause[0..1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "Actions::IfThenElseAction::action#in_out_parameter"))) (kind "in out parameter") (name "action") (declared-name "action") (range (start (line 418) (character 2)) (end (line 418) (character 29))) (parent (node (document "d0") (qualified-name "Actions::IfThenElseAction"))) (authored (relationships (typing (reference "action elseClause[0..1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "Actions::IfThenElseAction::ifTest"))) (kind "in out parameter") (name "ifTest") (declared-name "ifTest") (range (start (line 416) (character 2)) (end (line 416) (character 15))) (parent (node (document "d0") (qualified-name "Actions::IfThenElseAction"))) (authored (relationships (typing (reference "ifTest[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "Actions::IfThenElsePerformance"))) (kind "import") (name "IfThenElsePerformance") (declared-name "IfThenElsePerformance") (range (start (line 24) (character 1)) (end (line 24) (character 59))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlPerformances::IfThenElsePerformance") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 24) (character 16)) (end (line 24) (character 58))))))
    (element (id (node (document "d0") (qualified-name "Actions::IfThenPerformance"))) (kind "import") (name "IfThenPerformance") (declared-name "IfThenPerformance") (range (start (line 23) (character 1)) (end (line 23) (character 55))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlPerformances::IfThenPerformance") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 23) (character 16)) (end (line 23) (character 54))))))
    (element (id (node (document "d0") (qualified-name "Actions::JoinAction"))) (kind "action def") (name "JoinAction") (declared-name "JoinAction") (range (start (line 310) (character 1)) (end (line 310) (character 251))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ControlAction") (range none)) (specializes (reference "ControlAction") (range none)) (specializes (reference "ControlAction") (range (start (line 310) (character 26)) (end (line 310) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "Actions::JoinAction::_documentation"))) (kind "documentation") (name "") (range (start (line 310) (character 1)) (end (line 310) (character 251))) (parent (node (document "d0") (qualified-name "Actions::JoinAction"))))
    (element (id (node (document "d0") (qualified-name "Actions::LoopAction"))) (kind "action def") (name "LoopAction") (declared-name "LoopAction") (range (start (line 435) (character 1)) (end (line 435) (character 264))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Action") (range none)) (specializes (reference "Action") (range none)) (specializes (reference "Action") (range (start (line 435) (character 35)) (end (line 435) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "Actions::LoopAction::_documentation"))) (kind "documentation") (name "") (range (start (line 435) (character 1)) (end (line 435) (character 264))) (parent (node (document "d0") (qualified-name "Actions::LoopAction"))))
    (element (id (node (document "d0") (qualified-name "Actions::LoopAction::action"))) (kind "in out parameter") (name "action") (declared-name "action") (range (start (line 443) (character 2)) (end (line 443) (character 107))) (parent (node (document "d0") (qualified-name "Actions::LoopAction"))) (authored (relationships (typing (reference "action body[0..*]") (range none)))))
    (element (id (node (document "d0") (qualified-name "Actions::LoopAction::ref"))) (kind "in out parameter") (name "ref") (declared-name "ref") (range (start (line 441) (character 8)) (end (line 441) (character 24))) (parent (node (document "d0") (qualified-name "Actions::LoopAction"))) (authored (relationships (typing (reference "ref iterator") (range none)))))
    (element (id (node (document "d0") (qualified-name "Actions::LoopPerformance"))) (kind "import") (name "LoopPerformance") (declared-name "LoopPerformance") (range (start (line 25) (character 1)) (end (line 25) (character 53))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlPerformances::LoopPerformance") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 25) (character 16)) (end (line 25) (character 52))))))
    (element (id (node (document "d0") (qualified-name "Actions::MergeAction"))) (kind "action def") (name "MergeAction") (declared-name "MergeAction") (range (start (line 288) (character 1)) (end (line 288) (character 315))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ControlAction") (range none)) (specializes (reference "MergePerformance") (range none)) (specializes (reference "ControlAction") (range none)) (specializes (reference "MergePerformance") (range none)) (specializes (reference "ControlAction") (range (start (line 288) (character 27)) (end (line 288) (character 40)))) (specializes (reference "MergePerformance") (range (start (line 288) (character 42)) (end (line 288) (character 58)))))))
    (element (id (node (document "d0") (qualified-name "Actions::MergeAction::_documentation"))) (kind "documentation") (name "") (range (start (line 288) (character 1)) (end (line 288) (character 315))) (parent (node (document "d0") (qualified-name "Actions::MergeAction"))))
    (element (id (node (document "d0") (qualified-name "Actions::MergePerformance"))) (kind "import") (name "MergePerformance") (declared-name "MergePerformance") (range (start (line 21) (character 1)) (end (line 21) (character 54))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlPerformances::MergePerformance") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 21) (character 16)) (end (line 21) (character 53))))))
    (element (id (node (document "d0") (qualified-name "Actions::MessageAction"))) (kind "import") (name "MessageAction") (declared-name "MessageAction") (range (start (line 29) (character 1)) (end (line 29) (character 37))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Import) (visibility "private") (import (reference "Flows::MessageAction") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 29) (character 16)) (end (line 29) (character 36))))))
    (element (id (node (document "d0") (qualified-name "Actions::MessageTransfer"))) (kind "import") (name "MessageTransfer") (declared-name "MessageTransfer") (range (start (line 28) (character 1)) (end (line 28) (character 43))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::MessageTransfer") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 28) (character 16)) (end (line 28) (character 42))))))
    (element (id (node (document "d0") (qualified-name "Actions::Natural"))) (kind "import") (name "Natural") (declared-name "Natural") (range (start (line 9) (character 1)) (end (line 9) (character 38))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Natural") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 9) (character 16)) (end (line 9) (character 37))))))
    (element (id (node (document "d0") (qualified-name "Actions::NonStateTransitionPerformance"))) (kind "import") (name "NonStateTransitionPerformance") (declared-name "NonStateTransitionPerformance") (range (start (line 27) (character 1)) (end (line 27) (character 70))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Import) (visibility "private") (import (reference "TransitionPerformances::NonStateTransitionPerformance") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 27) (character 16)) (end (line 27) (character 69))))))
    (element (id (node (document "d0") (qualified-name "Actions::Occurrence"))) (kind "import") (name "Occurrence") (declared-name "Occurrence") (range (start (line 12) (character 1)) (end (line 12) (character 40))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::Occurrence") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 12) (character 16)) (end (line 12) (character 39))))))
    (element (id (node (document "d0") (qualified-name "Actions::Performance"))) (kind "import") (name "Performance") (declared-name "Performance") (range (start (line 14) (character 1)) (end (line 14) (character 42))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Import) (visibility "private") (import (reference "Performances::Performance") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 14) (character 16)) (end (line 14) (character 41))))))
    (element (id (node (document "d0") (qualified-name "Actions::Positive"))) (kind "import") (name "Positive") (declared-name "Positive") (range (start (line 8) (character 1)) (end (line 8) (character 39))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Positive") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 8) (character 16)) (end (line 8) (character 38))))))
    (element (id (node (document "d0") (qualified-name "Actions::SendAction"))) (kind "action def") (name "SendAction") (declared-name "SendAction") (range (start (line 186) (character 1)) (end (line 186) (character 447))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Action") (range none)) (specializes (reference "SendPerformance") (range none)) (specializes (reference "Action") (range none)) (specializes (reference "SendPerformance") (range none)) (specializes (reference "Action") (range (start (line 186) (character 26)) (end (line 186) (character 32)))) (specializes (reference "SendPerformance") (range (start (line 186) (character 34)) (end (line 186) (character 49)))))))
    (element (id (node (document "d0") (qualified-name "Actions::SendAction::_documentation"))) (kind "documentation") (name "") (range (start (line 186) (character 1)) (end (line 186) (character 447))) (parent (node (document "d0") (qualified-name "Actions::SendAction"))))
    (element (id (node (document "d0") (qualified-name "Actions::SendAction::payload"))) (kind "in out parameter") (name "payload") (declared-name "payload") (range (start (line 193) (character 2)) (end (line 193) (character 24))) (parent (node (document "d0") (qualified-name "Actions::SendAction"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Actions::SendAction::sentMessage"))) (kind "ref") (name "sentMessage") (declared-name "sentMessage") (range (start (line 194) (character 5)) (end (line 194) (character 144))) (parent (node (document "d0") (qualified-name "Actions::SendAction"))) (authored (membership (kind Feature)) (relationships (typing (reference "MessageTransfer") (range (start (line 194) (character 39)) (end (line 194) (character 54)))) (typing (reference "MessageAction") (range (start (line 194) (character 56)) (end (line 194) (character 69)))) (redefinition (reference "sentTransfer") (range (start (line 194) (character 25)) (end (line 194) (character 37)))))))
    (element (id (node (document "d0") (qualified-name "Actions::SendAction::sentMessage::MessageTransfer::payload, MessageAction::payload"))) (kind "in out parameter") (name "MessageTransfer::payload, MessageAction::payload") (declared-name "MessageTransfer::payload, MessageAction::payload") (range (start (line 195) (character 9)) (end (line 195) (character 65))) (parent (node (document "d0") (qualified-name "Actions::SendAction::sentMessage"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Actions::SendPerformance"))) (kind "import") (name "SendPerformance") (declared-name "SendPerformance") (range (start (line 16) (character 1)) (end (line 16) (character 43))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::SendPerformance") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 16) (character 16)) (end (line 16) (character 42))))))
    (element (id (node (document "d0") (qualified-name "Actions::TerminateAction"))) (kind "action def") (name "TerminateAction") (declared-name "TerminateAction") (range (start (line 239) (character 1)) (end (line 239) (character 481))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Action") (range none)) (specializes (reference "Action") (range none)) (specializes (reference "Action") (range (start (line 239) (character 40)) (end (line 239) (character 46)))) (perform (reference "Actions::TerminateAction::terminateOccurrence") (range none)))))
    (element (id (node (document "d0") (qualified-name "Actions::TerminateAction::_documentation"))) (kind "documentation") (name "") (range (start (line 239) (character 1)) (end (line 239) (character 481))) (parent (node (document "d0") (qualified-name "Actions::TerminateAction"))))
    (element (id (node (document "d0") (qualified-name "Actions::TerminateAction::occurrence"))) (kind "in out parameter") (name "occurrence") (declared-name "occurrence") (range (start (line 247) (character 2)) (end (line 247) (character 104))) (parent (node (document "d0") (qualified-name "Actions::TerminateAction"))) (authored (relationships (typing (reference "occurrence terminatedOccurrence[1]") (range none)))))
    (element (id (node (document "d0") (qualified-name "Actions::TerminateAction::terminateOccurrence"))) (kind "action") (name "terminateOccurrence") (declared-name "terminateOccurrence") (range (start (line 254) (character 2)) (end (line 254) (character 81))) (parent (node (document "d0") (qualified-name "Actions::TerminateAction"))) (authored (membership (kind Feature)) (relationships (typing (reference "destroy") (range none)))))
    (element (id (node (document "d0") (qualified-name "Actions::TerminateAction::terminateOccurrence::occ"))) (kind "in out parameter") (name "occ") (declared-name "occ") (range (start (line 255) (character 3)) (end (line 255) (character 33))) (parent (node (document "d0") (qualified-name "Actions::TerminateAction::terminateOccurrence"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Actions::TransitionAction"))) (kind "action def") (name "TransitionAction") (declared-name "TransitionAction") (range (start (line 330) (character 1)) (end (line 330) (character 714))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Action") (range none)) (specializes (reference "TransitionPerformance") (range none)) (specializes (reference "Action") (range none)) (specializes (reference "TransitionPerformance") (range none)) (specializes (reference "Action") (range (start (line 330) (character 41)) (end (line 330) (character 47)))) (specializes (reference "TransitionPerformance") (range (start (line 330) (character 49)) (end (line 330) (character 70)))) (perform (reference "Actions::TransitionAction::accepter") (range none)) (perform (reference "Actions::TransitionAction::effect") (range none)))))
    (element (id (node (document "d0") (qualified-name "Actions::TransitionAction::_documentation"))) (kind "documentation") (name "") (range (start (line 330) (character 1)) (end (line 330) (character 714))) (parent (node (document "d0") (qualified-name "Actions::TransitionAction"))))
    (element (id (node (document "d0") (qualified-name "Actions::TransitionAction::acceptedMessage"))) (kind "ref") (name "acceptedMessage") (declared-name "acceptedMessage") (range (start (line 338) (character 2)) (end (line 338) (character 147))) (parent (node (document "d0") (qualified-name "Actions::TransitionAction"))) (authored (membership (kind Feature)) (relationships (typing (reference "MessageTransfer") (range (start (line 338) (character 24)) (end (line 338) (character 39)))) (typing (reference "MessageAction") (range (start (line 338) (character 41)) (end (line 338) (character 54)))))))
    (element (id (node (document "d0") (qualified-name "Actions::TransitionAction::acceptedMessage::MessageTransfer::payload, MessageAction::payload"))) (kind "in out parameter") (name "MessageTransfer::payload, MessageAction::payload") (declared-name "MessageTransfer::payload, MessageAction::payload") (range (start (line 339) (character 12)) (end (line 339) (character 68))) (parent (node (document "d0") (qualified-name "Actions::TransitionAction::acceptedMessage"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Actions::TransitionAction::accepter"))) (kind "action") (name "accepter") (declared-name "accepter") (range (start (line 344) (character 2)) (end (line 344) (character 53))) (parent (node (document "d0") (qualified-name "Actions::TransitionAction"))) (authored (membership (kind Feature)) (relationships (typing (reference "AcceptMessageAction") (range none)) (redefinition (reference "accept") (range (start (line 344) (character 44)) (end (line 344) (character 52)))))))
    (element (id (node (document "d0") (qualified-name "Actions::TransitionAction::effect"))) (kind "action") (name "effect") (declared-name "effect") (range (start (line 349) (character 2)) (end (line 349) (character 58))) (parent (node (document "d0") (qualified-name "Actions::TransitionAction"))) (authored (membership (kind Feature)) (relationships (typing (reference "Action") (range none)) (redefinition (reference "TransitionPerformance::effect") (range (start (line 349) (character 28)) (end (line 349) (character 57)))))))
    (element (id (node (document "d0") (qualified-name "Actions::TransitionAction::receiver"))) (kind "ref") (name "receiver") (declared-name "receiver") (range (start (line 342) (character 2)) (end (line 342) (character 33))) (parent (node (document "d0") (qualified-name "Actions::TransitionAction"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "triggerTarget") (range (start (line 342) (character 19)) (end (line 342) (character 32)))))))
    (element (id (node (document "d0") (qualified-name "Actions::TransitionAction::transitionLinkSource"))) (kind "in out parameter") (name "transitionLinkSource") (declared-name "transitionLinkSource") (range (start (line 337) (character 2)) (end (line 337) (character 83))) (parent (node (document "d0") (qualified-name "Actions::TransitionAction"))) (authored (relationships (typing (reference "transitionLinkSource : Action :>> TransitionPerformance::transitionLinkSource") (range none)))))
    (element (id (node (document "d0") (qualified-name "Actions::TransitionPerformance"))) (kind "import") (name "TransitionPerformance") (declared-name "TransitionPerformance") (range (start (line 26) (character 1)) (end (line 26) (character 62))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Import) (visibility "private") (import (reference "TransitionPerformances::TransitionPerformance") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 26) (character 16)) (end (line 26) (character 61))))))
    (element (id (node (document "d0") (qualified-name "Actions::WhileLoopAction"))) (kind "action def") (name "WhileLoopAction") (declared-name "WhileLoopAction") (range (start (line 451) (character 1)) (end (line 451) (character 766))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "LoopAction") (range none)) (specializes (reference "LoopPerformance") (range none)) (specializes (reference "LoopAction") (range none)) (specializes (reference "LoopPerformance") (range none)) (specializes (reference "LoopAction") (range (start (line 451) (character 31)) (end (line 451) (character 41)))) (specializes (reference "LoopPerformance") (range (start (line 451) (character 43)) (end (line 451) (character 58)))))))
    (element (id (node (document "d0") (qualified-name "Actions::WhileLoopAction::_documentation"))) (kind "documentation") (name "") (range (start (line 451) (character 1)) (end (line 451) (character 766))) (parent (node (document "d0") (qualified-name "Actions::WhileLoopAction"))))
    (element (id (node (document "d0") (qualified-name "Actions::WhileLoopAction::body"))) (kind "in out parameter") (name "body") (declared-name "body") (range (start (line 467) (character 2)) (end (line 467) (character 139))) (parent (node (document "d0") (qualified-name "Actions::WhileLoopAction"))) (authored (relationships (typing (reference "action") (range none)))))
    (element (id (node (document "d0") (qualified-name "Actions::WhileLoopAction::untilTest"))) (kind "in out parameter") (name "untilTest") (declared-name "untilTest") (range (start (line 475) (character 2)) (end (line 475) (character 179))) (parent (node (document "d0") (qualified-name "Actions::WhileLoopAction"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Actions::WhileLoopAction::whileTest"))) (kind "in out parameter") (name "whileTest") (declared-name "whileTest") (range (start (line 458) (character 2)) (end (line 458) (character 223))) (parent (node (document "d0") (qualified-name "Actions::WhileLoopAction"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Actions::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 14635))) (parent (node (document "d0") (qualified-name "Actions"))))
    (element (id (node (document "d0") (qualified-name "Actions::acceptActions"))) (kind "action") (name "acceptActions") (declared-name "acceptActions") (range (start (line 232) (character 1)) (end (line 232) (character 187))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Feature)) (relationships (typing (reference "AcceptAction") (range none)) (subsetting (reference "actions") (range (start (line 232) (character 64)) (end (line 232) (character 71)))) (subsetting (reference "acceptPerformances") (range (start (line 232) (character 73)) (end (line 232) (character 91)))))))
    (element (id (node (document "d0") (qualified-name "Actions::acceptActions::_documentation"))) (kind "documentation") (name "") (range (start (line 232) (character 1)) (end (line 232) (character 187))) (parent (node (document "d0") (qualified-name "Actions::acceptActions"))))
    (element (id (node (document "d0") (qualified-name "Actions::acceptPerformances"))) (kind "import") (name "acceptPerformances") (declared-name "acceptPerformances") (range (start (line 19) (character 1)) (end (line 19) (character 46))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::acceptPerformances") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 19) (character 16)) (end (line 19) (character 45))))))
    (element (id (node (document "d0") (qualified-name "Actions::actions"))) (kind "action") (name "actions") (declared-name "actions") (range (start (line 179) (character 1)) (end (line 179) (character 141))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Feature)) (relationships (typing (reference "Action") (range none)) (subsetting (reference "performances") (range (start (line 179) (character 52)) (end (line 179) (character 64)))))))
    (element (id (node (document "d0") (qualified-name "Actions::actions::_documentation"))) (kind "documentation") (name "") (range (start (line 179) (character 1)) (end (line 179) (character 141))) (parent (node (document "d0") (qualified-name "Actions::actions"))))
    (element (id (node (document "d0") (qualified-name "Actions::assignmentActions"))) (kind "action") (name "assignmentActions") (declared-name "assignmentActions") (range (start (line 383) (character 1)) (end (line 383) (character 428))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Feature)) (relationships (typing (reference "AssignmentAction") (range none)) (subsetting (reference "actions") (range (start (line 383) (character 73)) (end (line 383) (character 80)))))))
    (element (id (node (document "d0") (qualified-name "Actions::assignmentActions::_documentation"))) (kind "documentation") (name "") (range (start (line 383) (character 1)) (end (line 383) (character 428))) (parent (node (document "d0") (qualified-name "Actions::assignmentActions"))))
    (element (id (node (document "d0") (qualified-name "Actions::assignmentActions::target"))) (kind "in out parameter") (name "target") (declared-name "target") (range (start (line 389) (character 8)) (end (line 389) (character 245))) (parent (node (document "d0") (qualified-name "Actions::assignmentActions"))) (authored (relationships (typing (reference "target : Occurrence[1] default that as Occurrence") (range none)))))
    (element (id (node (document "d0") (qualified-name "Actions::destroy"))) (kind "import") (name "destroy") (declared-name "destroy") (range (start (line 30) (character 1)) (end (line 30) (character 45))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Import) (visibility "private") (import (reference "OccurrenceFunctions::destroy") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 30) (character 16)) (end (line 30) (character 44))))))
    (element (id (node (document "d0") (qualified-name "Actions::forLoopActions"))) (kind "action") (name "forLoopActions") (declared-name "forLoopActions") (range (start (line 545) (character 1)) (end (line 545) (character 169))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Feature)) (relationships (typing (reference "ForLoopAction") (range none)) (subsetting (reference "loopActions") (range (start (line 545) (character 67)) (end (line 545) (character 78)))))))
    (element (id (node (document "d0") (qualified-name "Actions::forLoopActions::_documentation"))) (kind "documentation") (name "") (range (start (line 545) (character 1)) (end (line 545) (character 169))) (parent (node (document "d0") (qualified-name "Actions::forLoopActions"))))
    (element (id (node (document "d0") (qualified-name "Actions::ifThenActions"))) (kind "action") (name "ifThenActions") (declared-name "ifThenActions") (range (start (line 421) (character 1)) (end (line 421) (character 157))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Feature)) (relationships (typing (reference "IfThenAction") (range none)) (subsetting (reference "actions") (range (start (line 421) (character 65)) (end (line 421) (character 72)))))))
    (element (id (node (document "d0") (qualified-name "Actions::ifThenActions::_documentation"))) (kind "documentation") (name "") (range (start (line 421) (character 1)) (end (line 421) (character 157))) (parent (node (document "d0") (qualified-name "Actions::ifThenActions"))))
    (element (id (node (document "d0") (qualified-name "Actions::ifThenElseActions"))) (kind "action") (name "ifThenElseActions") (declared-name "ifThenElseActions") (range (start (line 428) (character 1)) (end (line 428) (character 193))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Feature)) (relationships (typing (reference "IfThenElseAction") (range none)) (subsetting (reference "actions") (range (start (line 428) (character 73)) (end (line 428) (character 80)))))))
    (element (id (node (document "d0") (qualified-name "Actions::ifThenElseActions::_documentation"))) (kind "documentation") (name "") (range (start (line 428) (character 1)) (end (line 428) (character 193))) (parent (node (document "d0") (qualified-name "Actions::ifThenElseActions"))))
    (element (id (node (document "d0") (qualified-name "Actions::isEmpty"))) (kind "import") (name "isEmpty") (declared-name "isEmpty") (range (start (line 11) (character 1)) (end (line 11) (character 43))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::isEmpty") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 11) (character 16)) (end (line 11) (character 42))))))
    (element (id (node (document "d0") (qualified-name "Actions::loopActions"))) (kind "action") (name "loopActions") (declared-name "loopActions") (range (start (line 531) (character 1)) (end (line 531) (character 153))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Feature)) (relationships (typing (reference "LoopAction") (range none)) (subsetting (reference "actions") (range (start (line 531) (character 61)) (end (line 531) (character 68)))))))
    (element (id (node (document "d0") (qualified-name "Actions::loopActions::_documentation"))) (kind "documentation") (name "") (range (start (line 531) (character 1)) (end (line 531) (character 153))) (parent (node (document "d0") (qualified-name "Actions::loopActions"))))
    (element (id (node (document "d0") (qualified-name "Actions::performances"))) (kind "import") (name "performances") (declared-name "performances") (range (start (line 15) (character 1)) (end (line 15) (character 43))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Import) (visibility "private") (import (reference "Performances::performances") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 15) (character 16)) (end (line 15) (character 42))))))
    (element (id (node (document "d0") (qualified-name "Actions::sendActions"))) (kind "action") (name "sendActions") (declared-name "sendActions") (range (start (line 199) (character 1)) (end (line 199) (character 170))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Feature)) (relationships (typing (reference "SendAction") (range none)) (subsetting (reference "actions") (range (start (line 199) (character 60)) (end (line 199) (character 67)))) (subsetting (reference "sendPerformances") (range (start (line 199) (character 69)) (end (line 199) (character 85)))))))
    (element (id (node (document "d0") (qualified-name "Actions::sendActions::_documentation"))) (kind "documentation") (name "") (range (start (line 199) (character 1)) (end (line 199) (character 170))) (parent (node (document "d0") (qualified-name "Actions::sendActions"))))
    (element (id (node (document "d0") (qualified-name "Actions::sendPerformances"))) (kind "import") (name "sendPerformances") (declared-name "sendPerformances") (range (start (line 17) (character 1)) (end (line 17) (character 44))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::sendPerformances") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 17) (character 16)) (end (line 17) (character 43))))))
    (element (id (node (document "d0") (qualified-name "Actions::size"))) (kind "import") (name "size") (declared-name "size") (range (start (line 10) (character 1)) (end (line 10) (character 40))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::size") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 10) (character 16)) (end (line 10) (character 39))))))
    (element (id (node (document "d0") (qualified-name "Actions::terminateActions"))) (kind "action") (name "terminateActions") (declared-name "terminateActions") (range (start (line 259) (character 1)) (end (line 259) (character 409))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Feature)) (relationships (typing (reference "TerminateAction") (range none)) (subsetting (reference "actions") (range (start (line 259) (character 71)) (end (line 259) (character 78)))))))
    (element (id (node (document "d0") (qualified-name "Actions::terminateActions::_documentation"))) (kind "documentation") (name "") (range (start (line 259) (character 1)) (end (line 259) (character 409))) (parent (node (document "d0") (qualified-name "Actions::terminateActions"))))
    (element (id (node (document "d0") (qualified-name "Actions::terminateActions::occurrence"))) (kind "in out parameter") (name "occurrence") (declared-name "occurrence") (range (start (line 265) (character 2)) (end (line 265) (character 231))) (parent (node (document "d0") (qualified-name "Actions::terminateActions"))) (authored (relationships (typing (reference "occurrence terminatedOccurrence default that as Occurrence") (range none)))))
    (element (id (node (document "d0") (qualified-name "Actions::transitionActions"))) (kind "action") (name "transitionActions") (declared-name "transitionActions") (range (start (line 364) (character 1)) (end (line 364) (character 170))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Feature)) (relationships (typing (reference "TransitionAction") (range none)) (subsetting (reference "actions") (range (start (line 364) (character 72)) (end (line 364) (character 79)))))))
    (element (id (node (document "d0") (qualified-name "Actions::transitionActions::_documentation"))) (kind "documentation") (name "") (range (start (line 364) (character 1)) (end (line 364) (character 170))) (parent (node (document "d0") (qualified-name "Actions::transitionActions"))))
    (element (id (node (document "d0") (qualified-name "Actions::whileLoopActions"))) (kind "action") (name "whileLoopActions") (declared-name "whileLoopActions") (range (start (line 538) (character 1)) (end (line 538) (character 177))) (parent (node (document "d0") (qualified-name "Actions"))) (authored (membership (kind Feature)) (relationships (typing (reference "WhileLoopAction") (range none)) (subsetting (reference "loopActions") (range (start (line 538) (character 71)) (end (line 538) (character 82)))))))
    (element (id (node (document "d0") (qualified-name "Actions::whileLoopActions::_documentation"))) (kind "documentation") (name "") (range (start (line 538) (character 1)) (end (line 538) (character 177))) (parent (node (document "d0") (qualified-name "Actions::whileLoopActions"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Actions::AcceptAction"))) (kind specialization) (ordinal 0)) (authored-target "AcceptMessageAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::AcceptMessageAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AcceptAction"))) (kind specialization) (ordinal 1)) (authored-target "AcceptMessageAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::AcceptMessageAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AcceptAction"))) (kind specialization) (ordinal 2)) (authored-target "AcceptMessageAction") (range (start (line 218) (character 28)) (end (line 218) (character 47))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::AcceptMessageAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AcceptAction"))) (kind bindSource) (ordinal 0)) (authored-target "payload") (range (start (line 229) (character 7)) (end (line 229) (character 14))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::AcceptMessageAction::payload")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AcceptAction"))) (kind bindTarget) (ordinal 0)) (authored-target "aState::aTransition::apayload") (range (start (line 229) (character 17)) (end (line 229) (character 44))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AcceptAction::"))) (kind redefinition) (ordinal 0)) (authored-target "acceptedMessage") (range (start (line 225) (character 10)) (end (line 225) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::AcceptMessageAction::acceptedMessage")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AcceptMessageAction"))) (kind specialization) (ordinal 0)) (authored-target "Action") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AcceptMessageAction"))) (kind specialization) (ordinal 1)) (authored-target "AcceptPerformance") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::AcceptPerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AcceptMessageAction"))) (kind specialization) (ordinal 2)) (authored-target "Action") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AcceptMessageAction"))) (kind specialization) (ordinal 3)) (authored-target "AcceptPerformance") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::AcceptPerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AcceptMessageAction"))) (kind specialization) (ordinal 4)) (authored-target "Action") (range (start (line 206) (character 35)) (end (line 206) (character 41))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AcceptMessageAction"))) (kind specialization) (ordinal 5)) (authored-target "AcceptPerformance") (range (start (line 206) (character 43)) (end (line 206) (character 60))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::AcceptPerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AcceptMessageAction::acceptedMessage"))) (kind featureTyping) (ordinal 0)) (authored-target "MessageTransfer") (range (start (line 213) (character 44)) (end (line 213) (character 59))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::MessageTransfer")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AcceptMessageAction::acceptedMessage"))) (kind featureTyping) (ordinal 1)) (authored-target "MessageAction") (range (start (line 213) (character 61)) (end (line 213) (character 74))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::MessageAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AcceptMessageAction::acceptedMessage"))) (kind redefinition) (ordinal 0)) (authored-target "acceptedTransfer") (range (start (line 213) (character 26)) (end (line 213) (character 42))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AcceptMessageAction::acceptedMessage::MessageTransfer::payload, MessageAction::payload"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::AcceptMessageAction::_documentation")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AcceptMessageAction::payload"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::AcceptMessageAction::_documentation")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AcceptPerformance"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::AcceptPerformance") (range (start (line 18) (character 16)) (end (line 18) (character 44))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action"))) (kind specialization) (ordinal 0)) (authored-target "Performance") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Performance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action"))) (kind specialization) (ordinal 1)) (authored-target "Performance") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Performance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action"))) (kind specialization) (ordinal 2)) (authored-target "Performance") (range (start (line 32) (character 31)) (end (line 32) (character 42))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Performance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 0)) (authored-target "Actions::Action::start") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::start")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 1)) (authored-target "Actions::Action::done") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::done")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 2)) (authored-target "Actions::Action::subactions") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::subactions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 3)) (authored-target "Actions::Action::sendSubactions") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::sendSubactions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 4)) (authored-target "Actions::Action::acceptSubactions") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::acceptSubactions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 5)) (authored-target "Actions::Action::terminateSubactions") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::terminateSubactions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 6)) (authored-target "Actions::Action::controls") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::controls")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 7)) (authored-target "Actions::Action::merges") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::merges")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 8)) (authored-target "Actions::Action::decisions") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::decisions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 9)) (authored-target "Actions::Action::joins") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::joins")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 10)) (authored-target "Actions::Action::forks") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::forks")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 11)) (authored-target "Actions::Action::transitions") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::transitions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 12)) (authored-target "Actions::Action::decisionTransitions") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::decisionTransitions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 13)) (authored-target "Actions::Action::assignments") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::assignments")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 14)) (authored-target "Actions::Action::ifSubactions") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::ifSubactions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 15)) (authored-target "Actions::Action::loops") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::loops")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 16)) (authored-target "Actions::Action::whileLoops") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::whileLoops")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action"))) (kind performSource) (ordinal 17)) (authored-target "Actions::Action::forLoops") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::forLoops")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::acceptSubactions"))) (kind featureTyping) (ordinal 0)) (authored-target "AcceptAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::AcceptAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::acceptSubactions"))) (kind subsetting) (ordinal 0)) (authored-target "subactions") (range (start (line 78) (character 49)) (end (line 78) (character 59))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::subactions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::acceptSubactions"))) (kind subsetting) (ordinal 1)) (authored-target "acceptActions") (range (start (line 78) (character 61)) (end (line 78) (character 74))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::acceptActions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::assignments"))) (kind featureTyping) (ordinal 0)) (authored-target "AssignmentAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::AssignmentAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::assignments"))) (kind subsetting) (ordinal 0)) (authored-target "subactions") (range (start (line 141) (character 58)) (end (line 141) (character 68))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::subactions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::assignments"))) (kind subsetting) (ordinal 1)) (authored-target "assignmentActions") (range (start (line 141) (character 70)) (end (line 141) (character 87))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::assignmentActions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::assignments::target"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::assignments::_documentation")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::controls"))) (kind featureTyping) (ordinal 0)) (authored-target "ControlAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::ControlAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::controls"))) (kind subsetting) (ordinal 0)) (authored-target "subactions") (range (start (line 92) (character 52)) (end (line 92) (character 62))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::subactions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::decisionTransitions"))) (kind featureTyping) (ordinal 0)) (authored-target "DecisionTransitionAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::DecisionTransitionAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::decisionTransitions"))) (kind subsetting) (ordinal 0)) (authored-target "transitions") (range (start (line 134) (character 74)) (end (line 134) (character 85))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::transitions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::decisions"))) (kind featureTyping) (ordinal 0)) (authored-target "DecisionAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::DecisionAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::decisions"))) (kind subsetting) (ordinal 0)) (authored-target "controls") (range (start (line 106) (character 48)) (end (line 106) (character 56))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::controls")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::done"))) (kind featureTyping) (ordinal 0)) (authored-target "Action") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::done"))) (kind redefinition) (ordinal 0)) (authored-target "endShot") (range (start (line 49) (character 26)) (end (line 49) (character 33))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::forLoops"))) (kind featureTyping) (ordinal 0)) (authored-target "ForLoopAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::ForLoopAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::forLoops"))) (kind subsetting) (ordinal 0)) (authored-target "loops") (range (start (line 171) (character 52)) (end (line 171) (character 57))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::loops")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::forLoops"))) (kind subsetting) (ordinal 1)) (authored-target "forLoopActions") (range (start (line 171) (character 59)) (end (line 171) (character 73))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::forLoopActions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::forks"))) (kind featureTyping) (ordinal 0)) (authored-target "ForkAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::ForkAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::forks"))) (kind subsetting) (ordinal 0)) (authored-target "controls") (range (start (line 120) (character 40)) (end (line 120) (character 48))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::controls")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::ifSubactions"))) (kind featureTyping) (ordinal 0)) (authored-target "IfThenAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::IfThenAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::ifSubactions"))) (kind subsetting) (ordinal 0)) (authored-target "subactions") (range (start (line 150) (character 55)) (end (line 150) (character 65))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::subactions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::ifSubactions"))) (kind subsetting) (ordinal 1)) (authored-target "ifThenActions") (range (start (line 150) (character 67)) (end (line 150) (character 80))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::ifThenActions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::incomingTransfers"))) (kind redefinition) (ordinal 0)) (authored-target "Performance::incomingTransfers") (range (start (line 40) (character 35)) (end (line 40) (character 65))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::joins"))) (kind featureTyping) (ordinal 0)) (authored-target "JoinAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::JoinAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::joins"))) (kind subsetting) (ordinal 0)) (authored-target "controls") (range (start (line 113) (character 40)) (end (line 113) (character 48))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::controls")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::loops"))) (kind featureTyping) (ordinal 0)) (authored-target "LoopAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::LoopAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::loops"))) (kind subsetting) (ordinal 0)) (authored-target "subactions") (range (start (line 157) (character 46)) (end (line 157) (character 56))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::subactions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::loops"))) (kind subsetting) (ordinal 1)) (authored-target "loopActions") (range (start (line 157) (character 58)) (end (line 157) (character 69))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::loopActions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::merges"))) (kind featureTyping) (ordinal 0)) (authored-target "MergeAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::MergeAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::merges"))) (kind subsetting) (ordinal 0)) (authored-target "controls") (range (start (line 99) (character 48)) (end (line 99) (character 56))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::controls")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::self"))) (kind featureTyping) (ordinal 0)) (authored-target "Action") (range (start (line 39) (character 19)) (end (line 39) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::sendSubactions"))) (kind featureTyping) (ordinal 0)) (authored-target "SendAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::SendAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::sendSubactions"))) (kind subsetting) (ordinal 0)) (authored-target "subactions") (range (start (line 71) (character 45)) (end (line 71) (character 55))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::subactions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::sendSubactions"))) (kind subsetting) (ordinal 1)) (authored-target "sendActions") (range (start (line 71) (character 57)) (end (line 71) (character 68))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::sendActions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::start"))) (kind featureTyping) (ordinal 0)) (authored-target "Action") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::start"))) (kind redefinition) (ordinal 0)) (authored-target "startShot") (range (start (line 42) (character 27)) (end (line 42) (character 36))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::subactions"))) (kind featureTyping) (ordinal 0)) (authored-target "Action") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::subactions"))) (kind subsetting) (ordinal 0)) (authored-target "actions") (range (start (line 56) (character 37)) (end (line 56) (character 44))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::actions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::subactions"))) (kind subsetting) (ordinal 1)) (authored-target "subperformances") (range (start (line 56) (character 46)) (end (line 56) (character 61))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::subactions::occurrence"))) (kind redefinition) (ordinal 0)) (authored-target "Action::this") (range (start (line 62) (character 22)) (end (line 62) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::terminateSubactions"))) (kind featureTyping) (ordinal 0)) (authored-target "TerminateAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::TerminateAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::terminateSubactions"))) (kind subsetting) (ordinal 0)) (authored-target "subactions") (range (start (line 85) (character 65)) (end (line 85) (character 75))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::subactions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::terminateSubactions"))) (kind subsetting) (ordinal 1)) (authored-target "terminateActions") (range (start (line 85) (character 77)) (end (line 85) (character 93))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::terminateActions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::transitions"))) (kind featureTyping) (ordinal 0)) (authored-target "TransitionAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::TransitionAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::transitions"))) (kind subsetting) (ordinal 0)) (authored-target "subactions") (range (start (line 127) (character 58)) (end (line 127) (character 68))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::subactions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::transitions"))) (kind subsetting) (ordinal 1)) (authored-target "transitionActions") (range (start (line 127) (character 70)) (end (line 127) (character 87))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::transitionActions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::whileLoops"))) (kind featureTyping) (ordinal 0)) (authored-target "WhileLoopAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::WhileLoopAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::whileLoops"))) (kind subsetting) (ordinal 0)) (authored-target "loops") (range (start (line 164) (character 56)) (end (line 164) (character 61))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::loops")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Action::whileLoops"))) (kind subsetting) (ordinal 1)) (authored-target "whileLoopActions") (range (start (line 164) (character 63)) (end (line 164) (character 79))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::whileLoopActions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Anything"))) (kind membershipImport) (ordinal 0)) (authored-target "Base::Anything") (range (start (line 7) (character 16)) (end (line 7) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AssignmentAction"))) (kind specialization) (ordinal 0)) (authored-target "FeatureWritePerformance") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::FeatureWritePerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AssignmentAction"))) (kind specialization) (ordinal 1)) (authored-target "Action") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AssignmentAction"))) (kind specialization) (ordinal 2)) (authored-target "FeatureWritePerformance") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::FeatureWritePerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AssignmentAction"))) (kind specialization) (ordinal 3)) (authored-target "Action") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AssignmentAction"))) (kind specialization) (ordinal 4)) (authored-target "FeatureWritePerformance") (range (start (line 371) (character 32)) (end (line 371) (character 55))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::FeatureWritePerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AssignmentAction"))) (kind specialization) (ordinal 5)) (authored-target "Action") (range (start (line 371) (character 57)) (end (line 371) (character 63))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AssignmentAction::replacementValues"))) (kind featureTyping) (ordinal 0)) (authored-target "replacementValues : Anything[0..*] nonunique") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::AssignmentAction::target"))) (kind featureTyping) (ordinal 0)) (authored-target "target : Occurrence[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::ControlAction"))) (kind specialization) (ordinal 0)) (authored-target "Action") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::ControlAction"))) (kind specialization) (ordinal 1)) (authored-target "Action") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::ControlAction"))) (kind specialization) (ordinal 2)) (authored-target "Action") (range (start (line 274) (character 38)) (end (line 274) (character 44))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::ControlAction"))) (kind bindSource) (ordinal 0)) (authored-target "start") (range (start (line 280) (character 7)) (end (line 280) (character 12))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::start")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::ControlAction"))) (kind bindTarget) (ordinal 0)) (authored-target "done") (range (start (line 280) (character 15)) (end (line 280) (character 19))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action::done")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::DecisionAction"))) (kind specialization) (ordinal 0)) (authored-target "ControlAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::ControlAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::DecisionAction"))) (kind specialization) (ordinal 1)) (authored-target "DecisionPerformance") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::DecisionPerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::DecisionAction"))) (kind specialization) (ordinal 2)) (authored-target "ControlAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::ControlAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::DecisionAction"))) (kind specialization) (ordinal 3)) (authored-target "DecisionPerformance") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::DecisionPerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::DecisionAction"))) (kind specialization) (ordinal 4)) (authored-target "ControlAction") (range (start (line 298) (character 30)) (end (line 298) (character 43))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::ControlAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::DecisionAction"))) (kind specialization) (ordinal 5)) (authored-target "DecisionPerformance") (range (start (line 298) (character 45)) (end (line 298) (character 64))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::DecisionPerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::DecisionPerformance"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlPerformances::DecisionPerformance") (range (start (line 22) (character 16)) (end (line 22) (character 56))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::DecisionTransitionAction"))) (kind specialization) (ordinal 0)) (authored-target "TransitionAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::TransitionAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::DecisionTransitionAction"))) (kind specialization) (ordinal 1)) (authored-target "NonStateTransitionPerformance") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::NonStateTransitionPerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::DecisionTransitionAction"))) (kind specialization) (ordinal 2)) (authored-target "TransitionAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::TransitionAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::DecisionTransitionAction"))) (kind specialization) (ordinal 3)) (authored-target "NonStateTransitionPerformance") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::NonStateTransitionPerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::DecisionTransitionAction"))) (kind specialization) (ordinal 4)) (authored-target "TransitionAction") (range (start (line 352) (character 40)) (end (line 352) (character 56))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::TransitionAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::DecisionTransitionAction"))) (kind specialization) (ordinal 5)) (authored-target "NonStateTransitionPerformance") (range (start (line 352) (character 58)) (end (line 352) (character 87))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::NonStateTransitionPerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::DecisionTransitionAction::"))) (kind redefinition) (ordinal 0)) (authored-target "accepter") (range (start (line 360) (character 17)) (end (line 360) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::TransitionAction::accepter")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::DecisionTransitionAction::#ref"))) (kind redefinition) (ordinal 0)) (authored-target "effect") (range (start (line 361) (character 17)) (end (line 361) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::TransitionAction::effect")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::FeatureWritePerformance"))) (kind membershipImport) (ordinal 0)) (authored-target "FeatureReferencingPerformances::FeatureWritePerformance") (range (start (line 20) (character 16)) (end (line 20) (character 71))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::ForLoopAction"))) (kind specialization) (ordinal 0)) (authored-target "LoopAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::LoopAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::ForLoopAction"))) (kind specialization) (ordinal 1)) (authored-target "LoopAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::LoopAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::ForLoopAction"))) (kind specialization) (ordinal 2)) (authored-target "LoopAction") (range (start (line 484) (character 29)) (end (line 484) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::LoopAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::ForLoopAction"))) (kind performSource) (ordinal 0)) (authored-target "Actions::ForLoopAction::initialization") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::ForLoopAction::initialization")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::ForLoopAction"))) (kind performSource) (ordinal 1)) (authored-target "Actions::ForLoopAction::whileLoop") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::ForLoopAction::whileLoop")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::ForLoopAction::_while"))) (kind flowSource) (ordinal 0)) (authored-target "Actions::ForLoopAction::_while::body") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::ForLoopAction::body"))) (kind featureTyping) (ordinal 0)) (authored-target "action") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::LoopAction::action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::ForLoopAction::ref"))) (kind featureTyping) (ordinal 0)) (authored-target "ref seq") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::ForLoopAction::var"))) (kind featureTyping) (ordinal 0)) (authored-target "seq") (range (start (line 491) (character 29)) (end (line 491) (character 32))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::ForLoopAction::whileLoop"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::ForLoopAction::_documentation")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::ForkAction"))) (kind specialization) (ordinal 0)) (authored-target "ControlAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::ControlAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::ForkAction"))) (kind specialization) (ordinal 1)) (authored-target "ControlAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::ControlAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::ForkAction"))) (kind specialization) (ordinal 2)) (authored-target "ControlAction") (range (start (line 320) (character 26)) (end (line 320) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::ControlAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::HappensWhile"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::HappensWhile") (range (start (line 13) (character 16)) (end (line 13) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::IfThenAction"))) (kind specialization) (ordinal 0)) (authored-target "Action") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::IfThenAction"))) (kind specialization) (ordinal 1)) (authored-target "IfThenPerformance") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::IfThenPerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::IfThenAction"))) (kind specialization) (ordinal 2)) (authored-target "Action") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::IfThenAction"))) (kind specialization) (ordinal 3)) (authored-target "IfThenPerformance") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::IfThenPerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::IfThenAction"))) (kind specialization) (ordinal 4)) (authored-target "Action") (range (start (line 398) (character 28)) (end (line 398) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::IfThenAction"))) (kind specialization) (ordinal 5)) (authored-target "IfThenPerformance") (range (start (line 398) (character 36)) (end (line 398) (character 53))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::IfThenPerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::IfThenAction::action"))) (kind featureTyping) (ordinal 0)) (authored-target "action thenClause[0..1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::IfThenAction::ifTest"))) (kind featureTyping) (ordinal 0)) (authored-target "ifTest[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::IfThenElseAction"))) (kind specialization) (ordinal 0)) (authored-target "IfThenAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::IfThenAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::IfThenElseAction"))) (kind specialization) (ordinal 1)) (authored-target "IfThenElsePerformance") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::IfThenElsePerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::IfThenElseAction"))) (kind specialization) (ordinal 2)) (authored-target "IfThenAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::IfThenAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::IfThenElseAction"))) (kind specialization) (ordinal 3)) (authored-target "IfThenElsePerformance") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::IfThenElsePerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::IfThenElseAction"))) (kind specialization) (ordinal 4)) (authored-target "IfThenAction") (range (start (line 409) (character 32)) (end (line 409) (character 44))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::IfThenAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::IfThenElseAction"))) (kind specialization) (ordinal 5)) (authored-target "IfThenElsePerformance") (range (start (line 409) (character 46)) (end (line 409) (character 67))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::IfThenElsePerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::IfThenElseAction::action"))) (kind featureTyping) (ordinal 0)) (authored-target "action thenClause[0..1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::IfThenElseAction::action#in_out_parameter"))) (kind featureTyping) (ordinal 0)) (authored-target "action elseClause[0..1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::IfThenElseAction::ifTest"))) (kind featureTyping) (ordinal 0)) (authored-target "ifTest[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::IfThenElsePerformance"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlPerformances::IfThenElsePerformance") (range (start (line 24) (character 16)) (end (line 24) (character 58))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::IfThenPerformance"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlPerformances::IfThenPerformance") (range (start (line 23) (character 16)) (end (line 23) (character 54))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::JoinAction"))) (kind specialization) (ordinal 0)) (authored-target "ControlAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::ControlAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::JoinAction"))) (kind specialization) (ordinal 1)) (authored-target "ControlAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::ControlAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::JoinAction"))) (kind specialization) (ordinal 2)) (authored-target "ControlAction") (range (start (line 310) (character 26)) (end (line 310) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::ControlAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::LoopAction"))) (kind specialization) (ordinal 0)) (authored-target "Action") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::LoopAction"))) (kind specialization) (ordinal 1)) (authored-target "Action") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::LoopAction"))) (kind specialization) (ordinal 2)) (authored-target "Action") (range (start (line 435) (character 35)) (end (line 435) (character 41))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::LoopAction::action"))) (kind featureTyping) (ordinal 0)) (authored-target "action body[0..*]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::LoopAction::ref"))) (kind featureTyping) (ordinal 0)) (authored-target "ref iterator") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::LoopPerformance"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlPerformances::LoopPerformance") (range (start (line 25) (character 16)) (end (line 25) (character 52))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::MergeAction"))) (kind specialization) (ordinal 0)) (authored-target "ControlAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::ControlAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::MergeAction"))) (kind specialization) (ordinal 1)) (authored-target "MergePerformance") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::MergePerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::MergeAction"))) (kind specialization) (ordinal 2)) (authored-target "ControlAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::ControlAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::MergeAction"))) (kind specialization) (ordinal 3)) (authored-target "MergePerformance") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::MergePerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::MergeAction"))) (kind specialization) (ordinal 4)) (authored-target "ControlAction") (range (start (line 288) (character 27)) (end (line 288) (character 40))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::ControlAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::MergeAction"))) (kind specialization) (ordinal 5)) (authored-target "MergePerformance") (range (start (line 288) (character 42)) (end (line 288) (character 58))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::MergePerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::MergePerformance"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlPerformances::MergePerformance") (range (start (line 21) (character 16)) (end (line 21) (character 53))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::MessageAction"))) (kind membershipImport) (ordinal 0)) (authored-target "Flows::MessageAction") (range (start (line 29) (character 16)) (end (line 29) (character 36))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::MessageTransfer"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::MessageTransfer") (range (start (line 28) (character 16)) (end (line 28) (character 42))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Natural"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Natural") (range (start (line 9) (character 16)) (end (line 9) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::NonStateTransitionPerformance"))) (kind membershipImport) (ordinal 0)) (authored-target "TransitionPerformances::NonStateTransitionPerformance") (range (start (line 27) (character 16)) (end (line 27) (character 69))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Occurrence"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::Occurrence") (range (start (line 12) (character 16)) (end (line 12) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Performance"))) (kind membershipImport) (ordinal 0)) (authored-target "Performances::Performance") (range (start (line 14) (character 16)) (end (line 14) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::Positive"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Positive") (range (start (line 8) (character 16)) (end (line 8) (character 38))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::SendAction"))) (kind specialization) (ordinal 0)) (authored-target "Action") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::SendAction"))) (kind specialization) (ordinal 1)) (authored-target "SendPerformance") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::SendPerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::SendAction"))) (kind specialization) (ordinal 2)) (authored-target "Action") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::SendAction"))) (kind specialization) (ordinal 3)) (authored-target "SendPerformance") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::SendPerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::SendAction"))) (kind specialization) (ordinal 4)) (authored-target "Action") (range (start (line 186) (character 26)) (end (line 186) (character 32))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::SendAction"))) (kind specialization) (ordinal 5)) (authored-target "SendPerformance") (range (start (line 186) (character 34)) (end (line 186) (character 49))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::SendPerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::SendAction::payload"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::SendAction::_documentation")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::SendAction::sentMessage"))) (kind featureTyping) (ordinal 0)) (authored-target "MessageTransfer") (range (start (line 194) (character 39)) (end (line 194) (character 54))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::MessageTransfer")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::SendAction::sentMessage"))) (kind featureTyping) (ordinal 1)) (authored-target "MessageAction") (range (start (line 194) (character 56)) (end (line 194) (character 69))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::MessageAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::SendAction::sentMessage"))) (kind redefinition) (ordinal 0)) (authored-target "sentTransfer") (range (start (line 194) (character 25)) (end (line 194) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::SendAction::sentMessage::MessageTransfer::payload, MessageAction::payload"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::SendAction::_documentation")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::SendPerformance"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::SendPerformance") (range (start (line 16) (character 16)) (end (line 16) (character 42))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TerminateAction"))) (kind specialization) (ordinal 0)) (authored-target "Action") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TerminateAction"))) (kind specialization) (ordinal 1)) (authored-target "Action") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TerminateAction"))) (kind specialization) (ordinal 2)) (authored-target "Action") (range (start (line 239) (character 40)) (end (line 239) (character 46))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TerminateAction"))) (kind performSource) (ordinal 0)) (authored-target "Actions::TerminateAction::terminateOccurrence") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::TerminateAction::terminateOccurrence")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TerminateAction::occurrence"))) (kind featureTyping) (ordinal 0)) (authored-target "occurrence terminatedOccurrence[1]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TerminateAction::terminateOccurrence"))) (kind featureTyping) (ordinal 0)) (authored-target "destroy") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::destroy")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TerminateAction::terminateOccurrence::occ"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::TerminateAction::_documentation")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TransitionAction"))) (kind specialization) (ordinal 0)) (authored-target "Action") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TransitionAction"))) (kind specialization) (ordinal 1)) (authored-target "TransitionPerformance") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::TransitionPerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TransitionAction"))) (kind specialization) (ordinal 2)) (authored-target "Action") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TransitionAction"))) (kind specialization) (ordinal 3)) (authored-target "TransitionPerformance") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::TransitionPerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TransitionAction"))) (kind specialization) (ordinal 4)) (authored-target "Action") (range (start (line 330) (character 41)) (end (line 330) (character 47))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TransitionAction"))) (kind specialization) (ordinal 5)) (authored-target "TransitionPerformance") (range (start (line 330) (character 49)) (end (line 330) (character 70))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::TransitionPerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TransitionAction"))) (kind bindSource) (ordinal 0)) (authored-target "receiver") (range (start (line 346) (character 7)) (end (line 346) (character 15))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::TransitionAction::receiver")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TransitionAction"))) (kind bindSource) (ordinal 1)) (authored-target "acceptedMessage") (range (start (line 347) (character 7)) (end (line 347) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::TransitionAction::acceptedMessage")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TransitionAction"))) (kind bindTarget) (ordinal 0)) (authored-target "accepter::receiver") (range (start (line 346) (character 18)) (end (line 346) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TransitionAction"))) (kind bindTarget) (ordinal 1)) (authored-target "accepter::acceptedMessage") (range (start (line 347) (character 25)) (end (line 347) (character 49))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TransitionAction"))) (kind performSource) (ordinal 0)) (authored-target "Actions::TransitionAction::accepter") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::TransitionAction::accepter")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TransitionAction"))) (kind performSource) (ordinal 1)) (authored-target "Actions::TransitionAction::effect") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::TransitionAction::effect")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TransitionAction::acceptedMessage"))) (kind featureTyping) (ordinal 0)) (authored-target "MessageTransfer") (range (start (line 338) (character 24)) (end (line 338) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::MessageTransfer")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TransitionAction::acceptedMessage"))) (kind featureTyping) (ordinal 1)) (authored-target "MessageAction") (range (start (line 338) (character 41)) (end (line 338) (character 54))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::MessageAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TransitionAction::acceptedMessage::MessageTransfer::payload, MessageAction::payload"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::TransitionAction::_documentation")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TransitionAction::accepter"))) (kind featureTyping) (ordinal 0)) (authored-target "AcceptMessageAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::AcceptMessageAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TransitionAction::accepter"))) (kind redefinition) (ordinal 0)) (authored-target "accept") (range (start (line 344) (character 44)) (end (line 344) (character 52))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TransitionAction::effect"))) (kind featureTyping) (ordinal 0)) (authored-target "Action") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TransitionAction::effect"))) (kind redefinition) (ordinal 0)) (authored-target "TransitionPerformance::effect") (range (start (line 349) (character 28)) (end (line 349) (character 57))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TransitionAction::receiver"))) (kind redefinition) (ordinal 0)) (authored-target "triggerTarget") (range (start (line 342) (character 19)) (end (line 342) (character 32))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TransitionAction::transitionLinkSource"))) (kind featureTyping) (ordinal 0)) (authored-target "transitionLinkSource : Action :>> TransitionPerformance::transitionLinkSource") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::TransitionPerformance"))) (kind membershipImport) (ordinal 0)) (authored-target "TransitionPerformances::TransitionPerformance") (range (start (line 26) (character 16)) (end (line 26) (character 61))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::WhileLoopAction"))) (kind specialization) (ordinal 0)) (authored-target "LoopAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::LoopAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::WhileLoopAction"))) (kind specialization) (ordinal 1)) (authored-target "LoopPerformance") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::LoopPerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::WhileLoopAction"))) (kind specialization) (ordinal 2)) (authored-target "LoopAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::LoopAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::WhileLoopAction"))) (kind specialization) (ordinal 3)) (authored-target "LoopPerformance") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::LoopPerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::WhileLoopAction"))) (kind specialization) (ordinal 4)) (authored-target "LoopAction") (range (start (line 451) (character 31)) (end (line 451) (character 41))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::LoopAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::WhileLoopAction"))) (kind specialization) (ordinal 5)) (authored-target "LoopPerformance") (range (start (line 451) (character 43)) (end (line 451) (character 58))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::LoopPerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::WhileLoopAction::body"))) (kind featureTyping) (ordinal 0)) (authored-target "action") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::LoopAction::action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::WhileLoopAction::untilTest"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::WhileLoopAction::_documentation")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::WhileLoopAction::whileTest"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::WhileLoopAction::_documentation")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::acceptActions"))) (kind featureTyping) (ordinal 0)) (authored-target "AcceptAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::AcceptAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::acceptActions"))) (kind subsetting) (ordinal 0)) (authored-target "actions") (range (start (line 232) (character 64)) (end (line 232) (character 71))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::actions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::acceptActions"))) (kind subsetting) (ordinal 1)) (authored-target "acceptPerformances") (range (start (line 232) (character 73)) (end (line 232) (character 91))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::acceptPerformances")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::acceptPerformances"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::acceptPerformances") (range (start (line 19) (character 16)) (end (line 19) (character 45))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::actions"))) (kind featureTyping) (ordinal 0)) (authored-target "Action") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::actions"))) (kind subsetting) (ordinal 0)) (authored-target "performances") (range (start (line 179) (character 52)) (end (line 179) (character 64))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::performances")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::assignmentActions"))) (kind featureTyping) (ordinal 0)) (authored-target "AssignmentAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::AssignmentAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::assignmentActions"))) (kind subsetting) (ordinal 0)) (authored-target "actions") (range (start (line 383) (character 73)) (end (line 383) (character 80))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::actions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::assignmentActions::target"))) (kind featureTyping) (ordinal 0)) (authored-target "target : Occurrence[1] default that as Occurrence") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::destroy"))) (kind membershipImport) (ordinal 0)) (authored-target "OccurrenceFunctions::destroy") (range (start (line 30) (character 16)) (end (line 30) (character 44))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::forLoopActions"))) (kind featureTyping) (ordinal 0)) (authored-target "ForLoopAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::ForLoopAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::forLoopActions"))) (kind subsetting) (ordinal 0)) (authored-target "loopActions") (range (start (line 545) (character 67)) (end (line 545) (character 78))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::loopActions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::ifThenActions"))) (kind featureTyping) (ordinal 0)) (authored-target "IfThenAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::IfThenAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::ifThenActions"))) (kind subsetting) (ordinal 0)) (authored-target "actions") (range (start (line 421) (character 65)) (end (line 421) (character 72))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::actions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::ifThenElseActions"))) (kind featureTyping) (ordinal 0)) (authored-target "IfThenElseAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::IfThenElseAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::ifThenElseActions"))) (kind subsetting) (ordinal 0)) (authored-target "actions") (range (start (line 428) (character 73)) (end (line 428) (character 80))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::actions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::isEmpty"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::isEmpty") (range (start (line 11) (character 16)) (end (line 11) (character 42))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::loopActions"))) (kind featureTyping) (ordinal 0)) (authored-target "LoopAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::LoopAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::loopActions"))) (kind subsetting) (ordinal 0)) (authored-target "actions") (range (start (line 531) (character 61)) (end (line 531) (character 68))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::actions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::performances"))) (kind membershipImport) (ordinal 0)) (authored-target "Performances::performances") (range (start (line 15) (character 16)) (end (line 15) (character 42))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::sendActions"))) (kind featureTyping) (ordinal 0)) (authored-target "SendAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::SendAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::sendActions"))) (kind subsetting) (ordinal 0)) (authored-target "actions") (range (start (line 199) (character 60)) (end (line 199) (character 67))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::actions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::sendActions"))) (kind subsetting) (ordinal 1)) (authored-target "sendPerformances") (range (start (line 199) (character 69)) (end (line 199) (character 85))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::sendPerformances")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::sendPerformances"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::sendPerformances") (range (start (line 17) (character 16)) (end (line 17) (character 43))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::size"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::size") (range (start (line 10) (character 16)) (end (line 10) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::terminateActions"))) (kind featureTyping) (ordinal 0)) (authored-target "TerminateAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::TerminateAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::terminateActions"))) (kind subsetting) (ordinal 0)) (authored-target "actions") (range (start (line 259) (character 71)) (end (line 259) (character 78))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::actions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::terminateActions::occurrence"))) (kind featureTyping) (ordinal 0)) (authored-target "occurrence terminatedOccurrence default that as Occurrence") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Actions::transitionActions"))) (kind featureTyping) (ordinal 0)) (authored-target "TransitionAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::TransitionAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::transitionActions"))) (kind subsetting) (ordinal 0)) (authored-target "actions") (range (start (line 364) (character 72)) (end (line 364) (character 79))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::actions")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::whileLoopActions"))) (kind featureTyping) (ordinal 0)) (authored-target "WhileLoopAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::WhileLoopAction")))))
    (reference (id (source (node (document "d0") (qualified-name "Actions::whileLoopActions"))) (kind subsetting) (ordinal 0)) (authored-target "loopActions") (range (start (line 538) (character 71)) (end (line 538) (character 82))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Actions::loopActions")))))
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
    (relationship (kind bind) (source (node (document "d0") (qualified-name "Actions::Action::start"))) (target (node (document "d0") (qualified-name "Actions::Action::done"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Actions::ControlAction"))) (kind bindSource) (ordinal 0)) (expression (kind bind) (source "start") (target "done") (source-range (start (line 280) (character 7)) (end (line 280) (character 12))) (target-range (start (line 280) (character 15)) (end (line 280) (character 19)))))
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
