# META
~~~ini
description=Generic flow builder endpoints resolve canonically
type=file
~~~
# SOURCE
~~~sysml
package P { action def ExecuteMission { action validateRoute; action startMission; first validateRoute then startMission; } }
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/generic_flow_publication.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:32b487b10690d370ed6ee068eba8bec0452680e6072ad314b3187c85ca1644b1") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/generic_flow_publication.md") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generic_flow_publication.md") (qualified-name "P::ExecuteMission"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generic_flow_publication.md") (path (named (kind package) (name "P")) (named (kind action-def) (name "ExecuteMission")) (anonymous (kind succession) (ordinal 0)))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "validateRoute")) (succession (reference "startMission"))))
    (declaration (id (node (document "memory://snapshot/generic_flow_publication.md") (qualified-name "P::ExecuteMission::startMission"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generic_flow_publication.md") (qualified-name "P::ExecuteMission::validateRoute"))) (kind action) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/generic_flow_publication.md") (path (named (kind package) (name "P")) (named (kind action-def) (name "ExecuteMission")) (anonymous (kind succession) (ordinal 0)))))) (kind succession) (ordinal 0))
      (authored-target "validateRoute")
      (outcome (status resolved) (target (node (document "memory://snapshot/generic_flow_publication.md") (qualified-name "P::ExecuteMission::validateRoute")))))
    (reference (id (source (node (document "memory://snapshot/generic_flow_publication.md") (path (named (kind package) (name "P")) (named (kind action-def) (name "ExecuteMission")) (anonymous (kind succession) (ordinal 0)))))) (kind succession) (ordinal 1))
      (authored-target "startMission")
      (outcome (status resolved) (target (node (document "memory://snapshot/generic_flow_publication.md") (qualified-name "P::ExecuteMission::startMission")))))
  )
  (relationships
    (relationship (kind succession) (source (node (document "memory://snapshot/generic_flow_publication.md") (path (named (kind package) (name "P")) (named (kind action-def) (name "ExecuteMission")) (anonymous (kind succession) (ordinal 0)))))) (target (node (document "memory://snapshot/generic_flow_publication.md") (qualified-name "P::ExecuteMission::validateRoute"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generic_flow_publication.md") (path (named (kind package) (name "P")) (named (kind action-def) (name "ExecuteMission")) (anonymous (kind succession) (ordinal 0)))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/generic_flow_publication.md") (path (named (kind package) (name "P")) (named (kind action-def) (name "ExecuteMission")) (anonymous (kind succession) (ordinal 0)))))) (target (node (document "memory://snapshot/generic_flow_publication.md") (qualified-name "P::ExecuteMission::startMission"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generic_flow_publication.md") (path (named (kind package) (name "P")) (named (kind action-def) (name "ExecuteMission")) (anonymous (kind succession) (ordinal 0)))))) (kind succession) (ordinal 1)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/generic_flow_publication.md") (range (start 0 89) (end 0 102)) (probe (position 0 89))
    (reference (id (source (node (document "memory://snapshot/generic_flow_publication.md") (path (named (kind package) (name "P")) (named (kind action-def) (name "ExecuteMission")) (anonymous (kind succession) (ordinal 0)))))) (kind succession) (ordinal 0) (authored-target "validateRoute")
      (outcome (status resolved) (target (node (document "memory://snapshot/generic_flow_publication.md") (qualified-name "P::ExecuteMission::validateRoute")))))
  )
  (query (document "memory://snapshot/generic_flow_publication.md") (range (start 0 108) (end 0 120)) (probe (position 0 108))
    (reference (id (source (node (document "memory://snapshot/generic_flow_publication.md") (path (named (kind package) (name "P")) (named (kind action-def) (name "ExecuteMission")) (anonymous (kind succession) (ordinal 0)))))) (kind succession) (ordinal 1) (authored-target "startMission")
      (outcome (status resolved) (target (node (document "memory://snapshot/generic_flow_publication.md") (qualified-name "P::ExecuteMission::startMission")))))
  )
)
~~~
