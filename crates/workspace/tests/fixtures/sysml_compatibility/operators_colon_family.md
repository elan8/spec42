# META
~~~ini
description=Colon family operators
type=file
~~~
# SOURCE
~~~sysml
: :: :> ::> :>> :=
~~~
# TOKENS
~~~zig
Colon,ColonColon,ColonGt,ColonColonGt,ColonGtGt,ColonEq,EndOfFile,
~~~
# AST
~~~
(root
  (malformed))
~~~
# FORMAT
~~~sysml
: :: :> ::> :>> :=
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
