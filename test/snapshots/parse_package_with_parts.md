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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "parse_package_with_parts.md"
    (diagnostics
    )
  )
)
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
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# FORMAT
~~~sysml
package Vehicles {
    part def Car;
    part def Truck;
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "f1dc7d7d38e200ff1184f784fcb692f5547594cb02870501509c8e645a4866b8") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Vehicles"))) (kind "package") (name "Vehicles") (declared-name "Vehicles") (range (start (line 0) (character 0)) (end (line 0) (character 58))))
    (element (id (node (document "d0") (qualified-name "Vehicles::Car"))) (kind "part def") (name "Car") (declared-name "Car") (range (start (line 1) (character 4)) (end (line 1) (character 17))) (parent (node (document "d0") (qualified-name "Vehicles"))))
    (element (id (node (document "d0") (qualified-name "Vehicles::Truck"))) (kind "part def") (name "Truck") (declared-name "Truck") (range (start (line 2) (character 4)) (end (line 2) (character 19))) (parent (node (document "d0") (qualified-name "Vehicles"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
