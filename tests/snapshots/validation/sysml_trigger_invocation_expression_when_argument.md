# META
~~~ini
description=SysML 8.3.17.17 validateTriggerInvocationExpressionWhenArgument requires a when TriggerInvocationExpression argument to be a FeatureReferenceExpression whose referent has a Boolean result
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.17.17 validateTriggerInvocationExpressionWhenArgument
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.17.17:validateTriggerInvocationExpressionWhenArgument
blocked_by=semantic-trigger-invocation-argument-typing
type=file
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
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:17b043642d050122c92d903d446076c906dd7f2f5085d39dc81f8b81c8562b8b") (contract-version "parser-owned-resolution-v2"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_trigger_invocation_expression_when_argument.md") (qualified-name "Triggers"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_trigger_invocation_expression_when_argument.md") (qualified-name "Triggers::Act"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_trigger_invocation_expression_when_argument.md") (path (named (kind package) (name "Triggers")) (named (kind action-def) (name "Act")) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/sysml_trigger_invocation_expression_when_argument.md") (path (named (kind package) (name "Triggers")) (named (kind action-def) (name "Act")) (anonymous (kind accept-action) (ordinal 1))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_trigger_invocation_expression_when_argument.md") (path (named (kind package) (name "Triggers")) (named (kind action-def) (name "Act")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_trigger_invocation_expression_when_argument.md") (qualified-name "Triggers::Act"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_trigger_invocation_expression_when_argument.md") (path (named (kind package) (name "Triggers")) (named (kind action-def) (name "Act")) (anonymous (kind accept-action) (ordinal 1))))) (target (node (document "memory://snapshot/sysml_trigger_invocation_expression_when_argument.md") (qualified-name "Triggers::Act"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_trigger_invocation_expression_when_argument.md") (path (named (kind package) (name "Triggers")) (named (kind action-def) (name "Act")) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_trigger_invocation_expression_when_argument.md") (qualified-name "Triggers::Act")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_trigger_invocation_expression_when_argument.md") (path (named (kind package) (name "Triggers")) (named (kind action-def) (name "Act")) (anonymous (kind accept-action) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/sysml_trigger_invocation_expression_when_argument.md") (qualified-name "Triggers::Act")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
