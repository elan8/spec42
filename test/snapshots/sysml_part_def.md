# META
~~~ini
description=SysML part definition
type=file
~~~
# SOURCE
~~~sysml
part def Vehicle { }
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
# TOKENS
~~~zig
KwPart,KwDef,Ident,OpenCurly,CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (part_def 'Vehicle'))
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
part def Vehicle { }

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "634454bd1dbce223013fe98c776a6702b55269ecc998bf5020039d0378c6b291") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 0) (character 0)) (end (line 0) (character 20))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
