# META
~~~ini
description=Fuzz: individual usage with direction prefix preserves 'individual' keyword
type=file
~~~
# SOURCE
~~~sysml
in individual it;
~~~
# TOKENS
~~~zig
KwIn,KwIndividual,Ident,Semicolon,EndOfFile,
~~~
# AST
~~~
(root
  (individual_usage in individual 'it'))
~~~
# FORMAT
~~~sysml
in individual it;
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
