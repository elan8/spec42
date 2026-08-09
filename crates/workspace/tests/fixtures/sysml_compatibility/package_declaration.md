# META
~~~ini
description=Simple package declaration
type=file
~~~
# SOURCE
~~~sysml
package MyPkg { }
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'MyPkg'))
~~~
# FORMAT
~~~sysml
package MyPkg { }
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
    (package 'MyPkg')))
~~~
