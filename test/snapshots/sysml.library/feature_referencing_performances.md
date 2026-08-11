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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "feature_referencing_performances.md"
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
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "1ea58d02d5828e68fce4f9372d43088716ef5daeaa8a04e14cdf42fed82486b9") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "FeatureReferencingPerformances"))) (kind "package") (name "FeatureReferencingPerformances") (declared-name "FeatureReferencingPerformances") (range (start (line 0) (character 0)) (end (line 0) (character 7575))))
    (element (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::Anything"))) (kind "import") (name "Anything") (declared-name "Anything") (range (start (line 7) (character 1)) (end (line 7) (character 31))) (parent (node (document "d0") (qualified-name "FeatureReferencingPerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Base::Anything") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 16)) (end (line 7) (character 30))))))
    (element (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::Boolean"))) (kind "import") (name "Boolean") (declared-name "Boolean") (range (start (line 16) (character 1)) (end (line 16) (character 38))) (parent (node (document "d0") (qualified-name "FeatureReferencingPerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Boolean") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 16) (character 16)) (end (line 16) (character 37))))))
    (element (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultMonitorPerformance"))) (kind "kermlDecl") (name "BooleanEvaluationResultMonitorPerformance") (declared-name "BooleanEvaluationResultMonitorPerformance") (range (start (line 141) (character 1)) (end (line 141) (character 540))) (parent (node (document "d0") (qualified-name "FeatureReferencingPerformances"))))
    (element (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance"))) (kind "kermlDecl") (name "BooleanEvaluationResultToMonitorPerformance") (declared-name "BooleanEvaluationResultToMonitorPerformance") (range (start (line 157) (character 1)) (end (line 157) (character 1442))) (parent (node (document "d0") (qualified-name "FeatureReferencingPerformances"))))
    (element (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::Evaluation"))) (kind "import") (name "Evaluation") (declared-name "Evaluation") (range (start (line 15) (character 1)) (end (line 15) (character 41))) (parent (node (document "d0") (qualified-name "FeatureReferencingPerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Performances::Evaluation") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 15) (character 16)) (end (line 15) (character 40))))))
    (element (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::EvaluationResultMonitorPerformance"))) (kind "kermlDecl") (name "EvaluationResultMonitorPerformance") (declared-name "EvaluationResultMonitorPerformance") (range (start (line 125) (character 1)) (end (line 125) (character 723))) (parent (node (document "d0") (qualified-name "FeatureReferencingPerformances"))))
    (element (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::FeatureAccessPerformance"))) (kind "kermlDecl") (name "FeatureAccessPerformance") (declared-name "FeatureAccessPerformance") (range (start (line 43) (character 4)) (end (line 43) (character 1057))) (parent (node (document "d0") (qualified-name "FeatureReferencingPerformances"))))
    (element (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::FeatureMonitorPerformance"))) (kind "kermlDecl") (name "FeatureMonitorPerformance") (declared-name "FeatureMonitorPerformance") (range (start (line 91) (character 1)) (end (line 91) (character 1608))) (parent (node (document "d0") (qualified-name "FeatureReferencingPerformances"))))
    (element (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::FeatureReadEvaluation"))) (kind "kermlDecl") (name "FeatureReadEvaluation") (declared-name "FeatureReadEvaluation") (range (start (line 69) (character 1)) (end (line 69) (character 411))) (parent (node (document "d0") (qualified-name "FeatureReferencingPerformances"))))
    (element (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::FeatureReferencingPerformance"))) (kind "kermlDecl") (name "FeatureReferencingPerformance") (declared-name "FeatureReferencingPerformance") (range (start (line 20) (character 1)) (end (line 20) (character 652))) (parent (node (document "d0") (qualified-name "FeatureReferencingPerformances"))))
    (element (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::FeatureWritePerformance"))) (kind "kermlDecl") (name "FeatureWritePerformance") (declared-name "FeatureWritePerformance") (range (start (line 80) (character 1)) (end (line 80) (character 430))) (parent (node (document "d0") (qualified-name "FeatureReferencingPerformances"))))
    (element (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::HappensBefore"))) (kind "import") (name "HappensBefore") (declared-name "HappensBefore") (range (start (line 11) (character 1)) (end (line 11) (character 43))) (parent (node (document "d0") (qualified-name "FeatureReferencingPerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::HappensBefore") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 11) (character 16)) (end (line 11) (character 42))))))
    (element (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::HappensJustBefore"))) (kind "import") (name "HappensJustBefore") (declared-name "HappensJustBefore") (range (start (line 12) (character 1)) (end (line 12) (character 47))) (parent (node (document "d0") (qualified-name "FeatureReferencingPerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::HappensJustBefore") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 12) (character 16)) (end (line 12) (character 46))))))
    (element (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::HappensWhile"))) (kind "import") (name "HappensWhile") (declared-name "HappensWhile") (range (start (line 10) (character 1)) (end (line 10) (character 42))) (parent (node (document "d0") (qualified-name "FeatureReferencingPerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::HappensWhile") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 10) (character 16)) (end (line 10) (character 41))))))
    (element (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::Occurrence"))) (kind "import") (name "Occurrence") (declared-name "Occurrence") (range (start (line 9) (character 1)) (end (line 9) (character 40))) (parent (node (document "d0") (qualified-name "FeatureReferencingPerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::Occurrence") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 9) (character 16)) (end (line 9) (character 39))))))
    (element (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::Performance"))) (kind "import") (name "Performance") (declared-name "Performance") (range (start (line 14) (character 1)) (end (line 14) (character 42))) (parent (node (document "d0") (qualified-name "FeatureReferencingPerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Performances::Performance") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 14) (character 16)) (end (line 14) (character 41))))))
    (element (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::SelfSameLifeLink"))) (kind "import") (name "SelfSameLifeLink") (declared-name "SelfSameLifeLink") (range (start (line 13) (character 1)) (end (line 13) (character 46))) (parent (node (document "d0") (qualified-name "FeatureReferencingPerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::SelfSameLifeLink") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 13) (character 16)) (end (line 13) (character 45))))))
    (element (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 7575))) (parent (node (document "d0") (qualified-name "FeatureReferencingPerformances"))))
    (element (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::equals"))) (kind "import") (name "equals") (declared-name "equals") (range (start (line 18) (character 1)) (end (line 18) (character 42))) (parent (node (document "d0") (qualified-name "FeatureReferencingPerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::equals") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 18) (character 16)) (end (line 18) (character 41))))))
    (element (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::isEmpty"))) (kind "import") (name "isEmpty") (declared-name "isEmpty") (range (start (line 17) (character 1)) (end (line 17) (character 43))) (parent (node (document "d0") (qualified-name "FeatureReferencingPerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::isEmpty") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 17) (character 16)) (end (line 17) (character 42))))))
    (element (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::things"))) (kind "import") (name "things") (declared-name "things") (range (start (line 8) (character 1)) (end (line 8) (character 29))) (parent (node (document "d0") (qualified-name "FeatureReferencingPerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Base::things") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 8) (character 16)) (end (line 8) (character 28))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "FeatureReferencingPerformances::Anything"))) (kind membershipImport) (ordinal 0)) (authored-target "Base::Anything") (range (start (line 7) (character 16)) (end (line 7) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "FeatureReferencingPerformances::Boolean"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Boolean") (range (start (line 16) (character 16)) (end (line 16) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "FeatureReferencingPerformances::Evaluation"))) (kind membershipImport) (ordinal 0)) (authored-target "Performances::Evaluation") (range (start (line 15) (character 16)) (end (line 15) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "FeatureReferencingPerformances::HappensBefore"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::HappensBefore") (range (start (line 11) (character 16)) (end (line 11) (character 42))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "FeatureReferencingPerformances::HappensJustBefore"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::HappensJustBefore") (range (start (line 12) (character 16)) (end (line 12) (character 46))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "FeatureReferencingPerformances::HappensWhile"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::HappensWhile") (range (start (line 10) (character 16)) (end (line 10) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "FeatureReferencingPerformances::Occurrence"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::Occurrence") (range (start (line 9) (character 16)) (end (line 9) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "FeatureReferencingPerformances::Performance"))) (kind membershipImport) (ordinal 0)) (authored-target "Performances::Performance") (range (start (line 14) (character 16)) (end (line 14) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "FeatureReferencingPerformances::SelfSameLifeLink"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::SelfSameLifeLink") (range (start (line 13) (character 16)) (end (line 13) (character 45))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "FeatureReferencingPerformances::equals"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::equals") (range (start (line 18) (character 16)) (end (line 18) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "FeatureReferencingPerformances::isEmpty"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::isEmpty") (range (start (line 17) (character 16)) (end (line 17) (character 42))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "FeatureReferencingPerformances::things"))) (kind membershipImport) (ordinal 0)) (authored-target "Base::things") (range (start (line 8) (character 16)) (end (line 8) (character 28))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
