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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:a5029ee6d112b6f709d8ff90b211cc52c379dd08624880a6c94288864dc9ee28") (contract-version "semantic-metadata-projection-v6"))
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
