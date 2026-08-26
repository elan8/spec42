# META
~~~ini
description=SysML 8.3.17.6 validateControlNodeIsComposite requires a ControlNode to be composite
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.17.6 validateControlNodeIsComposite
source_expectation=accepted
rule_family=validate
expectation=by_construction
rule_id=sysml-2.0:8.3.17.6:validateControlNodeIsComposite
blocked_by=abstract-syntax-invalid-control-node-shape
type=file
~~~
# SOURCE
~~~sysml
package Actions {
    action def Act {
        // Conforming: a control node declared in an action body.
        fork f;
    }
}
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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:9816577c7f43dbc25b0a96250f506aac58a7c50d4bc136b2e3e8fa774c21ff50") (contract-version "feature-value-expression-results-v5"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_control_node_is_composite.md") (qualified-name "Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_control_node_is_composite.md") (qualified-name "Actions::Act"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_control_node_is_composite.md") (qualified-name "Actions::Act::f"))) (kind fork) (membership (kind feature) (visibility default)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_control_node_is_composite.md") (qualified-name "Actions::Act::f"))) (target (node (document "memory://snapshot/sysml_control_node_is_composite.md") (qualified-name "Actions::Act"))) (provenance implied))
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
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
