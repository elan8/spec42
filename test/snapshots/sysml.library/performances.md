# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Semantic Library/Performances
type=file
~~~
# SOURCE
~~~kerml
standard library package Performances {
	doc
	/*
	 * This package defines classifiers and features that related to the typing of performances and evaluations.
	 */

	private import Base::Anything;
	private import Base::things;
	private import Occurrences::Occurrence;
	private import Occurrences::occurrences;
	private import Occurrences::HappensDuring;
	private import Objects::Object;
	private import Links::BinaryLink;
	private import Metaobjects::Metaobject;
	private import Transfers::Transfer;
	private import Transfers::transfers;
	private import Transfers::TransferBefore;
	private import Transfers::transfersBefore;
	private import ScalarValues::*;
	private import SequenceFunctions::includes;
	
	abstract behavior Performance specializes Occurrence disjoint from Object {
		doc
		/*
		 * Performance is the most general class of behavioral Occurrences that may be performed over time.
		 */

		feature self: Performance redefines Occurrence::self;
		
		feature involvedObjects: Object[0..*] {
			doc
			/*
			 * Objects that are involved in this Performance.
			 */
		}
		
		feature performers: Object[0..*] subsets involvedObjects {
			doc
			/*
			 * Objects that enact this Performance.
			 */
		}
		
		feature redefines isDispatch default true;
  		feature redefines dispatchScope default thisPerformance;
  		
		step enclosedPerformances: Performance[0..*] subsets performances, timeEnclosedOccurrences
			intersects performances, timeEnclosedOccurrences {
			doc
			/*
			 * timeEnclosedOccurrences of this Performance that are also Performances.
			 */
		}
		
		feature thisPerformance: Performance [1] default self {
			doc
			/*
			 * Defaults to the root of the subperformance composition tree.
			 */
		}
		connector :HappensDuring from [1] self to [1] thisPerformance; 
		
		composite step subperformances: Performance[0..*] subsets enclosedPerformances, suboccurrences
			intersects enclosedPerformances, suboccurrences {
			doc
			/*
			 * enclosedPerformances that are composite. 
			 */
		
			feature redefines this default (that as Performance).this {
				doc
				/*
				 * The default "this" context of a subperformance is the same as that of its owning Performance.
				 * This means that the context for any Performance that is in a composition tree rooted
				 * in a Performance that is not itself owned by an Object is the root Performance. If the
				 * root Performance is an ownedPerformance of an Object, then that Object is the context.
				 */
			}
		
			feature redefines thisPerformance default (that as Performance).thisPerformance;
		}		
	}
	
	abstract function Evaluation specializes Performance {
		doc
		/*
		 * Evaluation is the most general class of functions that may be evaluated to compute
		 * a result.
		 */
	 
		return result: Anything[0..*] nonunique;
	}
	
	abstract predicate BooleanEvaluation specializes Evaluation {
		doc
		/*
		 * BooleanEvaluation is a specialization of Evaluation that is the most general class of
		 * Predicates that may be evaluated to produce a Boolean truth value.
		 */
	 
		return : Boolean[1];
	}
	
	abstract function MetadataAccessEvaluation specializes Evaluation {
		doc
		/*
		 * MetadataAccessEvaluation is a specialization of Evaluation for the case of MetadataAccessExpressions.
		 */
		
		return : Metaobject[1..*];
	}
	
	abstract function LiteralEvaluation specializes Evaluation {
		doc
		/*
		 * LiteralEvaluation is a specialization of Evaluation for the case of LiteralExpressions.
		 */
	 
		return : ScalarValue[1];
	}
	
	abstract predicate LiteralBooleanEvaluation specializes LiteralEvaluation, BooleanEvaluation
		intersects LiteralEvaluation, BooleanEvaluation {
		doc
		/*
		 * LiteralBooleanEvaluation is a specialization of LiteralEvaluation for the case of LiteralBooleans.
		 * It is also a predicate and thus a specialization of BooleanEvaluation. 
		 */
	 
		return : Boolean[1];
	}
	abstract function LiteralIntegerEvaluation specializes LiteralEvaluation {
		doc
		/*
		 * LiteralIntegerEvaluation is a specialization of LiteralEvaluation for the case of LiteralIntegers.
		 */
	 
		return : Integer[1];
	}

	abstract function LiteralRationalEvaluation specializes LiteralEvaluation {
		doc
		/*
		 * LiteralRationalEvaluation is a specialization of LiteralEvaluation for the case of LiteralRationals.
		 * (Note: Return type is Real to allow easy type conformance of LiteralRationals when a Real result is expected.)
		 */
	 
		return : Real[1];
	}
	
	abstract function LiteralStringEvaluation specializes LiteralEvaluation {
		doc
		/*
		 * LiteralStringEvaluation is a specialization of LiteralEvaluation for the case of LiteralStrings.
		 */
	 
		return : String[1];
	}
	
	function NullEvaluation specializes Evaluation {
		doc
		/*
		 * NullEvaluation is a specialization of Evaluation for the case of NullExpressions.
		 */
	 
		return : Anything[0..0];
	}

	assoc all InvolvedIn specializes BinaryLink { 
		doc
		/*
		 * InvolvedIn asserts that the involvedObject is involved in the Behavior carried out by the 
		 * involvingPerformance.
		 */
		 
		end feature involvedObject: Object redefines source crosses involvingPerformance.involvedObjects;
		end feature involvingPerformance: Performance redefines target crosses involvedObject.involvingPerformances;
	}
	
	assoc all Performs specializes InvolvedIn {
		doc
		/*
		 * Performs asserts that the performer enacts the Behavior carried out by the performance.
		 */
	
	 	end feature performerObject: Object redefines involvedObject crosses performance.performers;
	 	end feature performance: Performance redefines involvingPerformance crosses performerObject.enactedPerformances;
	 }

	abstract step performances: Performance[0..*] nonunique subsets occurrences {
		doc
		/*
		 * performances is the most general feature for performances of Behaviors.
		 */
	}
	
	abstract expr evaluations: Evaluation[0..*] nonunique subsets performances {
		doc
		/*
		 * evaluations is a specialization of performances for evaluations of Functions.
		 */
	}
	
	abstract expr constructorEvaluations [0..*] nonunique subsets evaluations  {
	    doc
	    /*
	     * constructorEvaluations is a specialization of evaluations that restricts the multiplicity 
	     * of its result parameter to 1..1, requiring a constructorEvaluation to result in a single value.
	     */
	     
	     return result [1..1];
	}
	
	abstract expr booleanEvaluations: BooleanEvaluation[0..*] nonunique subsets evaluations {
		doc
		/*
		 * booleanEvaluations is a specialization of evaluations restricted to type BooleanEvaluation.
		 */
	}
	
	abstract expr trueEvaluations subsets booleanEvaluations {
		doc
		/*
		 * trueEvaluations is a subset of booleanEvaluations that result in true. It is the most general
		 * feature of invariants that are not negated. 
		 */
	
		private feature trueValue = true;
		binding result = trueValue;
	}
	
	abstract expr falseEvaluations subsets booleanEvaluations {
		doc
		/*
		 * falseEvaluations is a subset of booleanEvaluations that result in false. It is the most general
		 * feature of invariants that are negated.
		 */
	
        private feature falseValue = false;
        binding result = falseValue;
	}
	
	abstract expr metadataAccessEvaluations: MetadataAccessEvaluation[0..*] nonunique subsets evaluations {
		doc
		/*
		 * metadataAccessEvaluations is a specialization of evaluations restricted to type MetadataAccessEvaluation. 
		 */
	}
	
	abstract expr literalEvaluations: LiteralEvaluation[0..*] nonunique subsets evaluations {
		doc
		/*
		 * literalEvaluations is a specialization of evaluations restricted to type LiteralEvaluation.
		 */
	}
	
	abstract expr literalBooleanEvaluations: LiteralBooleanEvaluation[0..*] nonunique subsets literalEvaluations, booleanEvaluations
		intersects literalEvaluations, booleanEvaluations {
		doc
		/*
		 * literaBooleanlEvaluations is a specialization of literalEvaluations and booleanEvaluations restricted 
		 * to type LiteralBooleanEvaluation.
		 */
	}
	
	abstract expr literalIntegerEvaluations: LiteralIntegerEvaluation[0..*] nonunique subsets literalEvaluations {
		doc
		/*
		 * literalEvaluations is a specialization of evaluations restricted to type LiteralEvaluation.
		 */
	}
	
	abstract expr literalRationalEvaluations: LiteralRationalEvaluation[0..*] nonunique subsets literalEvaluations {
		doc
		/*
		 * literalRationalEvaluations is a specialization of literalEvaluations restricted to type LiteralRationalEvaluation.
		 */
	}
	
	abstract expr literalStringEvaluations: LiteralStringEvaluation[0..*] nonunique subsets literalEvaluations {
		doc
		/*
		 * literalStringEvaluations is a specialization of literalEvaluations restricted to type LiteralStringEvaluation.
		 */
	}
	
	abstract expr nullEvaluations: NullEvaluation[0..*] nonunique subsets evaluations {
		doc
		/*
		 * nullEvaluations is a specialization of evaluations restricted to type NullEvaluation.
		 */
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "performances.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 16) (end 6 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 28))
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
        (range (start 9 16) (end 9 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 16) (end 10 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 16) (end 11 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 12 16) (end 12 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 13 16) (end 13 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 14 16) (end 14 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 15 16) (end 15 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 16 16) (end 16 41))
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
        (range (start 18 16) (end 18 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 19 16) (end 19 43))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package Performances {
	doc
	/*
	 * This package defines classifiers and features that related to the typing of performances and evaluations.
	 */

	private import Base::Anything;
	private import Base::things;
	private import Occurrences::Occurrence;
	private import Occurrences::occurrences;
	private import Occurrences::HappensDuring;
	private import Objects::Object;
	private import Links::BinaryLink;
	private import Metaobjects::Metaobject;
	private import Transfers::Transfer;
	private import Transfers::transfers;
	private import Transfers::TransferBefore;
	private import Transfers::transfersBefore;
	private import ScalarValues::*;
	private import SequenceFunctions::includes;
	
	abstract behavior Performance specializes Occurrence disjoint from Object {
		doc
		/*
		 * Performance is the most general class of behavioral Occurrences that may be performed over time.
		 */

		feature self: Performance redefines Occurrence::self;
		
		feature involvedObjects: Object[0..*] {
			doc
			/*
			 * Objects that are involved in this Performance.
			 */
		}
		
		feature performers: Object[0..*] subsets involvedObjects {
			doc
			/*
			 * Objects that enact this Performance.
			 */
		}
		
		feature redefines isDispatch default true;
  		feature redefines dispatchScope default thisPerformance;
  		
		step enclosedPerformances: Performance[0..*] subsets performances, timeEnclosedOccurrences
			intersects performances, timeEnclosedOccurrences {
			doc
			/*
			 * timeEnclosedOccurrences of this Performance that are also Performances.
			 */
		}
		
		feature thisPerformance: Performance [1] default self {
			doc
			/*
			 * Defaults to the root of the subperformance composition tree.
			 */
		}
		connector :HappensDuring from [1] self to [1] thisPerformance; 
		
		composite step subperformances: Performance[0..*] subsets enclosedPerformances, suboccurrences
			intersects enclosedPerformances, suboccurrences {
			doc
			/*
			 * enclosedPerformances that are composite. 
			 */
		
			feature redefines this default (that as Performance).this {
				doc
				/*
				 * The default "this" context of a subperformance is the same as that of its owning Performance.
				 * This means that the context for any Performance that is in a composition tree rooted
				 * in a Performance that is not itself owned by an Object is the root Performance. If the
				 * root Performance is an ownedPerformance of an Object, then that Object is the context.
				 */
			}
		
			feature redefines thisPerformance default (that as Performance).thisPerformance;
		}		
	}
	
	abstract function Evaluation specializes Performance {
		doc
		/*
		 * Evaluation is the most general class of functions that may be evaluated to compute
		 * a result.
		 */
	 
		return result: Anything[0..*] nonunique;
	}
	
	abstract predicate BooleanEvaluation specializes Evaluation {
		doc
		/*
		 * BooleanEvaluation is a specialization of Evaluation that is the most general class of
		 * Predicates that may be evaluated to produce a Boolean truth value.
		 */
	 
		return : Boolean[1];
	}
	
	abstract function MetadataAccessEvaluation specializes Evaluation {
		doc
		/*
		 * MetadataAccessEvaluation is a specialization of Evaluation for the case of MetadataAccessExpressions.
		 */
		
		return : Metaobject[1..*];
	}
	
	abstract function LiteralEvaluation specializes Evaluation {
		doc
		/*
		 * LiteralEvaluation is a specialization of Evaluation for the case of LiteralExpressions.
		 */
	 
		return : ScalarValue[1];
	}
	
	abstract predicate LiteralBooleanEvaluation specializes LiteralEvaluation, BooleanEvaluation
		intersects LiteralEvaluation, BooleanEvaluation {
		doc
		/*
		 * LiteralBooleanEvaluation is a specialization of LiteralEvaluation for the case of LiteralBooleans.
		 * It is also a predicate and thus a specialization of BooleanEvaluation. 
		 */
	 
		return : Boolean[1];
	}
	abstract function LiteralIntegerEvaluation specializes LiteralEvaluation {
		doc
		/*
		 * LiteralIntegerEvaluation is a specialization of LiteralEvaluation for the case of LiteralIntegers.
		 */
	 
		return : Integer[1];
	}

	abstract function LiteralRationalEvaluation specializes LiteralEvaluation {
		doc
		/*
		 * LiteralRationalEvaluation is a specialization of LiteralEvaluation for the case of LiteralRationals.
		 * (Note: Return type is Real to allow easy type conformance of LiteralRationals when a Real result is expected.)
		 */
	 
		return : Real[1];
	}
	
	abstract function LiteralStringEvaluation specializes LiteralEvaluation {
		doc
		/*
		 * LiteralStringEvaluation is a specialization of LiteralEvaluation for the case of LiteralStrings.
		 */
	 
		return : String[1];
	}
	
	function NullEvaluation specializes Evaluation {
		doc
		/*
		 * NullEvaluation is a specialization of Evaluation for the case of NullExpressions.
		 */
	 
		return : Anything[0..0];
	}

	assoc all InvolvedIn specializes BinaryLink { 
		doc
		/*
		 * InvolvedIn asserts that the involvedObject is involved in the Behavior carried out by the 
		 * involvingPerformance.
		 */
		 
		end feature involvedObject: Object redefines source crosses involvingPerformance.involvedObjects;
		end feature involvingPerformance: Performance redefines target crosses involvedObject.involvingPerformances;
	}
	
	assoc all Performs specializes InvolvedIn {
		doc
		/*
		 * Performs asserts that the performer enacts the Behavior carried out by the performance.
		 */
	
	 	end feature performerObject: Object redefines involvedObject crosses performance.performers;
	 	end feature performance: Performance redefines involvingPerformance crosses performerObject.enactedPerformances;
	 }

	abstract step performances: Performance[0..*] nonunique subsets occurrences {
		doc
		/*
		 * performances is the most general feature for performances of Behaviors.
		 */
	}
	
	abstract expr evaluations: Evaluation[0..*] nonunique subsets performances {
		doc
		/*
		 * evaluations is a specialization of performances for evaluations of Functions.
		 */
	}
	
	abstract expr constructorEvaluations [0..*] nonunique subsets evaluations  {
	    doc
	    /*
	     * constructorEvaluations is a specialization of evaluations that restricts the multiplicity 
	     * of its result parameter to 1..1, requiring a constructorEvaluation to result in a single value.
	     */
	     
	     return result [1..1];
	}
	
	abstract expr booleanEvaluations: BooleanEvaluation[0..*] nonunique subsets evaluations {
		doc
		/*
		 * booleanEvaluations is a specialization of evaluations restricted to type BooleanEvaluation.
		 */
	}
	
	abstract expr trueEvaluations subsets booleanEvaluations {
		doc
		/*
		 * trueEvaluations is a subset of booleanEvaluations that result in true. It is the most general
		 * feature of invariants that are not negated. 
		 */
	
		private feature trueValue = true;
		binding result = trueValue;
	}
	
	abstract expr falseEvaluations subsets booleanEvaluations {
		doc
		/*
		 * falseEvaluations is a subset of booleanEvaluations that result in false. It is the most general
		 * feature of invariants that are negated.
		 */
	
        private feature falseValue = false;
        binding result = falseValue;
	}
	
	abstract expr metadataAccessEvaluations: MetadataAccessEvaluation[0..*] nonunique subsets evaluations {
		doc
		/*
		 * metadataAccessEvaluations is a specialization of evaluations restricted to type MetadataAccessEvaluation. 
		 */
	}
	
	abstract expr literalEvaluations: LiteralEvaluation[0..*] nonunique subsets evaluations {
		doc
		/*
		 * literalEvaluations is a specialization of evaluations restricted to type LiteralEvaluation.
		 */
	}
	
	abstract expr literalBooleanEvaluations: LiteralBooleanEvaluation[0..*] nonunique subsets literalEvaluations, booleanEvaluations
		intersects literalEvaluations, booleanEvaluations {
		doc
		/*
		 * literaBooleanlEvaluations is a specialization of literalEvaluations and booleanEvaluations restricted 
		 * to type LiteralBooleanEvaluation.
		 */
	}
	
	abstract expr literalIntegerEvaluations: LiteralIntegerEvaluation[0..*] nonunique subsets literalEvaluations {
		doc
		/*
		 * literalEvaluations is a specialization of evaluations restricted to type LiteralEvaluation.
		 */
	}
	
	abstract expr literalRationalEvaluations: LiteralRationalEvaluation[0..*] nonunique subsets literalEvaluations {
		doc
		/*
		 * literalRationalEvaluations is a specialization of literalEvaluations restricted to type LiteralRationalEvaluation.
		 */
	}
	
	abstract expr literalStringEvaluations: LiteralStringEvaluation[0..*] nonunique subsets literalEvaluations {
		doc
		/*
		 * literalStringEvaluations is a specialization of literalEvaluations restricted to type LiteralStringEvaluation.
		 */
	}
	
	abstract expr nullEvaluations: NullEvaluation[0..*] nonunique subsets evaluations {
		doc
		/*
		 * nullEvaluations is a specialization of evaluations restricted to type NullEvaluation.
		 */
	}
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "c95ff90f4873b53551bab11949ce68130fc2f40e4203b027ef3f419cb5be1d0a") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Performances"))) (kind "package") (name "Performances") (declared-name "Performances") (range (start (line 0) (character 0)) (end (line 0) (character 8925))))
    (element (id (node (document "d0") (qualified-name "Performances::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 18) (character 1)) (end (line 18) (character 32))) (parent (node (document "d0") (qualified-name "Performances"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 18) (character 16)) (end (line 18) (character 28))))))
    (element (id (node (document "d0") (qualified-name "Performances::Anything"))) (kind "import") (name "Anything") (declared-name "Anything") (range (start (line 6) (character 1)) (end (line 6) (character 31))) (parent (node (document "d0") (qualified-name "Performances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Base::Anything") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 6) (character 16)) (end (line 6) (character 30))))))
    (element (id (node (document "d0") (qualified-name "Performances::BinaryLink"))) (kind "import") (name "BinaryLink") (declared-name "BinaryLink") (range (start (line 12) (character 1)) (end (line 12) (character 34))) (parent (node (document "d0") (qualified-name "Performances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Links::BinaryLink") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 12) (character 16)) (end (line 12) (character 33))))))
    (element (id (node (document "d0") (qualified-name "Performances::BooleanEvaluation"))) (kind "kermlDecl") (name "BooleanEvaluation") (declared-name "BooleanEvaluation") (range (start (line 93) (character 1)) (end (line 93) (character 271))) (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::Evaluation"))) (kind "kermlDecl") (name "Evaluation") (declared-name "Evaluation") (range (start (line 83) (character 1)) (end (line 83) (character 224))) (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::HappensDuring"))) (kind "import") (name "HappensDuring") (declared-name "HappensDuring") (range (start (line 10) (character 1)) (end (line 10) (character 43))) (parent (node (document "d0") (qualified-name "Performances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::HappensDuring") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 10) (character 16)) (end (line 10) (character 42))))))
    (element (id (node (document "d0") (qualified-name "Performances::LiteralBooleanEvaluation"))) (kind "kermlDecl") (name "LiteralBooleanEvaluation") (declared-name "LiteralBooleanEvaluation") (range (start (line 121) (character 1)) (end (line 121) (character 372))) (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::LiteralEvaluation"))) (kind "kermlDecl") (name "LiteralEvaluation") (declared-name "LiteralEvaluation") (range (start (line 112) (character 1)) (end (line 112) (character 204))) (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::LiteralIntegerEvaluation"))) (kind "kermlDecl") (name "LiteralIntegerEvaluation") (declared-name "LiteralIntegerEvaluation") (range (start (line 131) (character 1)) (end (line 131) (character 225))) (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::LiteralRationalEvaluation"))) (kind "kermlDecl") (name "LiteralRationalEvaluation") (declared-name "LiteralRationalEvaluation") (range (start (line 140) (character 1)) (end (line 140) (character 341))) (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::LiteralStringEvaluation"))) (kind "kermlDecl") (name "LiteralStringEvaluation") (declared-name "LiteralStringEvaluation") (range (start (line 150) (character 1)) (end (line 150) (character 221))) (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::MetadataAccessEvaluation"))) (kind "kermlDecl") (name "MetadataAccessEvaluation") (declared-name "MetadataAccessEvaluation") (range (start (line 103) (character 1)) (end (line 103) (character 227))) (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::Metaobject"))) (kind "import") (name "Metaobject") (declared-name "Metaobject") (range (start (line 13) (character 1)) (end (line 13) (character 40))) (parent (node (document "d0") (qualified-name "Performances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Metaobjects::Metaobject") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 13) (character 16)) (end (line 13) (character 39))))))
    (element (id (node (document "d0") (qualified-name "Performances::NullEvaluation"))) (kind "kermlDecl") (name "NullEvaluation") (declared-name "NullEvaluation") (range (start (line 159) (character 1)) (end (line 159) (character 186))) (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::Object"))) (kind "import") (name "Object") (declared-name "Object") (range (start (line 11) (character 1)) (end (line 11) (character 32))) (parent (node (document "d0") (qualified-name "Performances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Objects::Object") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 11) (character 16)) (end (line 11) (character 31))))))
    (element (id (node (document "d0") (qualified-name "Performances::Occurrence"))) (kind "import") (name "Occurrence") (declared-name "Occurrence") (range (start (line 8) (character 1)) (end (line 8) (character 40))) (parent (node (document "d0") (qualified-name "Performances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::Occurrence") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 8) (character 16)) (end (line 8) (character 39))))))
    (element (id (node (document "d0") (qualified-name "Performances::Performance"))) (kind "kermlDecl") (name "Performance") (declared-name "Performance") (range (start (line 21) (character 1)) (end (line 21) (character 1878))) (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::Transfer"))) (kind "import") (name "Transfer") (declared-name "Transfer") (range (start (line 14) (character 1)) (end (line 14) (character 36))) (parent (node (document "d0") (qualified-name "Performances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::Transfer") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 14) (character 16)) (end (line 14) (character 35))))))
    (element (id (node (document "d0") (qualified-name "Performances::TransferBefore"))) (kind "import") (name "TransferBefore") (declared-name "TransferBefore") (range (start (line 16) (character 1)) (end (line 16) (character 42))) (parent (node (document "d0") (qualified-name "Performances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::TransferBefore") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 16) (character 16)) (end (line 16) (character 41))))))
    (element (id (node (document "d0") (qualified-name "Performances::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 8925))) (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::all"))) (kind "kermlDecl") (name "all") (declared-name "all") (range (start (line 168) (character 1)) (end (line 168) (character 405))) (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::all#kermlDecl"))) (kind "kermlDecl") (name "all") (declared-name "all") (range (start (line 179) (character 1)) (end (line 179) (character 372))) (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::booleanEvaluations"))) (kind "kermlDecl") (name "booleanEvaluations") (declared-name "booleanEvaluations") (range (start (line 213) (character 1)) (end (line 213) (character 207))) (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::constructorEvaluations"))) (kind "kermlDecl") (name "constructorEvaluations") (declared-name "constructorEvaluations") (range (start (line 203) (character 1)) (end (line 203) (character 344))) (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::evaluations"))) (kind "kermlDecl") (name "evaluations") (declared-name "evaluations") (range (start (line 196) (character 1)) (end (line 196) (character 180))) (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::falseEvaluations"))) (kind "kermlDecl") (name "falseEvaluations") (declared-name "falseEvaluations") (range (start (line 231) (character 1)) (end (line 231) (character 309))) (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::includes"))) (kind "import") (name "includes") (declared-name "includes") (range (start (line 19) (character 1)) (end (line 19) (character 44))) (parent (node (document "d0") (qualified-name "Performances"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::includes") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 19) (character 16)) (end (line 19) (character 43))))))
    (element (id (node (document "d0") (qualified-name "Performances::literalBooleanEvaluations"))) (kind "kermlDecl") (name "literalBooleanEvaluations") (declared-name "literalBooleanEvaluations") (range (start (line 256) (character 1)) (end (line 256) (character 350))) (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::literalEvaluations"))) (kind "kermlDecl") (name "literalEvaluations") (declared-name "literalEvaluations") (range (start (line 249) (character 1)) (end (line 249) (character 207))) (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::literalIntegerEvaluations"))) (kind "kermlDecl") (name "literalIntegerEvaluations") (declared-name "literalIntegerEvaluations") (range (start (line 265) (character 1)) (end (line 265) (character 228))) (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::literalRationalEvaluations"))) (kind "kermlDecl") (name "literalRationalEvaluations") (declared-name "literalRationalEvaluations") (range (start (line 272) (character 1)) (end (line 272) (character 253))) (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::literalStringEvaluations"))) (kind "kermlDecl") (name "literalStringEvaluations") (declared-name "literalStringEvaluations") (range (start (line 279) (character 1)) (end (line 279) (character 245))) (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::metadataAccessEvaluations"))) (kind "kermlDecl") (name "metadataAccessEvaluations") (declared-name "metadataAccessEvaluations") (range (start (line 242) (character 1)) (end (line 242) (character 236))) (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::nullEvaluations"))) (kind "kermlDecl") (name "nullEvaluations") (declared-name "nullEvaluations") (range (start (line 286) (character 1)) (end (line 286) (character 195))) (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::occurrences"))) (kind "import") (name "occurrences") (declared-name "occurrences") (range (start (line 9) (character 1)) (end (line 9) (character 41))) (parent (node (document "d0") (qualified-name "Performances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::occurrences") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 9) (character 16)) (end (line 9) (character 40))))))
    (element (id (node (document "d0") (qualified-name "Performances::performances"))) (kind "kermlDecl") (name "performances") (declared-name "performances") (range (start (line 189) (character 1)) (end (line 189) (character 175))) (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::things"))) (kind "import") (name "things") (declared-name "things") (range (start (line 7) (character 1)) (end (line 7) (character 29))) (parent (node (document "d0") (qualified-name "Performances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Base::things") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 16)) (end (line 7) (character 28))))))
    (element (id (node (document "d0") (qualified-name "Performances::transfers"))) (kind "import") (name "transfers") (declared-name "transfers") (range (start (line 15) (character 1)) (end (line 15) (character 37))) (parent (node (document "d0") (qualified-name "Performances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::transfers") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 15) (character 16)) (end (line 15) (character 36))))))
    (element (id (node (document "d0") (qualified-name "Performances::transfersBefore"))) (kind "import") (name "transfersBefore") (declared-name "transfersBefore") (range (start (line 17) (character 1)) (end (line 17) (character 43))) (parent (node (document "d0") (qualified-name "Performances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::transfersBefore") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 17) (character 16)) (end (line 17) (character 42))))))
    (element (id (node (document "d0") (qualified-name "Performances::trueEvaluations"))) (kind "kermlDecl") (name "trueEvaluations") (declared-name "trueEvaluations") (range (start (line 220) (character 1)) (end (line 220) (character 296))) (parent (node (document "d0") (qualified-name "Performances"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Performances::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 18) (character 16)) (end (line 18) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Performances::Anything"))) (kind membershipImport) (ordinal 0)) (authored-target "Base::Anything") (range (start (line 6) (character 16)) (end (line 6) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Performances::BinaryLink"))) (kind membershipImport) (ordinal 0)) (authored-target "Links::BinaryLink") (range (start (line 12) (character 16)) (end (line 12) (character 33))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Performances::HappensDuring"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::HappensDuring") (range (start (line 10) (character 16)) (end (line 10) (character 42))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Performances::Metaobject"))) (kind membershipImport) (ordinal 0)) (authored-target "Metaobjects::Metaobject") (range (start (line 13) (character 16)) (end (line 13) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Performances::Object"))) (kind membershipImport) (ordinal 0)) (authored-target "Objects::Object") (range (start (line 11) (character 16)) (end (line 11) (character 31))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Performances::Occurrence"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::Occurrence") (range (start (line 8) (character 16)) (end (line 8) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Performances::Transfer"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::Transfer") (range (start (line 14) (character 16)) (end (line 14) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Performances::TransferBefore"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::TransferBefore") (range (start (line 16) (character 16)) (end (line 16) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Performances::includes"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::includes") (range (start (line 19) (character 16)) (end (line 19) (character 43))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Performances::occurrences"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::occurrences") (range (start (line 9) (character 16)) (end (line 9) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Performances::things"))) (kind membershipImport) (ordinal 0)) (authored-target "Base::things") (range (start (line 7) (character 16)) (end (line 7) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Performances::transfers"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::transfers") (range (start (line 15) (character 16)) (end (line 15) (character 36))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Performances::transfersBefore"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::transfersBefore") (range (start (line 17) (character 16)) (end (line 17) (character 42))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
