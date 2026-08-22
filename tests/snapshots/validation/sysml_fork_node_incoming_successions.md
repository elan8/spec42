# META
~~~ini
description=SysML 8.3.17.8 validateForkNodeIncomingSuccessions allows a ForkNode at most one incoming Succession
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.17.8 validateForkNodeIncomingSuccessions
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.17.8:validateForkNodeIncomingSuccessions
blocked_by=semantic-fork-node-multiple-incoming
type=file
~~~
# SOURCE
~~~sysml
package Actions {
    action def Act {
        action a1;
        action a2;

        // Conforming: one incoming succession.
        fork good;
        succession first a1 then good;

        // Invalid: two incoming successions.
        fork bad;
        succession first a1 then bad;
        succession first a2 then bad;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_fork_node_incoming_successions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "fork_node_multiple_incoming")
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
  (document "memory://snapshot/sysml_fork_node_incoming_successions.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:64d3c642d72278509eeda5560b60e646705b86083edf813cc25e2c003c720db1") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (qualified-name "Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (qualified-name "Actions::Act"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "a1")) (succession (reference "good")))))
    (declaration (id (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "a1")) (succession (reference "bad")))))
    (declaration (id (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "a2")) (succession (reference "bad")))))
    (declaration (id (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (qualified-name "Actions::Act::a1"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (qualified-name "Actions::Act::a2"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (qualified-name "Actions::Act::bad"))) (kind fork) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (qualified-name "Actions::Act::good"))) (kind fork) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "a1")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (qualified-name "Actions::Act::a1")))))
    (reference (id (source (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 0))
      (authored-target "a1")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (qualified-name "Actions::Act::a1")))))
    (reference (id (source (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 0))
      (authored-target "a2")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (qualified-name "Actions::Act::a2")))))
    (reference (id (source (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "good")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (qualified-name "Actions::Act::good")))))
    (reference (id (source (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 1))
      (authored-target "bad")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (qualified-name "Actions::Act::bad")))))
    (reference (id (source (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 1))
      (authored-target "bad")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (qualified-name "Actions::Act::bad")))))
  )
  (relationships
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (qualified-name "Actions::Act::a1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (target (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (qualified-name "Actions::Act::a1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 2))))) (target (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (qualified-name "Actions::Act::a2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (qualified-name "Actions::Act::good"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (target (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (qualified-name "Actions::Act::bad"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 1)))
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 2))))) (target (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (qualified-name "Actions::Act::bad"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 1)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (qualified-name "Actions::Act"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (target (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (qualified-name "Actions::Act"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 2))))) (target (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (qualified-name "Actions::Act"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (qualified-name "Actions::Act::a1"))) (target (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (qualified-name "Actions::Act"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (qualified-name "Actions::Act::a2"))) (target (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (qualified-name "Actions::Act"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (qualified-name "Actions::Act::bad"))) (target (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (qualified-name "Actions::Act"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (qualified-name "Actions::Act::good"))) (target (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (qualified-name "Actions::Act"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (qualified-name "Actions::Act")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (qualified-name "Actions::Act")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (qualified-name "Actions::Act")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (qualified-name "Actions::Act::a1")))
      (featured-by (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (qualified-name "Actions::Act")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (qualified-name "Actions::Act::a2")))
      (featured-by (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (qualified-name "Actions::Act")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (qualified-name "Actions::Act::bad")))
      (featured-by (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (qualified-name "Actions::Act")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (qualified-name "Actions::Act::good")))
      (featured-by (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (qualified-name "Actions::Act")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (range (start 7 25) (end 7 27)) (probe (position 7 25))
    (reference (id (source (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "a1")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (qualified-name "Actions::Act::a1")))))
    )
  )
  (query (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (range (start 11 25) (end 11 27)) (probe (position 11 25))
    (reference (id (source (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 0) (authored-target "a1")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (qualified-name "Actions::Act::a1")))))
    )
  )
  (query (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (range (start 12 25) (end 12 27)) (probe (position 12 25))
    (reference (id (source (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 0) (authored-target "a2")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (qualified-name "Actions::Act::a2")))))
    )
  )
  (query (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (range (start 7 33) (end 7 37)) (probe (position 7 33))
    (reference (id (source (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "good")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (qualified-name "Actions::Act::good")))))
    )
  )
  (query (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (range (start 11 33) (end 11 36)) (probe (position 11 33))
    (reference (id (source (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 1) (authored-target "bad")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (qualified-name "Actions::Act::bad")))))
    )
  )
  (query (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (range (start 12 33) (end 12 36)) (probe (position 12 33))
    (reference (id (source (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 2))))) (kind succession) (ordinal 1) (authored-target "bad")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_fork_node_incoming_successions.md") (qualified-name "Actions::Act::bad")))))
    )
  )
)
~~~
