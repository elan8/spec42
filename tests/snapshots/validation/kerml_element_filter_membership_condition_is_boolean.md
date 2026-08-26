# META
~~~ini
description=KerML 8.3.4.13.2 validateElementFilterMembershipConditionIsBoolean requires the result parameter of the condition Expression to specialize ScalarValues::Boolean
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.13.2 validateElementFilterMembershipConditionIsBoolean
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.4.13.2:validateElementFilterMembershipConditionIsBoolean
type=file
~~~
# SOURCE
~~~kerml
package Filters {
    // Conforming: a Boolean filter condition.
    package Accepted {
        filter true;
    }

    // Invalid: a filter condition that settles to a non-Boolean constant.
    package Rejected {
        filter 1;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_element_filter_membership_condition_is_boolean.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "invalid_import_filter")
        (source "semantic")
        (range (start 8 15) (end 8 16))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_element_filter_membership_condition_is_boolean.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "invalid_import_filter")
        (source "semantic")
        (range (start 8 15) (end 8 16))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:cfd715b25d46874af008a4c50e1868e76a8a315e856b23afc5c8e243db80ac8c") (contract-version "constructor-expression-specialization-v9"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_boolean.md") (qualified-name "Filters"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_boolean.md") (qualified-name "Filters::Accepted"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_boolean.md") (qualified-name "Filters::Rejected"))) (kind package) (membership (kind owning) (visibility default)))
  )
  (references
  )
  (relationships
  )
  (evaluation
    (filter (owner (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_boolean.md") (qualified-name "Filters::Accepted"))) (form package-import) (state literal) (start 3 15) (end 3 19) (value (kind boolean) (boolean true)))
    (filter (owner (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_boolean.md") (qualified-name "Filters::Rejected"))) (form package-import) (state literal) (start 8 15) (end 8 16) (value (kind integer) (integer 1)))
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
