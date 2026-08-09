# META
~~~ini
description=Standard Library: Domain Libraries/Analysis/TradeStudies
type=file
~~~
# SOURCE
~~~sysml
standard library package TradeStudies {
	doc
	/*
	 * This package provides a simple framework for defining trade-off study analysis cases.
	 */

	private import Base::Anything;
	private import ScalarValues::*;
	private import ScalarFunctions::*;
	private import ControlFunctions::*;
	
	abstract calc def EvaluationFunction {
		doc
		/*
		 * An EvaluationFunction is a calculation that evaluates a TradeStudy alternative,
		 * producing a ScalarValue that can be comparted with the evaluation of other
		 * alternatives.
		 */
	
		in ref alternative : Anything {
			doc
			/*
			 * The alternative to be evaluated.
			 */
		} 
		
		return attribute result : ScalarValue[1] {
			doc
			/*
			 * A ScalarValue representing the evaluation of the given alternative.
			 */
		} 
	}
	
	abstract requirement def TradeStudyObjective {
		doc
		/*
		 * A TradeStudyObjective is the base definition for the objective of a TradeStudy.
		 * The requirement is to choose from a given set of alternatives the selectedAlternative
		 * for that has the best evaluation according to a given EvaluationFunction. What
		 * value is considered "best" is not defined in the abstract base definition but must be
		 * computed in any concrete specialization.
		 */
	
		subject selectedAlternative : Anything {
			doc
			/*
			 * The alternative that should be selected, as evaluated using the given 
			 * ObjectiveFunction.
			 */
		}
		
		in ref alternatives : Anything[1..*] {
			doc
			/*
			 * The alternatives being considered in the TradeStudy for which this TradeStudyObjective 
			 * is the objective.
			 */
		}
		
		in calc eval : EvaluationFunction {
			doc
			/*
			 * The EvaluationFunction to be used in evaluating the given alternatives.
			 */
		}
		
		attribute best : ScalarValue {
			doc
			/*
			 * Out of the evaluation results of all the given alternatives, the one that is considered 
			 * "best", in the sense that it is the value the selectedAlternative should have. This 
			 * value must be computed in any concrete specialization of TradeStudyObjective.
			 */
		}
				
		require constraint { eval(selectedAlternative) == best }
	}
	
	requirement def MinimizeObjective :> TradeStudyObjective {
		doc
		/*
		 * A MinimizeObjective is a TradeStudyObjective that requires that the 
		 * selectedAlternative have the minimum ObjectiveFunction value of all the
		 * given alternatives.
		 */
		 
		attribute :>> best = alternatives->minimize {
			doc
			/*
			 * For a MinimizeObjective, the best value is the minimum one.
			 */
		
			in x; eval(x)
		};
	}
	
	requirement def MaximizeObjective :> TradeStudyObjective {
		doc
		/*
		 * A MaximizeObjective is a TradeStudyObjective that requires that the 
		 * selectedAlternative have the maximum ObjectiveFunction value of all the
		 * given alternatives.
		 */
	
		attribute :>> best = alternatives->maximize {
			doc
			/*
			 * For a MinimizeObjective, the best value is the maximum one.
			 */
		
			in x; eval(x)
		};
	}
	
	abstract analysis def TradeStudy {
		doc
		/*
		 * A TradeStudy is an analysis case whose subject is a set of alternatives
		 * (at least one) and whose result is a selection of one of those alternatives.
		 * The alternatives are evaluated based on a given ObjectiveFunction and the
		 * selection is made such that it satisfies the objective of the TradeStudy
		 * (which must be a TradeStudyObjective).
		 */
	
		subject studyAlternatives : Anything[1..*] {
			doc
			/*
			 * The set of alternatives being considered in this TradeStudy. 
			 * 
			 * In a TradeStudy usage, bind this feature to the actual collection of
			 * alternatives to be considered.
			 */
		}
		
		abstract calc evaluationFunction : EvaluationFunction {
			doc
			/*
			 * The EvaluationFunction to be used to evaluate the alternatives.
			 * 
			 * In a TradeStudy usage, redefine this feature to provide the desired
			 * calculation (or bind it to a calculation usage that does so).
			 */
		}
		
		objective tradeStudyObjective : TradeStudyObjective {
			doc
			/*
			 * The objective of this TradeStudy.
			 * 
			 * Redefine this feature to give it a definition that is a concrete
			 * specialization of TradeStudyObjective. That can either be one of the
			 * specializations provided in this package, or a more specific user-
			 * defined one.
			 */
		
            subject :>> selectedAlternative;
			in ref :>> alternatives = studyAlternatives;
			in calc :>> eval = evaluationFunction;
		}
		
		return selectedAlternative : Anything = studyAlternatives->selectOne {in ref a {
			doc
			/*
			 * The alternative selected by this TradeStudy, which is the one that meets the
			 * requirement of the tradeStudyObjective.
			 */
		} tradeStudyObjective(selectedAlternative = a)};
	}
	
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'ScalarValue'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwAbstract,KwCalc,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,KwRef,Ident,Colon,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwReturn,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,
KwAbstract,KwRequirement,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
KwSubject,Ident,Colon,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwIn,KwRef,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwIn,KwCalc,Ident,Colon,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwRequire,KwConstraint,OpenCurly,Ident,OpenParen,Ident,CloseParen,EqEq,Ident,CloseCurly,
CloseCurly,
KwRequirement,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Eq,Ident,Arrow,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Semicolon,Ident,OpenParen,Ident,CloseParen,
CloseCurly,Semicolon,
CloseCurly,
KwRequirement,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Eq,Ident,Arrow,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Semicolon,Ident,OpenParen,Ident,CloseParen,
CloseCurly,Semicolon,
CloseCurly,
KwAbstract,KwAnalysis,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
KwSubject,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwCalc,Ident,Colon,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwObjective,Ident,Colon,Ident,OpenCurly,
KwDoc,
RegularComment,
KwSubject,ColonGtGt,Ident,Semicolon,
KwIn,KwRef,ColonGtGt,Ident,Eq,Ident,Semicolon,
KwIn,KwCalc,ColonGtGt,Ident,Eq,Ident,Semicolon,
CloseCurly,
KwReturn,Ident,Colon,Ident,Eq,Ident,Arrow,Ident,OpenCurly,KwIn,KwRef,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,Ident,OpenParen,Ident,Eq,Ident,CloseParen,CloseCurly,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'TradeStudies'
    (documentation)
    (import_decl private 'Base::Anything')
    (import_decl private 'ScalarValues::*')
    (import_decl private 'ScalarFunctions::*')
    (import_decl private 'ControlFunctions::*')
    (calc_def abstract 'EvaluationFunction'
      (documentation)
      (ref_usage in ref 'alternative' : 'Anything'
        (documentation))
      (return_member))
    (requirement_def abstract 'TradeStudyObjective'
      (documentation)
      (sysml_decl 'selectedAlternative' : 'Anything'
        (documentation))
      (ref_usage in ref 'alternatives' : 'Anything' multiplicity
        (documentation))
      (calc_usage in 'eval' : 'EvaluationFunction'
        (documentation))
      (attribute_usage 'best' : 'ScalarValue'
        (documentation))
      (sysml_decl
        (result_expr_member)))
    (requirement_def 'MinimizeObjective' :> 'TradeStudyObjective'
      (documentation)
      (attribute_usage :>> 'best' value))
    (requirement_def 'MaximizeObjective' :> 'TradeStudyObjective'
      (documentation)
      (attribute_usage :>> 'best' value))
    (analysis_case_def abstract 'TradeStudy'
      (documentation)
      (sysml_decl 'studyAlternatives' : 'Anything' multiplicity
        (documentation))
      (calc_usage abstract 'evaluationFunction' : 'EvaluationFunction'
        (documentation))
      (objective_member)
      (return_member))))
~~~
# FORMAT
~~~sysml
standard library package TradeStudies {
	doc
	/*
	 * This package provides a simple framework for defining trade-off study analysis cases.
	 */

	private import Base::Anything;
	private import ScalarValues::*;
	private import ScalarFunctions::*;
	private import ControlFunctions::*;
	
	abstract calc def EvaluationFunction {
		doc
		/*
		 * An EvaluationFunction is a calculation that evaluates a TradeStudy alternative,
		 * producing a ScalarValue that can be comparted with the evaluation of other
		 * alternatives.
		 */
	
		in ref alternative : Anything {
			doc
			/*
			 * The alternative to be evaluated.
			 */
		} 
		
		return attribute result : ScalarValue[1] {
			doc
			/*
			 * A ScalarValue representing the evaluation of the given alternative.
			 */
		} 
	}
	
	abstract requirement def TradeStudyObjective {
		doc
		/*
		 * A TradeStudyObjective is the base definition for the objective of a TradeStudy.
		 * The requirement is to choose from a given set of alternatives the selectedAlternative
		 * for that has the best evaluation according to a given EvaluationFunction. What
		 * value is considered "best" is not defined in the abstract base definition but must be
		 * computed in any concrete specialization.
		 */
	
		subject selectedAlternative : Anything {
			doc
			/*
			 * The alternative that should be selected, as evaluated using the given 
			 * ObjectiveFunction.
			 */
		}
		
		in ref alternatives : Anything[1..*] {
			doc
			/*
			 * The alternatives being considered in the TradeStudy for which this TradeStudyObjective 
			 * is the objective.
			 */
		}
		
		in calc eval : EvaluationFunction {
			doc
			/*
			 * The EvaluationFunction to be used in evaluating the given alternatives.
			 */
		}
		
		attribute best : ScalarValue {
			doc
			/*
			 * Out of the evaluation results of all the given alternatives, the one that is considered 
			 * "best", in the sense that it is the value the selectedAlternative should have. This 
			 * value must be computed in any concrete specialization of TradeStudyObjective.
			 */
		}
				
		require constraint { eval(selectedAlternative) == best }
	}
	
	requirement def MinimizeObjective :> TradeStudyObjective {
		doc
		/*
		 * A MinimizeObjective is a TradeStudyObjective that requires that the 
		 * selectedAlternative have the minimum ObjectiveFunction value of all the
		 * given alternatives.
		 */
		 
		attribute :>> best = alternatives->minimize {
			doc
			/*
			 * For a MinimizeObjective, the best value is the minimum one.
			 */
		
			in x; eval(x)
		};
	}
	
	requirement def MaximizeObjective :> TradeStudyObjective {
		doc
		/*
		 * A MaximizeObjective is a TradeStudyObjective that requires that the 
		 * selectedAlternative have the maximum ObjectiveFunction value of all the
		 * given alternatives.
		 */
	
		attribute :>> best = alternatives->maximize {
			doc
			/*
			 * For a MinimizeObjective, the best value is the maximum one.
			 */
		
			in x; eval(x)
		};
	}
	
	abstract analysis def TradeStudy {
		doc
		/*
		 * A TradeStudy is an analysis case whose subject is a set of alternatives
		 * (at least one) and whose result is a selection of one of those alternatives.
		 * The alternatives are evaluated based on a given ObjectiveFunction and the
		 * selection is made such that it satisfies the objective of the TradeStudy
		 * (which must be a TradeStudyObjective).
		 */
	
		subject studyAlternatives : Anything[1..*] {
			doc
			/*
			 * The set of alternatives being considered in this TradeStudy. 
			 * 
			 * In a TradeStudy usage, bind this feature to the actual collection of
			 * alternatives to be considered.
			 */
		}
		
		abstract calc evaluationFunction : EvaluationFunction {
			doc
			/*
			 * The EvaluationFunction to be used to evaluate the alternatives.
			 * 
			 * In a TradeStudy usage, redefine this feature to provide the desired
			 * calculation (or bind it to a calculation usage that does so).
			 */
		}
		
		objective tradeStudyObjective : TradeStudyObjective {
			doc
			/*
			 * The objective of this TradeStudy.
			 * 
			 * Redefine this feature to give it a definition that is a concrete
			 * specialization of TradeStudyObjective. That can either be one of the
			 * specializations provided in this package, or a more specific user-
			 * defined one.
			 */
		
            subject :>> selectedAlternative;
			in ref :>> alternatives = studyAlternatives;
			in calc :>> eval = evaluationFunction;
		}
		
		return selectedAlternative : Anything = studyAlternatives->selectOne {in ref a {
			doc
			/*
			 * The alternative selected by this TradeStudy, which is the one that meets the
			 * requirement of the tradeStudyObjective.
			 */
		} tradeStudyObjective(selectedAlternative = a)};
	}
	
}
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "TradeStudies"))) (name "TradeStudies") (declared-name "TradeStudies")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "TradeStudies::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "TradeStudies::*#import"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "TradeStudies::*#import2"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "TradeStudies::Anything"))) (name "Anything") (declared-name "Anything"))
        (element (kind "calc def") (id (node (document "d0") (qualified-name "TradeStudies::EvaluationFunction"))) (name "EvaluationFunction") (declared-name "EvaluationFunction")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "TradeStudies::EvaluationFunction::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "TradeStudies::EvaluationFunction")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "TradeStudies::EvaluationFunction::ref"))) (name "ref") (declared-name "ref") (effective (featuring-type (node (document "d0") (qualified-name "TradeStudies::EvaluationFunction")))))
          )
        )
        (element (kind "requirement def") (id (node (document "d0") (qualified-name "TradeStudies::MaximizeObjective"))) (name "MaximizeObjective") (declared-name "MaximizeObjective")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "TradeStudies::MaximizeObjective::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "TradeStudies::MaximizeObjective")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "TradeStudies::MaximizeObjective::best"))) (name "best") (declared-name "best") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "TradeStudies::MaximizeObjective")))))
          )
        )
        (element (kind "requirement def") (id (node (document "d0") (qualified-name "TradeStudies::MinimizeObjective"))) (name "MinimizeObjective") (declared-name "MinimizeObjective")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "TradeStudies::MinimizeObjective::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "TradeStudies::MinimizeObjective")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "TradeStudies::MinimizeObjective::best"))) (name "best") (declared-name "best") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "TradeStudies::MinimizeObjective")))))
          )
        )
        (element (kind "analysis def") (id (node (document "d0") (qualified-name "TradeStudies::TradeStudy"))) (name "TradeStudy") (declared-name "TradeStudy")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "TradeStudies::TradeStudy::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "TradeStudies::TradeStudy")))))
            (element (kind "calc") (id (node (document "d0") (qualified-name "TradeStudies::TradeStudy::evaluationFunction"))) (name "evaluationFunction") (declared-name "evaluationFunction") (effective (featuring-type (node (document "d0") (qualified-name "TradeStudies::TradeStudy"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "TradeStudies::TradeStudy::evaluationFunction::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "TradeStudies::EvaluationFunction")))))
              )
            )
            (element (kind "analysis result") (id (node (document "d0") (qualified-name "TradeStudies::TradeStudy::selectedAlternative"))) (name "selectedAlternative") (declared-name "selectedAlternative") (effective (featuring-type (node (document "d0") (qualified-name "TradeStudies::TradeStudy")))))
            (element (kind "subject") (id (node (document "d0") (qualified-name "TradeStudies::TradeStudy::studyAlternatives"))) (name "studyAlternatives") (declared-name "studyAlternatives") (effective (featuring-type (node (document "d0") (qualified-name "TradeStudies::TradeStudy")))))
            (element (kind "objective") (id (node (document "d0") (qualified-name "TradeStudies::TradeStudy::tradeStudyObjective"))) (name "tradeStudyObjective") (declared-name "tradeStudyObjective") (effective (featuring-type (node (document "d0") (qualified-name "TradeStudies::TradeStudy")))))
          )
        )
        (element (kind "requirement def") (id (node (document "d0") (qualified-name "TradeStudies::TradeStudyObjective"))) (name "TradeStudyObjective") (declared-name "TradeStudyObjective")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "TradeStudies::TradeStudyObjective::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "TradeStudies::TradeStudyObjective")))))
            (element (kind "require constraint") (id (node (document "d0") (qualified-name "TradeStudies::TradeStudyObjective::_requireConstraint_0"))) (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (effective (featuring-type (node (document "d0") (qualified-name "TradeStudies::TradeStudyObjective")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "TradeStudies::TradeStudyObjective::best"))) (name "best") (declared-name "best") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "TradeStudies::TradeStudyObjective")))))
            (element (kind "subject") (id (node (document "d0") (qualified-name "TradeStudies::TradeStudyObjective::selectedAlternative"))) (name "selectedAlternative") (declared-name "selectedAlternative") (effective (featuring-type (node (document "d0") (qualified-name "TradeStudies::TradeStudyObjective")))))
          )
        )
        (element (kind "documentation") (id (node (document "d0") (qualified-name "TradeStudies::_documentation"))) (name ""))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "TradeStudies::EvaluationFunction::_documentation"))) (to (node (document "d0") (qualified-name "TradeStudies::EvaluationFunction"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "TradeStudies::MaximizeObjective::_documentation"))) (to (node (document "d0") (qualified-name "TradeStudies::MaximizeObjective"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "TradeStudies::MinimizeObjective::_documentation"))) (to (node (document "d0") (qualified-name "TradeStudies::MinimizeObjective"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "TradeStudies::TradeStudy::_documentation"))) (to (node (document "d0") (qualified-name "TradeStudies::TradeStudy"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "TradeStudies::TradeStudy::evaluationFunction::_documentation"))) (to (node (document "d0") (qualified-name "TradeStudies::TradeStudy::evaluationFunction"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "TradeStudies::TradeStudyObjective::_documentation"))) (to (node (document "d0") (qualified-name "TradeStudies::TradeStudyObjective"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "TradeStudies::_documentation"))) (to (node (document "d0") (qualified-name "TradeStudies"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "TradeStudies::MaximizeObjective::best"))) (to (node (document "d0") (qualified-name "TradeStudies::TradeStudyObjective::best"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "TradeStudies::MinimizeObjective::best"))) (to (node (document "d0") (qualified-name "TradeStudies::TradeStudyObjective::best"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "TradeStudies::MaximizeObjective"))) (to (node (document "d0") (qualified-name "TradeStudies::TradeStudyObjective"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "TradeStudies::MinimizeObjective"))) (to (node (document "d0") (qualified-name "TradeStudies::TradeStudyObjective"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "TradeStudies::TradeStudyObjective"))) (to (node (document "d0") (qualified-name "TradeStudies::TradeStudyObjective::selectedAlternative"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "TradeStudies::TradeStudy::evaluationFunction"))) (to (node (document "d0") (qualified-name "TradeStudies::EvaluationFunction"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "TradeStudies::TradeStudy::tradeStudyObjective"))) (to (node (document "d0") (qualified-name "TradeStudies::TradeStudyObjective"))))
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
  (document "sysml.library/trade_studies.md"
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
        (range (start 8 16) (end 8 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 16) (end 9 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 19 2) (end 19 96))
      )
      (diagnostic
        (severity warning)
        (code "analysis_evaluation_unresolved")
        (source "semantic")
        (range (start 34 1) (end 34 1358))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 44 2) (end 44 168))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 67 2) (end 67 326))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 125 2) (end 125 257))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 161 2) (end 161 282))
      )
    )
  )
)
~~~
