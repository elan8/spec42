# META
~~~ini
description=SysML 8.3.18.5 validateStateDefinitionParallelSubactions forbids the ownedActions of a parallel StateDefinition from having incoming or outgoing transitions
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.18.5 validateStateDefinitionParallelSubactions
type=file
skip_validation=the pinned parser accepts the `parallel` body modifier on a state usage (src/parser/state.rs) but not on a state def, so `state def Good parallel { ... }` -- valid per the StateDefBody production -- fails with missing_body_or_semicolon
~~~
# SOURCE
~~~sysml
package States {
    // Conforming: a parallel state definition whose substates carry no transitions.
    state def Good parallel {
        state left;
        state right;
    }

    // Invalid: a parallel state definition whose substates are joined by a transition.
    state def Bad parallel {
        state left;
        state right;
        transition first left then right;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_state_definition_parallel_subactions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "parallel_state_substate_transition")
        (source "semantic")
        (range (start 11 8) (end 11 41))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_state_definition_parallel_subactions.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "missing_body_or_semicolon")
        (source "parser")
        (range (start 2 4) (end 8 4))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 2 4) (end 8 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:37627ee1572344f43e132d19531dad5bf8d98eebd6fe8731d1faad49518e7ed5") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_state_definition_parallel_subactions.md") (qualified-name "States"))) (kind package) (membership (kind owning) (visibility default)))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
