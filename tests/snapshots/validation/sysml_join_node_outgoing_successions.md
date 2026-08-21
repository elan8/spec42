# META
~~~ini
description=SysML 8.3.17.11 validateJoinNodeOutgoingSuccessions allows a JoinNode at most one outgoing Succession
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.17.11 validateJoinNodeOutgoingSuccessions
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.17.11:validateJoinNodeOutgoingSuccessions
blocked_by=semantic-join-node-multiple-outgoing
type=file
~~~
# SOURCE
~~~sysml
package Actions {
    action def Act {
        action a1;
        action a2;

        // Conforming: one outgoing succession.
        join good;
        succession first good then a1;

        // Invalid: two outgoing successions.
        join bad;
        succession first bad then a1;
        succession first bad then a2;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_join_node_outgoing_successions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "join_node_multiple_outgoing")
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
  (document "memory://snapshot/sysml_join_node_outgoing_successions.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:2119ae649a3f1f8e9dbc93aac50b8b3209a7b22619eedf235d96dda54b7eb517") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (qualified-name "Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (qualified-name "Actions::Act"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "good")) (succession (reference "a1")))))
    (declaration (id (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "bad")) (succession (reference "a1")))))
    (declaration (id (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "bad")) (succession (reference "a2")))))
    (declaration (id (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (qualified-name "Actions::Act::a1"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (qualified-name "Actions::Act::a2"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (qualified-name "Actions::Act::bad"))) (kind join) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (qualified-name "Actions::Act::good"))) (kind join) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "good")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (qualified-name "Actions::Act::good")))))
    (reference (id (source (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 0))
      (authored-target "bad")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (qualified-name "Actions::Act::bad")))))
    (reference (id (source (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 0))
      (authored-target "bad")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (qualified-name "Actions::Act::bad")))))
    (reference (id (source (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "a1")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (qualified-name "Actions::Act::a1")))))
    (reference (id (source (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 1))
      (authored-target "a1")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (qualified-name "Actions::Act::a1")))))
    (reference (id (source (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 1))
      (authored-target "a2")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (qualified-name "Actions::Act::a2")))))
  )
  (relationships
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (qualified-name "Actions::Act::good"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (target (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (qualified-name "Actions::Act::bad"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 2))))) (target (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (qualified-name "Actions::Act::bad"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (qualified-name "Actions::Act::a1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (target (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (qualified-name "Actions::Act::a1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 1)))
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 2))))) (target (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (qualified-name "Actions::Act::a2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 1)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (qualified-name "Actions::Act"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (target (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (qualified-name "Actions::Act"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 2))))) (target (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (qualified-name "Actions::Act"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (qualified-name "Actions::Act::a1"))) (target (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (qualified-name "Actions::Act"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (qualified-name "Actions::Act::a2"))) (target (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (qualified-name "Actions::Act"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (qualified-name "Actions::Act::bad"))) (target (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (qualified-name "Actions::Act"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (qualified-name "Actions::Act::good"))) (target (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (qualified-name "Actions::Act"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (qualified-name "Actions::Act")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (qualified-name "Actions::Act")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (qualified-name "Actions::Act")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (qualified-name "Actions::Act::a1")))
      (featured-by (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (qualified-name "Actions::Act")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (qualified-name "Actions::Act::a2")))
      (featured-by (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (qualified-name "Actions::Act")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (qualified-name "Actions::Act::bad")))
      (featured-by (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (qualified-name "Actions::Act")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (qualified-name "Actions::Act::good")))
      (featured-by (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (qualified-name "Actions::Act")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (range (start 7 25) (end 7 29)) (probe (position 7 25))
    (reference (id (source (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "good")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (qualified-name "Actions::Act::good")))))
    )
  )
  (query (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (range (start 11 25) (end 11 28)) (probe (position 11 25))
    (reference (id (source (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 0) (authored-target "bad")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (qualified-name "Actions::Act::bad")))))
    )
  )
  (query (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (range (start 12 25) (end 12 28)) (probe (position 12 25))
    (reference (id (source (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 0) (authored-target "bad")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (qualified-name "Actions::Act::bad")))))
    )
  )
  (query (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (range (start 7 35) (end 7 37)) (probe (position 7 35))
    (reference (id (source (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "a1")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (qualified-name "Actions::Act::a1")))))
    )
  )
  (query (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (range (start 11 34) (end 11 36)) (probe (position 11 34))
    (reference (id (source (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 1) (authored-target "a1")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (qualified-name "Actions::Act::a1")))))
    )
  )
  (query (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (range (start 12 34) (end 12 36)) (probe (position 12 34))
    (reference (id (source (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 1) (authored-target "a2")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_join_node_outgoing_successions.md") (qualified-name "Actions::Act::a2")))))
    )
  )
)
~~~
