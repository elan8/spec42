# META
~~~ini
description=Regular comments are tokens, notes are trivia
type=file
semantic_graph=skip
semantic_graph_skip_reason=parser recovery for non-empty source produced no typed semantic graph facts
~~~
# SOURCE
~~~sysml
x /* comment */ // note
y
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "comments_and_notes.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "expected_keyword")
        (source "sysml")
        (range (start 0 0) (end 0 23))
      )
      (diagnostic
        (severity error)
        (code "expected_keyword")
        (source "sysml")
        (range (start 1 0) (end 1 1))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
x /* comment */ // note
y

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "6d3d02213e63beb0f7b7c07e57a87e5aad02a14c1bbb7890f84ac4f8c6a2abb5") (contract-version "canonical-resolution-v1"))
  (structure
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
