# META
~~~ini
description=Parser recovers from unexpected tokens
type=file
~~~
# SOURCE
~~~sysml
package Foo {
    + bad stuff;
    part def Bar;
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
Plus,Ident,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'Foo'
    (malformed)
    (part_def 'Bar')))
~~~
# FORMAT
~~~sysml
package Foo {
    + bad stuff;
    part def Bar;
}

~~~
# EXPECTED
~~~
parse.unexpected_token
~~~
# PROBLEMS
~~~
parse.unexpected_token
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Foo"))) (name "Foo") (declared-name "Foo")
      (contains
        (element (kind "part def") (id (node (document "d0") (qualified-name "Foo::Bar"))) (name "Bar") (declared-name "Bar") (declared))
      )
    )
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Foo::Bar"))) (status missing-prerequisite) (target "Parts::Part"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "parse_malformed_recovery.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_package_body_element")
        (source "sysml")
        (range (start 1 4) (end 1 21))
      )
    )
  )
)
~~~
