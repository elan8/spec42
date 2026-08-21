# META
~~~ini
description=SysML 8.3.17.6 validateControlNodeOwningType requires the owningType of a ControlNode to be an ActionDefinition or an ActionUsage
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.17.6 validateControlNodeOwningType
type=file
skip_validation=the pinned parser rejects a control node outside an action body, reporting unexpected_keyword_in_scope, so the invalidly owned control node never reaches semantics
~~~
# SOURCE
~~~sysml
package Actions {
    // Conforming: the control node is owned by an action definition.
    action def Act {
        fork f;
    }

    // Invalid: the control node is owned by a part definition.
    part def Holder {
        fork f;
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
        (range (start 7 4) (end 7 21))
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
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 8 8) (end 9 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:bd4b65c4a0c15bbb9e864dbca2c5d2d60e4548caaa41f11b12a15c10bb2d1876") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_control_node_owning_type.md") (qualified-name "Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_control_node_owning_type.md") (qualified-name "Actions::Act"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_control_node_owning_type.md") (qualified-name "Actions::Act::f"))) (kind fork) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_control_node_owning_type.md") (qualified-name "Actions::Holder"))) (kind part-def) (membership (kind owning) (visibility default)))
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
