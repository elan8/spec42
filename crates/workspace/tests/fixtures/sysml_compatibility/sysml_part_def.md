# META
~~~ini
description=SysML part definition
type=file
semantic_graph=skip
semantic_graph_skip_reason=strictly parsed non-empty source produced no typed semantic graph facts
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
