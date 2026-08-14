# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Semantic Library/Observation
type=file
~~~
# SOURCE
~~~kerml
standard library package Observation {
	doc
	/*
	 * This package models a framework for monitoring Boolean conditions and notifying
	 * registered observers when they change from false to true.
	 */
	 
	private import ScalarValues::Boolean;
	private import Occurrences::Occurrence;
	private import Occurrences::Life;
	private import SequenceFunctions::including;
	private import SequenceFunctions::excluding;
	private import ControlFunctions::select;
	private import ControlPerformances::DecisionPerformance;
	private import ControlPerformances::IfThenPerformance;
	private import FeatureReferencingPerformances::FeatureWritePerformance;
	private import FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance;
	private import Transfers::TransferBefore;

    private struct DefaultMonitorLife[1] :> ChangeMonitor, Life {
        doc
        /*
         * DefaultMonitorLife is the classifier of the singleton Life of the defaultMonitor.
         */
    }
    
	feature defaultMonitor[1] : DefaultMonitorLife {
		doc
		/*
		 * defaultMonitor is a single ChangeMonitor that can be used as a default.
		 */
	}

	struct ChangeSignal {
		doc
		/*
		 * A ChangeSignal is a signal to be sent when the Boolean result of its
		 * changeCondition Expression changes from false to true.
		 */

		bool signalCondition {
			doc
			/*
			 * A BooleanExpression whose result is being monitored.
			 */
		}
		
		feature signalMonitor : ChangeMonitor {
			doc
			/*
			 * The ChangeMonitor responsible for monitoring the signalCondition.
			 */
		}
	}
	
	private behavior ObserveChange {
		doc
		/*
		 * Each Performance of ObserveChange waits for the result of the Boolean 
		 * condition of a given ChangeSignal to change from false to true, and, when 
		 * it does, sends the ChangeSignal to a given observer Occurrence.
		 */

		in feature changeObserver : Occurrence[1];
		in feature changeSignal : ChangeSignal[1];
		
		composite step wait : IfThenPerformance {
			doc
			/*
			 * If the result of the changeSignal.signalCondition is false, then wait for 
			 * it to become true.
			 */

			in bool redefines ifTest {
				not changeSignal.signalCondition()
			}
			in step redefines thenClause : BooleanEvaluationResultToMonitorPerformance {
				in bool onOccurrence = changeSignal.signalCondition;
			}
		}		
		
		succession wait then transfer;
		
	    step transfer : TransferBefore[1] 
	    	redefines outgoingTransfersFromSelf 
	    	subsets changeObserver.incomingTransfers {
	    	doc
			/*
			 * Then send changeSignal to changeObserver.
			 */
			 
	    	end feature source {
	    		feature redefines sourceOutput = changeSignal;
	    	}
	    	end feature target;
	    }
	}
	
	struct ChangeMonitor {
		doc
		/*
		 * A ChangeMonitor is a collection of ongoing ChangeSignal observations 
		 * for various observer Occurrences. It provides convenient operations for 
		 * starting and canceling the observations it manages.
		 */

		private thisMonitor : ChangeMonitor redefines self;
		private composite feature observations[0..*] : ObserveChange;
		
		private behavior AssignObservations specializes FeatureWritePerformance {
			doc
			/*
			 * Assign a replacement set of observations as those being managed by a
			 * given ChangeMonitor.
			 */

			in feature monitor : ChangeMonitor redefines onOccurrence {
				feature redefines startingAt {
					feature redefines accessedFeature, observations;
				}
			}
			inout feature redefines replacementValues[0..*] : ObserveChange;
		}
		
		step startObservation { 
			doc
			/*
			 * Start an observation of a given ChangeSignal for a given Occurrence.
			 */

			in observer : Occurrence[1]; 
			in signal : ChangeSignal[1];
			private composite step observation : ObserveChange {
				in changeObserver = observer;
				in changeSignal = signal;
			}
			private composite step addObservation : AssignObservations[1] {
				in monitor = thisMonitor; 
				inout replacementValues = observations->including(observation);	
			}
		}
		
		step cancelObservation { 
			doc
			/*
			 * Cancel all observations of a given ChangeSignal for a given Occurrence. 
			 */

			in observer : Occurrence[1]; 
			in signal : ChangeSignal[1];
			private feature observations[0..*] : ObserveChange = 
				observations->select{in observation : ObserveChange; 
					observation.changeObserver == observer and observation.changeSignal == signal
				};
			private composite step removeObservation : AssignObservations[1] {
				in monitor = thisMonitor; 
				inout replacementValues = observations->excluding(observations);
			}
		}
	}	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/observation.md"
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
        (range (start 8 16) (end 8 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 16) (end 9 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 16) (end 10 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 16) (end 11 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 12 16) (end 12 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 13 16) (end 13 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 14 16) (end 14 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 15 16) (end 15 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 16 16) (end 16 91))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 17 16) (end 17 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 19 59) (end 19 63))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 63 2) (end 63 44))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 64 2) (end 64 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 66 24) (end 66 41))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 73 3) (end 75 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 76 3) (end 78 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 81 2) (end 81 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 83 21) (end 83 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 84 16) (end 84 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 85 14) (end 85 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 92 25) (end 92 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 92 40) (end 92 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 106 48) (end 106 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 109 50) (end 109 73))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 116 3) (end 120 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 121 3) (end 121 67))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 130 17) (end 130 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 148 17) (end 148 27))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:d98f56ba402668116262a876a99c91e387677cead1ab91ae2f38a8b4ddaddabb") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/observation.md") (qualified-name "Observation"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/observation.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Boolean") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/observation.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Occurrences::Occurrence") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/observation.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Occurrences::Life") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/observation.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::including") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/observation.md") (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::excluding") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/observation.md") (anonymous (kind import) (ordinal 5))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ControlFunctions::select") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/observation.md") (anonymous (kind import) (ordinal 6))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ControlPerformances::DecisionPerformance") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/observation.md") (anonymous (kind import) (ordinal 7))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ControlPerformances::IfThenPerformance") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/observation.md") (anonymous (kind import) (ordinal 8))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "FeatureReferencingPerformances::FeatureWritePerformance") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/observation.md") (anonymous (kind import) (ordinal 9))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/observation.md") (anonymous (kind import) (ordinal 10))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Transfers::TransferBefore") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::AssignObservations"))) (kind kerml-classifier) (membership (kind owning) (visibility private)) (authored (membership (kind owning) (visibility private)) (relationships (specialization (reference "FeatureWritePerformance"))))
    (declaration (id (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation::observations"))) (kind kerml-feature) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "ObserveChange")) (expressionOperand (reference "observations"))))
    (declaration (id (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation::observer"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence") (direction in))))
    (declaration (id (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation::removeObservation"))) (kind kerml-feature) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "AssignObservations"))))
    (declaration (id (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation::removeObservation::monitor"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "thisMonitor"))))
    (declaration (id (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation::removeObservation::replacementValues"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "observations")) (expressionOperand (reference "observations"))))
    (declaration (id (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation::signal"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ChangeSignal") (direction in))))
    (declaration (id (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::observations"))) (kind kerml-feature) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "ObserveChange"))))
    (declaration (id (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::addObservation"))) (kind kerml-feature) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "AssignObservations"))))
    (declaration (id (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::addObservation::monitor"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "thisMonitor"))))
    (declaration (id (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::addObservation::replacementValues"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "observations")) (expressionOperand (reference "observation"))))
    (declaration (id (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::observation"))) (kind kerml-feature) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "ObserveChange"))))
    (declaration (id (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::observation::changeObserver"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "observer"))))
    (declaration (id (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::observation::changeSignal"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "signal"))))
    (declaration (id (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::observer"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence") (direction in))))
    (declaration (id (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::signal"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ChangeSignal") (direction in))))
    (declaration (id (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::thisMonitor"))) (kind default-reference) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "ChangeMonitor")) (redefinition (reference "self"))))
    (declaration (id (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeSignal"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeSignal::signalCondition"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeSignal::signalMonitor"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ChangeMonitor"))))
    (declaration (id (node (document "memory://snapshot/observation.md") (qualified-name "Observation::DefaultMonitorLife"))) (kind kerml-classifier) (membership (kind owning) (visibility private)) (authored (membership (kind owning) (visibility private)) (relationships (specialization (reference "ChangeMonitor")) (specialization (reference "Life"))))
    (declaration (id (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ObserveChange"))) (kind kerml-classifier) (membership (kind owning) (visibility private)))
    (declaration (id (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ObserveChange::transfer"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TransferBefore")) (subsetting (reference "changeObserver::incomingTransfers")) (redefinition (reference "outgoingTransfersFromSelf"))))
    (declaration (id (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ObserveChange::transfer::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/observation.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "sourceOutput")) (expressionOperand (reference "changeSignal"))))
    (declaration (id (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ObserveChange::transfer::target"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ObserveChange::wait"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "IfThenPerformance"))))
    (declaration (id (node (document "memory://snapshot/observation.md") (qualified-name "Observation::defaultMonitor"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DefaultMonitorLife"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/observation.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/observation.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "Occurrences::Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/observation.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "Occurrences::Life")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/observation.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::including")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/observation.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::excluding")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/observation.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0))
      (authored-target "ControlFunctions::select")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/observation.md") (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0))
      (authored-target "ControlPerformances::DecisionPerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/observation.md") (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0))
      (authored-target "ControlPerformances::IfThenPerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/observation.md") (anonymous (kind import) (ordinal 8))))) (kind membershipImport) (ordinal 0))
      (authored-target "FeatureReferencingPerformances::FeatureWritePerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/observation.md") (anonymous (kind import) (ordinal 9))))) (kind membershipImport) (ordinal 0))
      (authored-target "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/observation.md") (anonymous (kind import) (ordinal 10))))) (kind membershipImport) (ordinal 0))
      (authored-target "Transfers::TransferBefore")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::AssignObservations"))) (kind specialization) (ordinal 0))
      (authored-target "FeatureWritePerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation::observations"))) (kind featureTyping) (ordinal 0))
      (authored-target "ObserveChange")
      (outcome (status resolved) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ObserveChange")))))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation::observations"))) (kind expressionOperand) (ordinal 0))
      (authored-target "observations")
      (outcome (status resolved) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation::observations")))))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation::observer"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation::removeObservation"))) (kind featureTyping) (ordinal 0))
      (authored-target "AssignObservations")
      (outcome (status resolved) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::AssignObservations")))))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation::removeObservation::monitor"))) (kind expressionOperand) (ordinal 0))
      (authored-target "thisMonitor")
      (outcome (status resolved) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::thisMonitor")))))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation::removeObservation::replacementValues"))) (kind expressionOperand) (ordinal 0))
      (authored-target "observations")
      (outcome (status resolved) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation::observations")))))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation::removeObservation::replacementValues"))) (kind expressionOperand) (ordinal 1))
      (authored-target "observations")
      (outcome (status resolved) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation::observations")))))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation::signal"))) (kind featureTyping) (ordinal 0))
      (authored-target "ChangeSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeSignal")))))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::observations"))) (kind featureTyping) (ordinal 0))
      (authored-target "ObserveChange")
      (outcome (status resolved) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ObserveChange")))))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::addObservation"))) (kind featureTyping) (ordinal 0))
      (authored-target "AssignObservations")
      (outcome (status resolved) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::AssignObservations")))))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::addObservation::monitor"))) (kind expressionOperand) (ordinal 0))
      (authored-target "thisMonitor")
      (outcome (status resolved) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::thisMonitor")))))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::addObservation::replacementValues"))) (kind expressionOperand) (ordinal 0))
      (authored-target "observations")
      (outcome (status resolved) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::observations")))))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::addObservation::replacementValues"))) (kind expressionOperand) (ordinal 1))
      (authored-target "observation")
      (outcome (status resolved) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::observation")))))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::observation"))) (kind featureTyping) (ordinal 0))
      (authored-target "ObserveChange")
      (outcome (status resolved) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ObserveChange")))))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::observation::changeObserver"))) (kind expressionOperand) (ordinal 0))
      (authored-target "observer")
      (outcome (status resolved) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::observer")))))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::observation::changeSignal"))) (kind expressionOperand) (ordinal 0))
      (authored-target "signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::signal")))))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::observer"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::signal"))) (kind featureTyping) (ordinal 0))
      (authored-target "ChangeSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeSignal")))))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::thisMonitor"))) (kind featureTyping) (ordinal 0))
      (authored-target "ChangeMonitor")
      (outcome (status resolved) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor")))))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::thisMonitor"))) (kind redefinition) (ordinal 0))
      (authored-target "self")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeSignal::signalMonitor"))) (kind featureTyping) (ordinal 0))
      (authored-target "ChangeMonitor")
      (outcome (status resolved) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor")))))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::DefaultMonitorLife"))) (kind specialization) (ordinal 0))
      (authored-target "ChangeMonitor")
      (outcome (status resolved) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor")))))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::DefaultMonitorLife"))) (kind specialization) (ordinal 1))
      (authored-target "Life")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ObserveChange::transfer"))) (kind featureTyping) (ordinal 0))
      (authored-target "TransferBefore")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ObserveChange::transfer"))) (kind subsetting) (ordinal 0))
      (authored-target "changeObserver::incomingTransfers")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ObserveChange::transfer"))) (kind redefinition) (ordinal 0))
      (authored-target "outgoingTransfersFromSelf")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/observation.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "sourceOutput")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/observation.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "changeSignal")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ObserveChange::wait"))) (kind featureTyping) (ordinal 0))
      (authored-target "IfThenPerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::defaultMonitor"))) (kind featureTyping) (ordinal 0))
      (authored-target "DefaultMonitorLife")
      (outcome (status resolved) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::DefaultMonitorLife")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation::observations"))) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ObserveChange"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation::observations"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation::observations"))) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation::observations"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation::observations"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation::removeObservation"))) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::AssignObservations"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation::removeObservation"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation::removeObservation::monitor"))) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::thisMonitor"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation::removeObservation::monitor"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation::removeObservation::replacementValues"))) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation::observations"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation::removeObservation::replacementValues"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation::removeObservation::replacementValues"))) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation::observations"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation::removeObservation::replacementValues"))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation::signal"))) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeSignal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation::signal"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::observations"))) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ObserveChange"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::observations"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::addObservation"))) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::AssignObservations"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::addObservation"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::addObservation::monitor"))) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::thisMonitor"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::addObservation::monitor"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::addObservation::replacementValues"))) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::observations"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::addObservation::replacementValues"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::addObservation::replacementValues"))) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::observation"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::addObservation::replacementValues"))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::observation"))) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ObserveChange"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::observation"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::observation::changeObserver"))) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::observer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::observation::changeObserver"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::observation::changeSignal"))) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::signal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::observation::changeSignal"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::signal"))) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeSignal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::signal"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::thisMonitor"))) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::thisMonitor"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeSignal::signalMonitor"))) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeSignal::signalMonitor"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::DefaultMonitorLife"))) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::DefaultMonitorLife"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::defaultMonitor"))) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::DefaultMonitorLife"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::defaultMonitor"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation::observations"))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation::removeObservation::monitor"))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation::removeObservation::replacementValues"))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::addObservation::monitor"))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::addObservation::replacementValues"))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::observation::changeObserver"))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::observation::changeSignal"))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/observation.md") (anonymous (kind kerml-feature) (ordinal 0))))) (value (kind unresolved-operand)))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/observation.md") (range (start 7 16) (end 7 37)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/observation.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/observation.md") (range (start 8 16) (end 8 39)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/observation.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "Occurrences::Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/observation.md") (range (start 9 16) (end 9 33)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/observation.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "Occurrences::Life")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/observation.md") (range (start 10 16) (end 10 44)) (probe (position 10 16))
    (reference (id (source (node (document "memory://snapshot/observation.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::including")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/observation.md") (range (start 11 16) (end 11 44)) (probe (position 11 16))
    (reference (id (source (node (document "memory://snapshot/observation.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::excluding")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/observation.md") (range (start 12 16) (end 12 40)) (probe (position 12 16))
    (reference (id (source (node (document "memory://snapshot/observation.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::select")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/observation.md") (range (start 13 16) (end 13 56)) (probe (position 13 16))
    (reference (id (source (node (document "memory://snapshot/observation.md") (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0) (authored-target "ControlPerformances::DecisionPerformance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/observation.md") (range (start 14 16) (end 14 54)) (probe (position 14 16))
    (reference (id (source (node (document "memory://snapshot/observation.md") (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0) (authored-target "ControlPerformances::IfThenPerformance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/observation.md") (range (start 15 16) (end 15 71)) (probe (position 15 16))
    (reference (id (source (node (document "memory://snapshot/observation.md") (anonymous (kind import) (ordinal 8))))) (kind membershipImport) (ordinal 0) (authored-target "FeatureReferencingPerformances::FeatureWritePerformance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/observation.md") (range (start 16 16) (end 16 91)) (probe (position 16 16))
    (reference (id (source (node (document "memory://snapshot/observation.md") (anonymous (kind import) (ordinal 9))))) (kind membershipImport) (ordinal 0) (authored-target "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/observation.md") (range (start 17 16) (end 17 41)) (probe (position 17 16))
    (reference (id (source (node (document "memory://snapshot/observation.md") (anonymous (kind import) (ordinal 10))))) (kind membershipImport) (ordinal 0) (authored-target "Transfers::TransferBefore")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/observation.md") (range (start 109 50) (end 109 73)) (probe (position 109 50))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::AssignObservations"))) (kind specialization) (ordinal 0) (authored-target "FeatureWritePerformance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/observation.md") (range (start 150 40) (end 150 53)) (probe (position 150 40))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation::observations"))) (kind featureTyping) (ordinal 0) (authored-target "ObserveChange")
      (outcome (status resolved) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ObserveChange")))))
  )
  (query (document "memory://snapshot/observation.md") (range (start 151 4) (end 151 16)) (probe (position 151 4))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation::observations"))) (kind expressionOperand) (ordinal 0) (authored-target "observations")
      (outcome (status resolved) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation::observations")))))
  )
  (query (document "memory://snapshot/observation.md") (range (start 148 17) (end 148 27)) (probe (position 148 17))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation::observer"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/observation.md") (range (start 154 46) (end 154 64)) (probe (position 154 46))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation::removeObservation"))) (kind featureTyping) (ordinal 0) (authored-target "AssignObservations")
      (outcome (status resolved) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::AssignObservations")))))
  )
  (query (document "memory://snapshot/observation.md") (range (start 155 17) (end 155 28)) (probe (position 155 17))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation::removeObservation::monitor"))) (kind expressionOperand) (ordinal 0) (authored-target "thisMonitor")
      (outcome (status resolved) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::thisMonitor")))))
  )
  (query (document "memory://snapshot/observation.md") (range (start 156 30) (end 156 42)) (probe (position 156 30))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation::removeObservation::replacementValues"))) (kind expressionOperand) (ordinal 0) (authored-target "observations")
      (outcome (status resolved) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation::observations")))))
  )
  (query (document "memory://snapshot/observation.md") (range (start 156 54) (end 156 66)) (probe (position 156 54))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation::removeObservation::replacementValues"))) (kind expressionOperand) (ordinal 1) (authored-target "observations")
      (outcome (status resolved) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation::observations")))))
  )
  (query (document "memory://snapshot/observation.md") (range (start 149 15) (end 149 27)) (probe (position 149 15))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::cancelObservation::signal"))) (kind featureTyping) (ordinal 0) (authored-target "ChangeSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeSignal")))))
  )
  (query (document "memory://snapshot/observation.md") (range (start 107 49) (end 107 62)) (probe (position 107 49))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::observations"))) (kind featureTyping) (ordinal 0) (authored-target "ObserveChange")
      (outcome (status resolved) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ObserveChange")))))
  )
  (query (document "memory://snapshot/observation.md") (range (start 136 43) (end 136 61)) (probe (position 136 43))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::addObservation"))) (kind featureTyping) (ordinal 0) (authored-target "AssignObservations")
      (outcome (status resolved) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::AssignObservations")))))
  )
  (query (document "memory://snapshot/observation.md") (range (start 137 17) (end 137 28)) (probe (position 137 17))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::addObservation::monitor"))) (kind expressionOperand) (ordinal 0) (authored-target "thisMonitor")
      (outcome (status resolved) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::thisMonitor")))))
  )
  (query (document "memory://snapshot/observation.md") (range (start 138 30) (end 138 42)) (probe (position 138 30))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::addObservation::replacementValues"))) (kind expressionOperand) (ordinal 0) (authored-target "observations")
      (outcome (status resolved) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::observations")))))
  )
  (query (document "memory://snapshot/observation.md") (range (start 138 54) (end 138 65)) (probe (position 138 54))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::addObservation::replacementValues"))) (kind expressionOperand) (ordinal 1) (authored-target "observation")
      (outcome (status resolved) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::observation")))))
  )
  (query (document "memory://snapshot/observation.md") (range (start 132 40) (end 132 53)) (probe (position 132 40))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::observation"))) (kind featureTyping) (ordinal 0) (authored-target "ObserveChange")
      (outcome (status resolved) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ObserveChange")))))
  )
  (query (document "memory://snapshot/observation.md") (range (start 133 24) (end 133 32)) (probe (position 133 24))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::observation::changeObserver"))) (kind expressionOperand) (ordinal 0) (authored-target "observer")
      (outcome (status resolved) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::observer")))))
  )
  (query (document "memory://snapshot/observation.md") (range (start 134 22) (end 134 28)) (probe (position 134 22))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::observation::changeSignal"))) (kind expressionOperand) (ordinal 0) (authored-target "signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::signal")))))
  )
  (query (document "memory://snapshot/observation.md") (range (start 130 17) (end 130 27)) (probe (position 130 17))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::observer"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/observation.md") (range (start 131 15) (end 131 27)) (probe (position 131 15))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::startObservation::signal"))) (kind featureTyping) (ordinal 0) (authored-target "ChangeSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeSignal")))))
  )
  (query (document "memory://snapshot/observation.md") (range (start 106 24) (end 106 37)) (probe (position 106 24))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::thisMonitor"))) (kind featureTyping) (ordinal 0) (authored-target "ChangeMonitor")
      (outcome (status resolved) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor")))))
  )
  (query (document "memory://snapshot/observation.md") (range (start 106 48) (end 106 52)) (probe (position 106 48))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor::thisMonitor"))) (kind redefinition) (ordinal 0) (authored-target "self")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/observation.md") (range (start 47 26) (end 47 39)) (probe (position 47 26))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeSignal::signalMonitor"))) (kind featureTyping) (ordinal 0) (authored-target "ChangeMonitor")
      (outcome (status resolved) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor")))))
  )
  (query (document "memory://snapshot/observation.md") (range (start 19 44) (end 19 57)) (probe (position 19 44))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::DefaultMonitorLife"))) (kind specialization) (ordinal 0) (authored-target "ChangeMonitor")
      (outcome (status resolved) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ChangeMonitor")))))
  )
  (query (document "memory://snapshot/observation.md") (range (start 19 59) (end 19 63)) (probe (position 19 59))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::DefaultMonitorLife"))) (kind specialization) (ordinal 1) (authored-target "Life")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/observation.md") (range (start 83 21) (end 83 35)) (probe (position 83 21))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ObserveChange::transfer"))) (kind featureTyping) (ordinal 0) (authored-target "TransferBefore")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/observation.md") (range (start 85 14) (end 85 46)) (probe (position 85 14))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ObserveChange::transfer"))) (kind subsetting) (ordinal 0) (authored-target "changeObserver::incomingTransfers")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/observation.md") (range (start 84 16) (end 84 41)) (probe (position 84 16))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ObserveChange::transfer"))) (kind redefinition) (ordinal 0) (authored-target "outgoingTransfersFromSelf")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/observation.md") (range (start 92 25) (end 92 37)) (probe (position 92 25))
    (reference (id (source (node (document "memory://snapshot/observation.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "sourceOutput")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/observation.md") (range (start 92 40) (end 92 52)) (probe (position 92 40))
    (reference (id (source (node (document "memory://snapshot/observation.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "changeSignal")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/observation.md") (range (start 66 24) (end 66 41)) (probe (position 66 24))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::ObserveChange::wait"))) (kind featureTyping) (ordinal 0) (authored-target "IfThenPerformance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/observation.md") (range (start 26 29) (end 26 47)) (probe (position 26 29))
    (reference (id (source (node (document "memory://snapshot/observation.md") (qualified-name "Observation::defaultMonitor"))) (kind featureTyping) (ordinal 0) (authored-target "DefaultMonitorLife")
      (outcome (status resolved) (target (node (document "memory://snapshot/observation.md") (qualified-name "Observation::DefaultMonitorLife")))))
  )
)
~~~
