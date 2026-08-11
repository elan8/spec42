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
  (document "generic_flow_publication.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "f42e20b2d294c93778f4643d001d842a4d14d6674e3208f60bd49827a3500cde") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "P"))) (kind "package") (name "P") (declared-name "P"))
    (element (id (node (document "d0") (qualified-name "P::ExecuteMission"))) (kind "action def") (name "ExecuteMission") (declared-name "ExecuteMission") (parent (node (document "d0") (qualified-name "P"))) (authored (membership (kind Owning)) (relationships (perform (reference "P::ExecuteMission::validateRoute")) (perform (reference "P::ExecuteMission::startMission")))))
    (element (id (node (document "d0") (qualified-name "P::ExecuteMission::startMission"))) (kind "action") (name "startMission") (declared-name "startMission") (parent (node (document "d0") (qualified-name "P::ExecuteMission"))))
    (element (id (node (document "d0") (qualified-name "P::ExecuteMission::validateRoute"))) (kind "action") (name "validateRoute") (declared-name "validateRoute") (parent (node (document "d0") (qualified-name "P::ExecuteMission"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "P::ExecuteMission"))) (kind flowSource) (ordinal 0)) (authored-target "validateRoute") (outcome (status resolved) (target (node (document "d0") (qualified-name "P::ExecuteMission::validateRoute")))))
    (reference (id (source (node (document "d0") (qualified-name "P::ExecuteMission"))) (kind flowTarget) (ordinal 0)) (authored-target "startMission") (outcome (status resolved) (target (node (document "d0") (qualified-name "P::ExecuteMission::startMission")))))
    (reference (id (source (node (document "d0") (qualified-name "P::ExecuteMission"))) (kind performSource) (ordinal 0)) (authored-target "P::ExecuteMission::validateRoute") (outcome (status resolved) (target (node (document "d0") (qualified-name "P::ExecuteMission::validateRoute")))))
    (reference (id (source (node (document "d0") (qualified-name "P::ExecuteMission"))) (kind performSource) (ordinal 1)) (authored-target "P::ExecuteMission::startMission") (outcome (status resolved) (target (node (document "d0") (qualified-name "P::ExecuteMission::startMission")))))
  )
  (relationships
    (relationship (kind perform) (source (node (document "d0") (qualified-name "P::ExecuteMission"))) (target (node (document "d0") (qualified-name "P::ExecuteMission::startMission"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "P::ExecuteMission"))) (kind performSource) (ordinal 1)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "P::ExecuteMission"))) (target (node (document "d0") (qualified-name "P::ExecuteMission::validateRoute"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "P::ExecuteMission"))) (kind performSource) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "P::ExecuteMission::validateRoute"))) (target (node (document "d0") (qualified-name "P::ExecuteMission::startMission"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "P::ExecuteMission"))) (kind flowSource) (ordinal 0)) (expression (kind flow) (source "validateRoute") (target "startMission")))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 0 108) (end 0 120)) (probe (position 0 108))
      (reference
        (source (document "d0") (qualified-name "P::ExecuteMission"))
        (kind flowTarget) (ordinal 0) (authored-target "startMission")
        (range (start 0 108) (end 0 120))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "P::ExecuteMission::startMission") (range (start 0 62) (end 0 82)))
        )
      )
    )
    (query (range (start 0 89) (end 0 102)) (probe (position 0 89))
      (reference
        (source (document "d0") (qualified-name "P::ExecuteMission"))
        (kind flowSource) (ordinal 0) (authored-target "validateRoute")
        (range (start 0 89) (end 0 102))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "P::ExecuteMission::validateRoute") (range (start 0 40) (end 0 61)))
        )
      )
    )
  )
)
~~~
