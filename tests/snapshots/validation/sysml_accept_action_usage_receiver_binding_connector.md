# META
~~~ini
description=SysML 8.3.17.2 checkAcceptActionUsageReceiverBindingConnector requires the payload argument and receiver input parameter to have a canonical binding connector
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.17.2:checkAcceptActionUsageReceiverBindingConnector
blocked_by=lowering-gap-binding-connector-accept-receiver-endpoints
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
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:1d34c31ed27841b6cf65ee20c45160fa5684731fce31faec3f763abc11e7f392") (contract-version "semantic-metadata-projection-v6"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_accept_action_usage_receiver_binding_connector.md") (qualified-name "Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_accept_action_usage_receiver_binding_connector.md") (qualified-name "Actions::Act"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_accept_action_usage_receiver_binding_connector.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_accept_action_usage_receiver_binding_connector.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_accept_action_usage_receiver_binding_connector.md") (qualified-name "Actions::Act"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_accept_action_usage_receiver_binding_connector.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_accept_action_usage_receiver_binding_connector.md") (qualified-name "Actions::Act")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
