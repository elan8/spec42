# META
~~~ini
description=SysML 8.3.17.6 validateControlNodeIncomingSuccessions requires all incoming Successions to a ControlNode to have a target multiplicity of 1..1
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.17.6 validateControlNodeIncomingSuccessions
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.17.6:validateControlNodeIncomingSuccessions
blocked_by=semantic-control-node-incoming-multiplicity
type=file
~~~
# SOURCE
~~~sysml
package Actions {
    action def Act {
        action a1;
        fork f;

        // Conforming: the incoming succession has a 1..1 target multiplicity.
        succession first a1 then [1] f;

        // Invalid: the incoming succession widens its target multiplicity.
        succession first a1 then [0..1] f;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_control_node_incoming_successions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "control_node_incoming_multiplicity")
        (source "semantic")
        (range (start 9 8) (end 9 42))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_control_node_incoming_successions.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:9cea06aa18d7c17c948e23cc125eb6c5963891fade942cec04af5e10da4b88b5") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (qualified-name "Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (qualified-name "Actions::Act"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "a1")) (succession (reference "f")))))
    (declaration (id (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "a1")) (succession (reference "f")))))
    (declaration (id (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (qualified-name "Actions::Act::a1"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (qualified-name "Actions::Act::f"))) (kind fork) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "a1")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (qualified-name "Actions::Act::a1")))))
    (reference (id (source (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 0))
      (authored-target "a1")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (qualified-name "Actions::Act::a1")))))
    (reference (id (source (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "f")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (qualified-name "Actions::Act::f")))))
    (reference (id (source (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 1))
      (authored-target "f")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (qualified-name "Actions::Act::f")))))
  )
  (relationships
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (qualified-name "Actions::Act::a1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (target (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (qualified-name "Actions::Act::a1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (qualified-name "Actions::Act::f"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (target (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (qualified-name "Actions::Act::f"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 1)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (qualified-name "Actions::Act"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (target (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (qualified-name "Actions::Act"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (qualified-name "Actions::Act::a1"))) (target (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (qualified-name "Actions::Act"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (qualified-name "Actions::Act::f"))) (target (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (qualified-name "Actions::Act"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (qualified-name "Actions::Act")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (qualified-name "Actions::Act")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (qualified-name "Actions::Act::a1")))
      (featured-by (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (qualified-name "Actions::Act")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (qualified-name "Actions::Act::f")))
      (featured-by (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (qualified-name "Actions::Act")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_control_node_incoming_successions.md") (range (start 6 25) (end 6 27)) (probe (position 6 25))
    (reference (id (source (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "a1")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (qualified-name "Actions::Act::a1")))))
    )
  )
  (query (document "memory://snapshot/sysml_control_node_incoming_successions.md") (range (start 9 25) (end 9 27)) (probe (position 9 25))
    (reference (id (source (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 0) (authored-target "a1")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (qualified-name "Actions::Act::a1")))))
    )
  )
  (query (document "memory://snapshot/sysml_control_node_incoming_successions.md") (range (start 6 37) (end 6 38)) (probe (position 6 37))
    (reference (id (source (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "f")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (qualified-name "Actions::Act::f")))))
    )
  )
  (query (document "memory://snapshot/sysml_control_node_incoming_successions.md") (range (start 9 40) (end 9 41)) (probe (position 9 40))
    (reference (id (source (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 1) (authored-target "f")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_control_node_incoming_successions.md") (qualified-name "Actions::Act::f")))))
    )
  )
)
~~~
