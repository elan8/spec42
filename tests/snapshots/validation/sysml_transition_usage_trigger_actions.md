# META
~~~ini
description=SysML 8.3.18.9 validateTransitionUsageTriggerActions forbids a TransitionUsage whose source is not a StateUsage from having any triggerActions
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.18.9 validateTransitionUsageTriggerActions
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.18.9:validateTransitionUsageTriggerActions
type=file
~~~
# SOURCE
~~~sysml
package Transitions {
    state def Machine {
        entry; then idle;
        state idle;
        state running;

        // Conforming: the triggered transition leaves a state usage.
        transition first idle accept when true then running;
        transition first running then idle;
    }
    action def Flow {
        action step;
        action done;

        // Invalid: the triggered transition leaves an action usage, not a state usage.
        transition first step accept when true then done;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_transition_usage_trigger_actions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "transition_trigger_source_not_state")
        (source "semantic")
        (range (start 15 8) (end 15 57))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_transition_usage_trigger_actions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "transition_trigger_source_not_state")
        (source "semantic")
        (range (start 15 8) (end 15 57))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:2a2b00f0399864e66eef1a2f3f959237ad6f10bfe248688d119f464332e5fdab"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Flow"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind action-def) (name "Flow")) (anonymous (kind transition) (ordinal 0))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "step")) (transitionTarget (reference "done")))))
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind action-def) (name "Flow")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (succession (reference "step")) (succession (reference "done")))))
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind action-def) (name "Flow")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind action-def) (name "Flow")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind action-def) (name "Flow")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind parameter) (ordinal 1))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind action-def) (name "Flow")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction inout)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Flow::done"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Flow::step"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Machine"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (kind initial-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (initialState (reference "idle")))))
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "idle")) (transitionTarget (reference "running")))))
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "running")) (transitionTarget (reference "idle")))))
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (succession (reference "idle")) (succession (reference "running")))))
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind parameter) (ordinal 1))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (succession (reference "running")) (succession (reference "idle")))))
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction inout)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Machine::idle"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Machine::running"))) (kind state) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind action-def) (name "Flow")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0))
      (authored-target "step")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Flow::step")))))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind action-def) (name "Flow")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0))
      (authored-target "done")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Flow::done")))))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind action-def) (name "Flow")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "step")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Flow::step")))))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind action-def) (name "Flow")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "done")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Flow::done")))))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Machine::idle")))))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Machine::idle")))))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1))))) (kind transitionSource) (ordinal 0))
      (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Machine::running")))))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0))
      (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Machine::running")))))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1))))) (kind transitionTarget) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Machine::idle")))))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Machine::idle")))))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Machine::running")))))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Machine::running")))))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Machine::idle")))))
  )
  (relationships
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind action-def) (name "Flow")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Flow::step"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind action-def) (name "Flow")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind action-def) (name "Flow")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Flow::done"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind action-def) (name "Flow")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind action-def) (name "Flow")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Flow::step"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind action-def) (name "Flow")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind action-def) (name "Flow")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Flow::done"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind action-def) (name "Flow")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind initialState) (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Machine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Machine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1))))) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Machine::running"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1))))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Machine::running"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1))))) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Machine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Machine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Machine::running"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Machine::running"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Machine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind action-def) (name "Flow")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Flow"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind action-def) (name "Flow")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind action-def) (name "Flow")) (anonymous (kind transition) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind action-def) (name "Flow")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind action-def) (name "Flow")) (anonymous (kind transition) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind action-def) (name "Flow")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind action-def) (name "Flow")) (anonymous (kind transition) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind action-def) (name "Flow")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind action-def) (name "Flow")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind action-def) (name "Flow")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind action-def) (name "Flow")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Flow::done"))) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Flow"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Flow::step"))) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Flow"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Machine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Machine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1))))) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Machine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Machine::idle"))) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Machine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Machine::running"))) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Machine"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind action-def) (name "Flow")) (anonymous (kind transition) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Flow")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind action-def) (name "Flow")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind action-def) (name "Flow")) (anonymous (kind transition) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind action-def) (name "Flow")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind action-def) (name "Flow")) (anonymous (kind transition) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind action-def) (name "Flow")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind parameter) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind action-def) (name "Flow")) (anonymous (kind transition) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind action-def) (name "Flow")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind action-def) (name "Flow")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind action-def) (name "Flow")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind action-def) (name "Flow")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind parameter) (ordinal 1)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Flow::done")))
      (featured-by (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Flow")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Flow::step")))
      (featured-by (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Flow")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Machine")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Machine")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Machine")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind parameter) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)))))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind parameter) (ordinal 1)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Machine::idle")))
      (featured-by (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Machine")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Machine::running")))
      (featured-by (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Machine")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (range (start 15 25) (end 15 29)) (probe (position 15 25))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind action-def) (name "Flow")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0) (authored-target "step")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Flow::step")))))
    )
  )
  (query (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (range (start 15 52) (end 15 56)) (probe (position 15 52))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind action-def) (name "Flow")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0) (authored-target "done")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Flow::done")))))
    )
  )
  (query (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (range (start 15 25) (end 15 29)) (probe (position 15 25))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind action-def) (name "Flow")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "step")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Flow::step")))))
    )
  )
  (query (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (range (start 15 52) (end 15 56)) (probe (position 15 52))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind action-def) (name "Flow")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "done")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Flow::done")))))
    )
  )
  (query (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (range (start 2 20) (end 2 24)) (probe (position 2 20))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Machine::idle")))))
    )
  )
  (query (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (range (start 7 25) (end 7 29)) (probe (position 7 25))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Machine::idle")))))
    )
  )
  (query (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (range (start 8 25) (end 8 32)) (probe (position 8 25))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1))))) (kind transitionSource) (ordinal 0) (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Machine::running")))))
    )
  )
  (query (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (range (start 7 52) (end 7 59)) (probe (position 7 52))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0) (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Machine::running")))))
    )
  )
  (query (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (range (start 8 38) (end 8 42)) (probe (position 8 38))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1))))) (kind transitionTarget) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Machine::idle")))))
    )
  )
  (query (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (range (start 7 25) (end 7 29)) (probe (position 7 25))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Machine::idle")))))
    )
  )
  (query (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (range (start 8 25) (end 8 32)) (probe (position 8 25))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Machine::running")))))
    )
  )
  (query (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (range (start 7 52) (end 7 59)) (probe (position 7 52))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Machine::running")))))
    )
  )
  (query (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (range (start 8 38) (end 8 42)) (probe (position 8 38))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_trigger_actions.md") (qualified-name "Transitions::Machine::idle")))))
    )
  )
)
~~~
