# META
~~~ini
description=SysML 8.3.17.17 validateTriggerInvocationExpressionAtArgument requires an at TriggerInvocationExpression to have an argument whose result conforms to Time::TimeInstantValue
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.17.17 validateTriggerInvocationExpressionAtArgument
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.17.17:validateTriggerInvocationExpressionAtArgument
blocked_by=parser-gap-76-action-body-members
type=file
~~~
# SOURCE
~~~sysml
package Triggers {
    attribute def Reading;
    action def Act {
        ref attribute instant : Time::TimeInstantValue;
        ref attribute reading : Reading;

        // Conforming: the at argument is a time instant value.
        accept at instant;

        // Invalid: the at argument is not a time instant value.
        accept at reading;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_trigger_invocation_expression_at_argument.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "trigger_at_argument_not_time_instant")
        (source "semantic")
        (range (start 10 8) (end 10 26))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_trigger_invocation_expression_at_argument.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 3 32) (end 3 54))
      )
      (diagnostic
        (severity error)
        (code "recovered_action_body_element")
        (source "parser")
        (range (start 7 8) (end 10 8))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 7 8) (end 10 8))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:02e166546ba41c5d6b26cf5ef36f7469106adeec0709fc369f561d0547007102") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_trigger_invocation_expression_at_argument.md") (qualified-name "Triggers"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_trigger_invocation_expression_at_argument.md") (qualified-name "Triggers::Act"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_trigger_invocation_expression_at_argument.md") (qualified-name "Triggers::Act::instant"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (modifiers reference)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Time::TimeInstantValue")))))
    (declaration (id (node (document "memory://snapshot/sysml_trigger_invocation_expression_at_argument.md") (qualified-name "Triggers::Act::reading"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (modifiers reference)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Reading")))))
    (declaration (id (node (document "memory://snapshot/sysml_trigger_invocation_expression_at_argument.md") (qualified-name "Triggers::Reading"))) (kind attribute-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_trigger_invocation_expression_at_argument.md") (qualified-name "Triggers::Act::instant"))) (kind featureTyping) (ordinal 0))
      (authored-target "Time::TimeInstantValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sysml_trigger_invocation_expression_at_argument.md") (qualified-name "Triggers::Act::reading"))) (kind featureTyping) (ordinal 0))
      (authored-target "Reading")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_trigger_invocation_expression_at_argument.md") (qualified-name "Triggers::Reading")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_trigger_invocation_expression_at_argument.md") (qualified-name "Triggers::Act::reading"))) (target (node (document "memory://snapshot/sysml_trigger_invocation_expression_at_argument.md") (qualified-name "Triggers::Reading"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_trigger_invocation_expression_at_argument.md") (qualified-name "Triggers::Act::reading"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_trigger_invocation_expression_at_argument.md") (qualified-name "Triggers::Act::instant"))) (target (node (document "memory://snapshot/sysml_trigger_invocation_expression_at_argument.md") (qualified-name "Triggers::Act"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_trigger_invocation_expression_at_argument.md") (qualified-name "Triggers::Act::reading"))) (target (node (document "memory://snapshot/sysml_trigger_invocation_expression_at_argument.md") (qualified-name "Triggers::Act"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_trigger_invocation_expression_at_argument.md") (qualified-name "Triggers::Act::instant")))
      (featured-by (node (document "memory://snapshot/sysml_trigger_invocation_expression_at_argument.md") (qualified-name "Triggers::Act")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_trigger_invocation_expression_at_argument.md") (qualified-name "Triggers::Act::reading")))
      (featured-by (node (document "memory://snapshot/sysml_trigger_invocation_expression_at_argument.md") (qualified-name "Triggers::Act")))
      (type (node (document "memory://snapshot/sysml_trigger_invocation_expression_at_argument.md") (qualified-name "Triggers::Reading")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_trigger_invocation_expression_at_argument.md") (qualified-name "Triggers::Reading")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_trigger_invocation_expression_at_argument.md") (qualified-name "Triggers::Reading")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_trigger_invocation_expression_at_argument.md") (qualified-name "Triggers::Reading")))
      (subtype (node (document "memory://snapshot/sysml_trigger_invocation_expression_at_argument.md") (qualified-name "Triggers::Act::reading")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_trigger_invocation_expression_at_argument.md") (range (start 3 32) (end 3 54)) (probe (position 3 32))
    (reference (id (source (node (document "memory://snapshot/sysml_trigger_invocation_expression_at_argument.md") (qualified-name "Triggers::Act::instant"))) (kind featureTyping) (ordinal 0) (authored-target "Time::TimeInstantValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/sysml_trigger_invocation_expression_at_argument.md") (range (start 4 32) (end 4 39)) (probe (position 4 32))
    (reference (id (source (node (document "memory://snapshot/sysml_trigger_invocation_expression_at_argument.md") (qualified-name "Triggers::Act::reading"))) (kind featureTyping) (ordinal 0) (authored-target "Reading")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_trigger_invocation_expression_at_argument.md") (qualified-name "Triggers::Reading")))))
    )
  )
)
~~~
