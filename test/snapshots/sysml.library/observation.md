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
  (document "observation.md"
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
    )
  )
)
~~~
# FORMAT
~~~sysml
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "77fccaf206d36eabcd5662d79670280d2b15d1b5947134861c5878e801f51538") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Observation"))) (kind "package") (name "Observation") (declared-name "Observation") (range (start (line 0) (character 0)) (end (line 0) (character 4578))))
    (element (id (node (document "d0") (qualified-name "Observation::Boolean"))) (kind "import") (name "Boolean") (declared-name "Boolean") (range (start (line 7) (character 1)) (end (line 7) (character 38))) (parent (node (document "d0") (qualified-name "Observation"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Boolean") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 16)) (end (line 7) (character 37))))))
    (element (id (node (document "d0") (qualified-name "Observation::BooleanEvaluationResultToMonitorPerformance"))) (kind "import") (name "BooleanEvaluationResultToMonitorPerformance") (declared-name "BooleanEvaluationResultToMonitorPerformance") (range (start (line 16) (character 1)) (end (line 16) (character 92))) (parent (node (document "d0") (qualified-name "Observation"))) (authored (membership (kind Import) (visibility "private") (import (reference "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 16) (character 16)) (end (line 16) (character 91))))))
    (element (id (node (document "d0") (qualified-name "Observation::ChangeMonitor"))) (kind "classifier decl") (name "ChangeMonitor") (declared-name "ChangeMonitor") (range (start (line 98) (character 1)) (end (line 98) (character 1881))) (parent (node (document "d0") (qualified-name "Observation"))))
    (element (id (node (document "d0") (qualified-name "Observation::ChangeSignal"))) (kind "classifier decl") (name "ChangeSignal") (declared-name "ChangeSignal") (range (start (line 33) (character 1)) (end (line 33) (character 426))) (parent (node (document "d0") (qualified-name "Observation"))))
    (element (id (node (document "d0") (qualified-name "Observation::DecisionPerformance"))) (kind "import") (name "DecisionPerformance") (declared-name "DecisionPerformance") (range (start (line 13) (character 1)) (end (line 13) (character 57))) (parent (node (document "d0") (qualified-name "Observation"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlPerformances::DecisionPerformance") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 13) (character 16)) (end (line 13) (character 56))))))
    (element (id (node (document "d0") (qualified-name "Observation::DefaultMonitorLife1"))) (kind "classifier decl") (name "DefaultMonitorLife1") (declared-name "DefaultMonitorLife1") (range (start (line 19) (character 4)) (end (line 19) (character 199))) (parent (node (document "d0") (qualified-name "Observation"))))
    (element (id (node (document "d0") (qualified-name "Observation::FeatureWritePerformance"))) (kind "import") (name "FeatureWritePerformance") (declared-name "FeatureWritePerformance") (range (start (line 15) (character 1)) (end (line 15) (character 72))) (parent (node (document "d0") (qualified-name "Observation"))) (authored (membership (kind Import) (visibility "private") (import (reference "FeatureReferencingPerformances::FeatureWritePerformance") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 15) (character 16)) (end (line 15) (character 71))))))
    (element (id (node (document "d0") (qualified-name "Observation::IfThenPerformance"))) (kind "import") (name "IfThenPerformance") (declared-name "IfThenPerformance") (range (start (line 14) (character 1)) (end (line 14) (character 55))) (parent (node (document "d0") (qualified-name "Observation"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlPerformances::IfThenPerformance") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 14) (character 16)) (end (line 14) (character 54))))))
    (element (id (node (document "d0") (qualified-name "Observation::Life"))) (kind "import") (name "Life") (declared-name "Life") (range (start (line 9) (character 1)) (end (line 9) (character 34))) (parent (node (document "d0") (qualified-name "Observation"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::Life") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 9) (character 16)) (end (line 9) (character 33))))))
    (element (id (node (document "d0") (qualified-name "Observation::ObserveChange"))) (kind "kermlDecl") (name "ObserveChange") (declared-name "ObserveChange") (range (start (line 55) (character 1)) (end (line 55) (character 1134))) (parent (node (document "d0") (qualified-name "Observation"))))
    (element (id (node (document "d0") (qualified-name "Observation::Occurrence"))) (kind "import") (name "Occurrence") (declared-name "Occurrence") (range (start (line 8) (character 1)) (end (line 8) (character 40))) (parent (node (document "d0") (qualified-name "Observation"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::Occurrence") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 8) (character 16)) (end (line 8) (character 39))))))
    (element (id (node (document "d0") (qualified-name "Observation::TransferBefore"))) (kind "import") (name "TransferBefore") (declared-name "TransferBefore") (range (start (line 17) (character 1)) (end (line 17) (character 42))) (parent (node (document "d0") (qualified-name "Observation"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::TransferBefore") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 17) (character 16)) (end (line 17) (character 41))))))
    (element (id (node (document "d0") (qualified-name "Observation::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 4578))) (parent (node (document "d0") (qualified-name "Observation"))))
    (element (id (node (document "d0") (qualified-name "Observation::defaultMonitor1"))) (kind "feature decl") (name "defaultMonitor1") (declared-name "defaultMonitor1") (range (start (line 26) (character 1)) (end (line 26) (character 146))) (parent (node (document "d0") (qualified-name "Observation"))))
    (element (id (node (document "d0") (qualified-name "Observation::excluding"))) (kind "import") (name "excluding") (declared-name "excluding") (range (start (line 11) (character 1)) (end (line 11) (character 45))) (parent (node (document "d0") (qualified-name "Observation"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::excluding") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 11) (character 16)) (end (line 11) (character 44))))))
    (element (id (node (document "d0") (qualified-name "Observation::including"))) (kind "import") (name "including") (declared-name "including") (range (start (line 10) (character 1)) (end (line 10) (character 45))) (parent (node (document "d0") (qualified-name "Observation"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::including") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 10) (character 16)) (end (line 10) (character 44))))))
    (element (id (node (document "d0") (qualified-name "Observation::select"))) (kind "import") (name "select") (declared-name "select") (range (start (line 12) (character 1)) (end (line 12) (character 41))) (parent (node (document "d0") (qualified-name "Observation"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::select") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 12) (character 16)) (end (line 12) (character 40))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Observation::Boolean"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Boolean") (range (start (line 7) (character 16)) (end (line 7) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Observation::BooleanEvaluationResultToMonitorPerformance"))) (kind membershipImport) (ordinal 0)) (authored-target "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance") (range (start (line 16) (character 16)) (end (line 16) (character 91))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Observation::DecisionPerformance"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlPerformances::DecisionPerformance") (range (start (line 13) (character 16)) (end (line 13) (character 56))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Observation::FeatureWritePerformance"))) (kind membershipImport) (ordinal 0)) (authored-target "FeatureReferencingPerformances::FeatureWritePerformance") (range (start (line 15) (character 16)) (end (line 15) (character 71))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Observation::IfThenPerformance"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlPerformances::IfThenPerformance") (range (start (line 14) (character 16)) (end (line 14) (character 54))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Observation::Life"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::Life") (range (start (line 9) (character 16)) (end (line 9) (character 33))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Observation::Occurrence"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::Occurrence") (range (start (line 8) (character 16)) (end (line 8) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Observation::TransferBefore"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::TransferBefore") (range (start (line 17) (character 16)) (end (line 17) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Observation::excluding"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::excluding") (range (start (line 11) (character 16)) (end (line 11) (character 44))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Observation::including"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::including") (range (start (line 10) (character 16)) (end (line 10) (character 44))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Observation::select"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlFunctions::select") (range (start (line 12) (character 16)) (end (line 12) (character 40))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
