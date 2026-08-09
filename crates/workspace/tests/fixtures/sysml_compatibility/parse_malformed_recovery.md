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
(model
  (namespace
    (package 'Foo'
      (not_implemented 'malformed')
      (part_def 'Bar'))))
~~~
