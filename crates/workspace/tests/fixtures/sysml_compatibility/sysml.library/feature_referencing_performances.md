# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Semantic Library/FeatureReferencingPerformances
type=file
~~~
# SOURCE
~~~kerml
standard library package FeatureReferencingPerformances {
	doc
	/*
	 * This package defines Behaviors used to read, write and monitor values of a referenced Feature of an 
	 * Occurrence.
	 */

	private import Base::Anything;
	private import Base::things;
	private import Occurrences::Occurrence;
	private import Occurrences::HappensWhile;
	private import Occurrences::HappensBefore;
	private import Occurrences::HappensJustBefore;
	private import Occurrences::SelfSameLifeLink;
	private import Performances::Performance;
	private import Performances::Evaluation;
	private import ScalarValues::Boolean;
	private import SequenceFunctions::isEmpty;
	private import SequenceFunctions::equals;

	abstract behavior FeatureReferencingPerformance specializes Performance {
		doc
		/*
		 * A FeatureReferencingPerformance is the base Performance for specialized behaviors related 
		 * to values of a referenced Feature of a given Occurrence, as identified in specializations 
		 * of this Behavior.
		 */
		
		in abstract feature onOccurrence : Occurrence [1] {
			doc
			/*
			 * Occurrence with values for the referenced feature in specializations of this behavior.
			 */
		}
		 
		inout feature values : Anything [*] nonunique {
			doc
			/*
			 * Values of the referenced feature, as specified in specializations of this behavior.
			 */
		}	
	}

    abstract behavior FeatureAccessPerformance specializes FeatureReferencingPerformance {
    	doc
		/*
		 * A FeatureAccessPerformance is a FeatureReferencingPerformance where the result values
		 * are all the values of a feature of onOccurrence at the time the Performance ends. The
		 * feature is specified by restricting accessedFeature in specializations or usages.
		 */
     	
		in abstract feature onOccurrence : Occurrence {
			abstract feature startingAt : Occurrence [1] subsets timeSlices {
				abstract feature accessedFeature : Anything [*] nonunique;
			}
		}

	  	 connector :HappensWhile from onOccurrence.startingAt.startShot to endShot {
	  	 	doc
			/*
			 * Requires some time slice of onOccurrence to start when this performance
			 * ends (this connector), with particular feature values (following connector).
			 * The feature is specified by restricting the onOccurrence::accessedFeature 
			 * on usages of this behavior.
			 */
	  	 }
	  	 connector :SelfSameLifeLink from onOccurrence.startingAt.accessedFeature to values;
	}

	abstract function FeatureReadEvaluation specializes FeatureAccessPerformance, Evaluation { 
		doc
		/*
		 * A FeatureReadEvaluation is a FeatureAccessPerformance that is a function providing as
		 * its result the values of a feature on an occurrence at the time its evaluation ends.
		 */

		in onOccurrence: Occurrence [1];
		return resultValues : Anything [*] nonunique redefines result redefines values;
	}

	abstract behavior FeatureWritePerformance specializes FeatureAccessPerformance {
		doc
		/*
		 * A FeatureWritePerformance is a FeatureAccessPerformance that assigns the values of a 
		 * feature on an occurrence to the given replacementValues at time its performance ends.
		 */
		 
		in feature onOccurrence : Occurrence[1] redefines onOccurrence;
		inout feature replacementValues : Anything redefines values [*] nonunique;
	}
	
	abstract behavior FeatureMonitorPerformance specializes FeatureReferencingPerformance {
		doc
		/*
		 * A FeatureMonitorPerformance is a FeatureReferencingPerformance that waits for values
		 * of monitoredFeature to change on onOccurrence from what they were when the performance 
		 * started. The values before and after the change are given by beforeValues and afterValues.
		 */	

		in feature redefines onOccurrence {
	    	feature monitoredOccurrence : Occurrence [1] subsets timeSlices {
	      		abstract feature monitoredFeature : Anything[*] nonunique;
	      		feature beforeTimeSlice : Occurrence [1] subsets timeSlices {
	        		feature redefines monitoredFeature;
	        	}
	      		feature afterSnapshot : Occurrence [1] subsets snapshots {
	        		feature redefines monitoredFeature;
	        	}
	      		connector :HappensJustBefore from beforeTimeSlice to afterSnapshot;
	        }
	  	}
	  	out feature afterValues redefines values;
	  	out feature beforeValues : Anything[*] nonunique;
	  	inv { not beforeValues->equals(afterValues) }
	
	  	private connector : HappensWhile 
	  		from [1] onOccurrence.monitoredOccurrence.beforeTimeSlice.startShot to [1] startShot;
	  	private connector : SelfSameLifeLink 
	  		from [1] onOccurrence.monitoredOccurrence.beforeTimeSlice.monitoredFeature to [1] beforeValues;
	  	private connector : SelfSameLifeLink 
	  		from [1] onOccurrence.monitoredOccurrence.afterSnapshot.monitoredFeature to [1] afterValues;
	  	protected connector endWhen : HappensBefore 
	  		from [1] onOccurrence.monitoredOccurrence.afterSnapshot to [1] endShot;
	}
		
	behavior EvaluationResultMonitorPerformance specializes FeatureMonitorPerformance {
		doc
		/*
		 * An EvaluationResultMonitorPerformance is a FeatureMonitorPerformance that waits for changes 
		 * in the result of an Evaluation identified by onOccurrence. The Predicate being evaluated must 
		 * be able to produce multiple results over time, for example by only using BindingConnectors 
		 * between Steps, rather than Successions or Flows, including in its Step behaviors.
		 */
		
		in feature onOccurrence : Evaluation redefines onOccurrence {
	    	protected expr monitoredOccurrence : Evaluation [1] redefines monitoredOccurrence {
				return result : Anything[*] redefines result, monitoredFeature; 
			} 
		} 
	}
	
	behavior BooleanEvaluationResultMonitorPerformance specializes EvaluationResultMonitorPerformance {
		doc
		/*
		 * A BooleanEvaluationResultMonitorPerformance is a EvaluationResultMonitorPerformance
		 * that waits for changes in the result of a BooleanEvaluation identified by onOccurrence.
		 */	
		
	  	in bool redefines onOccurrence {
	    	protected bool redefines monitoredOccurrence[1] {
	    	    return result : Boolean [1];
	    	}
		}
		out redefines afterValues : Boolean [1]; 
		out redefines beforeValues : Boolean [1];	 
	}
	
	behavior BooleanEvaluationResultToMonitorPerformance specializes FeatureReferencingPerformance {
		doc
		/*
		 * A BooleanEvaluationResultToMonitorPerformance is a FeatureReferencingPerformance that waits 
		 * for the result of a BooleanEvaluation (identified by onOccurrence) to change to either true 
		 * or false, as indicated by isToTrue (defaulting to true). If the result is already true (or false), 
		 * the performance waits for the result to become false (or true) before waiting again for it to 
		 * change back.
		 */	
		 
  		in bool redefines onOccurrence;
  		feature isToTrue : Boolean [1] default true;
  		out afterValues: Boolean[1] redefines values  = isToTrue;
  		private feature monitor1 : BooleanEvaluationResultMonitorPerformance [1] {
    		feature redefines endWhen : HappensJustBefore {
    		    end feature earlierOccurrence;
    		    end feature laterOccurrence;
    		}
  		}
  		private feature monitor2 : BooleanEvaluationResultMonitorPerformance [1] {
    		feature redefines endWhen : HappensJustBefore {
                end feature earlierOccurrence;
                end feature laterOccurrence;
            }
  		}

  		private connector : HappensJustBefore from [1] monitor1 to [0..1] monitor2;
  		inv { isEmpty(monitor2) == (monitor1.afterValues == isToTrue) }

  		private binding [1] monitor1.onOccurrence = [1] onOccurrence;
  		private binding [1] monitor2.onOccurrence = [1] onOccurrence;
	}
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Performance'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'timeSlices'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'HappensWhile'
semantic.unresolved_name 'SelfSameLifeLink'
semantic.unresolved_name 'Evaluation'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'result'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'timeSlices'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'timeSlices'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'snapshots'
semantic.unresolved_name 'HappensJustBefore'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'HappensWhile'
semantic.unresolved_name 'SelfSameLifeLink'
semantic.unresolved_name 'SelfSameLifeLink'
semantic.unresolved_name 'HappensBefore'
semantic.unresolved_name 'Evaluation'
semantic.unresolved_name 'Evaluation'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'result'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'HappensJustBefore'
semantic.unresolved_name 'HappensJustBefore'
semantic.unresolved_name 'HappensJustBefore'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Performance'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'timeSlices'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'HappensWhile'
semantic.unresolved_name 'SelfSameLifeLink'
semantic.unresolved_name 'Evaluation'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'result'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'timeSlices'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'timeSlices'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'snapshots'
semantic.unresolved_name 'HappensJustBefore'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'HappensWhile'
semantic.unresolved_name 'SelfSameLifeLink'
semantic.unresolved_name 'SelfSameLifeLink'
semantic.unresolved_name 'HappensBefore'
semantic.unresolved_name 'Evaluation'
semantic.unresolved_name 'Evaluation'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'result'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'HappensJustBefore'
semantic.unresolved_name 'HappensJustBefore'
semantic.unresolved_name 'HappensJustBefore'
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
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwAbstract,KwBehavior,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,KwAbstract,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwInout,KwFeature,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,
KwAbstract,KwBehavior,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,KwAbstract,KwFeature,Ident,Colon,Ident,OpenCurly,
KwAbstract,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,OpenCurly,
KwAbstract,KwFeature,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,Semicolon,
CloseCurly,
CloseCurly,
KwConnector,Colon,Ident,KwFrom,Ident,Dot,Ident,Dot,Ident,KwTo,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwConnector,Colon,Ident,KwFrom,Ident,Dot,Ident,Dot,Ident,KwTo,Ident,Semicolon,
CloseCurly,
KwAbstract,KwFunction,Ident,KwSpecializes,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwReturn,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,KwRedefines,Ident,KwRedefines,Ident,Semicolon,
CloseCurly,
KwAbstract,KwBehavior,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwRedefines,Ident,Semicolon,
KwInout,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,OpenSquare,Star,CloseSquare,KwNonunique,Semicolon,
CloseCurly,
KwAbstract,KwBehavior,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,KwFeature,KwRedefines,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,OpenCurly,
KwAbstract,KwFeature,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,OpenCurly,
KwFeature,KwRedefines,Ident,Semicolon,
CloseCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,OpenCurly,
KwFeature,KwRedefines,Ident,Semicolon,
CloseCurly,
KwConnector,Colon,Ident,KwFrom,Ident,KwTo,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwOut,KwFeature,Ident,KwRedefines,Ident,Semicolon,
KwOut,KwFeature,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,Semicolon,
KwInv,OpenCurly,KwNot,Ident,Arrow,Ident,OpenParen,Ident,CloseParen,CloseCurly,
KwPrivate,KwConnector,Colon,Ident,
KwFrom,OpenSquare,DecimalValue,CloseSquare,Ident,Dot,Ident,Dot,Ident,Dot,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
KwPrivate,KwConnector,Colon,Ident,
KwFrom,OpenSquare,DecimalValue,CloseSquare,Ident,Dot,Ident,Dot,Ident,Dot,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
KwPrivate,KwConnector,Colon,Ident,
KwFrom,OpenSquare,DecimalValue,CloseSquare,Ident,Dot,Ident,Dot,Ident,Dot,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
KwProtected,KwConnector,Ident,Colon,Ident,
KwFrom,OpenSquare,DecimalValue,CloseSquare,Ident,Dot,Ident,Dot,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
CloseCurly,
KwBehavior,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,OpenCurly,
KwProtected,KwExpr,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwRedefines,Ident,OpenCurly,
KwReturn,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwRedefines,Ident,Comma,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwBehavior,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,KwBool,KwRedefines,Ident,OpenCurly,
KwProtected,KwBool,KwRedefines,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwReturn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
KwOut,KwRedefines,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwOut,KwRedefines,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwBehavior,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,KwBool,KwRedefines,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,KwTrue,Semicolon,
KwOut,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwRedefines,Ident,Eq,Ident,Semicolon,
KwPrivate,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwFeature,KwRedefines,Ident,Colon,Ident,OpenCurly,
KwEnd,KwFeature,Ident,Semicolon,
KwEnd,KwFeature,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPrivate,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwFeature,KwRedefines,Ident,Colon,Ident,OpenCurly,
KwEnd,KwFeature,Ident,Semicolon,
KwEnd,KwFeature,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPrivate,KwConnector,Colon,Ident,KwFrom,OpenSquare,DecimalValue,CloseSquare,Ident,KwTo,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwInv,OpenCurly,Ident,OpenParen,Ident,CloseParen,EqEq,OpenParen,Ident,Dot,Ident,EqEq,Ident,CloseParen,CloseCurly,
KwPrivate,KwBinding,OpenSquare,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
KwPrivate,KwBinding,OpenSquare,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'FeatureReferencingPerformances'
    (documentation)
    (import_decl private 'Base::Anything')
    (import_decl private 'Base::things')
    (import_decl private 'Occurrences::Occurrence')
    (import_decl private 'Occurrences::HappensWhile')
    (import_decl private 'Occurrences::HappensBefore')
    (import_decl private 'Occurrences::HappensJustBefore')
    (import_decl private 'Occurrences::SelfSameLifeLink')
    (import_decl private 'Performances::Performance')
    (import_decl private 'Performances::Evaluation')
    (import_decl private 'ScalarValues::Boolean')
    (import_decl private 'SequenceFunctions::isEmpty')
    (import_decl private 'SequenceFunctions::equals')
    (behavior_def
      (documentation)
      (feature_def in abstract 'onOccurrence' : 'Occurrence' multiplicity
        (documentation))
      (feature_def inout 'values' : 'Anything' multiplicity nonunique
        (documentation)))
    (behavior_def
      (documentation)
      (feature_def in abstract 'onOccurrence' : 'Occurrence'
        (feature_def abstract 'startingAt' : 'Occurrence' multiplicity :> 'timeSlices'
          (feature_def abstract 'accessedFeature' : 'Anything' multiplicity nonunique)))
      (connector_def : 'HappensWhile'
        (connector_end)
        (connector_end)
        (documentation))
      (connector_def : 'SelfSameLifeLink'
        (connector_end)
        (connector_end)))
    (function_def
      (documentation)
      (feature_def in 'onOccurrence' : 'Occurrence' multiplicity)
      (return_member))
    (behavior_def
      (documentation)
      (feature_def in 'onOccurrence' : 'Occurrence' multiplicity :>> 'onOccurrence')
      (feature_def inout 'replacementValues' : 'Anything' :>> 'values' multiplicity nonunique))
    (behavior_def
      (documentation)
      (feature_def in :>> 'onOccurrence'
        (feature_def 'monitoredOccurrence' : 'Occurrence' multiplicity :> 'timeSlices'
          (feature_def abstract 'monitoredFeature' : 'Anything' multiplicity nonunique)
          (feature_def 'beforeTimeSlice' : 'Occurrence' multiplicity :> 'timeSlices'
            (feature_def :>> 'monitoredFeature'))
          (feature_def 'afterSnapshot' : 'Occurrence' multiplicity :> 'snapshots'
            (feature_def :>> 'monitoredFeature'))
          (connector_def : 'HappensJustBefore'
            (connector_end)
            (connector_end))))
      (feature_def out 'afterValues' :>> 'values')
      (feature_def out 'beforeValues' : 'Anything' multiplicity nonunique)
      (invariant_def
        (result_expr_member))
      (connector_def private : 'HappensWhile'
        (connector_end)
        (connector_end))
      (connector_def private : 'SelfSameLifeLink'
        (connector_end)
        (connector_end))
      (connector_def private : 'SelfSameLifeLink'
        (connector_end)
        (connector_end))
      (connector_def protected 'endWhen' : 'HappensBefore'
        (connector_end)
        (connector_end)))
    (behavior_def
      (documentation)
      (feature_def in 'onOccurrence' : 'Evaluation' :>> 'onOccurrence'
        (expression_def
          (return_member))))
    (behavior_def
      (documentation)
      (boolean_expr_usage
        (boolean_expr_def
          (return_member)))
      (feature_def out :>> 'afterValues' : 'Boolean' multiplicity)
      (feature_def out :>> 'beforeValues' : 'Boolean' multiplicity))
    (behavior_def
      (documentation)
      (boolean_expr_usage)
      (feature_def 'isToTrue' : 'Boolean' multiplicity value)
      (feature_def out 'afterValues' : 'Boolean' multiplicity :>> 'values' value)
      (feature_def private 'monitor1' : 'BooleanEvaluationResultMonitorPerformance' multiplicity
        (feature_def :>> 'endWhen' : 'HappensJustBefore'
          (feature_def end 'earlierOccurrence')
          (feature_def end 'laterOccurrence')))
      (feature_def private 'monitor2' : 'BooleanEvaluationResultMonitorPerformance' multiplicity
        (feature_def :>> 'endWhen' : 'HappensJustBefore'
          (feature_def end 'earlierOccurrence')
          (feature_def end 'laterOccurrence')))
      (connector_def private : 'HappensJustBefore'
        (connector_end)
        (connector_end))
      (invariant_def
        (result_expr_member))
      (binding_connector private multiplicity
        (connector_end)
        (connector_end))
      (binding_connector private multiplicity
        (connector_end)
        (connector_end)))))
~~~
# FORMAT
~~~sysml
standard library package FeatureReferencingPerformances {
    doc /*
	 * This package defines Behaviors used to read, write and monitor values of a referenced Feature of an 
	 * Occurrence.
	 */

    private import Base::Anything;
    private import Base::things;
    private import Occurrences::Occurrence;
    private import Occurrences::HappensWhile;
    private import Occurrences::HappensBefore;
    private import Occurrences::HappensJustBefore;
    private import Occurrences::SelfSameLifeLink;
    private import Performances::Performance;
    private import Performances::Evaluation;
    private import ScalarValues::Boolean;
    private import SequenceFunctions::isEmpty;
    private import SequenceFunctions::equals;

    abstract behavior FeatureReferencingPerformance specializes Performance {
        doc /*
		 * A FeatureReferencingPerformance is the base Performance for specialized behaviors related 
		 * to values of a referenced Feature of a given Occurrence, as identified in specializations 
		 * of this Behavior.
		 */

        in abstract feature onOccurrence : Occurrence [1] {
            doc /*
			 * Occurrence with values for the referenced feature in specializations of this behavior.
			 */
        }

        inout feature values : Anything [*] nonunique {
            doc /*
			 * Values of the referenced feature, as specified in specializations of this behavior.
			 */
        }
    }

    abstract behavior FeatureAccessPerformance specializes FeatureReferencingPerformance {
        doc /*
		 * A FeatureAccessPerformance is a FeatureReferencingPerformance where the result values
		 * are all the values of a feature of onOccurrence at the time the Performance ends. The
		 * feature is specified by restricting accessedFeature in specializations or usages.
		 */

        in abstract feature onOccurrence : Occurrence {
            abstract feature startingAt : Occurrence [1] subsets timeSlices {
                abstract feature accessedFeature : Anything [*] nonunique;
            }
        }

        connector : HappensWhile from onOccurrence.startingAt.startShot to endShot {
            doc /*
			 * Requires some time slice of onOccurrence to start when this performance
			 * ends (this connector), with particular feature values (following connector).
			 * The feature is specified by restricting the onOccurrence::accessedFeature 
			 * on usages of this behavior.
			 */
        }
        connector : SelfSameLifeLink from onOccurrence.startingAt.accessedFeature to values;
    }

    abstract function FeatureReadEvaluation specializes FeatureAccessPerformance, Evaluation { 
		doc
		/*
		 * A FeatureReadEvaluation is a FeatureAccessPerformance that is a function providing as
		 * its result the values of a feature on an occurrence at the time its evaluation ends.
		 */

		in onOccurrence: Occurrence [1];
		return resultValues : Anything [*] nonunique redefines result redefines values;
	}

    abstract behavior FeatureWritePerformance specializes FeatureAccessPerformance {
        doc /*
		 * A FeatureWritePerformance is a FeatureAccessPerformance that assigns the values of a 
		 * feature on an occurrence to the given replacementValues at time its performance ends.
		 */

        in feature onOccurrence : Occurrence [1] redefines onOccurrence;
        inout feature replacementValues : Anything redefines values [*] nonunique;
    }

    abstract behavior FeatureMonitorPerformance specializes FeatureReferencingPerformance {
        doc /*
		 * A FeatureMonitorPerformance is a FeatureReferencingPerformance that waits for values
		 * of monitoredFeature to change on onOccurrence from what they were when the performance 
		 * started. The values before and after the change are given by beforeValues and afterValues.
		 */

        in feature redefines onOccurrence {
            feature monitoredOccurrence : Occurrence [1] subsets timeSlices {
                abstract feature monitoredFeature : Anything [*] nonunique;
                feature beforeTimeSlice : Occurrence [1] subsets timeSlices {
                    feature redefines monitoredFeature;
                }
                feature afterSnapshot : Occurrence [1] subsets snapshots {
                    feature redefines monitoredFeature;
                }
                connector : HappensJustBefore from beforeTimeSlice to afterSnapshot;
            }
        }
        out feature afterValues redefines values;
        out feature beforeValues : Anything [*] nonunique;
        inv { not beforeValues->equals(afterValues) }

        private connector : HappensWhile from [1] onOccurrence.monitoredOccurrence.beforeTimeSlice.startShot to [1] startShot;
        private connector : SelfSameLifeLink from [1] onOccurrence.monitoredOccurrence.beforeTimeSlice.monitoredFeature to [1] beforeValues;
        private connector : SelfSameLifeLink from [1] onOccurrence.monitoredOccurrence.afterSnapshot.monitoredFeature to [1] afterValues;
        protected connector endWhen : HappensBefore from [1] onOccurrence.monitoredOccurrence.afterSnapshot to [1] endShot;
    }

    behavior EvaluationResultMonitorPerformance specializes FeatureMonitorPerformance {
        doc /*
		 * An EvaluationResultMonitorPerformance is a FeatureMonitorPerformance that waits for changes 
		 * in the result of an Evaluation identified by onOccurrence. The Predicate being evaluated must 
		 * be able to produce multiple results over time, for example by only using BindingConnectors 
		 * between Steps, rather than Successions or Flows, including in its Step behaviors.
		 */

        in feature onOccurrence : Evaluation redefines onOccurrence {
            protected expr monitoredOccurrence : Evaluation [1] redefines monitoredOccurrence {
				return result : Anything[*] redefines result, monitoredFeature; 
			}
        }
    }

    behavior BooleanEvaluationResultMonitorPerformance specializes EvaluationResultMonitorPerformance {
        doc /*
		 * A BooleanEvaluationResultMonitorPerformance is a EvaluationResultMonitorPerformance
		 * that waits for changes in the result of a BooleanEvaluation identified by onOccurrence.
		 */

        in bool redefines onOccurrence {
	    	protected bool redefines monitoredOccurrence[1] {
	    	    return result : Boolean [1];
	    	}
		}
        out redefines afterValues : Boolean [1];
        out redefines beforeValues : Boolean [1];
    }

    behavior BooleanEvaluationResultToMonitorPerformance specializes FeatureReferencingPerformance {
        doc /*
		 * A BooleanEvaluationResultToMonitorPerformance is a FeatureReferencingPerformance that waits 
		 * for the result of a BooleanEvaluation (identified by onOccurrence) to change to either true 
		 * or false, as indicated by isToTrue (defaulting to true). If the result is already true (or false), 
		 * the performance waits for the result to become false (or true) before waiting again for it to 
		 * change back.
		 */

        in bool redefines onOccurrence;
        feature isToTrue : Boolean [1] default = true;
        out afterValues: Boolean [1] redefines values = isToTrue;
        private feature monitor1 : BooleanEvaluationResultMonitorPerformance [1] {
            feature redefines endWhen : HappensJustBefore {
                end feature earlierOccurrence;
                end feature laterOccurrence;
            }
        }
        private feature monitor2 : BooleanEvaluationResultMonitorPerformance [1] {
            feature redefines endWhen : HappensJustBefore {
                end feature earlierOccurrence;
                end feature laterOccurrence;
            }
        }

        private connector : HappensJustBefore from [1] monitor1 to [0..1] monitor2;
        inv { isEmpty(monitor2) == (monitor1.afterValues == isToTrue) }

        private binding [1] monitor1.onOccurrence = [1] onOccurrence;
        private binding [1] monitor2.onOccurrence = [1] onOccurrence;
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'FeatureReferencingPerformances'
      (documentation)
      (membership_import private -> 'Base::Anything'[unresolved])
      (membership_import private -> 'Base::things'[unresolved])
      (membership_import private -> 'Occurrences::Occurrence'[unresolved])
      (membership_import private -> 'Occurrences::HappensWhile'[unresolved])
      (membership_import private -> 'Occurrences::HappensBefore'[unresolved])
      (membership_import private -> 'Occurrences::HappensJustBefore'[unresolved])
      (membership_import private -> 'Occurrences::SelfSameLifeLink'[unresolved])
      (membership_import private -> 'Performances::Performance'[unresolved])
      (membership_import private -> 'Performances::Evaluation'[unresolved])
      (membership_import private -> 'ScalarValues::Boolean'[unresolved])
      (membership_import private -> 'SequenceFunctions::isEmpty'[unresolved])
      (membership_import private -> 'SequenceFunctions::equals'[unresolved])
      (behavior_def abstract 'FeatureReferencingPerformance' :> 'Performance'[unresolved]
        (documentation)
        (feature_def in abstract 'onOccurrence' : 'Occurrence'[unresolved]
          (multiplicity_range [1])
          (documentation))
        (feature_def inout 'values' : 'Anything'[unresolved]
          (multiplicity_range [*])
          (documentation)))
      (behavior_def abstract 'FeatureAccessPerformance' :> 'FeatureReferencingPerformances::FeatureReferencingPerformance'[behavior_def]
        (documentation)
        (feature_def in abstract 'onOccurrence' : 'Occurrence'[unresolved] :>> 'FeatureReferencingPerformances::FeatureReferencingPerformance::onOccurrence'[feature_def][implied]
          (feature_def abstract 'startingAt' : 'Occurrence'[unresolved] :> 'timeSlices'[unresolved]
            (multiplicity_range [1])
            (feature_def abstract 'accessedFeature' : 'Anything'[unresolved]
              (multiplicity_range [*]))))
        (connector_def : 'HappensWhile'[unresolved]
          (connector_end 'onOccurrence.startingAt.startShot')
          (connector_end 'endShot')
          (documentation))
        (connector_def : 'SelfSameLifeLink'[unresolved]
          (connector_end 'onOccurrence.startingAt.accessedFeature')
          (connector_end 'values')))
      (function_def abstract 'FeatureReadEvaluation' :> 'FeatureReferencingPerformances::FeatureAccessPerformance'[behavior_def] :> 'Evaluation'[unresolved]
        (documentation)
        (feature_def in 'onOccurrence' : 'Occurrence'[unresolved] :>> 'FeatureReferencingPerformances::FeatureAccessPerformance::onOccurrence'[feature_def][implied]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out 'resultValues' : 'Anything'[unresolved] :>> 'result'[unresolved] :>> 'FeatureReferencingPerformances::FeatureReferencingPerformance::values'[feature_def]
            (multiplicity_range [*]))))
      (behavior_def abstract 'FeatureWritePerformance' :> 'FeatureReferencingPerformances::FeatureAccessPerformance'[behavior_def]
        (documentation)
        (feature_def in 'onOccurrence' : 'Occurrence'[unresolved] :>> 'FeatureReferencingPerformances::FeatureAccessPerformance::onOccurrence'[feature_def]
          (multiplicity_range [1]))
        (feature_def inout 'replacementValues' : 'Anything'[unresolved] :>> 'FeatureReferencingPerformances::FeatureReferencingPerformance::values'[feature_def]
          (multiplicity_range [*])))
      (behavior_def abstract 'FeatureMonitorPerformance' :> 'FeatureReferencingPerformances::FeatureReferencingPerformance'[behavior_def]
        (documentation)
        (feature_def in :>> 'FeatureReferencingPerformances::FeatureReferencingPerformance::onOccurrence'[feature_def]
          (feature_def 'monitoredOccurrence' : 'Occurrence'[unresolved] :> 'timeSlices'[unresolved]
            (multiplicity_range [1])
            (feature_def abstract 'monitoredFeature' : 'Anything'[unresolved]
              (multiplicity_range [*]))
            (feature_def 'beforeTimeSlice' : 'Occurrence'[unresolved] :> 'timeSlices'[unresolved]
              (multiplicity_range [1])
              (feature_def :>> 'monitoredOccurrence::monitoredFeature'[feature_def]))
            (feature_def 'afterSnapshot' : 'Occurrence'[unresolved] :> 'snapshots'[unresolved]
              (multiplicity_range [1])
              (feature_def :>> 'monitoredOccurrence::monitoredFeature'[feature_def]))
            (connector_def : 'HappensJustBefore'[unresolved]
              (connector_end 'beforeTimeSlice')
              (connector_end 'afterSnapshot'))))
        (feature_def out 'afterValues' :>> 'FeatureReferencingPerformances::FeatureReferencingPerformance::values'[feature_def])
        (feature_def out 'beforeValues' : 'Anything'[unresolved]
          (multiplicity_range [*]))
        (invariant_def
          (result_expr_membership))
        (connector_def : 'HappensWhile'[unresolved]
          (connector_end 'onOccurrence.monitoredOccurrence.beforeTimeSlice.startShot')
          (connector_end 'startShot'))
        (connector_def : 'SelfSameLifeLink'[unresolved]
          (connector_end 'onOccurrence.monitoredOccurrence.beforeTimeSlice.monitoredFeature')
          (connector_end 'beforeValues'))
        (connector_def : 'SelfSameLifeLink'[unresolved]
          (connector_end 'onOccurrence.monitoredOccurrence.afterSnapshot.monitoredFeature')
          (connector_end 'afterValues'))
        (connector_def 'endWhen' : 'HappensBefore'[unresolved]
          (connector_end 'onOccurrence.monitoredOccurrence.afterSnapshot')
          (connector_end 'endShot')))
      (behavior_def 'EvaluationResultMonitorPerformance' :> 'FeatureReferencingPerformances::FeatureMonitorPerformance'[behavior_def]
        (documentation)
        (feature_def in 'onOccurrence' : 'Evaluation'[unresolved] :>> ''[feature_def]
          (expression_def 'monitoredOccurrence' : 'Evaluation'[unresolved] :>> 'monitoredOccurrence'[feature_def]
            (multiplicity_range [1])
            (return_parameter_membership
              (feature_def out 'result' : 'Anything'[unresolved] :>> 'result'[unresolved] :>> 'monitoredOccurrence::monitoredFeature'[feature_def]
                (multiplicity_range [*]))))))
      (behavior_def 'BooleanEvaluationResultMonitorPerformance' :> 'FeatureReferencingPerformances::EvaluationResultMonitorPerformance'[behavior_def]
        (documentation)
        (boolean_expr_usage in :>> 'FeatureReferencingPerformances::EvaluationResultMonitorPerformance::onOccurrence'[feature_def]
          (boolean_expr_def :>> 'FeatureReferencingPerformances::EvaluationResultMonitorPerformance::onOccurrence::monitoredOccurrence'[expression_def]
            (multiplicity_range [1])
            (return_parameter_membership
              (feature_def out 'result' : 'Boolean'[unresolved]
                (multiplicity_range [1])))))
        (feature_def out :>> 'FeatureReferencingPerformances::FeatureMonitorPerformance::afterValues'[feature_def] : 'Boolean'[unresolved]
          (multiplicity_range [1]))
        (feature_def out :>> 'FeatureReferencingPerformances::FeatureMonitorPerformance::beforeValues'[feature_def] : 'Boolean'[unresolved]
          (multiplicity_range [1])))
      (behavior_def 'BooleanEvaluationResultToMonitorPerformance' :> 'FeatureReferencingPerformances::FeatureReferencingPerformance'[behavior_def]
        (documentation)
        (boolean_expr_usage in :>> 'FeatureReferencingPerformances::FeatureReferencingPerformance::onOccurrence'[feature_def])
        (feature_def 'isToTrue' : 'Boolean'[unresolved]
          (multiplicity_range [1])
          (feature_value (default =)))
        (feature_def out 'afterValues' : 'Boolean'[unresolved] :>> 'FeatureReferencingPerformances::FeatureReferencingPerformance::values'[feature_def]
          (multiplicity_range [1])
          (feature_value (=)))
        (feature_def 'monitor1' : 'FeatureReferencingPerformances::BooleanEvaluationResultMonitorPerformance'[behavior_def]
          (multiplicity_range [1])
          (feature_def :>> 'FeatureReferencingPerformances::FeatureMonitorPerformance::endWhen'[connector_def] : 'HappensJustBefore'[unresolved]
            (feature_def end 'earlierOccurrence')
            (feature_def end 'laterOccurrence')))
        (feature_def 'monitor2' : 'FeatureReferencingPerformances::BooleanEvaluationResultMonitorPerformance'[behavior_def]
          (multiplicity_range [1])
          (feature_def :>> 'FeatureReferencingPerformances::FeatureMonitorPerformance::endWhen'[connector_def] : 'HappensJustBefore'[unresolved]
            (feature_def end 'earlierOccurrence')
            (feature_def end 'laterOccurrence')))
        (connector_def : 'HappensJustBefore'[unresolved]
          (connector_end 'monitor1')
          (connector_end 'monitor2'))
        (invariant_def
          (result_expr_membership))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'monitor1.onOccurrence')
          (connector_end 'onOccurrence'))
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'monitor2.onOccurrence')
          (connector_end 'onOccurrence'))))))
~~~
