# META
~~~ini
description=KerML 8.3.4.7.3 validateExpressionResultExpressionMembership allows an Expression at most one ResultExpressionMembership
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.7.3 validateExpressionResultExpressionMembership
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.4.7.3:validateExpressionResultExpressionMembership
type=file
~~~
# SOURCE
~~~kerml
package Expressions {
    // Conforming: a single result expression.
    expr One { 1 }

    // Invalid: two result expressions in one expression body.
    expr Two { 1 2 }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_expression_result_expression_membership.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "expression_multiple_result_expressions")
        (source "semantic")
        (range (start 5 4) (end 5 20))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_expression_result_expression_membership.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "expression_multiple_result_expressions")
        (source "semantic")
        (range (start 5 4) (end 5 20))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:94a83cecd644e197f57f2740c8c142ebee6217d87435c190f06cc096bbc1c85d"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (qualified-name "Expressions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (qualified-name "Expressions::One"))) (kind kerml-expression) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-expression) (name "One")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind feature) (visibility default) (role result-expression)) (facts (expression-result (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-expression) (name "One")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-expression) (name "One")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (qualified-name "Expressions::Two"))) (kind kerml-expression) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-expression) (name "Two")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind feature) (visibility default) (role result-expression)) (facts (expression-result (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-expression) (name "Two")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-expression) (name "Two")) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (kind kerml-literal-integer) (membership (kind feature) (visibility default) (role result-expression)) (facts (expression-result (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-expression) (name "Two")) (anonymous (kind kerml-literal-integer) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-expression) (name "Two")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-expression) (name "Two")) (anonymous (kind kerml-literal-integer) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-expression) (name "One")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (qualified-name "Expressions::One"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-expression) (name "One")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-expression) (name "One")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-expression) (name "Two")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (qualified-name "Expressions::Two"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-expression) (name "Two")) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (target (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (qualified-name "Expressions::Two"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-expression) (name "Two")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-expression) (name "Two")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-expression) (name "Two")) (anonymous (kind kerml-literal-integer) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-expression) (name "Two")) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-expression) (name "One")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (state literal) (value (kind integer) (integer 1)))
    (evaluated (declaration (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-expression) (name "Two")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (state literal) (value (kind integer) (integer 1)))
    (evaluated (declaration (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-expression) (name "Two")) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (state literal) (value (kind integer) (integer 2)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-expression) (name "One")) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (qualified-name "Expressions::One")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-expression) (name "One")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-expression) (name "One")) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-expression) (name "Two")) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (qualified-name "Expressions::Two")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-expression) (name "Two")) (anonymous (kind kerml-literal-integer) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (qualified-name "Expressions::Two")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-expression) (name "Two")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-expression) (name "Two")) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-expression) (name "Two")) (anonymous (kind kerml-literal-integer) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-expression) (name "Two")) (anonymous (kind kerml-literal-integer) (ordinal 1)))))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-expression) (name "One")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (outcome resolved) (literal (value (kind integer) (integer 1))))
  (declaration (id (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-expression) (name "Two")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (outcome resolved) (literal (value (kind integer) (integer 1))))
  (declaration (id (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-expression) (name "Two")) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (outcome resolved) (literal (value (kind integer) (integer 2))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
