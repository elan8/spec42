# META
~~~ini
description=Fuzz: binding connector formats name before multiplicity
type=file
~~~
# SOURCE
~~~sysml
package P {
    binding b [5] of a = c;
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwBinding,Ident,OpenSquare,DecimalValue,CloseSquare,KwOf,Ident,Eq,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'P'
    (binding_connector 'b' multiplicity
      (connector_end)
      (connector_end))))
~~~
# FORMAT
~~~sysml
package P {
    binding b [5] of a = c;
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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "P"))) (name "P") (declared-name "P"))
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
