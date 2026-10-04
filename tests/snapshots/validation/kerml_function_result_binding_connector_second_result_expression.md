# META
~~~ini
description=KerML 8.3.4.7.4 checkFunctionResultBindingConnector is violated by a function with two result expressions: the implied connector binds only the first (Pilot TypeAdapter.createResultConnector), so the second has none
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.4.7.4:checkFunctionResultBindingConnector
type=file
~~~
# SOURCE
~~~kerml
package Functions {
    classifier Thing;
    function Pick {
        return feature result : Thing;
        1 2
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (binding-connector-check
    (rule_id "kerml-1.0:8.3.4.7.4:checkFunctionResultBindingConnector")
    (outcome violated)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_function_result_binding_connector_second_result_expression.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "function_multiple_result_expressions")
        (source "semantic")
        (range (start 2 4) (end 5 5))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:28f6965919a8492099cb26943207dcc18e60c8923a8f627a4237075477768798"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector_second_result_expression.md") (qualified-name "Functions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector_second_result_expression.md") (qualified-name "Functions::Pick"))) (kind kerml-function) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector_second_result_expression.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "Pick")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind feature) (visibility default) (role result-expression)) (facts (expression-result (node (document "memory://snapshot/kerml_function_result_binding_connector_second_result_expression.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "Pick")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector_second_result_expression.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "Pick")) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (kind kerml-literal-integer) (membership (kind feature) (visibility default) (role result-expression)) (facts (expression-result (node (document "memory://snapshot/kerml_function_result_binding_connector_second_result_expression.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "Pick")) (anonymous (kind kerml-literal-integer) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector_second_result_expression.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "Pick")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector_second_result_expression.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "Pick")) (anonymous (kind kerml-literal-integer) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector_second_result_expression.md") (qualified-name "Functions::Pick::result"))) (kind parameter) (membership (kind feature) (visibility default) (role return-parameter)) (authored (membership (kind feature) (visibility default) (role return-parameter)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector_second_result_expression.md") (qualified-name "Functions::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_function_result_binding_connector_second_result_expression.md") (qualified-name "Functions::Pick::result"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_function_result_binding_connector_second_result_expression.md") (qualified-name "Functions::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_function_result_binding_connector_second_result_expression.md") (qualified-name "Functions::Pick::result"))) (target (node (document "memory://snapshot/kerml_function_result_binding_connector_second_result_expression.md") (qualified-name "Functions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_function_result_binding_connector_second_result_expression.md") (qualified-name "Functions::Pick::result"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_function_result_binding_connector_second_result_expression.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "Pick")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_function_result_binding_connector_second_result_expression.md") (qualified-name "Functions::Pick"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_function_result_binding_connector_second_result_expression.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "Pick")) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (target (node (document "memory://snapshot/kerml_function_result_binding_connector_second_result_expression.md") (qualified-name "Functions::Pick"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_function_result_binding_connector_second_result_expression.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "Pick")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_function_result_binding_connector_second_result_expression.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "Pick")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_function_result_binding_connector_second_result_expression.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "Pick")) (anonymous (kind kerml-literal-integer) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_function_result_binding_connector_second_result_expression.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "Pick")) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_function_result_binding_connector_second_result_expression.md") (qualified-name "Functions::Pick::result"))) (target (node (document "memory://snapshot/kerml_function_result_binding_connector_second_result_expression.md") (qualified-name "Functions::Pick"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_function_result_binding_connector_second_result_expression.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "Pick")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (state literal) (value (kind integer) (integer 1)))
    (evaluated (declaration (node (document "memory://snapshot/kerml_function_result_binding_connector_second_result_expression.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "Pick")) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (state literal) (value (kind integer) (integer 2)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector_second_result_expression.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "Pick")) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_function_result_binding_connector_second_result_expression.md") (qualified-name "Functions::Pick")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector_second_result_expression.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "Pick")) (anonymous (kind kerml-literal-integer) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/kerml_function_result_binding_connector_second_result_expression.md") (qualified-name "Functions::Pick")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector_second_result_expression.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "Pick")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_function_result_binding_connector_second_result_expression.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "Pick")) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector_second_result_expression.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "Pick")) (anonymous (kind kerml-literal-integer) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_function_result_binding_connector_second_result_expression.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "Pick")) (anonymous (kind kerml-literal-integer) (ordinal 1)))))
    )
    (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector_second_result_expression.md") (qualified-name "Functions::Pick::result")))
      (featured-by (node (document "memory://snapshot/kerml_function_result_binding_connector_second_result_expression.md") (qualified-name "Functions::Pick")))
      (type (node (document "memory://snapshot/kerml_function_result_binding_connector_second_result_expression.md") (qualified-name "Functions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_function_result_binding_connector_second_result_expression.md") (qualified-name "Functions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_function_result_binding_connector_second_result_expression.md") (qualified-name "Functions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector_second_result_expression.md") (qualified-name "Functions::Thing")))
      (subtype (node (document "memory://snapshot/kerml_function_result_binding_connector_second_result_expression.md") (qualified-name "Functions::Pick::result")) (scopes any))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector_second_result_expression.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "Pick")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (outcome resolved) (literal (value (kind integer) (integer 1))))
  (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector_second_result_expression.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "Pick")) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (outcome resolved) (literal (value (kind integer) (integer 2))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_function_result_binding_connector_second_result_expression.md") (range (start 3 32) (end 3 37)) (probe (position 3 32))
    (reference (id (source (node (document "memory://snapshot/kerml_function_result_binding_connector_second_result_expression.md") (qualified-name "Functions::Pick::result"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_function_result_binding_connector_second_result_expression.md") (qualified-name "Functions::Thing")))))
    )
  )
)
~~~
