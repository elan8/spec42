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
(model
  (namespace
    (package 'Vehicles'
      (part_def 'Car')
      (part_def 'Truck'))))
~~~
