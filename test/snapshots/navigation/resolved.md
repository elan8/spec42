# META
~~~ini
description=Resolved navigation preserves target identity and range
type=file
~~~
# SOURCE
~~~sysml
package P {
    part def Engine;
    part engine : Engine;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "resolved.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package P {
    part def Engine;
    part engine : Engine;
}

~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "ecc72463ef9dbae1b5c2af0ceba2adc0b8222e2e1dcaa4e2d31a10e323f1491a") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "P"))) (kind "package") (name "P") (declared-name "P"))
    (element (id (node (document "d0") (qualified-name "P::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (parent (node (document "d0") (qualified-name "P"))))
    (element (id (node (document "d0") (qualified-name "P::engine"))) (kind "part") (name "engine") (declared-name "engine") (parent (node (document "d0") (qualified-name "P"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "P::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "P::Engine")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "P::engine"))) (target (node (document "d0") (qualified-name "P::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "P::engine"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 2 18) (end 2 24)) (probe (position 2 18))
      (reference
        (source (document "d0") (qualified-name "P::engine"))
        (kind featureTyping) (ordinal 0) (authored-target "Engine")
        (range (start 2 18) (end 2 24))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "P::Engine") (range (start 1 4) (end 1 20)))
        )
      )
    )
  )
)
~~~
