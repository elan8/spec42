# META
~~~ini
description=Incomplete part definition with unmatched braces - formatter adds compensating braces
type=file
~~~
# SOURCE
~~~sysml
package AyPkpowerTrain {
    part engine {
        g { }
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/incomplete_part_unmatched_braces.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "missing_closing_brace")
        (source "parser")
        (range (start 2 13) (end 2 13))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:0c2d67e08e6f5d3e43b5bee6f6b630ee6e80f874121b9a1595f708cd970a1473"))
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
