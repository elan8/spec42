# META
~~~ini
description=KerML Simple Tests: Inheritance
type=file
~~~
# SOURCE
~~~kerml
package Inheritance {
	class A {
		feature f;
	}
	
	class B specializes A {
		
	}
		
	feature y: A {
		alias x for B::f;
		feature g redefines f;
	}
	
	alias z for y::g;
	
	feature w subsets y;
	
	alias us for w::g;
	
	feature yy: y;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "inheritance.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package Inheritance {
	class A {
		feature f;
	}
	
	class B specializes A {
		
	}
		
	feature y: A {
		alias x for B::f;
		feature g redefines f;
	}
	
	alias z for y::g;
	
	feature w subsets y;
	
	alias us for w::g;
	
	feature yy: y;
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "11c9649ec23a10bc21a040a438dd1d7f8f7ed187b86045d1a00cc1008019a78e") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Inheritance"))) (kind "package") (name "Inheritance") (declared-name "Inheritance"))
    (element (id (node (document "d0") (qualified-name "Inheritance::A"))) (kind "classifier decl") (name "A") (declared-name "A") (parent (node (document "d0") (qualified-name "Inheritance"))))
    (element (id (node (document "d0") (qualified-name "Inheritance::B"))) (kind "classifier decl") (name "B") (declared-name "B") (parent (node (document "d0") (qualified-name "Inheritance"))))
    (element (id (node (document "d0") (qualified-name "Inheritance::us"))) (kind "alias") (name "us") (declared-name "us") (parent (node (document "d0") (qualified-name "Inheritance"))))
    (element (id (node (document "d0") (qualified-name "Inheritance::w"))) (kind "feature decl") (name "w") (declared-name "w") (parent (node (document "d0") (qualified-name "Inheritance"))))
    (element (id (node (document "d0") (qualified-name "Inheritance::y"))) (kind "feature decl") (name "y") (declared-name "y") (parent (node (document "d0") (qualified-name "Inheritance"))))
    (element (id (node (document "d0") (qualified-name "Inheritance::yy"))) (kind "feature decl") (name "yy") (declared-name "yy") (parent (node (document "d0") (qualified-name "Inheritance"))))
    (element (id (node (document "d0") (qualified-name "Inheritance::z"))) (kind "alias") (name "z") (declared-name "z") (parent (node (document "d0") (qualified-name "Inheritance"))))
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
