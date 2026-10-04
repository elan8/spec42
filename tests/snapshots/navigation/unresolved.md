# META
~~~ini
description=Unresolved navigation remains an explicit outcome
type=file
~~~
# SOURCE
~~~sysml
package P {
    part engine : MissingEngine;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/unresolved.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1 18) (end 1 31))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:fdb055c6ed43dc036f9d2d0725e264c3c07d15ac7aa0390926cc34976c806dce"))
  (declarations
    (declaration (id (node (document "memory://snapshot/unresolved.md") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/unresolved.md") (qualified-name "P::engine"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MissingEngine")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/unresolved.md") (qualified-name "P::engine"))) (kind featureTyping) (ordinal 0))
      (authored-target "MissingEngine")
      (outcome (status unresolved)))
  )
  (relationships
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
  (query (document "memory://snapshot/unresolved.md") (range (start 1 18) (end 1 31)) (probe (position 1 18))
    (reference (id (source (node (document "memory://snapshot/unresolved.md") (qualified-name "P::engine"))) (kind featureTyping) (ordinal 0) (authored-target "MissingEngine")
      (outcome (status unresolved)))
    )
  )
)
~~~
