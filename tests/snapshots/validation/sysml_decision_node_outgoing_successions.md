# META
~~~ini
description=SysML 8.3.17.7 validateDecisionNodeOutgoingSuccessions requires all outgoing Successions from a DecisionNode to have a target multiplicity of 0..1
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.17.7 validateDecisionNodeOutgoingSuccessions
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.17.7:validateDecisionNodeOutgoingSuccessions
blocked_by=semantic-decision-node-outgoing-multiplicity
type=file
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
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:8b52f5b26e279816bff64ac4f74e2f3fb94d030d00a75a285afb807953c873ab") (contract-version "operator-expression-arguments-v7"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (qualified-name "Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (qualified-name "Actions::Act"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "d")) (succession (reference "a1")))))
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "d")) (succession (reference "a1")))))
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (qualified-name "Actions::Act::a1"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (qualified-name "Actions::Act::d"))) (kind decide) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "d")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (qualified-name "Actions::Act::d")))))
    (reference (id (source (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 0))
      (authored-target "d")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (qualified-name "Actions::Act::d")))))
    (reference (id (source (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "a1")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (qualified-name "Actions::Act::a1")))))
    (reference (id (source (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 1))
      (authored-target "a1")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (qualified-name "Actions::Act::a1")))))
  )
  (relationships
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (qualified-name "Actions::Act::d"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (target (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (qualified-name "Actions::Act::d"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (qualified-name "Actions::Act::a1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (target (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (qualified-name "Actions::Act::a1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 1)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (qualified-name "Actions::Act"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (target (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (qualified-name "Actions::Act"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (qualified-name "Actions::Act::a1"))) (target (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (qualified-name "Actions::Act"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (qualified-name "Actions::Act::d"))) (target (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (qualified-name "Actions::Act"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (qualified-name "Actions::Act")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (qualified-name "Actions::Act")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (qualified-name "Actions::Act::a1")))
      (featured-by (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (qualified-name "Actions::Act")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (qualified-name "Actions::Act::d")))
      (featured-by (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (qualified-name "Actions::Act")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (range (start 6 25) (end 6 26)) (probe (position 6 25))
    (reference (id (source (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "d")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (qualified-name "Actions::Act::d")))))
    )
  )
  (query (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (range (start 9 25) (end 9 26)) (probe (position 9 25))
    (reference (id (source (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 0) (authored-target "d")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_decision_node_outgoing_successions.md") (qualified-name "Actions::Act::d")))))
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
)
~~~
