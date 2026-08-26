# META
~~~ini
description=Colon family operators
type=file
~~~
# SOURCE
~~~sysml
: :: :> ::> :>> :=
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/operators_colon_family.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "expected_keyword")
        (source "parser")
        (range (start 0 0) (end 0 18))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:a03e10e527fd0adf56d7070a263c35541c810bcde8c7b5ff70906c8e395635d5") (contract-version "owned-cross-feature-typing-v4"))
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
