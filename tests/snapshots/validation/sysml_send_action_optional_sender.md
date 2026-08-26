# META
~~~ini
description=SysML SendActionUsage has no sender argument when its optional via clause is absent
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=sysml-2.0:8.3.17.15:deriveSendActionUsageSenderArgument
coverage_role=secondary
libraries=none
~~~
# SOURCE
~~~sysml
package Actions { action def Procedure { action receiver; send 1 to receiver; } }
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics (action-derived-fact (rule_id "sysml-2.0:8.3.17.15:deriveSendActionUsageSenderArgument") (source "Actions::Procedure::") (outcome absent)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_send_action_optional_sender.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:1ca6a609151b06c8508354500f0ff1bd6539384199059939336d463b66e021bd") (contract-version "feature-chain-expression-result-v10"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_send_action_optional_sender.md") (qualified-name "Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_send_action_optional_sender.md") (qualified-name "Actions::Procedure"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_send_action_optional_sender.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind send-action) (ordinal 0))))) (kind send-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)) (authored (membership (kind feature) (visibility default)) (relationships (sendTarget (reference "receiver")))))
    (declaration (id (node (document "memory://snapshot/sysml_send_action_optional_sender.md") (qualified-name "Actions::Procedure::receiver"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_send_action_optional_sender.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind send-action) (ordinal 0))))) (kind sendTarget) (ordinal 0))
      (authored-target "receiver")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_send_action_optional_sender.md") (qualified-name "Actions::Procedure::receiver")))))
  )
  (relationships
    (relationship (kind sendTarget) (source (node (document "memory://snapshot/sysml_send_action_optional_sender.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind send-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_send_action_optional_sender.md") (qualified-name "Actions::Procedure::receiver"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_send_action_optional_sender.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind send-action) (ordinal 0))))) (kind sendTarget) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_send_action_optional_sender.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind send-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_send_action_optional_sender.md") (qualified-name "Actions::Procedure"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_send_action_optional_sender.md") (qualified-name "Actions::Procedure::receiver"))) (target (node (document "memory://snapshot/sysml_send_action_optional_sender.md") (qualified-name "Actions::Procedure"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_send_action_optional_sender.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind send-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_send_action_optional_sender.md") (qualified-name "Actions::Procedure")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_send_action_optional_sender.md") (qualified-name "Actions::Procedure::receiver")))
      (featured-by (node (document "memory://snapshot/sysml_send_action_optional_sender.md") (qualified-name "Actions::Procedure")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_send_action_optional_sender.md") (range (start 0 68) (end 0 76)) (probe (position 0 68))
    (reference (id (source (node (document "memory://snapshot/sysml_send_action_optional_sender.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind send-action) (ordinal 0))))) (kind sendTarget) (ordinal 0) (authored-target "receiver")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_send_action_optional_sender.md") (qualified-name "Actions::Procedure::receiver")))))
    )
  )
)
~~~
