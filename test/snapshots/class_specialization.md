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
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 0 0) (end 0 16))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:70cd1d0dc28ee204383a65ad77622c9dea4ae28b3f1ad23f51ab5625c6d08f7a") (contract-version "parser-owned-resolution-v1"))
  (declarations
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
