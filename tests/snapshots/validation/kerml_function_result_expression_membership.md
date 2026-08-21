# META
~~~ini
description=KerML 8.3.4.7.4 validateFunctionResultExpressionMembership allows a Function at most one ResultExpressionMembership
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.7.4 validateFunctionResultExpressionMembership
type=file
skip_validation=the publication does not record a ResultExpressionMembership -- a function or expression body result expression produces no member in SMG -- so the rule has nothing to count
~~~
# SOURCE
~~~kerml
package Functions {
    // Conforming: a single result expression.
    function One { 1 }

    // Invalid: two result expressions in one function body.
    function Two { 1 2 }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_function_result_expression_membership.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "function_multiple_result_expressions")
        (source "semantic")
        (range (start 5 4) (end 5 24))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_function_result_expression_membership.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:48f9e939bc713a262773c62e816a287baf8f836debe59d4626ece2695b30955e") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_function_result_expression_membership.md") (qualified-name "Functions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_function_result_expression_membership.md") (qualified-name "Functions::One"))) (kind kerml-function) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_function_result_expression_membership.md") (qualified-name "Functions::Two"))) (kind kerml-function) (membership (kind owning) (visibility default)))
  )
  (references
  )
  (relationships
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_function_result_expression_membership.md") (qualified-name "Functions::One"))) (state literal) (value (kind integer) (integer 1)))
    (evaluated (declaration (node (document "memory://snapshot/kerml_function_result_expression_membership.md") (qualified-name "Functions::Two"))) (state literal) (value (kind integer) (integer 2)))
    (evaluated (declaration (node (document "memory://snapshot/kerml_function_result_expression_membership.md") (qualified-name "Functions::Two"))) (state literal) (value (kind integer) (integer 2)))
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
