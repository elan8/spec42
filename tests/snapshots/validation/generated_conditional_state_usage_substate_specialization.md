# META
~~~ini
description=Generated substate specialization preserves the exact parallel StateUsage branch from SysML 8.3.18.6
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.18.6:checkStateUsageSubstateSpecialization
blocked_by=lowering-gap-state-usage-specialization-facts
type=file
libraries=standard
~~~
# SOURCE
~~~sysml
package StateUsageSubstateSpecialization {
    state machine parallel {
        state nested;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind specialization) (source "StateUsageSubstateSpecialization::machine::nested") (target "States::StateAction::substates") (provenance implied) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/generated_conditional_state_usage_substate_specialization.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:1ee63394f9516d5df262cd2fce70be0dfb6a60093bd13119342281ea85777d33") (contract-version "feature-chain-expression-result-v10") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/generated_conditional_state_usage_substate_specialization.md") (qualified-name "StateUsageSubstateSpecialization"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_state_usage_substate_specialization.md") (qualified-name "StateUsageSubstateSpecialization::machine"))) (kind state) (membership (kind feature) (visibility default)) (facts (modifiers parallel)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_state_usage_substate_specialization.md") (qualified-name "StateUsageSubstateSpecialization::machine::nested"))) (kind state) (membership (kind feature) (visibility default)))
  )
  (references
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_state_usage_substate_specialization.md") (qualified-name "StateUsageSubstateSpecialization::machine"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_state_usage_substate_specialization.md") (qualified-name "StateUsageSubstateSpecialization::machine::nested"))) (target (node (document "memory://snapshot/generated_conditional_state_usage_substate_specialization.md") (qualified-name "StateUsageSubstateSpecialization::machine"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_state_usage_substate_specialization.md") (qualified-name "StateUsageSubstateSpecialization::machine::nested"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/generated_conditional_state_usage_substate_specialization.md") (qualified-name "StateUsageSubstateSpecialization::machine")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/control_performances.md") (qualified-name "ControlPerformances::DecisionPerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_state_usage_substate_specialization.md") (qualified-name "StateUsageSubstateSpecialization::machine::nested")))
      (featured-by (node (document "memory://snapshot/generated_conditional_state_usage_substate_specialization.md") (qualified-name "StateUsageSubstateSpecialization::machine")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/control_performances.md") (qualified-name "ControlPerformances::DecisionPerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions")) (scopes any subclassification))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
