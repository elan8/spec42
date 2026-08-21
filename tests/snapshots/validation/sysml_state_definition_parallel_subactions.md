# META
~~~ini
description=SysML 8.3.18.5 validateStateDefinitionParallelSubactions forbids the ownedActions of a parallel StateDefinition from having incoming or outgoing transitions
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.18.5 validateStateDefinitionParallelSubactions
type=file
skip_validation=the pinned parser has no production for the `parallel` state modifier, so the whole parallel state definition is reported as unexpected_keyword_in_scope
~~~
# SOURCE
~~~sysml
package States {
    // Conforming: a parallel state definition whose substates carry no transitions.
    parallel state def Good {
        state left;
        state right;
    }

    // Invalid: a parallel state definition whose substates are joined by a transition.
    parallel state def Bad {
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
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 2 4) (end 13 0))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:c8ef93e253a03fde3a5c9e8418caea997f4e1f236144a210561ca693034f973c") (contract-version "parser-owned-resolution-v1"))
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
