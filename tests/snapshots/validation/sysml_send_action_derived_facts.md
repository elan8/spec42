# META
~~~ini
description=SysML SendActionUsage derives payload, sender, and receiver argument expressions from its canonical action identity
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=sysml-2.0:8.3.17.15:deriveSendActionUsagePayloadArgument
rule_id=sysml-2.0:8.3.17.15:deriveSendActionUsageReceiverArgument
rule_id=sysml-2.0:8.3.17.15:deriveSendActionUsageSenderArgument
libraries=none
~~~
# SOURCE
~~~sysml
package Actions { action def Procedure { action sender; action receiver; send 1 via sender to receiver; } }
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (action-derived-fact (rule_id "sysml-2.0:8.3.17.15:deriveSendActionUsagePayloadArgument") (source "Actions::Procedure::") (position 1) (outcome resolved))
  (action-derived-fact (rule_id "sysml-2.0:8.3.17.15:deriveSendActionUsageReceiverArgument") (source "Actions::Procedure::") (position 3) (outcome resolved))
  (action-derived-fact (rule_id "sysml-2.0:8.3.17.15:deriveSendActionUsageSenderArgument") (source "Actions::Procedure::") (position 2) (outcome resolved))
  (action-derived-fact (rule_id "sysml-2.0:8.3.17.15:deriveSendActionUsagePayloadArgument") (source "Actions::Procedure") (outcome absent)))
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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:1470344c5745bbe8fc1b18c8f0e097a563bb59cd07f654afed1526ee55a65990") (contract-version "owned-cross-feature-typing-v4"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (qualified-name "Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (qualified-name "Actions::Procedure"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind send-action) (ordinal 0))))) (kind send-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)) (authored (membership (kind feature) (visibility default)) (relationships (acceptVia (reference "sender")) (sendTarget (reference "receiver")))))
    (declaration (id (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (qualified-name "Actions::Procedure::receiver"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (qualified-name "Actions::Procedure::sender"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind send-action) (ordinal 0))))) (kind acceptVia) (ordinal 0))
      (authored-target "sender")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (qualified-name "Actions::Procedure::sender")))))
    (reference (id (source (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind send-action) (ordinal 0))))) (kind sendTarget) (ordinal 0))
      (authored-target "receiver")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (qualified-name "Actions::Procedure::receiver")))))
  )
  (relationships
    (relationship (kind acceptVia) (source (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind send-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (qualified-name "Actions::Procedure::sender"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind send-action) (ordinal 0))))) (kind acceptVia) (ordinal 0)))
    (relationship (kind sendTarget) (source (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind send-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (qualified-name "Actions::Procedure::receiver"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind send-action) (ordinal 0))))) (kind sendTarget) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind send-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (qualified-name "Actions::Procedure"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (qualified-name "Actions::Procedure::receiver"))) (target (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (qualified-name "Actions::Procedure"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (qualified-name "Actions::Procedure::sender"))) (target (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (qualified-name "Actions::Procedure"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind send-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (qualified-name "Actions::Procedure")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (qualified-name "Actions::Procedure::receiver")))
      (featured-by (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (qualified-name "Actions::Procedure")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (qualified-name "Actions::Procedure::sender")))
      (featured-by (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (qualified-name "Actions::Procedure")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_send_action_derived_facts.md") (range (start 0 84) (end 0 90)) (probe (position 0 84))
    (reference (id (source (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind send-action) (ordinal 0))))) (kind acceptVia) (ordinal 0) (authored-target "sender")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (qualified-name "Actions::Procedure::sender")))))
    )
  )
  (query (document "memory://snapshot/sysml_send_action_derived_facts.md") (range (start 0 94) (end 0 102)) (probe (position 0 94))
    (reference (id (source (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind send-action) (ordinal 0))))) (kind sendTarget) (ordinal 0) (authored-target "receiver")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_send_action_derived_facts.md") (qualified-name "Actions::Procedure::receiver")))))
    )
  )
)
~~~
