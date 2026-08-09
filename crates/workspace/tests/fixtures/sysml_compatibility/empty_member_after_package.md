# META
~~~ini
description=Empty member (bare semicolon) at file level after package
type=file
~~~
# SOURCE
~~~sysml
package MyPkg { }; in newX : Real;
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,CloseCurly,Semicolon,KwIn,Ident,Colon,Ident,Semicolon,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'MyPkg')
  (default_ref_usage in 'newX' : 'Real'))
~~~
# FORMAT
~~~sysml
package MyPkg { }
in newX : Real;
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Real'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Real'
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
