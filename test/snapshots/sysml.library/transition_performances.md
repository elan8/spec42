# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Semantic Library/TransitionPerformances
type=file
~~~
# SOURCE
~~~kerml
standard library package TransitionPerformances {
	doc
	/*
	 * This package contains a library model of the semantics of conditional transitions between occurrences, 
	 * including the performance of specified Behaviors when the transition occurs.
	 */

	private import ScalarValues::Boolean;
	private import ScalarValues::Natural;
	private import SequenceFunctions::isEmpty;
	private import Occurrences::Occurrence;
	private import Occurrences::HappensBefore;
	private import Performances::Performance;
	private import Performances::Evaluation;
	private import Transfers::MessageTransfer;
	private import Transfers::AcceptPerformance;
	private import Transfers::acceptPerformances;
	private import ControlFunctions::allTrue;
	private import SequenceFunctions::size;
	
	abstract behavior TransitionPerformance {
		in feature transitionLinkSource: Performance[1];
		
		feature trigger: MessageTransfer[*];
		bool guard[*] subsets enclosedPerformances;
		step effect[*] subsets enclosedPerformances;

		feature triggerTarget : Occurrence [1] default this;
		feature transitionLink: HappensBefore[0..1];
		
		private binding [0..1] transitionLink.earlierOccurrence = [1] transitionLinkSource;
		private succession [1] transitionLinkSource then [*] effect;
		private succession [*] effect then [1] transitionLink.laterOccurrence;
		
		private connector [0..1] transitionLink to [1..*] trigger;
		private connector all guardConstraint: TPCGuardConstraint[*] 
			from [0..1] transitionLink to [*] guard;
			
		private succession all [*] trigger then [*] guard;
		private succession all [*] guard then [*] effect;

		feature accNum: Natural [1] = if isEmpty(trigger) ? 0 else 1;
		step 'accept': AcceptPerformance[accNum] subsets timeEnclosedOccurrences, acceptPerformances {
			feature redefines acceptedTransfer = trigger;
		}
        binding 'accept'.receiver = triggerTarget;

		private succession [*] guard then [accNum] 'accept';
	}
	
	behavior NonStateTransitionPerformance specializes TransitionPerformance {
		feature isTriggerAfter: Boolean default true;
		private succession [1] transitionLinkSource then [1] Performance::self;
		private feature taNum: Natural [1] = if isTriggerAfter ? size(trigger) else 0;
		private succession triggerAfter [taNum] first [0..1] transitionLinkSource then [*] trigger.endShot;
				
		private succession all [*] guard then [0..1] transitionLink.laterOccurrence;
	}
	
	assoc struct TPCGuardConstraint {
		end guardedLink [0..1] feature constrainedHBLink: HappensBefore;
		end 'bool' constrainedGuard;
		
		private inv { allTrue(constrainedGuard()) }
	}	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "transition_performances.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 16) (end 9 42))
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
        (range (start 12 16) (end 12 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 13 16) (end 13 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 14 16) (end 14 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 15 16) (end 15 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 16 16) (end 16 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 17 16) (end 17 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 18 16) (end 18 39))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package TransitionPerformances {
	doc
	/*
	 * This package contains a library model of the semantics of conditional transitions between occurrences, 
	 * including the performance of specified Behaviors when the transition occurs.
	 */

	private import ScalarValues::Boolean;
	private import ScalarValues::Natural;
	private import SequenceFunctions::isEmpty;
	private import Occurrences::Occurrence;
	private import Occurrences::HappensBefore;
	private import Performances::Performance;
	private import Performances::Evaluation;
	private import Transfers::MessageTransfer;
	private import Transfers::AcceptPerformance;
	private import Transfers::acceptPerformances;
	private import ControlFunctions::allTrue;
	private import SequenceFunctions::size;
	
	abstract behavior TransitionPerformance {
		in feature transitionLinkSource: Performance[1];
		
		feature trigger: MessageTransfer[*];
		bool guard[*] subsets enclosedPerformances;
		step effect[*] subsets enclosedPerformances;

		feature triggerTarget : Occurrence [1] default this;
		feature transitionLink: HappensBefore[0..1];
		
		private binding [0..1] transitionLink.earlierOccurrence = [1] transitionLinkSource;
		private succession [1] transitionLinkSource then [*] effect;
		private succession [*] effect then [1] transitionLink.laterOccurrence;
		
		private connector [0..1] transitionLink to [1..*] trigger;
		private connector all guardConstraint: TPCGuardConstraint[*] 
			from [0..1] transitionLink to [*] guard;
			
		private succession all [*] trigger then [*] guard;
		private succession all [*] guard then [*] effect;

		feature accNum: Natural [1] = if isEmpty(trigger) ? 0 else 1;
		step 'accept': AcceptPerformance[accNum] subsets timeEnclosedOccurrences, acceptPerformances {
			feature redefines acceptedTransfer = trigger;
		}
        binding 'accept'.receiver = triggerTarget;

		private succession [*] guard then [accNum] 'accept';
	}
	
	behavior NonStateTransitionPerformance specializes TransitionPerformance {
		feature isTriggerAfter: Boolean default true;
		private succession [1] transitionLinkSource then [1] Performance::self;
		private feature taNum: Natural [1] = if isTriggerAfter ? size(trigger) else 0;
		private succession triggerAfter [taNum] first [0..1] transitionLinkSource then [*] trigger.endShot;
				
		private succession all [*] guard then [0..1] transitionLink.laterOccurrence;
	}
	
	assoc struct TPCGuardConstraint {
		end guardedLink [0..1] feature constrainedHBLink: HappensBefore;
		end 'bool' constrainedGuard;
		
		private inv { allTrue(constrainedGuard()) }
	}	
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "1979ea721f982879efcd9ea04a6cc4665e264363b7838cccce67a41c0523f7ad") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "TransitionPerformances"))) (kind "package") (name "TransitionPerformances") (declared-name "TransitionPerformances") (range (start (line 0) (character 0)) (end (line 0) (character 2591))))
    (element (id (node (document "d0") (qualified-name "TransitionPerformances::AcceptPerformance"))) (kind "import") (name "AcceptPerformance") (declared-name "AcceptPerformance") (range (start (line 15) (character 1)) (end (line 15) (character 45))) (parent (node (document "d0") (qualified-name "TransitionPerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::AcceptPerformance") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 15) (character 16)) (end (line 15) (character 44))))))
    (element (id (node (document "d0") (qualified-name "TransitionPerformances::Boolean"))) (kind "import") (name "Boolean") (declared-name "Boolean") (range (start (line 7) (character 1)) (end (line 7) (character 38))) (parent (node (document "d0") (qualified-name "TransitionPerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Boolean") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 16)) (end (line 7) (character 37))))))
    (element (id (node (document "d0") (qualified-name "TransitionPerformances::Evaluation"))) (kind "import") (name "Evaluation") (declared-name "Evaluation") (range (start (line 13) (character 1)) (end (line 13) (character 41))) (parent (node (document "d0") (qualified-name "TransitionPerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Performances::Evaluation") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 13) (character 16)) (end (line 13) (character 40))))))
    (element (id (node (document "d0") (qualified-name "TransitionPerformances::HappensBefore"))) (kind "import") (name "HappensBefore") (declared-name "HappensBefore") (range (start (line 11) (character 1)) (end (line 11) (character 43))) (parent (node (document "d0") (qualified-name "TransitionPerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::HappensBefore") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 11) (character 16)) (end (line 11) (character 42))))))
    (element (id (node (document "d0") (qualified-name "TransitionPerformances::MessageTransfer"))) (kind "import") (name "MessageTransfer") (declared-name "MessageTransfer") (range (start (line 14) (character 1)) (end (line 14) (character 43))) (parent (node (document "d0") (qualified-name "TransitionPerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::MessageTransfer") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 14) (character 16)) (end (line 14) (character 42))))))
    (element (id (node (document "d0") (qualified-name "TransitionPerformances::Natural"))) (kind "import") (name "Natural") (declared-name "Natural") (range (start (line 8) (character 1)) (end (line 8) (character 38))) (parent (node (document "d0") (qualified-name "TransitionPerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Natural") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 8) (character 16)) (end (line 8) (character 37))))))
    (element (id (node (document "d0") (qualified-name "TransitionPerformances::NonStateTransitionPerformance"))) (kind "kermlDecl") (name "NonStateTransitionPerformance") (declared-name "NonStateTransitionPerformance") (range (start (line 50) (character 1)) (end (line 50) (character 467))) (parent (node (document "d0") (qualified-name "TransitionPerformances"))))
    (element (id (node (document "d0") (qualified-name "TransitionPerformances::Occurrence"))) (kind "import") (name "Occurrence") (declared-name "Occurrence") (range (start (line 10) (character 1)) (end (line 10) (character 40))) (parent (node (document "d0") (qualified-name "TransitionPerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::Occurrence") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 10) (character 16)) (end (line 10) (character 39))))))
    (element (id (node (document "d0") (qualified-name "TransitionPerformances::Performance"))) (kind "import") (name "Performance") (declared-name "Performance") (range (start (line 12) (character 1)) (end (line 12) (character 42))) (parent (node (document "d0") (qualified-name "TransitionPerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Performances::Performance") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 12) (character 16)) (end (line 12) (character 41))))))
    (element (id (node (document "d0") (qualified-name "TransitionPerformances::TransitionPerformance"))) (kind "kermlDecl") (name "TransitionPerformance") (declared-name "TransitionPerformance") (range (start (line 20) (character 1)) (end (line 20) (character 1162))) (parent (node (document "d0") (qualified-name "TransitionPerformances"))))
    (element (id (node (document "d0") (qualified-name "TransitionPerformances::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 2591))) (parent (node (document "d0") (qualified-name "TransitionPerformances"))))
    (element (id (node (document "d0") (qualified-name "TransitionPerformances::acceptPerformances"))) (kind "import") (name "acceptPerformances") (declared-name "acceptPerformances") (range (start (line 16) (character 1)) (end (line 16) (character 46))) (parent (node (document "d0") (qualified-name "TransitionPerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::acceptPerformances") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 16) (character 16)) (end (line 16) (character 45))))))
    (element (id (node (document "d0") (qualified-name "TransitionPerformances::allTrue"))) (kind "import") (name "allTrue") (declared-name "allTrue") (range (start (line 17) (character 1)) (end (line 17) (character 42))) (parent (node (document "d0") (qualified-name "TransitionPerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::allTrue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 17) (character 16)) (end (line 17) (character 41))))))
    (element (id (node (document "d0") (qualified-name "TransitionPerformances::isEmpty"))) (kind "import") (name "isEmpty") (declared-name "isEmpty") (range (start (line 9) (character 1)) (end (line 9) (character 43))) (parent (node (document "d0") (qualified-name "TransitionPerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::isEmpty") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 9) (character 16)) (end (line 9) (character 42))))))
    (element (id (node (document "d0") (qualified-name "TransitionPerformances::size"))) (kind "import") (name "size") (declared-name "size") (range (start (line 18) (character 1)) (end (line 18) (character 40))) (parent (node (document "d0") (qualified-name "TransitionPerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::size") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 18) (character 16)) (end (line 18) (character 39))))))
    (element (id (node (document "d0") (qualified-name "TransitionPerformances::struct"))) (kind "kermlDecl") (name "struct") (declared-name "struct") (range (start (line 59) (character 1)) (end (line 59) (character 184))) (parent (node (document "d0") (qualified-name "TransitionPerformances"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "TransitionPerformances::AcceptPerformance"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::AcceptPerformance") (range (start (line 15) (character 16)) (end (line 15) (character 44))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TransitionPerformances::Boolean"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Boolean") (range (start (line 7) (character 16)) (end (line 7) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TransitionPerformances::Evaluation"))) (kind membershipImport) (ordinal 0)) (authored-target "Performances::Evaluation") (range (start (line 13) (character 16)) (end (line 13) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TransitionPerformances::HappensBefore"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::HappensBefore") (range (start (line 11) (character 16)) (end (line 11) (character 42))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TransitionPerformances::MessageTransfer"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::MessageTransfer") (range (start (line 14) (character 16)) (end (line 14) (character 42))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TransitionPerformances::Natural"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Natural") (range (start (line 8) (character 16)) (end (line 8) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TransitionPerformances::Occurrence"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::Occurrence") (range (start (line 10) (character 16)) (end (line 10) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TransitionPerformances::Performance"))) (kind membershipImport) (ordinal 0)) (authored-target "Performances::Performance") (range (start (line 12) (character 16)) (end (line 12) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TransitionPerformances::acceptPerformances"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::acceptPerformances") (range (start (line 16) (character 16)) (end (line 16) (character 45))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TransitionPerformances::allTrue"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlFunctions::allTrue") (range (start (line 17) (character 16)) (end (line 17) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TransitionPerformances::isEmpty"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::isEmpty") (range (start (line 9) (character 16)) (end (line 9) (character 42))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TransitionPerformances::size"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::size") (range (start (line 18) (character 16)) (end (line 18) (character 39))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
