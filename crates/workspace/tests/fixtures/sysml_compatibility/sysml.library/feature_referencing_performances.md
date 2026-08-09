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
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "FeatureReferencingPerformances"))) (name "FeatureReferencingPerformances") (declared-name "FeatureReferencingPerformances")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::Anything"))) (name "Anything") (declared-name "Anything"))
        (element (kind "import") (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::Boolean"))) (name "Boolean") (declared-name "Boolean"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultMonitorPerformance"))) (name "BooleanEvaluationResultMonitorPerformance") (declared-name "BooleanEvaluationResultMonitorPerformance"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance"))) (name "BooleanEvaluationResultToMonitorPerformance") (declared-name "BooleanEvaluationResultToMonitorPerformance"))
        (element (kind "import") (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::Evaluation"))) (name "Evaluation") (declared-name "Evaluation"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::EvaluationResultMonitorPerformance"))) (name "EvaluationResultMonitorPerformance") (declared-name "EvaluationResultMonitorPerformance"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::FeatureAccessPerformance"))) (name "FeatureAccessPerformance") (declared-name "FeatureAccessPerformance"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::FeatureMonitorPerformance"))) (name "FeatureMonitorPerformance") (declared-name "FeatureMonitorPerformance"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::FeatureReadEvaluation"))) (name "FeatureReadEvaluation") (declared-name "FeatureReadEvaluation"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::FeatureReferencingPerformance"))) (name "FeatureReferencingPerformance") (declared-name "FeatureReferencingPerformance"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::FeatureWritePerformance"))) (name "FeatureWritePerformance") (declared-name "FeatureWritePerformance"))
        (element (kind "import") (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::HappensBefore"))) (name "HappensBefore") (declared-name "HappensBefore"))
        (element (kind "import") (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::HappensJustBefore"))) (name "HappensJustBefore") (declared-name "HappensJustBefore"))
        (element (kind "import") (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::HappensWhile"))) (name "HappensWhile") (declared-name "HappensWhile"))
        (element (kind "import") (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::Occurrence"))) (name "Occurrence") (declared-name "Occurrence"))
        (element (kind "import") (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::Performance"))) (name "Performance") (declared-name "Performance"))
        (element (kind "import") (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::SelfSameLifeLink"))) (name "SelfSameLifeLink") (declared-name "SelfSameLifeLink"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::_documentation"))) (name ""))
        (element (kind "import") (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::equals"))) (name "equals") (declared-name "equals"))
        (element (kind "import") (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::isEmpty"))) (name "isEmpty") (declared-name "isEmpty"))
        (element (kind "import") (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::things"))) (name "things") (declared-name "things"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "FeatureReferencingPerformances::_documentation"))) (to (node (document "d0") (qualified-name "FeatureReferencingPerformances"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml.library/feature_referencing_performances.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 28))
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
        (range (start 10 16) (end 10 41))
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
        (range (start 12 16) (end 12 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 13 16) (end 13 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 14 16) (end 14 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 15 16) (end 15 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 16 16) (end 16 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 17 16) (end 17 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 18 16) (end 18 41))
      )
    )
  )
)
~~~
