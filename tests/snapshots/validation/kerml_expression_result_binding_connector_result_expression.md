# META
~~~ini
description=KerML 8.3.4.7.3 checkExpressionResultBindingConnector over an expression that owns a result expression: the implied binding connector between the expression result and the result expression's result
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.4.7.3:checkExpressionResultBindingConnector
blocked_by=lowering-gap-result-expression-binding-endpoints
type=file
~~~
# SOURCE
~~~kerml
package Expressions {
    classifier Thing;
    expr Value {
        in feature input : Thing;
        return feature result : Thing;
        input
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (binding-connector-check
    (rule_id "kerml-1.0:8.3.4.7.3:checkExpressionResultBindingConnector")
    (outcome satisfied)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:ce819822219741ff88323703266f0052ff9641c6c307adff3546cf814941b749"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (qualified-name "Expressions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (qualified-name "Expressions::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (qualified-name "Expressions::Value"))) (kind kerml-expression) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "input")))))
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (qualified-name "Expressions::Value::input"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing") (direction in)))))
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (qualified-name "Expressions::Value::result"))) (kind parameter) (membership (kind feature) (visibility default) (role return-parameter)) (authored (membership (kind feature) (visibility default) (role return-parameter)) (relationships (featureTyping (reference "Thing")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (qualified-name "Expressions::Value"))) (kind expressionOperand) (ordinal 0))
      (authored-target "input")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (qualified-name "Expressions::Value::input")))))
    (reference (id (source (node (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (qualified-name "Expressions::Value::input"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (qualified-name "Expressions::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (qualified-name "Expressions::Value::result"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (qualified-name "Expressions::Thing")))))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (qualified-name "Expressions::Value"))) (target (node (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (qualified-name "Expressions::Value::input"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (qualified-name "Expressions::Value"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (qualified-name "Expressions::Value::input"))) (target (node (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (qualified-name "Expressions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (qualified-name "Expressions::Value::input"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (qualified-name "Expressions::Value::result"))) (target (node (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (qualified-name "Expressions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (qualified-name "Expressions::Value::result"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (qualified-name "Expressions::Value::input"))) (target (node (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (qualified-name "Expressions::Value"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (qualified-name "Expressions::Value::result"))) (target (node (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (qualified-name "Expressions::Value"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (qualified-name "Expressions::Value"))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (qualified-name "Expressions::Thing")))
      (subtype (node (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (qualified-name "Expressions::Value::input")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (qualified-name "Expressions::Value::result")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (qualified-name "Expressions::Value::input")))
      (featured-by (node (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (qualified-name "Expressions::Value")))
      (type (node (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (qualified-name "Expressions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (qualified-name "Expressions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (qualified-name "Expressions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (qualified-name "Expressions::Value::result")))
      (featured-by (node (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (qualified-name "Expressions::Value")))
      (type (node (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (qualified-name "Expressions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (qualified-name "Expressions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (qualified-name "Expressions::Thing")) (scopes any))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (qualified-name "Expressions::Value"))) (outcome resolved) (feature-reference "input" (target (node (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (qualified-name "Expressions::Value::input")))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (range (start 5 8) (end 5 13)) (probe (position 5 8))
    (reference (id (source (node (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (qualified-name "Expressions::Value"))) (kind expressionOperand) (ordinal 0) (authored-target "input")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (qualified-name "Expressions::Value::input")))))
    )
  )
  (query (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (range (start 3 27) (end 3 32)) (probe (position 3 27))
    (reference (id (source (node (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (qualified-name "Expressions::Value::input"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (qualified-name "Expressions::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (range (start 4 32) (end 4 37)) (probe (position 4 32))
    (reference (id (source (node (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (qualified-name "Expressions::Value::result"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_expression_result_binding_connector_result_expression.md") (qualified-name "Expressions::Thing")))))
    )
  )
)
~~~
