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
  (status (skip (code "SMG-EMPTY-RECOVERY") (reason "parser recovery for non-empty source produced no typed semantic graph facts")))
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
