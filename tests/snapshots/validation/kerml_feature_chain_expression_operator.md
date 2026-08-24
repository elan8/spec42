# META
~~~ini
description=KerML 8.3.4.8.4 validateFeatureChainExpressionOperator requires the operator of a FeatureChainExpression to be '.'
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.8.4 validateFeatureChainExpressionOperator
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.4.8.4:validateFeatureChainExpressionOperator
type=file
~~~
# SOURCE
~~~kerml
// Conforming: the operator of this expression comes from the concrete syntax token itself.
//
// The violating side has no textual counterpart: KerML concrete syntax fixes the operator of a
// FeatureChainExpression at the point the expression is recognised, so a source document cannot author one
// carrying a different operator.
package Expressions {
    classifier Thing {
        feature inner : Thing;
    }
    classifier Holder {
        feature outer : Thing;
        feature reached = outer.inner;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_chain_expression_operator.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_chain_expression_operator.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:d06175e7f66d5edc2c3d0b04ab3464234f6a742369554a9b2ca44c0b9a1ca01d") (contract-version "parser-owned-resolution-v2"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions::Holder::outer"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions::Holder::reached"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "outer::inner")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions::Thing::inner"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions::Holder::outer"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions::Holder::reached"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "outer::inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions::Thing::inner")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions::Thing::inner"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions::Holder::outer"))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions::Holder::outer"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions::Holder::reached"))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions::Thing::inner"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions::Holder::reached"))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions::Thing::inner"))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions::Thing::inner"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions::Holder::outer"))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions::Holder::reached"))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions::Thing::inner"))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions::Thing"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions::Holder::reached"))) (state unsupported))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions::Holder::outer")))
      (featured-by (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions::Holder")))
      (type (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions::Holder::reached")))
      (featured-by (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions::Holder")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions::Thing")))
      (subtype (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions::Holder::outer")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions::Thing::inner")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions::Thing::inner")))
      (featured-by (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions::Thing")))
      (type (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions::Thing")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (range (start 10 24) (end 10 29)) (probe (position 10 24))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions::Holder::outer"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (range (start 11 26) (end 11 37)) (probe (position 11 26))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions::Holder::reached"))) (kind memberAccessOperand) (ordinal 0) (authored-target "outer::inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions::Thing::inner")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (range (start 7 24) (end 7 29)) (probe (position 7 24))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions::Thing::inner"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chain_expression_operator.md") (qualified-name "Expressions::Thing")))))
    )
  )
)
~~~
