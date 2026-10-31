# META
~~~ini
description=KerML 8.3.4.7.3 validateExpressionResultExpressionMembership allows an Expression at most one ResultExpressionMembership
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.7.3 validateExpressionResultExpressionMembership
source_expectation=accepted
rule_family=validate
expectation=by_construction
rule_id=kerml-1.0:8.3.4.7.3:validateExpressionResultExpressionMembership
blocked_by=abstract-syntax-nonrepresentable-second-result-expression
type=file
~~~
# SOURCE
~~~kerml
package Expressions {
    // Conforming: a single result expression. `ResultExpressionMember?` closes the body, so a
    // violating second one has no concrete-syntax spelling.
    expr One { 1 }
}
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
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:aa2d85bee5b7a1de95fb9f84012cb395e21e42f7c99a4ad1cbe65dc93a70cf98"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (qualified-name "Expressions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (qualified-name "Expressions::One"))) (kind kerml-expression) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-expression) (name "One")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind feature) (visibility default) (role result-expression)) (facts (expression-result (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-expression) (name "One")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-expression) (name "One")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-expression) (name "One")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (qualified-name "Expressions::One"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-expression) (name "One")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-expression) (name "One")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-expression) (name "One")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (state literal) (value (kind integer) (integer 1)))
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
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/kerml_expression_result_expression_membership.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-expression) (name "One")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (outcome resolved) (literal (value (kind integer) (integer 1))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
