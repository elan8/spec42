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
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:eec6454bb65bcec84a4e8d8b22fde487a80f86d1f2a367d8f16bee8a039e4e06"))
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
