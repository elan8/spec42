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
