# META
~~~ini
description=SysML AcceptActionUsage derives payload and receiver roles from ordered canonical action parameters and arguments
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=sysml-2.0:8.3.17.2:deriveAcceptActionUsagePayloadArgument
rule_id=sysml-2.0:8.3.17.2:deriveAcceptActionUsagePayloadParameter
rule_id=sysml-2.0:8.3.17.2:deriveAcceptActionUsageReceiverArgument
blocked_by=parser-gap-76-action-body-members
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
      (diagnostic
        (severity error)
        (code "recovered_action_body_element")
        (source "parser")
        (range (start 0 41) (end 0 59))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:9709fd40f604db97d46214e9201084c787b157a89e10d295681cd3ecd9abc9ad") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_accept_action_derived_facts.md") (qualified-name "Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_accept_action_derived_facts.md") (qualified-name "Actions::Procedure"))) (kind action-def) (membership (kind owning) (visibility default)))
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
