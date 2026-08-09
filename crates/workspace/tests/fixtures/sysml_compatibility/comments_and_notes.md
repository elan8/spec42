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
# TOKENS
~~~zig
Ident,RegularComment,LineComment,
Ident,EndOfFile,
~~~
# AST
~~~
(root
  (malformed))
~~~
# FORMAT
~~~sysml
x /* comment */ // note
y

~~~
# EXPECTED
~~~
parse.unexpected_token
~~~
# PROBLEMS
~~~
parse.unexpected_token
~~~
# SMG
~~~
(semantic-graph
  (containment
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
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
