# META
~~~ini
description=Malformed tokens with recovery
type=file
~~~
# SOURCE
~~~sysml
x ` y
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/malformed_recovery.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "expected_keyword")
        (source "parser")
        (range (start 0 0) (end 0 5))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:6abdd935816cff6e45b2889be28e6ba45df55264a66614e07bc755a4c4fae025") (contract-version "feature-value-expression-results-v5"))
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
# TYPES
~~~sexpr
(types
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
