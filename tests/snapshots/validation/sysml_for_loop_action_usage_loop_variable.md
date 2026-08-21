# META
~~~ini
description=SysML 8.3.17.9 validateForLoopActionUsageLoopVariable requires the first ownedFeature of a ForLoopActionUsage to be a ReferenceUsage
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.17.9 validateForLoopActionUsageLoopVariable
type=file
skip_validation=the for loop variable is not published as an owned feature of the loop action, so the accepted side cannot be observed in SMG either
~~~
# SOURCE
~~~sysml
// Conforming: the loop variable below is the first owned feature of the for loop and is a
// reference usage, which is what the for syntax produces.
//
// The violating side has no textual counterpart: SysML for syntax always authors the loop
// variable first and always as a reference usage.
package Actions {
    part def Component;
    action def Act {
        ref part components : Component[0..*];
        for c : Component in components { action step; }
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_action_body_element")
        (source "parser")
        (range (start 9 8) (end 10 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:ebf39e57feca5f150d63842b093eb1c04d1988160875066b849eb8a6acf1a6c6") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (qualified-name "Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (qualified-name "Actions::Act"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (qualified-name "Actions::Act::part"))) (kind ref) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (qualified-name "Actions::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
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
    (declaration (id (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (qualified-name "Actions::Act::part")))
      (featured-by (node (document "memory://snapshot/sysml_for_loop_action_usage_loop_variable.md") (qualified-name "Actions::Act")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
