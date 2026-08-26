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
blocked_by=lowering-result-expression-memberships
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
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:4d2af3b82a80f888d16af0ca2a51f4414bfd61043a38e256dab95d875cea6d14") (contract-version "feature-chain-expression-result-v10"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_result_expression_membership_owning_type.md") (qualified-name "Results"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_result_expression_membership_owning_type.md") (qualified-name "Results::Computing"))) (kind kerml-function) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_result_expression_membership_owning_type.md") (qualified-name "Results::Object"))) (kind kerml-structure) (membership (kind owning) (visibility default)))
  )
  (references
  )
  (relationships
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_result_expression_membership_owning_type.md") (qualified-name "Results::Computing"))) (state literal) (value (kind integer) (integer 1)))
    (evaluated (declaration (node (document "memory://snapshot/kerml_result_expression_membership_owning_type.md") (qualified-name "Results::Object"))) (state literal) (value (kind integer) (integer 1)))
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
