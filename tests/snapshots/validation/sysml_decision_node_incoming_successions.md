# META
~~~ini
description=SysML 8.3.17.7 validateDecisionNodeIncomingSuccessions allows a DecisionNode at most one incoming Succession
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.17.7 validateDecisionNodeIncomingSuccessions
type=file
skip_validation=the parser models a control-node statement only as a reference: JoinStmt.join, ForkStmt.fork and DecisionStmt.decide are each a Node<Expression> path with no declared-name field, so `join good;` publishes an anonymous control node with an unresolved joinInput reference to `good` rather than declaring a named ControlNode
~~~
# SOURCE
~~~sysml
package Actions {
    action def Act {
        action a1;
        action a2;

        // Conforming: one incoming succession.
        decide good;
        succession first a1 then good;

        // Invalid: two incoming successions.
        decide bad;
        succession first a1 then bad;
        succession first a2 then bad;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_decision_node_incoming_successions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "decision_node_multiple_incoming")
        (source "semantic")
        (range (start 12 8) (end 12 37))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_decision_node_incoming_successions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 6 15) (end 6 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 7 33) (end 7 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 10 15) (end 10 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 33) (end 11 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 12 33) (end 12 36))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:bd668e2a73f9e16e2b8f8e8baf9f86f5a0c9b53fdeac689a2c1546a8ceeab34c") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (qualified-name "Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (qualified-name "Actions::Act"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind decide) (ordinal 0))))) (kind decide) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (decisionInput (reference "good")))))
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "a1")) (succession (reference "good")))))
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind decide) (ordinal 1))))) (kind decide) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (decisionInput (reference "bad")))))
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "a1")) (succession (reference "bad")))))
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "a2")) (succession (reference "bad")))))
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (qualified-name "Actions::Act::a1"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (qualified-name "Actions::Act::a2"))) (kind action) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "a1")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (qualified-name "Actions::Act::a1")))))
    (reference (id (source (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 0))
      (authored-target "a1")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (qualified-name "Actions::Act::a1")))))
    (reference (id (source (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 0))
      (authored-target "a2")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (qualified-name "Actions::Act::a2")))))
    (reference (id (source (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "good")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 1))
      (authored-target "bad")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 1))
      (authored-target "bad")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind decide) (ordinal 0))))) (kind decisionInput) (ordinal 0))
      (authored-target "good")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind decide) (ordinal 1))))) (kind decisionInput) (ordinal 0))
      (authored-target "bad")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (qualified-name "Actions::Act::a1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (target (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (qualified-name "Actions::Act::a1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 2))))) (target (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (qualified-name "Actions::Act::a2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind decide) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (qualified-name "Actions::Act")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (qualified-name "Actions::Act")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind decide) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (qualified-name "Actions::Act")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (qualified-name "Actions::Act")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (qualified-name "Actions::Act")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (qualified-name "Actions::Act::a1")))
      (featured-by (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (qualified-name "Actions::Act")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (qualified-name "Actions::Act::a2")))
      (featured-by (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (qualified-name "Actions::Act")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (range (start 7 25) (end 7 27)) (probe (position 7 25))
    (reference (id (source (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "a1")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (qualified-name "Actions::Act::a1")))))
    )
  )
  (query (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (range (start 11 25) (end 11 27)) (probe (position 11 25))
    (reference (id (source (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 0) (authored-target "a1")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (qualified-name "Actions::Act::a1")))))
    )
  )
  (query (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (range (start 12 25) (end 12 27)) (probe (position 12 25))
    (reference (id (source (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 0) (authored-target "a2")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (qualified-name "Actions::Act::a2")))))
    )
  )
  (query (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (range (start 7 33) (end 7 37)) (probe (position 7 33))
    (reference (id (source (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "good")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (range (start 11 33) (end 11 36)) (probe (position 11 33))
    (reference (id (source (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 1) (authored-target "bad")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (range (start 12 33) (end 12 36)) (probe (position 12 33))
    (reference (id (source (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 1) (authored-target "bad")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (range (start 6 15) (end 6 19)) (probe (position 6 15))
    (reference (id (source (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind decide) (ordinal 0))))) (kind decisionInput) (ordinal 0) (authored-target "good")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (range (start 10 15) (end 10 18)) (probe (position 10 15))
    (reference (id (source (node (document "memory://snapshot/sysml_decision_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind decide) (ordinal 1))))) (kind decisionInput) (ordinal 0) (authored-target "bad")
      (outcome (status unresolved)))
    )
  )
)
~~~
