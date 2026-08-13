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
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 0 12) (end 0 123))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:32b487b10690d370ed6ee068eba8bec0452680e6072ad314b3187c85ca1644b1") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/generic_flow_publication.md") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
