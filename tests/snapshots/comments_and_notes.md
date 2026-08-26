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
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:9a85d59eaeaeaad8c336704fb0e44441e1bbf6b336b45038c23551f9b5bdbabc") (contract-version "owned-cross-feature-typing-v4"))
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
