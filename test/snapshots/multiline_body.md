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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "multiline_body.md"
    (diagnostics
    )
  )
)
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
package Foo {
    feature x;
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "7a9f76c1b8cc49135603b67be876abcbfe412e3a8f83a8bc96c9e06927e135c5") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Foo"))) (kind "package") (name "Foo") (declared-name "Foo") (range (start (line 0) (character 0)) (end (line 0) (character 28))))
    (element (id (node (document "d0") (qualified-name "Foo::x"))) (kind "feature decl") (name "x") (declared-name "x") (range (start (line 1) (character 2)) (end (line 1) (character 12))) (parent (node (document "d0") (qualified-name "Foo"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
