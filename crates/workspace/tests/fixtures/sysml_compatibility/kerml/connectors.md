# META
~~~ini
description=KerML Simple Tests: Connectors
type=file
~~~
# SOURCE
~~~kerml
package Connectors {
	
	class A {
		feature a : A;
		feature b : A;
		
		connector c1 from a to b;
		abstract connector c2 = c1;
		connector = c2 {
			end feature references a;
			end feature references b;
		}
		
		binding a = b;
		binding ab of a = b;
		binding {
			end feature references a;
			end feature references b;
		}
		
		succession a then b;
		succession s first a then b;
		succession {
			end feature references a;
			end feature references b;
		}
	}
	
	class B {
	    feature a : A;	    
	    connector :> a.c1 from a.a to a.b;
	}
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwClass,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,Semicolon,
KwConnector,Ident,KwFrom,Ident,KwTo,Ident,Semicolon,
KwAbstract,KwConnector,Ident,Eq,Ident,Semicolon,
KwConnector,Eq,Ident,OpenCurly,
KwEnd,KwFeature,KwReferences,Ident,Semicolon,
KwEnd,KwFeature,KwReferences,Ident,Semicolon,
CloseCurly,
KwBinding,Ident,Eq,Ident,Semicolon,
KwBinding,Ident,KwOf,Ident,Eq,Ident,Semicolon,
KwBinding,OpenCurly,
KwEnd,KwFeature,KwReferences,Ident,Semicolon,
KwEnd,KwFeature,KwReferences,Ident,Semicolon,
CloseCurly,
KwSuccession,Ident,KwThen,Ident,Semicolon,
KwSuccession,Ident,KwFirst,Ident,KwThen,Ident,Semicolon,
KwSuccession,OpenCurly,
KwEnd,KwFeature,KwReferences,Ident,Semicolon,
KwEnd,KwFeature,KwReferences,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwClass,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,Semicolon,
KwConnector,ColonGt,Ident,Dot,Ident,KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'Connectors'
    (class_def 'A'
      (feature_def 'a' : 'A')
      (feature_def 'b' : 'A')
      (connector_def 'c1'
        (connector_end)
        (connector_end))
      (connector_def 'c2' value)
      (connector_def value
        (feature_def end references 'a')
        (feature_def end references 'b'))
      (binding_connector
        (connector_end)
        (connector_end))
      (binding_connector 'ab'
        (connector_end)
        (connector_end))
      (binding_connector
        (feature_def end references 'a')
        (feature_def end references 'b'))
      (succession_def
        (connector_end)
        (connector_end))
      (succession_def 's'
        (connector_end)
        (connector_end))
      (succession_def
        (feature_def end references 'a')
        (feature_def end references 'b')))
    (class_def 'B'
      (feature_def 'a' : 'A')
      (connector_def :> 'a.c1'
        (connector_end)
        (connector_end)))))
~~~
# FORMAT
~~~sysml
package Connectors {
    class A {
        feature a : A;
        feature b : A;

        connector c1 from a to b;
        abstract connector c2 = c1;
        connector = c2 {
            end feature references a;
            end feature references b;
        }

        binding a = b;
        binding ab of a = b;
        binding {
            end feature references a;
            end feature references b;
        }

        succession a then b;
        succession s first a then b;
        succession {
            end feature references a;
            end feature references b;
        }
    }

    class B {
        feature a : A;
        connector :> a.c1 from a.a to a.b;
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
# SMG
~~~
(model
  (namespace
    (package 'Connectors'
      (class_def 'A'
        (feature_def 'a' : 'Connectors::A'[class_def])
        (feature_def 'b' : 'Connectors::A'[class_def])
        (connector_def 'c1'
          (connector_end 'a')
          (connector_end 'b'))
        (connector_def abstract 'c2'
          (feature_value (=)))
        (connector_def
          (feature_value (=))
          (feature_def end :> 'Connectors::A::a'[feature_def])
          (feature_def end :> 'Connectors::A::b'[feature_def]))
        (binding_connector_def
          (connector_end 'a')
          (connector_end 'b'))
        (binding_connector_def 'ab'
          (connector_end 'a')
          (connector_end 'b'))
        (binding_connector_def
          (feature_def end :> 'Connectors::A::a'[feature_def])
          (feature_def end :> 'Connectors::A::b'[feature_def]))
        (succession_def
          (connector_end 'a')
          (connector_end 'b'))
        (succession_def 's'
          (connector_end 'a')
          (connector_end 'b'))
        (succession_def
          (feature_def end :> 'Connectors::A::a'[feature_def])
          (feature_def end :> 'Connectors::A::b'[feature_def])))
      (class_def 'B'
        (feature_def 'a' : 'Connectors::A'[class_def])
        (connector_def :> 'Connectors::A::c1'[connector_def]
          (connector_end 'a.a' :>> 'Connectors::A::c1::a'[connector_end][implied])
          (connector_end 'a.b' :>> 'Connectors::A::c1::b'[connector_end][implied]))))))
~~~
