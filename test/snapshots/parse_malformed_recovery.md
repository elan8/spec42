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
# EXPECTED
~~~
parse.unexpected_token
~~~
# PROBLEMS
~~~
parse.unexpected_token
~~~
# FORMAT
~~~sysml
package Foo {
    + bad stuff;
    part def Bar;
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "48ee27ed9713cb535165b5d7f95f97bffa1bb6d372c20353f1bb500cc087621b") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Foo"))) (kind "package") (name "Foo") (declared-name "Foo") (range (start (line 0) (character 0)) (end (line 0) (character 50))))
    (element (id (node (document "d0") (qualified-name "Foo::Bar"))) (kind "part def") (name "Bar") (declared-name "Bar") (range (start (line 2) (character 4)) (end (line 2) (character 17))) (parent (node (document "d0") (qualified-name "Foo"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
