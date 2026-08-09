# META
~~~ini
description=Part def with nested part defs
type=file
~~~
# SOURCE
~~~sysml
part def Vehicle {
    part def Engine;
    part def Wheel;
}
~~~
# TOKENS
~~~zig
KwPart,KwDef,Ident,OpenCurly,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (part_def 'Vehicle'
    (part_def 'Engine')
    (part_def 'Wheel')))
~~~
# FORMAT
~~~sysml
part def Vehicle {
    part def Engine;
    part def Wheel;
}
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
