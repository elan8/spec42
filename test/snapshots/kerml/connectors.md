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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "connectors.md"
    (diagnostics
    )
  )
)
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
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "1abfbd474987d8a5beaeb9a1f6dc96b1f219375bf11126d32974ab2caa045a5c") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Connectors"))) (kind "package") (name "Connectors") (declared-name "Connectors") (range (start (line 0) (character 0)) (end (line 0) (character 546))))
    (element (id (node (document "d0") (qualified-name "Connectors::A"))) (kind "classifier decl") (name "A") (declared-name "A") (range (start (line 2) (character 1)) (end (line 2) (character 440))) (parent (node (document "d0") (qualified-name "Connectors"))))
    (element (id (node (document "d0") (qualified-name "Connectors::B"))) (kind "classifier decl") (name "B") (declared-name "B") (range (start (line 28) (character 1)) (end (line 28) (character 78))) (parent (node (document "d0") (qualified-name "Connectors"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
