# META
~~~ini
description=SysML 8.3.17.6 validateControlNodeIsComposite requires a ControlNode to be composite
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.17.6 validateControlNodeIsComposite
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.17.6:validateControlNodeIsComposite
blocked_by=parser-gap-directed-control-node
type=file
~~~
# SOURCE
~~~sysml
package Actions {
    action def Act {
        // Conforming: a control node declared in an action body is composite.
        fork f;

        // Invalid: ControlNodePrefix admits RefPrefix, whose FeatureDirection makes the usage
        // referential (the Pilot's UsageAdapter.postProcess clears isComposite for a direction).
        in fork g;
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
        (range (start 7 8) (end 7 18))
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
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:ba395f263d38e635dc0a6677665fbf2bb8320dfce8d0cff2b06d8b30c3c76dc6"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_control_node_is_composite.md") (qualified-name "Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_control_node_is_composite.md") (qualified-name "Actions::Act"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_control_node_is_composite.md") (qualified-name "Actions::Act::f"))) (kind fork) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_control_node_is_composite.md") (qualified-name "Actions::Act::g"))) (kind fork) (membership (kind feature) (visibility default)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_control_node_is_composite.md") (qualified-name "Actions::Act::f"))) (target (node (document "memory://snapshot/sysml_control_node_is_composite.md") (qualified-name "Actions::Act"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_control_node_is_composite.md") (qualified-name "Actions::Act::g"))) (target (node (document "memory://snapshot/sysml_control_node_is_composite.md") (qualified-name "Actions::Act"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_control_node_is_composite.md") (qualified-name "Actions::Act::f")))
      (featured-by (node (document "memory://snapshot/sysml_control_node_is_composite.md") (qualified-name "Actions::Act")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_control_node_is_composite.md") (qualified-name "Actions::Act::g")))
      (featured-by (node (document "memory://snapshot/sysml_control_node_is_composite.md") (qualified-name "Actions::Act")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
