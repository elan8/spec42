# META
~~~ini
description=Empty input produces only EndOfFile
type=file
~~~
# SOURCE
~~~sysml
~~~
# TOKENS
~~~zig
EndOfFile,
~~~
# AST
~~~
(root)
~~~
# FORMAT
~~~sysml


~~~
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
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
  (document "empty_input.md"
    (diagnostics
    )
  )
)
~~~
