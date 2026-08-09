# META
~~~ini
description=Wildcard import statement
type=file
semantic_graph=skip
semantic_graph_skip_reason=strictly parsed non-empty source produced no typed semantic graph facts
~~~
# SOURCE
~~~sysml
import ScalarValues::*;
~~~
# TOKENS
~~~zig
KwImport,Ident,ColonColon,Star,Semicolon,EndOfFile,
~~~
# AST
~~~
(root
  (import_decl 'ScalarValues::*'))
~~~
# FORMAT
~~~sysml
import ScalarValues::*;

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
