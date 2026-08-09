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
(model
  (namespace
    (package 'Foo'
      (feature_def 'x'))))
~~~
