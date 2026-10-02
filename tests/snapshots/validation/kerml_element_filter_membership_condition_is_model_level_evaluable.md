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

    classifier Holder {
        feature flag : Thing;
    }

    // Invalid: the condition references a feature featured by a classifier that is not a
    // metaclass, so its value depends on an instance and is not model-level evaluable. (A
    // package-level feature has no featuring type and would be evaluable.)
    package Rejected {
        filter Holder::flag;
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
        (range (start 16 15) (end 16 27))
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
      (diagnostic
        (severity warning)
        (code "filter_condition_not_model_level_evaluable")
        (source "semantic")
        (range (start 16 15) (end 16 27))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:4a43d1fea2b87299b7f674d57d07ff1b4906c96e2be02098f6d3f8187e3385e9"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Accepted"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (path (named (kind package) (name "Filters")) (named (kind package) (name "Accepted")) (anonymous (kind kerml-literal-boolean) (ordinal 0))))) (kind kerml-literal-boolean) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Holder::flag"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Rejected"))) (kind package) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "Holder::flag")))))
    (declaration (id (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Holder::flag"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Rejected"))) (kind expressionOperand) (ordinal 0))
      (authored-target "Holder::flag")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Holder::flag")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Holder::flag"))) (target (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Holder::flag"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Rejected"))) (target (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Holder::flag"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Rejected"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Holder::flag"))) (target (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Holder"))) (provenance implied))
  )
  (evaluation
    (filter (owner (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Accepted"))) (form package-import) (state literal) (start 5 15) (end 5 19) (value (kind boolean) (boolean true)))
    (filter (owner (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Rejected"))) (form package-import) (state non-constant) (start 16 15) (end 16 27))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Holder::flag")))
      (featured-by (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Holder")))
      (type (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Thing")))
      (subtype (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Holder::flag")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (range (start 9 23) (end 9 28)) (probe (position 9 23))
    (reference (id (source (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Holder::flag"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (range (start 16 15) (end 16 27)) (probe (position 16 15))
    (reference (id (source (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Rejected"))) (kind expressionOperand) (ordinal 0) (authored-target "Holder::flag")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_element_filter_membership_condition_is_model_level_evaluable.md") (qualified-name "Filters::Holder::flag")))))
    )
  )
)
~~~
