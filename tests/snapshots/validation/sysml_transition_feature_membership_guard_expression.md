# META
~~~ini
description=SysML 8.3.18.8 validateTransitionFeatureMembershipGuardExpression requires the transitionFeature of a guard TransitionFeatureMembership to be an Expression with a Boolean result
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.18.8 validateTransitionFeatureMembershipGuardExpression
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.18.8:validateTransitionFeatureMembershipGuardExpression
blocked_by=lowering-transition-feature-memberships
type=file
~~~
# SOURCE
~~~sysml
package Transitions {
    state def Machine {
        state idle;
        state running;

        // Conforming: a Boolean guard.
        transition good first idle if true then running;

        // Invalid: the guard is not Boolean.
        transition bad first idle if 1 then running;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "transition_guard_non_boolean")
        (source "semantic")
        (range (start 9 8) (end 9 52))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md"
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
      (diagnostic
        (severity warning)
        (code "transition_guard_non_boolean")
        (source "semantic")
        (range (start 9 8) (end 9 52))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:79cf25576c49e35811506c88780317359e076bd9592449c5ec0faf3ab41ce4a7") (contract-version "parser-owned-resolution-v2"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::bad"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "idle")) (transitionTarget (reference "running")))))
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::good"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "idle")) (transitionTarget (reference "running")))))
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::idle"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::running"))) (kind state) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::bad"))) (kind transitionSource) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::idle")))))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::bad"))) (kind transitionTarget) (ordinal 0))
      (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::running")))))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::good"))) (kind transitionSource) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::idle")))))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::good"))) (kind transitionTarget) (ordinal 0))
      (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::running")))))
  )
  (relationships
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::bad"))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::bad"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::bad"))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::running"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::bad"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::good"))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::good"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::good"))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::running"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::good"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::bad"))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::good"))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::idle"))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::running"))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::bad"))) (state literal) (value (kind integer) (integer 1)))
    (evaluated (declaration (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::good"))) (state literal) (value (kind boolean) (boolean true)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::bad")))
      (featured-by (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::good")))
      (featured-by (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::idle")))
      (featured-by (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::running")))
      (featured-by (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (range (start 9 29) (end 9 33)) (probe (position 9 29))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::bad"))) (kind transitionSource) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::idle")))))
    )
  )
  (query (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (range (start 9 44) (end 9 51)) (probe (position 9 44))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::bad"))) (kind transitionTarget) (ordinal 0) (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::running")))))
    )
  )
  (query (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (range (start 6 30) (end 6 34)) (probe (position 6 30))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::good"))) (kind transitionSource) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::idle")))))
    )
  )
  (query (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (range (start 6 48) (end 6 55)) (probe (position 6 48))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::good"))) (kind transitionTarget) (ordinal 0) (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::running")))))
    )
  )
)
~~~
