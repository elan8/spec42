# META
~~~ini
description=Empty member (bare semicolon) inside package body
type=file
~~~
# SOURCE
~~~sysml
package MyPkg {;}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,Semicolon,CloseCurly,EndOfFile,
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
