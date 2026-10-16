# META
~~~ini
description=KerML 8.3.4.7.3 deriveExpressionIsModelLevelEvaluable needs every owned sub-expression, which is not lowered as an Expression element
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=kerml-1.0:8.3.4.7.3:deriveExpressionIsModelLevelEvaluable
blocked_by=lowering-gap-expression-model-level-evaluable
libraries=standard
type=file
~~~
# SOURCE
~~~kerml
package Evaluable {
    classifier Limits {
        feature total = 1 + 2;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (action-derived-fact (rule_id "kerml-1.0:8.3.4.7.3:deriveExpressionIsModelLevelEvaluable") (source (anonymous (owner "Evaluable::Limits::total") (kind Expression) (ordinal 0))) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_expression_model_level_evaluable_lowering_gap.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:a699af54bf25ed21fdc27ba37d35b00e75bfa82fd19f9583f79b868f0a06cb51") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_expression_model_level_evaluable_lowering_gap.md") (qualified-name "Evaluable"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_expression_model_level_evaluable_lowering_gap.md") (qualified-name "Evaluable::Limits"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_expression_model_level_evaluable_lowering_gap.md") (qualified-name "Evaluable::Limits::total"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/kerml_expression_model_level_evaluable_lowering_gap.md") (path (named (kind package) (name "Evaluable")) (named (kind kerml-classifier) (name "Limits")) (named (kind kerml-feature) (name "total")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/kerml_expression_model_level_evaluable_lowering_gap.md") (path (named (kind package) (name "Evaluable")) (named (kind kerml-classifier) (name "Limits")) (named (kind kerml-feature) (name "total")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_expression_model_level_evaluable_lowering_gap.md") (path (named (kind package) (name "Evaluable")) (named (kind kerml-classifier) (name "Limits")) (named (kind kerml-feature) (name "total")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_expression_model_level_evaluable_lowering_gap.md") (path (named (kind package) (name "Evaluable")) (named (kind kerml-classifier) (name "Limits")) (named (kind kerml-feature) (name "total")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_expression_model_level_evaluable_lowering_gap.md") (path (named (kind package) (name "Evaluable")) (named (kind kerml-classifier) (name "Limits")) (named (kind kerml-feature) (name "total")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_expression_model_level_evaluable_lowering_gap.md") (path (named (kind package) (name "Evaluable")) (named (kind kerml-classifier) (name "Limits")) (named (kind kerml-feature) (name "total")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_expression_model_level_evaluable_lowering_gap.md") (path (named (kind package) (name "Evaluable")) (named (kind kerml-classifier) (name "Limits")) (named (kind kerml-feature) (name "total")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)))
  )
  (references
  )
  (relationships
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_expression_model_level_evaluable_lowering_gap.md") (qualified-name "Evaluable::Limits::total"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_expression_model_level_evaluable_lowering_gap.md") (qualified-name "Evaluable::Limits::total"))) (target (node (document "memory://snapshot/kerml_expression_model_level_evaluable_lowering_gap.md") (qualified-name "Evaluable::Limits"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_expression_model_level_evaluable_lowering_gap.md") (qualified-name "Evaluable::Limits::total"))) (target (node (document "memory://snapshot/kerml_expression_model_level_evaluable_lowering_gap.md") (path (named (kind package) (name "Evaluable")) (named (kind kerml-classifier) (name "Limits")) (named (kind kerml-feature) (name "total")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_expression_model_level_evaluable_lowering_gap.md") (path (named (kind package) (name "Evaluable")) (named (kind kerml-classifier) (name "Limits")) (named (kind kerml-feature) (name "total")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_expression_model_level_evaluable_lowering_gap.md") (qualified-name "Evaluable::Limits"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_expression_model_level_evaluable_lowering_gap.md") (path (named (kind package) (name "Evaluable")) (named (kind kerml-classifier) (name "Limits")) (named (kind kerml-feature) (name "total")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_expression_model_level_evaluable_lowering_gap.md") (path (named (kind package) (name "Evaluable")) (named (kind kerml-classifier) (name "Limits")) (named (kind kerml-feature) (name "total")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_expression_model_level_evaluable_lowering_gap.md") (path (named (kind package) (name "Evaluable")) (named (kind kerml-classifier) (name "Limits")) (named (kind kerml-feature) (name "total")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_expression_model_level_evaluable_lowering_gap.md") (path (named (kind package) (name "Evaluable")) (named (kind kerml-classifier) (name "Limits")) (named (kind kerml-feature) (name "total")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_expression_model_level_evaluable_lowering_gap.md") (path (named (kind package) (name "Evaluable")) (named (kind kerml-classifier) (name "Limits")) (named (kind kerml-feature) (name "total")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation::result"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_expression_model_level_evaluable_lowering_gap.md") (path (named (kind package) (name "Evaluable")) (named (kind kerml-classifier) (name "Limits")) (named (kind kerml-feature) (name "total")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalIntegerEvaluations"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_expression_model_level_evaluable_lowering_gap.md") (path (named (kind package) (name "Evaluable")) (named (kind kerml-classifier) (name "Limits")) (named (kind kerml-feature) (name "total")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalIntegerEvaluations"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_expression_model_level_evaluable_lowering_gap.md") (path (named (kind package) (name "Evaluable")) (named (kind kerml-classifier) (name "Limits")) (named (kind kerml-feature) (name "total")) (anonymous (kind kerml-expression) (ordinal 0))))) (state evaluated) (value (kind integer) (integer 3)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_expression_model_level_evaluable_lowering_gap.md") (qualified-name "Evaluable::Limits::total")))
      (featured-by (node (document "memory://snapshot/kerml_expression_model_level_evaluable_lowering_gap.md") (qualified-name "Evaluable::Limits")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation::result"))))
      (supertype (node (document "memory://snapshot/kerml_expression_model_level_evaluable_lowering_gap.md") (path (named (kind package) (name "Evaluable")) (named (kind kerml-classifier) (name "Limits")) (named (kind kerml-feature) (name "total")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation::result")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_expression_model_level_evaluable_lowering_gap.md") (path (named (kind package) (name "Evaluable")) (named (kind kerml-classifier) (name "Limits")) (named (kind kerml-feature) (name "total")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_expression_model_level_evaluable_lowering_gap.md") (qualified-name "Evaluable::Limits")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_expression_model_level_evaluable_lowering_gap.md") (path (named (kind package) (name "Evaluable")) (named (kind kerml-classifier) (name "Limits")) (named (kind kerml-feature) (name "total")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_expression_model_level_evaluable_lowering_gap.md") (path (named (kind package) (name "Evaluable")) (named (kind kerml-classifier) (name "Limits")) (named (kind kerml-feature) (name "total")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation::result"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation::result")) (scopes any feature))
      (subtype (node (document "memory://snapshot/kerml_expression_model_level_evaluable_lowering_gap.md") (qualified-name "Evaluable::Limits::total")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_expression_model_level_evaluable_lowering_gap.md") (path (named (kind package) (name "Evaluable")) (named (kind kerml-classifier) (name "Limits")) (named (kind kerml-feature) (name "total")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
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
    (declaration (id (node (document "memory://snapshot/kerml_expression_model_level_evaluable_lowering_gap.md") (path (named (kind package) (name "Evaluable")) (named (kind kerml-classifier) (name "Limits")) (named (kind kerml-feature) (name "total")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)))))
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
  (declaration (id (node (document "memory://snapshot/kerml_expression_model_level_evaluable_lowering_gap.md") (path (named (kind package) (name "Evaluable")) (named (kind kerml-classifier) (name "Limits")) (named (kind kerml-feature) (name "total")) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (operator "+" (literal (value (kind integer) (integer 1))) (literal (value (kind integer) (integer 2)))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
