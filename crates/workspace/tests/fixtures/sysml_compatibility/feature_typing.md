# META
~~~ini
description=Feature with type annotation
type=file
~~~
# SOURCE
~~~sysml
feature x : Integer;
~~~
# TOKENS
~~~zig
KwFeature,Ident,Colon,Ident,Semicolon,EndOfFile,
~~~
# AST
~~~
(root
  (feature_def 'x' : 'Integer'))
~~~
# FORMAT
~~~sysml
feature x : Integer;
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Integer'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Integer'
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
