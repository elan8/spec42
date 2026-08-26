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
blocked_by=lowering-result-expression-memberships
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
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:87a3462b64bdc22533fe53b1f589f131fcb48cded285ee85d86a5f0ed6621712") (contract-version "constructor-expression-specialization-v9"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (qualified-name "Expressions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (qualified-name "Expressions::One"))) (kind kerml-expression) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (qualified-name "Expressions::Two"))) (kind kerml-expression) (membership (kind feature) (visibility default)))
  )
  (references
  )
  (relationships
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (qualified-name "Expressions::One"))) (state literal) (value (kind integer) (integer 1)))
    (evaluated (declaration (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (qualified-name "Expressions::Two"))) (state literal) (value (kind integer) (integer 2)))
    (evaluated (declaration (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (qualified-name "Expressions::Two"))) (state literal) (value (kind integer) (integer 2)))
  )
)
~~~
# TYPES
~~~sexpr
(types
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
