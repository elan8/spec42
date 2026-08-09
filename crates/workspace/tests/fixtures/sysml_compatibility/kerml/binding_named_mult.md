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
(model
  (namespace
    (package 'BindingNamedMult'
      (binding_connector_def 'instant'
        (multiplicity_range [?])
        (connector_end 'startShot')
        (connector_end 'endShot'))
      (binding_connector_def
        (connector_end 'startShot')
        (connector_end 'endShot'))
      (binding_connector_def 'x'
        (connector_end 'a')
        (connector_end 'b'))
      (binding_connector_def
        (multiplicity_range [0..1])
        (connector_end 'a')
        (connector_end 'b')))))
~~~
