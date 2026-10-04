# META
~~~ini
description=Fuzz: individual usage with direction prefix preserves 'individual' keyword
type=file
~~~
# SOURCE
~~~sysml
in individual it;
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/fuzz_individual_direction_prefix.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:84b742e6c699734ef32aed6e83100530289525064ca9ce18ade42d8050aab112"))
  (declarations
    (declaration (id (node (document "memory://snapshot/fuzz_individual_direction_prefix.md") (qualified-name "it"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers individual) (direction in)))
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
