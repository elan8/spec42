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
  (containment
    (element (kind "part def") (id (node (document "d0") (qualified-name "Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared))
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
  (document "sysml_part_def.md"
    (diagnostics
    )
  )
)
~~~
