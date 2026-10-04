# META
~~~ini
description=Regular comments are tokens, notes are trivia
type=file
~~~
# SOURCE
~~~sysml
x /* comment */ // note
y
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/comments_and_notes.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "expected_keyword")
        (source "parser")
        (range (start 0 0) (end 0 23))
      )
      (diagnostic
        (severity error)
        (code "expected_keyword")
        (source "parser")
        (range (start 1 0) (end 1 1))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:8d7a31261c1634c5c7fbede19f139458259927afbfe9f4fc967c966353cd9c80"))
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
