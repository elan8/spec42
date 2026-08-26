# META
~~~ini
description=KerML 8.3.4.13.2 validateElementFilterMembershipConditionIsModelLevelEvaluable requires the condition Expression of an ElementFilterMembership to be model-level evaluable
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.13.2 validateElementFilterMembershipConditionIsModelLevelEvaluable
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.4.13.2:validateElementFilterMembershipConditionIsModelLevelEvaluable
blocked_by=semantic-filter-condition-not-model-level-evaluable
type=file
~~~
# SOURCE
~~~kerml
package Filters {
    classifier Thing;

    // Conforming: a model-level evaluable filter condition.
    package Accepted {
        filter true;
    }

    // Invalid: the condition depends on a feature value and is not model-level evaluable.
    package Rejected {
        feature flag : Thing;
        filter flag;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "filter_condition_not_model_level_evaluable")
        (source "semantic")
        (range (start 11 8) (end 11 20))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:9dd6c8be56f2f31bf9d350df71652faa920cca447088740f08502e00f346868a") (contract-version "feature-chain-expression-result-v10"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Accepted"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Rejected"))) (kind package) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "flag")))))
    (declaration (id (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Rejected::flag"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Rejected"))) (kind expressionOperand) (ordinal 0))
      (authored-target "flag")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Rejected::flag")))))
    (reference (id (source (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Rejected::flag"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Thing")))))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Rejected"))) (target (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Rejected::flag"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Rejected"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Rejected::flag"))) (target (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Rejected::flag"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (filter (owner (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Accepted"))) (form package-import) (state literal) (start 5 15) (end 5 19) (value (kind boolean) (boolean true)))
    (filter (owner (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Rejected"))) (form package-import) (state non-constant) (start 11 15) (end 11 19))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Rejected::flag")))
      (type (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Thing")))
      (subtype (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Rejected::flag")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (range (start 11 15) (end 11 19)) (probe (position 11 15))
    (reference (id (source (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Rejected"))) (kind expressionOperand) (ordinal 0) (authored-target "flag")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Rejected::flag")))))
    )
  )
  (query (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (range (start 10 23) (end 10 28)) (probe (position 10 23))
    (reference (id (source (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Rejected::flag"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Thing")))))
    )
  )
)
~~~
