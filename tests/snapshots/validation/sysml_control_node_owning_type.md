# META
~~~ini
description=SysML 8.3.17.6 validateControlNodeOwningType requires the owningType of a ControlNode to be an ActionDefinition or an ActionUsage
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.17.6 validateControlNodeOwningType
type=file
skip_validation=the pinned parser has no control-node production -- `join j;` is parsed as a typed usage whose type reference `join` does not resolve -- so no ControlNode reaches semantics
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
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 3 13) (end 3 14))
      )
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
    (declaration (id (node (document "memory://snapshot/sysml_control_node_owning_type.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind fork) (ordinal 0))))) (kind fork) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (forkInput (reference "f")))))
    (declaration (id (node (document "memory://snapshot/sysml_control_node_owning_type.md") (qualified-name "Actions::Holder"))) (kind part-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_control_node_owning_type.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind fork) (ordinal 0))))) (kind forkInput) (ordinal 0))
      (authored-target "f")
      (outcome (status unresolved)))
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
    (declaration (id (node (document "memory://snapshot/sysml_control_node_owning_type.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind fork) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_control_node_owning_type.md") (qualified-name "Actions::Act")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_control_node_owning_type.md") (range (start 3 13) (end 3 14)) (probe (position 3 13))
    (reference (id (source (node (document "memory://snapshot/sysml_control_node_owning_type.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind fork) (ordinal 0))))) (kind forkInput) (ordinal 0) (authored-target "f")
      (outcome (status unresolved)))
    )
  )
)
~~~
