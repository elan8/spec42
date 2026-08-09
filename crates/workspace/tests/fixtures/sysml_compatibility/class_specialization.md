# META
~~~ini
description=Class with specialization
type=file
semantic_graph=skip
semantic_graph_skip_reason=strictly parsed non-empty source produced no typed semantic graph facts
~~~
# SOURCE
~~~sysml
class B :> A { }
~~~
# TOKENS
~~~zig
KwClass,Ident,ColonGt,Ident,OpenCurly,CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (class_def 'B' :> 'A'))
~~~
# FORMAT
~~~sysml
class B :> A { }

~~~
# EXPECTED
~~~
semantic.unresolved_name 'A'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'A'
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
  (document "class_specialization.md"
    (diagnostics
    )
  )
)
~~~
