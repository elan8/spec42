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
# EXPECTED
~~~
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Occurrence::self'
semantic.unresolved_name 'Object'
semantic.unresolved_name 'Object'
semantic.unresolved_name 'isDispatch'
semantic.unresolved_name 'dispatchScope'
semantic.unresolved_name 'timeEnclosedOccurrences'
semantic.unresolved_name 'HappensDuring'
semantic.unresolved_name 'suboccurrences'
semantic.unresolved_name 'this'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Metaobject'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'BinaryLink'
semantic.unresolved_name 'Object'
semantic.unresolved_name 'source'
semantic.unresolved_name 'target'
semantic.unresolved_name 'involvedObject::involvingPerformances'
semantic.unresolved_name 'Object'
semantic.unresolved_name 'performerObject::enactedPerformances'
semantic.unresolved_name 'occurrences'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Occurrence::self'
semantic.unresolved_name 'Object'
semantic.unresolved_name 'Object'
semantic.unresolved_name 'isDispatch'
semantic.unresolved_name 'dispatchScope'
semantic.unresolved_name 'timeEnclosedOccurrences'
semantic.unresolved_name 'HappensDuring'
semantic.unresolved_name 'suboccurrences'
semantic.unresolved_name 'this'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Metaobject'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'BinaryLink'
semantic.unresolved_name 'Object'
semantic.unresolved_name 'source'
semantic.unresolved_name 'target'
semantic.unresolved_name 'involvedObject::involvingPerformances'
semantic.unresolved_name 'Object'
semantic.unresolved_name 'performerObject::enactedPerformances'
semantic.unresolved_name 'occurrences'
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
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwAbstract,KwBehavior,Ident,KwSpecializes,Ident,KwDisjoint,KwFrom,Ident,OpenCurly,
KwDoc,
RegularComment,
KwFeature,Ident,Colon,Ident,KwRedefines,Ident,ColonColon,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwFeature,KwRedefines,Ident,KwDefault,KwTrue,Semicolon,
KwFeature,KwRedefines,Ident,KwDefault,Ident,Semicolon,
KwStep,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,Comma,Ident,
KwIntersects,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwConnector,Colon,Ident,KwFrom,OpenSquare,DecimalValue,CloseSquare,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
KwComposite,KwStep,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,Comma,Ident,
KwIntersects,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
KwFeature,KwRedefines,Ident,KwDefault,OpenParen,Ident,KwAs,Ident,CloseParen,Dot,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwFeature,KwRedefines,Ident,KwDefault,OpenParen,Ident,KwAs,Ident,CloseParen,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwAbstract,KwFunction,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwReturn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,Semicolon,
CloseCurly,
KwAbstract,KwPredicate,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAbstract,KwFunction,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
CloseCurly,
KwAbstract,KwFunction,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAbstract,KwPredicate,Ident,KwSpecializes,Ident,Comma,Ident,
KwIntersects,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAbstract,KwFunction,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAbstract,KwFunction,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAbstract,KwFunction,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwFunction,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAssoc,KwAll,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwEnd,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,KwCrosses,Ident,Dot,Ident,Semicolon,
KwEnd,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,KwCrosses,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwAssoc,KwAll,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwEnd,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,KwCrosses,Ident,Dot,Ident,Semicolon,
KwEnd,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,KwCrosses,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwAbstract,KwStep,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,KwSubsets,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwExpr,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,KwSubsets,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwExpr,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,KwSubsets,Ident,OpenCurly,
KwDoc,
RegularComment,
KwReturn,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAbstract,KwExpr,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,KwSubsets,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwExpr,Ident,KwSubsets,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwFeature,Ident,Eq,KwTrue,Semicolon,
KwBinding,Ident,Eq,Ident,Semicolon,
CloseCurly,
KwAbstract,KwExpr,Ident,KwSubsets,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwFeature,Ident,Eq,KwFalse,Semicolon,
KwBinding,Ident,Eq,Ident,Semicolon,
CloseCurly,
KwAbstract,KwExpr,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,KwSubsets,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwExpr,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,KwSubsets,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwExpr,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,KwSubsets,Ident,Comma,Ident,
KwIntersects,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwExpr,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,KwSubsets,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwExpr,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,KwSubsets,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwExpr,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,KwSubsets,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwExpr,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,KwSubsets,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'Performances'
    (documentation)
    (import_decl private 'Base::Anything')
    (import_decl private 'Base::things')
    (import_decl private 'Occurrences::Occurrence')
    (import_decl private 'Occurrences::occurrences')
    (import_decl private 'Occurrences::HappensDuring')
    (import_decl private 'Objects::Object')
    (import_decl private 'Links::BinaryLink')
    (import_decl private 'Metaobjects::Metaobject')
    (import_decl private 'Transfers::Transfer')
    (import_decl private 'Transfers::transfers')
    (import_decl private 'Transfers::TransferBefore')
    (import_decl private 'Transfers::transfersBefore')
    (import_decl private 'ScalarValues::*')
    (import_decl private 'SequenceFunctions::includes')
    (behavior_def
      (documentation)
      (feature_def 'self' : 'Performance' :>> 'Occurrence::self')
      (feature_def 'involvedObjects' : 'Object' multiplicity
        (documentation))
      (feature_def 'performers' : 'Object' multiplicity :> 'involvedObjects'
        (documentation))
      (feature_def :>> 'isDispatch' value)
      (feature_def :>> 'dispatchScope' value)
      (step_def
        (documentation))
      (feature_def 'thisPerformance' : 'Performance' multiplicity value
        (documentation))
      (connector_def : 'HappensDuring'
        (connector_end)
        (connector_end))
      (step_def
        (documentation)
        (feature_def :>> 'this' value
          (documentation))
        (feature_def :>> 'thisPerformance' value)))
    (function_def
      (documentation)
      (return_member))
    (predicate_def
      (documentation)
      (return_member))
    (function_def
      (documentation)
      (return_member))
    (function_def
      (documentation)
      (return_member))
    (predicate_def
      (documentation)
      (return_member))
    (function_def
      (documentation)
      (return_member))
    (function_def
      (documentation)
      (return_member))
    (function_def
      (documentation)
      (return_member))
    (function_def
      (documentation)
      (return_member))
    (association_def all 'InvolvedIn' :> 'BinaryLink'
      (documentation)
      (feature_def end 'involvedObject' : 'Object' :>> 'source' crosses 'involvingPerformance.involvedObjects')
      (feature_def end 'involvingPerformance' : 'Performance' :>> 'target' crosses 'involvedObject.involvingPerformances'))
    (association_def all 'Performs' :> 'InvolvedIn'
      (documentation)
      (feature_def end 'performerObject' : 'Object' :>> 'involvedObject' crosses 'performance.performers')
      (feature_def end 'performance' : 'Performance' :>> 'involvingPerformance' crosses 'performerObject.enactedPerformances'))
    (step_def
      (documentation))
    (expression_def
      (documentation))
    (expression_def
      (documentation)
      (return_member))
    (expression_def
      (documentation))
    (expression_def
      (documentation)
      (feature_def private 'trueValue' value)
      (binding_connector
        (connector_end)
        (connector_end)))
    (expression_def
      (documentation)
      (feature_def private 'falseValue' value)
      (binding_connector
        (connector_end)
        (connector_end)))
    (expression_def
      (documentation))
    (expression_def
      (documentation))
    (expression_def
      (documentation))
    (expression_def
      (documentation))
    (expression_def
      (documentation))
    (expression_def
      (documentation))
    (expression_def
      (documentation))))
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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Performances"))) (name "Performances") (declared-name "Performances")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Performances::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Performances::Anything"))) (name "Anything") (declared-name "Anything"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Performances::BinaryLink"))) (name "BinaryLink") (declared-name "BinaryLink"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Performances::BooleanEvaluation"))) (name "BooleanEvaluation") (declared-name "BooleanEvaluation"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Performances::Evaluation"))) (name "Evaluation") (declared-name "Evaluation"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Performances::HappensDuring"))) (name "HappensDuring") (declared-name "HappensDuring"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Performances::LiteralBooleanEvaluation"))) (name "LiteralBooleanEvaluation") (declared-name "LiteralBooleanEvaluation"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Performances::LiteralEvaluation"))) (name "LiteralEvaluation") (declared-name "LiteralEvaluation"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Performances::LiteralIntegerEvaluation"))) (name "LiteralIntegerEvaluation") (declared-name "LiteralIntegerEvaluation"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Performances::LiteralRationalEvaluation"))) (name "LiteralRationalEvaluation") (declared-name "LiteralRationalEvaluation"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Performances::LiteralStringEvaluation"))) (name "LiteralStringEvaluation") (declared-name "LiteralStringEvaluation"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Performances::MetadataAccessEvaluation"))) (name "MetadataAccessEvaluation") (declared-name "MetadataAccessEvaluation"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Performances::Metaobject"))) (name "Metaobject") (declared-name "Metaobject"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Performances::NullEvaluation"))) (name "NullEvaluation") (declared-name "NullEvaluation"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Performances::Object"))) (name "Object") (declared-name "Object"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Performances::Occurrence"))) (name "Occurrence") (declared-name "Occurrence"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Performances::Performance"))) (name "Performance") (declared-name "Performance"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Performances::Transfer"))) (name "Transfer") (declared-name "Transfer"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Performances::TransferBefore"))) (name "TransferBefore") (declared-name "TransferBefore"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "Performances::_documentation"))) (name ""))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Performances::all"))) (name "all") (declared-name "all"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Performances::all#kermlDecl"))) (name "all") (declared-name "all"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Performances::booleanEvaluations"))) (name "booleanEvaluations") (declared-name "booleanEvaluations"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Performances::constructorEvaluations"))) (name "constructorEvaluations") (declared-name "constructorEvaluations"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Performances::evaluations"))) (name "evaluations") (declared-name "evaluations"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Performances::falseEvaluations"))) (name "falseEvaluations") (declared-name "falseEvaluations"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Performances::includes"))) (name "includes") (declared-name "includes"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Performances::literalBooleanEvaluations"))) (name "literalBooleanEvaluations") (declared-name "literalBooleanEvaluations"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Performances::literalEvaluations"))) (name "literalEvaluations") (declared-name "literalEvaluations"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Performances::literalIntegerEvaluations"))) (name "literalIntegerEvaluations") (declared-name "literalIntegerEvaluations"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Performances::literalRationalEvaluations"))) (name "literalRationalEvaluations") (declared-name "literalRationalEvaluations"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Performances::literalStringEvaluations"))) (name "literalStringEvaluations") (declared-name "literalStringEvaluations"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Performances::metadataAccessEvaluations"))) (name "metadataAccessEvaluations") (declared-name "metadataAccessEvaluations"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Performances::nullEvaluations"))) (name "nullEvaluations") (declared-name "nullEvaluations"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Performances::occurrences"))) (name "occurrences") (declared-name "occurrences"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Performances::performances"))) (name "performances") (declared-name "performances"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Performances::things"))) (name "things") (declared-name "things"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Performances::transfers"))) (name "transfers") (declared-name "transfers"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Performances::transfersBefore"))) (name "transfersBefore") (declared-name "transfersBefore"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Performances::trueEvaluations"))) (name "trueEvaluations") (declared-name "trueEvaluations"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Performances::_documentation"))) (to (node (document "d0") (qualified-name "Performances"))) (provenance authored))
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
  (document "sysml.library/performances.md"
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
      (diagnostic
        (severity warning)
        (code "duplicate_namespace_member")
        (source "semantic")
        (range (start 179 1) (end 179 372))
      )
    )
  )
)
~~~
