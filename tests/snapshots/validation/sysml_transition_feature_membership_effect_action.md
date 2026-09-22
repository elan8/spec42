# META
~~~ini
description=SysML 8.3.18.8 validateTransitionFeatureMembershipEffectAction requires the transitionFeature of an effect TransitionFeatureMembership to be an ActionUsage
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.18.8 validateTransitionFeatureMembershipEffectAction
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.18.8:validateTransitionFeatureMembershipEffectAction
type=file
~~~
# SOURCE
~~~sysml
// Conforming: EffectBehaviorMember (`do`) precedes TransitionSuccessionMember (`then`)
// (SysML BNF 1277-1286). The previous `then … do …` order is not a production.
//
// The violating side has no textual counterpart: the do clause always produces an ActionUsage.
package Transitions {
    state def Machine {
        state idle;
        state running;
        action notify;
        transition first idle do notify then running;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_final_state")
        (source "semantic")
        (range (start 5 4) (end 10 5))
      )
      (diagnostic
        (severity information)
        (code "missing_initial_state")
        (source "semantic")
        (range (start 5 4) (end 10 5))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_final_state")
        (source "semantic")
        (range (start 5 4) (end 10 5))
      )
      (diagnostic
        (severity information)
        (code "missing_initial_state")
        (source "semantic")
        (range (start 5 4) (end 10 5))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:2f9ecd512cc8eae180c29a37a5d119fa169f080df8c6baa397ac476b37392a7c"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (qualified-name "Transitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (qualified-name "Transitions::Machine"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "idle")) (transitionTarget (reference "running")) (transitionEffect (reference "notify")))))
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (succession (reference "idle")) (succession (reference "running")))))
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind action) (ordinal 0))))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (qualified-name "Transitions::Machine::idle"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (qualified-name "Transitions::Machine::notify"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (qualified-name "Transitions::Machine::running"))) (kind state) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (qualified-name "Transitions::Machine::idle")))))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0))
      (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (qualified-name "Transitions::Machine::running")))))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionEffect) (ordinal 0))
      (authored-target "notify")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (qualified-name "Transitions::Machine::notify")))))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (qualified-name "Transitions::Machine::idle")))))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (qualified-name "Transitions::Machine::running")))))
  )
  (relationships
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (qualified-name "Transitions::Machine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (qualified-name "Transitions::Machine::running"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionEffect) (source (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (qualified-name "Transitions::Machine::notify"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionEffect) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (qualified-name "Transitions::Machine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (qualified-name "Transitions::Machine::running"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (qualified-name "Transitions::Machine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (qualified-name "Transitions::Machine::idle"))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (qualified-name "Transitions::Machine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (qualified-name "Transitions::Machine::notify"))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (qualified-name "Transitions::Machine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (qualified-name "Transitions::Machine::running"))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (qualified-name "Transitions::Machine"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (qualified-name "Transitions::Machine")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (qualified-name "Transitions::Machine::idle")))
      (featured-by (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (qualified-name "Transitions::Machine")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (qualified-name "Transitions::Machine::notify")))
      (featured-by (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (qualified-name "Transitions::Machine")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (qualified-name "Transitions::Machine::running")))
      (featured-by (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (qualified-name "Transitions::Machine")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (range (start 9 25) (end 9 29)) (probe (position 9 25))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (qualified-name "Transitions::Machine::idle")))))
    )
  )
  (query (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (range (start 9 45) (end 9 52)) (probe (position 9 45))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0) (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (qualified-name "Transitions::Machine::running")))))
    )
  )
  (query (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (range (start 9 33) (end 9 39)) (probe (position 9 33))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionEffect) (ordinal 0) (authored-target "notify")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (qualified-name "Transitions::Machine::notify")))))
    )
  )
  (query (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (range (start 9 25) (end 9 29)) (probe (position 9 25))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (qualified-name "Transitions::Machine::idle")))))
    )
  )
  (query (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (range (start 9 45) (end 9 52)) (probe (position 9 45))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (qualified-name "Transitions::Machine::running")))))
    )
  )
)
~~~
