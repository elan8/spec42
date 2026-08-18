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
      (diagnostic
        (severity error)
        (code "expected_keyword")
        (source "parser")
        (range (start 0 0) (end 0 17))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:a5029ee6d112b6f709d8ff90b211cc52c379dd08624880a6c94288864dc9ee28") (contract-version "parser-owned-resolution-v1"))
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
