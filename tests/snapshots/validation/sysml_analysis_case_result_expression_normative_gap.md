# META
~~~ini
description=SysML 8.3.23.2 and 8.3.23.3 analysis case result expressions navigate a property that does not exist in their pinned bodies
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=sysml-2.0:8.3.23.2:deriveAnalysisCaseDefinitionResultExpression
rule_id=sysml-2.0:8.3.23.3:deriveAnalysisCaseUsageResultExpression
blocked_by=normative-specification-gap-analysis-case-result-expression-bodies
libraries=standard
type=file
~~~
# SOURCE
~~~sysml
package Analyses {
    analysis def Range {
        subject distance : ScalarValues::Real;
        distance * 2
    }
    analysis range : Range {
        distance * 3
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (action-derived-fact (rule_id "sysml-2.0:8.3.23.2:deriveAnalysisCaseDefinitionResultExpression") (source "Analyses::Range") (outcome resolved))
  (action-derived-fact (rule_id "sysml-2.0:8.3.23.3:deriveAnalysisCaseUsageResultExpression") (source "Analyses::range") (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:10d5c0464775341ecbd2682883acdc021105d8d88080a59ab2b1981fa318837c") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (qualified-name "Analyses"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (qualified-name "Analyses::Range"))) (kind analysis-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "distance")))))
    (declaration (id (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (path (named (kind package) (name "Analyses")) (named (kind analysis-def) (name "Range")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (qualified-name "Analyses::Range::distance"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValues::Real")))))
    (declaration (id (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (qualified-name "Analyses::range"))) (kind analysis) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Range")) (expressionOperand (reference "distance")))))
    (declaration (id (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (path (named (kind package) (name "Analyses")) (named (kind analysis) (name "range")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (qualified-name "Analyses::Range"))) (kind expressionOperand) (ordinal 0))
      (authored-target "distance")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (qualified-name "Analyses::Range::distance")))))
    (reference (id (source (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (qualified-name "Analyses::Range::distance"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")))))
    (reference (id (source (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (qualified-name "Analyses::range"))) (kind featureTyping) (ordinal 0))
      (authored-target "Range")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (qualified-name "Analyses::Range")))))
    (reference (id (source (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (qualified-name "Analyses::range"))) (kind expressionOperand) (ordinal 0))
      (authored-target "distance")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (qualified-name "Analyses::Range::distance")))))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (qualified-name "Analyses::Range"))) (target (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (qualified-name "Analyses::Range::distance"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (qualified-name "Analyses::Range"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (qualified-name "Analyses::Range::distance"))) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (qualified-name "Analyses::Range::distance"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (qualified-name "Analyses::range"))) (target (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (qualified-name "Analyses::Range"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (qualified-name "Analyses::range"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (qualified-name "Analyses::range"))) (target (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (qualified-name "Analyses::Range::distance"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (qualified-name "Analyses::range"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (qualified-name "Analyses::Range"))) (target (node (document "memory://snapshot/sysml.library/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (path (named (kind package) (name "Analyses")) (named (kind analysis-def) (name "Range")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalIntegerEvaluations"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (qualified-name "Analyses::Range::distance"))) (target (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (qualified-name "Analyses::Range"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (qualified-name "Analyses::Range::distance"))) (target (node (document "memory://snapshot/sysml.library/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase::subj"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (qualified-name "Analyses::range"))) (target (node (document "memory://snapshot/sysml.library/analysis_cases.md") (qualified-name "AnalysisCases::analysisCases"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (path (named (kind package) (name "Analyses")) (named (kind analysis) (name "range")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalIntegerEvaluations"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (qualified-name "Analyses::Range"))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (qualified-name "Analyses::range"))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (qualified-name "Analyses::Range")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/calculations.md") (qualified-name "Calculations::Calculation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/cases.md") (qualified-name "Cases::Case")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (qualified-name "Analyses::range")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (path (named (kind package) (name "Analyses")) (named (kind analysis-def) (name "Range")) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralIntegerEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalIntegerEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralIntegerEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalIntegerEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (qualified-name "Analyses::Range::distance")))
      (featured-by (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (qualified-name "Analyses::Range")))
      (type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/cases.md") (qualified-name "Cases::Case::subj"))))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase::subj")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/cases.md") (qualified-name "Cases::Case::subj")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Complex")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Number")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (qualified-name "Analyses::range")))
      (type (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (qualified-name "Analyses::Range")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase")) (source inherited) (from (node (document "memory://snapshot/sysml.library/analysis_cases.md") (qualified-name "AnalysisCases::analysisCases"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/calculations.md") (qualified-name "Calculations::Calculation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/calculations.md") (qualified-name "Calculations::calculations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/cases.md") (qualified-name "Cases::Case")) (source inherited) (from (node (document "memory://snapshot/sysml.library/cases.md") (qualified-name "Cases::cases"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (effective-type (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (qualified-name "Analyses::Range")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/analysis_cases.md") (qualified-name "AnalysisCases::analysisCases")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/calculations.md") (qualified-name "Calculations::Calculation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/calculations.md") (qualified-name "Calculations::calculations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/cases.md") (qualified-name "Cases::Case")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/cases.md") (qualified-name "Cases::cases")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (qualified-name "Analyses::Range")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (path (named (kind package) (name "Analyses")) (named (kind analysis) (name "range")) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralIntegerEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalIntegerEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralIntegerEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalIntegerEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (qualified-name "Analyses::Range"))) (outcome resolved) (operator "*" (feature-reference "distance" (target (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (qualified-name "Analyses::Range::distance")))) (literal (value (kind integer) (integer 2)))))
  (declaration (id (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (qualified-name "Analyses::range"))) (outcome resolved) (operator "*" (feature-reference "distance" (target (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (qualified-name "Analyses::Range::distance")))) (literal (value (kind integer) (integer 3)))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (range (start 3 8) (end 3 16)) (probe (position 3 8))
    (reference (id (source (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (qualified-name "Analyses::Range"))) (kind expressionOperand) (ordinal 0) (authored-target "distance")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (qualified-name "Analyses::Range::distance")))))
    )
  )
  (query (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (range (start 2 27) (end 2 45)) (probe (position 2 27))
    (reference (id (source (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (qualified-name "Analyses::Range::distance"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")))))
    )
  )
  (query (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (range (start 5 21) (end 5 26)) (probe (position 5 21))
    (reference (id (source (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (qualified-name "Analyses::range"))) (kind featureTyping) (ordinal 0) (authored-target "Range")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (qualified-name "Analyses::Range")))))
    )
  )
  (query (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (range (start 6 8) (end 6 16)) (probe (position 6 8))
    (reference (id (source (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (qualified-name "Analyses::range"))) (kind expressionOperand) (ordinal 0) (authored-target "distance")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_analysis_case_result_expression_normative_gap.md") (qualified-name "Analyses::Range::distance")))))
    )
  )
)
~~~
