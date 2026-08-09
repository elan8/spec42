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
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "States"))) (name "States") (declared-name "States")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "States::AcceptAction"))) (name "AcceptAction") (declared-name "AcceptAction"))
        (element (kind "import") (id (node (document "d0") (qualified-name "States::Action"))) (name "Action") (declared-name "Action"))
        (element (kind "import") (id (node (document "d0") (qualified-name "States::Occurrence"))) (name "Occurrence") (declared-name "Occurrence"))
        (element (kind "state def") (id (node (document "d0") (qualified-name "States::StateAction"))) (name "StateAction") (declared-name "StateAction")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "States::StateAction::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "States::StateAction")))))
            (element (kind "ref") (id (node (document "d0") (qualified-name "States::StateAction::done"))) (name "done") (declared-name "done") (declared (properties (composite false) (reference true))) (effective (featuring-type (node (document "d0") (qualified-name "States::StateAction")))))
            (element (kind "state") (id (node (document "d0") (qualified-name "States::StateAction::exclusiveStates"))) (name "exclusiveStates") (declared-name "exclusiveStates") (effective (featuring-type (node (document "d0") (qualified-name "States::StateAction"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "States::StateAction::exclusiveStates::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "States::StateAction")))))
              )
            )
            (element (kind "ref") (id (node (document "d0") (qualified-name "States::StateAction::self"))) (name "self") (declared-name "self") (declared (properties (composite false) (reference true))) (effective (featuring-type (node (document "d0") (qualified-name "States::StateAction")))))
            (element (kind "ref") (id (node (document "d0") (qualified-name "States::StateAction::start"))) (name "start") (declared-name "start") (declared (properties (composite false) (reference true))) (effective (featuring-type (node (document "d0") (qualified-name "States::StateAction")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "States::StatePerformance"))) (name "StatePerformance") (declared-name "StatePerformance"))
        (element (kind "action def") (id (node (document "d0") (qualified-name "States::StateTransitionAction"))) (name "StateTransitionAction") (declared-name "StateTransitionAction")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "States::StateTransitionAction::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "States::StateTransitionAction")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "States::StateTransitionAction::payload"))) (name "payload") (declared-name "payload") (effective (featuring-type (node (document "d0") (qualified-name "States::StateTransitionAction")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "States::StateTransitionAction::receiver"))) (name "receiver") (declared-name "receiver") (effective (featuring-type (node (document "d0") (qualified-name "States::StateTransitionAction")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "States::StateTransitionAction::transitionLinkSource"))) (name "transitionLinkSource") (declared-name "transitionLinkSource") (effective (featuring-type (node (document "d0") (qualified-name "States::StateTransitionAction")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "States::StateTransitionPerformance"))) (name "StateTransitionPerformance") (declared-name "StateTransitionPerformance"))
        (element (kind "import") (id (node (document "d0") (qualified-name "States::TransitionAction"))) (name "TransitionAction") (declared-name "TransitionAction"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "States::_documentation"))) (name ""))
        (element (kind "import") (id (node (document "d0") (qualified-name "States::actions"))) (name "actions") (declared-name "actions"))
        (element (kind "import") (id (node (document "d0") (qualified-name "States::notEmpty"))) (name "notEmpty") (declared-name "notEmpty"))
        (element (kind "import") (id (node (document "d0") (qualified-name "States::size"))) (name "size") (declared-name "size"))
        (element (kind "state") (id (node (document "d0") (qualified-name "States::stateActions"))) (name "stateActions") (declared-name "stateActions") (declared (properties (abstract true) (composite true) (reference false)) (multiplicity (lower 0) (upper unbounded) (ordered false) (provenance authored)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "States::stateActions::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "States::StateAction")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "States::transitionActions"))) (name "transitionActions") (declared-name "transitionActions"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "States::StateAction::_documentation"))) (to (node (document "d0") (qualified-name "States::StateAction"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "States::StateAction::exclusiveStates::_documentation"))) (to (node (document "d0") (qualified-name "States::StateAction::exclusiveStates"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "States::StateTransitionAction::_documentation"))) (to (node (document "d0") (qualified-name "States::StateTransitionAction"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "States::_documentation"))) (to (node (document "d0") (qualified-name "States"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "States::stateActions::_documentation"))) (to (node (document "d0") (qualified-name "States::stateActions"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "States::stateActions"))) (to (node (document "d0") (qualified-name "States::actions"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "States::StateAction::done"))) (to (node (document "d0") (qualified-name "States::StateAction"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "States::StateAction::exclusiveStates"))) (to (node (document "d0") (qualified-name "States::StateAction"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "States::StateAction::self"))) (to (node (document "d0") (qualified-name "States::StateAction"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "States::StateAction::start"))) (to (node (document "d0") (qualified-name "States::StateAction"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "States::stateActions"))) (to (node (document "d0") (qualified-name "States::StateAction"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
    (bind (status pending-expression) (document "d0") (source-expression "payload") (target-expression "accepter::payload") (container-prefix "States::StateTransitionAction"))
    (bind (status pending-expression) (document "d0") (source-expression "receiver") (target-expression "accepter::receiver") (container-prefix "States::StateTransitionAction"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml.library/states.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 1) (end 7 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 1) (end 8 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 1) (end 9 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 1) (end 10 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 1) (end 11 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 12 1) (end 12 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 13 1) (end 13 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 14 1) (end 14 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 15 1) (end 15 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 16 1) (end 16 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 18 1) (end 18 2048))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 18 1) (end 18 2048))
      )
      (diagnostic
        (severity information)
        (code "missing_final_state")
        (source "semantic")
        (range (start 18 1) (end 18 2048))
      )
      (diagnostic
        (severity information)
        (code "missing_initial_state")
        (source "semantic")
        (range (start 18 1) (end 18 2048))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 79 1) (end 79 569))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 79 1) (end 79 569))
      )
      (diagnostic
        (severity error)
        (code "unresolved_pending_expression_relationship")
        (source "semantic")
        (range (start 92 7) (end 92 14))
      )
      (diagnostic
        (severity error)
        (code "unresolved_pending_expression_relationship")
        (source "semantic")
        (range (start 93 7) (end 93 15))
      )
    )
  )
)
~~~
