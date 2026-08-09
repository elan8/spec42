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
  (containment
    (element (kind "part def") (id (node (document "d0") (qualified-name "Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared)
      (contains
        (element (kind "part def") (id (node (document "d0") (qualified-name "Vehicle::Engine"))) (name "Engine") (declared-name "Engine") (declared) (effective (featuring-type (node (document "d0") (qualified-name "Vehicle")))))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Vehicle::Wheel"))) (name "Wheel") (declared-name "Wheel") (declared) (effective (featuring-type (node (document "d0") (qualified-name "Vehicle")))))
      )
    )
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
