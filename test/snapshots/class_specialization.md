# META
~~~ini
description=Class with specialization
type=file
~~~
# SOURCE
~~~sysml
class B :> A { }
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/class_specialization.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 0 11) (end 0 12))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:70cd1d0dc28ee204383a65ad77622c9dea4ae28b3f1ad23f51ab5625c6d08f7a") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/class_specialization.md") (qualified-name "B"))) (kind class-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "A"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/class_specialization.md") (qualified-name "B"))) (kind specialization) (ordinal 0))
      (authored-target "A")
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
  (query (document "memory://snapshot/class_specialization.md") (range (start 0 11) (end 0 12)) (probe (position 0 11))
    (reference (id (source (node (document "memory://snapshot/class_specialization.md") (qualified-name "B"))) (kind specialization) (ordinal 0) (authored-target "A")
      (outcome (status unresolved)))
  )
)
~~~
