# META
~~~ini
description=KerML 8.3.4.7.4 checkFunctionResultBindingConnector over a function that owns a result expression: the implied binding connector between the function result and the result expression's result
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
    function Identity {
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
    (rule_id "kerml-1.0:8.3.4.7.4:checkFunctionResultBindingConnector")
    (outcome satisfied)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:e43be390e30585f2edb9f865784ceeaa4e35f14958319a66344c1126407e05e2"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (qualified-name "Functions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (qualified-name "Functions::Identity"))) (kind kerml-function) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "Identity")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind feature) (visibility default) (role result-expression)) (facts (expression-result (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "Identity")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default) (role result-expression)) (relationships (expressionOperand (reference "input")))))
    (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "Identity")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (qualified-name "Functions::Identity::input"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing") (direction in)))))
    (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (qualified-name "Functions::Identity::result"))) (kind parameter) (membership (kind feature) (visibility default) (role return-parameter)) (authored (membership (kind feature) (visibility default) (role return-parameter)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (qualified-name "Functions::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "Identity")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "input")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (qualified-name "Functions::Identity::input")))))
    (reference (id (source (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (qualified-name "Functions::Identity::input"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (qualified-name "Functions::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (qualified-name "Functions::Identity::result"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (qualified-name "Functions::Thing")))))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "Identity")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (qualified-name "Functions::Identity::input"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "Identity")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (qualified-name "Functions::Identity::input"))) (target (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (qualified-name "Functions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (qualified-name "Functions::Identity::input"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (qualified-name "Functions::Identity::result"))) (target (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (qualified-name "Functions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (qualified-name "Functions::Identity::result"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "Identity")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (qualified-name "Functions::Identity"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "Identity")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "Identity")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "Identity")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (qualified-name "Functions::Identity::input"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (qualified-name "Functions::Identity::input"))) (target (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (qualified-name "Functions::Identity"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (qualified-name "Functions::Identity::result"))) (target (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (qualified-name "Functions::Identity"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "Identity")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "Identity")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (qualified-name "Functions::Identity")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "Identity")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "Identity")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (qualified-name "Functions::Thing")) (source inherited) (from (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (qualified-name "Functions::Identity::input"))))
      (supertype (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (qualified-name "Functions::Identity::input")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (qualified-name "Functions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (qualified-name "Functions::Identity::input")))
      (featured-by (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (qualified-name "Functions::Identity")))
      (type (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (qualified-name "Functions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (qualified-name "Functions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (qualified-name "Functions::Thing")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "Identity")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (qualified-name "Functions::Identity::result")))
      (featured-by (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (qualified-name "Functions::Identity")))
      (type (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (qualified-name "Functions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (qualified-name "Functions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (qualified-name "Functions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (qualified-name "Functions::Thing")))
      (subtype (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (qualified-name "Functions::Identity::input")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (qualified-name "Functions::Identity::result")) (scopes any))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "Identity")) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (feature-reference "input" (target (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (qualified-name "Functions::Identity::input")))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (range (start 5 8) (end 5 13)) (probe (position 5 8))
    (reference (id (source (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "Identity")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "input")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (qualified-name "Functions::Identity::input")))))
    )
  )
  (query (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (range (start 3 27) (end 3 32)) (probe (position 3 27))
    (reference (id (source (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (qualified-name "Functions::Identity::input"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (qualified-name "Functions::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (range (start 4 32) (end 4 37)) (probe (position 4 32))
    (reference (id (source (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (qualified-name "Functions::Identity::result"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_function_result_binding_connector_result_expression.md") (qualified-name "Functions::Thing")))))
    )
  )
)
~~~
