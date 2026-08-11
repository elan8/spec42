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
# EXPECTED
~~~
parse.unexpected_token
~~~
# PROBLEMS
~~~
parse.unexpected_token
~~~
# FORMAT
~~~sysml
x /* comment */ // note
y

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "6d575d6005859261ad81038f2378e4277aeb265cae8f4725455dcc00b15fee0a") (contract-version "canonical-resolution-v1"))
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
