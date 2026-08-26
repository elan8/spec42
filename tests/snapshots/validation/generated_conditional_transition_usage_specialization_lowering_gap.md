# META
~~~ini
description=TransitionUsage Action and State specializations remain explicit until lowering publishes the SysML TransitionUsage fact family
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.18.9:checkTransitionUsageStateSpecialization
blocked_by=lowering-gap-transition-usage-specialization-facts
type=file
libraries=standard
~~~
# SOURCE
~~~sysml
package TransitionUsageStateSpecialization {
    state def Machine {
        state idle;
        state running;
        transition Route first idle then running;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind specialization) (source "TransitionUsageStateSpecialization::Machine::Route") (target "States::StateAction::stateTransitions") (provenance implied) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/generated_conditional_transition_usage_specialization_lowering_gap.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_final_state")
        (source "semantic")
        (range (start 1 4) (end 5 5))
      )
      (diagnostic
        (severity information)
        (code "missing_initial_state")
        (source "semantic")
        (range (start 1 4) (end 5 5))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:070cda20f389cbc36fe5cf886577d4b7513f80f195570f33e1e5d25287c939f4") (contract-version "feature-chain-expression-result-v10") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/generated_conditional_transition_usage_specialization_lowering_gap.md") (qualified-name "TransitionUsageStateSpecialization"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_transition_usage_specialization_lowering_gap.md") (qualified-name "TransitionUsageStateSpecialization::Machine"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_transition_usage_specialization_lowering_gap.md") (qualified-name "TransitionUsageStateSpecialization::Machine::Route"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "idle")) (transitionTarget (reference "running")))))
    (declaration (id (node (document "memory://snapshot/generated_conditional_transition_usage_specialization_lowering_gap.md") (qualified-name "TransitionUsageStateSpecialization::Machine::idle"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_transition_usage_specialization_lowering_gap.md") (qualified-name "TransitionUsageStateSpecialization::Machine::running"))) (kind state) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/generated_conditional_transition_usage_specialization_lowering_gap.md") (qualified-name "TransitionUsageStateSpecialization::Machine::Route"))) (kind transitionSource) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_transition_usage_specialization_lowering_gap.md") (qualified-name "TransitionUsageStateSpecialization::Machine::idle")))))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_transition_usage_specialization_lowering_gap.md") (qualified-name "TransitionUsageStateSpecialization::Machine::Route"))) (kind transitionTarget) (ordinal 0))
      (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_transition_usage_specialization_lowering_gap.md") (qualified-name "TransitionUsageStateSpecialization::Machine::running")))))
  )
  (relationships
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/generated_conditional_transition_usage_specialization_lowering_gap.md") (qualified-name "TransitionUsageStateSpecialization::Machine::Route"))) (target (node (document "memory://snapshot/generated_conditional_transition_usage_specialization_lowering_gap.md") (qualified-name "TransitionUsageStateSpecialization::Machine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_conditional_transition_usage_specialization_lowering_gap.md") (qualified-name "TransitionUsageStateSpecialization::Machine::Route"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/generated_conditional_transition_usage_specialization_lowering_gap.md") (qualified-name "TransitionUsageStateSpecialization::Machine::Route"))) (target (node (document "memory://snapshot/generated_conditional_transition_usage_specialization_lowering_gap.md") (qualified-name "TransitionUsageStateSpecialization::Machine::running"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_conditional_transition_usage_specialization_lowering_gap.md") (qualified-name "TransitionUsageStateSpecialization::Machine::Route"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_transition_usage_specialization_lowering_gap.md") (qualified-name "TransitionUsageStateSpecialization::Machine"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_transition_usage_specialization_lowering_gap.md") (qualified-name "TransitionUsageStateSpecialization::Machine::Route"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_transition_usage_specialization_lowering_gap.md") (qualified-name "TransitionUsageStateSpecialization::Machine::Route"))) (target (node (document "memory://snapshot/generated_conditional_transition_usage_specialization_lowering_gap.md") (qualified-name "TransitionUsageStateSpecialization::Machine"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_transition_usage_specialization_lowering_gap.md") (qualified-name "TransitionUsageStateSpecialization::Machine::idle"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_transition_usage_specialization_lowering_gap.md") (qualified-name "TransitionUsageStateSpecialization::Machine::idle"))) (target (node (document "memory://snapshot/generated_conditional_transition_usage_specialization_lowering_gap.md") (qualified-name "TransitionUsageStateSpecialization::Machine"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_transition_usage_specialization_lowering_gap.md") (qualified-name "TransitionUsageStateSpecialization::Machine::running"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_transition_usage_specialization_lowering_gap.md") (qualified-name "TransitionUsageStateSpecialization::Machine::running"))) (target (node (document "memory://snapshot/generated_conditional_transition_usage_specialization_lowering_gap.md") (qualified-name "TransitionUsageStateSpecialization::Machine"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/generated_conditional_transition_usage_specialization_lowering_gap.md") (qualified-name "TransitionUsageStateSpecialization::Machine")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/control_performances.md") (qualified-name "ControlPerformances::DecisionPerformance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_transition_usage_specialization_lowering_gap.md") (qualified-name "TransitionUsageStateSpecialization::Machine::Route")))
      (featured-by (node (document "memory://snapshot/generated_conditional_transition_usage_specialization_lowering_gap.md") (qualified-name "TransitionUsageStateSpecialization::Machine")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_transition_usage_specialization_lowering_gap.md") (qualified-name "TransitionUsageStateSpecialization::Machine::idle")))
      (featured-by (node (document "memory://snapshot/generated_conditional_transition_usage_specialization_lowering_gap.md") (qualified-name "TransitionUsageStateSpecialization::Machine")))
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
    (declaration (id (node (document "memory://snapshot/generated_conditional_transition_usage_specialization_lowering_gap.md") (qualified-name "TransitionUsageStateSpecialization::Machine::running")))
      (featured-by (node (document "memory://snapshot/generated_conditional_transition_usage_specialization_lowering_gap.md") (qualified-name "TransitionUsageStateSpecialization::Machine")))
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
  (query (document "memory://snapshot/generated_conditional_transition_usage_specialization_lowering_gap.md") (range (start 4 31) (end 4 35)) (probe (position 4 31))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_transition_usage_specialization_lowering_gap.md") (qualified-name "TransitionUsageStateSpecialization::Machine::Route"))) (kind transitionSource) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_transition_usage_specialization_lowering_gap.md") (qualified-name "TransitionUsageStateSpecialization::Machine::idle")))))
    )
  )
  (query (document "memory://snapshot/generated_conditional_transition_usage_specialization_lowering_gap.md") (range (start 4 41) (end 4 48)) (probe (position 4 41))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_transition_usage_specialization_lowering_gap.md") (qualified-name "TransitionUsageStateSpecialization::Machine::Route"))) (kind transitionTarget) (ordinal 0) (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_transition_usage_specialization_lowering_gap.md") (qualified-name "TransitionUsageStateSpecialization::Machine::running")))))
    )
  )
)
~~~
