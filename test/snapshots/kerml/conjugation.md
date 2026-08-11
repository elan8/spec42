# META
~~~ini
description=KerML Simple Tests: Conjugation
type=file
~~~
# SOURCE
~~~kerml
package Conjugation {
	class A {
		in feature f;
	}
	
	class B conjugates A;
	
	feature g ~ B::f;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "conjugation.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "298ce35cd527980bef9b1f60de186509b444966aae024f67e767d6a762c73107") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Conjugation"))) (kind "package") (name "Conjugation") (declared-name "Conjugation"))
    (element (id (node (document "d0") (qualified-name "Conjugation::A"))) (kind "classifier decl") (name "A") (declared-name "A") (parent (node (document "d0") (qualified-name "Conjugation"))))
    (element (id (node (document "d0") (qualified-name "Conjugation::B"))) (kind "classifier decl") (name "B") (declared-name "B") (parent (node (document "d0") (qualified-name "Conjugation"))))
    (element (id (node (document "d0") (qualified-name "Conjugation::g"))) (kind "feature decl") (name "g") (declared-name "g") (parent (node (document "d0") (qualified-name "Conjugation"))))
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
