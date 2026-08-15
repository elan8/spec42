# META
~~~ini
description=KerML Simple Tests: Inverses
type=file
~~~
# SOURCE
~~~kerml
package Inverses {
	class A {
		feature f : B inverse of B::g disjoint from h;
		feature h : B;
	}
	
	class B {
		feature g : A;
	}
	
	inverse B::g of A::f;
	inverting Invert inverse B::g.f of A::h;
	
	feature gg : A featured by B inverse of A::f;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/inverses.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 10 1) (end 10 22))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 11 1) (end 11 41))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 13 1) (end 13 46))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 13 1) (end 13 46))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:9b8082649edd57d5c889bad34e80d0ba66845a57081bcb129570c18006dcadd4") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/inverses.md") (qualified-name "Inverses"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/inverses.md") (qualified-name "Inverses::A"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/inverses.md") (qualified-name "Inverses::A::f"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "B")))))
    (declaration (id (node (document "memory://snapshot/inverses.md") (qualified-name "Inverses::A::h"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "B")))))
    (declaration (id (node (document "memory://snapshot/inverses.md") (qualified-name "Inverses::B"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/inverses.md") (qualified-name "Inverses::B::g"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/inverses.md") (qualified-name "Inverses::A::f"))) (kind featureTyping) (ordinal 0))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/inverses.md") (qualified-name "Inverses::B")))))
    (reference (id (source (node (document "memory://snapshot/inverses.md") (qualified-name "Inverses::A::h"))) (kind featureTyping) (ordinal 0))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/inverses.md") (qualified-name "Inverses::B")))))
    (reference (id (source (node (document "memory://snapshot/inverses.md") (qualified-name "Inverses::B::g"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/inverses.md") (qualified-name "Inverses::A")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/inverses.md") (qualified-name "Inverses::A::f"))) (target (node (document "memory://snapshot/inverses.md") (qualified-name "Inverses::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/inverses.md") (qualified-name "Inverses::A::f"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/inverses.md") (qualified-name "Inverses::A::h"))) (target (node (document "memory://snapshot/inverses.md") (qualified-name "Inverses::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/inverses.md") (qualified-name "Inverses::A::h"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/inverses.md") (qualified-name "Inverses::B::g"))) (target (node (document "memory://snapshot/inverses.md") (qualified-name "Inverses::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/inverses.md") (qualified-name "Inverses::B::g"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/inverses.md") (qualified-name "Inverses::A")))
      (subtype (node (document "memory://snapshot/inverses.md") (qualified-name "Inverses::B::g")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/inverses.md") (qualified-name "Inverses::A::f")))
      (featured-by (node (document "memory://snapshot/inverses.md") (qualified-name "Inverses::A")))
      (type (node (document "memory://snapshot/inverses.md") (qualified-name "Inverses::B")) (provenance authored))
      (effective-type (node (document "memory://snapshot/inverses.md") (qualified-name "Inverses::B")) (source direct))
      (supertype (node (document "memory://snapshot/inverses.md") (qualified-name "Inverses::B")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/inverses.md") (qualified-name "Inverses::A::h")))
      (featured-by (node (document "memory://snapshot/inverses.md") (qualified-name "Inverses::A")))
      (type (node (document "memory://snapshot/inverses.md") (qualified-name "Inverses::B")) (provenance authored))
      (effective-type (node (document "memory://snapshot/inverses.md") (qualified-name "Inverses::B")) (source direct))
      (supertype (node (document "memory://snapshot/inverses.md") (qualified-name "Inverses::B")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/inverses.md") (qualified-name "Inverses::B")))
      (subtype (node (document "memory://snapshot/inverses.md") (qualified-name "Inverses::A::f")) (scopes any))
      (subtype (node (document "memory://snapshot/inverses.md") (qualified-name "Inverses::A::h")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/inverses.md") (qualified-name "Inverses::B::g")))
      (featured-by (node (document "memory://snapshot/inverses.md") (qualified-name "Inverses::B")))
      (type (node (document "memory://snapshot/inverses.md") (qualified-name "Inverses::A")) (provenance authored))
      (effective-type (node (document "memory://snapshot/inverses.md") (qualified-name "Inverses::A")) (source direct))
      (supertype (node (document "memory://snapshot/inverses.md") (qualified-name "Inverses::A")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/inverses.md") (range (start 2 14) (end 2 15)) (probe (position 2 14))
    (reference (id (source (node (document "memory://snapshot/inverses.md") (qualified-name "Inverses::A::f"))) (kind featureTyping) (ordinal 0) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/inverses.md") (qualified-name "Inverses::B")))))
    )
  )
  (query (document "memory://snapshot/inverses.md") (range (start 3 14) (end 3 15)) (probe (position 3 14))
    (reference (id (source (node (document "memory://snapshot/inverses.md") (qualified-name "Inverses::A::h"))) (kind featureTyping) (ordinal 0) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/inverses.md") (qualified-name "Inverses::B")))))
    )
  )
  (query (document "memory://snapshot/inverses.md") (range (start 7 14) (end 7 15)) (probe (position 7 14))
    (reference (id (source (node (document "memory://snapshot/inverses.md") (qualified-name "Inverses::B::g"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/inverses.md") (qualified-name "Inverses::A")))))
    )
  )
)
~~~
