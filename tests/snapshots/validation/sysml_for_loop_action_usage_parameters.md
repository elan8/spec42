# META
~~~ini
description=SysML 8.3.17.9 validateForLoopActionUsageParameters requires a ForLoopActionUsage to have exactly two owned input parameters
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.17.9 validateForLoopActionUsageParameters
type=file
skip_validation=the implied input parameters of this action form are not published, so the rule has no parameter list to count; only the accepted side is authored here
~~~
# SOURCE
~~~sysml
// Conforming: the for loop below owns the sequence and body input parameters its concrete syntax
// implies.
//
// The violating side has no textual counterpart: SysML for syntax always authors a loop variable,
// a sequence and a body, so a source document cannot produce a ForLoopActionUsage with a
// different owned input parameter count.
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
  (document "memory://snapshot/sysml_for_loop_action_usage_parameters.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_for_loop_action_usage_parameters.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_action_body_element")
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
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:3e71fe0ff27b7e2bfe250785942a0bf60e675a9194d188077b887d6758ae00a8") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_for_loop_action_usage_parameters.md") (qualified-name "Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_for_loop_action_usage_parameters.md") (qualified-name "Actions::Act"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_for_loop_action_usage_parameters.md") (qualified-name "Actions::Act::part"))) (kind ref) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_for_loop_action_usage_parameters.md") (qualified-name "Actions::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
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
    (declaration (id (node (document "memory://snapshot/sysml_for_loop_action_usage_parameters.md") (qualified-name "Actions::Act::part")))
      (featured-by (node (document "memory://snapshot/sysml_for_loop_action_usage_parameters.md") (qualified-name "Actions::Act")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
