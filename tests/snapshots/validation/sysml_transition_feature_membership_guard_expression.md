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
type=file
~~~
# SOURCE
~~~sysml
package Transitions {
    state def Machine {
        state idle;
        state running;
        entry; then idle;
        transition first running then idle;

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
        (range (start 11 8) (end 11 52))
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
        (severity warning)
        (code "transition_guard_non_boolean")
        (source "semantic")
        (range (start 11 8) (end 11 52))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:f2ff4404da7522cdc9f1b4028ac26e37df2f649831f96a8df98f898188e933c9"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (kind initial-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (initialState (reference "idle")))))
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "running")) (transitionTarget (reference "idle")))))
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (succession (reference "running")) (succession (reference "idle")))))
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::bad"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "idle")) (transitionTarget (reference "running")))))
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "bad")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (succession (reference "idle")) (succession (reference "running")))))
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "bad")) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "bad")) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "bad")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (kind kerml-boolean-expression) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::good"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "idle")) (transitionTarget (reference "running")))))
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "good")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (succession (reference "idle")) (succession (reference "running")))))
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "good")) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "good")) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "good")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (kind kerml-boolean-expression) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::idle"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::running"))) (kind state) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::idle")))))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0))
      (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::running")))))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::idle")))))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::running")))))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::idle")))))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::bad"))) (kind transitionSource) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::idle")))))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::bad"))) (kind transitionTarget) (ordinal 0))
      (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::running")))))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "bad")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::idle")))))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "bad")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::running")))))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::good"))) (kind transitionSource) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::idle")))))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::good"))) (kind transitionTarget) (ordinal 0))
      (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::running")))))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "good")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::idle")))))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "good")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::running")))))
  )
  (relationships
    (relationship (kind initialState) (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::running"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::running"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::bad"))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::bad"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::bad"))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::running"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::bad"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "bad")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "bad")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "bad")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::running"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "bad")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::good"))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::good"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::good"))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::running"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::good"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "good")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "good")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "good")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::running"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "good")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::bad"))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "bad")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::bad"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "bad")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::bad"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "bad")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::bad"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::good"))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "good")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::good"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "good")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::good"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "good")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::good"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::idle"))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::running"))) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "bad")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (state literal) (value (kind integer) (integer 1)))
    (evaluated (declaration (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "good")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (state literal) (value (kind boolean) (boolean true)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::bad")))
      (featured-by (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "bad")) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::bad")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "bad")) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::bad")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "bad")) (anonymous (kind kerml-boolean-expression) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::bad")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::good")))
      (featured-by (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "good")) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::good")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "good")) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::good")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "good")) (anonymous (kind kerml-boolean-expression) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::good")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::idle")))
      (featured-by (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::running")))
      (featured-by (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine")))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "bad")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (outcome resolved) (literal (value (kind integer) (integer 1))))
  (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "good")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (outcome resolved) (literal (value (kind boolean) (boolean true))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (range (start 4 20) (end 4 24)) (probe (position 4 20))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::idle")))))
    )
  )
  (query (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (range (start 5 25) (end 5 32)) (probe (position 5 25))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0) (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::running")))))
    )
  )
  (query (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (range (start 5 38) (end 5 42)) (probe (position 5 38))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::idle")))))
    )
  )
  (query (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (range (start 5 25) (end 5 32)) (probe (position 5 25))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::running")))))
    )
  )
  (query (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (range (start 5 38) (end 5 42)) (probe (position 5 38))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::idle")))))
    )
  )
  (query (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (range (start 11 29) (end 11 33)) (probe (position 11 29))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::bad"))) (kind transitionSource) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::idle")))))
    )
  )
  (query (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (range (start 11 44) (end 11 51)) (probe (position 11 44))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::bad"))) (kind transitionTarget) (ordinal 0) (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::running")))))
    )
  )
  (query (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (range (start 11 29) (end 11 33)) (probe (position 11 29))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "bad")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::idle")))))
    )
  )
  (query (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (range (start 11 44) (end 11 51)) (probe (position 11 44))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "bad")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::running")))))
    )
  )
  (query (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (range (start 8 30) (end 8 34)) (probe (position 8 30))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::good"))) (kind transitionSource) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::idle")))))
    )
  )
  (query (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (range (start 8 48) (end 8 55)) (probe (position 8 48))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::good"))) (kind transitionTarget) (ordinal 0) (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::running")))))
    )
  )
  (query (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (range (start 8 30) (end 8 34)) (probe (position 8 30))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "good")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::idle")))))
    )
  )
  (query (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (range (start 8 48) (end 8 55)) (probe (position 8 48))
    (reference (id (source (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (path (named (kind package) (name "Transitions")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "good")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_transition_feature_membership_guard_expression.md") (qualified-name "Transitions::Machine::running")))))
    )
  )
)
~~~
