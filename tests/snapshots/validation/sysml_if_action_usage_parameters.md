# META
~~~ini
description=SysML 8.3.17.10 validateIfActionUsageParameters requires an IfActionUsage to have at least two owned input parameters
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.17.10 validateIfActionUsageParameters
type=file
skip_validation=the implied input parameters of this action form are not published, so the rule has no parameter list to count; only the accepted side is authored here
~~~
# SOURCE
~~~sysml
// Conforming: the if action below owns the condition and then-branch input parameters its
// concrete syntax implies.
//
// The violating side has no textual counterpart: SysML if syntax always authors a condition and
// at least one branch, so a source document cannot produce an IfActionUsage with fewer than two
// owned input parameters.
package Actions {
    action def Act {
        action a1;
        action a2;
        if true then a1 else a2;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_if_action_usage_parameters.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_if_action_usage_parameters.md"
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
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:2e457c706c8fd46dc33725cb52f366beeb45cca51109e24d6d83c379d0c3556e") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (qualified-name "Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (qualified-name "Actions::Act"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (qualified-name "Actions::Act::a1"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (qualified-name "Actions::Act::a2"))) (kind action) (membership (kind feature) (visibility default)))
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
    (declaration (id (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (qualified-name "Actions::Act::a1")))
      (featured-by (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (qualified-name "Actions::Act")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (qualified-name "Actions::Act::a2")))
      (featured-by (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (qualified-name "Actions::Act")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
