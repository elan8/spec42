# META
~~~ini
description=SysML SendActionUsage derives payload, sender, and receiver argument expressions from its canonical action identity
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=sysml-2.0:8.3.17.15:deriveSendActionUsagePayloadArgument
rule_id=sysml-2.0:8.3.17.15:deriveSendActionUsageReceiverArgument
rule_id=sysml-2.0:8.3.17.15:deriveSendActionUsageSenderArgument
blocked_by=lowering-gap-action-anonymous-metaclass-identity
libraries=none
~~~
# SOURCE
~~~sysml
package Actions { action def Procedure { action target; send 1 to target; } }
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (action-derived-fact (rule_id "sysml-2.0:8.3.17.15:deriveSendActionUsagePayloadArgument") (source "Actions::Procedure::send") (outcome resolved))
  (action-derived-fact (rule_id "sysml-2.0:8.3.17.15:deriveSendActionUsageReceiverArgument") (source "Actions::Procedure::send") (outcome resolved))
  (action-derived-fact (rule_id "sysml-2.0:8.3.17.15:deriveSendActionUsageSenderArgument") (source "Actions::Procedure::send") (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_send_action_derived_facts.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:306ff2d5b50b3a937b5a2157f68a35e4e084efc548ca4029ab46926dfcb0a6d2") (contract-version "parser-owned-resolution-v2"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (qualified-name "Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (qualified-name "Actions::Procedure"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind action) (ordinal 0))))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)) (authored (membership (kind feature) (visibility default)) (relationships (sendTarget (reference "target")))))
    (declaration (id (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (qualified-name "Actions::Procedure::target"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind action) (ordinal 0))))) (kind sendTarget) (ordinal 0))
      (authored-target "target")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (qualified-name "Actions::Procedure::target")))))
  )
  (relationships
    (relationship (kind sendTarget) (source (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (qualified-name "Actions::Procedure::target"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind action) (ordinal 0))))) (kind sendTarget) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (qualified-name "Actions::Procedure"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (qualified-name "Actions::Procedure::target"))) (target (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (qualified-name "Actions::Procedure"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (qualified-name "Actions::Procedure")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (qualified-name "Actions::Procedure::target")))
      (featured-by (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (qualified-name "Actions::Procedure")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_send_action_derived_facts.md") (range (start 0 66) (end 0 72)) (probe (position 0 66))
    (reference (id (source (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind action) (ordinal 0))))) (kind sendTarget) (ordinal 0) (authored-target "target")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (qualified-name "Actions::Procedure::target")))))
    )
  )
)
~~~
