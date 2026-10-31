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
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 1 4) (end 2 13))
      )
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
    (declaration (id (node (document "memory://snapshot/incomplete_part_unmatched_braces.md") (qualified-name "AyPkpowerTrain"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/incomplete_part_unmatched_braces.md") (qualified-name "AyPkpowerTrain::engine"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/incomplete_part_unmatched_braces.md") (qualified-name "AyPkpowerTrain::engine::g"))) (kind default-reference) (membership (kind feature) (visibility default)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/incomplete_part_unmatched_braces.md") (qualified-name "AyPkpowerTrain::engine::g"))) (target (node (document "memory://snapshot/incomplete_part_unmatched_braces.md") (qualified-name "AyPkpowerTrain::engine"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/incomplete_part_unmatched_braces.md") (qualified-name "AyPkpowerTrain::engine::g")))
      (featured-by (node (document "memory://snapshot/incomplete_part_unmatched_braces.md") (qualified-name "AyPkpowerTrain::engine")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
