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
    (element (id (node (document "d0") (qualified-name "States"))) (kind "package") (name "States") (declared-name "States"))
    (element (id (node (document "d0") (qualified-name "States::AcceptAction"))) (kind "import") (name "AcceptAction") (declared-name "AcceptAction") (parent (node (document "d0") (qualified-name "States"))) (authored (membership (kind Import) (visibility "private") (import (reference "Actions::AcceptAction") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "States::Action"))) (kind "import") (name "Action") (declared-name "Action") (parent (node (document "d0") (qualified-name "States"))) (authored (membership (kind Import) (visibility "private") (import (reference "Actions::Action") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "States::Occurrence"))) (kind "import") (name "Occurrence") (declared-name "Occurrence") (parent (node (document "d0") (qualified-name "States"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::Occurrence") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "States::StateAction"))) (kind "state def") (name "StateAction") (declared-name "StateAction") (parent (node (document "d0") (qualified-name "States"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Action")) (specializes (reference "StatePerformance")))))
    (element (id (node (document "d0") (qualified-name "States::StateAction::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "States::StateAction"))))
    (element (id (node (document "d0") (qualified-name "States::StateAction::done"))) (kind "ref") (name "done") (declared-name "done") (parent (node (document "d0") (qualified-name "States::StateAction"))) (authored (membership (kind Feature)) (relationships (typing (reference "StateAction")))))
    (element (id (node (document "d0") (qualified-name "States::StateAction::exclusiveStates"))) (kind "state") (name "exclusiveStates") (declared-name "exclusiveStates") (parent (node (document "d0") (qualified-name "States::StateAction"))) (authored (membership (kind Feature)) (relationships (typing (reference "StateAction")))))
    (element (id (node (document "d0") (qualified-name "States::StateAction::exclusiveStates::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "States::StateAction::exclusiveStates"))))
    (element (id (node (document "d0") (qualified-name "States::StateAction::self"))) (kind "ref") (name "self") (declared-name "self") (parent (node (document "d0") (qualified-name "States::StateAction"))) (authored (membership (kind Feature)) (relationships (typing (reference "StateAction")))))
    (element (id (node (document "d0") (qualified-name "States::StateAction::start"))) (kind "ref") (name "start") (declared-name "start") (parent (node (document "d0") (qualified-name "States::StateAction"))) (authored (membership (kind Feature)) (relationships (typing (reference "StateAction")))))
    (element (id (node (document "d0") (qualified-name "States::StatePerformance"))) (kind "import") (name "StatePerformance") (declared-name "StatePerformance") (parent (node (document "d0") (qualified-name "States"))) (authored (membership (kind Import) (visibility "private") (import (reference "StatePerformances::StatePerformance") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "States::StateTransitionAction"))) (kind "action def") (name "StateTransitionAction") (declared-name "StateTransitionAction") (parent (node (document "d0") (qualified-name "States"))) (authored (membership (kind Owning)) (relationships (specializes (reference "TransitionAction")) (specializes (reference "StateTransitionPerformance")) (specializes (reference "TransitionAction")) (specializes (reference "StateTransitionPerformance")) (specializes (reference "TransitionAction")) (specializes (reference "StateTransitionPerformance")))))
    (element (id (node (document "d0") (qualified-name "States::StateTransitionAction::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "States::StateTransitionAction"))))
    (element (id (node (document "d0") (qualified-name "States::StateTransitionAction::payload"))) (kind "in out parameter") (name "payload") (declared-name "payload") (parent (node (document "d0") (qualified-name "States::StateTransitionAction"))) (authored (relationships (typing (reference "payload[0..*]")))))
    (element (id (node (document "d0") (qualified-name "States::StateTransitionAction::receiver"))) (kind "in out parameter") (name "receiver") (declared-name "receiver") (parent (node (document "d0") (qualified-name "States::StateTransitionAction"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "States::StateTransitionAction::transitionLinkSource"))) (kind "in out parameter") (name "transitionLinkSource") (declared-name "transitionLinkSource") (parent (node (document "d0") (qualified-name "States::StateTransitionAction"))) (authored (relationships (typing (reference "transitionLinkSource[1]: StateAction :>> \n\t\t\tTransitionAction::transitionLinkSource, StateTransitionPerformance::transitionLinkSource")))))
    (element (id (node (document "d0") (qualified-name "States::StateTransitionPerformance"))) (kind "import") (name "StateTransitionPerformance") (declared-name "StateTransitionPerformance") (parent (node (document "d0") (qualified-name "States"))) (authored (membership (kind Import) (visibility "private") (import (reference "StatePerformances::StateTransitionPerformance") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "States::TransitionAction"))) (kind "import") (name "TransitionAction") (declared-name "TransitionAction") (parent (node (document "d0") (qualified-name "States"))) (authored (membership (kind Import) (visibility "private") (import (reference "Actions::TransitionAction") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "States::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "States"))))
    (element (id (node (document "d0") (qualified-name "States::actions"))) (kind "import") (name "actions") (declared-name "actions") (parent (node (document "d0") (qualified-name "States"))) (authored (membership (kind Import) (visibility "private") (import (reference "Actions::actions") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "States::notEmpty"))) (kind "import") (name "notEmpty") (declared-name "notEmpty") (parent (node (document "d0") (qualified-name "States"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::notEmpty") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "States::size"))) (kind "import") (name "size") (declared-name "size") (parent (node (document "d0") (qualified-name "States"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::size") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "States::stateActions"))) (kind "state") (name "stateActions") (declared-name "stateActions") (parent (node (document "d0") (qualified-name "States"))) (authored (membership (kind Feature)) (relationships (typing (reference "StateAction")) (subsetting (reference "actions")))))
    (element (id (node (document "d0") (qualified-name "States::stateActions::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "States::stateActions"))))
    (element (id (node (document "d0") (qualified-name "States::transitionActions"))) (kind "import") (name "transitionActions") (declared-name "transitionActions") (parent (node (document "d0") (qualified-name "States"))) (authored (membership (kind Import) (visibility "private") (import (reference "Actions::transitionActions") (origin Import) (shape Membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "States::AcceptAction"))) (kind membershipImport) (ordinal 0)) (authored-target "Actions::AcceptAction") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "States::Action"))) (kind membershipImport) (ordinal 0)) (authored-target "Actions::Action") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "States::Occurrence"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::Occurrence") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "States::StateAction"))) (kind specialization) (ordinal 0)) (authored-target "Action") (outcome (status resolved) (target (node (document "d0") (qualified-name "States::Action")))))
    (reference (id (source (node (document "d0") (qualified-name "States::StateAction"))) (kind specialization) (ordinal 1)) (authored-target "StatePerformance") (outcome (status resolved) (target (node (document "d0") (qualified-name "States::StatePerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "States::StateAction::done"))) (kind featureTyping) (ordinal 0)) (authored-target "StateAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "States::StateAction")))))
    (reference (id (source (node (document "d0") (qualified-name "States::StateAction::exclusiveStates"))) (kind featureTyping) (ordinal 0)) (authored-target "StateAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "States::StateAction")))))
    (reference (id (source (node (document "d0") (qualified-name "States::StateAction::self"))) (kind featureTyping) (ordinal 0)) (authored-target "StateAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "States::StateAction")))))
    (reference (id (source (node (document "d0") (qualified-name "States::StateAction::start"))) (kind featureTyping) (ordinal 0)) (authored-target "StateAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "States::StateAction")))))
    (reference (id (source (node (document "d0") (qualified-name "States::StatePerformance"))) (kind membershipImport) (ordinal 0)) (authored-target "StatePerformances::StatePerformance") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "States::StateTransitionAction"))) (kind specialization) (ordinal 0)) (authored-target "TransitionAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "States::TransitionAction")))))
    (reference (id (source (node (document "d0") (qualified-name "States::StateTransitionAction"))) (kind specialization) (ordinal 1)) (authored-target "StateTransitionPerformance") (outcome (status resolved) (target (node (document "d0") (qualified-name "States::StateTransitionPerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "States::StateTransitionAction"))) (kind specialization) (ordinal 2)) (authored-target "TransitionAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "States::TransitionAction")))))
    (reference (id (source (node (document "d0") (qualified-name "States::StateTransitionAction"))) (kind specialization) (ordinal 3)) (authored-target "StateTransitionPerformance") (outcome (status resolved) (target (node (document "d0") (qualified-name "States::StateTransitionPerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "States::StateTransitionAction"))) (kind specialization) (ordinal 4)) (authored-target "TransitionAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "States::TransitionAction")))))
    (reference (id (source (node (document "d0") (qualified-name "States::StateTransitionAction"))) (kind specialization) (ordinal 5)) (authored-target "StateTransitionPerformance") (outcome (status resolved) (target (node (document "d0") (qualified-name "States::StateTransitionPerformance")))))
    (reference (id (source (node (document "d0") (qualified-name "States::StateTransitionAction"))) (kind bindSource) (ordinal 0)) (authored-target "payload") (outcome (status resolved) (target (node (document "d0") (qualified-name "States::StateTransitionAction::payload")))))
    (reference (id (source (node (document "d0") (qualified-name "States::StateTransitionAction"))) (kind bindSource) (ordinal 1)) (authored-target "receiver") (outcome (status resolved) (target (node (document "d0") (qualified-name "States::StateTransitionAction::receiver")))))
    (reference (id (source (node (document "d0") (qualified-name "States::StateTransitionAction"))) (kind bindTarget) (ordinal 0)) (authored-target "accepter::payload") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "States::StateTransitionAction"))) (kind bindTarget) (ordinal 1)) (authored-target "accepter::receiver") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "States::StateTransitionAction::payload"))) (kind featureTyping) (ordinal 0)) (authored-target "payload[0..*]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "States::StateTransitionAction::receiver"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status resolved) (target (node (document "d0") (qualified-name "States::StateTransitionAction::_documentation")))))
    (reference (id (source (node (document "d0") (qualified-name "States::StateTransitionAction::transitionLinkSource"))) (kind featureTyping) (ordinal 0)) (authored-target "transitionLinkSource[1]: StateAction :>> \n\t\t\tTransitionAction::transitionLinkSource, StateTransitionPerformance::transitionLinkSource") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "States::StateTransitionPerformance"))) (kind membershipImport) (ordinal 0)) (authored-target "StatePerformances::StateTransitionPerformance") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "States::TransitionAction"))) (kind membershipImport) (ordinal 0)) (authored-target "Actions::TransitionAction") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "States::actions"))) (kind membershipImport) (ordinal 0)) (authored-target "Actions::actions") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "States::notEmpty"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::notEmpty") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "States::size"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::size") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "States::stateActions"))) (kind featureTyping) (ordinal 0)) (authored-target "StateAction") (outcome (status resolved) (target (node (document "d0") (qualified-name "States::StateAction")))))
    (reference (id (source (node (document "d0") (qualified-name "States::stateActions"))) (kind subsetting) (ordinal 0)) (authored-target "actions") (outcome (status resolved) (target (node (document "d0") (qualified-name "States::actions")))))
    (reference (id (source (node (document "d0") (qualified-name "States::transitionActions"))) (kind membershipImport) (ordinal 0)) (authored-target "Actions::transitionActions") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
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
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 18 35) (end 18 41)) (probe (position 18 35))
      (reference
        (source (document "d0") (qualified-name "States::StateAction"))
        (kind specialization) (ordinal 0) (authored-target "Action")
        (range (start 18 35) (end 18 41))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "States::Action") (range (start 10 1) (end 10 32)))
        )
      )
    )
    (query (range (start 92 7) (end 92 14)) (probe (position 92 7))
      (reference
        (source (document "d0") (qualified-name "States::StateTransitionAction"))
        (kind bindSource) (ordinal 0) (authored-target "payload")
        (range (start 92 7) (end 92 14))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "States::StateTransitionAction::payload") (range (start 89 2) (end 89 22)))
        )
      )
    )
    (query (range (start 96 61) (end 96 68)) (probe (position 96 61))
      (reference
        (source (document "d0") (qualified-name "States::stateActions"))
        (kind subsetting) (ordinal 0) (authored-target "actions")
        (range (start 96 61) (end 96 68))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "States::actions") (range (start 14 1) (end 14 33)))
        )
      )
    )
    (query (range (start 93 7) (end 93 15)) (probe (position 93 7))
      (reference
        (source (document "d0") (qualified-name "States::StateTransitionAction"))
        (kind bindSource) (ordinal 1) (authored-target "receiver")
        (range (start 93 7) (end 93 15))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "States::StateTransitionAction::receiver") (range (start 90 2) (end 90 18)))
        )
      )
    )
    (query (range (start 31 18) (end 31 29)) (probe (position 31 18))
      (reference
        (source (document "d0") (qualified-name "States::StateAction::self"))
        (kind featureTyping) (ordinal 0) (authored-target "StateAction")
        (range (start 31 18) (end 31 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "States::StateAction") (range (start 18 1) (end 18 2048)))
        )
      )
    )
    (query (range (start 32 19) (end 32 30)) (probe (position 32 19))
      (reference
        (source (document "d0") (qualified-name "States::StateAction::start"))
        (kind featureTyping) (ordinal 0) (authored-target "StateAction")
        (range (start 32 19) (end 32 30))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "States::StateAction") (range (start 18 1) (end 18 2048)))
        )
      )
    )
    (query (range (start 33 18) (end 33 29)) (probe (position 33 18))
      (reference
        (source (document "d0") (qualified-name "States::StateAction::done"))
        (kind featureTyping) (ordinal 0) (authored-target "StateAction")
        (range (start 33 18) (end 33 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "States::StateAction") (range (start 18 1) (end 18 2048)))
        )
      )
    )
    (query (range (start 10 16) (end 10 31)) (probe (position 10 16))
      (reference
        (source (document "d0") (qualified-name "States::Action"))
        (kind membershipImport) (ordinal 0) (authored-target "Actions::Action")
        (range (start 10 16) (end 10 31))
        (outcome (status unresolved))
      )
    )
    (query (range (start 14 16) (end 14 32)) (probe (position 14 16))
      (reference
        (source (document "d0") (qualified-name "States::actions"))
        (kind membershipImport) (ordinal 0) (authored-target "Actions::actions")
        (range (start 14 16) (end 14 32))
        (outcome (status unresolved))
      )
    )
    (query (range (start 18 43) (end 18 59)) (probe (position 18 43))
      (reference
        (source (document "d0") (qualified-name "States::StateAction"))
        (kind specialization) (ordinal 1) (authored-target "StatePerformance")
        (range (start 18 43) (end 18 59))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "States::StatePerformance") (range (start 8 1) (end 8 52)))
        )
      )
    )
    (query (range (start 79 37) (end 79 53)) (probe (position 79 37))
      (reference
        (source (document "d0") (qualified-name "States::StateTransitionAction"))
        (kind specialization) (ordinal 4) (authored-target "TransitionAction")
        (range (start 79 37) (end 79 53))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "States::TransitionAction") (range (start 11 1) (end 11 42)))
        )
      )
    )
    (query (range (start 92 17) (end 92 33)) (probe (position 92 17))
      (reference
        (source (document "d0") (qualified-name "States::StateTransitionAction"))
        (kind bindTarget) (ordinal 0) (authored-target "accepter::payload")
        (range (start 92 17) (end 92 33))
        (outcome (status unresolved))
      )
    )
    (query (range (start 93 18) (end 93 35)) (probe (position 93 18))
      (reference
        (source (document "d0") (qualified-name "States::StateTransitionAction"))
        (kind bindTarget) (ordinal 1) (authored-target "accepter::receiver")
        (range (start 93 18) (end 93 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 13 16) (end 13 37)) (probe (position 13 16))
      (reference
        (source (document "d0") (qualified-name "States::AcceptAction"))
        (kind membershipImport) (ordinal 0) (authored-target "Actions::AcceptAction")
        (range (start 13 16) (end 13 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 16) (end 7 39)) (probe (position 7 16))
      (reference
        (source (document "d0") (qualified-name "States::Occurrence"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::Occurrence")
        (range (start 7 16) (end 7 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 16 16) (end 16 39)) (probe (position 16 16))
      (reference
        (source (document "d0") (qualified-name "States::size"))
        (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::size")
        (range (start 16 16) (end 16 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 11 16) (end 11 41)) (probe (position 11 16))
      (reference
        (source (document "d0") (qualified-name "States::TransitionAction"))
        (kind membershipImport) (ordinal 0) (authored-target "Actions::TransitionAction")
        (range (start 11 16) (end 11 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 12 16) (end 12 42)) (probe (position 12 16))
      (reference
        (source (document "d0") (qualified-name "States::transitionActions"))
        (kind membershipImport) (ordinal 0) (authored-target "Actions::transitionActions")
        (range (start 12 16) (end 12 42))
        (outcome (status unresolved))
      )
    )
    (query (range (start 79 55) (end 79 81)) (probe (position 79 55))
      (reference
        (source (document "d0") (qualified-name "States::StateTransitionAction"))
        (kind specialization) (ordinal 5) (authored-target "StateTransitionPerformance")
        (range (start 79 55) (end 79 81))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "States::StateTransitionPerformance") (range (start 9 1) (end 9 62)))
        )
      )
    )
    (query (range (start 15 16) (end 15 43)) (probe (position 15 16))
      (reference
        (source (document "d0") (qualified-name "States::notEmpty"))
        (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::notEmpty")
        (range (start 15 16) (end 15 43))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 16) (end 8 51)) (probe (position 8 16))
      (reference
        (source (document "d0") (qualified-name "States::StatePerformance"))
        (kind membershipImport) (ordinal 0) (authored-target "StatePerformances::StatePerformance")
        (range (start 8 16) (end 8 51))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 16) (end 9 61)) (probe (position 9 16))
      (reference
        (source (document "d0") (qualified-name "States::StateTransitionPerformance"))
        (kind membershipImport) (ordinal 0) (authored-target "StatePerformances::StateTransitionPerformance")
        (range (start 9 16) (end 9 61))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
