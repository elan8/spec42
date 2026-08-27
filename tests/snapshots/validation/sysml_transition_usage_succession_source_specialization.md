# META
~~~ini
description=SysML checkTransitionUsageSuccessionSourceSpecialization desired semantics
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.18.9:checkTransitionUsageSuccessionSourceSpecialization
libraries=standard
type=file
~~~
# SOURCE
~~~sysml
package Model {
    state def Machine {
        state idle;
        state running;

        // The implicit-source form has no source on either derived endpoint.
        transition initial then idle;

        // The transition source and its owned succession source resolve to the same state.
        transition move first idle then running;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics (specialization-check (rule_id "sysml-2.0:8.3.18.9:checkTransitionUsageSuccessionSourceSpecialization") (outcome satisfied)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_final_state")
        (source "semantic")
        (range (start 1 4) (end 10 5))
      )
      (diagnostic
        (severity information)
        (code "missing_initial_state")
        (source "semantic")
        (range (start 1 4) (end 10 5))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:734d79e98783cb237ca03517a707f3dd9d2394ac35a4451b2718851a0929433b") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine::idle"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine::initial"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionTarget (reference "idle")))))
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (path (named (kind package) (name "Model")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "initial")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (succession (reference "idle")))))
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine::move"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "idle")) (transitionTarget (reference "running")))))
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (path (named (kind package) (name "Model")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "move")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (succession (reference "idle")) (succession (reference "running")))))
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine::running"))) (kind state) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine::initial"))) (kind transitionTarget) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine::idle")))))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (path (named (kind package) (name "Model")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "initial")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine::idle")))))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine::move"))) (kind transitionSource) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine::idle")))))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine::move"))) (kind transitionTarget) (ordinal 0))
      (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine::running")))))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (path (named (kind package) (name "Model")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "move")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine::idle")))))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (path (named (kind package) (name "Model")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "move")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine::running")))))
  )
  (relationships
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine::initial"))) (target (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine::initial"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (path (named (kind package) (name "Model")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "initial")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (path (named (kind package) (name "Model")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "initial")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine::move"))) (target (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine::move"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine::move"))) (target (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine::running"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine::move"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (path (named (kind package) (name "Model")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "move")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (path (named (kind package) (name "Model")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "move")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (path (named (kind package) (name "Model")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "move")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine::running"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (path (named (kind package) (name "Model")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "move")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine::idle"))) (target (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine::idle"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine::initial"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine::initial"))) (target (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine::move"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine::move"))) (target (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine::running"))) (target (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine::running"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/control_performances.md") (qualified-name "ControlPerformances::DecisionPerformance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine::idle")))
      (featured-by (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/control_performances.md") (qualified-name "ControlPerformances::DecisionPerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine::initial")))
      (featured-by (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine::move")))
      (featured-by (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine::running")))
      (featured-by (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/control_performances.md") (qualified-name "ControlPerformances::DecisionPerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (range (start 6 32) (end 6 36)) (probe (position 6 32))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine::initial"))) (kind transitionTarget) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine::idle")))))
    )
  )
  (query (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (range (start 6 32) (end 6 36)) (probe (position 6 32))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (path (named (kind package) (name "Model")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "initial")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine::idle")))))
    )
  )
  (query (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (range (start 9 30) (end 9 34)) (probe (position 9 30))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine::move"))) (kind transitionSource) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine::idle")))))
    )
  )
  (query (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (range (start 9 40) (end 9 47)) (probe (position 9 40))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine::move"))) (kind transitionTarget) (ordinal 0) (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine::running")))))
    )
  )
  (query (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (range (start 9 30) (end 9 34)) (probe (position 9 30))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (path (named (kind package) (name "Model")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "move")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine::idle")))))
    )
  )
  (query (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (range (start 9 40) (end 9 47)) (probe (position 9 40))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (path (named (kind package) (name "Model")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "move")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_succession_source_specialization.md") (qualified-name "Model::Machine::running")))))
    )
  )
)
~~~
