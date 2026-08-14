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
  (document "memory://snapshot/states.md"
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
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 18 35) (end 18 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 18 43) (end 18 59))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 25 2) (end 26 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 26 2) (end 27 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 27 2) (end 29 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 29 2) (end 31 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 35 2) (end 44 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 44 2) (end 55 2))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 55 55) (end 55 64))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 63 2) (end 70 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 70 2) (end 76 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 76 2) (end 77 1))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 79 37) (end 79 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 79 55) (end 79 81))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 87 3) (end 87 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 87 43) (end 87 91))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 90 9) (end 90 17))
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
        (range (start 93 7) (end 93 15))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 93 18) (end 93 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 96 61) (end 96 68))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:5e47aa8945167801f31fee2834add515c580320b0c2e6ba268b29953c8156f61") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/states.md") (qualified-name "States"))) (kind library-package) (membership (kind owning) (visibility default)) (facts (modifiers standard)) (documentation (doc (text "\n\t * This package defines the base types for states and related behavioral elements in the\n\t * SysML language.\n\t "))))
    (declaration (id (node (document "memory://snapshot/states.md") (path (named (kind library-package) (name "States")) (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Occurrences::Occurrence") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/states.md") (path (named (kind library-package) (name "States")) (anonymous (kind import) (ordinal 1)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "StatePerformances::StatePerformance") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/states.md") (path (named (kind library-package) (name "States")) (anonymous (kind import) (ordinal 2)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "StatePerformances::StateTransitionPerformance") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/states.md") (path (named (kind library-package) (name "States")) (anonymous (kind import) (ordinal 3)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Actions::Action") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/states.md") (path (named (kind library-package) (name "States")) (anonymous (kind import) (ordinal 4)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Actions::TransitionAction") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/states.md") (path (named (kind library-package) (name "States")) (anonymous (kind import) (ordinal 5)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Actions::transitionActions") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/states.md") (path (named (kind library-package) (name "States")) (anonymous (kind import) (ordinal 6)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Actions::AcceptAction") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/states.md") (path (named (kind library-package) (name "States")) (anonymous (kind import) (ordinal 7)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Actions::actions") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/states.md") (path (named (kind library-package) (name "States")) (anonymous (kind import) (ordinal 8)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::notEmpty") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/states.md") (path (named (kind library-package) (name "States")) (anonymous (kind import) (ordinal 9)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::size") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/states.md") (qualified-name "States::StateAction"))) (kind state-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * A StateAction is a kind of Action that is also a StatePerformance. It is the base type for all\n\t\t * StateDefinitions.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Action")) (specialization (reference "StatePerformance"))))
    (declaration (id (node (document "memory://snapshot/states.md") (qualified-name "States::StateAction::done"))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "StateAction"))))
    (declaration (id (node (document "memory://snapshot/states.md") (qualified-name "States::StateAction::exclusiveStates"))) (kind state) (membership (kind feature) (visibility default)) (facts (modifiers abstract) (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n\t\t\t * The substates of this state that are mutually exclusive, that is, whose performances do not\n\t\t\t * overlap in time.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "StateAction")) (subsetting (reference "substates"))))
    (declaration (id (node (document "memory://snapshot/states.md") (qualified-name "States::StateAction::self"))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "StateAction"))))
    (declaration (id (node (document "memory://snapshot/states.md") (qualified-name "States::StateAction::start"))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "StateAction"))))
    (declaration (id (node (document "memory://snapshot/states.md") (qualified-name "States::StateTransitionAction"))) (kind action-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * A StateTransitionAction is a TransitionAction and a StateTransitionPerformance whose transitionLinkSource \n\t\t * is a State. It is the base type of TransitionUsages used transitions in state models.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "TransitionAction")) (specialization (reference "StateTransitionPerformance"))))
    (declaration (id (node (document "memory://snapshot/states.md") (path (named (kind library-package) (name "States")) (named (kind action-def) (name "StateTransitionAction")) (anonymous (kind parameter) (ordinal 0)))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "receiver"))))
    (declaration (id (node (document "memory://snapshot/states.md") (path (named (kind library-package) (name "States")) (named (kind action-def) (name "StateTransitionAction")) (anonymous (kind bind) (ordinal 0)))))) (kind bind) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindSource (reference "payload")) (memberAccessOperand (reference "accepter::payload"))))
    (declaration (id (node (document "memory://snapshot/states.md") (path (named (kind library-package) (name "States")) (named (kind action-def) (name "StateTransitionAction")) (anonymous (kind bind) (ordinal 1)))))) (kind bind) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindSource (reference "receiver")) (memberAccessOperand (reference "accepter::receiver"))))
    (declaration (id (node (document "memory://snapshot/states.md") (qualified-name "States::StateTransitionAction::payload"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction inout) (multiplicity (lower 0) (upper unbounded))))
    (declaration (id (node (document "memory://snapshot/states.md") (qualified-name "States::StateTransitionAction::transitionLinkSource"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "StateAction") (direction in)) (redefinition (reference "TransitionAction::transitionLinkSource")) (redefinition (reference "StateTransitionPerformance::transitionLinkSource"))))
    (declaration (id (node (document "memory://snapshot/states.md") (qualified-name "States::stateActions"))) (kind state) (membership (kind feature) (visibility default)) (facts (modifiers abstract) (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n\t\t * stateActions is the base feature for all StateUsages.\n\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "StateAction")) (subsetting (reference "actions"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/states.md") (path (named (kind library-package) (name "States")) (anonymous (kind import) (ordinal 0)))))) (kind membershipImport) (ordinal 0))
      (authored-target "Occurrences::Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/states.md") (path (named (kind library-package) (name "States")) (anonymous (kind import) (ordinal 1)))))) (kind membershipImport) (ordinal 0))
      (authored-target "StatePerformances::StatePerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/states.md") (path (named (kind library-package) (name "States")) (anonymous (kind import) (ordinal 2)))))) (kind membershipImport) (ordinal 0))
      (authored-target "StatePerformances::StateTransitionPerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/states.md") (path (named (kind library-package) (name "States")) (anonymous (kind import) (ordinal 3)))))) (kind membershipImport) (ordinal 0))
      (authored-target "Actions::Action")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/states.md") (path (named (kind library-package) (name "States")) (anonymous (kind import) (ordinal 4)))))) (kind membershipImport) (ordinal 0))
      (authored-target "Actions::TransitionAction")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/states.md") (path (named (kind library-package) (name "States")) (anonymous (kind import) (ordinal 5)))))) (kind membershipImport) (ordinal 0))
      (authored-target "Actions::transitionActions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/states.md") (path (named (kind library-package) (name "States")) (anonymous (kind import) (ordinal 6)))))) (kind membershipImport) (ordinal 0))
      (authored-target "Actions::AcceptAction")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/states.md") (path (named (kind library-package) (name "States")) (anonymous (kind import) (ordinal 7)))))) (kind membershipImport) (ordinal 0))
      (authored-target "Actions::actions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/states.md") (path (named (kind library-package) (name "States")) (anonymous (kind import) (ordinal 8)))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::notEmpty")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/states.md") (path (named (kind library-package) (name "States")) (anonymous (kind import) (ordinal 9)))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::size")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/states.md") (qualified-name "States::StateAction"))) (kind specialization) (ordinal 0))
      (authored-target "Action")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/states.md") (qualified-name "States::StateAction"))) (kind specialization) (ordinal 1))
      (authored-target "StatePerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/states.md") (qualified-name "States::StateAction::done"))) (kind featureTyping) (ordinal 0))
      (authored-target "StateAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/states.md") (qualified-name "States::StateAction")))))
    (reference (id (source (node (document "memory://snapshot/states.md") (qualified-name "States::StateAction::exclusiveStates"))) (kind featureTyping) (ordinal 0))
      (authored-target "StateAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/states.md") (qualified-name "States::StateAction")))))
    (reference (id (source (node (document "memory://snapshot/states.md") (qualified-name "States::StateAction::exclusiveStates"))) (kind subsetting) (ordinal 0))
      (authored-target "substates")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/states.md") (qualified-name "States::StateAction::self"))) (kind featureTyping) (ordinal 0))
      (authored-target "StateAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/states.md") (qualified-name "States::StateAction")))))
    (reference (id (source (node (document "memory://snapshot/states.md") (qualified-name "States::StateAction::start"))) (kind featureTyping) (ordinal 0))
      (authored-target "StateAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/states.md") (qualified-name "States::StateAction")))))
    (reference (id (source (node (document "memory://snapshot/states.md") (qualified-name "States::StateTransitionAction"))) (kind specialization) (ordinal 0))
      (authored-target "TransitionAction")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/states.md") (qualified-name "States::StateTransitionAction"))) (kind specialization) (ordinal 1))
      (authored-target "StateTransitionPerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/states.md") (path (named (kind library-package) (name "States")) (named (kind action-def) (name "StateTransitionAction")) (anonymous (kind parameter) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "receiver")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/states.md") (path (named (kind library-package) (name "States")) (named (kind action-def) (name "StateTransitionAction")) (anonymous (kind bind) (ordinal 0)))))) (kind bindSource) (ordinal 0))
      (authored-target "payload")
      (outcome (status resolved) (target (node (document "memory://snapshot/states.md") (qualified-name "States::StateTransitionAction::payload")))))
    (reference (id (source (node (document "memory://snapshot/states.md") (path (named (kind library-package) (name "States")) (named (kind action-def) (name "StateTransitionAction")) (anonymous (kind bind) (ordinal 1)))))) (kind bindSource) (ordinal 0))
      (authored-target "receiver")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/states.md") (path (named (kind library-package) (name "States")) (named (kind action-def) (name "StateTransitionAction")) (anonymous (kind bind) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "accepter::payload")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/states.md") (path (named (kind library-package) (name "States")) (named (kind action-def) (name "StateTransitionAction")) (anonymous (kind bind) (ordinal 1)))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "accepter::receiver")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/states.md") (qualified-name "States::StateTransitionAction::transitionLinkSource"))) (kind featureTyping) (ordinal 0))
      (authored-target "StateAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/states.md") (qualified-name "States::StateAction")))))
    (reference (id (source (node (document "memory://snapshot/states.md") (qualified-name "States::StateTransitionAction::transitionLinkSource"))) (kind redefinition) (ordinal 0))
      (authored-target "TransitionAction::transitionLinkSource")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/states.md") (qualified-name "States::StateTransitionAction::transitionLinkSource"))) (kind redefinition) (ordinal 1))
      (authored-target "StateTransitionPerformance::transitionLinkSource")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/states.md") (qualified-name "States::stateActions"))) (kind featureTyping) (ordinal 0))
      (authored-target "StateAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/states.md") (qualified-name "States::StateAction")))))
    (reference (id (source (node (document "memory://snapshot/states.md") (qualified-name "States::stateActions"))) (kind subsetting) (ordinal 0))
      (authored-target "actions")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/states.md") (qualified-name "States::StateAction::done"))) (target (node (document "memory://snapshot/states.md") (qualified-name "States::StateAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/states.md") (qualified-name "States::StateAction::done"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/states.md") (qualified-name "States::StateAction::exclusiveStates"))) (target (node (document "memory://snapshot/states.md") (qualified-name "States::StateAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/states.md") (qualified-name "States::StateAction::exclusiveStates"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/states.md") (qualified-name "States::StateAction::self"))) (target (node (document "memory://snapshot/states.md") (qualified-name "States::StateAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/states.md") (qualified-name "States::StateAction::self"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/states.md") (qualified-name "States::StateAction::start"))) (target (node (document "memory://snapshot/states.md") (qualified-name "States::StateAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/states.md") (qualified-name "States::StateAction::start"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind bindSource) (source (node (document "memory://snapshot/states.md") (path (named (kind library-package) (name "States")) (named (kind action-def) (name "StateTransitionAction")) (anonymous (kind bind) (ordinal 0)))))) (target (node (document "memory://snapshot/states.md") (qualified-name "States::StateTransitionAction::payload"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/states.md") (path (named (kind library-package) (name "States")) (named (kind action-def) (name "StateTransitionAction")) (anonymous (kind bind) (ordinal 0)))))) (kind bindSource) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/states.md") (qualified-name "States::StateTransitionAction::transitionLinkSource"))) (target (node (document "memory://snapshot/states.md") (qualified-name "States::StateAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/states.md") (qualified-name "States::StateTransitionAction::transitionLinkSource"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/states.md") (qualified-name "States::stateActions"))) (target (node (document "memory://snapshot/states.md") (qualified-name "States::StateAction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/states.md") (qualified-name "States::stateActions"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/states.md") (range (start 7 16) (end 7 39)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/states.md") (path (named (kind library-package) (name "States")) (anonymous (kind import) (ordinal 0)))))) (kind membershipImport) (ordinal 0) (authored-target "Occurrences::Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/states.md") (range (start 8 16) (end 8 51)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/states.md") (path (named (kind library-package) (name "States")) (anonymous (kind import) (ordinal 1)))))) (kind membershipImport) (ordinal 0) (authored-target "StatePerformances::StatePerformance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/states.md") (range (start 9 16) (end 9 61)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/states.md") (path (named (kind library-package) (name "States")) (anonymous (kind import) (ordinal 2)))))) (kind membershipImport) (ordinal 0) (authored-target "StatePerformances::StateTransitionPerformance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/states.md") (range (start 10 16) (end 10 31)) (probe (position 10 16))
    (reference (id (source (node (document "memory://snapshot/states.md") (path (named (kind library-package) (name "States")) (anonymous (kind import) (ordinal 3)))))) (kind membershipImport) (ordinal 0) (authored-target "Actions::Action")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/states.md") (range (start 11 16) (end 11 41)) (probe (position 11 16))
    (reference (id (source (node (document "memory://snapshot/states.md") (path (named (kind library-package) (name "States")) (anonymous (kind import) (ordinal 4)))))) (kind membershipImport) (ordinal 0) (authored-target "Actions::TransitionAction")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/states.md") (range (start 12 16) (end 12 42)) (probe (position 12 16))
    (reference (id (source (node (document "memory://snapshot/states.md") (path (named (kind library-package) (name "States")) (anonymous (kind import) (ordinal 5)))))) (kind membershipImport) (ordinal 0) (authored-target "Actions::transitionActions")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/states.md") (range (start 13 16) (end 13 37)) (probe (position 13 16))
    (reference (id (source (node (document "memory://snapshot/states.md") (path (named (kind library-package) (name "States")) (anonymous (kind import) (ordinal 6)))))) (kind membershipImport) (ordinal 0) (authored-target "Actions::AcceptAction")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/states.md") (range (start 14 16) (end 14 32)) (probe (position 14 16))
    (reference (id (source (node (document "memory://snapshot/states.md") (path (named (kind library-package) (name "States")) (anonymous (kind import) (ordinal 7)))))) (kind membershipImport) (ordinal 0) (authored-target "Actions::actions")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/states.md") (range (start 15 16) (end 15 43)) (probe (position 15 16))
    (reference (id (source (node (document "memory://snapshot/states.md") (path (named (kind library-package) (name "States")) (anonymous (kind import) (ordinal 8)))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::notEmpty")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/states.md") (range (start 16 16) (end 16 39)) (probe (position 16 16))
    (reference (id (source (node (document "memory://snapshot/states.md") (path (named (kind library-package) (name "States")) (anonymous (kind import) (ordinal 9)))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::size")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/states.md") (range (start 18 35) (end 18 41)) (probe (position 18 35))
    (reference (id (source (node (document "memory://snapshot/states.md") (qualified-name "States::StateAction"))) (kind specialization) (ordinal 0) (authored-target "Action")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/states.md") (range (start 18 43) (end 18 59)) (probe (position 18 43))
    (reference (id (source (node (document "memory://snapshot/states.md") (qualified-name "States::StateAction"))) (kind specialization) (ordinal 1) (authored-target "StatePerformance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/states.md") (range (start 33 18) (end 33 29)) (probe (position 33 18))
    (reference (id (source (node (document "memory://snapshot/states.md") (qualified-name "States::StateAction::done"))) (kind featureTyping) (ordinal 0) (authored-target "StateAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/states.md") (qualified-name "States::StateAction")))))
  )
  (query (document "memory://snapshot/states.md") (range (start 55 34) (end 55 45)) (probe (position 55 34))
    (reference (id (source (node (document "memory://snapshot/states.md") (qualified-name "States::StateAction::exclusiveStates"))) (kind featureTyping) (ordinal 0) (authored-target "StateAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/states.md") (qualified-name "States::StateAction")))))
  )
  (query (document "memory://snapshot/states.md") (range (start 55 55) (end 55 64)) (probe (position 55 55))
    (reference (id (source (node (document "memory://snapshot/states.md") (qualified-name "States::StateAction::exclusiveStates"))) (kind subsetting) (ordinal 0) (authored-target "substates")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/states.md") (range (start 31 18) (end 31 29)) (probe (position 31 18))
    (reference (id (source (node (document "memory://snapshot/states.md") (qualified-name "States::StateAction::self"))) (kind featureTyping) (ordinal 0) (authored-target "StateAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/states.md") (qualified-name "States::StateAction")))))
  )
  (query (document "memory://snapshot/states.md") (range (start 32 19) (end 32 30)) (probe (position 32 19))
    (reference (id (source (node (document "memory://snapshot/states.md") (qualified-name "States::StateAction::start"))) (kind featureTyping) (ordinal 0) (authored-target "StateAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/states.md") (qualified-name "States::StateAction")))))
  )
  (query (document "memory://snapshot/states.md") (range (start 79 37) (end 79 53)) (probe (position 79 37))
    (reference (id (source (node (document "memory://snapshot/states.md") (qualified-name "States::StateTransitionAction"))) (kind specialization) (ordinal 0) (authored-target "TransitionAction")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/states.md") (range (start 79 55) (end 79 81)) (probe (position 79 55))
    (reference (id (source (node (document "memory://snapshot/states.md") (qualified-name "States::StateTransitionAction"))) (kind specialization) (ordinal 1) (authored-target "StateTransitionPerformance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/states.md") (range (start 90 9) (end 90 17)) (probe (position 90 9))
    (reference (id (source (node (document "memory://snapshot/states.md") (path (named (kind library-package) (name "States")) (named (kind action-def) (name "StateTransitionAction")) (anonymous (kind parameter) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "receiver")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/states.md") (range (start 92 7) (end 92 14)) (probe (position 92 7))
    (reference (id (source (node (document "memory://snapshot/states.md") (path (named (kind library-package) (name "States")) (named (kind action-def) (name "StateTransitionAction")) (anonymous (kind bind) (ordinal 0)))))) (kind bindSource) (ordinal 0) (authored-target "payload")
      (outcome (status resolved) (target (node (document "memory://snapshot/states.md") (qualified-name "States::StateTransitionAction::payload")))))
  )
  (query (document "memory://snapshot/states.md") (range (start 93 7) (end 93 15)) (probe (position 93 7))
    (reference (id (source (node (document "memory://snapshot/states.md") (path (named (kind library-package) (name "States")) (named (kind action-def) (name "StateTransitionAction")) (anonymous (kind bind) (ordinal 1)))))) (kind bindSource) (ordinal 0) (authored-target "receiver")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/states.md") (range (start 92 17) (end 92 33)) (probe (position 92 17))
    (reference (id (source (node (document "memory://snapshot/states.md") (path (named (kind library-package) (name "States")) (named (kind action-def) (name "StateTransitionAction")) (anonymous (kind bind) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 0) (authored-target "accepter::payload")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/states.md") (range (start 93 18) (end 93 35)) (probe (position 93 18))
    (reference (id (source (node (document "memory://snapshot/states.md") (path (named (kind library-package) (name "States")) (named (kind action-def) (name "StateTransitionAction")) (anonymous (kind bind) (ordinal 1)))))) (kind memberAccessOperand) (ordinal 0) (authored-target "accepter::receiver")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/states.md") (range (start 86 30) (end 86 41)) (probe (position 86 30))
    (reference (id (source (node (document "memory://snapshot/states.md") (qualified-name "States::StateTransitionAction::transitionLinkSource"))) (kind featureTyping) (ordinal 0) (authored-target "StateAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/states.md") (qualified-name "States::StateAction")))))
  )
  (query (document "memory://snapshot/states.md") (range (start 87 3) (end 87 41)) (probe (position 87 3))
    (reference (id (source (node (document "memory://snapshot/states.md") (qualified-name "States::StateTransitionAction::transitionLinkSource"))) (kind redefinition) (ordinal 0) (authored-target "TransitionAction::transitionLinkSource")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/states.md") (range (start 87 43) (end 87 91)) (probe (position 87 43))
    (reference (id (source (node (document "memory://snapshot/states.md") (qualified-name "States::StateTransitionAction::transitionLinkSource"))) (kind redefinition) (ordinal 1) (authored-target "StateTransitionPerformance::transitionLinkSource")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/states.md") (range (start 96 30) (end 96 41)) (probe (position 96 30))
    (reference (id (source (node (document "memory://snapshot/states.md") (qualified-name "States::stateActions"))) (kind featureTyping) (ordinal 0) (authored-target "StateAction")
      (outcome (status resolved) (target (node (document "memory://snapshot/states.md") (qualified-name "States::StateAction")))))
  )
  (query (document "memory://snapshot/states.md") (range (start 96 61) (end 96 68)) (probe (position 96 61))
    (reference (id (source (node (document "memory://snapshot/states.md") (qualified-name "States::stateActions"))) (kind subsetting) (ordinal 0) (authored-target "actions")
      (outcome (status unresolved)))
  )
)
~~~
