# META
~~~ini
description=KerML 8.3.4.7.4 checkFunctionResultBindingConnector stays unresolved for a function with a result expression but no settled result: it declares none and, without the standard library, inherits none
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
    function One { 1 }
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
  (document "memory://snapshot/kerml_function_result_binding_connector_absent_result.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:8aad422e0007f26af2449e06754418db66495f45e7a59398e603de026eb0309a"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector_absent_result.md") (qualified-name "Functions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector_absent_result.md") (qualified-name "Functions::One"))) (kind kerml-function) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector_absent_result.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "One")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind feature) (visibility default) (role result-expression)) (facts (expression-result (node (document "memory://snapshot/kerml_function_result_binding_connector_absent_result.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "One")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector_absent_result.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "One")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_function_result_binding_connector_absent_result.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "One")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_function_result_binding_connector_absent_result.md") (qualified-name "Functions::One"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_function_result_binding_connector_absent_result.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "One")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_function_result_binding_connector_absent_result.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "One")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_function_result_binding_connector_absent_result.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "One")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (state literal) (value (kind integer) (integer 1)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector_absent_result.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "One")) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_function_result_binding_connector_absent_result.md") (qualified-name "Functions::One")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector_absent_result.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "One")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_function_result_binding_connector_absent_result.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "One")) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector_absent_result.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "One")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (outcome resolved) (literal (value (kind integer) (integer 1))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
