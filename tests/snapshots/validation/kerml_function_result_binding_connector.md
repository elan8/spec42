# META
~~~ini
description=KerML 8.3.4.7.4 checkFunctionResultBindingConnector requires each function result expression membership to have its canonical binding connector
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.4.7.4:checkFunctionResultBindingConnector
blocked_by=lowering-result-expression-memberships
type=file
~~~
# SOURCE
~~~kerml
package Functions {
    classifier Thing;
    function Identity {
        in feature input : Thing;
        return feature result : Thing;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (binding-connector-check
    (rule_id "kerml-1.0:8.3.4.7.4:checkFunctionResultBindingConnector")
    (outcome satisfied)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_function_result_binding_connector.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:c35bac7903c6d21f62c184e0b7708a140378a9a8d4fb5d70dce7cabad9b6bc53"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector.md") (qualified-name "Functions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector.md") (qualified-name "Functions::Identity"))) (kind kerml-function) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector.md") (qualified-name "Functions::Identity::input"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing") (direction in)))))
    (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector.md") (qualified-name "Functions::Identity::result"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector.md") (qualified-name "Functions::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_function_result_binding_connector.md") (qualified-name "Functions::Identity::input"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_function_result_binding_connector.md") (qualified-name "Functions::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_function_result_binding_connector.md") (qualified-name "Functions::Identity::result"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_function_result_binding_connector.md") (qualified-name "Functions::Thing")))))
  )
  (relationships
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/kerml_function_result_binding_connector.md") (qualified-name "Functions::Identity::input"))) (target (node (document "memory://snapshot/kerml_function_result_binding_connector.md") (qualified-name "Functions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_function_result_binding_connector.md") (qualified-name "Functions::Identity::input"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_function_result_binding_connector.md") (qualified-name "Functions::Identity::result"))) (target (node (document "memory://snapshot/kerml_function_result_binding_connector.md") (qualified-name "Functions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_function_result_binding_connector.md") (qualified-name "Functions::Identity::result"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_function_result_binding_connector.md") (qualified-name "Functions::Identity::input"))) (target (node (document "memory://snapshot/kerml_function_result_binding_connector.md") (qualified-name "Functions::Identity"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_function_result_binding_connector.md") (qualified-name "Functions::Identity::result"))) (target (node (document "memory://snapshot/kerml_function_result_binding_connector.md") (qualified-name "Functions::Identity"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector.md") (qualified-name "Functions::Identity::input")))
      (featured-by (node (document "memory://snapshot/kerml_function_result_binding_connector.md") (qualified-name "Functions::Identity")))
      (type (node (document "memory://snapshot/kerml_function_result_binding_connector.md") (qualified-name "Functions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_function_result_binding_connector.md") (qualified-name "Functions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_function_result_binding_connector.md") (qualified-name "Functions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector.md") (qualified-name "Functions::Identity::result")))
      (featured-by (node (document "memory://snapshot/kerml_function_result_binding_connector.md") (qualified-name "Functions::Identity")))
      (type (node (document "memory://snapshot/kerml_function_result_binding_connector.md") (qualified-name "Functions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_function_result_binding_connector.md") (qualified-name "Functions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_function_result_binding_connector.md") (qualified-name "Functions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector.md") (qualified-name "Functions::Thing")))
      (subtype (node (document "memory://snapshot/kerml_function_result_binding_connector.md") (qualified-name "Functions::Identity::input")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_function_result_binding_connector.md") (qualified-name "Functions::Identity::result")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_function_result_binding_connector.md") (range (start 3 27) (end 3 32)) (probe (position 3 27))
    (reference (id (source (node (document "memory://snapshot/kerml_function_result_binding_connector.md") (qualified-name "Functions::Identity::input"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_function_result_binding_connector.md") (qualified-name "Functions::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_function_result_binding_connector.md") (range (start 4 32) (end 4 37)) (probe (position 4 32))
    (reference (id (source (node (document "memory://snapshot/kerml_function_result_binding_connector.md") (qualified-name "Functions::Identity::result"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_function_result_binding_connector.md") (qualified-name "Functions::Thing")))))
    )
  )
)
~~~
