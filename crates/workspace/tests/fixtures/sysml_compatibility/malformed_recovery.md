# META
~~~ini
description=Malformed tokens with recovery
type=file
~~~
# SOURCE
~~~sysml
x ` y
~~~
# TOKENS
~~~zig
Ident,MalformedUnknownToken,Ident,EndOfFile,
~~~
# AST
~~~
(root
  (malformed))
~~~
# FORMAT
~~~sysml
x ` y
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
