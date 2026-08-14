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
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 52 2) (end 58 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 60 2) (end 65 3))
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
        (range (start 76 23) (end 76 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 87 2) (end 94 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 94 3) (end 94 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 105 2) (end 112 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 112 3) (end 112 4))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 125 30) (end 125 38))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_analysis_case_definition_member")
        (source "semantic")
        (range (start 145 2) (end 159 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_analysis_case_definition_member")
        (source "semantic")
        (range (start 161 2) (end 167 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_analysis_case_definition_member")
        (source "semantic")
        (range (start 167 49) (end 167 50))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:5ed7630da059a289077f7d826fedb85589e626714badf7c70b4674d942fb5ff7") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Base::Anything") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarFunctions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ControlFunctions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::EvaluationFunction"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::EvaluationFunction::alternative"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Anything") (direction in))))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::EvaluationFunction::result"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValue"))))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::MaximizeObjective"))) (kind requirement-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "TradeStudyObjective"))))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::MinimizeObjective"))) (kind requirement-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "TradeStudyObjective"))))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy"))) (kind analysis-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy::evaluationFunction"))) (kind calc) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "EvaluationFunction"))))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy::studyAlternatives"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Anything"))))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (anonymous (kind constraint) (ordinal 0))))) (kind constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "selectedAlternative")) (expressionOperand (reference "best")) (invocationCallee (reference "eval"))))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::best"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValue"))))
    (declaration (id (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::selectedAlternative"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Anything"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarFunctions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ControlFunctions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
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
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::MinimizeObjective"))) (kind specialization) (ordinal 0))
      (authored-target "TradeStudyObjective")
      (outcome (status resolved) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective")))))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy::evaluationFunction"))) (kind featureTyping) (ordinal 0))
      (authored-target "EvaluationFunction")
      (outcome (status resolved) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::EvaluationFunction")))))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy::studyAlternatives"))) (kind featureTyping) (ordinal 0))
      (authored-target "Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (anonymous (kind constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "selectedAlternative")
      (outcome (status resolved) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::selectedAlternative")))))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (anonymous (kind constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 1))
      (authored-target "best")
      (outcome (status resolved) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::best")))))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (anonymous (kind constraint) (ordinal 0))))) (kind invocationCallee) (ordinal 0))
      (authored-target "eval")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::best"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::selectedAlternative"))) (kind featureTyping) (ordinal 0))
      (authored-target "Anything")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::MaximizeObjective"))) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::MaximizeObjective"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::MinimizeObjective"))) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::MinimizeObjective"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy::evaluationFunction"))) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::EvaluationFunction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy::evaluationFunction"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/trade_studies.md") (anonymous (kind constraint) (ordinal 0))))) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::selectedAlternative"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/trade_studies.md") (anonymous (kind constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/trade_studies.md") (anonymous (kind constraint) (ordinal 0))))) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::best"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/trade_studies.md") (anonymous (kind constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 1)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/trade_studies.md") (anonymous (kind constraint) (ordinal 0))))) (value (kind non-constant)))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/trade_studies.md") (range (start 7 16) (end 7 31)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/trade_studies.md") (range (start 8 16) (end 8 34)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarFunctions")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/trade_studies.md") (range (start 9 16) (end 9 35)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0) (authored-target "ControlFunctions")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/trade_studies.md") (range (start 6 16) (end 6 30)) (probe (position 6 16))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Base::Anything")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/trade_studies.md") (range (start 19 23) (end 19 31)) (probe (position 19 23))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::EvaluationFunction::alternative"))) (kind featureTyping) (ordinal 0) (authored-target "Anything")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/trade_studies.md") (range (start 26 28) (end 26 39)) (probe (position 26 28))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::EvaluationFunction::result"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/trade_studies.md") (range (start 97 38) (end 97 57)) (probe (position 97 38))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::MaximizeObjective"))) (kind specialization) (ordinal 0) (authored-target "TradeStudyObjective")
      (outcome (status resolved) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective")))))
  )
  (query (document "memory://snapshot/trade_studies.md") (range (start 79 38) (end 79 57)) (probe (position 79 38))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::MinimizeObjective"))) (kind specialization) (ordinal 0) (authored-target "TradeStudyObjective")
      (outcome (status resolved) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective")))))
  )
  (query (document "memory://snapshot/trade_studies.md") (range (start 135 37) (end 135 55)) (probe (position 135 37))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy::evaluationFunction"))) (kind featureTyping) (ordinal 0) (authored-target "EvaluationFunction")
      (outcome (status resolved) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::EvaluationFunction")))))
  )
  (query (document "memory://snapshot/trade_studies.md") (range (start 125 30) (end 125 38)) (probe (position 125 30))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudy::studyAlternatives"))) (kind featureTyping) (ordinal 0) (authored-target "Anything")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/trade_studies.md") (range (start 76 28) (end 76 47)) (probe (position 76 28))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (anonymous (kind constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "selectedAlternative")
      (outcome (status resolved) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::selectedAlternative")))))
  )
  (query (document "memory://snapshot/trade_studies.md") (range (start 76 52) (end 76 56)) (probe (position 76 52))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (anonymous (kind constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 1) (authored-target "best")
      (outcome (status resolved) (target (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::best")))))
  )
  (query (document "memory://snapshot/trade_studies.md") (range (start 76 23) (end 76 27)) (probe (position 76 23))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (anonymous (kind constraint) (ordinal 0))))) (kind invocationCallee) (ordinal 0) (authored-target "eval")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/trade_studies.md") (range (start 67 19) (end 67 30)) (probe (position 67 19))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::best"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/trade_studies.md") (range (start 44 32) (end 44 40)) (probe (position 44 32))
    (reference (id (source (node (document "memory://snapshot/trade_studies.md") (qualified-name "TradeStudies::TradeStudyObjective::selectedAlternative"))) (kind featureTyping) (ordinal 0) (authored-target "Anything")
      (outcome (status unresolved)))
  )
)
~~~
