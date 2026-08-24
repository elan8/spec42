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
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 7 1) (end 7 18))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 7 1) (end 7 18))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:606df734549da2df657d6be0d7c5e2628d713ba1525e5e3b8ae95ce961d9e51a") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/conjugation.md") (qualified-name "Conjugation"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/conjugation.md") (qualified-name "Conjugation::A"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/conjugation.md") (qualified-name "Conjugation::A::f"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/conjugation.md") (qualified-name "Conjugation::B"))) (kind class-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (conjugation (reference "A")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/conjugation.md") (qualified-name "Conjugation::B"))) (kind conjugation) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/conjugation.md") (qualified-name "Conjugation::A")))))
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
)
~~~
