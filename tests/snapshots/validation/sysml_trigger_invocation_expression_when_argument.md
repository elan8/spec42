# META
~~~ini
description=SysML 8.3.17.17 validateTriggerInvocationExpressionWhenArgument requires a when TriggerInvocationExpression argument to be a FeatureReferenceExpression whose referent has a Boolean result
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.17.17 validateTriggerInvocationExpressionWhenArgument
type=file
skip_validation=no semantic rule checks the argument of a when trigger; the canonical code trigger_when_argument_not_boolean does not exist yet
~~~
# SOURCE
~~~sysml
package Triggers {
    action def Act {
        // Conforming: a Boolean when condition.
        accept when true;

        // Invalid: the when condition is not Boolean.
        accept when 1;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_trigger_invocation_expression_when_argument.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "trigger_when_argument_not_boolean")
        (source "semantic")
        (range (start 6 8) (end 6 22))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_trigger_invocation_expression_when_argument.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_action_body_element")
        (source "parser")
        (range (start 3 8) (end 6 8))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 3 8) (end 6 8))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:17b043642d050122c92d903d446076c906dd7f2f5085d39dc81f8b81c8562b8b") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_trigger_invocation_expression_when_argument.md") (qualified-name "Triggers"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_trigger_invocation_expression_when_argument.md") (qualified-name "Triggers::Act"))) (kind action-def) (membership (kind owning) (visibility default)))
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
