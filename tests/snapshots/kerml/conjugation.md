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
  (document "memory://snapshot/conjugation.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 7 13) (end 7 17))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:6f73e9a9a568f9d3ee99cbd1c5b83dfdd0e037c8e2f134cc4656a5d38cf16fba"))
  (declarations
    (declaration (id (node (document "memory://snapshot/conjugation.md") (qualified-name "Conjugation"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/conjugation.md") (qualified-name "Conjugation::A"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/conjugation.md") (qualified-name "Conjugation::A::f"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/conjugation.md") (qualified-name "Conjugation::B"))) (kind class-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (conjugation (reference "A")))))
    (declaration (id (node (document "memory://snapshot/conjugation.md") (qualified-name "Conjugation::g"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (conjugation (reference "B::f")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/conjugation.md") (qualified-name "Conjugation::B"))) (kind conjugation) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/conjugation.md") (qualified-name "Conjugation::A")))))
    (reference (id (source (node (document "memory://snapshot/conjugation.md") (qualified-name "Conjugation::g"))) (kind conjugation) (ordinal 0))
      (authored-target "B::f")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind conjugation) (source (node (document "memory://snapshot/conjugation.md") (qualified-name "Conjugation::B"))) (target (node (document "memory://snapshot/conjugation.md") (qualified-name "Conjugation::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/conjugation.md") (qualified-name "Conjugation::B"))) (kind conjugation) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/conjugation.md") (qualified-name "Conjugation::A::f"))) (target (node (document "memory://snapshot/conjugation.md") (qualified-name "Conjugation::A"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/conjugation.md") (qualified-name "Conjugation::A::f")))
      (featured-by (node (document "memory://snapshot/conjugation.md") (qualified-name "Conjugation::A")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/conjugation.md") (range (start 5 20) (end 5 21)) (probe (position 5 20))
    (reference (id (source (node (document "memory://snapshot/conjugation.md") (qualified-name "Conjugation::B"))) (kind conjugation) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/conjugation.md") (qualified-name "Conjugation::A")))))
    )
  )
  (query (document "memory://snapshot/conjugation.md") (range (start 7 13) (end 7 17)) (probe (position 7 13))
    (reference (id (source (node (document "memory://snapshot/conjugation.md") (qualified-name "Conjugation::g"))) (kind conjugation) (ordinal 0) (authored-target "B::f")
      (outcome (status unresolved)))
    )
  )
)
~~~
