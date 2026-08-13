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
  (document "memory://snapshot/transition_performances.md"
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
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 21 2) (end 21 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 23 19) (end 23 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 24 24) (end 24 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 25 25) (end 25 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 27 26) (end 27 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 27 49) (end 27 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 28 26) (end 28 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 30 2) (end 30 85))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 31 2) (end 31 62))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 32 2) (end 32 72))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 34 2) (end 34 60))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 35 2) (end 36 43))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 38 2) (end 38 52))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 39 2) (end 39 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 41 18) (end 41 25))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 41 32) (end 41 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 42 17) (end 42 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 42 51) (end 42 74))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 42 76) (end 42 94))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 43 21) (end 43 37))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 45 8) (end 45 50))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 47 2) (end 47 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 51 26) (end 51 33))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 52 2) (end 52 73))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 53 25) (end 53 32))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 53 39) (end 53 79))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 54 2) (end 54 101))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 56 2) (end 56 78))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 60 2) (end 60 66))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 61 2) (end 63 2))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 63 2) (end 63 9))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 63 10) (end 63 45))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:46de6a9b5e529c14cf77db03c1c64e8b63dcb8c0a57f6b0e91781ca945c77f06") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/transition_performances.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Boolean") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/transition_performances.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Natural") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/transition_performances.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::isEmpty") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/transition_performances.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Occurrences::Occurrence") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/transition_performances.md") (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Occurrences::HappensBefore") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/transition_performances.md") (anonymous (kind import) (ordinal 5))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Performances::Performance") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/transition_performances.md") (anonymous (kind import) (ordinal 6))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Performances::Evaluation") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/transition_performances.md") (anonymous (kind import) (ordinal 7))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Transfers::MessageTransfer") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/transition_performances.md") (anonymous (kind import) (ordinal 8))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Transfers::AcceptPerformance") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/transition_performances.md") (anonymous (kind import) (ordinal 9))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Transfers::acceptPerformances") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/transition_performances.md") (anonymous (kind import) (ordinal 10))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ControlFunctions::allTrue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/transition_performances.md") (anonymous (kind import) (ordinal 11))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::size") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::NonStateTransitionPerformance"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "TransitionPerformance"))))
    (declaration (id (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::NonStateTransitionPerformance::isTriggerAfter"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean"))))
    (declaration (id (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::NonStateTransitionPerformance::taNum"))) (kind kerml-feature) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "Natural"))))
    (declaration (id (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::TPCGuardConstraint"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "private"))))
    (declaration (id (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::accNum"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Natural"))))
    (declaration (id (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::accept"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AcceptPerformance")) (subsetting (reference "timeEnclosedOccurrences")) (subsetting (reference "acceptPerformances"))))
    (declaration (id (node (document "memory://snapshot/transition_performances.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "acceptedTransfer")) (expressionOperand (reference "trigger"))))
    (declaration (id (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::effect"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "enclosedPerformances"))))
    (declaration (id (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::guard"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "enclosedPerformances"))))
    (declaration (id (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::transitionLink"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "HappensBefore"))))
    (declaration (id (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::trigger"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MessageTransfer"))))
    (declaration (id (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::triggerTarget"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (expressionOperand (reference "this"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Natural")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::isEmpty")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "Occurrences::Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0))
      (authored-target "Occurrences::HappensBefore")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0))
      (authored-target "Performances::Performance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0))
      (authored-target "Performances::Evaluation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0))
      (authored-target "Transfers::MessageTransfer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (anonymous (kind import) (ordinal 8))))) (kind membershipImport) (ordinal 0))
      (authored-target "Transfers::AcceptPerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (anonymous (kind import) (ordinal 9))))) (kind membershipImport) (ordinal 0))
      (authored-target "Transfers::acceptPerformances")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (anonymous (kind import) (ordinal 10))))) (kind membershipImport) (ordinal 0))
      (authored-target "ControlFunctions::allTrue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (anonymous (kind import) (ordinal 11))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::size")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::NonStateTransitionPerformance"))) (kind specialization) (ordinal 0))
      (authored-target "TransitionPerformance")
      (outcome (status resolved) (target (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance")))))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::NonStateTransitionPerformance::isTriggerAfter"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::NonStateTransitionPerformance::taNum"))) (kind featureTyping) (ordinal 0))
      (authored-target "Natural")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::TPCGuardConstraint"))) (kind expressionOperand) (ordinal 0))
      (authored-target "private")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::accNum"))) (kind featureTyping) (ordinal 0))
      (authored-target "Natural")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::accept"))) (kind featureTyping) (ordinal 0))
      (authored-target "AcceptPerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::accept"))) (kind subsetting) (ordinal 0))
      (authored-target "timeEnclosedOccurrences")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::accept"))) (kind subsetting) (ordinal 1))
      (authored-target "acceptPerformances")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "acceptedTransfer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "trigger")
      (outcome (status resolved) (target (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::trigger")))))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::effect"))) (kind subsetting) (ordinal 0))
      (authored-target "enclosedPerformances")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::guard"))) (kind subsetting) (ordinal 0))
      (authored-target "enclosedPerformances")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::transitionLink"))) (kind featureTyping) (ordinal 0))
      (authored-target "HappensBefore")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::trigger"))) (kind featureTyping) (ordinal 0))
      (authored-target "MessageTransfer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::triggerTarget"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::triggerTarget"))) (kind expressionOperand) (ordinal 0))
      (authored-target "this")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::NonStateTransitionPerformance"))) (target (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::NonStateTransitionPerformance"))) (kind specialization) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/transition_performances.md") (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::trigger"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/transition_performances.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::NonStateTransitionPerformance::isTriggerAfter"))) (value (kind boolean) (boolean true)))
    (evaluated (declaration (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::TPCGuardConstraint"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/transition_performances.md") (anonymous (kind kerml-feature) (ordinal 0))))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::triggerTarget"))) (value (kind unresolved-operand)))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/transition_performances.md") (range (start 7 16) (end 7 37)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transition_performances.md") (range (start 8 16) (end 8 37)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Natural")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transition_performances.md") (range (start 9 16) (end 9 42)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::isEmpty")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transition_performances.md") (range (start 10 16) (end 10 39)) (probe (position 10 16))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "Occurrences::Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transition_performances.md") (range (start 11 16) (end 11 42)) (probe (position 11 16))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0) (authored-target "Occurrences::HappensBefore")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transition_performances.md") (range (start 12 16) (end 12 41)) (probe (position 12 16))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0) (authored-target "Performances::Performance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transition_performances.md") (range (start 13 16) (end 13 40)) (probe (position 13 16))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0) (authored-target "Performances::Evaluation")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transition_performances.md") (range (start 14 16) (end 14 42)) (probe (position 14 16))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0) (authored-target "Transfers::MessageTransfer")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transition_performances.md") (range (start 15 16) (end 15 44)) (probe (position 15 16))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (anonymous (kind import) (ordinal 8))))) (kind membershipImport) (ordinal 0) (authored-target "Transfers::AcceptPerformance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transition_performances.md") (range (start 16 16) (end 16 45)) (probe (position 16 16))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (anonymous (kind import) (ordinal 9))))) (kind membershipImport) (ordinal 0) (authored-target "Transfers::acceptPerformances")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transition_performances.md") (range (start 17 16) (end 17 41)) (probe (position 17 16))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (anonymous (kind import) (ordinal 10))))) (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::allTrue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transition_performances.md") (range (start 18 16) (end 18 39)) (probe (position 18 16))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (anonymous (kind import) (ordinal 11))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::size")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transition_performances.md") (range (start 50 52) (end 50 73)) (probe (position 50 52))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::NonStateTransitionPerformance"))) (kind specialization) (ordinal 0) (authored-target "TransitionPerformance")
      (outcome (status resolved) (target (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance")))))
  )
  (query (document "memory://snapshot/transition_performances.md") (range (start 51 26) (end 51 33)) (probe (position 51 26))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::NonStateTransitionPerformance::isTriggerAfter"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transition_performances.md") (range (start 53 25) (end 53 32)) (probe (position 53 25))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::NonStateTransitionPerformance::taNum"))) (kind featureTyping) (ordinal 0) (authored-target "Natural")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transition_performances.md") (range (start 63 2) (end 63 9)) (probe (position 63 2))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::TPCGuardConstraint"))) (kind expressionOperand) (ordinal 0) (authored-target "private")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transition_performances.md") (range (start 41 18) (end 41 25)) (probe (position 41 18))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::accNum"))) (kind featureTyping) (ordinal 0) (authored-target "Natural")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transition_performances.md") (range (start 42 17) (end 42 34)) (probe (position 42 17))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::accept"))) (kind featureTyping) (ordinal 0) (authored-target "AcceptPerformance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transition_performances.md") (range (start 42 51) (end 42 74)) (probe (position 42 51))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::accept"))) (kind subsetting) (ordinal 0) (authored-target "timeEnclosedOccurrences")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transition_performances.md") (range (start 42 76) (end 42 94)) (probe (position 42 76))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::accept"))) (kind subsetting) (ordinal 1) (authored-target "acceptPerformances")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transition_performances.md") (range (start 43 21) (end 43 37)) (probe (position 43 21))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "acceptedTransfer")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transition_performances.md") (range (start 43 40) (end 43 47)) (probe (position 43 40))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "trigger")
      (outcome (status resolved) (target (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::trigger")))))
  )
  (query (document "memory://snapshot/transition_performances.md") (range (start 25 25) (end 25 45)) (probe (position 25 25))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::effect"))) (kind subsetting) (ordinal 0) (authored-target "enclosedPerformances")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transition_performances.md") (range (start 24 24) (end 24 44)) (probe (position 24 24))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::guard"))) (kind subsetting) (ordinal 0) (authored-target "enclosedPerformances")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transition_performances.md") (range (start 28 26) (end 28 39)) (probe (position 28 26))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::transitionLink"))) (kind featureTyping) (ordinal 0) (authored-target "HappensBefore")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transition_performances.md") (range (start 23 19) (end 23 34)) (probe (position 23 19))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::trigger"))) (kind featureTyping) (ordinal 0) (authored-target "MessageTransfer")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transition_performances.md") (range (start 27 26) (end 27 36)) (probe (position 27 26))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::triggerTarget"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/transition_performances.md") (range (start 27 49) (end 27 53)) (probe (position 27 49))
    (reference (id (source (node (document "memory://snapshot/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::triggerTarget"))) (kind expressionOperand) (ordinal 0) (authored-target "this")
      (outcome (status unresolved)))
  )
)
~~~
