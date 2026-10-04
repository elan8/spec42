# META
~~~ini
description=Unclosed comment (missing */) at file level should be preserved
type=file
~~~
# SOURCE
~~~sysml
/* unclosed comment without closing marker
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/unclosed_comment_at_file_level.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "expected_keyword")
        (source "parser")
        (range (start 0 0) (end 0 40))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:f37272cd2b57984d8f3940f5bf2acc36691c0e7c7636b2f882eb6f97deb87a74"))
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
