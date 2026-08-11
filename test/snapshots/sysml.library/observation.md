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
    (element (id (node (document "d0") (qualified-name "Observation"))) (kind "package") (name "Observation") (declared-name "Observation"))
    (element (id (node (document "d0") (qualified-name "Observation::Boolean"))) (kind "import") (name "Boolean") (declared-name "Boolean") (parent (node (document "d0") (qualified-name "Observation"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Boolean") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Observation::BooleanEvaluationResultToMonitorPerformance"))) (kind "import") (name "BooleanEvaluationResultToMonitorPerformance") (declared-name "BooleanEvaluationResultToMonitorPerformance") (parent (node (document "d0") (qualified-name "Observation"))) (authored (membership (kind Import) (visibility "private") (import (reference "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Observation::ChangeMonitor"))) (kind "classifier decl") (name "ChangeMonitor") (declared-name "ChangeMonitor") (parent (node (document "d0") (qualified-name "Observation"))))
    (element (id (node (document "d0") (qualified-name "Observation::ChangeSignal"))) (kind "classifier decl") (name "ChangeSignal") (declared-name "ChangeSignal") (parent (node (document "d0") (qualified-name "Observation"))))
    (element (id (node (document "d0") (qualified-name "Observation::DecisionPerformance"))) (kind "import") (name "DecisionPerformance") (declared-name "DecisionPerformance") (parent (node (document "d0") (qualified-name "Observation"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlPerformances::DecisionPerformance") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Observation::DefaultMonitorLife1"))) (kind "classifier decl") (name "DefaultMonitorLife1") (declared-name "DefaultMonitorLife1") (parent (node (document "d0") (qualified-name "Observation"))))
    (element (id (node (document "d0") (qualified-name "Observation::FeatureWritePerformance"))) (kind "import") (name "FeatureWritePerformance") (declared-name "FeatureWritePerformance") (parent (node (document "d0") (qualified-name "Observation"))) (authored (membership (kind Import) (visibility "private") (import (reference "FeatureReferencingPerformances::FeatureWritePerformance") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Observation::IfThenPerformance"))) (kind "import") (name "IfThenPerformance") (declared-name "IfThenPerformance") (parent (node (document "d0") (qualified-name "Observation"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlPerformances::IfThenPerformance") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Observation::Life"))) (kind "import") (name "Life") (declared-name "Life") (parent (node (document "d0") (qualified-name "Observation"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::Life") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Observation::ObserveChange"))) (kind "kermlDecl") (name "ObserveChange") (declared-name "ObserveChange") (parent (node (document "d0") (qualified-name "Observation"))))
    (element (id (node (document "d0") (qualified-name "Observation::Occurrence"))) (kind "import") (name "Occurrence") (declared-name "Occurrence") (parent (node (document "d0") (qualified-name "Observation"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::Occurrence") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Observation::TransferBefore"))) (kind "import") (name "TransferBefore") (declared-name "TransferBefore") (parent (node (document "d0") (qualified-name "Observation"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::TransferBefore") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Observation::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Observation"))))
    (element (id (node (document "d0") (qualified-name "Observation::defaultMonitor1"))) (kind "feature decl") (name "defaultMonitor1") (declared-name "defaultMonitor1") (parent (node (document "d0") (qualified-name "Observation"))))
    (element (id (node (document "d0") (qualified-name "Observation::excluding"))) (kind "import") (name "excluding") (declared-name "excluding") (parent (node (document "d0") (qualified-name "Observation"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::excluding") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Observation::including"))) (kind "import") (name "including") (declared-name "including") (parent (node (document "d0") (qualified-name "Observation"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::including") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Observation::select"))) (kind "import") (name "select") (declared-name "select") (parent (node (document "d0") (qualified-name "Observation"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::select") (origin Import) (shape Membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Observation::Boolean"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Boolean") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Observation::BooleanEvaluationResultToMonitorPerformance"))) (kind membershipImport) (ordinal 0)) (authored-target "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Observation::DecisionPerformance"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlPerformances::DecisionPerformance") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Observation::FeatureWritePerformance"))) (kind membershipImport) (ordinal 0)) (authored-target "FeatureReferencingPerformances::FeatureWritePerformance") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Observation::IfThenPerformance"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlPerformances::IfThenPerformance") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Observation::Life"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::Life") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Observation::Occurrence"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::Occurrence") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Observation::TransferBefore"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::TransferBefore") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Observation::excluding"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::excluding") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Observation::including"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::including") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Observation::select"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlFunctions::select") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 9 16) (end 9 33)) (probe (position 9 16))
      (reference
        (source (document "d0") (qualified-name "Observation::Life"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::Life")
        (range (start 9 16) (end 9 33))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 16) (end 7 37)) (probe (position 7 16))
      (reference
        (source (document "d0") (qualified-name "Observation::Boolean"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Boolean")
        (range (start 7 16) (end 7 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 16) (end 8 39)) (probe (position 8 16))
      (reference
        (source (document "d0") (qualified-name "Observation::Occurrence"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::Occurrence")
        (range (start 8 16) (end 8 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 12 16) (end 12 40)) (probe (position 12 16))
      (reference
        (source (document "d0") (qualified-name "Observation::select"))
        (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::select")
        (range (start 12 16) (end 12 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 17 16) (end 17 41)) (probe (position 17 16))
      (reference
        (source (document "d0") (qualified-name "Observation::TransferBefore"))
        (kind membershipImport) (ordinal 0) (authored-target "Transfers::TransferBefore")
        (range (start 17 16) (end 17 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 16) (end 10 44)) (probe (position 10 16))
      (reference
        (source (document "d0") (qualified-name "Observation::including"))
        (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::including")
        (range (start 10 16) (end 10 44))
        (outcome (status unresolved))
      )
    )
    (query (range (start 11 16) (end 11 44)) (probe (position 11 16))
      (reference
        (source (document "d0") (qualified-name "Observation::excluding"))
        (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::excluding")
        (range (start 11 16) (end 11 44))
        (outcome (status unresolved))
      )
    )
    (query (range (start 14 16) (end 14 54)) (probe (position 14 16))
      (reference
        (source (document "d0") (qualified-name "Observation::IfThenPerformance"))
        (kind membershipImport) (ordinal 0) (authored-target "ControlPerformances::IfThenPerformance")
        (range (start 14 16) (end 14 54))
        (outcome (status unresolved))
      )
    )
    (query (range (start 13 16) (end 13 56)) (probe (position 13 16))
      (reference
        (source (document "d0") (qualified-name "Observation::DecisionPerformance"))
        (kind membershipImport) (ordinal 0) (authored-target "ControlPerformances::DecisionPerformance")
        (range (start 13 16) (end 13 56))
        (outcome (status unresolved))
      )
    )
    (query (range (start 15 16) (end 15 71)) (probe (position 15 16))
      (reference
        (source (document "d0") (qualified-name "Observation::FeatureWritePerformance"))
        (kind membershipImport) (ordinal 0) (authored-target "FeatureReferencingPerformances::FeatureWritePerformance")
        (range (start 15 16) (end 15 71))
        (outcome (status unresolved))
      )
    )
    (query (range (start 16 16) (end 16 91)) (probe (position 16 16))
      (reference
        (source (document "d0") (qualified-name "Observation::BooleanEvaluationResultToMonitorPerformance"))
        (kind membershipImport) (ordinal 0) (authored-target "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance")
        (range (start 16 16) (end 16 91))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
