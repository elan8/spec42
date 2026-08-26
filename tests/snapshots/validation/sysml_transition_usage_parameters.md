# META
~~~ini
description=SysML 8.3.18.9 validateTransitionUsageParameters requires a TransitionUsage to have at least one owned input parameter, and at least two when it has a triggerAction
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.18.9 validateTransitionUsageParameters
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.18.9:validateTransitionUsageParameters
blocked_by=lowering-action-parameter-facts
type=file
~~~
# SOURCE
~~~sysml
// Conforming: each transition below owns the input parameters its concrete syntax implies -- one
// without a trigger, two with one.
//
// The violating side has no textual counterpart: the transition syntax always authors them, so a
// source document cannot produce a TransitionUsage with fewer.
package Transitions {
    state def Machine {
        state idle;
        state running;
        transition first idle then running;
        transition first running accept when true then idle;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_transition_usage_parameters.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_transition_usage_parameters.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_initial_state")
        (source "semantic")
        (range (start 6 4) (end 11 5))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:eb54aa19647d9d4ac0c2e8dd1dd7c74bab090c4c244e795fd6187504e3313429") (contract-version "feature-chain-expression-result-v10"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (qualified-name "Transitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (qualified-name "Transitions::Machine"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "idle")) (transitionTarget (reference "running")))))
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "running")) (transitionTarget (reference "idle")))))
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (qualified-name "Transitions::Machine::idle"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (qualified-name "Transitions::Machine::running"))) (kind state) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (qualified-name "Transitions::Machine::idle")))))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1))))) (kind transitionSource) (ordinal 0))
      (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (qualified-name "Transitions::Machine::running")))))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0))
      (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (qualified-name "Transitions::Machine::running")))))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1))))) (kind transitionTarget) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (qualified-name "Transitions::Machine::idle")))))
  )
  (relationships
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (qualified-name "Transitions::Machine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1))))) (target (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (qualified-name "Transitions::Machine::running"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1))))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (qualified-name "Transitions::Machine::running"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1))))) (target (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (qualified-name "Transitions::Machine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (qualified-name "Transitions::Machine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1))))) (target (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (qualified-name "Transitions::Machine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (qualified-name "Transitions::Machine::idle"))) (target (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (qualified-name "Transitions::Machine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (qualified-name "Transitions::Machine::running"))) (target (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (qualified-name "Transitions::Machine"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (qualified-name "Transitions::Machine")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (qualified-name "Transitions::Machine")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)))))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (qualified-name "Transitions::Machine::idle")))
      (featured-by (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (qualified-name "Transitions::Machine")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (qualified-name "Transitions::Machine::running")))
      (featured-by (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (qualified-name "Transitions::Machine")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_transition_usage_parameters.md") (range (start 9 25) (end 9 29)) (probe (position 9 25))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (qualified-name "Transitions::Machine::idle")))))
    )
  )
  (query (document "memory://snapshot/sysml_transition_usage_parameters.md") (range (start 10 25) (end 10 32)) (probe (position 10 25))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1))))) (kind transitionSource) (ordinal 0) (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (qualified-name "Transitions::Machine::running")))))
    )
  )
  (query (document "memory://snapshot/sysml_transition_usage_parameters.md") (range (start 9 35) (end 9 42)) (probe (position 9 35))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0) (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (qualified-name "Transitions::Machine::running")))))
    )
  )
  (query (document "memory://snapshot/sysml_transition_usage_parameters.md") (range (start 10 55) (end 10 59)) (probe (position 10 55))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1))))) (kind transitionTarget) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_parameters.md") (qualified-name "Transitions::Machine::idle")))))
    )
  )
)
~~~
