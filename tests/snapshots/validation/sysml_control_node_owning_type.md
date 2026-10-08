# META
~~~ini
description=SysML 8.3.17.6 validateControlNodeOwningType requires the owningType of a ControlNode to be an ActionDefinition or an ActionUsage
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.17.6 validateControlNodeOwningType
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.17.6:validateControlNodeOwningType
blocked_by=semantic-control-node-owning-type
type=file
~~~
# SOURCE
~~~sysml
package Actions {
    // Conforming: the control node is owned by an action definition.
    action def Act {
        fork f;
    }

    // Invalid: a constraint body is a CalculationBody, whose items include every ActionBodyItem,
    // so the grammar admits a control node here; its owning type is not an action.
    constraint def Guarded {
        fork g;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_control_node_owning_type.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "control_node_invalid_owner")
        (source "semantic")
        (range (start 10 8) (end 10 15))
      )
    )
  )
)
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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:cf0bd8fb963122f2856e1d440312a67f681194009bbacc447709e9de307218df"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_control_node_owning_type.md") (qualified-name "Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_control_node_owning_type.md") (qualified-name "Actions::Act"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_control_node_owning_type.md") (qualified-name "Actions::Act::f"))) (kind fork) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_control_node_owning_type.md") (qualified-name "Actions::Guarded"))) (kind constraint-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_control_node_owning_type.md") (qualified-name "Actions::Guarded::g"))) (kind fork) (membership (kind feature) (visibility default)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_control_node_owning_type.md") (qualified-name "Actions::Act::f"))) (target (node (document "memory://snapshot/sysml_control_node_owning_type.md") (qualified-name "Actions::Act"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_control_node_owning_type.md") (qualified-name "Actions::Guarded::g"))) (target (node (document "memory://snapshot/sysml_control_node_owning_type.md") (qualified-name "Actions::Guarded"))) (provenance implied))
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
    (declaration (id (node (document "memory://snapshot/sysml_control_node_owning_type.md") (qualified-name "Actions::Guarded::g")))
      (featured-by (node (document "memory://snapshot/sysml_control_node_owning_type.md") (qualified-name "Actions::Guarded")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
