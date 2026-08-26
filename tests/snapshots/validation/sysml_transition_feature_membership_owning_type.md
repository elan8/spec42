# META
~~~ini
description=SysML 8.3.18.8 validateTransitionFeatureMembershipOwningType requires the owningType of a TransitionFeatureMembership to be a TransitionUsage
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.18.8 validateTransitionFeatureMembershipOwningType
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.18.8:validateTransitionFeatureMembershipOwningType
blocked_by=lowering-transition-feature-memberships
type=file
~~~
# SOURCE
~~~sysml
// Conforming: the guard and effect features below are owned by the transition that declares
// them.
//
// The violating side has no textual counterpart: SysML concrete syntax authors a transition
// feature only inside a transition declaration, so a source document cannot give one a
// non-TransitionUsage owning type.
package Transitions {
    state def Machine {
        state idle;
        state running;
        transition first idle if true then running;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_transition_feature_membership_owning_type.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_transition_feature_membership_owning_type.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_final_state")
        (source "semantic")
        (range (start 7 4) (end 11 5))
      )
      (diagnostic
        (severity information)
        (code "missing_initial_state")
        (source "semantic")
        (range (start 7 4) (end 11 5))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:e12c0b256061ca304460ced303930211107e77801653d01a50577c5c95381be2") (contract-version "constructor-expression-specialization-v9"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_owning_type.md") (qualified-name "Transitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_owning_type.md") (qualified-name "Transitions::Machine"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_owning_type.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "idle")) (transitionTarget (reference "running")))))
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_owning_type.md") (qualified-name "Transitions::Machine::idle"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_owning_type.md") (qualified-name "Transitions::Machine::running"))) (kind state) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_owning_type.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_owning_type.md") (qualified-name "Transitions::Machine::idle")))))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_owning_type.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0))
      (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_owning_type.md") (qualified-name "Transitions::Machine::running")))))
  )
  (relationships
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/sysml_transition_feature_membership_owning_type.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_owning_type.md") (qualified-name "Transitions::Machine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_feature_membership_owning_type.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/sysml_transition_feature_membership_owning_type.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_owning_type.md") (qualified-name "Transitions::Machine::running"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_feature_membership_owning_type.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_feature_membership_owning_type.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_owning_type.md") (qualified-name "Transitions::Machine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_feature_membership_owning_type.md") (qualified-name "Transitions::Machine::idle"))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_owning_type.md") (qualified-name "Transitions::Machine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_feature_membership_owning_type.md") (qualified-name "Transitions::Machine::running"))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_owning_type.md") (qualified-name "Transitions::Machine"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/sysml_transition_feature_membership_owning_type.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (state literal) (value (kind boolean) (boolean true)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_owning_type.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_transition_feature_membership_owning_type.md") (qualified-name "Transitions::Machine")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_owning_type.md") (qualified-name "Transitions::Machine::idle")))
      (featured-by (node (document "memory://snapshot/sysml_transition_feature_membership_owning_type.md") (qualified-name "Transitions::Machine")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_owning_type.md") (qualified-name "Transitions::Machine::running")))
      (featured-by (node (document "memory://snapshot/sysml_transition_feature_membership_owning_type.md") (qualified-name "Transitions::Machine")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_transition_feature_membership_owning_type.md") (range (start 10 25) (end 10 29)) (probe (position 10 25))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_owning_type.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_owning_type.md") (qualified-name "Transitions::Machine::idle")))))
    )
  )
  (query (document "memory://snapshot/sysml_transition_feature_membership_owning_type.md") (range (start 10 43) (end 10 50)) (probe (position 10 43))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_owning_type.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0) (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_owning_type.md") (qualified-name "Transitions::Machine::running")))))
    )
  )
)
~~~
