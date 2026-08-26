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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/trade_studies.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 6 16) (end 6 30))
      )
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
        (range (start 7 16) (end 7 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 16) (end 9 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 19 23) (end 19 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 26 28) (end 26 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 44 32) (end 44 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 52 24) (end 52 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 67 19) (end 67 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 87 23) (end 87 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 105 23) (end 105 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 125 30) (end 125 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 161 31) (end 161 39))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:5ed7630da059a289077f7d826fedb85589e626714badf7c70b4674d942fb5ff7") (contract-version "operator-expression-arguments-v7"))
  (declarations
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies"))) (kind library-package) (membership (kind owning) (visibility default)) (facts (modifiers standard)) (documentation (doc (text "\n\t * This package provides a simple framework for defining trade-off study analysis cases.\n\t "))))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Base::Anything") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarFunctions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ControlFunctions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::EvaluationFunction"))) (kind calc-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * An EvaluationFunction is a calculation that evaluates a TradeStudy alternative,\n\t\t * producing a ScalarValue that can be comparted with the evaluation of other\n\t\t * alternatives.\n\t\t "))))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::EvaluationFunction::alternative"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (modifiers reference) (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Anything") (direction in)))))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::EvaluationFunction::result"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValue")))))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::MaximizeObjective"))) (kind requirement-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * A MaximizeObjective is a TradeStudyObjective that requires that the \n\t\t * selectedAlternative have the maximum ObjectiveFunction value of all the\n\t\t * given alternatives.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "TradeStudyObjective")))))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "MaximizeObjective")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "MaximizeObjective")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "MaximizeObjective")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "best")))))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "MaximizeObjective")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "MaximizeObjective")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "alternatives")))))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "MaximizeObjective")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::MinimizeObjective"))) (kind requirement-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * A MinimizeObjective is a TradeStudyObjective that requires that the \n\t\t * selectedAlternative have the minimum ObjectiveFunction value of all the\n\t\t * given alternatives.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "TradeStudyObjective")))))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "MinimizeObjective")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "MinimizeObjective")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "MinimizeObjective")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "best")))))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "MinimizeObjective")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "MinimizeObjective")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "alternatives")))))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "MinimizeObjective")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy"))) (kind analysis-def) (membership (kind owning) (visibility default)) (facts (modifiers abstract)) (documentation (doc (text "\n\t\t * A TradeStudy is an analysis case whose subject is a set of alternatives\n\t\t * (at least one) and whose result is a selection of one of those alternatives.\n\t\t * The alternatives are evaluated based on a given ObjectiveFunction and the\n\t\t * selection is made such that it satisfies the objective of the TradeStudy\n\t\t * (which must be a TradeStudyObjective).\n\t\t "))))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy::evaluationFunction"))) (kind calc) (membership (kind feature) (visibility default)) (facts (modifiers abstract)) (documentation (doc (text "\n\t\t\t * The EvaluationFunction to be used to evaluate the alternatives.\n\t\t\t * \n\t\t\t * In a TradeStudy usage, redefine this feature to provide the desired\n\t\t\t * calculation (or bind it to a calculation usage that does so).\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "EvaluationFunction")))))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy::selectedAlternative"))) (kind parameter) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind parameter) (name "selectedAlternative")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind parameter) (name "selectedAlternative")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Anything")))))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind parameter) (name "selectedAlternative")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind parameter) (name "selectedAlternative")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "studyAlternatives")))))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind parameter) (name "selectedAlternative")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy::studyAlternatives"))) (kind subject) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Anything")))))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy::tradeStudyObjective"))) (kind requirement) (membership (kind feature) (visibility default)) (documentation (doc (text "\n\t\t\t * The objective of this TradeStudy.\n\t\t\t * \n\t\t\t * Redefine this feature to give it a definition that is a concrete\n\t\t\t * specialization of TradeStudyObjective. That can either be one of the\n\t\t\t * specializations provided in this package, or a more specific user-\n\t\t\t * defined one.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TradeStudyObjective")))))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind requirement) (name "tradeStudyObjective")) (anonymous (kind subject) (ordinal 0))))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "selectedAlternative")))))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind requirement) (name "tradeStudyObjective")) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)) (facts (direction in)) (feature-value (kind bind) (value (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind requirement) (name "tradeStudyObjective")) (anonymous (kind ref) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind requirement) (name "tradeStudyObjective")) (anonymous (kind ref) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "alternatives")))))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind requirement) (name "tradeStudyObjective")) (anonymous (kind calc) (ordinal 0))))) (kind calc) (membership (kind feature) (visibility default)) (facts (direction in)) (feature-value (kind bind) (value (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind requirement) (name "tradeStudyObjective")) (anonymous (kind calc) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind requirement) (name "tradeStudyObjective")) (anonymous (kind calc) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "eval")))))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind requirement) (name "tradeStudyObjective")) (anonymous (kind ref) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind requirement) (name "tradeStudyObjective")) (anonymous (kind ref) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind requirement) (name "tradeStudyObjective")) (anonymous (kind calc) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind requirement) (name "tradeStudyObjective")) (anonymous (kind calc) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind requirement) (name "tradeStudyObjective")) (anonymous (kind ref) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind requirement) (name "tradeStudyObjective")) (anonymous (kind calc) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective"))) (kind requirement-def) (membership (kind owning) (visibility default)) (facts (modifiers abstract)) (documentation (doc (text "\n\t\t * A TradeStudyObjective is the base definition for the objective of a TradeStudy.\n\t\t * The requirement is to choose from a given set of alternatives the selectedAlternative\n\t\t * for that has the best evaluation according to a given EvaluationFunction. What\n\t\t * value is considered \"best\" is not defined in the abstract base definition but must be\n\t\t * computed in any concrete specialization.\n\t\t "))))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "TradeStudyObjective")) (anonymous (kind require-constraint) (ordinal 0))))) (kind require-constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "selectedAlternative")) (expressionOperand (reference "best")) (invocationCallee (reference "eval")))))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::alternatives"))) (kind ref) (membership (kind feature) (visibility default)) (facts (direction in) (multiplicity (lower 1) (upper unbounded))) (documentation (doc (text "\n\t\t\t * The alternatives being considered in the TradeStudy for which this TradeStudyObjective \n\t\t\t * is the objective.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Anything")))))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::best"))) (kind attribute) (membership (kind feature) (visibility default)) (documentation (doc (text "\n\t\t\t * Out of the evaluation results of all the given alternatives, the one that is considered \n\t\t\t * \"best\", in the sense that it is the value the selectedAlternative should have. This \n\t\t\t * value must be computed in any concrete specialization of TradeStudyObjective.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValue")))))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::eval"))) (kind calc) (membership (kind feature) (visibility default)) (facts (direction in)) (documentation (doc (text "\n\t\t\t * The EvaluationFunction to be used in evaluating the given alternatives.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "EvaluationFunction")))))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::selectedAlternative"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Anything")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarFunctions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ControlFunctions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Base::Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::EvaluationFunction::alternative"))) (kind featureTyping) (ordinal 0))
      (authored-target "Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::EvaluationFunction::result"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::MaximizeObjective"))) (kind specialization) (ordinal 0))
      (authored-target "TradeStudyObjective")
      (outcome (status resolved) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective")))))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "MaximizeObjective")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "best")
      (outcome (status resolved) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::best")))))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "MaximizeObjective")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "alternatives")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::MinimizeObjective"))) (kind specialization) (ordinal 0))
      (authored-target "TradeStudyObjective")
      (outcome (status resolved) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective")))))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "MinimizeObjective")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "best")
      (outcome (status resolved) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::best")))))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "MinimizeObjective")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "alternatives")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy::evaluationFunction"))) (kind featureTyping) (ordinal 0))
      (authored-target "EvaluationFunction")
      (outcome (status resolved) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::EvaluationFunction")))))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy::selectedAlternative"))) (kind featureTyping) (ordinal 0))
      (authored-target "Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind parameter) (name "selectedAlternative")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "studyAlternatives")
      (outcome (status resolved) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy::studyAlternatives")))))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy::studyAlternatives"))) (kind featureTyping) (ordinal 0))
      (authored-target "Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy::tradeStudyObjective"))) (kind featureTyping) (ordinal 0))
      (authored-target "TradeStudyObjective")
      (outcome (status resolved) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective")))))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind requirement) (name "tradeStudyObjective")) (anonymous (kind subject) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "selectedAlternative")
      (outcome (status resolved) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::selectedAlternative")))))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind requirement) (name "tradeStudyObjective")) (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "alternatives")
      (outcome (status resolved) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::alternatives")))))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind requirement) (name "tradeStudyObjective")) (anonymous (kind calc) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "eval")
      (outcome (status resolved) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::eval")))))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "TradeStudyObjective")) (anonymous (kind require-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "selectedAlternative")
      (outcome (status resolved) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::selectedAlternative")))))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "TradeStudyObjective")) (anonymous (kind require-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 1))
      (authored-target "best")
      (outcome (status resolved) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::best")))))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "TradeStudyObjective")) (anonymous (kind require-constraint) (ordinal 0))))) (kind invocationCallee) (ordinal 0))
      (authored-target "eval")
      (outcome (status resolved) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::eval")))))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::alternatives"))) (kind featureTyping) (ordinal 0))
      (authored-target "Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::best"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::eval"))) (kind featureTyping) (ordinal 0))
      (authored-target "EvaluationFunction")
      (outcome (status resolved) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::EvaluationFunction")))))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::selectedAlternative"))) (kind featureTyping) (ordinal 0))
      (authored-target "Anything")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::MaximizeObjective"))) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::MaximizeObjective"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "MaximizeObjective")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::best"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "MaximizeObjective")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::MinimizeObjective"))) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::MinimizeObjective"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "MinimizeObjective")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::best"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "MinimizeObjective")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy::evaluationFunction"))) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::EvaluationFunction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy::evaluationFunction"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind parameter) (name "selectedAlternative")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy::studyAlternatives"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind parameter) (name "selectedAlternative")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy::tradeStudyObjective"))) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy::tradeStudyObjective"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind requirement) (name "tradeStudyObjective")) (anonymous (kind subject) (ordinal 0))))) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::selectedAlternative"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind requirement) (name "tradeStudyObjective")) (anonymous (kind subject) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind requirement) (name "tradeStudyObjective")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::alternatives"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind requirement) (name "tradeStudyObjective")) (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind requirement) (name "tradeStudyObjective")) (anonymous (kind calc) (ordinal 0))))) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::eval"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind requirement) (name "tradeStudyObjective")) (anonymous (kind calc) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "TradeStudyObjective")) (anonymous (kind require-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::selectedAlternative"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "TradeStudyObjective")) (anonymous (kind require-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "TradeStudyObjective")) (anonymous (kind require-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::best"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "TradeStudyObjective")) (anonymous (kind require-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "TradeStudyObjective")) (anonymous (kind require-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::eval"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "TradeStudyObjective")) (anonymous (kind require-constraint) (ordinal 0))))) (kind invocationCallee) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::eval"))) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::EvaluationFunction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::eval"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::EvaluationFunction::alternative"))) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::EvaluationFunction"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::EvaluationFunction::result"))) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::EvaluationFunction"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "MaximizeObjective")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::MaximizeObjective"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "MaximizeObjective")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "MaximizeObjective")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "MinimizeObjective")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::MinimizeObjective"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "MinimizeObjective")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "MinimizeObjective")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy::evaluationFunction"))) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy::selectedAlternative"))) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind parameter) (name "selectedAlternative")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind parameter) (name "selectedAlternative")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy::studyAlternatives"))) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy::tradeStudyObjective"))) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind requirement) (name "tradeStudyObjective")) (anonymous (kind subject) (ordinal 0))))) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy::tradeStudyObjective"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind requirement) (name "tradeStudyObjective")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy::tradeStudyObjective"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind requirement) (name "tradeStudyObjective")) (anonymous (kind calc) (ordinal 0))))) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy::tradeStudyObjective"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind requirement) (name "tradeStudyObjective")) (anonymous (kind ref) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind requirement) (name "tradeStudyObjective")) (anonymous (kind ref) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind requirement) (name "tradeStudyObjective")) (anonymous (kind calc) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind requirement) (name "tradeStudyObjective")) (anonymous (kind calc) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "TradeStudyObjective")) (anonymous (kind require-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::alternatives"))) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::best"))) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::eval"))) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::selectedAlternative"))) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "MaximizeObjective")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "MinimizeObjective")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind parameter) (name "selectedAlternative")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "TradeStudyObjective")) (anonymous (kind require-constraint) (ordinal 0))))) (state non-constant))
    (invocation (declaration (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "TradeStudyObjective")) (anonymous (kind require-constraint) (ordinal 0))))) (callee (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::eval"))) (supplied 1) (required 0) (start 76 23) (end 76 48))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::EvaluationFunction")))
      (subtype (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy::evaluationFunction")) (scopes any))
      (subtype (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::eval")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::EvaluationFunction::alternative")))
      (featured-by (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::EvaluationFunction")))
    )
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::EvaluationFunction::result")))
      (featured-by (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::EvaluationFunction")))
    )
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::MaximizeObjective")))
      (supertype (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "MaximizeObjective")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::MaximizeObjective")))
      (supertype (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::best")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "MaximizeObjective")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "MaximizeObjective")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::MinimizeObjective")))
      (supertype (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "MinimizeObjective")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::MinimizeObjective")))
      (supertype (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::best")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "MinimizeObjective")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "MinimizeObjective")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy::evaluationFunction")))
      (featured-by (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy")))
      (type (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::EvaluationFunction")) (provenance authored))
      (effective-type (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::EvaluationFunction")) (source direct))
      (supertype (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::EvaluationFunction")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy::selectedAlternative")))
      (featured-by (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy")))
    )
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind parameter) (name "selectedAlternative")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind parameter) (name "selectedAlternative")) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy::studyAlternatives")))
      (featured-by (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy")))
    )
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy::tradeStudyObjective")))
      (featured-by (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy")))
      (type (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective")) (provenance authored))
      (effective-type (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective")) (source direct))
      (supertype (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind requirement) (name "tradeStudyObjective")) (anonymous (kind subject) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy::tradeStudyObjective")))
      (supertype (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::selectedAlternative")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind requirement) (name "tradeStudyObjective")) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy::tradeStudyObjective")))
      (supertype (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::alternatives")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind requirement) (name "tradeStudyObjective")) (anonymous (kind calc) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy::tradeStudyObjective")))
      (effective-type (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::EvaluationFunction")) (source inherited) (from (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::eval"))))
      (supertype (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::EvaluationFunction")) (scopes any))
      (supertype (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::eval")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind requirement) (name "tradeStudyObjective")) (anonymous (kind ref) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind requirement) (name "tradeStudyObjective")) (anonymous (kind ref) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind requirement) (name "tradeStudyObjective")) (anonymous (kind calc) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind requirement) (name "tradeStudyObjective")) (anonymous (kind calc) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective")))
      (subtype (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::MaximizeObjective")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::MinimizeObjective")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy::tradeStudyObjective")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "TradeStudyObjective")) (anonymous (kind require-constraint) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective")))
    )
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::alternatives")))
      (featured-by (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective")))
      (subtype (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind requirement) (name "tradeStudyObjective")) (anonymous (kind ref) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::best")))
      (featured-by (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective")))
      (subtype (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "MaximizeObjective")) (anonymous (kind attribute) (ordinal 0)))) (scopes any feature))
      (subtype (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "MinimizeObjective")) (anonymous (kind attribute) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::eval")))
      (featured-by (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective")))
      (type (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::EvaluationFunction")) (provenance authored))
      (effective-type (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::EvaluationFunction")) (source direct))
      (supertype (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::EvaluationFunction")) (scopes any))
      (subtype (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind requirement) (name "tradeStudyObjective")) (anonymous (kind calc) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::selectedAlternative")))
      (featured-by (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective")))
      (subtype (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind requirement) (name "tradeStudyObjective")) (anonymous (kind subject) (ordinal 0)))) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/trade_studies.md") (range (start 7 16) (end 7 31)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/trade_studies.md") (range (start 8 16) (end 8 34)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarFunctions")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/trade_studies.md") (range (start 9 16) (end 9 35)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0) (authored-target "ControlFunctions")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/trade_studies.md") (range (start 6 16) (end 6 30)) (probe (position 6 16))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Base::Anything")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/trade_studies.md") (range (start 19 23) (end 19 31)) (probe (position 19 23))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::EvaluationFunction::alternative"))) (kind featureTyping) (ordinal 0) (authored-target "Anything")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/trade_studies.md") (range (start 26 28) (end 26 39)) (probe (position 26 28))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::EvaluationFunction::result"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/trade_studies.md") (range (start 97 38) (end 97 57)) (probe (position 97 38))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::MaximizeObjective"))) (kind specialization) (ordinal 0) (authored-target "TradeStudyObjective")
      (outcome (status resolved) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective")))))
    )
  )
  (query (document "memory://snapshot/trade_studies.md") (range (start 105 16) (end 105 20)) (probe (position 105 16))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "MaximizeObjective")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "best")
      (outcome (status resolved) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::best")))))
    )
  )
  (query (document "memory://snapshot/trade_studies.md") (range (start 105 23) (end 105 35)) (probe (position 105 23))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "MaximizeObjective")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "alternatives")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/trade_studies.md") (range (start 79 38) (end 79 57)) (probe (position 79 38))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::MinimizeObjective"))) (kind specialization) (ordinal 0) (authored-target "TradeStudyObjective")
      (outcome (status resolved) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective")))))
    )
  )
  (query (document "memory://snapshot/trade_studies.md") (range (start 87 16) (end 87 20)) (probe (position 87 16))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "MinimizeObjective")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "best")
      (outcome (status resolved) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::best")))))
    )
  )
  (query (document "memory://snapshot/trade_studies.md") (range (start 87 23) (end 87 35)) (probe (position 87 23))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "MinimizeObjective")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "alternatives")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/trade_studies.md") (range (start 135 37) (end 135 55)) (probe (position 135 37))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy::evaluationFunction"))) (kind featureTyping) (ordinal 0) (authored-target "EvaluationFunction")
      (outcome (status resolved) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::EvaluationFunction")))))
    )
  )
  (query (document "memory://snapshot/trade_studies.md") (range (start 161 31) (end 161 39)) (probe (position 161 31))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy::selectedAlternative"))) (kind featureTyping) (ordinal 0) (authored-target "Anything")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/trade_studies.md") (range (start 161 42) (end 161 59)) (probe (position 161 42))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind parameter) (name "selectedAlternative")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "studyAlternatives")
      (outcome (status resolved) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy::studyAlternatives")))))
    )
  )
  (query (document "memory://snapshot/trade_studies.md") (range (start 125 30) (end 125 38)) (probe (position 125 30))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy::studyAlternatives"))) (kind featureTyping) (ordinal 0) (authored-target "Anything")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/trade_studies.md") (range (start 145 34) (end 145 53)) (probe (position 145 34))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy::tradeStudyObjective"))) (kind featureTyping) (ordinal 0) (authored-target "TradeStudyObjective")
      (outcome (status resolved) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective")))))
    )
  )
  (query (document "memory://snapshot/trade_studies.md") (range (start 156 24) (end 156 43)) (probe (position 156 24))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind requirement) (name "tradeStudyObjective")) (anonymous (kind subject) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "selectedAlternative")
      (outcome (status resolved) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::selectedAlternative")))))
    )
  )
  (query (document "memory://snapshot/trade_studies.md") (range (start 157 14) (end 157 26)) (probe (position 157 14))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind requirement) (name "tradeStudyObjective")) (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "alternatives")
      (outcome (status resolved) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::alternatives")))))
    )
  )
  (query (document "memory://snapshot/trade_studies.md") (range (start 158 15) (end 158 19)) (probe (position 158 15))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind analysis-def) (name "TradeStudy")) (named (kind requirement) (name "tradeStudyObjective")) (anonymous (kind calc) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "eval")
      (outcome (status resolved) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::eval")))))
    )
  )
  (query (document "memory://snapshot/trade_studies.md") (range (start 76 28) (end 76 47)) (probe (position 76 28))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "TradeStudyObjective")) (anonymous (kind require-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "selectedAlternative")
      (outcome (status resolved) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::selectedAlternative")))))
    )
  )
  (query (document "memory://snapshot/trade_studies.md") (range (start 76 52) (end 76 56)) (probe (position 76 52))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "TradeStudyObjective")) (anonymous (kind require-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 1) (authored-target "best")
      (outcome (status resolved) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::best")))))
    )
  )
  (query (document "memory://snapshot/trade_studies.md") (range (start 76 23) (end 76 27)) (probe (position 76 23))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (path (named (kind library-package) (name "TradeStudies")) (named (kind requirement-def) (name "TradeStudyObjective")) (anonymous (kind require-constraint) (ordinal 0))))) (kind invocationCallee) (ordinal 0) (authored-target "eval")
      (outcome (status resolved) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::eval")))))
    )
  )
  (query (document "memory://snapshot/trade_studies.md") (range (start 52 24) (end 52 32)) (probe (position 52 24))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::alternatives"))) (kind featureTyping) (ordinal 0) (authored-target "Anything")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/trade_studies.md") (range (start 67 19) (end 67 30)) (probe (position 67 19))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::best"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/trade_studies.md") (range (start 60 17) (end 60 35)) (probe (position 60 17))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::eval"))) (kind featureTyping) (ordinal 0) (authored-target "EvaluationFunction")
      (outcome (status resolved) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::EvaluationFunction")))))
    )
  )
  (query (document "memory://snapshot/trade_studies.md") (range (start 44 32) (end 44 40)) (probe (position 44 32))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::selectedAlternative"))) (kind featureTyping) (ordinal 0) (authored-target "Anything")
      (outcome (status unresolved)))
    )
  )
)
~~~
