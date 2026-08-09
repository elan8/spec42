# META
~~~ini
description=Multiline body with newline insertion
type=file
~~~
# SOURCE
~~~sysml
package Foo {
  feature x;
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwFeature,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'Foo'
    (feature_def 'x')))
~~~
# FORMAT
~~~sysml
package Foo {
    feature x;
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
    (element (kind "package") (id (node (document "d0") (qualified-name "Foo"))) (name "Foo") (declared-name "Foo")
      (contains
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "Foo::x"))) (name "x") (declared-name "x"))
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
