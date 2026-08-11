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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "a584141b68b061427b7446ce41755100e73ae0a8daef9b7f35906542c67f2bb3") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Connectors"))) (kind "package") (name "Connectors") (declared-name "Connectors"))
    (element (id (node (document "d0") (qualified-name "Connectors::A"))) (kind "classifier decl") (name "A") (declared-name "A") (parent (node (document "d0") (qualified-name "Connectors"))))
    (element (id (node (document "d0") (qualified-name "Connectors::B"))) (kind "classifier decl") (name "B") (declared-name "B") (parent (node (document "d0") (qualified-name "Connectors"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
