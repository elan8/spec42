# META
~~~ini
description=SysML AcceptActionUsage derives payload and receiver roles from ordered canonical action parameters and arguments
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=sysml-2.0:8.3.17.2:deriveAcceptActionUsagePayloadArgument
rule_id=sysml-2.0:8.3.17.2:deriveAcceptActionUsagePayloadParameter
rule_id=sysml-2.0:8.3.17.2:deriveAcceptActionUsageReceiverArgument
blocked_by=lowering-gap-action-input-parameter-identities
libraries=none
~~~
# SOURCE
~~~sysml
package Actions { action def Procedure { accept when true; } }
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (action-derived-fact (rule_id "sysml-2.0:8.3.17.2:deriveAcceptActionUsagePayloadArgument") (source "Actions::Procedure") (outcome resolved))
  (action-derived-fact (rule_id "sysml-2.0:8.3.17.2:deriveAcceptActionUsagePayloadParameter") (source "Actions::Procedure") (outcome resolved))
  (action-derived-fact (rule_id "sysml-2.0:8.3.17.2:deriveAcceptActionUsageReceiverArgument") (source "Actions::Procedure") (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_accept_action_derived_facts.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:9709fd40f604db97d46214e9201084c787b157a89e10d295681cd3ecd9abc9ad") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_accept_action_derived_facts.md") (qualified-name "Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_accept_action_derived_facts.md") (qualified-name "Actions::Procedure"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_accept_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_accept_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_accept_action_derived_facts.md") (qualified-name "Actions::Procedure"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_accept_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_accept_action_derived_facts.md") (qualified-name "Actions::Procedure")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
