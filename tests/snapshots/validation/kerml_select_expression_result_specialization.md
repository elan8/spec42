# META
~~~ini
description=KerML checkSelectExpressionResultSpecialization desired semantics
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.4.8.18:checkSelectExpressionResultSpecialization
~~~
# SOURCE
~~~kerml
package Model {
  feature values;
  feature selected = values.?values;
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics (specialization-check (rule_id "kerml-1.0:8.3.4.8.18:checkSelectExpressionResultSpecialization") (outcome satisfied)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_select_expression_result_specialization.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:2e9c8cf7d0e4a60865999902ad98442397bffc95a55163606e7ea774684b0963"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (qualified-name "Model::selected"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (operator-expression (kind select) (arguments (argument (ordinal 0) (expression (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (argument (ordinal 1) (expression (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 1))))) (result (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "values")) (expressionOperand (reference "values")))))
    (declaration (id (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 1))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (qualified-name "Model::values"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "values")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (qualified-name "Model::values")))))
    (reference (id (source (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 1))
      (authored-target "values")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (qualified-name "Model::values")))))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (qualified-name "Model::values"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (qualified-name "Model::values"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (qualified-name "Model::selected"))) (target (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 1))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (qualified-name "Model::selected")))
      (supertype (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
      (subtype (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (qualified-name "Model::selected")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 1)))))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (unsupported (feature-reference "values" (target (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (qualified-name "Model::values")))) (feature-reference "values" (target (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (qualified-name "Model::values"))))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_select_expression_result_specialization.md") (range (start 2 21) (end 2 27)) (probe (position 2 21))
    (reference (id (source (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "values")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (qualified-name "Model::values")))))
    )
  )
  (query (document "memory://snapshot/kerml_select_expression_result_specialization.md") (range (start 2 29) (end 2 35)) (probe (position 2 29))
    (reference (id (source (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 1) (authored-target "values")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (qualified-name "Model::values")))))
    )
  )
)
~~~
