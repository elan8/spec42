# META
~~~ini
description=SysML 8.3.17.6 validateControlNodeOwningType requires the owningType of a ControlNode to be an ActionDefinition or an ActionUsage
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.17.6 validateControlNodeOwningType
source_expectation=accepted
rule_family=validate
expectation=by_construction
rule_id=sysml-2.0:8.3.17.6:validateControlNodeOwningType
blocked_by=abstract-syntax-invalid-control-node-owner
type=file
~~~
# SOURCE
~~~sysml
package Actions {
    // Conforming: the control node is owned by an action definition.
    action def Act {
        fork f;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_control_node_owning_type.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:194e77c1003c39ec7f71d2cbd3078c28e16d076b1ec75d928f4dea7cad23c245") (contract-version "feature-chain-expression-result-v10"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_control_node_owning_type.md") (qualified-name "Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_control_node_owning_type.md") (qualified-name "Actions::Act"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_control_node_owning_type.md") (qualified-name "Actions::Act::f"))) (kind fork) (membership (kind feature) (visibility default)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_control_node_owning_type.md") (qualified-name "Actions::Act::f"))) (target (node (document "memory://snapshot/sysml_control_node_owning_type.md") (qualified-name "Actions::Act"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_control_node_owning_type.md") (qualified-name "Actions::Act::f")))
      (featured-by (node (document "memory://snapshot/sysml_control_node_owning_type.md") (qualified-name "Actions::Act")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
