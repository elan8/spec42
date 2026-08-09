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
(model
  (namespace
    (package 'Outer'
      (package 'Inner'))))
~~~
