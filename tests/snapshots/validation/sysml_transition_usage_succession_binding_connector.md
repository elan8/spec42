# META
~~~ini
description=SysML 8.3.18.9 checkTransitionUsageSuccessionBindingConnector requires a canonical binding connector between a transition succession and TransitionPerformance transitionLink
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.18.9:checkTransitionUsageSuccessionBindingConnector
blocked_by=lowering-transition-usage-succession
type=file
~~~
# SOURCE
~~~sysml
package Transitions {
    state def Machine {
        state idle;
        state running;
        transition first idle then running;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (binding-connector-check
    (rule_id "sysml-2.0:8.3.18.9:checkTransitionUsageSuccessionBindingConnector")
    (outcome satisfied)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_transition_usage_succession_binding_connector.md"
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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:7ae3dae86b43627f927581a6bf08e26ddb5e193075e51adc59ba7e01ce4476bf") (contract-version "parser-owned-resolution-v2"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_succession_binding_connector.md") (qualified-name "Transitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_succession_binding_connector.md") (qualified-name "Transitions::Machine"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_succession_binding_connector.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "idle")) (transitionTarget (reference "running")))))
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_succession_binding_connector.md") (qualified-name "Transitions::Machine::idle"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_succession_binding_connector.md") (qualified-name "Transitions::Machine::running"))) (kind state) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_succession_binding_connector.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_succession_binding_connector.md") (qualified-name "Transitions::Machine::idle")))))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_succession_binding_connector.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0))
      (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_succession_binding_connector.md") (qualified-name "Transitions::Machine::running")))))
  )
  (relationships
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/sysml_transition_usage_succession_binding_connector.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_usage_succession_binding_connector.md") (qualified-name "Transitions::Machine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_usage_succession_binding_connector.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/sysml_transition_usage_succession_binding_connector.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_usage_succession_binding_connector.md") (qualified-name "Transitions::Machine::running"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_usage_succession_binding_connector.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_usage_succession_binding_connector.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_usage_succession_binding_connector.md") (qualified-name "Transitions::Machine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_usage_succession_binding_connector.md") (qualified-name "Transitions::Machine::idle"))) (target (node (document "memory://snapshot/sysml_transition_usage_succession_binding_connector.md") (qualified-name "Transitions::Machine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_usage_succession_binding_connector.md") (qualified-name "Transitions::Machine::running"))) (target (node (document "memory://snapshot/sysml_transition_usage_succession_binding_connector.md") (qualified-name "Transitions::Machine"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_succession_binding_connector.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_transition_usage_succession_binding_connector.md") (qualified-name "Transitions::Machine")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_succession_binding_connector.md") (qualified-name "Transitions::Machine::idle")))
      (featured-by (node (document "memory://snapshot/sysml_transition_usage_succession_binding_connector.md") (qualified-name "Transitions::Machine")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_usage_succession_binding_connector.md") (qualified-name "Transitions::Machine::running")))
      (featured-by (node (document "memory://snapshot/sysml_transition_usage_succession_binding_connector.md") (qualified-name "Transitions::Machine")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_transition_usage_succession_binding_connector.md") (range (start 4 25) (end 4 29)) (probe (position 4 25))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_succession_binding_connector.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_succession_binding_connector.md") (qualified-name "Transitions::Machine::idle")))))
    )
  )
  (query (document "memory://snapshot/sysml_transition_usage_succession_binding_connector.md") (range (start 4 35) (end 4 42)) (probe (position 4 35))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_usage_succession_binding_connector.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0) (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_usage_succession_binding_connector.md") (qualified-name "Transitions::Machine::running")))))
    )
  )
)
~~~
