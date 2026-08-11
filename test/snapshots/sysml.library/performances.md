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
    (element (id (node (document "d0") (qualified-name "Performances"))) (kind "package") (name "Performances") (declared-name "Performances"))
    (element (id (node (document "d0") (qualified-name "Performances::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Performances"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Performances::Anything"))) (kind "import") (name "Anything") (declared-name "Anything") (parent (node (document "d0") (qualified-name "Performances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Base::Anything") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Performances::BinaryLink"))) (kind "import") (name "BinaryLink") (declared-name "BinaryLink") (parent (node (document "d0") (qualified-name "Performances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Links::BinaryLink") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Performances::BooleanEvaluation"))) (kind "kermlDecl") (name "BooleanEvaluation") (declared-name "BooleanEvaluation") (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::Evaluation"))) (kind "kermlDecl") (name "Evaluation") (declared-name "Evaluation") (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::HappensDuring"))) (kind "import") (name "HappensDuring") (declared-name "HappensDuring") (parent (node (document "d0") (qualified-name "Performances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::HappensDuring") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Performances::LiteralBooleanEvaluation"))) (kind "kermlDecl") (name "LiteralBooleanEvaluation") (declared-name "LiteralBooleanEvaluation") (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::LiteralEvaluation"))) (kind "kermlDecl") (name "LiteralEvaluation") (declared-name "LiteralEvaluation") (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::LiteralIntegerEvaluation"))) (kind "kermlDecl") (name "LiteralIntegerEvaluation") (declared-name "LiteralIntegerEvaluation") (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::LiteralRationalEvaluation"))) (kind "kermlDecl") (name "LiteralRationalEvaluation") (declared-name "LiteralRationalEvaluation") (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::LiteralStringEvaluation"))) (kind "kermlDecl") (name "LiteralStringEvaluation") (declared-name "LiteralStringEvaluation") (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::MetadataAccessEvaluation"))) (kind "kermlDecl") (name "MetadataAccessEvaluation") (declared-name "MetadataAccessEvaluation") (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::Metaobject"))) (kind "import") (name "Metaobject") (declared-name "Metaobject") (parent (node (document "d0") (qualified-name "Performances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Metaobjects::Metaobject") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Performances::NullEvaluation"))) (kind "kermlDecl") (name "NullEvaluation") (declared-name "NullEvaluation") (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::Object"))) (kind "import") (name "Object") (declared-name "Object") (parent (node (document "d0") (qualified-name "Performances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Objects::Object") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Performances::Occurrence"))) (kind "import") (name "Occurrence") (declared-name "Occurrence") (parent (node (document "d0") (qualified-name "Performances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::Occurrence") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Performances::Performance"))) (kind "kermlDecl") (name "Performance") (declared-name "Performance") (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::Transfer"))) (kind "import") (name "Transfer") (declared-name "Transfer") (parent (node (document "d0") (qualified-name "Performances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::Transfer") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Performances::TransferBefore"))) (kind "import") (name "TransferBefore") (declared-name "TransferBefore") (parent (node (document "d0") (qualified-name "Performances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::TransferBefore") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Performances::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::all"))) (kind "kermlDecl") (name "all") (declared-name "all") (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::all#kermlDecl"))) (kind "kermlDecl") (name "all") (declared-name "all") (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::booleanEvaluations"))) (kind "kermlDecl") (name "booleanEvaluations") (declared-name "booleanEvaluations") (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::constructorEvaluations"))) (kind "kermlDecl") (name "constructorEvaluations") (declared-name "constructorEvaluations") (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::evaluations"))) (kind "kermlDecl") (name "evaluations") (declared-name "evaluations") (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::falseEvaluations"))) (kind "kermlDecl") (name "falseEvaluations") (declared-name "falseEvaluations") (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::includes"))) (kind "import") (name "includes") (declared-name "includes") (parent (node (document "d0") (qualified-name "Performances"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::includes") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Performances::literalBooleanEvaluations"))) (kind "kermlDecl") (name "literalBooleanEvaluations") (declared-name "literalBooleanEvaluations") (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::literalEvaluations"))) (kind "kermlDecl") (name "literalEvaluations") (declared-name "literalEvaluations") (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::literalIntegerEvaluations"))) (kind "kermlDecl") (name "literalIntegerEvaluations") (declared-name "literalIntegerEvaluations") (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::literalRationalEvaluations"))) (kind "kermlDecl") (name "literalRationalEvaluations") (declared-name "literalRationalEvaluations") (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::literalStringEvaluations"))) (kind "kermlDecl") (name "literalStringEvaluations") (declared-name "literalStringEvaluations") (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::metadataAccessEvaluations"))) (kind "kermlDecl") (name "metadataAccessEvaluations") (declared-name "metadataAccessEvaluations") (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::nullEvaluations"))) (kind "kermlDecl") (name "nullEvaluations") (declared-name "nullEvaluations") (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::occurrences"))) (kind "import") (name "occurrences") (declared-name "occurrences") (parent (node (document "d0") (qualified-name "Performances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::occurrences") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Performances::performances"))) (kind "kermlDecl") (name "performances") (declared-name "performances") (parent (node (document "d0") (qualified-name "Performances"))))
    (element (id (node (document "d0") (qualified-name "Performances::things"))) (kind "import") (name "things") (declared-name "things") (parent (node (document "d0") (qualified-name "Performances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Base::things") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Performances::transfers"))) (kind "import") (name "transfers") (declared-name "transfers") (parent (node (document "d0") (qualified-name "Performances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::transfers") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Performances::transfersBefore"))) (kind "import") (name "transfersBefore") (declared-name "transfersBefore") (parent (node (document "d0") (qualified-name "Performances"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transfers::transfersBefore") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Performances::trueEvaluations"))) (kind "kermlDecl") (name "trueEvaluations") (declared-name "trueEvaluations") (parent (node (document "d0") (qualified-name "Performances"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Performances::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Performances::Anything"))) (kind membershipImport) (ordinal 0)) (authored-target "Base::Anything") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Performances::BinaryLink"))) (kind membershipImport) (ordinal 0)) (authored-target "Links::BinaryLink") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Performances::HappensDuring"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::HappensDuring") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Performances::Metaobject"))) (kind membershipImport) (ordinal 0)) (authored-target "Metaobjects::Metaobject") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Performances::Object"))) (kind membershipImport) (ordinal 0)) (authored-target "Objects::Object") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Performances::Occurrence"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::Occurrence") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Performances::Transfer"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::Transfer") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Performances::TransferBefore"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::TransferBefore") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Performances::includes"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::includes") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Performances::occurrences"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::occurrences") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Performances::things"))) (kind membershipImport) (ordinal 0)) (authored-target "Base::things") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Performances::transfers"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::transfers") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Performances::transfersBefore"))) (kind membershipImport) (ordinal 0)) (authored-target "Transfers::transfersBefore") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
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
    (query (range (start 7 16) (end 7 28)) (probe (position 7 16))
      (reference
        (source (document "d0") (qualified-name "Performances::things"))
        (kind membershipImport) (ordinal 0) (authored-target "Base::things")
        (range (start 7 16) (end 7 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 18 16) (end 18 28)) (probe (position 18 16))
      (reference
        (source (document "d0") (qualified-name "Performances::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 18 16) (end 18 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 6 16) (end 6 30)) (probe (position 6 16))
      (reference
        (source (document "d0") (qualified-name "Performances::Anything"))
        (kind membershipImport) (ordinal 0) (authored-target "Base::Anything")
        (range (start 6 16) (end 6 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 11 16) (end 11 31)) (probe (position 11 16))
      (reference
        (source (document "d0") (qualified-name "Performances::Object"))
        (kind membershipImport) (ordinal 0) (authored-target "Objects::Object")
        (range (start 11 16) (end 11 31))
        (outcome (status unresolved))
      )
    )
    (query (range (start 12 16) (end 12 33)) (probe (position 12 16))
      (reference
        (source (document "d0") (qualified-name "Performances::BinaryLink"))
        (kind membershipImport) (ordinal 0) (authored-target "Links::BinaryLink")
        (range (start 12 16) (end 12 33))
        (outcome (status unresolved))
      )
    )
    (query (range (start 14 16) (end 14 35)) (probe (position 14 16))
      (reference
        (source (document "d0") (qualified-name "Performances::Transfer"))
        (kind membershipImport) (ordinal 0) (authored-target "Transfers::Transfer")
        (range (start 14 16) (end 14 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 15 16) (end 15 36)) (probe (position 15 16))
      (reference
        (source (document "d0") (qualified-name "Performances::transfers"))
        (kind membershipImport) (ordinal 0) (authored-target "Transfers::transfers")
        (range (start 15 16) (end 15 36))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 16) (end 8 39)) (probe (position 8 16))
      (reference
        (source (document "d0") (qualified-name "Performances::Occurrence"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::Occurrence")
        (range (start 8 16) (end 8 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 13 16) (end 13 39)) (probe (position 13 16))
      (reference
        (source (document "d0") (qualified-name "Performances::Metaobject"))
        (kind membershipImport) (ordinal 0) (authored-target "Metaobjects::Metaobject")
        (range (start 13 16) (end 13 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 16) (end 9 40)) (probe (position 9 16))
      (reference
        (source (document "d0") (qualified-name "Performances::occurrences"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::occurrences")
        (range (start 9 16) (end 9 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 16 16) (end 16 41)) (probe (position 16 16))
      (reference
        (source (document "d0") (qualified-name "Performances::TransferBefore"))
        (kind membershipImport) (ordinal 0) (authored-target "Transfers::TransferBefore")
        (range (start 16 16) (end 16 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 16) (end 10 42)) (probe (position 10 16))
      (reference
        (source (document "d0") (qualified-name "Performances::HappensDuring"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::HappensDuring")
        (range (start 10 16) (end 10 42))
        (outcome (status unresolved))
      )
    )
    (query (range (start 17 16) (end 17 42)) (probe (position 17 16))
      (reference
        (source (document "d0") (qualified-name "Performances::transfersBefore"))
        (kind membershipImport) (ordinal 0) (authored-target "Transfers::transfersBefore")
        (range (start 17 16) (end 17 42))
        (outcome (status unresolved))
      )
    )
    (query (range (start 19 16) (end 19 43)) (probe (position 19 16))
      (reference
        (source (document "d0") (qualified-name "Performances::includes"))
        (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::includes")
        (range (start 19 16) (end 19 43))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
