# META
~~~ini
description=KerML 8.3.4.8.3 checkConstructorExpressionResultDefaultValueBindingConnector has an exact TBD body in the pinned normative XMI
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.4.8.3:checkConstructorExpressionResultDefaultValueBindingConnector
blocked_by=normative-specification-gap-constructor-expression-result-default-value-binding-connector
type=file
~~~
# SOURCE
~~~kerml
package Constructors {
    classifier Thing;
    classifier Holder {
        feature item : Thing;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (binding-connector-check
    (rule_id "kerml-1.0:8.3.4.8.3:checkConstructorExpressionResultDefaultValueBindingConnector")
    (outcome satisfied)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_constructor_expression_result_default_value_binding_connector.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:18ee2024aaecac2db793477d3ac2b293c122adfd91b6de1f49bcd5d63ccfcd16") (contract-version "semantic-metadata-projection-v6"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_default_value_binding_connector.md") (qualified-name "Constructors"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_default_value_binding_connector.md") (qualified-name "Constructors::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_default_value_binding_connector.md") (qualified-name "Constructors::Holder::item"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_default_value_binding_connector.md") (qualified-name "Constructors::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_result_default_value_binding_connector.md") (qualified-name "Constructors::Holder::item"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_constructor_expression_result_default_value_binding_connector.md") (qualified-name "Constructors::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_constructor_expression_result_default_value_binding_connector.md") (qualified-name "Constructors::Holder::item"))) (target (node (document "memory://snapshot/kerml_constructor_expression_result_default_value_binding_connector.md") (qualified-name "Constructors::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_constructor_expression_result_default_value_binding_connector.md") (qualified-name "Constructors::Holder::item"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_constructor_expression_result_default_value_binding_connector.md") (qualified-name "Constructors::Holder::item"))) (target (node (document "memory://snapshot/kerml_constructor_expression_result_default_value_binding_connector.md") (qualified-name "Constructors::Holder"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_default_value_binding_connector.md") (qualified-name "Constructors::Holder::item")))
      (featured-by (node (document "memory://snapshot/kerml_constructor_expression_result_default_value_binding_connector.md") (qualified-name "Constructors::Holder")))
      (type (node (document "memory://snapshot/kerml_constructor_expression_result_default_value_binding_connector.md") (qualified-name "Constructors::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_constructor_expression_result_default_value_binding_connector.md") (qualified-name "Constructors::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_constructor_expression_result_default_value_binding_connector.md") (qualified-name "Constructors::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_default_value_binding_connector.md") (qualified-name "Constructors::Thing")))
      (subtype (node (document "memory://snapshot/kerml_constructor_expression_result_default_value_binding_connector.md") (qualified-name "Constructors::Holder::item")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_constructor_expression_result_default_value_binding_connector.md") (range (start 3 23) (end 3 28)) (probe (position 3 23))
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_result_default_value_binding_connector.md") (qualified-name "Constructors::Holder::item"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_constructor_expression_result_default_value_binding_connector.md") (qualified-name "Constructors::Thing")))))
    )
  )
)
~~~
