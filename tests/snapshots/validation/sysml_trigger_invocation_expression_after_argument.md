# META
~~~ini
description=SysML 8.3.17.17 validateTriggerInvocationExpressionAfterArgument requires an after TriggerInvocationExpression to have an argument whose result conforms to Quantities::ScalarQuantityValue with a time reference
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.17.17 validateTriggerInvocationExpressionAfterArgument
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.17.17:validateTriggerInvocationExpressionAfterArgument
blocked_by=semantic-trigger-invocation-argument-typing
type=file
~~~
# SOURCE
~~~sysml
package Triggers {
    attribute def Reading;
    action def Act {
        ref attribute duration : ISQ::DurationValue;
        ref attribute reading : Reading;

        // Conforming: the after argument is a scalar quantity value measured against time.
        accept after duration;

        // Invalid: the after argument is not a scalar quantity value.
        accept after reading;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "trigger_after_argument_not_duration")
        (source "semantic")
        (range (start 10 8) (end 10 29))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 3 33) (end 3 51))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:b1d63eb15740fff6262b52f62e626ee1735c6d4ddfce53bec94aa94fad63744b") (contract-version "constructor-expression-result-v8"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (qualified-name "Triggers"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (qualified-name "Triggers::Act"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (path (named (kind package) (name "Triggers")) (named (kind action-def) (name "Act")) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "duration")))))
    (declaration (id (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (path (named (kind package) (name "Triggers")) (named (kind action-def) (name "Act")) (anonymous (kind accept-action) (ordinal 1))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "reading")))))
    (declaration (id (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (qualified-name "Triggers::Act::duration"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (modifiers reference)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ISQ::DurationValue")))))
    (declaration (id (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (qualified-name "Triggers::Act::reading"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (modifiers reference)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Reading")))))
    (declaration (id (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (qualified-name "Triggers::Reading"))) (kind attribute-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (path (named (kind package) (name "Triggers")) (named (kind action-def) (name "Act")) (anonymous (kind accept-action) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "duration")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (qualified-name "Triggers::Act::duration")))))
    (reference (id (source (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (path (named (kind package) (name "Triggers")) (named (kind action-def) (name "Act")) (anonymous (kind accept-action) (ordinal 1))))) (kind expressionOperand) (ordinal 0))
      (authored-target "reading")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (qualified-name "Triggers::Act::reading")))))
    (reference (id (source (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (qualified-name "Triggers::Act::duration"))) (kind featureTyping) (ordinal 0))
      (authored-target "ISQ::DurationValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (qualified-name "Triggers::Act::reading"))) (kind featureTyping) (ordinal 0))
      (authored-target "Reading")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (qualified-name "Triggers::Reading")))))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (path (named (kind package) (name "Triggers")) (named (kind action-def) (name "Act")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (qualified-name "Triggers::Act::duration"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (path (named (kind package) (name "Triggers")) (named (kind action-def) (name "Act")) (anonymous (kind accept-action) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (path (named (kind package) (name "Triggers")) (named (kind action-def) (name "Act")) (anonymous (kind accept-action) (ordinal 1))))) (target (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (qualified-name "Triggers::Act::reading"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (path (named (kind package) (name "Triggers")) (named (kind action-def) (name "Act")) (anonymous (kind accept-action) (ordinal 1))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (qualified-name "Triggers::Act::reading"))) (target (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (qualified-name "Triggers::Reading"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (qualified-name "Triggers::Act::reading"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (path (named (kind package) (name "Triggers")) (named (kind action-def) (name "Act")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (qualified-name "Triggers::Act"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (path (named (kind package) (name "Triggers")) (named (kind action-def) (name "Act")) (anonymous (kind accept-action) (ordinal 1))))) (target (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (qualified-name "Triggers::Act"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (qualified-name "Triggers::Act::duration"))) (target (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (qualified-name "Triggers::Act"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (qualified-name "Triggers::Act::reading"))) (target (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (qualified-name "Triggers::Act"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (path (named (kind package) (name "Triggers")) (named (kind action-def) (name "Act")) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (qualified-name "Triggers::Act")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (path (named (kind package) (name "Triggers")) (named (kind action-def) (name "Act")) (anonymous (kind accept-action) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (qualified-name "Triggers::Act")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (qualified-name "Triggers::Act::duration")))
      (featured-by (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (qualified-name "Triggers::Act")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (qualified-name "Triggers::Act::reading")))
      (featured-by (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (qualified-name "Triggers::Act")))
      (type (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (qualified-name "Triggers::Reading")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (qualified-name "Triggers::Reading")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (qualified-name "Triggers::Reading")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (qualified-name "Triggers::Reading")))
      (subtype (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (qualified-name "Triggers::Act::reading")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (range (start 7 21) (end 7 29)) (probe (position 7 21))
    (reference (id (source (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (path (named (kind package) (name "Triggers")) (named (kind action-def) (name "Act")) (anonymous (kind accept-action) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "duration")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (qualified-name "Triggers::Act::duration")))))
    )
  )
  (query (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (range (start 10 21) (end 10 28)) (probe (position 10 21))
    (reference (id (source (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (path (named (kind package) (name "Triggers")) (named (kind action-def) (name "Act")) (anonymous (kind accept-action) (ordinal 1))))) (kind expressionOperand) (ordinal 0) (authored-target "reading")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (qualified-name "Triggers::Act::reading")))))
    )
  )
  (query (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (range (start 3 33) (end 3 51)) (probe (position 3 33))
    (reference (id (source (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (qualified-name "Triggers::Act::duration"))) (kind featureTyping) (ordinal 0) (authored-target "ISQ::DurationValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (range (start 4 32) (end 4 39)) (probe (position 4 32))
    (reference (id (source (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (qualified-name "Triggers::Act::reading"))) (kind featureTyping) (ordinal 0) (authored-target "Reading")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_trigger_invocation_expression_after_argument.md") (qualified-name "Triggers::Reading")))))
    )
  )
)
~~~
