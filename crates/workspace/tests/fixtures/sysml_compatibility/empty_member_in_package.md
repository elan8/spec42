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
package MyPkg {;}

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
    (element (kind "package") (id (node (document "d0") (qualified-name "MyPkg"))) (name "MyPkg") (declared-name "MyPkg"))
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
