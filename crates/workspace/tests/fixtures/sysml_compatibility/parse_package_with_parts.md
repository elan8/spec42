# META
~~~ini
description=Package containing part definitions
type=file
~~~
# SOURCE
~~~sysml
package Vehicles {
    part def Car;
    part def Truck;
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'Vehicles'
    (part_def 'Car')
    (part_def 'Truck')))
~~~
# FORMAT
~~~sysml
package Vehicles {
    part def Car;
    part def Truck;
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
    (element (kind "package") (id (node (document "d0") (qualified-name "Vehicles"))) (name "Vehicles") (declared-name "Vehicles")
      (contains
        (element (kind "part def") (id (node (document "d0") (qualified-name "Vehicles::Car"))) (name "Car") (declared-name "Car") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Vehicles::Truck"))) (name "Truck") (declared-name "Truck") (declared))
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
