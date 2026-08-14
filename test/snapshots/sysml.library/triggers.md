# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Semantic Library/Triggers
type=file
~~~
# SOURCE
~~~kerml
standard library package Triggers {
	doc
	/*
	 * This package contains functions that return ChangeSignals for triggering
	 * when a Boolean condition changes from false to true, at a specific time
	 * or after a specific time delay.
	 */

	private import ScalarValues::Boolean;
	private import ScalarValues::NumericalValue;
	private import Occurrences::Occurrence;
	
	public import Clocks::*;
	public import Observation::*;
	
	struct TimeSignal :> ChangeSignal {
		doc
		/*
		 * A TimeSignal is a ChangeSignal whose condition is the currentTime
		 * of a given Clock reaching a specific signalTime.
		 */
	
		feature signalTime : NumericalValue[1] {
			doc
			/*
			 * The time at which the TimeSignal should be sent.
			 */
		}
		
		feature signalClock : Clock[1] {
			doc
			/*
			 * The Clock whose currentTime is being monitored.
			 */
		}
		
		private bool :>> signalCondition {
			doc
			/*
			 * The Boolean condition of the currentTime of the signalClock being
			 * equal to the signalTime.
			 */
		
			signalClock.currentTime == signalTime
		}
	}
	
	function TriggerWhen {
		doc
		/*
		 * TriggerWhen returns a monitored ChangeSignal for a given condition,
		 * to be sent to a given receiver when the condition occurs.
		 */
	
		in bool condition[1] {
			doc
			/*
			 * The BooleanExpression to be monitored for changing from 
			 * false to true.
		 */
		}
		
		in feature receiver : Occurrence [1] {
			doc
			/*
			 * The Occurrence to which the ChangeSignal is to be sent.
			 */
		}
		
		in feature monitor : ChangeMonitor[1] default defaultMonitor {
			doc
			/*
			 * The ChangeMonitor to be used to monitor the ChangeSignal condition.
			 * The default is the Observation::defaultMonitor.
			 */
		}
		
		return feature changeSignal : ChangeSignal[1] = new ChangeSignal(condition, monitor) {
			doc
			/*
			 * The ChangeSignal for the condition, as monitored by the monitor.
			 */
		}
		
		step :> monitor.startObservation {
			in observer = receiver;
			in signal = changeSignal;
		}		
	}
	
	function TriggerAt {
		doc
		/*
		 * TriggerAt returns a monitored TimeSignal to be sent to a receiver when
		 * the currentTime of a given Clock reaches a specific timeInstant. 
		 */
	
		in feature timeInstant : NumericalValue[1] {
			doc
			/*
			 * The time instant, relative to the clock, at which the TimeSignal should be sent. 
			 */
		}
		
		in feature receiver : Occurrence[1] {
			doc
			/*
			 * The Occurrence to which the TimeSignal is to be sent.
			 */
		}
		
		in feature clock : Clock[1] default localClock {
			doc
			/*
			 * The Clock to be used as the reference for the timeInstant. The default is
			 * the localClock, which will be bound when the function is invoked. 
			 */
		}
		
		in feature monitor : ChangeMonitor[1] default defaultMonitor {
			doc
			/*
			 * The ChangeMonitor to be used to monitor the TimeSignal condition.
			 * The default is the Observation::defaultMonitor.
			 */
		}
		
		return feature timeSignal : TimeSignal[1] = new TimeSignal(timeInstant, clock, monitor) {
			doc
			/*
			 * The TimeSignal for the given timeInstant, as monitored by the monitor.
			 */
		}
		
		step :> monitor.startObservation {
			in observer = receiver;
			in signal = timeSignal;
		}
	}
	
	function TriggerAfter {
		doc
		/*
		 * TriggerAfter returns a monitored TimeSignal to be sent to a receiver after
		 * a certain time delay relative to a given Clock.
		 */
	
		in feature delay : NumericalValue[1] {
			doc
			/*
			 * The time duration, relative to the clock, after which the TimeSignal is sent.
			 */
		}
		
		in feature receiver : Occurrence[1] {
			doc
			/*
			 * The Occurrence to which the TimeSignal is to be sent.
			 */
		}
		
		in feature clock : Clock[1] default localClock {
			doc
			/*
			 * The Clock to be used as the reference for the time delay. The default is
			 * the localClock, which will be bound when the function is invoked. 
			 */
		}
		
		in feature monitor : ChangeMonitor[1] default defaultMonitor {
			doc
			/*
			 * The ChangeMonitor to be used to monitor the TimeSignal condition.
			 * The default is the Observation::defaultMonitor.
			 */
		}
		
		return signal : TimeSignal[1] = 
			TriggerAt(clock.currentTime + delay, receiver, clock, monitor) {
			doc
			/*
			 * The TimeSignal for the currentTime of the clock when the function is invoked
			 * plus the given time delay, as monitored by the monitor.
			 */
		}
	}	
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/triggers.md"
    (diagnostics
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
        (range (start 9 16) (end 9 44))
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
        (range (start 12 15) (end 12 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 13 15) (end 13 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 15 22) (end 15 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 22 23) (end 22 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 29 24) (end 29 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 36 19) (end 36 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 43 3) (end 43 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 62 24) (end 62 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 69 23) (end 69 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 69 48) (end 69 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 77 32) (end 77 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 77 54) (end 77 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 84 10) (end 84 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 97 27) (end 97 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 104 24) (end 104 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 111 21) (end 111 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 111 38) (end 111 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 119 23) (end 119 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 119 48) (end 119 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 134 10) (end 134 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 147 21) (end 147 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 154 24) (end 154 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 161 21) (end 161 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 161 38) (end 161 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 169 23) (end 169 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 169 48) (end 169 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 178 13) (end 178 30))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:53f10488723397ed2a09a22d6f301d8c999224bd41a89da6ce79aa64fa11d49e") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/triggers.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Boolean") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/triggers.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::NumericalValue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/triggers.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Occurrences::Occurrence") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/triggers.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "Clocks") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/triggers.md") (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "Observation") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TimeSignal"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ChangeSignal"))))
    (declaration (id (node (document "memory://snapshot/triggers.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (redefinition (reference "signalCondition")) (expressionOperand (reference "signalTime")) (memberAccessOperand (reference "signalClock::currentTime"))))
    (declaration (id (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TimeSignal::signalClock"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Clock"))))
    (declaration (id (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TimeSignal::signalTime"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "NumericalValue"))))
    (declaration (id (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::clock"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Clock") (direction in)) (expressionOperand (reference "localClock"))))
    (declaration (id (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::delay"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "NumericalValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::monitor"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ChangeMonitor") (direction in)) (expressionOperand (reference "defaultMonitor"))))
    (declaration (id (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::receiver"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence") (direction in))))
    (declaration (id (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::signal"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TimeSignal")) (expressionOperand (reference "delay")) (expressionOperand (reference "receiver")) (expressionOperand (reference "clock")) (expressionOperand (reference "monitor")) (memberAccessOperand (reference "clock::currentTime")) (invocationCallee (reference "TriggerAt"))))
    (declaration (id (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/triggers.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "monitor::startObservation"))))
    (declaration (id (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::::observer"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "receiver"))))
    (declaration (id (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::::signal"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "timeSignal"))))
    (declaration (id (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::clock"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Clock") (direction in)) (expressionOperand (reference "localClock"))))
    (declaration (id (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::monitor"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ChangeMonitor") (direction in)) (expressionOperand (reference "defaultMonitor"))))
    (declaration (id (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::receiver"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence") (direction in))))
    (declaration (id (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::timeInstant"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "NumericalValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::timeSignal"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TimeSignal")) (expressionOperand (reference "timeInstant")) (expressionOperand (reference "clock")) (expressionOperand (reference "monitor")) (invocationCallee (reference "TimeSignal"))))
    (declaration (id (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/triggers.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "monitor::startObservation"))))
    (declaration (id (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen::::observer"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "receiver"))))
    (declaration (id (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen::::signal"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "changeSignal"))))
    (declaration (id (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen::changeSignal"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ChangeSignal")) (expressionOperand (reference "condition")) (expressionOperand (reference "monitor")) (invocationCallee (reference "ChangeSignal"))))
    (declaration (id (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen::condition"))) (kind parameter) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen::monitor"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ChangeMonitor") (direction in)) (expressionOperand (reference "defaultMonitor"))))
    (declaration (id (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen::receiver"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence") (direction in))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/triggers.md") (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Clocks")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (anonymous (kind import) (ordinal 4))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Observation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::NumericalValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "Occurrences::Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TimeSignal"))) (kind specialization) (ordinal 0))
      (authored-target "ChangeSignal")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "signalCondition")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "signalTime")
      (outcome (status resolved) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TimeSignal::signalTime")))))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "signalClock::currentTime")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TimeSignal::signalClock"))) (kind featureTyping) (ordinal 0))
      (authored-target "Clock")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TimeSignal::signalTime"))) (kind featureTyping) (ordinal 0))
      (authored-target "NumericalValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::clock"))) (kind featureTyping) (ordinal 0))
      (authored-target "Clock")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::clock"))) (kind expressionOperand) (ordinal 0))
      (authored-target "localClock")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::delay"))) (kind featureTyping) (ordinal 0))
      (authored-target "NumericalValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::monitor"))) (kind featureTyping) (ordinal 0))
      (authored-target "ChangeMonitor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::monitor"))) (kind expressionOperand) (ordinal 0))
      (authored-target "defaultMonitor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::receiver"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::signal"))) (kind featureTyping) (ordinal 0))
      (authored-target "TimeSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TimeSignal")))))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::signal"))) (kind expressionOperand) (ordinal 0))
      (authored-target "delay")
      (outcome (status resolved) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::delay")))))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::signal"))) (kind expressionOperand) (ordinal 1))
      (authored-target "receiver")
      (outcome (status resolved) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::receiver")))))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::signal"))) (kind expressionOperand) (ordinal 2))
      (authored-target "clock")
      (outcome (status resolved) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::clock")))))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::signal"))) (kind expressionOperand) (ordinal 3))
      (authored-target "monitor")
      (outcome (status resolved) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::monitor")))))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::signal"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "clock::currentTime")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::signal"))) (kind invocationCallee) (ordinal 0))
      (authored-target "TriggerAt")
      (outcome (status resolved) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt")))))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind subsetting) (ordinal 0))
      (authored-target "monitor::startObservation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::::observer"))) (kind expressionOperand) (ordinal 0))
      (authored-target "receiver")
      (outcome (status resolved) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::receiver")))))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::::signal"))) (kind expressionOperand) (ordinal 0))
      (authored-target "timeSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::timeSignal")))))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::clock"))) (kind featureTyping) (ordinal 0))
      (authored-target "Clock")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::clock"))) (kind expressionOperand) (ordinal 0))
      (authored-target "localClock")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::monitor"))) (kind featureTyping) (ordinal 0))
      (authored-target "ChangeMonitor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::monitor"))) (kind expressionOperand) (ordinal 0))
      (authored-target "defaultMonitor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::receiver"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::timeInstant"))) (kind featureTyping) (ordinal 0))
      (authored-target "NumericalValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::timeSignal"))) (kind featureTyping) (ordinal 0))
      (authored-target "TimeSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TimeSignal")))))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::timeSignal"))) (kind expressionOperand) (ordinal 0))
      (authored-target "timeInstant")
      (outcome (status resolved) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::timeInstant")))))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::timeSignal"))) (kind expressionOperand) (ordinal 1))
      (authored-target "clock")
      (outcome (status resolved) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::clock")))))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::timeSignal"))) (kind expressionOperand) (ordinal 2))
      (authored-target "monitor")
      (outcome (status resolved) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::monitor")))))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::timeSignal"))) (kind invocationCallee) (ordinal 0))
      (authored-target "TimeSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TimeSignal")))))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind subsetting) (ordinal 0))
      (authored-target "monitor::startObservation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen::::observer"))) (kind expressionOperand) (ordinal 0))
      (authored-target "receiver")
      (outcome (status resolved) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen::receiver")))))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen::::signal"))) (kind expressionOperand) (ordinal 0))
      (authored-target "changeSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen::changeSignal")))))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen::changeSignal"))) (kind featureTyping) (ordinal 0))
      (authored-target "ChangeSignal")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen::changeSignal"))) (kind expressionOperand) (ordinal 0))
      (authored-target "condition")
      (outcome (status resolved) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen::condition")))))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen::changeSignal"))) (kind expressionOperand) (ordinal 1))
      (authored-target "monitor")
      (outcome (status resolved) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen::monitor")))))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen::changeSignal"))) (kind invocationCallee) (ordinal 0))
      (authored-target "ChangeSignal")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen::monitor"))) (kind featureTyping) (ordinal 0))
      (authored-target "ChangeMonitor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen::monitor"))) (kind expressionOperand) (ordinal 0))
      (authored-target "defaultMonitor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen::receiver"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/triggers.md") (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TimeSignal::signalTime"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/triggers.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::signal"))) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TimeSignal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::signal"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::signal"))) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::delay"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::signal"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::signal"))) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::receiver"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::signal"))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::signal"))) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::clock"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::signal"))) (kind expressionOperand) (ordinal 2)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::signal"))) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::monitor"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::signal"))) (kind expressionOperand) (ordinal 3)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::signal"))) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::signal"))) (kind invocationCallee) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::::observer"))) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::receiver"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::::observer"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::::signal"))) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::timeSignal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::::signal"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::timeSignal"))) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TimeSignal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::timeSignal"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::timeSignal"))) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::timeInstant"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::timeSignal"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::timeSignal"))) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::clock"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::timeSignal"))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::timeSignal"))) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::monitor"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::timeSignal"))) (kind expressionOperand) (ordinal 2)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::timeSignal"))) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TimeSignal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::timeSignal"))) (kind invocationCallee) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen::::observer"))) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen::receiver"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen::::observer"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen::::signal"))) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen::changeSignal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen::::signal"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen::changeSignal"))) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen::condition"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen::changeSignal"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen::changeSignal"))) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen::monitor"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen::changeSignal"))) (kind expressionOperand) (ordinal 1)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::clock"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::monitor"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::::observer"))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::::signal"))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::clock"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::monitor"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::timeSignal"))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen::::observer"))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen::::signal"))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen::changeSignal"))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen::monitor"))) (value (kind unresolved-operand)))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/triggers.md") (range (start 12 15) (end 12 24)) (probe (position 12 15))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0) (authored-target "Clocks")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/triggers.md") (range (start 13 15) (end 13 29)) (probe (position 13 15))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (anonymous (kind import) (ordinal 4))))) (kind namespaceImport) (ordinal 0) (authored-target "Observation")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/triggers.md") (range (start 8 16) (end 8 37)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/triggers.md") (range (start 9 16) (end 9 44)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::NumericalValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/triggers.md") (range (start 10 16) (end 10 39)) (probe (position 10 16))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "Occurrences::Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/triggers.md") (range (start 15 22) (end 15 34)) (probe (position 15 22))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TimeSignal"))) (kind specialization) (ordinal 0) (authored-target "ChangeSignal")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/triggers.md") (range (start 36 19) (end 36 34)) (probe (position 36 19))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "signalCondition")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/triggers.md") (range (start 43 30) (end 43 40)) (probe (position 43 30))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "signalTime")
      (outcome (status resolved) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TimeSignal::signalTime")))))
  )
  (query (document "memory://snapshot/triggers.md") (range (start 43 3) (end 43 26)) (probe (position 43 3))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "signalClock::currentTime")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/triggers.md") (range (start 29 24) (end 29 29)) (probe (position 29 24))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TimeSignal::signalClock"))) (kind featureTyping) (ordinal 0) (authored-target "Clock")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/triggers.md") (range (start 22 23) (end 22 37)) (probe (position 22 23))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TimeSignal::signalTime"))) (kind featureTyping) (ordinal 0) (authored-target "NumericalValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/triggers.md") (range (start 161 21) (end 161 26)) (probe (position 161 21))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::clock"))) (kind featureTyping) (ordinal 0) (authored-target "Clock")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/triggers.md") (range (start 161 38) (end 161 48)) (probe (position 161 38))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::clock"))) (kind expressionOperand) (ordinal 0) (authored-target "localClock")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/triggers.md") (range (start 147 21) (end 147 35)) (probe (position 147 21))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::delay"))) (kind featureTyping) (ordinal 0) (authored-target "NumericalValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/triggers.md") (range (start 169 23) (end 169 36)) (probe (position 169 23))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::monitor"))) (kind featureTyping) (ordinal 0) (authored-target "ChangeMonitor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/triggers.md") (range (start 169 48) (end 169 62)) (probe (position 169 48))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::monitor"))) (kind expressionOperand) (ordinal 0) (authored-target "defaultMonitor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/triggers.md") (range (start 154 24) (end 154 34)) (probe (position 154 24))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::receiver"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/triggers.md") (range (start 177 18) (end 177 28)) (probe (position 177 18))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::signal"))) (kind featureTyping) (ordinal 0) (authored-target "TimeSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TimeSignal")))))
  )
  (query (document "memory://snapshot/triggers.md") (range (start 178 33) (end 178 38)) (probe (position 178 33))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::signal"))) (kind expressionOperand) (ordinal 0) (authored-target "delay")
      (outcome (status resolved) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::delay")))))
  )
  (query (document "memory://snapshot/triggers.md") (range (start 178 40) (end 178 48)) (probe (position 178 40))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::signal"))) (kind expressionOperand) (ordinal 1) (authored-target "receiver")
      (outcome (status resolved) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::receiver")))))
  )
  (query (document "memory://snapshot/triggers.md") (range (start 178 50) (end 178 55)) (probe (position 178 50))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::signal"))) (kind expressionOperand) (ordinal 2) (authored-target "clock")
      (outcome (status resolved) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::clock")))))
  )
  (query (document "memory://snapshot/triggers.md") (range (start 178 57) (end 178 64)) (probe (position 178 57))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::signal"))) (kind expressionOperand) (ordinal 3) (authored-target "monitor")
      (outcome (status resolved) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::monitor")))))
  )
  (query (document "memory://snapshot/triggers.md") (range (start 178 13) (end 178 30)) (probe (position 178 13))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::signal"))) (kind memberAccessOperand) (ordinal 0) (authored-target "clock::currentTime")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/triggers.md") (range (start 178 3) (end 178 12)) (probe (position 178 3))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAfter::signal"))) (kind invocationCallee) (ordinal 0) (authored-target "TriggerAt")
      (outcome (status resolved) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt")))))
  )
  (query (document "memory://snapshot/triggers.md") (range (start 134 10) (end 134 34)) (probe (position 134 10))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind subsetting) (ordinal 0) (authored-target "monitor::startObservation")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/triggers.md") (range (start 135 17) (end 135 25)) (probe (position 135 17))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::::observer"))) (kind expressionOperand) (ordinal 0) (authored-target "receiver")
      (outcome (status resolved) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::receiver")))))
  )
  (query (document "memory://snapshot/triggers.md") (range (start 136 15) (end 136 25)) (probe (position 136 15))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::::signal"))) (kind expressionOperand) (ordinal 0) (authored-target "timeSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::timeSignal")))))
  )
  (query (document "memory://snapshot/triggers.md") (range (start 111 21) (end 111 26)) (probe (position 111 21))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::clock"))) (kind featureTyping) (ordinal 0) (authored-target "Clock")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/triggers.md") (range (start 111 38) (end 111 48)) (probe (position 111 38))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::clock"))) (kind expressionOperand) (ordinal 0) (authored-target "localClock")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/triggers.md") (range (start 119 23) (end 119 36)) (probe (position 119 23))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::monitor"))) (kind featureTyping) (ordinal 0) (authored-target "ChangeMonitor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/triggers.md") (range (start 119 48) (end 119 62)) (probe (position 119 48))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::monitor"))) (kind expressionOperand) (ordinal 0) (authored-target "defaultMonitor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/triggers.md") (range (start 104 24) (end 104 34)) (probe (position 104 24))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::receiver"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/triggers.md") (range (start 97 27) (end 97 41)) (probe (position 97 27))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::timeInstant"))) (kind featureTyping) (ordinal 0) (authored-target "NumericalValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/triggers.md") (range (start 127 30) (end 127 40)) (probe (position 127 30))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::timeSignal"))) (kind featureTyping) (ordinal 0) (authored-target "TimeSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TimeSignal")))))
  )
  (query (document "memory://snapshot/triggers.md") (range (start 127 61) (end 127 72)) (probe (position 127 61))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::timeSignal"))) (kind expressionOperand) (ordinal 0) (authored-target "timeInstant")
      (outcome (status resolved) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::timeInstant")))))
  )
  (query (document "memory://snapshot/triggers.md") (range (start 127 74) (end 127 79)) (probe (position 127 74))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::timeSignal"))) (kind expressionOperand) (ordinal 1) (authored-target "clock")
      (outcome (status resolved) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::clock")))))
  )
  (query (document "memory://snapshot/triggers.md") (range (start 127 81) (end 127 88)) (probe (position 127 81))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::timeSignal"))) (kind expressionOperand) (ordinal 2) (authored-target "monitor")
      (outcome (status resolved) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::monitor")))))
  )
  (query (document "memory://snapshot/triggers.md") (range (start 127 50) (end 127 60)) (probe (position 127 50))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerAt::timeSignal"))) (kind invocationCallee) (ordinal 0) (authored-target "TimeSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TimeSignal")))))
  )
  (query (document "memory://snapshot/triggers.md") (range (start 84 10) (end 84 34)) (probe (position 84 10))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind subsetting) (ordinal 0) (authored-target "monitor::startObservation")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/triggers.md") (range (start 85 17) (end 85 25)) (probe (position 85 17))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen::::observer"))) (kind expressionOperand) (ordinal 0) (authored-target "receiver")
      (outcome (status resolved) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen::receiver")))))
  )
  (query (document "memory://snapshot/triggers.md") (range (start 86 15) (end 86 27)) (probe (position 86 15))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen::::signal"))) (kind expressionOperand) (ordinal 0) (authored-target "changeSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen::changeSignal")))))
  )
  (query (document "memory://snapshot/triggers.md") (range (start 77 32) (end 77 44)) (probe (position 77 32))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen::changeSignal"))) (kind featureTyping) (ordinal 0) (authored-target "ChangeSignal")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/triggers.md") (range (start 77 67) (end 77 76)) (probe (position 77 67))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen::changeSignal"))) (kind expressionOperand) (ordinal 0) (authored-target "condition")
      (outcome (status resolved) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen::condition")))))
  )
  (query (document "memory://snapshot/triggers.md") (range (start 77 78) (end 77 85)) (probe (position 77 78))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen::changeSignal"))) (kind expressionOperand) (ordinal 1) (authored-target "monitor")
      (outcome (status resolved) (target (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen::monitor")))))
  )
  (query (document "memory://snapshot/triggers.md") (range (start 77 54) (end 77 66)) (probe (position 77 54))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen::changeSignal"))) (kind invocationCallee) (ordinal 0) (authored-target "ChangeSignal")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/triggers.md") (range (start 69 23) (end 69 36)) (probe (position 69 23))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen::monitor"))) (kind featureTyping) (ordinal 0) (authored-target "ChangeMonitor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/triggers.md") (range (start 69 48) (end 69 62)) (probe (position 69 48))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen::monitor"))) (kind expressionOperand) (ordinal 0) (authored-target "defaultMonitor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/triggers.md") (range (start 62 24) (end 62 34)) (probe (position 62 24))
    (reference (id (source (node (document "memory://snapshot/triggers.md") (qualified-name "Triggers::TriggerWhen::receiver"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
  )
)
~~~
