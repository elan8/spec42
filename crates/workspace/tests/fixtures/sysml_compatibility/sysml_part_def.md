# META
~~~ini
description=SysML part definition
type=file
~~~
# SOURCE
~~~sysml
part def Vehicle { }
~~~
# TOKENS
~~~zig
KwPart,KwDef,Ident,OpenCurly,CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (part_def 'Vehicle'))
~~~
# FORMAT
~~~sysml
part def Vehicle { }
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
  (status (skip (code "SMG-EMPTY-STRICT") (reason "strictly parsed non-empty source produced no typed semantic graph facts")))
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
