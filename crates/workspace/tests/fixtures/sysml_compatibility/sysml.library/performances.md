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
    doc /*
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
        doc /*
		 * Performance is the most general class of behavioral Occurrences that may be performed over time.
		 */

        feature self : Performance redefines Occurrence::self;

        feature involvedObjects : Object [0..*] {
            doc /*
			 * Objects that are involved in this Performance.
			 */
        }

        feature performers : Object [0..*] subsets involvedObjects {
            doc /*
			 * Objects that enact this Performance.
			 */
        }

        feature redefines isDispatch default = true;
        feature redefines dispatchScope default = thisPerformance;

        step enclosedPerformances: Performance[0..*] subsets performances, timeEnclosedOccurrences
			intersects performances, timeEnclosedOccurrences {
			doc
			/*
			 * timeEnclosedOccurrences of this Performance that are also Performances.
			 */
		}

        feature thisPerformance : Performance [1] default = self {
            doc /*
			 * Defaults to the root of the subperformance composition tree.
			 */
        }
        connector : HappensDuring from [1] self to [1] thisPerformance;

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
        doc /*
		 * InvolvedIn asserts that the involvedObject is involved in the Behavior carried out by the 
		 * involvingPerformance.
		 */

        end feature involvedObject : Object redefines source crosses involvingPerformance.involvedObjects;
        end feature involvingPerformance : Performance redefines target crosses involvedObject.involvingPerformances;
    }

    assoc all Performs specializes InvolvedIn {
        doc /*
		 * Performs asserts that the performer enacts the Behavior carried out by the performance.
		 */

        end feature performerObject : Object redefines involvedObject crosses performance.performers;
        end feature performance : Performance redefines involvingPerformance crosses performerObject.enactedPerformances;
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
(model
  (namespace
    (library_package 'Performances'
      (documentation)
      (membership_import private -> 'Base::Anything'[unresolved])
      (membership_import private -> 'Base::things'[unresolved])
      (membership_import private -> 'Occurrences::Occurrence'[unresolved])
      (membership_import private -> 'Occurrences::occurrences'[unresolved])
      (membership_import private -> 'Occurrences::HappensDuring'[unresolved])
      (membership_import private -> 'Objects::Object'[unresolved])
      (membership_import private -> 'Links::BinaryLink'[unresolved])
      (membership_import private -> 'Metaobjects::Metaobject'[unresolved])
      (membership_import private -> 'Transfers::Transfer'[unresolved])
      (membership_import private -> 'Transfers::transfers'[unresolved])
      (membership_import private -> 'Transfers::TransferBefore'[unresolved])
      (membership_import private -> 'Transfers::transfersBefore'[unresolved])
      (namespace_import private -> 'ScalarValues'[unresolved])
      (membership_import private -> 'SequenceFunctions::includes'[unresolved])
      (behavior_def abstract 'Performance' :> 'Occurrence'[unresolved]
        (disjoining_decl)
        (documentation)
        (feature_def 'self' : 'Performances::Performance'[behavior_def] :>> 'Occurrence::self'[unresolved])
        (feature_def 'involvedObjects' : 'Object'[unresolved]
          (multiplicity_range [0..*])
          (documentation))
        (feature_def 'performers' : 'Object'[unresolved] :> 'Performances::Performance::involvedObjects'[feature_def]
          (multiplicity_range [0..*])
          (documentation))
        (feature_def :>> 'isDispatch'[unresolved]
          (feature_value (default =)))
        (feature_def :>> 'dispatchScope'[unresolved]
          (feature_value (default =)))
        (step_def 'enclosedPerformances' : 'Performances::Performance'[behavior_def] :> 'Performances::performances'[step_def] :> 'timeEnclosedOccurrences'[unresolved]
          (multiplicity_range [0..*])
          (documentation))
        (feature_def 'thisPerformance' : 'Performances::Performance'[behavior_def]
          (multiplicity_range [1])
          (feature_value (default =))
          (documentation))
        (connector_def : 'HappensDuring'[unresolved]
          (connector_end 'self')
          (connector_end 'thisPerformance'))
        (step_def composite 'subperformances' : 'Performances::Performance'[behavior_def] :> 'Performances::Performance::enclosedPerformances'[step_def] :> 'suboccurrences'[unresolved]
          (multiplicity_range [0..*])
          (documentation)
          (feature_def :>> 'this'[unresolved]
            (feature_value (default =))
            (documentation))
          (feature_def :>> 'Performances::Performance::thisPerformance'[feature_def]
            (feature_value (default =)))))
      (function_def abstract 'Evaluation' :> 'Performances::Performance'[behavior_def]
        (documentation)
        (return_parameter_membership
          (feature_def out 'result' : 'Anything'[unresolved]
            (multiplicity_range [0..*]))))
      (predicate_def abstract 'BooleanEvaluation' :> 'Performances::Evaluation'[function_def]
        (documentation)
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved] :>> 'result'[feature_def][implied]
            (multiplicity_range [1]))))
      (function_def abstract 'MetadataAccessEvaluation' :> 'Performances::Evaluation'[function_def]
        (documentation)
        (return_parameter_membership
          (feature_def out : 'Metaobject'[unresolved] :>> 'result'[feature_def][implied]
            (multiplicity_range [1..*]))))
      (function_def abstract 'LiteralEvaluation' :> 'Performances::Evaluation'[function_def]
        (documentation)
        (return_parameter_membership
          (feature_def out : 'ScalarValue'[unresolved] :>> 'result'[feature_def][implied]
            (multiplicity_range [1]))))
      (predicate_def abstract 'LiteralBooleanEvaluation' :> 'Performances::LiteralEvaluation'[function_def] :> 'Performances::BooleanEvaluation'[predicate_def]
        (intersecting)
        (intersecting)
        (documentation)
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved] :>> ''[feature_def][implied] :>> ''[feature_def][implied]
            (multiplicity_range [1]))))
      (function_def abstract 'LiteralIntegerEvaluation' :> 'Performances::LiteralEvaluation'[function_def]
        (documentation)
        (return_parameter_membership
          (feature_def out : 'Integer'[unresolved] :>> ''[feature_def][implied]
            (multiplicity_range [1]))))
      (function_def abstract 'LiteralRationalEvaluation' :> 'Performances::LiteralEvaluation'[function_def]
        (documentation)
        (return_parameter_membership
          (feature_def out : 'Real'[unresolved] :>> ''[feature_def][implied]
            (multiplicity_range [1]))))
      (function_def abstract 'LiteralStringEvaluation' :> 'Performances::LiteralEvaluation'[function_def]
        (documentation)
        (return_parameter_membership
          (feature_def out : 'String'[unresolved] :>> ''[feature_def][implied]
            (multiplicity_range [1]))))
      (function_def 'NullEvaluation' :> 'Performances::Evaluation'[function_def]
        (documentation)
        (return_parameter_membership
          (feature_def out : 'Anything'[unresolved] :>> 'result'[feature_def][implied]
            (multiplicity_range [0..0]))))
      (association_def sufficient 'InvolvedIn' :> 'BinaryLink'[unresolved]
        (documentation)
        (feature_def end 'involvedObject' : 'Object'[unresolved] :>> 'source'[unresolved] :> 'Performances::Performance::involvedObjects'[feature_def])
        (feature_def end 'involvingPerformance' : 'Performances::Performance'[behavior_def] :>> 'target'[unresolved] :> 'involvedObject::involvingPerformances'[unresolved]))
      (association_def sufficient 'Performs' :> 'Performances::InvolvedIn'[association_def]
        (documentation)
        (feature_def end 'performerObject' : 'Object'[unresolved] :>> 'Performances::InvolvedIn::involvedObject'[feature_def] :> 'Performances::Performance::performers'[feature_def])
        (feature_def end 'performance' : 'Performances::Performance'[behavior_def] :>> 'Performances::InvolvedIn::involvingPerformance'[feature_def] :> 'performerObject::enactedPerformances'[unresolved]))
      (step_def abstract 'performances' : 'Performances::Performance'[behavior_def] :> 'occurrences'[unresolved]
        (multiplicity_range [0..*])
        (documentation))
      (expression_def abstract 'evaluations' : 'Performances::Evaluation'[function_def] :> 'Performances::performances'[step_def]
        (multiplicity_range [0..*])
        (documentation))
      (expression_def abstract 'constructorEvaluations' :> 'Performances::evaluations'[expression_def]
        (multiplicity_range [0..*])
        (documentation)
        (return_parameter_membership
          (feature_def out 'result'
            (multiplicity_range [1..1]))))
      (expression_def abstract 'booleanEvaluations' : 'Performances::BooleanEvaluation'[predicate_def] :> 'Performances::evaluations'[expression_def]
        (multiplicity_range [0..*])
        (documentation))
      (expression_def abstract 'trueEvaluations' :> 'Performances::booleanEvaluations'[expression_def]
        (documentation)
        (feature_def 'trueValue'
          (feature_value (=)))
        (binding_connector_def
          (connector_end 'result')
          (connector_end 'trueValue')))
      (expression_def abstract 'falseEvaluations' :> 'Performances::booleanEvaluations'[expression_def]
        (documentation)
        (feature_def 'falseValue'
          (feature_value (=)))
        (binding_connector_def
          (connector_end 'result')
          (connector_end 'falseValue')))
      (expression_def abstract 'metadataAccessEvaluations' : 'Performances::MetadataAccessEvaluation'[function_def] :> 'Performances::evaluations'[expression_def]
        (multiplicity_range [0..*])
        (documentation))
      (expression_def abstract 'literalEvaluations' : 'Performances::LiteralEvaluation'[function_def] :> 'Performances::evaluations'[expression_def]
        (multiplicity_range [0..*])
        (documentation))
      (expression_def abstract 'literalBooleanEvaluations' : 'Performances::LiteralBooleanEvaluation'[predicate_def] :> 'Performances::literalEvaluations'[expression_def] :> 'Performances::booleanEvaluations'[expression_def]
        (multiplicity_range [0..*])
        (documentation))
      (expression_def abstract 'literalIntegerEvaluations' : 'Performances::LiteralIntegerEvaluation'[function_def] :> 'Performances::literalEvaluations'[expression_def]
        (multiplicity_range [0..*])
        (documentation))
      (expression_def abstract 'literalRationalEvaluations' : 'Performances::LiteralRationalEvaluation'[function_def] :> 'Performances::literalEvaluations'[expression_def]
        (multiplicity_range [0..*])
        (documentation))
      (expression_def abstract 'literalStringEvaluations' : 'Performances::LiteralStringEvaluation'[function_def] :> 'Performances::literalEvaluations'[expression_def]
        (multiplicity_range [0..*])
        (documentation))
      (expression_def abstract 'nullEvaluations' : 'Performances::NullEvaluation'[function_def] :> 'Performances::evaluations'[expression_def]
        (multiplicity_range [0..*])
        (documentation)))))
~~~
