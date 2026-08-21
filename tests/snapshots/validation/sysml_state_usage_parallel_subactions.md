# META
~~~ini
description=SysML 8.3.18.6 validateStateUsageParallelSubactions forbids the nestedActions of a parallel StateUsage from having incoming or outgoing transitions
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.18.6 validateStateUsageParallelSubactions
type=file
skip_validation=the pinned parser has no production for the `parallel` state modifier, so the whole parallel state definition is reported as unexpected_keyword_in_scope
~~~
# SOURCE
~~~sysml
package States {
    state def Machine {
        // Conforming: a parallel state whose substates carry no transitions.
        parallel state good {
            state left;
            state right;
        }

        // Invalid: a parallel state whose substates are joined by a transition.
        parallel state bad {
            state left;
            state right;
            transition first left then right;
        }
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_state_usage_parallel_subactions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "parallel_state_substate_transition")
        (source "semantic")
        (range (start 12 12) (end 12 45))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_state_usage_parallel_subactions.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 3 8) (end 9 8))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 9 8) (end 14 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:59dc9b175c2c338116473f34317c0ac3d76f3cb7ce51d3832927e3309f8204a8") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (qualified-name "States"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (qualified-name "States::Machine"))) (kind state-def) (membership (kind owning) (visibility default)))
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
