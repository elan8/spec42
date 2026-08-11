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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "states.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 16) (end 9 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 16) (end 10 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 16) (end 11 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 12 16) (end 12 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 13 16) (end 13 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 14 16) (end 14 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 15 16) (end 15 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 16 16) (end 16 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 86 2) (end 86 139))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 89 2) (end 89 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 92 17) (end 92 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 93 18) (end 93 35))
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
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "5421ec0680183ac1e26759ae6e3beee7ba56192227113ea03cef868138df1a49") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "States"))) (kind "package") (name "States") (declared-name "States") (range (start (line 0) (character 0)) (end (line 0) (character 3371))))
    (element (id (node (document "d0") (qualified-name "States::AcceptAction"))) (kind "import") (name "AcceptAction") (declared-name "AcceptAction") (range (start (line 13) (character 1)) (end (line 13) (character 38))) (parent (node (document "d0") (qualified-name "States"))) (authored (membership (kind Import) (visibility "private") (import (reference "Actions::AcceptAction") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 13) (character 16)) (end (line 13) (character 37))))))
    (element (id (node (document "d0") (qualified-name "States::Action"))) (kind "import") (name "Action") (declared-name "Action") (range (start (line 10) (character 1)) (end (line 10) (character 32))) (parent (node (document "d0") (qualified-name "States"))) (authored (membership (kind Import) (visibility "private") (import (reference "Actions::Action") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 10) (character 16)) (end (line 10) (character 31))))))
    (element (id (node (document "d0") (qualified-name "States::Occurrence"))) (kind "import") (name "Occurrence") (declared-name "Occurrence") (range (start (line 7) (character 1)) (end (line 7) (character 40))) (parent (node (document "d0") (qualified-name "States"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::Occurrence") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 16)) (end (line 7) (character 39))))))
    (element (id (node (document "d0") (qualified-name "States::StateAction"))) (kind "state def") (name "StateAction") (declared-name "StateAction") (range (start (line 18) (character 1)) (end (line 18) (character 2048))) (parent (node (document "d0") (qualified-name "States"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Action") (range (start (line 18) (character 35)) (end (line 18) (character 41)))) (specializes (reference "StatePerformance") (range (start (line 18) (character 43)) (end (line 18) (character 59)))))))
    (element (id (node (document "d0") (qualified-name "States::StateAction::_documentation"))) (kind "documentation") (name "") (range (start (line 18) (character 1)) (end (line 18) (character 2048))) (parent (node (document "d0") (qualified-name "States::StateAction"))))
    (element (id (node (document "d0") (qualified-name "States::StateAction::done"))) (kind "ref") (name "done") (declared-name "done") (range (start (line 33) (character 2)) (end (line 33) (character 74))) (parent (node (document "d0") (qualified-name "States::StateAction"))) (authored (membership (kind Feature)) (relationships (typing (reference "StateAction") (range (start (line 33) (character 18)) (end (line 33) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "States::StateAction::exclusiveStates"))) (kind "state") (name "exclusiveStates") (declared-name "exclusiveStates") (range (start (line 55) (character 2)) (end (line 55) (character 211))) (parent (node (document "d0") (qualified-name "States::StateAction"))) (authored (membership (kind Feature)) (relationships (typing (reference "StateAction") (range none)))))
    (element (id (node (document "d0") (qualified-name "States::StateAction::exclusiveStates::_documentation"))) (kind "documentation") (name "") (range (start (line 55) (character 2)) (end (line 55) (character 211))) (parent (node (document "d0") (qualified-name "States::StateAction::exclusiveStates"))))
    (element (id (node (document "d0") (qualified-name "States::StateAction::self"))) (kind "ref") (name "self") (declared-name "self") (range (start (line 31) (character 2)) (end (line 31) (character 71))) (parent (node (document "d0") (qualified-name "States::StateAction"))) (authored (membership (kind Feature)) (relationships (typing (reference "StateAction") (range (start (line 31) (character 18)) (end (line 31) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "States::StateAction::start"))) (kind "ref") (name "start") (declared-name "start") (range (start (line 32) (character 2)) (end (line 32) (character 78))) (parent (node (document "d0") (qualified-name "States::StateAction"))) (authored (membership (kind Feature)) (relationships (typing (reference "StateAction") (range (start (line 32) (character 19)) (end (line 32) (character 30)))))))
    (element (id (node (document "d0") (qualified-name "States::StatePerformance"))) (kind "import") (name "StatePerformance") (declared-name "StatePerformance") (range (start (line 8) (character 1)) (end (line 8) (character 52))) (parent (node (document "d0") (qualified-name "States"))) (authored (membership (kind Import) (visibility "private") (import (reference "StatePerformances::StatePerformance") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 8) (character 16)) (end (line 8) (character 51))))))
    (element (id (node (document "d0") (qualified-name "States::StateTransitionAction"))) (kind "action def") (name "StateTransitionAction") (declared-name "StateTransitionAction") (range (start (line 79) (character 1)) (end (line 79) (character 569))) (parent (node (document "d0") (qualified-name "States"))) (authored (membership (kind Owning)) (relationships (specializes (reference "TransitionAction") (range none)) (specializes (reference "StateTransitionPerformance") (range none)) (specializes (reference "TransitionAction") (range none)) (specializes (reference "StateTransitionPerformance") (range none)) (specializes (reference "TransitionAction") (range (start (line 79) (character 37)) (end (line 79) (character 53)))) (specializes (reference "StateTransitionPerformance") (range (start (line 79) (character 55)) (end (line 79) (character 81)))))))
    (element (id (node (document "d0") (qualified-name "States::StateTransitionAction::_documentation"))) (kind "documentation") (name "") (range (start (line 79) (character 1)) (end (line 79) (character 569))) (parent (node (document "d0") (qualified-name "States::StateTransitionAction"))))
    (element (id (node (document "d0") (qualified-name "States::StateTransitionAction::payload"))) (kind "in out parameter") (name "payload") (declared-name "payload") (range (start (line 89) (character 2)) (end (line 89) (character 22))) (parent (node (document "d0") (qualified-name "States::StateTransitionAction"))) (authored (relationships (typing (reference "payload[0..*]") (range none)))))
    (element (id (node (document "d0") (qualified-name "States::StateTransitionAction::receiver"))) (kind "in out parameter") (name "receiver") (declared-name "receiver") (range (start (line 90) (character 2)) (end (line 90) (character 18))) (parent (node (document "d0") (qualified-name "States::StateTransitionAction"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "States::StateTransitionAction::transitionLinkSource"))) (kind "in out parameter") (name "transitionLinkSource") (declared-name "transitionLinkSource") (range (start (line 86) (character 2)) (end (line 86) (character 139))) (parent (node (document "d0") (qualified-name "States::StateTransitionAction"))) (authored (relationships (typing (reference "transitionLinkSource[1]: StateAction :>> \n\t\t\tTransitionAction::transitionLinkSource, StateTransitionPerformance::transitionLinkSource") (range none)))))
    (element (id (node (document "d0") (qualified-name "States::StateTransitionPerformance"))) (kind "import") (name "StateTransitionPerformance") (declared-name "StateTransitionPerformance") (range (start (line 9) (character 1)) (end (line 9) (character 62))) (parent (node (document "d0") (qualified-name "States"))) (authored (membership (kind Import) (visibility "private") (import (reference "StatePerformances::StateTransitionPerformance") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 9) (character 16)) (end (line 9) (character 61))))))
    (element (id (node (document "d0") (qualified-name "States::TransitionAction"))) (kind "import") (name "TransitionAction") (declared-name "TransitionAction") (range (start (line 11) (character 1)) (end (line 11) (character 42))) (parent (node (document "d0") (qualified-name "States"))) (authored (membership (kind Import) (visibility "private") (import (reference "Actions::TransitionAction") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 11) (character 16)) (end (line 11) (character 41))))))
    (element (id (node (document "d0") (qualified-name "States::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 3371))) (parent (node (document "d0") (qualified-name "States"))))
    (element (id (node (document "d0") (qualified-name "States::actions"))) (kind "import") (name "actions") (declared-name "actions") (range (start (line 14) (character 1)) (end (line 14) (character 33))) (parent (node (document "d0") (qualified-name "States"))) (authored (membership (kind Import) (visibility "private") (import (reference "Actions::actions") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 14) (character 16)) (end (line 14) (character 32))))))
    (element (id (node (document "d0") (qualified-name "States::notEmpty"))) (kind "import") (name "notEmpty") (declared-name "notEmpty") (range (start (line 15) (character 1)) (end (line 15) (character 44))) (parent (node (document "d0") (qualified-name "States"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::notEmpty") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 15) (character 16)) (end (line 15) (character 43))))))
    (element (id (node (document "d0") (qualified-name "States::size"))) (kind "import") (name "size") (declared-name "size") (range (start (line 16) (character 1)) (end (line 16) (character 40))) (parent (node (document "d0") (qualified-name "States"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::size") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 16) (character 16)) (end (line 16) (character 39))))))
    (element (id (node (document "d0") (qualified-name "States::stateActions"))) (kind "state") (name "stateActions") (declared-name "stateActions") (range (start (line 96) (character 1)) (end (line 96) (character 149))) (parent (node (document "d0") (qualified-name "States"))) (authored (membership (kind Feature)) (relationships (typing (reference "StateAction") (range none)) (subsetting (reference "actions") (range (start (line 96) (character 61)) (end (line 96) (character 68)))))))
    (element (id (node (document "d0") (qualified-name "States::stateActions::_documentation"))) (kind "documentation") (name "") (range (start (line 96) (character 1)) (end (line 96) (character 149))) (parent (node (document "d0") (qualified-name "States::stateActions"))))
    (element (id (node (document "d0") (qualified-name "States::transitionActions"))) (kind "import") (name "transitionActions") (declared-name "transitionActions") (range (start (line 12) (character 1)) (end (line 12) (character 43))) (parent (node (document "d0") (qualified-name "States"))) (authored (membership (kind Import) (visibility "private") (import (reference "Actions::transitionActions") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 12) (character 16)) (end (line 12) (character 42))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "States::AcceptAction"))) (kind membershipImport) (ordinal 0)) (authored-target "Actions::AcceptAction") (range (start (line 13) (character 16)) (end (line 13) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "States::Action"))) (kind membershipImport) (ordinal 0)) (authored-target "Actions::Action") (range (start (line 10) (character 16)) (end (line 10) (character 31))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "States::Occurrence"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::Occurrence") (range (start (line 7) (character 16)) (end (line 7) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "States::StateAction"))) (kind specialization) (ordinal 0)) (authored-target "Action") (range (start (line 18) (character 35)) (end (line 18) (character 41))) (outcome (status resolved) (target (node (document "d0") (qualified-name "States::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "States::StateAction"))) (kind specialization) (ordinal 1)) (authored-target "StatePerformance") (range (start (line 18) (character 43)) (end (line 18) (character 59))) (outcome (status resolved) (target (node (document "d0") (qualified-name "States::StatePerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "States::StateAction::done"))) (kind featureTyping) (ordinal 0)) (authored-target "StateAction") (range (start (line 33) (character 18)) (end (line 33) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "States::StateAction")))))
    (reference (id (source (node (document "d0") (qualified-name "States::StateAction::exclusiveStates"))) (kind featureTyping) (ordinal 0)) (authored-target "StateAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "States::StateAction")))))
    (reference (id (source (node (document "d0") (qualified-name "States::StateAction::self"))) (kind featureTyping) (ordinal 0)) (authored-target "StateAction") (range (start (line 31) (character 18)) (end (line 31) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "States::StateAction")))))
    (reference (id (source (node (document "d0") (qualified-name "States::StateAction::start"))) (kind featureTyping) (ordinal 0)) (authored-target "StateAction") (range (start (line 32) (character 19)) (end (line 32) (character 30))) (outcome (status resolved) (target (node (document "d0") (qualified-name "States::StateAction")))))
    (reference (id (source (node (document "d0") (qualified-name "States::StatePerformance"))) (kind membershipImport) (ordinal 0)) (authored-target "StatePerformances::StatePerformance") (range (start (line 8) (character 16)) (end (line 8) (character 51))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "States::StateTransitionAction"))) (kind specialization) (ordinal 0)) (authored-target "TransitionAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "States::TransitionAction")))))
    (reference (id (source (node (document "d0") (qualified-name "States::StateTransitionAction"))) (kind specialization) (ordinal 1)) (authored-target "StateTransitionPerformance") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "States::StateTransitionPerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "States::StateTransitionAction"))) (kind specialization) (ordinal 2)) (authored-target "TransitionAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "States::TransitionAction")))))
    (reference (id (source (node (document "d0") (qualified-name "States::StateTransitionAction"))) (kind specialization) (ordinal 3)) (authored-target "StateTransitionPerformance") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "States::StateTransitionPerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "States::StateTransitionAction"))) (kind specialization) (ordinal 4)) (authored-target "TransitionAction") (range (start (line 79) (character 37)) (end (line 79) (character 53))) (outcome (status resolved) (target (node (document "d0") (qualified-name "States::TransitionAction")))))
    (reference (id (source (node (document "d0") (qualified-name "States::StateTransitionAction"))) (kind specialization) (ordinal 5)) (authored-target "StateTransitionPerformance") (range (start (line 79) (character 55)) (end (line 79) (character 81))) (outcome (status resolved) (target (node (document "d0") (qualified-name "States::StateTransitionPerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "States::StateTransitionAction"))) (kind bindSource) (ordinal 0)) (authored-target "payload") (range (start (line 92) (character 7)) (end (line 92) (character 14))) (outcome (status resolved) (target (node (document "d0") (qualified-name "States::StateTransitionAction::payload")))))
    (reference (id (source (node (document "d0") (qualified-name "States::StateTransitionAction"))) (kind bindSource) (ordinal 1)) (authored-target "receiver") (range (start (line 93) (character 7)) (end (line 93) (character 15))) (outcome (status resolved) (target (node (document "d0") (qualified-name "States::StateTransitionAction::receiver")))))
    (reference (id (source (node (document "d0") (qualified-name "States::StateTransitionAction"))) (kind bindTarget) (ordinal 0)) (authored-target "accepter::payload") (range (start (line 92) (character 17)) (end (line 92) (character 33))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "States::StateTransitionAction"))) (kind bindTarget) (ordinal 1)) (authored-target "accepter::receiver") (range (start (line 93) (character 18)) (end (line 93) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "States::StateTransitionAction::payload"))) (kind featureTyping) (ordinal 0)) (authored-target "payload[0..*]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "States::StateTransitionAction::receiver"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "States::StateTransitionAction::_documentation")))))
    (reference (id (source (node (document "d0") (qualified-name "States::StateTransitionAction::transitionLinkSource"))) (kind featureTyping) (ordinal 0)) (authored-target "transitionLinkSource[1]: StateAction :>> \n\t\t\tTransitionAction::transitionLinkSource, StateTransitionPerformance::transitionLinkSource") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "States::StateTransitionPerformance"))) (kind membershipImport) (ordinal 0)) (authored-target "StatePerformances::StateTransitionPerformance") (range (start (line 9) (character 16)) (end (line 9) (character 61))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "States::TransitionAction"))) (kind membershipImport) (ordinal 0)) (authored-target "Actions::TransitionAction") (range (start (line 11) (character 16)) (end (line 11) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "States::actions"))) (kind membershipImport) (ordinal 0)) (authored-target "Actions::actions") (range (start (line 14) (character 16)) (end (line 14) (character 32))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "States::notEmpty"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::notEmpty") (range (start (line 15) (character 16)) (end (line 15) (character 43))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "States::size"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::size") (range (start (line 16) (character 16)) (end (line 16) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "States::stateActions"))) (kind featureTyping) (ordinal 0)) (authored-target "StateAction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "States::StateAction")))))
    (reference (id (source (node (document "d0") (qualified-name "States::stateActions"))) (kind subsetting) (ordinal 0)) (authored-target "actions") (range (start (line 96) (character 61)) (end (line 96) (character 68))) (outcome (status resolved) (target (node (document "d0") (qualified-name "States::actions")))))
    (reference (id (source (node (document "d0") (qualified-name "States::transitionActions"))) (kind membershipImport) (ordinal 0)) (authored-target "Actions::transitionActions") (range (start (line 12) (character 16)) (end (line 12) (character 42))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "States::StateAction"))) (target (node (document "d0") (qualified-name "States::Action"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "States::StateAction"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "States::StateAction"))) (target (node (document "d0") (qualified-name "States::StatePerformance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "States::StateAction"))) (kind specialization) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "States::StateAction::done"))) (target (node (document "d0") (qualified-name "States::StateAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "States::StateAction::done"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "States::StateAction::exclusiveStates"))) (target (node (document "d0") (qualified-name "States::StateAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "States::StateAction::exclusiveStates"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "States::StateAction::self"))) (target (node (document "d0") (qualified-name "States::StateAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "States::StateAction::self"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "States::StateAction::start"))) (target (node (document "d0") (qualified-name "States::StateAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "States::StateAction::start"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "States::StateTransitionAction"))) (target (node (document "d0") (qualified-name "States::StateTransitionPerformance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "States::StateTransitionAction"))) (kind specialization) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "States::StateTransitionAction"))) (target (node (document "d0") (qualified-name "States::StateTransitionPerformance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "States::StateTransitionAction"))) (kind specialization) (ordinal 3)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "States::StateTransitionAction"))) (target (node (document "d0") (qualified-name "States::StateTransitionPerformance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "States::StateTransitionAction"))) (kind specialization) (ordinal 5)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "States::StateTransitionAction"))) (target (node (document "d0") (qualified-name "States::TransitionAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "States::StateTransitionAction"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "States::StateTransitionAction"))) (target (node (document "d0") (qualified-name "States::TransitionAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "States::StateTransitionAction"))) (kind specialization) (ordinal 2)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "States::StateTransitionAction"))) (target (node (document "d0") (qualified-name "States::TransitionAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "States::StateTransitionAction"))) (kind specialization) (ordinal 4)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "States::StateTransitionAction::receiver"))) (target (node (document "d0") (qualified-name "States::StateTransitionAction::_documentation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "States::StateTransitionAction::receiver"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "States::stateActions"))) (target (node (document "d0") (qualified-name "States::StateAction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "States::stateActions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "States::stateActions"))) (target (node (document "d0") (qualified-name "States::actions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "States::stateActions"))) (kind subsetting) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
