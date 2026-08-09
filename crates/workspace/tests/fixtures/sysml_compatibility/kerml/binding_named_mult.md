# META
~~~ini
description=KerML binding connector with named form + multiplicity + 'of' disambiguation
type=file
~~~
# SOURCE
~~~kerml
package BindingNamedMult {
    binding instant[instantNum] of startShot = endShot;
    binding all startShot = endShot;
    binding x bind a = b;
    binding [0..1] a = b;
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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwBinding,Ident,OpenSquare,Ident,CloseSquare,KwOf,Ident,Eq,Ident,Semicolon,
KwBinding,KwAll,Ident,Eq,Ident,Semicolon,
KwBinding,Ident,KwBind,Ident,Eq,Ident,Semicolon,
KwBinding,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Eq,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'BindingNamedMult'
    (binding_connector 'instant' multiplicity
      (connector_end)
      (connector_end))
    (binding_connector
      (connector_end)
      (connector_end))
    (binding_connector 'x'
      (connector_end)
      (connector_end))
    (binding_connector multiplicity
      (connector_end)
      (connector_end))))
~~~
# FORMAT
~~~sysml
package BindingNamedMult {
    binding instant [instantNum] of startShot = endShot;
    binding all startShot = endShot;
    binding x bind a = b;
    binding [0..1] a = b;
}
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "BindingNamedMult"))) (name "BindingNamedMult") (declared-name "BindingNamedMult"))
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
