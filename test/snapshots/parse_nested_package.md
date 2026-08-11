# META
~~~ini
description=Nested package definitions
type=file
~~~
# SOURCE
~~~sysml
package Outer {
    package Inner { }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "parse_nested_package.md"
    (diagnostics
    )
  )
)
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPackage,Ident,OpenCurly,CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'Outer'
    (package_def 'Inner')))
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
package Outer {
    package Inner { }
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "a2627b9f3d8683569cafa05640614e835f0da1eb6ec8aa85c527ae3e46eafa23") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Outer"))) (kind "package") (name "Outer") (declared-name "Outer") (range (start (line 0) (character 0)) (end (line 0) (character 39))))
    (element (id (node (document "d0") (qualified-name "Outer::Inner"))) (kind "package") (name "Inner") (declared-name "Inner") (range (start (line 1) (character 4)) (end (line 1) (character 21))) (parent (node (document "d0") (qualified-name "Outer"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
