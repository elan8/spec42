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
# FORMAT
~~~sysml
package Conjugation {
	class A {
		in feature f;
	}
	
	class B conjugates A;
	
	feature g ~ B::f;
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "298ce35cd527980bef9b1f60de186509b444966aae024f67e767d6a762c73107") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Conjugation"))) (kind "package") (name "Conjugation") (declared-name "Conjugation") (range (start (line 0) (character 0)) (end (line 0) (character 99))))
    (element (id (node (document "d0") (qualified-name "Conjugation::A"))) (kind "classifier decl") (name "A") (declared-name "A") (range (start (line 1) (character 1)) (end (line 1) (character 29))) (parent (node (document "d0") (qualified-name "Conjugation"))))
    (element (id (node (document "d0") (qualified-name "Conjugation::B"))) (kind "classifier decl") (name "B") (declared-name "B") (range (start (line 5) (character 1)) (end (line 5) (character 22))) (parent (node (document "d0") (qualified-name "Conjugation"))))
    (element (id (node (document "d0") (qualified-name "Conjugation::g"))) (kind "feature decl") (name "g") (declared-name "g") (range (start (line 7) (character 1)) (end (line 7) (character 18))) (parent (node (document "d0") (qualified-name "Conjugation"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
