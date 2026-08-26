# META
~~~ini
description=Empty member (bare semicolon) at file level
type=file
~~~
# SOURCE
~~~sysml
; in v : SpeedVal
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/empty_member_at_file_level.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "expected_keyword")
        (source "parser")
        (range (start 0 2) (end 0 17))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:00822b861e583c5aba03c15441e4b75ab39e4ee721a8a593dee5c678e970552a") (contract-version "owned-cross-feature-typing-v4"))
  (declarations
    (declaration (id (node (document "memory://snapshot/empty_member_at_file_level.md") (path (anonymous (kind default-reference) (ordinal 0))))) (kind default-reference) (membership (kind feature) (visibility default)))
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
