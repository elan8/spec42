# META
~~~ini
description=SysML 8.3.17.2 checkAcceptActionUsageReceiverBindingConnector requires the payload argument and receiver input parameter to have a canonical binding connector
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.17.2:checkAcceptActionUsageReceiverBindingConnector
blocked_by=parser-gap-76-action-body-members
type=file
~~~
# SOURCE
~~~sysml
package Actions {
    action def Act {
        accept when true;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (binding-connector-check
    (rule_id "sysml-2.0:8.3.17.2:checkAcceptActionUsageReceiverBindingConnector")
    (outcome satisfied)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_accept_action_usage_receiver_binding_connector.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_action_body_element")
        (source "parser")
        (range (start 2 8) (end 3 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:1d34c31ed27841b6cf65ee20c45160fa5684731fce31faec3f763abc11e7f392") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_accept_action_usage_receiver_binding_connector.md") (qualified-name "Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_accept_action_usage_receiver_binding_connector.md") (qualified-name "Actions::Act"))) (kind action-def) (membership (kind owning) (visibility default)))
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
