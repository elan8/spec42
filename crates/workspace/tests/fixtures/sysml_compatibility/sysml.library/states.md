# META
~~~ini
description=Standard Library: Systems Library/States
type=file
~~~
# SOURCE
~~~sysml
standard library package States {
	doc
	/*
	 * This package defines the base types for states and related behavioral elements in the
	 * SysML language.
	 */

	private import Occurrences::Occurrence;
	private import StatePerformances::StatePerformance;
	private import StatePerformances::StateTransitionPerformance;
	private import Actions::Action;
	private import Actions::TransitionAction;
	private import Actions::transitionActions;
	private import Actions::AcceptAction;
	private import Actions::actions;
	private import SequenceFunctions::notEmpty;
	private import SequenceFunctions::size;
	
	abstract state def StateAction :> Action, StatePerformance {
		doc
		/*
		 * A StateAction is a kind of Action that is also a StatePerformance. It is the base type for all
		 * StateDefinitions.
		 */
	
		entry action entryAction :>> 'entry';
		do action doAction: Action :>> 'do';
		exit action exitAction: Action :>> 'exit';
		
		attribute :>> isTriggerDuring;
		
		ref state self: StateAction :>> Action::self, StatePerformance::self;
		ref state start: StateAction :>> Action::start, StatePerformance::startShot;
		ref state done: StateAction :>> Action::done, StatePerformance::endShot;
		
		action :>> subactions :> middle {
			doc
			/*
			 * The subperformances of this StateAction that are Actions, other than the entry and exit Actions. 
			 * These subactions all take place in the "middle" of the StatePerformance, that is, after the 
			 * entry Action and before the exit Action. 
			 */
		}
		
		action substates: StateAction[0..*] :> stateActions, subactions {
			doc
			/*
			 * The subactions of this state that are states.
			 * 
			 * NOTE: This feature is declared as an ActionUsage, not a StateUsage, so that the constraint 
 			 * checkStateUsageExclusiveStateSpecialization does not apply to it, since this constraint 
			 * would otherwise incorrectly require that "substates" subset "exclusiveStates".
			 */
		}
		
		abstract state exclusiveStates: StateAction[0..*] :> substates {
			doc
			/*
			 * The substates of this state that are mutually exclusive, that is, whose performances do not
			 * overlap in time.
			 */
		}
		
		abstract action stateTransitions: StateTransitionAction[0..*] :> transitions {
			doc
			/*
			 * The transitions of this state that are state transitions.
			 */
		}
		
		succession stateSequencing first [0..1] exclusiveStates then [0..1] exclusiveStates {
			doc
			/*
			 * Exclusive states cannot overlap, so it must be possible to strictly sequence them in time.
			 */
		}
		assert constraint {notEmpty(exclusiveStates) implies size(stateSequencing) == size(exclusiveStates) - 1}
	}
	
	action def StateTransitionAction :> TransitionAction, StateTransitionPerformance {
		doc
		/*
		 * A StateTransitionAction is a TransitionAction and a StateTransitionPerformance whose transitionLinkSource 
		 * is a State. It is the base type of TransitionUsages used transitions in state models.
		 */
	
		in transitionLinkSource[1]: StateAction :>> 
			TransitionAction::transitionLinkSource, StateTransitionPerformance::transitionLinkSource;
			
		inout payload[0..*];
		in :>> receiver;
		
		bind payload = accepter.payload;
		bind receiver = accepter.receiver;
	}
	
	abstract state stateActions: StateAction[0..*] nonunique :> actions {
		doc
		/*
		 * stateActions is the base feature for all StateUsages.
		 */
	}
}
~~~
# EXPECTED
~~~
semantic.redefinition_featuring_type_overlap
semantic.redefinition_featuring_type_overlap
semantic.redefinition_featuring_type_overlap
semantic.unresolved_name 'Action'
semantic.unresolved_name 'StatePerformance'
semantic.unresolved_name 'Action'
semantic.unresolved_name 'Action'
semantic.unresolved_name 'isTriggerDuring'
semantic.unresolved_name 'Action::self'
semantic.unresolved_name 'StatePerformance::self'
semantic.unresolved_name 'Action::start'
semantic.unresolved_name 'StatePerformance::startShot'
semantic.unresolved_name 'Action::done'
semantic.unresolved_name 'StatePerformance::endShot'
semantic.unresolved_name 'subactions'
semantic.unresolved_name 'middle'
semantic.unresolved_name 'subactions'
semantic.unresolved_name 'transitions'
semantic.unresolved_name 'TransitionAction'
semantic.unresolved_name 'StateTransitionPerformance'
semantic.unresolved_name 'TransitionAction::transitionLinkSource'
semantic.unresolved_name 'StateTransitionPerformance::transitionLinkSource'
semantic.unresolved_name 'receiver'
semantic.unresolved_name 'actions'
~~~
# PROBLEMS
~~~
semantic.redefinition_featuring_type_overlap
semantic.redefinition_featuring_type_overlap
semantic.redefinition_featuring_type_overlap
semantic.unresolved_name 'Action'
semantic.unresolved_name 'StatePerformance'
semantic.unresolved_name 'Action'
semantic.unresolved_name 'Action'
semantic.unresolved_name 'isTriggerDuring'
semantic.unresolved_name 'Action::self'
semantic.unresolved_name 'StatePerformance::self'
semantic.unresolved_name 'Action::start'
semantic.unresolved_name 'StatePerformance::startShot'
semantic.unresolved_name 'Action::done'
semantic.unresolved_name 'StatePerformance::endShot'
semantic.unresolved_name 'subactions'
semantic.unresolved_name 'middle'
semantic.unresolved_name 'subactions'
semantic.unresolved_name 'transitions'
semantic.unresolved_name 'TransitionAction'
semantic.unresolved_name 'StateTransitionPerformance'
semantic.unresolved_name 'TransitionAction::transitionLinkSource'
semantic.unresolved_name 'StateTransitionPerformance::transitionLinkSource'
semantic.unresolved_name 'receiver'
semantic.unresolved_name 'actions'
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
KwAbstract,KwState,KwDef,Ident,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
KwEntry,KwAction,Ident,ColonGtGt,UnrestrictedName,Semicolon,
KwDo,KwAction,Ident,Colon,Ident,ColonGtGt,UnrestrictedName,Semicolon,
KwExit,KwAction,Ident,Colon,Ident,ColonGtGt,UnrestrictedName,Semicolon,
KwAttribute,ColonGtGt,Ident,Semicolon,
KwRef,KwState,Ident,Colon,Ident,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
KwRef,KwState,Ident,Colon,Ident,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
KwRef,KwState,Ident,Colon,Ident,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
KwAction,ColonGtGt,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAction,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwState,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwAction,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwSuccession,Ident,KwFirst,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,KwThen,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAssert,KwConstraint,OpenCurly,Ident,OpenParen,Ident,CloseParen,KwImplies,Ident,OpenParen,Ident,CloseParen,EqEq,Ident,OpenParen,Ident,CloseParen,Minus,DecimalValue,CloseCurly,
CloseCurly,
KwAction,KwDef,Ident,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,OpenSquare,DecimalValue,CloseSquare,Colon,Ident,ColonGtGt,
Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
KwInout,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwIn,ColonGtGt,Ident,Semicolon,
KwBind,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwBind,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwAbstract,KwState,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'States'
    (documentation)
    (import_decl private 'Occurrences::Occurrence')
    (import_decl private 'StatePerformances::StatePerformance')
    (import_decl private 'StatePerformances::StateTransitionPerformance')
    (import_decl private 'Actions::Action')
    (import_decl private 'Actions::TransitionAction')
    (import_decl private 'Actions::transitionActions')
    (import_decl private 'Actions::AcceptAction')
    (import_decl private 'Actions::actions')
    (import_decl private 'SequenceFunctions::notEmpty')
    (import_decl private 'SequenceFunctions::size')
    (state_def abstract 'StateAction' :> 'Action', 'StatePerformance'
      (documentation)
      (entry_action 'entryAction' :>> ''entry'')
      (do_action 'doAction' : 'Action' :>> ''do'')
      (exit_action 'exitAction' : 'Action' :>> ''exit'')
      (attribute_usage :>> 'isTriggerDuring')
      (state_usage ref 'self' : 'StateAction' :>> 'Action::self', 'StatePerformance::self')
      (state_usage ref 'start' : 'StateAction' :>> 'Action::start', 'StatePerformance::startShot')
      (state_usage ref 'done' : 'StateAction' :>> 'Action::done', 'StatePerformance::endShot')
      (action_usage :>> 'subactions' :> 'middle'
        (documentation))
      (action_usage 'substates' : 'StateAction' multiplicity :> 'stateActions', 'subactions'
        (documentation))
      (state_usage abstract 'exclusiveStates' : 'StateAction' :> 'substates' multiplicity
        (documentation))
      (action_usage abstract 'stateTransitions' : 'StateTransitionAction' multiplicity :> 'transitions'
        (documentation))
      (succession_as_usage 'stateSequencing'
        (connector_end)
        (connector_end)
        (documentation))
      (sysml_decl
        (result_expr_member)))
    (action_def 'StateTransitionAction' :> 'TransitionAction', 'StateTransitionPerformance'
      (documentation)
      (default_ref_usage in 'transitionLinkSource' : 'StateAction' :>> 'TransitionAction::transitionLinkSource', 'StateTransitionPerformance::transitionLinkSource' multiplicity)
      (default_ref_usage inout 'payload' multiplicity)
      (default_ref_usage in :>> 'receiver')
      (binding_as_usage
        (connector_end)
        (connector_end))
      (binding_as_usage
        (connector_end)
        (connector_end)))
    (state_usage abstract 'stateActions' : 'StateAction' :> 'actions' multiplicity nonunique
      (documentation))))
~~~
# FORMAT
~~~sysml
standard library package States {
    doc /*
	 * This package defines the base types for states and related behavioral elements in the
	 * SysML language.
	 */

    private import Occurrences::Occurrence;
    private import StatePerformances::StatePerformance;
    private import StatePerformances::StateTransitionPerformance;
    private import Actions::Action;
    private import Actions::TransitionAction;
    private import Actions::transitionActions;
    private import Actions::AcceptAction;
    private import Actions::actions;
    private import SequenceFunctions::notEmpty;
    private import SequenceFunctions::size;

    abstract state def StateAction :> Action, StatePerformance {
        doc /*
		 * A StateAction is a kind of Action that is also a StatePerformance. It is the base type for all
		 * StateDefinitions.
		 */

        entry entryAction :>> 'entry';
        do doAction : Action :>> 'do';
        exit exitAction : Action :>> 'exit';

        attribute :>> isTriggerDuring;

        ref state self : StateAction :>> Action::self, StatePerformance::self;
        ref state start : StateAction :>> Action::start, StatePerformance::startShot;
        ref state done : StateAction :>> Action::done, StatePerformance::endShot;

        action :>> subactions :> middle {
            doc /*
			 * The subperformances of this StateAction that are Actions, other than the entry and exit Actions. 
			 * These subactions all take place in the "middle" of the StatePerformance, that is, after the 
			 * entry Action and before the exit Action. 
			 */
        }

        action substates : StateAction [0..*] :> stateActions, subactions {
            doc /*
			 * The subactions of this state that are states.
			 * 
			 * NOTE: This feature is declared as an ActionUsage, not a StateUsage, so that the constraint 
 			 * checkStateUsageExclusiveStateSpecialization does not apply to it, since this constraint 
			 * would otherwise incorrectly require that "substates" subset "exclusiveStates".
			 */
        }

        abstract state exclusiveStates : StateAction :> substates [0..*] {
            doc /*
			 * The substates of this state that are mutually exclusive, that is, whose performances do not
			 * overlap in time.
			 */
        }

        abstract action stateTransitions : StateTransitionAction [0..*] :> transitions {
            doc /*
			 * The transitions of this state that are state transitions.
			 */
        }

        succession stateSequencing first [0..1] exclusiveStates then [0..1] exclusiveStates {
            doc /*
			 * Exclusive states cannot overlap, so it must be possible to strictly sequence them in time.
			 */
        }
        assert constraint {
            = notEmpty(exclusiveStates) implies size(stateSequencing) == size(exclusiveStates) - 1;
        }
    }

    action def StateTransitionAction :> TransitionAction, StateTransitionPerformance {
        doc /*
		 * A StateTransitionAction is a TransitionAction and a StateTransitionPerformance whose transitionLinkSource 
		 * is a State. It is the base type of TransitionUsages used transitions in state models.
		 */

        in transitionLinkSource : StateAction :>> TransitionAction::transitionLinkSource, StateTransitionPerformance::transitionLinkSource [1];

        inout payload [0..*];
        in :>> receiver;

        bind payload = accepter.payload;
        bind receiver = accepter.receiver;
    }

    abstract state stateActions : StateAction :> actions [0..*] nonunique {
        doc /*
		 * stateActions is the base feature for all StateUsages.
		 */
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'States'
      (documentation)
      (membership_import private -> 'Occurrences::Occurrence'[unresolved])
      (membership_import private -> 'StatePerformances::StatePerformance'[unresolved])
      (membership_import private -> 'StatePerformances::StateTransitionPerformance'[unresolved])
      (membership_import private -> 'Actions::Action'[unresolved])
      (membership_import private -> 'Actions::TransitionAction'[unresolved])
      (membership_import private -> 'Actions::transitionActions'[unresolved])
      (membership_import private -> 'Actions::AcceptAction'[unresolved])
      (membership_import private -> 'Actions::actions'[unresolved])
      (membership_import private -> 'SequenceFunctions::notEmpty'[unresolved])
      (membership_import private -> 'SequenceFunctions::size'[unresolved])
      (state_def abstract 'StateAction' :> 'Action'[unresolved] :> 'StatePerformance'[unresolved]
        (documentation)
        (state_subaction_membership 'entry'
          (action_usage 'entryAction' :>> 'States::StateAction::entry'[state_subaction_membership]))
        (state_subaction_membership 'do'
          (action_usage 'doAction' : 'Action'[unresolved] :>> 'States::StateAction::do'[state_subaction_membership]))
        (state_subaction_membership 'exit'
          (action_usage 'exitAction' : 'Action'[unresolved] :>> 'States::StateAction::exit'[state_subaction_membership]))
        (attribute_usage composite :>> 'isTriggerDuring'[unresolved])
        (state_usage reference 'self' : 'States::StateAction'[state_def] :>> 'Action::self'[unresolved] :>> 'StatePerformance::self'[unresolved] :> 'States::stateActions'[state_usage][implied])
        (state_usage reference 'start' : 'States::StateAction'[state_def] :>> 'Action::start'[unresolved] :>> 'StatePerformance::startShot'[unresolved] :> 'States::stateActions'[state_usage][implied])
        (state_usage reference 'done' : 'States::StateAction'[state_def] :>> 'Action::done'[unresolved] :>> 'StatePerformance::endShot'[unresolved] :> 'States::stateActions'[state_usage][implied])
        (action_usage composite :>> 'subactions'[unresolved] :> 'middle'[unresolved]
          (documentation))
        (action_usage composite 'substates' : 'States::StateAction'[state_def] :> 'States::stateActions'[state_usage] :> 'subactions'[unresolved]
          (multiplicity_range [0..*])
          (documentation))
        (state_usage abstract composite 'exclusiveStates' : 'States::StateAction'[state_def] :> 'States::StateAction::substates'[action_usage]
          (multiplicity_range [0..*])
          (documentation))
        (action_usage abstract composite 'stateTransitions' : 'States::StateTransitionAction'[action_def] :> 'transitions'[unresolved]
          (multiplicity_range [0..*])
          (documentation))
        (succession_def 'stateSequencing'
          (connector_end 'exclusiveStates')
          (connector_end 'exclusiveStates')
          (documentation))
        (assert_constraint_usage
          (result_expr_membership)))
      (action_def 'StateTransitionAction' :> 'TransitionAction'[unresolved] :> 'StateTransitionPerformance'[unresolved]
        (documentation)
        (reference_usage in reference 'transitionLinkSource' : 'States::StateAction'[state_def] :>> 'TransitionAction::transitionLinkSource'[unresolved] :>> 'StateTransitionPerformance::transitionLinkSource'[unresolved]
          (multiplicity_range [1]))
        (reference_usage inout reference 'payload'
          (multiplicity_range [0..*]))
        (reference_usage in reference :>> 'receiver'[unresolved])
        (binding_connector_def
          (connector_end 'payload')
          (connector_end 'accepter.payload'))
        (binding_connector_def
          (connector_end 'receiver')
          (connector_end 'accepter.receiver')))
      (state_usage abstract 'stateActions' : 'States::StateAction'[state_def] :> 'actions'[unresolved]
        (multiplicity_range [0..*])
        (documentation)))))
~~~
