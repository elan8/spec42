# META
~~~ini
description=KerML 8.3.4.7.4 checkFunctionResultBindingConnector stays unresolved when body recovery may have hidden a result expression of the function
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=malformed
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.4.7.4:checkFunctionResultBindingConnector
type=file
~~~
# SOURCE
~~~kerml
package Functions {
    classifier Thing;
    function Identity {
        return feature result : Thing;
        input +;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (binding-connector-check
    (rule_id "kerml-1.0:8.3.4.7.4:checkFunctionResultBindingConnector")
    (outcome unresolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_function_result_binding_connector_recovered_body.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 4 8) (end 5 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:24b54d33e224c602c78d042d4a7562741f2603ef4cb5d7c0ac75ff894d6513f5"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector_recovered_body.md") (qualified-name "Functions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector_recovered_body.md") (qualified-name "Functions::Identity"))) (kind kerml-function) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector_recovered_body.md") (qualified-name "Functions::Identity::result"))) (kind parameter) (membership (kind feature) (visibility default) (role return-parameter)) (authored (membership (kind feature) (visibility default) (role return-parameter)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector_recovered_body.md") (qualified-name "Functions::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_function_result_binding_connector_recovered_body.md") (qualified-name "Functions::Identity::result"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_function_result_binding_connector_recovered_body.md") (qualified-name "Functions::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_function_result_binding_connector_recovered_body.md") (qualified-name "Functions::Identity::result"))) (target (node (document "memory://snapshot/kerml_function_result_binding_connector_recovered_body.md") (qualified-name "Functions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_function_result_binding_connector_recovered_body.md") (qualified-name "Functions::Identity::result"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_function_result_binding_connector_recovered_body.md") (qualified-name "Functions::Identity::result"))) (target (node (document "memory://snapshot/kerml_function_result_binding_connector_recovered_body.md") (qualified-name "Functions::Identity"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector_recovered_body.md") (qualified-name "Functions::Identity::result")))
      (featured-by (node (document "memory://snapshot/kerml_function_result_binding_connector_recovered_body.md") (qualified-name "Functions::Identity")))
      (type (node (document "memory://snapshot/kerml_function_result_binding_connector_recovered_body.md") (qualified-name "Functions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_function_result_binding_connector_recovered_body.md") (qualified-name "Functions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_function_result_binding_connector_recovered_body.md") (qualified-name "Functions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector_recovered_body.md") (qualified-name "Functions::Thing")))
      (subtype (node (document "memory://snapshot/kerml_function_result_binding_connector_recovered_body.md") (qualified-name "Functions::Identity::result")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_function_result_binding_connector_recovered_body.md") (range (start 3 32) (end 3 37)) (probe (position 3 32))
    (reference (id (source (node (document "memory://snapshot/kerml_function_result_binding_connector_recovered_body.md") (qualified-name "Functions::Identity::result"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_function_result_binding_connector_recovered_body.md") (qualified-name "Functions::Thing")))))
    )
  )
)
~~~
