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
# FORMAT
~~~sysml
package Outer {
    package Inner { }
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
    (element (kind "package") (id (node (document "d0") (qualified-name "Outer"))) (name "Outer") (declared-name "Outer")
      (contains
        (element (kind "package") (id (node (document "d0") (qualified-name "Outer::Inner"))) (name "Inner") (declared-name "Inner"))
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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "parse_nested_package.md"
    (diagnostics
    )
  )
)
~~~
