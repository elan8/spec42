# META
~~~ini
description=The parser does not lower an Action-owned transition as a queryable SysML TransitionUsage
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.18.9:checkTransitionUsageActionSpecialization
blocked_by=lowering-gap-transition-usage-specialization-facts
type=file
libraries=standard
~~~
# SOURCE
~~~sysml
package TransitionUsageActionSpecialization {
    action def Decision;
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind specialization) (source "TransitionUsageActionSpecialization::Decision::<anonymous>") (target "Actions::Action::decisionTransitions") (provenance implied) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/generated_conditional_transition_usage_action_specialization_lowering_gap.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:6ea0ffb352156fc029805613b2b12248fc99459ba6b055b2d61388a2d2b6680d") (contract-version "lossless-publication-completeness-v3") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/generated_conditional_transition_usage_action_specialization_lowering_gap.md") (qualified-name "TransitionUsageActionSpecialization"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_transition_usage_action_specialization_lowering_gap.md") (qualified-name "TransitionUsageActionSpecialization::Decision"))) (kind action-def) (membership (kind owning) (visibility default)))
  )
  (references
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_transition_usage_action_specialization_lowering_gap.md") (qualified-name "TransitionUsageActionSpecialization::Decision"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/generated_conditional_transition_usage_action_specialization_lowering_gap.md") (qualified-name "TransitionUsageActionSpecialization::Decision")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
