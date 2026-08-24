# META
~~~ini
description=KerML 8.3.4.8.5 validateFeatureReferenceExpressionResult requires a FeatureReferenceExpression to own its result parameter
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.8.5 validateFeatureReferenceExpressionResult
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.4.8.5:validateFeatureReferenceExpressionResult
type=file
~~~
# SOURCE
~~~kerml
// Conforming: the feature reference expression below owns the result parameter its value is
// bound to.
//
// The violating side has no textual counterpart: a KerML source document never authors an
// expression's result parameter separately from the expression, so it cannot produce a
// FeatureReferenceExpression whose result parameter is owned by another type.
package Expressions {
    classifier Thing;
    classifier Holder {
        feature referent : Thing;
        feature reference = referent;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_reference_expression_result.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_reference_expression_result.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:a6c733b2344a11a6586c11041238ff8cceb65658091d0f4b87e616ec8304a035") (contract-version "parser-owned-resolution-v2"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_result.md") (qualified-name "Expressions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_result.md") (qualified-name "Expressions::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_result.md") (qualified-name "Expressions::Holder::reference"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "referent")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_result.md") (qualified-name "Expressions::Holder::referent"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_result.md") (qualified-name "Expressions::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_reference_expression_result.md") (qualified-name "Expressions::Holder::reference"))) (kind expressionOperand) (ordinal 0))
      (authored-target "referent")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_reference_expression_result.md") (qualified-name "Expressions::Holder::referent")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_reference_expression_result.md") (qualified-name "Expressions::Holder::referent"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_reference_expression_result.md") (qualified-name "Expressions::Thing")))))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/kerml_feature_reference_expression_result.md") (qualified-name "Expressions::Holder::reference"))) (target (node (document "memory://snapshot/kerml_feature_reference_expression_result.md") (qualified-name "Expressions::Holder::referent"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_reference_expression_result.md") (qualified-name "Expressions::Holder::reference"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_reference_expression_result.md") (qualified-name "Expressions::Holder::referent"))) (target (node (document "memory://snapshot/kerml_feature_reference_expression_result.md") (qualified-name "Expressions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_reference_expression_result.md") (qualified-name "Expressions::Holder::referent"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_reference_expression_result.md") (qualified-name "Expressions::Holder::reference"))) (target (node (document "memory://snapshot/kerml_feature_reference_expression_result.md") (qualified-name "Expressions::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_reference_expression_result.md") (qualified-name "Expressions::Holder::referent"))) (target (node (document "memory://snapshot/kerml_feature_reference_expression_result.md") (qualified-name "Expressions::Holder"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_feature_reference_expression_result.md") (qualified-name "Expressions::Holder::reference"))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_result.md") (qualified-name "Expressions::Holder::reference")))
      (featured-by (node (document "memory://snapshot/kerml_feature_reference_expression_result.md") (qualified-name "Expressions::Holder")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_result.md") (qualified-name "Expressions::Holder::referent")))
      (featured-by (node (document "memory://snapshot/kerml_feature_reference_expression_result.md") (qualified-name "Expressions::Holder")))
      (type (node (document "memory://snapshot/kerml_feature_reference_expression_result.md") (qualified-name "Expressions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_reference_expression_result.md") (qualified-name "Expressions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_reference_expression_result.md") (qualified-name "Expressions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_result.md") (qualified-name "Expressions::Thing")))
      (subtype (node (document "memory://snapshot/kerml_feature_reference_expression_result.md") (qualified-name "Expressions::Holder::referent")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_reference_expression_result.md") (range (start 10 28) (end 10 36)) (probe (position 10 28))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_reference_expression_result.md") (qualified-name "Expressions::Holder::reference"))) (kind expressionOperand) (ordinal 0) (authored-target "referent")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_reference_expression_result.md") (qualified-name "Expressions::Holder::referent")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_reference_expression_result.md") (range (start 9 27) (end 9 32)) (probe (position 9 27))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_reference_expression_result.md") (qualified-name "Expressions::Holder::referent"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_reference_expression_result.md") (qualified-name "Expressions::Thing")))))
    )
  )
)
~~~
