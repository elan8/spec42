# META
~~~ini
description=KerML Binding Connector: bind keyword and per-end multiplicities
type=file
~~~
# SOURCE
~~~kerml
package P {
    class C {
        feature x;
        feature y;
        feature startShot;
        feature endShot;
        feature baseEdges;

        binding [1] bind [0..*] x = [0..*] y;
        binding b bind lhs = rhs;
    }
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
KwClass,Ident,OpenCurly,
KwFeature,Ident,Semicolon,
KwFeature,Ident,Semicolon,
KwFeature,Ident,Semicolon,
KwFeature,Ident,Semicolon,
KwFeature,Ident,Semicolon,
KwBinding,OpenSquare,DecimalValue,CloseSquare,KwBind,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Ident,Eq,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Ident,Semicolon,
KwBinding,Ident,KwBind,Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'P'
    (class_def 'C'
      (feature_def 'x')
      (feature_def 'y')
      (feature_def 'startShot')
      (feature_def 'endShot')
      (feature_def 'baseEdges')
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (binding_connector 'b'
        (connector_end)
        (connector_end)))))
~~~
# FORMAT
~~~sysml
package P {
    class C {
        feature x;
        feature y;
        feature startShot;
        feature endShot;
        feature baseEdges;

        binding [1] bind [0..*] x = [0..*] y;
        binding b bind lhs = rhs;
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (package 'P'
      (class_def 'C'
        (feature_def 'x')
        (feature_def 'y')
        (feature_def 'startShot')
        (feature_def 'endShot')
        (feature_def 'baseEdges')
        (binding_connector_def
          (multiplicity_range [1])
          (connector_end 'x')
          (connector_end 'y'))
        (binding_connector_def 'b'
          (connector_end 'lhs')
          (connector_end 'rhs'))))))
~~~
