# META
~~~ini
description=KerML 8.3.4.7.4 validateFunctionResultExpressionMembership allows a Function at most one ResultExpressionMembership
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.7.4 validateFunctionResultExpressionMembership
source_expectation=accepted
rule_family=validate
expectation=by_construction
rule_id=kerml-1.0:8.3.4.7.4:validateFunctionResultExpressionMembership
blocked_by=abstract-syntax-nonrepresentable-second-result-expression
type=file
~~~
# SOURCE
~~~kerml
package Functions {
    // Conforming: a single result expression. `ResultExpressionMember?` closes the body, so a
    // violating second one has no concrete-syntax spelling.
    function One { 1 }
}
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
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:26a902afbb78b246b50dfd6d2b443e5ff1a99e61e8cd8652cc8e23766fb86de9"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_function_result_expression_membership.md") (qualified-name "Functions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_function_result_expression_membership.md") (qualified-name "Functions::One"))) (kind kerml-function) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_function_result_expression_membership.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "One")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind feature) (visibility default) (role result-expression)) (facts (expression-result (node (document "memory://snapshot/kerml_function_result_expression_membership.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "One")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_function_result_expression_membership.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "One")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_function_result_expression_membership.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "One")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_function_result_expression_membership.md") (qualified-name "Functions::One"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_function_result_expression_membership.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "One")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_function_result_expression_membership.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "One")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_function_result_expression_membership.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "One")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (state literal) (value (kind integer) (integer 1)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_function_result_expression_membership.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "One")) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_function_result_expression_membership.md") (qualified-name "Functions::One")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_function_result_expression_membership.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "One")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_function_result_expression_membership.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "One")) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/kerml_function_result_expression_membership.md") (path (named (kind package) (name "Functions")) (named (kind kerml-function) (name "One")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (outcome resolved) (literal (value (kind integer) (integer 1))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
