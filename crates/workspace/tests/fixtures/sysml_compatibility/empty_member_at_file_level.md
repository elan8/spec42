# META
~~~ini
description=Empty member (bare semicolon) at file level
type=file
semantic_graph=skip
semantic_graph_skip_reason=parser recovery for non-empty source produced no typed semantic graph facts
~~~
# SOURCE
~~~sysml
; in v : SpeedVal
~~~
# TOKENS
~~~zig
Semicolon,KwIn,Ident,Colon,Ident,EndOfFile,
~~~
# AST
~~~
(root
  (malformed))
~~~
# FORMAT
~~~sysml
; in v : SpeedVal

~~~
# EXPECTED
~~~
parse.expected_semicolon_or_body
~~~
# PROBLEMS
~~~
parse.expected_semicolon_or_body
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
