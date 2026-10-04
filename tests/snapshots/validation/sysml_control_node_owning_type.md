# META
~~~ini
description=SysML 8.3.17.6 validateControlNodeOwningType requires the owningType of a ControlNode to be an ActionDefinition or an ActionUsage
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.17.6 validateControlNodeOwningType
source_expectation=accepted
rule_family=validate
expectation=by_construction
evidence_reference=file:tests/snapshots/validation/sysml_grammar_restricted_membership_owners.md
rule_id=sysml-2.0:8.3.17.6:validateControlNodeOwningType
type=file
~~~
# SOURCE
~~~sysml
// The violating side has no textual counterpart: the grammar admits this element only in
// action bodies (ControlNode via ActionNode in ActionBodyItem), whose owners all satisfy the rule.
// sysml_grammar_restricted_membership_owners.md pins the parser rejecting a forbidden owner.
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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:bceb9a68063330b5592e507925baa2f9bcf17e959087b6015cc54630d1d4398a"))
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
