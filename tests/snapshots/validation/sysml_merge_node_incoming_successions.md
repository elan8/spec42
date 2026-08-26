# META
~~~ini
description=SysML 8.3.17.13 validateMergeNodeIncomingSuccessions requires all incoming Successions to a MergeNode to have a source multiplicity of 0..1
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.17.13 validateMergeNodeIncomingSuccessions
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.17.13:validateMergeNodeIncomingSuccessions
blocked_by=semantic-merge-node-incoming-multiplicity
type=file
~~~
# SOURCE
~~~sysml
package Actions {
    action def Act {
        action a1;
        merge m;

        // Conforming: the incoming succession has a 0..1 source multiplicity.
        succession first [0..1] a1 then m;

        // Invalid: the incoming succession widens its source multiplicity.
        succession first [0..*] a1 then m;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_merge_node_incoming_successions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "merge_node_incoming_multiplicity")
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
  (document "memory://snapshot/sysml_merge_node_incoming_successions.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:565aa1164fec4959fadf22ff3b45222f02a959bca9203968c4a5771a42bc43ff") (contract-version "owned-cross-feature-typing-v4"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (qualified-name "Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (qualified-name "Actions::Act"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "a1")) (succession (reference "m")))))
    (declaration (id (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "a1")) (succession (reference "m")))))
    (declaration (id (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (qualified-name "Actions::Act::a1"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (qualified-name "Actions::Act::m"))) (kind merge) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "a1")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (qualified-name "Actions::Act::a1")))))
    (reference (id (source (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 0))
      (authored-target "a1")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (qualified-name "Actions::Act::a1")))))
    (reference (id (source (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "m")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (qualified-name "Actions::Act::m")))))
    (reference (id (source (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 1))
      (authored-target "m")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (qualified-name "Actions::Act::m")))))
  )
  (relationships
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (qualified-name "Actions::Act::a1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (target (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (qualified-name "Actions::Act::a1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (qualified-name "Actions::Act::m"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (target (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (qualified-name "Actions::Act::m"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 1)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (qualified-name "Actions::Act"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (target (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (qualified-name "Actions::Act"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (qualified-name "Actions::Act::a1"))) (target (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (qualified-name "Actions::Act"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (qualified-name "Actions::Act::m"))) (target (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (qualified-name "Actions::Act"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (qualified-name "Actions::Act")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (qualified-name "Actions::Act")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (qualified-name "Actions::Act::a1")))
      (featured-by (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (qualified-name "Actions::Act")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (qualified-name "Actions::Act::m")))
      (featured-by (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (qualified-name "Actions::Act")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (range (start 6 32) (end 6 34)) (probe (position 6 32))
    (reference (id (source (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "a1")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (qualified-name "Actions::Act::a1")))))
    )
  )
  (query (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (range (start 9 32) (end 9 34)) (probe (position 9 32))
    (reference (id (source (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 0) (authored-target "a1")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (qualified-name "Actions::Act::a1")))))
    )
  )
  (query (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (range (start 6 40) (end 6 41)) (probe (position 6 40))
    (reference (id (source (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "m")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (qualified-name "Actions::Act::m")))))
    )
  )
  (query (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (range (start 9 40) (end 9 41)) (probe (position 9 40))
    (reference (id (source (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind succession) (ordinal 1))))) (kind succession) (ordinal 1) (authored-target "m")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_merge_node_incoming_successions.md") (qualified-name "Actions::Act::m")))))
    )
  )
)
~~~
