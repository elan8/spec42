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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "parse_part_def_with_body.md"
    (diagnostics
    )
  )
)
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
part def Vehicle {
    part def Engine;
    part def Wheel;
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "bc702f31b0368fc84f99bd2fe7f8374f228aab87c7825b457e3bb3c64f6aaa92") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 0) (character 0)) (end (line 0) (character 61))))
    (element (id (node (document "d0") (qualified-name "Vehicle::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (range (start (line 1) (character 4)) (end (line 1) (character 20))) (parent (node (document "d0") (qualified-name "Vehicle"))))
    (element (id (node (document "d0") (qualified-name "Vehicle::Wheel"))) (kind "part def") (name "Wheel") (declared-name "Wheel") (range (start (line 2) (character 4)) (end (line 2) (character 19))) (parent (node (document "d0") (qualified-name "Vehicle"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
