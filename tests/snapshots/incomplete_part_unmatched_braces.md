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
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:df2a2f953c2cc0a7983df848811ace14f5e5c2ecbcd71859cf86770ba9d1b74d") (contract-version "constructor-expression-specialization-v9"))
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
