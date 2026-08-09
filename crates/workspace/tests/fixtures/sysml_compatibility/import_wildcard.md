# META
~~~ini
description=Wildcard import statement
type=file
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
    (element (kind "import") (id (node (document "d0") (qualified-name "*"))) (name "*") (declared-name "*"))
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
