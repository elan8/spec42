# META
~~~ini
description=KerML 8.3.4.7.3 checkExpressionResultBindingConnector requires each result expression membership to have its canonical binding connector
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.4.7.3:checkExpressionResultBindingConnector
blocked_by=lowering-result-expression-memberships
type=file
~~~
# SOURCE
~~~kerml
package Expressions {
    classifier Thing;
    expr Value {
        return feature result : Thing;
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
  (document "memory://snapshot/kerml_expression_result_binding_connector.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:bb997dfc7214a67c0d5af380360a278d15bca74e8edf1672f38ad47a3d532db7") (contract-version "constructor-expression-result-v8"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_binding_connector.md") (qualified-name "Expressions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_binding_connector.md") (qualified-name "Expressions::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_binding_connector.md") (qualified-name "Expressions::Value"))) (kind kerml-expression) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_binding_connector.md") (qualified-name "Expressions::Value::result"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_expression_result_binding_connector.md") (qualified-name "Expressions::Value::result"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_expression_result_binding_connector.md") (qualified-name "Expressions::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_expression_result_binding_connector.md") (qualified-name "Expressions::Value::result"))) (target (node (document "memory://snapshot/kerml_expression_result_binding_connector.md") (qualified-name "Expressions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_expression_result_binding_connector.md") (qualified-name "Expressions::Value::result"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_expression_result_binding_connector.md") (qualified-name "Expressions::Value::result"))) (target (node (document "memory://snapshot/kerml_expression_result_binding_connector.md") (qualified-name "Expressions::Value"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_binding_connector.md") (qualified-name "Expressions::Thing")))
      (subtype (node (document "memory://snapshot/kerml_expression_result_binding_connector.md") (qualified-name "Expressions::Value::result")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_binding_connector.md") (qualified-name "Expressions::Value::result")))
      (featured-by (node (document "memory://snapshot/kerml_expression_result_binding_connector.md") (qualified-name "Expressions::Value")))
      (type (node (document "memory://snapshot/kerml_expression_result_binding_connector.md") (qualified-name "Expressions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_expression_result_binding_connector.md") (qualified-name "Expressions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_expression_result_binding_connector.md") (qualified-name "Expressions::Thing")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_expression_result_binding_connector.md") (range (start 3 32) (end 3 37)) (probe (position 3 32))
    (reference (id (source (node (document "memory://snapshot/kerml_expression_result_binding_connector.md") (qualified-name "Expressions::Value::result"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_expression_result_binding_connector.md") (qualified-name "Expressions::Thing")))))
    )
  )
)
~~~
