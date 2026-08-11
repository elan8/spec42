# META
~~~ini
description=KerML Simple Tests: ArgumentResolution
type=file
~~~
# SOURCE
~~~kerml
package ArgumentResolutionBug {
	class A {
		feature x;
	}
	
	behavior B  {
		in feature x;
		out feature : A = new A(x);
	}
	
	class C {
		feature a : A;
		feature b : B;
		
		connector a ::> a.x to b;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "argument_resolution.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package ArgumentResolutionBug {
	class A {
		feature x;
	}
	
	behavior B  {
		in feature x;
		out feature : A = new A(x);
	}
	
	class C {
		feature a : A;
		feature b : B;
		
		connector a ::> a.x to b;
	}
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "168d41ced08f9a9163ae5446c034a66acc1a27d514697af3ccb6cfdf8df87f88") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ArgumentResolutionBug"))) (kind "package") (name "ArgumentResolutionBug") (declared-name "ArgumentResolutionBug"))
    (element (id (node (document "d0") (qualified-name "ArgumentResolutionBug::A"))) (kind "classifier decl") (name "A") (declared-name "A") (parent (node (document "d0") (qualified-name "ArgumentResolutionBug"))))
    (element (id (node (document "d0") (qualified-name "ArgumentResolutionBug::B"))) (kind "kermlDecl") (name "B") (declared-name "B") (parent (node (document "d0") (qualified-name "ArgumentResolutionBug"))))
    (element (id (node (document "d0") (qualified-name "ArgumentResolutionBug::C"))) (kind "classifier decl") (name "C") (declared-name "C") (parent (node (document "d0") (qualified-name "ArgumentResolutionBug"))))
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
