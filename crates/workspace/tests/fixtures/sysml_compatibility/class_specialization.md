# META
~~~ini
description=Class with specialization
type=file
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
