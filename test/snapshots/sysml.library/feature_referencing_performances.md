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
  (document "memory://snapshot/feature_referencing_performances.md"
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
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 20 61) (end 20 72))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 28 2) (end 33 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 35 2) (end 40 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 51 2) (end 55 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 57 16) (end 57 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 57 34) (end 57 67))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 57 71) (end 57 78))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 66 16) (end 66 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 66 38) (end 66 77))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 66 81) (end 66 87))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 69 79) (end 69 89))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 76 19) (end 76 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 77 24) (end 77 32))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 87 2) (end 87 65))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 88 2) (end 88 76))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 99 2) (end 110 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 111 4) (end 111 45))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 112 4) (end 112 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 113 14) (end 113 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 113 35) (end 113 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 115 24) (end 115 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 116 14) (end 116 72))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 116 80) (end 116 89))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 117 24) (end 117 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 118 14) (end 118 79))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 118 87) (end 118 99))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 119 24) (end 119 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 120 14) (end 120 77))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 120 85) (end 120 96))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 121 34) (end 121 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 122 14) (end 122 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 122 68) (end 122 75))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 134 2) (end 138 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 148 4) (end 152 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 153 16) (end 153 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 153 30) (end 153 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 154 16) (end 154 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 154 31) (end 154 38))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 167 4) (end 167 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 168 23) (end 168 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 169 21) (end 169 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 169 42) (end 169 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 171 34) (end 171 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 177 34) (end 177 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 183 24) (end 183 41))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 184 10) (end 184 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 186 24) (end 186 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 186 52) (end 186 64))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 187 24) (end 187 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 187 52) (end 187 64))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:28acb0c1454566044fe26b51ac43502cbfc9bba5590327273a39da0cb7d626c6") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Base::Anything") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Base::things") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Occurrences::Occurrence") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Occurrences::HappensWhile") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Occurrences::HappensBefore") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind import) (ordinal 5))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Occurrences::HappensJustBefore") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind import) (ordinal 6))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Occurrences::SelfSameLifeLink") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind import) (ordinal 7))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Performances::Performance") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind import) (ordinal 8))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Performances::Evaluation") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind import) (ordinal 9))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Boolean") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind import) (ordinal 10))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::isEmpty") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind import) (ordinal 11))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::equals") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultMonitorPerformance"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "EvaluationResultMonitorPerformance"))))
    (declaration (id (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean") (direction out)) (redefinition (reference "afterValues"))))
    (declaration (id (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind parameter) (ordinal 1))))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean") (direction out)) (redefinition (reference "beforeValues"))))
    (declaration (id (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "FeatureReferencingPerformance"))))
    (declaration (id (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-connector) (ordinal 0))))) (kind kerml-connector) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "HappensJustBefore")) (connectorEnd (reference "monitor1")) (connectorEnd (reference "monitor2"))))
    (declaration (id (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-invariant) (ordinal 0))))) (kind kerml-invariant) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-binding) (ordinal 0))))) (kind kerml-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindSource (reference "monitor1::onOccurrence")) (bindTarget (reference "onOccurrence"))))
    (declaration (id (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-binding) (ordinal 1))))) (kind kerml-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindSource (reference "monitor2::onOccurrence")) (bindTarget (reference "onOccurrence"))))
    (declaration (id (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance::afterValues"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean") (direction out)) (redefinition (reference "values")) (expressionOperand (reference "isToTrue"))))
    (declaration (id (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance::isToTrue"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean"))))
    (declaration (id (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance::monitor1"))) (kind kerml-feature) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "BooleanEvaluationResultMonitorPerformance"))))
    (declaration (id (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "HappensJustBefore")) (redefinition (reference "endWhen"))))
    (declaration (id (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance::monitor1::::earlierOccurrence"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance::monitor1::::laterOccurrence"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance::monitor2"))) (kind kerml-feature) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "BooleanEvaluationResultMonitorPerformance"))))
    (declaration (id (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "HappensJustBefore")) (redefinition (reference "endWhen"))))
    (declaration (id (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance::monitor2::::earlierOccurrence"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance::monitor2::::laterOccurrence"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::EvaluationResultMonitorPerformance"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "FeatureMonitorPerformance"))))
    (declaration (id (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureAccessPerformance"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "FeatureReferencingPerformance"))))
    (declaration (id (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-connector) (ordinal 0))))) (kind kerml-connector) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "HappensWhile")) (connectorEnd (reference "onOccurrence::startingAt::startShot")) (connectorEnd (reference "endShot"))))
    (declaration (id (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-connector) (ordinal 1))))) (kind kerml-connector) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SelfSameLifeLink")) (connectorEnd (reference "onOccurrence::startingAt::accessedFeature")) (connectorEnd (reference "values"))))
    (declaration (id (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureMonitorPerformance"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "FeatureReferencingPerformance"))))
    (declaration (id (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-invariant) (ordinal 0))))) (kind kerml-invariant) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "beforeValues")) (expressionOperand (reference "afterValues"))))
    (declaration (id (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-connector) (ordinal 0))))) (kind kerml-connector) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "HappensWhile")) (connectorEnd (reference "onOccurrence::monitoredOccurrence::beforeTimeSlice::startShot")) (connectorEnd (reference "startShot"))))
    (declaration (id (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-connector) (ordinal 1))))) (kind kerml-connector) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SelfSameLifeLink")) (connectorEnd (reference "onOccurrence::monitoredOccurrence::beforeTimeSlice::monitoredFeature")) (connectorEnd (reference "beforeValues"))))
    (declaration (id (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-connector) (ordinal 2))))) (kind kerml-connector) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SelfSameLifeLink")) (connectorEnd (reference "onOccurrence::monitoredOccurrence::afterSnapshot::monitoredFeature")) (connectorEnd (reference "afterValues"))))
    (declaration (id (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureMonitorPerformance::endWhen"))) (kind kerml-connector) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "HappensBefore")) (connectorEnd (reference "onOccurrence::monitoredOccurrence::afterSnapshot")) (connectorEnd (reference "endShot"))))
    (declaration (id (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureReadEvaluation"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "FeatureAccessPerformance")) (specialization (reference "Evaluation"))))
    (declaration (id (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureReadEvaluation::onOccurrence"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence") (direction in))))
    (declaration (id (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureReadEvaluation::resultValues"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Anything"))))
    (declaration (id (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureReferencingPerformance"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Performance"))))
    (declaration (id (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureWritePerformance"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "FeatureAccessPerformance"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Base::Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "Base::things")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "Occurrences::Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "Occurrences::HappensWhile")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0))
      (authored-target "Occurrences::HappensBefore")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0))
      (authored-target "Occurrences::HappensJustBefore")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0))
      (authored-target "Occurrences::SelfSameLifeLink")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0))
      (authored-target "Performances::Performance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind import) (ordinal 8))))) (kind membershipImport) (ordinal 0))
      (authored-target "Performances::Evaluation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind import) (ordinal 9))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind import) (ordinal 10))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::isEmpty")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind import) (ordinal 11))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::equals")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultMonitorPerformance"))) (kind specialization) (ordinal 0))
      (authored-target "EvaluationResultMonitorPerformance")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::EvaluationResultMonitorPerformance")))))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind parameter) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind parameter) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "afterValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind parameter) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "beforeValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance"))) (kind specialization) (ordinal 0))
      (authored-target "FeatureReferencingPerformance")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureReferencingPerformance")))))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-connector) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "HappensJustBefore")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-connector) (ordinal 0))))) (kind connectorEnd) (ordinal 0))
      (authored-target "monitor1")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance::monitor1")))))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-connector) (ordinal 0))))) (kind connectorEnd) (ordinal 1))
      (authored-target "monitor2")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance::monitor2")))))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-binding) (ordinal 0))))) (kind bindSource) (ordinal 0))
      (authored-target "monitor1::onOccurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-binding) (ordinal 1))))) (kind bindSource) (ordinal 0))
      (authored-target "monitor2::onOccurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-binding) (ordinal 0))))) (kind bindTarget) (ordinal 0))
      (authored-target "onOccurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-binding) (ordinal 1))))) (kind bindTarget) (ordinal 0))
      (authored-target "onOccurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance::afterValues"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance::afterValues"))) (kind redefinition) (ordinal 0))
      (authored-target "values")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance::afterValues"))) (kind expressionOperand) (ordinal 0))
      (authored-target "isToTrue")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance::isToTrue")))))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance::isToTrue"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance::monitor1"))) (kind featureTyping) (ordinal 0))
      (authored-target "BooleanEvaluationResultMonitorPerformance")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultMonitorPerformance")))))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "HappensJustBefore")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "endWhen")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureMonitorPerformance::endWhen")))))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance::monitor2"))) (kind featureTyping) (ordinal 0))
      (authored-target "BooleanEvaluationResultMonitorPerformance")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultMonitorPerformance")))))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "HappensJustBefore")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "endWhen")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureMonitorPerformance::endWhen")))))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::EvaluationResultMonitorPerformance"))) (kind specialization) (ordinal 0))
      (authored-target "FeatureMonitorPerformance")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureMonitorPerformance")))))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureAccessPerformance"))) (kind specialization) (ordinal 0))
      (authored-target "FeatureReferencingPerformance")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureReferencingPerformance")))))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-connector) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "HappensWhile")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-connector) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "SelfSameLifeLink")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-connector) (ordinal 0))))) (kind connectorEnd) (ordinal 0))
      (authored-target "onOccurrence::startingAt::startShot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-connector) (ordinal 1))))) (kind connectorEnd) (ordinal 0))
      (authored-target "onOccurrence::startingAt::accessedFeature")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-connector) (ordinal 0))))) (kind connectorEnd) (ordinal 1))
      (authored-target "endShot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-connector) (ordinal 1))))) (kind connectorEnd) (ordinal 1))
      (authored-target "values")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureMonitorPerformance"))) (kind specialization) (ordinal 0))
      (authored-target "FeatureReferencingPerformance")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureReferencingPerformance")))))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-connector) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "HappensWhile")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-connector) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "SelfSameLifeLink")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-connector) (ordinal 2))))) (kind featureTyping) (ordinal 0))
      (authored-target "SelfSameLifeLink")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-connector) (ordinal 0))))) (kind connectorEnd) (ordinal 0))
      (authored-target "onOccurrence::monitoredOccurrence::beforeTimeSlice::startShot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-connector) (ordinal 1))))) (kind connectorEnd) (ordinal 0))
      (authored-target "onOccurrence::monitoredOccurrence::beforeTimeSlice::monitoredFeature")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-connector) (ordinal 2))))) (kind connectorEnd) (ordinal 0))
      (authored-target "onOccurrence::monitoredOccurrence::afterSnapshot::monitoredFeature")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-connector) (ordinal 0))))) (kind connectorEnd) (ordinal 1))
      (authored-target "startShot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-connector) (ordinal 1))))) (kind connectorEnd) (ordinal 1))
      (authored-target "beforeValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-connector) (ordinal 2))))) (kind connectorEnd) (ordinal 1))
      (authored-target "afterValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-invariant) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "beforeValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-invariant) (ordinal 0))))) (kind expressionOperand) (ordinal 1))
      (authored-target "afterValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureMonitorPerformance::endWhen"))) (kind featureTyping) (ordinal 0))
      (authored-target "HappensBefore")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureMonitorPerformance::endWhen"))) (kind connectorEnd) (ordinal 0))
      (authored-target "onOccurrence::monitoredOccurrence::afterSnapshot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureMonitorPerformance::endWhen"))) (kind connectorEnd) (ordinal 1))
      (authored-target "endShot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureReadEvaluation"))) (kind specialization) (ordinal 0))
      (authored-target "FeatureAccessPerformance")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureAccessPerformance")))))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureReadEvaluation"))) (kind specialization) (ordinal 1))
      (authored-target "Evaluation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureReadEvaluation::onOccurrence"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureReadEvaluation::resultValues"))) (kind featureTyping) (ordinal 0))
      (authored-target "Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureReferencingPerformance"))) (kind specialization) (ordinal 0))
      (authored-target "Performance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureWritePerformance"))) (kind specialization) (ordinal 0))
      (authored-target "FeatureAccessPerformance")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureAccessPerformance")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultMonitorPerformance"))) (target (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::EvaluationResultMonitorPerformance"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultMonitorPerformance"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance"))) (target (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureReferencingPerformance"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance"))) (kind specialization) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-connector) (ordinal 0))))) (target (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance::monitor1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-connector) (ordinal 0))))) (kind connectorEnd) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-connector) (ordinal 0))))) (target (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance::monitor2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-connector) (ordinal 0))))) (kind connectorEnd) (ordinal 1)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance::afterValues"))) (target (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance::isToTrue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance::afterValues"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance::monitor1"))) (target (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultMonitorPerformance"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance::monitor1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureMonitorPerformance::endWhen"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance::monitor2"))) (target (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultMonitorPerformance"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance::monitor2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureMonitorPerformance::endWhen"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::EvaluationResultMonitorPerformance"))) (target (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureMonitorPerformance"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::EvaluationResultMonitorPerformance"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureAccessPerformance"))) (target (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureReferencingPerformance"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureAccessPerformance"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureMonitorPerformance"))) (target (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureReferencingPerformance"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureMonitorPerformance"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureReadEvaluation"))) (target (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureAccessPerformance"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureReadEvaluation"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureWritePerformance"))) (target (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureAccessPerformance"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureWritePerformance"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance::afterValues"))) (value (kind boolean) (boolean true)))
    (evaluated (declaration (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance::isToTrue"))) (value (kind boolean) (boolean true)))
    (evaluated (declaration (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-invariant) (ordinal 0))))) (value (kind non-constant)))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 7 16) (end 7 30)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Base::Anything")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 8 16) (end 8 28)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "Base::things")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 9 16) (end 9 39)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "Occurrences::Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 10 16) (end 10 41)) (probe (position 10 16))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "Occurrences::HappensWhile")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 11 16) (end 11 42)) (probe (position 11 16))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0) (authored-target "Occurrences::HappensBefore")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 12 16) (end 12 46)) (probe (position 12 16))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0) (authored-target "Occurrences::HappensJustBefore")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 13 16) (end 13 45)) (probe (position 13 16))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0) (authored-target "Occurrences::SelfSameLifeLink")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 14 16) (end 14 41)) (probe (position 14 16))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0) (authored-target "Performances::Performance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 15 16) (end 15 40)) (probe (position 15 16))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind import) (ordinal 8))))) (kind membershipImport) (ordinal 0) (authored-target "Performances::Evaluation")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 16 16) (end 16 37)) (probe (position 16 16))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind import) (ordinal 9))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 17 16) (end 17 42)) (probe (position 17 16))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind import) (ordinal 10))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::isEmpty")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 18 16) (end 18 41)) (probe (position 18 16))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind import) (ordinal 11))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::equals")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 141 64) (end 141 98)) (probe (position 141 64))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultMonitorPerformance"))) (kind specialization) (ordinal 0) (authored-target "EvaluationResultMonitorPerformance")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::EvaluationResultMonitorPerformance")))))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 153 30) (end 153 37)) (probe (position 153 30))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 154 31) (end 154 38)) (probe (position 154 31))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind parameter) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 153 16) (end 153 27)) (probe (position 153 16))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind parameter) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "afterValues")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 154 16) (end 154 28)) (probe (position 154 16))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind parameter) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "beforeValues")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 157 66) (end 157 95)) (probe (position 157 66))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance"))) (kind specialization) (ordinal 0) (authored-target "FeatureReferencingPerformance")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureReferencingPerformance")))))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 183 24) (end 183 41)) (probe (position 183 24))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-connector) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "HappensJustBefore")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 183 51) (end 183 59)) (probe (position 183 51))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-connector) (ordinal 0))))) (kind connectorEnd) (ordinal 0) (authored-target "monitor1")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance::monitor1")))))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 183 70) (end 183 78)) (probe (position 183 70))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-connector) (ordinal 0))))) (kind connectorEnd) (ordinal 1) (authored-target "monitor2")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance::monitor2")))))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 186 24) (end 186 45)) (probe (position 186 24))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-binding) (ordinal 0))))) (kind bindSource) (ordinal 0) (authored-target "monitor1::onOccurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 187 24) (end 187 45)) (probe (position 187 24))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-binding) (ordinal 1))))) (kind bindSource) (ordinal 0) (authored-target "monitor2::onOccurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 186 52) (end 186 64)) (probe (position 186 52))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-binding) (ordinal 0))))) (kind bindTarget) (ordinal 0) (authored-target "onOccurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 187 52) (end 187 64)) (probe (position 187 52))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-binding) (ordinal 1))))) (kind bindTarget) (ordinal 0) (authored-target "onOccurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 169 21) (end 169 28)) (probe (position 169 21))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance::afterValues"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 169 42) (end 169 48)) (probe (position 169 42))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance::afterValues"))) (kind redefinition) (ordinal 0) (authored-target "values")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 169 52) (end 169 60)) (probe (position 169 52))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance::afterValues"))) (kind expressionOperand) (ordinal 0) (authored-target "isToTrue")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance::isToTrue")))))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 168 23) (end 168 30)) (probe (position 168 23))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance::isToTrue"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 170 31) (end 170 72)) (probe (position 170 31))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance::monitor1"))) (kind featureTyping) (ordinal 0) (authored-target "BooleanEvaluationResultMonitorPerformance")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultMonitorPerformance")))))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 171 34) (end 171 51)) (probe (position 171 34))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "HappensJustBefore")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 171 24) (end 171 31)) (probe (position 171 24))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "endWhen")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureMonitorPerformance::endWhen")))))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 176 31) (end 176 72)) (probe (position 176 31))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultToMonitorPerformance::monitor2"))) (kind featureTyping) (ordinal 0) (authored-target "BooleanEvaluationResultMonitorPerformance")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::BooleanEvaluationResultMonitorPerformance")))))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 177 34) (end 177 51)) (probe (position 177 34))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "HappensJustBefore")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 177 24) (end 177 31)) (probe (position 177 24))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "endWhen")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureMonitorPerformance::endWhen")))))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 125 57) (end 125 82)) (probe (position 125 57))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::EvaluationResultMonitorPerformance"))) (kind specialization) (ordinal 0) (authored-target "FeatureMonitorPerformance")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureMonitorPerformance")))))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 43 59) (end 43 88)) (probe (position 43 59))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureAccessPerformance"))) (kind specialization) (ordinal 0) (authored-target "FeatureReferencingPerformance")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureReferencingPerformance")))))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 57 16) (end 57 28)) (probe (position 57 16))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-connector) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "HappensWhile")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 66 16) (end 66 32)) (probe (position 66 16))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-connector) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "SelfSameLifeLink")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 57 34) (end 57 67)) (probe (position 57 34))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-connector) (ordinal 0))))) (kind connectorEnd) (ordinal 0) (authored-target "onOccurrence::startingAt::startShot")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 66 38) (end 66 77)) (probe (position 66 38))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-connector) (ordinal 1))))) (kind connectorEnd) (ordinal 0) (authored-target "onOccurrence::startingAt::accessedFeature")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 57 71) (end 57 78)) (probe (position 57 71))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-connector) (ordinal 0))))) (kind connectorEnd) (ordinal 1) (authored-target "endShot")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 66 81) (end 66 87)) (probe (position 66 81))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-connector) (ordinal 1))))) (kind connectorEnd) (ordinal 1) (authored-target "values")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 91 57) (end 91 86)) (probe (position 91 57))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureMonitorPerformance"))) (kind specialization) (ordinal 0) (authored-target "FeatureReferencingPerformance")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureReferencingPerformance")))))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 115 24) (end 115 36)) (probe (position 115 24))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-connector) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "HappensWhile")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 117 24) (end 117 40)) (probe (position 117 24))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-connector) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "SelfSameLifeLink")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 119 24) (end 119 40)) (probe (position 119 24))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-connector) (ordinal 2))))) (kind featureTyping) (ordinal 0) (authored-target "SelfSameLifeLink")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 116 14) (end 116 72)) (probe (position 116 14))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-connector) (ordinal 0))))) (kind connectorEnd) (ordinal 0) (authored-target "onOccurrence::monitoredOccurrence::beforeTimeSlice::startShot")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 118 14) (end 118 79)) (probe (position 118 14))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-connector) (ordinal 1))))) (kind connectorEnd) (ordinal 0) (authored-target "onOccurrence::monitoredOccurrence::beforeTimeSlice::monitoredFeature")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 120 14) (end 120 77)) (probe (position 120 14))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-connector) (ordinal 2))))) (kind connectorEnd) (ordinal 0) (authored-target "onOccurrence::monitoredOccurrence::afterSnapshot::monitoredFeature")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 116 80) (end 116 89)) (probe (position 116 80))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-connector) (ordinal 0))))) (kind connectorEnd) (ordinal 1) (authored-target "startShot")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 118 87) (end 118 99)) (probe (position 118 87))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-connector) (ordinal 1))))) (kind connectorEnd) (ordinal 1) (authored-target "beforeValues")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 120 85) (end 120 96)) (probe (position 120 85))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-connector) (ordinal 2))))) (kind connectorEnd) (ordinal 1) (authored-target "afterValues")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 113 14) (end 113 26)) (probe (position 113 14))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-invariant) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "beforeValues")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 113 35) (end 113 46)) (probe (position 113 35))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (anonymous (kind kerml-invariant) (ordinal 0))))) (kind expressionOperand) (ordinal 1) (authored-target "afterValues")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 121 34) (end 121 47)) (probe (position 121 34))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureMonitorPerformance::endWhen"))) (kind featureTyping) (ordinal 0) (authored-target "HappensBefore")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 122 14) (end 122 60)) (probe (position 122 14))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureMonitorPerformance::endWhen"))) (kind connectorEnd) (ordinal 0) (authored-target "onOccurrence::monitoredOccurrence::afterSnapshot")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 122 68) (end 122 75)) (probe (position 122 68))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureMonitorPerformance::endWhen"))) (kind connectorEnd) (ordinal 1) (authored-target "endShot")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 69 53) (end 69 77)) (probe (position 69 53))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureReadEvaluation"))) (kind specialization) (ordinal 0) (authored-target "FeatureAccessPerformance")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureAccessPerformance")))))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 69 79) (end 69 89)) (probe (position 69 79))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureReadEvaluation"))) (kind specialization) (ordinal 1) (authored-target "Evaluation")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 76 19) (end 76 29)) (probe (position 76 19))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureReadEvaluation::onOccurrence"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 77 24) (end 77 32)) (probe (position 77 24))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureReadEvaluation::resultValues"))) (kind featureTyping) (ordinal 0) (authored-target "Anything")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 20 61) (end 20 72)) (probe (position 20 61))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureReferencingPerformance"))) (kind specialization) (ordinal 0) (authored-target "Performance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_referencing_performances.md") (range (start 80 55) (end 80 79)) (probe (position 80 55))
    (reference (id (source (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureWritePerformance"))) (kind specialization) (ordinal 0) (authored-target "FeatureAccessPerformance")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureAccessPerformance")))))
  )
)
~~~
