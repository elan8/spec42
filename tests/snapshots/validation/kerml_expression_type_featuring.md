# META
~~~ini
description=KerML 8.3.4.7.3 checkExpressionTypeFeaturing requires an Expression owned by a FeatureValue to share the featureWithValue featuringTypes
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.4.7.3:checkExpressionTypeFeaturing
blocked_by=lowering-gap-type-featuring-expression-feature-value-owner
type=file
~~~
# SOURCE
~~~kerml
package Expressions {
    classifier Thing;
    classifier Holder {
        feature referent : Thing;
        feature value = referent;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship
    (kind type_featuring)
    (source "Expressions::Holder::value::expression")
    (target "Expressions::Holder")
    (provenance implied)
    (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_expression_type_featuring.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:c215891ba262123cff9247c6dfbe8586271f2a65a8ae6a5643a1c57fb7e3a2ec") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder::referent"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder::value"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "referent")))))
    (declaration (id (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder::referent"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder::value"))) (kind expressionOperand) (ordinal 0))
      (authored-target "referent")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder::referent")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder::referent"))) (target (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder::referent"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder::value"))) (target (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder::referent"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder::value"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder::referent"))) (target (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder::value"))) (target (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder::value"))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder::referent")))
      (featured-by (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder")))
      (type (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder::value")))
      (featured-by (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Thing")))
      (subtype (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder::referent")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_expression_type_featuring.md") (range (start 3 27) (end 3 32)) (probe (position 3 27))
    (reference (id (source (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder::referent"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_expression_type_featuring.md") (range (start 4 24) (end 4 32)) (probe (position 4 24))
    (reference (id (source (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder::value"))) (kind expressionOperand) (ordinal 0) (authored-target "referent")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder::referent")))))
    )
  )
)
~~~
