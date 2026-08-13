# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Semantic Library/Clocks
type=file
~~~
# SOURCE
~~~kerml
standard library package Clocks {
	doc
	/*
	 * This package models Clocks that provide an advancing numerical reference 
	 * usable for quantifying the time of an Occurrence.
	 */

	private import ScalarValues::NumericalValue;
	private import ScalarValues::Real;
	private import Occurrences::Occurrence;
	private import Occurrences::Life;
	private import ControlFunctions::forAll;
	
	private struct UniversalClockLife[1] :> Clock, Life {
	    doc
	    /*
	     * UniversalClockLife is the classifier of the singleton Life of the universalClock.
	     */
	}
	
	feature universalClock : UniversalClockLife[1] {
		doc
		/*
		 * universalClock is a single Clock that can be used as a default universal
		 * time reference.
		 */
	}
	
	abstract struct Clock {
		doc
		/*
		 * A Clock provides a numerical currentTime that advances montonically
		 * over its lifetime. Clock is an abstract base Structure that can be
		 * specialized for different kinds of time quantification (e.g., discrete
		 * time, continuous time, time with units, etc.).
		 */
		 
		private thisClock : Clock :>> self;
		
		var feature currentTime : NumericalValue[1] {
			doc
			/*
			 * A scalar time reference that advances over the lifetime of the Clock. 
			 */
		}
						
		inv timeFlowConstraint {
			doc
			/*
			 * The currentTime of a snapshot of a Clock is equal to
			 * the TimeOf the snapshot relative to that Clock.
			 */
			
			snapshots->forAll{in s : Clock; 
				TimeOf(s, thisClock) == s.currentTime
			}
		}		
	}
	
	abstract function TimeOf {
		doc
		/*
		 * TimeOf returns a numerical timeInstant for a given Occurrence relative to
		 * a given Clock. The timeInstant is the time of the start of the Occurrence,
		 * which is considered to be synchronized with the snapshot of the Clock 
		 * with a currentTime equal to the returned timeInstant.
		 */
		
		in o : Occurrence[1];
		in clock : Clock[1] default localClock;
		return timeInstant : NumericalValue[1];
		
		 inv startTimeConstraint {
		 	doc
			/*
			 * The TimeOf an Occurrence is equal to the time of its start snapshot.
			 */
			 
		 	timeInstant == TimeOf(o.startShot, clock)
		 }	 

		inv timeOrderingConstraint {
			doc
			/*
			 * If one Occurrence happens before another, then the TimeOf the end
			 * snapshot of the first Occurrence is no greater than the TimeOf the 
			 * second Occurrence.
			 */
			
			o.predecessors->forAll{in p : Occurrence; 
				TimeOf(p.endShot, clock) <= timeInstant
			}
		}
				
		inv timeContinuityConstraint {
			doc
			/*
			 * If one Occurrence happens immediately before another, then the TimeOf 
			 * the end snapshot of the first Occurrence equals the TimeOf the second
			 * Occurrence.
			 */
		 
			o.immediatePredecessors->forAll{in p : Occurrence; 
				TimeOf(p.endShot, clock) == timeInstant
			}
		}				
	}
	
	function DurationOf {
		doc
		/*
		 * DurationOf returns the duration of a given Occurrence relative to a
		 * given Clock, which is equal to the TimeOf the end snapshot of the
		 * Occurrence minus the TimeOf its start snapshot.
		 */
		
		in o : Occurrence[1]; 
		in clock : Clock[1] default localClock;
		return duration : NumericalValue =
			TimeOf(o.endShot, clock) - TimeOf(o.startShot, clock);
	}
	
	struct BasicClock :> Clock {
		doc
		/*
		 * A BasicClock is a Clock whose currentTime is a Real number.
		 */
		
		var feature :>> currentTime : Real;
	}
	
	function BasicTimeOf :> TimeOf {
		doc
		/*
		 * BasicTimeOf returns the TimeOf an Occurrence as a Real number relative
		 * to a BasicClock.
		 */

		in o : Occurrence[1];
		in clock : BasicClock[1];
		return : Real[1];
	}
	
	function BasicDurationOf :> DurationOf {
		doc
		/*
		 * BasicDurationOf returns the DurationOf an Occurrence as a Real number relative
		 * to a BasicClock.
		 */
		
		in o : Occurrence[1];
		in clock : BasicClock[1];
		return : Real[1];
	}

}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/clocks.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 16) (end 9 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 16) (end 10 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 16) (end 11 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 13 48) (end 13 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 37 32) (end 37 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 39 28) (end 39 42))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 46 2) (end 56 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 68 9) (end 68 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 69 30) (end 69 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 70 23) (end 70 37))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 72 3) (end 79 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 81 2) (end 92 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 94 2) (end 105 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 116 9) (end 116 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 117 30) (end 117 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 118 20) (end 118 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 119 10) (end 119 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 119 37) (end 119 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 128 32) (end 128 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 138 9) (end 138 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 140 11) (end 140 15))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 150 9) (end 150 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 152 11) (end 152 15))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:5ec509b765f83e5d2e9e087a562d8db10cc8341a8c5581574f6ab933a2f35550") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/clocks.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::NumericalValue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/clocks.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Real") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/clocks.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Occurrences::Occurrence") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/clocks.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Occurrences::Life") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/clocks.md") (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ControlFunctions::forAll") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::BasicClock"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Clock"))))
    (declaration (id (node (document "memory://snapshot/clocks.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "currentTime"))))
    (declaration (id (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::BasicDurationOf"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DurationOf"))))
    (declaration (id (node (document "memory://snapshot/clocks.md") (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::BasicDurationOf::clock"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "BasicClock") (direction in))))
    (declaration (id (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::BasicDurationOf::o"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence") (direction in))))
    (declaration (id (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::BasicTimeOf"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "TimeOf"))))
    (declaration (id (node (document "memory://snapshot/clocks.md") (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::BasicTimeOf::clock"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "BasicClock") (direction in))))
    (declaration (id (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::BasicTimeOf::o"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence") (direction in))))
    (declaration (id (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::Clock"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::Clock::currentTime"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "NumericalValue"))))
    (declaration (id (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::Clock::thisClock"))) (kind default-reference) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "Clock")) (redefinition (reference "self"))))
    (declaration (id (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::DurationOf"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::DurationOf::clock"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Clock") (direction in)) (expressionOperand (reference "localClock"))))
    (declaration (id (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::DurationOf::duration"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "NumericalValue")) (expressionOperand (reference "clock")) (expressionOperand (reference "clock")) (memberAccessOperand (reference "o::endShot")) (memberAccessOperand (reference "o::startShot")) (invocationCallee (reference "TimeOf")) (invocationCallee (reference "TimeOf"))))
    (declaration (id (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::DurationOf::o"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence") (direction in))))
    (declaration (id (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::TimeOf"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::TimeOf::clock"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Clock") (direction in)) (expressionOperand (reference "localClock"))))
    (declaration (id (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::TimeOf::o"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence") (direction in))))
    (declaration (id (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::TimeOf::timeInstant"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "NumericalValue"))))
    (declaration (id (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::UniversalClockLife"))) (kind kerml-classifier) (membership (kind owning) (visibility private)) (authored (membership (kind owning) (visibility private)) (relationships (specialization (reference "Clock")) (specialization (reference "Life"))))
    (declaration (id (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::universalClock"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "UniversalClockLife"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/clocks.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::NumericalValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "Occurrences::Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "Occurrences::Life")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0))
      (authored-target "ControlFunctions::forAll")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::BasicClock"))) (kind specialization) (ordinal 0))
      (authored-target "Clock")
      (outcome (status resolved) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::Clock")))))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "currentTime")
      (outcome (status resolved) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::Clock::currentTime")))))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::BasicDurationOf"))) (kind specialization) (ordinal 0))
      (authored-target "DurationOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::DurationOf")))))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::BasicDurationOf::clock"))) (kind featureTyping) (ordinal 0))
      (authored-target "BasicClock")
      (outcome (status resolved) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::BasicClock")))))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::BasicDurationOf::o"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::BasicTimeOf"))) (kind specialization) (ordinal 0))
      (authored-target "TimeOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::TimeOf")))))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::BasicTimeOf::clock"))) (kind featureTyping) (ordinal 0))
      (authored-target "BasicClock")
      (outcome (status resolved) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::BasicClock")))))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::BasicTimeOf::o"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::Clock::currentTime"))) (kind featureTyping) (ordinal 0))
      (authored-target "NumericalValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::Clock::thisClock"))) (kind featureTyping) (ordinal 0))
      (authored-target "Clock")
      (outcome (status resolved) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::Clock")))))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::Clock::thisClock"))) (kind redefinition) (ordinal 0))
      (authored-target "self")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::DurationOf::clock"))) (kind featureTyping) (ordinal 0))
      (authored-target "Clock")
      (outcome (status resolved) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::Clock")))))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::DurationOf::clock"))) (kind expressionOperand) (ordinal 0))
      (authored-target "localClock")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::DurationOf::duration"))) (kind featureTyping) (ordinal 0))
      (authored-target "NumericalValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::DurationOf::duration"))) (kind expressionOperand) (ordinal 0))
      (authored-target "clock")
      (outcome (status resolved) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::DurationOf::clock")))))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::DurationOf::duration"))) (kind expressionOperand) (ordinal 1))
      (authored-target "clock")
      (outcome (status resolved) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::DurationOf::clock")))))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::DurationOf::duration"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "o::endShot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::DurationOf::duration"))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "o::startShot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::DurationOf::duration"))) (kind invocationCallee) (ordinal 0))
      (authored-target "TimeOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::TimeOf")))))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::DurationOf::duration"))) (kind invocationCallee) (ordinal 1))
      (authored-target "TimeOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::TimeOf")))))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::DurationOf::o"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::TimeOf::clock"))) (kind featureTyping) (ordinal 0))
      (authored-target "Clock")
      (outcome (status resolved) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::Clock")))))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::TimeOf::clock"))) (kind expressionOperand) (ordinal 0))
      (authored-target "localClock")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::TimeOf::o"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::TimeOf::timeInstant"))) (kind featureTyping) (ordinal 0))
      (authored-target "NumericalValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::UniversalClockLife"))) (kind specialization) (ordinal 0))
      (authored-target "Clock")
      (outcome (status resolved) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::Clock")))))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::UniversalClockLife"))) (kind specialization) (ordinal 1))
      (authored-target "Life")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::universalClock"))) (kind featureTyping) (ordinal 0))
      (authored-target "UniversalClockLife")
      (outcome (status resolved) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::UniversalClockLife")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::BasicClock"))) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::Clock"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::BasicClock"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/clocks.md") (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::Clock::currentTime"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/clocks.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::BasicDurationOf"))) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::DurationOf"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::BasicDurationOf"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::BasicDurationOf::clock"))) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::BasicClock"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::BasicDurationOf::clock"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::BasicTimeOf"))) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::TimeOf"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::BasicTimeOf"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::BasicTimeOf::clock"))) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::BasicClock"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::BasicTimeOf::clock"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::Clock::thisClock"))) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::Clock"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::Clock::thisClock"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::DurationOf::clock"))) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::Clock"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::DurationOf::clock"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::DurationOf::duration"))) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::DurationOf::clock"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::DurationOf::duration"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::DurationOf::duration"))) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::DurationOf::clock"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::DurationOf::duration"))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::DurationOf::duration"))) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::TimeOf"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::DurationOf::duration"))) (kind invocationCallee) (ordinal 0)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::DurationOf::duration"))) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::TimeOf"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::DurationOf::duration"))) (kind invocationCallee) (ordinal 1)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::TimeOf::clock"))) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::Clock"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::TimeOf::clock"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::UniversalClockLife"))) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::Clock"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::UniversalClockLife"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::universalClock"))) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::UniversalClockLife"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::universalClock"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::BasicDurationOf::clock"))) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::DurationOf::clock"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::BasicDurationOf::o"))) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::DurationOf::o"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::BasicTimeOf::clock"))) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::TimeOf::clock"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::BasicTimeOf::o"))) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::TimeOf::o"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::DurationOf::clock"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::TimeOf::clock"))) (value (kind unresolved-operand)))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/clocks.md") (range (start 7 16) (end 7 44)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::NumericalValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/clocks.md") (range (start 8 16) (end 8 34)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/clocks.md") (range (start 9 16) (end 9 39)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "Occurrences::Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/clocks.md") (range (start 10 16) (end 10 33)) (probe (position 10 16))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "Occurrences::Life")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/clocks.md") (range (start 11 16) (end 11 40)) (probe (position 11 16))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::forAll")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/clocks.md") (range (start 122 22) (end 122 27)) (probe (position 122 22))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::BasicClock"))) (kind specialization) (ordinal 0) (authored-target "Clock")
      (outcome (status resolved) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::Clock")))))
  )
  (query (document "memory://snapshot/clocks.md") (range (start 128 32) (end 128 36)) (probe (position 128 32))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/clocks.md") (range (start 128 18) (end 128 29)) (probe (position 128 18))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "currentTime")
      (outcome (status resolved) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::Clock::currentTime")))))
  )
  (query (document "memory://snapshot/clocks.md") (range (start 143 29) (end 143 39)) (probe (position 143 29))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::BasicDurationOf"))) (kind specialization) (ordinal 0) (authored-target "DurationOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::DurationOf")))))
  )
  (query (document "memory://snapshot/clocks.md") (range (start 152 11) (end 152 15)) (probe (position 152 11))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/clocks.md") (range (start 151 13) (end 151 23)) (probe (position 151 13))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::BasicDurationOf::clock"))) (kind featureTyping) (ordinal 0) (authored-target "BasicClock")
      (outcome (status resolved) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::BasicClock")))))
  )
  (query (document "memory://snapshot/clocks.md") (range (start 150 9) (end 150 19)) (probe (position 150 9))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::BasicDurationOf::o"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/clocks.md") (range (start 131 25) (end 131 31)) (probe (position 131 25))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::BasicTimeOf"))) (kind specialization) (ordinal 0) (authored-target "TimeOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::TimeOf")))))
  )
  (query (document "memory://snapshot/clocks.md") (range (start 140 11) (end 140 15)) (probe (position 140 11))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/clocks.md") (range (start 139 13) (end 139 23)) (probe (position 139 13))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::BasicTimeOf::clock"))) (kind featureTyping) (ordinal 0) (authored-target "BasicClock")
      (outcome (status resolved) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::BasicClock")))))
  )
  (query (document "memory://snapshot/clocks.md") (range (start 138 9) (end 138 19)) (probe (position 138 9))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::BasicTimeOf::o"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/clocks.md") (range (start 39 28) (end 39 42)) (probe (position 39 28))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::Clock::currentTime"))) (kind featureTyping) (ordinal 0) (authored-target "NumericalValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/clocks.md") (range (start 37 22) (end 37 27)) (probe (position 37 22))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::Clock::thisClock"))) (kind featureTyping) (ordinal 0) (authored-target "Clock")
      (outcome (status resolved) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::Clock")))))
  )
  (query (document "memory://snapshot/clocks.md") (range (start 37 32) (end 37 36)) (probe (position 37 32))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::Clock::thisClock"))) (kind redefinition) (ordinal 0) (authored-target "self")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/clocks.md") (range (start 117 13) (end 117 18)) (probe (position 117 13))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::DurationOf::clock"))) (kind featureTyping) (ordinal 0) (authored-target "Clock")
      (outcome (status resolved) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::Clock")))))
  )
  (query (document "memory://snapshot/clocks.md") (range (start 117 30) (end 117 40)) (probe (position 117 30))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::DurationOf::clock"))) (kind expressionOperand) (ordinal 0) (authored-target "localClock")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/clocks.md") (range (start 118 20) (end 118 34)) (probe (position 118 20))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::DurationOf::duration"))) (kind featureTyping) (ordinal 0) (authored-target "NumericalValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/clocks.md") (range (start 119 21) (end 119 26)) (probe (position 119 21))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::DurationOf::duration"))) (kind expressionOperand) (ordinal 0) (authored-target "clock")
      (outcome (status resolved) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::DurationOf::clock")))))
  )
  (query (document "memory://snapshot/clocks.md") (range (start 119 50) (end 119 55)) (probe (position 119 50))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::DurationOf::duration"))) (kind expressionOperand) (ordinal 1) (authored-target "clock")
      (outcome (status resolved) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::DurationOf::clock")))))
  )
  (query (document "memory://snapshot/clocks.md") (range (start 119 10) (end 119 19)) (probe (position 119 10))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::DurationOf::duration"))) (kind memberAccessOperand) (ordinal 0) (authored-target "o::endShot")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/clocks.md") (range (start 119 37) (end 119 48)) (probe (position 119 37))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::DurationOf::duration"))) (kind memberAccessOperand) (ordinal 1) (authored-target "o::startShot")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/clocks.md") (range (start 119 3) (end 119 9)) (probe (position 119 3))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::DurationOf::duration"))) (kind invocationCallee) (ordinal 0) (authored-target "TimeOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::TimeOf")))))
  )
  (query (document "memory://snapshot/clocks.md") (range (start 119 30) (end 119 36)) (probe (position 119 30))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::DurationOf::duration"))) (kind invocationCallee) (ordinal 1) (authored-target "TimeOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::TimeOf")))))
  )
  (query (document "memory://snapshot/clocks.md") (range (start 116 9) (end 116 19)) (probe (position 116 9))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::DurationOf::o"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/clocks.md") (range (start 69 13) (end 69 18)) (probe (position 69 13))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::TimeOf::clock"))) (kind featureTyping) (ordinal 0) (authored-target "Clock")
      (outcome (status resolved) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::Clock")))))
  )
  (query (document "memory://snapshot/clocks.md") (range (start 69 30) (end 69 40)) (probe (position 69 30))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::TimeOf::clock"))) (kind expressionOperand) (ordinal 0) (authored-target "localClock")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/clocks.md") (range (start 68 9) (end 68 19)) (probe (position 68 9))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::TimeOf::o"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/clocks.md") (range (start 70 23) (end 70 37)) (probe (position 70 23))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::TimeOf::timeInstant"))) (kind featureTyping) (ordinal 0) (authored-target "NumericalValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/clocks.md") (range (start 13 41) (end 13 46)) (probe (position 13 41))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::UniversalClockLife"))) (kind specialization) (ordinal 0) (authored-target "Clock")
      (outcome (status resolved) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::Clock")))))
  )
  (query (document "memory://snapshot/clocks.md") (range (start 13 48) (end 13 52)) (probe (position 13 48))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::UniversalClockLife"))) (kind specialization) (ordinal 1) (authored-target "Life")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/clocks.md") (range (start 20 26) (end 20 44)) (probe (position 20 26))
    (reference (id (source (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::universalClock"))) (kind featureTyping) (ordinal 0) (authored-target "UniversalClockLife")
      (outcome (status resolved) (target (node (document "memory://snapshot/clocks.md") (qualified-name "Clocks::UniversalClockLife")))))
  )
)
~~~
