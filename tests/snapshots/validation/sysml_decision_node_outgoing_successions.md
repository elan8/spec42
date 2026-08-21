# META
~~~ini
description=SysML 8.3.17.7 validateDecisionNodeOutgoingSuccessions requires all outgoing Successions from a DecisionNode to have a target multiplicity of 0..1
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.17.7 validateDecisionNodeOutgoingSuccessions
type=file
skip_validation=the parser models a control-node statement only as a reference: JoinStmt.join, ForkStmt.fork and DecisionStmt.decide are each a Node<Expression> path with no declared-name field, so `join good;` publishes an anonymous control node with an unresolved joinInput reference to `good` rather than declaring a named ControlNode
~~~
# SOURCE
~~~sysml
package Actions {
    action def Act {
        action a1;
        decide d;

        // Conforming: the outgoing succession has a 0..1 target multiplicity.
        succession first d then [0..1] a1;

        // Invalid: the outgoing succession widens its target multiplicity.
        succession first d then [0..*] a1;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_decision_node_outgoing_successions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "decision_node_outgoing_multiplicity")
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
  (document "memory://snapshot/sysml_decision_node_outgoing_successions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 3 15) (end 3 16))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 6 25) (end 6 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 9 25) (end 9 26))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:8b52f5b26e279816bff64ac4f74e2f3fb94d030d00a75a285afb807953c873ab") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (qualified-name "Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (qualified-name "Actions::Act"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind decide) (ordinal 0))))) (kind decide) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (decisionInput (reference "d")))))
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "d")) (succession (reference "a1")))))
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "d")) (succession (reference "a1")))))
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (qualified-name "Actions::Act::a1"))) (kind action) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "d")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 0))
      (authored-target "d")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "a1")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (qualified-name "Actions::Act::a1")))))
    (reference (id (source (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 1))
      (authored-target "a1")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (qualified-name "Actions::Act::a1")))))
    (reference (id (source (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind decide) (ordinal 0))))) (kind decisionInput) (ordinal 0))
      (authored-target "d")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (qualified-name "Actions::Act::a1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (target (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (qualified-name "Actions::Act::a1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 1)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind decide) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (qualified-name "Actions::Act")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (qualified-name "Actions::Act")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (qualified-name "Actions::Act")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (qualified-name "Actions::Act::a1")))
      (featured-by (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (qualified-name "Actions::Act")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (range (start 6 25) (end 6 26)) (probe (position 6 25))
    (reference (id (source (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "d")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (range (start 9 25) (end 9 26)) (probe (position 9 25))
    (reference (id (source (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 0) (authored-target "d")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (range (start 6 39) (end 6 41)) (probe (position 6 39))
    (reference (id (source (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "a1")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (qualified-name "Actions::Act::a1")))))
    )
  )
  (query (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (range (start 9 39) (end 9 41)) (probe (position 9 39))
    (reference (id (source (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 1) (authored-target "a1")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (qualified-name "Actions::Act::a1")))))
    )
  )
  (query (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (range (start 3 15) (end 3 16)) (probe (position 3 15))
    (reference (id (source (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind decide) (ordinal 0))))) (kind decisionInput) (ordinal 0) (authored-target "d")
      (outcome (status unresolved)))
    )
  )
)
~~~
