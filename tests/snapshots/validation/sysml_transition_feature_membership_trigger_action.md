# META
~~~ini
description=SysML 8.3.18.8 validateTransitionFeatureMembershipTriggerAction requires the transitionFeature of a trigger TransitionFeatureMembership to be an AcceptActionUsage
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.18.8 validateTransitionFeatureMembershipTriggerAction
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.18.8:validateTransitionFeatureMembershipTriggerAction
blocked_by=lowering-transition-feature-memberships
type=file
~~~
# SOURCE
~~~sysml
// Conforming: the trigger feature below is the accept action the transition syntax authors.
//
// The violating side has no textual counterpart: the accept clause of a transition always
// produces an AcceptActionUsage, so a source document cannot give a trigger membership another
// kind of transition feature.
package Transitions {
    state def Machine {
        state idle;
        state running;
        transition first idle accept when true then running;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_transition_feature_membership_trigger_action.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_transition_feature_membership_trigger_action.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_final_state")
        (source "semantic")
        (range (start 6 4) (end 10 5))
      )
      (diagnostic
        (severity information)
        (code "missing_initial_state")
        (source "semantic")
        (range (start 6 4) (end 10 5))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:2809cf864f7a3d4ba19d9f904af75b1c3e565f3438c53531f54377493917af84") (contract-version "semantic-metadata-projection-v6"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_trigger_action.md") (qualified-name "Transitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_trigger_action.md") (qualified-name "Transitions::Machine"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_trigger_action.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "idle")) (transitionTarget (reference "running")))))
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_trigger_action.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_trigger_action.md") (qualified-name "Transitions::Machine::idle"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_trigger_action.md") (qualified-name "Transitions::Machine::running"))) (kind state) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_trigger_action.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_trigger_action.md") (qualified-name "Transitions::Machine::idle")))))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_trigger_action.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0))
      (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_trigger_action.md") (qualified-name "Transitions::Machine::running")))))
  )
  (relationships
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/sysml_transition_feature_membership_trigger_action.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_trigger_action.md") (qualified-name "Transitions::Machine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_feature_membership_trigger_action.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/sysml_transition_feature_membership_trigger_action.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_trigger_action.md") (qualified-name "Transitions::Machine::running"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_feature_membership_trigger_action.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_feature_membership_trigger_action.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_trigger_action.md") (qualified-name "Transitions::Machine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_feature_membership_trigger_action.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_trigger_action.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_feature_membership_trigger_action.md") (qualified-name "Transitions::Machine::idle"))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_trigger_action.md") (qualified-name "Transitions::Machine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_feature_membership_trigger_action.md") (qualified-name "Transitions::Machine::running"))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_trigger_action.md") (qualified-name "Transitions::Machine"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_trigger_action.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_transition_feature_membership_trigger_action.md") (qualified-name "Transitions::Machine")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_trigger_action.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_transition_feature_membership_trigger_action.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_trigger_action.md") (qualified-name "Transitions::Machine::idle")))
      (featured-by (node (document "memory://snapshot/sysml_transition_feature_membership_trigger_action.md") (qualified-name "Transitions::Machine")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_trigger_action.md") (qualified-name "Transitions::Machine::running")))
      (featured-by (node (document "memory://snapshot/sysml_transition_feature_membership_trigger_action.md") (qualified-name "Transitions::Machine")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_transition_feature_membership_trigger_action.md") (range (start 9 25) (end 9 29)) (probe (position 9 25))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_trigger_action.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_trigger_action.md") (qualified-name "Transitions::Machine::idle")))))
    )
  )
  (query (document "memory://snapshot/sysml_transition_feature_membership_trigger_action.md") (range (start 9 52) (end 9 59)) (probe (position 9 52))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_trigger_action.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0) (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_trigger_action.md") (qualified-name "Transitions::Machine::running")))))
    )
  )
)
~~~
