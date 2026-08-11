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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "1ea58d02d5828e68fce4f9372d43088716ef5daeaa8a04e14cdf42fed82486b9") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "FeatureReferencingPerformances"))) (kind "package") (name "FeatureReferencingPerformances") (declared-name "FeatureReferencingPerformances"))
    (element (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::Anything"))) (kind "import") (name "Anything") (declared-name "Anything") (parent (node (document "d0") (qualified-name "FeatureReferencingPerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Base::Anything") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::Boolean"))) (kind "import") (name "Boolean") (declared-name "Boolean") (parent (node (document "d0") (qualified-name "FeatureReferencingPerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Boolean") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultMonitorPerformance"))) (kind "kermlDecl") (name "BooleanEvaluationResultMonitorPerformance") (declared-name "BooleanEvaluationResultMonitorPerformance") (parent (node (document "d0") (qualified-name "FeatureReferencingPerformances"))))
    (element (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance"))) (kind "kermlDecl") (name "BooleanEvaluationResultToMonitorPerformance") (declared-name "BooleanEvaluationResultToMonitorPerformance") (parent (node (document "d0") (qualified-name "FeatureReferencingPerformances"))))
    (element (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::Evaluation"))) (kind "import") (name "Evaluation") (declared-name "Evaluation") (parent (node (document "d0") (qualified-name "FeatureReferencingPerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Performances::Evaluation") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::EvaluationResultMonitorPerformance"))) (kind "kermlDecl") (name "EvaluationResultMonitorPerformance") (declared-name "EvaluationResultMonitorPerformance") (parent (node (document "d0") (qualified-name "FeatureReferencingPerformances"))))
    (element (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::FeatureAccessPerformance"))) (kind "kermlDecl") (name "FeatureAccessPerformance") (declared-name "FeatureAccessPerformance") (parent (node (document "d0") (qualified-name "FeatureReferencingPerformances"))))
    (element (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::FeatureMonitorPerformance"))) (kind "kermlDecl") (name "FeatureMonitorPerformance") (declared-name "FeatureMonitorPerformance") (parent (node (document "d0") (qualified-name "FeatureReferencingPerformances"))))
    (element (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::FeatureReadEvaluation"))) (kind "kermlDecl") (name "FeatureReadEvaluation") (declared-name "FeatureReadEvaluation") (parent (node (document "d0") (qualified-name "FeatureReferencingPerformances"))))
    (element (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::FeatureReferencingPerformance"))) (kind "kermlDecl") (name "FeatureReferencingPerformance") (declared-name "FeatureReferencingPerformance") (parent (node (document "d0") (qualified-name "FeatureReferencingPerformances"))))
    (element (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::FeatureWritePerformance"))) (kind "kermlDecl") (name "FeatureWritePerformance") (declared-name "FeatureWritePerformance") (parent (node (document "d0") (qualified-name "FeatureReferencingPerformances"))))
    (element (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::HappensBefore"))) (kind "import") (name "HappensBefore") (declared-name "HappensBefore") (parent (node (document "d0") (qualified-name "FeatureReferencingPerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::HappensBefore") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::HappensJustBefore"))) (kind "import") (name "HappensJustBefore") (declared-name "HappensJustBefore") (parent (node (document "d0") (qualified-name "FeatureReferencingPerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::HappensJustBefore") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::HappensWhile"))) (kind "import") (name "HappensWhile") (declared-name "HappensWhile") (parent (node (document "d0") (qualified-name "FeatureReferencingPerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::HappensWhile") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::Occurrence"))) (kind "import") (name "Occurrence") (declared-name "Occurrence") (parent (node (document "d0") (qualified-name "FeatureReferencingPerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::Occurrence") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::Performance"))) (kind "import") (name "Performance") (declared-name "Performance") (parent (node (document "d0") (qualified-name "FeatureReferencingPerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Performances::Performance") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::SelfSameLifeLink"))) (kind "import") (name "SelfSameLifeLink") (declared-name "SelfSameLifeLink") (parent (node (document "d0") (qualified-name "FeatureReferencingPerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::SelfSameLifeLink") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "FeatureReferencingPerformances"))))
    (element (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::equals"))) (kind "import") (name "equals") (declared-name "equals") (parent (node (document "d0") (qualified-name "FeatureReferencingPerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::equals") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::isEmpty"))) (kind "import") (name "isEmpty") (declared-name "isEmpty") (parent (node (document "d0") (qualified-name "FeatureReferencingPerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::isEmpty") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "FeatureReferencingPerformances::things"))) (kind "import") (name "things") (declared-name "things") (parent (node (document "d0") (qualified-name "FeatureReferencingPerformances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Base::things") (origin Import) (shape Membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "FeatureReferencingPerformances::Anything"))) (kind membershipImport) (ordinal 0)) (authored-target "Base::Anything") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "FeatureReferencingPerformances::Boolean"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Boolean") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "FeatureReferencingPerformances::Evaluation"))) (kind membershipImport) (ordinal 0)) (authored-target "Performances::Evaluation") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "FeatureReferencingPerformances::HappensBefore"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::HappensBefore") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "FeatureReferencingPerformances::HappensJustBefore"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::HappensJustBefore") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "FeatureReferencingPerformances::HappensWhile"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::HappensWhile") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "FeatureReferencingPerformances::Occurrence"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::Occurrence") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "FeatureReferencingPerformances::Performance"))) (kind membershipImport) (ordinal 0)) (authored-target "Performances::Performance") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "FeatureReferencingPerformances::SelfSameLifeLink"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::SelfSameLifeLink") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "FeatureReferencingPerformances::equals"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::equals") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "FeatureReferencingPerformances::isEmpty"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::isEmpty") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "FeatureReferencingPerformances::things"))) (kind membershipImport) (ordinal 0)) (authored-target "Base::things") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
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
    (query (range (start 8 16) (end 8 28)) (probe (position 8 16))
      (reference
        (source (document "d0") (qualified-name "FeatureReferencingPerformances::things"))
        (kind membershipImport) (ordinal 0) (authored-target "Base::things")
        (range (start 8 16) (end 8 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 16) (end 7 30)) (probe (position 7 16))
      (reference
        (source (document "d0") (qualified-name "FeatureReferencingPerformances::Anything"))
        (kind membershipImport) (ordinal 0) (authored-target "Base::Anything")
        (range (start 7 16) (end 7 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 16 16) (end 16 37)) (probe (position 16 16))
      (reference
        (source (document "d0") (qualified-name "FeatureReferencingPerformances::Boolean"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Boolean")
        (range (start 16 16) (end 16 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 16) (end 9 39)) (probe (position 9 16))
      (reference
        (source (document "d0") (qualified-name "FeatureReferencingPerformances::Occurrence"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::Occurrence")
        (range (start 9 16) (end 9 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 15 16) (end 15 40)) (probe (position 15 16))
      (reference
        (source (document "d0") (qualified-name "FeatureReferencingPerformances::Evaluation"))
        (kind membershipImport) (ordinal 0) (authored-target "Performances::Evaluation")
        (range (start 15 16) (end 15 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 16) (end 10 41)) (probe (position 10 16))
      (reference
        (source (document "d0") (qualified-name "FeatureReferencingPerformances::HappensWhile"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::HappensWhile")
        (range (start 10 16) (end 10 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 14 16) (end 14 41)) (probe (position 14 16))
      (reference
        (source (document "d0") (qualified-name "FeatureReferencingPerformances::Performance"))
        (kind membershipImport) (ordinal 0) (authored-target "Performances::Performance")
        (range (start 14 16) (end 14 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 18 16) (end 18 41)) (probe (position 18 16))
      (reference
        (source (document "d0") (qualified-name "FeatureReferencingPerformances::equals"))
        (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::equals")
        (range (start 18 16) (end 18 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 11 16) (end 11 42)) (probe (position 11 16))
      (reference
        (source (document "d0") (qualified-name "FeatureReferencingPerformances::HappensBefore"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::HappensBefore")
        (range (start 11 16) (end 11 42))
        (outcome (status unresolved))
      )
    )
    (query (range (start 17 16) (end 17 42)) (probe (position 17 16))
      (reference
        (source (document "d0") (qualified-name "FeatureReferencingPerformances::isEmpty"))
        (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::isEmpty")
        (range (start 17 16) (end 17 42))
        (outcome (status unresolved))
      )
    )
    (query (range (start 13 16) (end 13 45)) (probe (position 13 16))
      (reference
        (source (document "d0") (qualified-name "FeatureReferencingPerformances::SelfSameLifeLink"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::SelfSameLifeLink")
        (range (start 13 16) (end 13 45))
        (outcome (status unresolved))
      )
    )
    (query (range (start 12 16) (end 12 46)) (probe (position 12 16))
      (reference
        (source (document "d0") (qualified-name "FeatureReferencingPerformances::HappensJustBefore"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::HappensJustBefore")
        (range (start 12 16) (end 12 46))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
