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
blocked_by=parser-gap-77-transition-body-members
type=file
~~~
# SOURCE
~~~sysml
// Conforming: the effect feature below is the action the transition syntax authors.
//
// The violating side has no textual counterpart: the do clause of a transition always produces an
// ActionUsage, so a source document cannot give an effect membership another kind of transition
// feature.
package Transitions {
    state def Machine {
        state idle;
        state running;
        action notify;
        transition first idle then running do notify;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md"
    (diagnostics
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
        (range (start 6 4) (end 11 5))
      )
      (diagnostic
        (severity information)
        (code "missing_initial_state")
        (source "semantic")
        (range (start 6 4) (end 11 5))
      )
      (diagnostic
        (severity error)
        (code "recovered_state_body_element")
        (source "parser")
        (range (start 10 8) (end 11 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:2a5d9438dab8ab4e82aed67d828a4427ed8c08d3b4822e571fcff75b5db91005") (contract-version "constructor-expression-result-v8"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (qualified-name "Transitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (qualified-name "Transitions::Machine"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (qualified-name "Transitions::Machine::idle"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (qualified-name "Transitions::Machine::notify"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/sysml_transition_feature_membership_effect_action.md") (qualified-name "Transitions::Machine::running"))) (kind state) (membership (kind feature) (visibility default)))
  )
  (references
  )
  (relationships
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
)
~~~
