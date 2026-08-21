# META
~~~ini
description=SysML 8.3.17.6 validateControlNodeIsComposite requires a ControlNode to be composite
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.17.6 validateControlNodeIsComposite
type=file
skip_validation=the pinned parser rejects a `ref` prefix on a control node, reporting recovered_action_body_element, so the referential control node never reaches semantics
~~~
# SOURCE
~~~sysml
package Actions {
    action def Act {
        action a1;
        action a2;

        // Conforming: a composite control node.
        fork composite f;

        // Invalid: a referential control node.
        ref fork g;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_control_node_is_composite.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "control_node_not_composite")
        (source "semantic")
        (range (start 9 8) (end 9 19))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_control_node_is_composite.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_action_body_element")
        (source "parser")
        (range (start 6 8) (end 9 8))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:e545809ddca24c6284ce08ed894e6e7560d1d3bca93a7c095b79e263d801cbf9") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_control_node_is_composite.md") (qualified-name "Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_control_node_is_composite.md") (qualified-name "Actions::Act"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_control_node_is_composite.md") (qualified-name "Actions::Act::a1"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_control_node_is_composite.md") (qualified-name "Actions::Act::a2"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_control_node_is_composite.md") (qualified-name "Actions::Act::fork"))) (kind ref) (membership (kind feature) (visibility default)))
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
    (declaration (id (node (document "memory://snapshot/sysml_control_node_is_composite.md") (qualified-name "Actions::Act::a1")))
      (featured-by (node (document "memory://snapshot/sysml_control_node_is_composite.md") (qualified-name "Actions::Act")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_control_node_is_composite.md") (qualified-name "Actions::Act::a2")))
      (featured-by (node (document "memory://snapshot/sysml_control_node_is_composite.md") (qualified-name "Actions::Act")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_control_node_is_composite.md") (qualified-name "Actions::Act::fork")))
      (featured-by (node (document "memory://snapshot/sysml_control_node_is_composite.md") (qualified-name "Actions::Act")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
