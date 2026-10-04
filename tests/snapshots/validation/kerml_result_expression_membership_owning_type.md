# META
~~~ini
description=KerML 8.3.4.7.7 validateResultExpressionMembershipOwningType requires the owningType of a ResultExpressionMembership to be a Function or an Expression
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.7.7 validateResultExpressionMembershipOwningType
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.4.7.7:validateResultExpressionMembershipOwningType
type=file
~~~
# SOURCE
~~~kerml
package Results {
    // Conforming: the result expression is owned by a function.
    function Computing { 1 }

    // Invalid: a structure is neither a Function nor an Expression.
    struct Object { 1 }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_result_expression_membership_owning_type.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "result_expression_membership_invalid_owner")
        (source "semantic")
        (range (start 5 4) (end 5 23))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_result_expression_membership_owning_type.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "result_expression_membership_invalid_owner")
        (source "semantic")
        (range (start 5 4) (end 5 23))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:0d5b3772f3e437ebce5d72ac99736542d53133d2e9df4632fdae8ee763408a43"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_result_expression_membership_owning_type.md") (qualified-name "Results"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_result_expression_membership_owning_type.md") (qualified-name "Results::Computing"))) (kind kerml-function) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_result_expression_membership_owning_type.md") (path (named (kind package) (name "Results")) (named (kind kerml-function) (name "Computing")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind feature) (visibility default) (role result-expression)) (facts (expression-result (node (document "memory://snapshot/kerml_result_expression_membership_owning_type.md") (path (named (kind package) (name "Results")) (named (kind kerml-function) (name "Computing")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_result_expression_membership_owning_type.md") (path (named (kind package) (name "Results")) (named (kind kerml-function) (name "Computing")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_result_expression_membership_owning_type.md") (qualified-name "Results::Object"))) (kind kerml-structure) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_result_expression_membership_owning_type.md") (path (named (kind package) (name "Results")) (named (kind kerml-structure) (name "Object")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind feature) (visibility default) (role result-expression)) (facts (expression-result (node (document "memory://snapshot/kerml_result_expression_membership_owning_type.md") (path (named (kind package) (name "Results")) (named (kind kerml-structure) (name "Object")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_result_expression_membership_owning_type.md") (path (named (kind package) (name "Results")) (named (kind kerml-structure) (name "Object")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_result_expression_membership_owning_type.md") (path (named (kind package) (name "Results")) (named (kind kerml-function) (name "Computing")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_result_expression_membership_owning_type.md") (qualified-name "Results::Computing"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_result_expression_membership_owning_type.md") (path (named (kind package) (name "Results")) (named (kind kerml-function) (name "Computing")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_result_expression_membership_owning_type.md") (path (named (kind package) (name "Results")) (named (kind kerml-function) (name "Computing")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_result_expression_membership_owning_type.md") (path (named (kind package) (name "Results")) (named (kind kerml-structure) (name "Object")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_result_expression_membership_owning_type.md") (qualified-name "Results::Object"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_result_expression_membership_owning_type.md") (path (named (kind package) (name "Results")) (named (kind kerml-structure) (name "Object")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_result_expression_membership_owning_type.md") (path (named (kind package) (name "Results")) (named (kind kerml-structure) (name "Object")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_result_expression_membership_owning_type.md") (path (named (kind package) (name "Results")) (named (kind kerml-function) (name "Computing")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (state literal) (value (kind integer) (integer 1)))
    (evaluated (declaration (node (document "memory://snapshot/kerml_result_expression_membership_owning_type.md") (path (named (kind package) (name "Results")) (named (kind kerml-structure) (name "Object")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (state literal) (value (kind integer) (integer 1)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_result_expression_membership_owning_type.md") (path (named (kind package) (name "Results")) (named (kind kerml-function) (name "Computing")) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_result_expression_membership_owning_type.md") (qualified-name "Results::Computing")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_result_expression_membership_owning_type.md") (path (named (kind package) (name "Results")) (named (kind kerml-function) (name "Computing")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_result_expression_membership_owning_type.md") (path (named (kind package) (name "Results")) (named (kind kerml-function) (name "Computing")) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/kerml_result_expression_membership_owning_type.md") (path (named (kind package) (name "Results")) (named (kind kerml-structure) (name "Object")) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_result_expression_membership_owning_type.md") (qualified-name "Results::Object")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_result_expression_membership_owning_type.md") (path (named (kind package) (name "Results")) (named (kind kerml-structure) (name "Object")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_result_expression_membership_owning_type.md") (path (named (kind package) (name "Results")) (named (kind kerml-structure) (name "Object")) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/kerml_result_expression_membership_owning_type.md") (path (named (kind package) (name "Results")) (named (kind kerml-function) (name "Computing")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (outcome resolved) (literal (value (kind integer) (integer 1))))
  (declaration (id (node (document "memory://snapshot/kerml_result_expression_membership_owning_type.md") (path (named (kind package) (name "Results")) (named (kind kerml-structure) (name "Object")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (outcome resolved) (literal (value (kind integer) (integer 1))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
