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
# EXPECTED
~~~
semantic.unresolved_name 'Life'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'IfThenPerformance'
semantic.unresolved_name 'ifTest'
semantic.unresolved_name 'thenClause'
semantic.unresolved_name 'BooleanEvaluationResultToMonitorPerformance'
semantic.unresolved_name 'TransferBefore'
semantic.unresolved_name 'outgoingTransfersFromSelf'
semantic.unresolved_name 'changeObserver::incomingTransfers'
semantic.unresolved_name 'sourceOutput'
semantic.unresolved_name 'self'
semantic.unresolved_name 'FeatureWritePerformance'
semantic.unresolved_name 'onOccurrence'
semantic.unresolved_name 'startingAt'
semantic.unresolved_name 'accessedFeature'
semantic.unresolved_name 'replacementValues'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Occurrence'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Life'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'IfThenPerformance'
semantic.unresolved_name 'ifTest'
semantic.unresolved_name 'thenClause'
semantic.unresolved_name 'BooleanEvaluationResultToMonitorPerformance'
semantic.unresolved_name 'TransferBefore'
semantic.unresolved_name 'outgoingTransfersFromSelf'
semantic.unresolved_name 'changeObserver::incomingTransfers'
semantic.unresolved_name 'sourceOutput'
semantic.unresolved_name 'self'
semantic.unresolved_name 'FeatureWritePerformance'
semantic.unresolved_name 'onOccurrence'
semantic.unresolved_name 'startingAt'
semantic.unresolved_name 'accessedFeature'
semantic.unresolved_name 'replacementValues'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Occurrence'
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
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwStruct,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwFeature,Ident,OpenSquare,DecimalValue,CloseSquare,Colon,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwStruct,Ident,OpenCurly,
KwDoc,
RegularComment,
KwBool,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwFeature,Ident,Colon,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,
KwPrivate,KwBehavior,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwComposite,KwStep,Ident,Colon,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,KwBool,KwRedefines,Ident,OpenCurly,
KwNot,Ident,Dot,Ident,OpenParen,CloseParen,
CloseCurly,
KwIn,KwStep,KwRedefines,Ident,Colon,Ident,OpenCurly,
KwIn,KwBool,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwSuccession,Ident,KwThen,Ident,Semicolon,
KwStep,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,
KwRedefines,Ident,
KwSubsets,Ident,Dot,Ident,OpenCurly,
KwDoc,
RegularComment,
KwEnd,KwFeature,Ident,OpenCurly,
KwFeature,KwRedefines,Ident,Eq,Ident,Semicolon,
CloseCurly,
KwEnd,KwFeature,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwStruct,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,Ident,Colon,Ident,KwRedefines,Ident,Semicolon,
KwPrivate,KwComposite,KwFeature,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Colon,Ident,Semicolon,
KwPrivate,KwBehavior,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,OpenCurly,
KwFeature,KwRedefines,Ident,OpenCurly,
KwFeature,KwRedefines,Ident,Comma,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwInout,KwFeature,KwRedefines,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Colon,Ident,Semicolon,
CloseCurly,
KwStep,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPrivate,KwComposite,KwStep,Ident,Colon,Ident,OpenCurly,
KwIn,Ident,Eq,Ident,Semicolon,
KwIn,Ident,Eq,Ident,Semicolon,
CloseCurly,
KwPrivate,KwComposite,KwStep,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwIn,Ident,Eq,Ident,Semicolon,
KwInout,Ident,Eq,Ident,Arrow,Ident,OpenParen,Ident,CloseParen,Semicolon,
CloseCurly,
CloseCurly,
KwStep,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPrivate,KwFeature,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Colon,Ident,Eq,
Ident,Arrow,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,
Ident,Dot,Ident,EqEq,Ident,KwAnd,Ident,Dot,Ident,EqEq,Ident,
CloseCurly,Semicolon,
KwPrivate,KwComposite,KwStep,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwIn,Ident,Eq,Ident,Semicolon,
KwInout,Ident,Eq,Ident,Arrow,Ident,OpenParen,Ident,CloseParen,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'Observation'
    (documentation)
    (import_decl private 'ScalarValues::Boolean')
    (import_decl private 'Occurrences::Occurrence')
    (import_decl private 'Occurrences::Life')
    (import_decl private 'SequenceFunctions::including')
    (import_decl private 'SequenceFunctions::excluding')
    (import_decl private 'ControlFunctions::select')
    (import_decl private 'ControlPerformances::DecisionPerformance')
    (import_decl private 'ControlPerformances::IfThenPerformance')
    (import_decl private 'FeatureReferencingPerformances::FeatureWritePerformance')
    (import_decl private 'FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance')
    (import_decl private 'Transfers::TransferBefore')
    (structure_def private 'DefaultMonitorLife' multiplicity     (multiplicity_range) :> 'ChangeMonitor', 'Life'
      (documentation))
    (feature_def 'defaultMonitor' multiplicity : 'DefaultMonitorLife'
      (documentation))
    (structure_def 'ChangeSignal'
      (documentation)
      (boolean_expr_def
        (documentation))
      (feature_def 'signalMonitor' : 'ChangeMonitor'
        (documentation)))
    (behavior_def
      (documentation)
      (feature_def in 'changeObserver' : 'Occurrence' multiplicity)
      (feature_def in 'changeSignal' : 'ChangeSignal' multiplicity)
      (step_def
        (documentation)
        (boolean_expr_usage
          (result_expr_member))
        (step_def
          (boolean_expr_usage)))
      (succession_def
        (connector_end)
        (connector_end))
      (step_def
        (documentation)
        (feature_def end 'source'
          (feature_def :>> 'sourceOutput' value))
        (feature_def end 'target')))
    (structure_def 'ChangeMonitor'
      (documentation)
      (feature_def private 'thisMonitor' : 'ChangeMonitor' :>> 'self')
      (feature_def private composite 'observations' multiplicity : 'ObserveChange')
      (behavior_def
        (documentation)
        (feature_def in 'monitor' : 'ChangeMonitor' :>> 'onOccurrence'
          (feature_def :>> 'startingAt'
            (feature_def :>> 'accessedFeature', 'observations')))
        (feature_def inout :>> 'replacementValues' multiplicity : 'ObserveChange'))
      (step_def
        (documentation)
        (feature_def in 'observer' : 'Occurrence' multiplicity)
        (feature_def in 'signal' : 'ChangeSignal' multiplicity)
        (step_def
          (feature_def in 'changeObserver' value)
          (feature_def in 'changeSignal' value))
        (step_def
          (feature_def in 'monitor' value)
          (feature_def inout 'replacementValues' value)))
      (step_def
        (documentation)
        (feature_def in 'observer' : 'Occurrence' multiplicity)
        (feature_def in 'signal' : 'ChangeSignal' multiplicity)
        (feature_def private 'observations' multiplicity : 'ObserveChange' value)
        (step_def
          (feature_def in 'monitor' value)
          (feature_def inout 'replacementValues' value))))))
~~~
# FORMAT
~~~sysml
standard library package Observation {
    doc /*
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
        doc /*
         * DefaultMonitorLife is the classifier of the singleton Life of the defaultMonitor.
         */
    }

    feature defaultMonitor[1] : DefaultMonitorLife {
        doc /*
		 * defaultMonitor is a single ChangeMonitor that can be used as a default.
		 */
    }

    struct ChangeSignal {
        doc /*
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
            doc /*
			 * The ChangeMonitor responsible for monitoring the signalCondition.
			 */
        }
    }

    private behavior ObserveChange {
        doc /*
		 * Each Performance of ObserveChange waits for the result of the Boolean 
		 * condition of a given ChangeSignal to change from false to true, and, when 
		 * it does, sends the ChangeSignal to a given observer Occurrence.
		 */

        in feature changeObserver : Occurrence [1];
        in feature changeSignal : ChangeSignal [1];

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
        doc /*
		 * A ChangeMonitor is a collection of ongoing ChangeSignal observations 
		 * for various observer Occurrences. It provides convenient operations for 
		 * starting and canceling the observations it manages.
		 */

        private thisMonitor: ChangeMonitor redefines self;
        private composite feature observations[0..*] : ObserveChange;

        private behavior AssignObservations specializes FeatureWritePerformance {
            doc /*
			 * Assign a replacement set of observations as those being managed by a
			 * given ChangeMonitor.
			 */

            in feature monitor : ChangeMonitor redefines onOccurrence {
                feature redefines startingAt {
                    feature redefines accessedFeature, observations;
                }
            }
            inout feature redefines replacementValues [0..*] : ObserveChange;
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
(model
  (namespace
    (library_package 'Observation'
      (documentation)
      (membership_import private -> 'ScalarValues::Boolean'[unresolved])
      (membership_import private -> 'Occurrences::Occurrence'[unresolved])
      (membership_import private -> 'Occurrences::Life'[unresolved])
      (membership_import private -> 'SequenceFunctions::including'[unresolved])
      (membership_import private -> 'SequenceFunctions::excluding'[unresolved])
      (membership_import private -> 'ControlFunctions::select'[unresolved])
      (membership_import private -> 'ControlPerformances::DecisionPerformance'[unresolved])
      (membership_import private -> 'ControlPerformances::IfThenPerformance'[unresolved])
      (membership_import private -> 'FeatureReferencingPerformances::FeatureWritePerformance'[unresolved])
      (membership_import private -> 'FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance'[unresolved])
      (membership_import private -> 'Transfers::TransferBefore'[unresolved])
      (structure_def 'DefaultMonitorLife' :> 'Observation::ChangeMonitor'[structure_def] :> 'Life'[unresolved]
        (multiplicity_range [1])
        (documentation))
      (feature_def 'defaultMonitor' : 'Observation::DefaultMonitorLife'[structure_def]
        (multiplicity_range [1])
        (documentation))
      (structure_def 'ChangeSignal'
        (documentation)
        (boolean_expr_def 'signalCondition'
          (documentation))
        (feature_def 'signalMonitor' : 'Observation::ChangeMonitor'[structure_def]
          (documentation)))
      (behavior_def 'ObserveChange'
        (documentation)
        (feature_def in 'changeObserver' : 'Occurrence'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'changeSignal' : 'Observation::ChangeSignal'[structure_def]
          (multiplicity_range [1]))
        (step_def composite 'wait' : 'IfThenPerformance'[unresolved]
          (documentation)
          (boolean_expr_usage in :>> 'ifTest'[unresolved]
            (result_expr_membership))
          (step_def in :>> 'thenClause'[unresolved] : 'BooleanEvaluationResultToMonitorPerformance'[unresolved]
            (boolean_expr_usage in 'onOccurrence'
              (feature_value (=)))))
        (succession_def
          (connector_end 'wait')
          (connector_end 'transfer'))
        (step_def 'transfer' : 'TransferBefore'[unresolved] :>> 'outgoingTransfersFromSelf'[unresolved] :> 'changeObserver::incomingTransfers'[unresolved]
          (multiplicity_range [1])
          (documentation)
          (feature_def end 'source'
            (feature_def :>> 'sourceOutput'[unresolved]
              (feature_value (=))))
          (feature_def end 'target')))
      (structure_def 'ChangeMonitor'
        (documentation)
        (feature_def 'thisMonitor' : 'Observation::ChangeMonitor'[structure_def] :>> 'self'[unresolved])
        (feature_def composite 'observations' : 'Observation::ObserveChange'[behavior_def]
          (multiplicity_range [0..*]))
        (behavior_def 'AssignObservations' :> 'FeatureWritePerformance'[unresolved]
          (documentation)
          (feature_def in 'monitor' : 'Observation::ChangeMonitor'[structure_def] :>> 'onOccurrence'[unresolved]
            (feature_def :>> 'startingAt'[unresolved]
              (feature_def :>> 'accessedFeature'[unresolved] :>> 'Observation::ChangeMonitor::observations'[feature_def])))
          (feature_def inout :>> 'replacementValues'[unresolved] : 'Observation::ObserveChange'[behavior_def]
            (multiplicity_range [0..*])))
        (step_def 'startObservation'
          (documentation)
          (feature_def in 'observer' : 'Occurrence'[unresolved]
            (multiplicity_range [1]))
          (feature_def in 'signal' : 'Observation::ChangeSignal'[structure_def]
            (multiplicity_range [1]))
          (step_def composite 'observation' : 'Observation::ObserveChange'[behavior_def]
            (feature_def in 'changeObserver'
              (feature_value (=)))
            (feature_def in 'changeSignal'
              (feature_value (=))))
          (step_def composite 'addObservation' : 'Observation::ChangeMonitor::AssignObservations'[behavior_def]
            (multiplicity_range [1])
            (feature_def in 'monitor'
              (feature_value (=)))
            (feature_def inout 'replacementValues'
              (feature_value (=)))))
        (step_def 'cancelObservation'
          (documentation)
          (feature_def in 'observer' : 'Occurrence'[unresolved]
            (multiplicity_range [1]))
          (feature_def in 'signal' : 'Observation::ChangeSignal'[structure_def]
            (multiplicity_range [1]))
          (feature_def 'observations' : 'Observation::ObserveChange'[behavior_def]
            (multiplicity_range [0..*])
            (feature_value (=)))
          (step_def composite 'removeObservation' : 'Observation::ChangeMonitor::AssignObservations'[behavior_def]
            (multiplicity_range [1])
            (feature_def in 'monitor'
              (feature_value (=)))
            (feature_def inout 'replacementValues'
              (feature_value (=)))))))))
~~~
