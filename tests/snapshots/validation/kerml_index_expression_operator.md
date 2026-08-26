# META
~~~ini
description=KerML 8.3.4.8.6 validateIndexExpressionOperator requires the operator of an IndexExpression to be '#'
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.8.6 validateIndexExpressionOperator
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.4.8.6:validateIndexExpressionOperator
type=file
~~~
# SOURCE
~~~kerml
// Conforming: the operator of this expression comes from the concrete syntax token itself.
//
// The violating side has no textual counterpart: KerML concrete syntax fixes the operator of a
// IndexExpression at the point the expression is recognised, so a source document cannot author one
// carrying a different operator.
package Expressions {
    classifier Thing;
    classifier Holder {
        feature items : Thing;
        feature first = items#(1);
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_index_expression_operator.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_index_expression_operator.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:ec89ce142c20702e1999ee1620e3a6d356420a1e981ec1bc5cbc7d0d33ec9706") (contract-version "feature-value-expression-results-v5"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_index_expression_operator.md") (qualified-name "Expressions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_index_expression_operator.md") (qualified-name "Expressions::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_index_expression_operator.md") (qualified-name "Expressions::Holder::first"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/kerml_index_expression_operator.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "first")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/kerml_index_expression_operator.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "first")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_index_expression_operator.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "first")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_index_expression_operator.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "first")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "items")))))
    (declaration (id (node (document "memory://snapshot/kerml_index_expression_operator.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "first")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_index_expression_operator.md") (qualified-name "Expressions::Holder::items"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_index_expression_operator.md") (qualified-name "Expressions::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_index_expression_operator.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "first")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "items")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_index_expression_operator.md") (qualified-name "Expressions::Holder::items")))))
    (reference (id (source (node (document "memory://snapshot/kerml_index_expression_operator.md") (qualified-name "Expressions::Holder::items"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_index_expression_operator.md") (qualified-name "Expressions::Thing")))))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/kerml_index_expression_operator.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "first")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_index_expression_operator.md") (qualified-name "Expressions::Holder::items"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_index_expression_operator.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "first")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_index_expression_operator.md") (qualified-name "Expressions::Holder::items"))) (target (node (document "memory://snapshot/kerml_index_expression_operator.md") (qualified-name "Expressions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_index_expression_operator.md") (qualified-name "Expressions::Holder::items"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_index_expression_operator.md") (qualified-name "Expressions::Holder::first"))) (target (node (document "memory://snapshot/kerml_index_expression_operator.md") (qualified-name "Expressions::Holder"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_index_expression_operator.md") (qualified-name "Expressions::Holder::first"))) (target (node (document "memory://snapshot/kerml_index_expression_operator.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "first")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_index_expression_operator.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "first")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_index_expression_operator.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "first")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_index_expression_operator.md") (qualified-name "Expressions::Holder::items"))) (target (node (document "memory://snapshot/kerml_index_expression_operator.md") (qualified-name "Expressions::Holder"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_index_expression_operator.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "first")) (anonymous (kind kerml-expression) (ordinal 0))))) (state unsupported))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_index_expression_operator.md") (qualified-name "Expressions::Holder::first")))
      (featured-by (node (document "memory://snapshot/kerml_index_expression_operator.md") (qualified-name "Expressions::Holder")))
      (supertype (node (document "memory://snapshot/kerml_index_expression_operator.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "first")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_index_expression_operator.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "first")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_index_expression_operator.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "first")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/kerml_index_expression_operator.md") (qualified-name "Expressions::Holder::first")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_index_expression_operator.md") (qualified-name "Expressions::Holder::items")))
      (featured-by (node (document "memory://snapshot/kerml_index_expression_operator.md") (qualified-name "Expressions::Holder")))
      (type (node (document "memory://snapshot/kerml_index_expression_operator.md") (qualified-name "Expressions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_index_expression_operator.md") (qualified-name "Expressions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_index_expression_operator.md") (qualified-name "Expressions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_index_expression_operator.md") (qualified-name "Expressions::Thing")))
      (subtype (node (document "memory://snapshot/kerml_index_expression_operator.md") (qualified-name "Expressions::Holder::items")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_index_expression_operator.md") (range (start 9 24) (end 9 29)) (probe (position 9 24))
    (reference (id (source (node (document "memory://snapshot/kerml_index_expression_operator.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "first")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "items")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_index_expression_operator.md") (qualified-name "Expressions::Holder::items")))))
    )
  )
  (query (document "memory://snapshot/kerml_index_expression_operator.md") (range (start 8 24) (end 8 29)) (probe (position 8 24))
    (reference (id (source (node (document "memory://snapshot/kerml_index_expression_operator.md") (qualified-name "Expressions::Holder::items"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_index_expression_operator.md") (qualified-name "Expressions::Thing")))))
    )
  )
)
~~~
