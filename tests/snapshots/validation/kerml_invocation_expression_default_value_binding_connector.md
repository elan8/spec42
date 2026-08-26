# META
~~~ini
description=KerML 8.3.4.8.8 checkInvocationExpressionDefaultValueBindingConnector has an exact TBD body in the pinned normative XMI
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.4.8.8:checkInvocationExpressionDefaultValueBindingConnector
blocked_by=normative-specification-gap-invocation-expression-default-value-binding-connector
type=file
~~~
# SOURCE
~~~kerml
package Invocations {
    classifier Thing;
    function Identity {
        return feature result : Thing;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (binding-connector-check
    (rule_id "kerml-1.0:8.3.4.8.8:checkInvocationExpressionDefaultValueBindingConnector")
    (outcome satisfied)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_invocation_expression_default_value_binding_connector.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:609c4cd625050b90063ee6adae6685a48d8cfe7bd87d12b8c16abf3db5eae3bd") (contract-version "semantic-metadata-projection-v6"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_default_value_binding_connector.md") (qualified-name "Invocations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_default_value_binding_connector.md") (qualified-name "Invocations::Identity"))) (kind kerml-function) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_default_value_binding_connector.md") (qualified-name "Invocations::Identity::result"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_default_value_binding_connector.md") (qualified-name "Invocations::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_default_value_binding_connector.md") (qualified-name "Invocations::Identity::result"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_default_value_binding_connector.md") (qualified-name "Invocations::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_invocation_expression_default_value_binding_connector.md") (qualified-name "Invocations::Identity::result"))) (target (node (document "memory://snapshot/kerml_invocation_expression_default_value_binding_connector.md") (qualified-name "Invocations::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_invocation_expression_default_value_binding_connector.md") (qualified-name "Invocations::Identity::result"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_invocation_expression_default_value_binding_connector.md") (qualified-name "Invocations::Identity::result"))) (target (node (document "memory://snapshot/kerml_invocation_expression_default_value_binding_connector.md") (qualified-name "Invocations::Identity"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_default_value_binding_connector.md") (qualified-name "Invocations::Identity::result")))
      (featured-by (node (document "memory://snapshot/kerml_invocation_expression_default_value_binding_connector.md") (qualified-name "Invocations::Identity")))
      (type (node (document "memory://snapshot/kerml_invocation_expression_default_value_binding_connector.md") (qualified-name "Invocations::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_invocation_expression_default_value_binding_connector.md") (qualified-name "Invocations::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_invocation_expression_default_value_binding_connector.md") (qualified-name "Invocations::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_default_value_binding_connector.md") (qualified-name "Invocations::Thing")))
      (subtype (node (document "memory://snapshot/kerml_invocation_expression_default_value_binding_connector.md") (qualified-name "Invocations::Identity::result")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_invocation_expression_default_value_binding_connector.md") (range (start 3 32) (end 3 37)) (probe (position 3 32))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_default_value_binding_connector.md") (qualified-name "Invocations::Identity::result"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_default_value_binding_connector.md") (qualified-name "Invocations::Thing")))))
    )
  )
)
~~~
