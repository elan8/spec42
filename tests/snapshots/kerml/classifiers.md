# META
~~~ini
description=KerML Simple Tests: Classifiers
type=file
~~~
# SOURCE
~~~kerml
package Classifiers {
	classifier A;
	classifier B;
	
	specialization Super subclassifier A specializes B;
	specialization subclassifier B :> A;
	
	subclassifier C specializes A;
	subclassifier C specializes B;
	
	classifier C specializes A, B;
	
	classifier D disjoint from C differences A, B;
	classifier E specializes C intersects A, B;
	classifier F unions A unions B;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/classifiers.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 4 1) (end 4 52))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 5 1) (end 5 37))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 7 1) (end 7 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 8 1) (end 8 31))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:715d0cd223e7dbf7c86afe8131c35b26cfcefd7525691936f6f2fc9cc129b41d") (contract-version "parser-owned-resolution-v2"))
  (declarations
    (declaration (id (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::A"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::B"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::C"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "A")) (specialization (reference "B")))))
    (declaration (id (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::D"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (differencing (reference "A")) (differencing (reference "B")) (disjoining (reference "C")))))
    (declaration (id (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::E"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "C")) (intersecting (reference "A")) (intersecting (reference "B")))))
    (declaration (id (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::F"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (unioning (reference "A")) (unioning (reference "B")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::C"))) (kind specialization) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::A")))))
    (reference (id (source (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::C"))) (kind specialization) (ordinal 1))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::B")))))
    (reference (id (source (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::D"))) (kind differencing) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::A")))))
    (reference (id (source (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::D"))) (kind differencing) (ordinal 1))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::B")))))
    (reference (id (source (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::D"))) (kind disjoining) (ordinal 0))
      (authored-target "C")
      (outcome (status resolved) (target (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::C")))))
    (reference (id (source (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::E"))) (kind specialization) (ordinal 0))
      (authored-target "C")
      (outcome (status resolved) (target (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::C")))))
    (reference (id (source (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::E"))) (kind intersecting) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::A")))))
    (reference (id (source (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::E"))) (kind intersecting) (ordinal 1))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::B")))))
    (reference (id (source (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::F"))) (kind unioning) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::A")))))
    (reference (id (source (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::F"))) (kind unioning) (ordinal 1))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::B")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::C"))) (target (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::C"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::C"))) (target (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::C"))) (kind specialization) (ordinal 1)))
    (relationship (kind differencing) (source (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::D"))) (target (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::D"))) (kind differencing) (ordinal 0)))
    (relationship (kind differencing) (source (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::D"))) (target (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::D"))) (kind differencing) (ordinal 1)))
    (relationship (kind disjoining) (source (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::D"))) (target (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::C"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::D"))) (kind disjoining) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::E"))) (target (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::C"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::E"))) (kind specialization) (ordinal 0)))
    (relationship (kind intersecting) (source (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::E"))) (target (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::E"))) (kind intersecting) (ordinal 0)))
    (relationship (kind intersecting) (source (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::E"))) (target (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::E"))) (kind intersecting) (ordinal 1)))
    (relationship (kind unioning) (source (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::F"))) (target (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::F"))) (kind unioning) (ordinal 0)))
    (relationship (kind unioning) (source (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::F"))) (target (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::F"))) (kind unioning) (ordinal 1)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::A")))
      (subtype (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::C")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::B")))
      (subtype (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::C")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::C")))
      (supertype (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::A")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::B")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::E")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::D")))
      (set-operand (operator difference) (ordinal 0) (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::A")))
      (set-operand (operator disjoint) (ordinal 0) (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::C")))
      (set-operand (operator difference) (ordinal 1) (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::B")))
    )
    (declaration (id (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::E")))
      (set-operand (operator intersection) (ordinal 0) (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::A")))
      (set-operand (operator intersection) (ordinal 1) (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::B")))
      (supertype (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::A")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::B")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::C")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::F")))
      (set-operand (operator union) (ordinal 0) (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::A")))
      (set-operand (operator union) (ordinal 1) (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::B")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/classifiers.md") (range (start 10 26) (end 10 27)) (probe (position 10 26))
    (reference (id (source (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::C"))) (kind specialization) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::A")))))
    )
  )
  (query (document "memory://snapshot/classifiers.md") (range (start 10 29) (end 10 30)) (probe (position 10 29))
    (reference (id (source (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::C"))) (kind specialization) (ordinal 1) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::B")))))
    )
  )
  (query (document "memory://snapshot/classifiers.md") (range (start 12 42) (end 12 43)) (probe (position 12 42))
    (reference (id (source (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::D"))) (kind differencing) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::A")))))
    )
  )
  (query (document "memory://snapshot/classifiers.md") (range (start 12 45) (end 12 46)) (probe (position 12 45))
    (reference (id (source (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::D"))) (kind differencing) (ordinal 1) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::B")))))
    )
  )
  (query (document "memory://snapshot/classifiers.md") (range (start 12 28) (end 12 29)) (probe (position 12 28))
    (reference (id (source (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::D"))) (kind disjoining) (ordinal 0) (authored-target "C")
      (outcome (status resolved) (target (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::C")))))
    )
  )
  (query (document "memory://snapshot/classifiers.md") (range (start 13 26) (end 13 27)) (probe (position 13 26))
    (reference (id (source (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::E"))) (kind specialization) (ordinal 0) (authored-target "C")
      (outcome (status resolved) (target (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::C")))))
    )
  )
  (query (document "memory://snapshot/classifiers.md") (range (start 13 39) (end 13 40)) (probe (position 13 39))
    (reference (id (source (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::E"))) (kind intersecting) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::A")))))
    )
  )
  (query (document "memory://snapshot/classifiers.md") (range (start 13 42) (end 13 43)) (probe (position 13 42))
    (reference (id (source (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::E"))) (kind intersecting) (ordinal 1) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::B")))))
    )
  )
  (query (document "memory://snapshot/classifiers.md") (range (start 14 21) (end 14 22)) (probe (position 14 21))
    (reference (id (source (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::F"))) (kind unioning) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::A")))))
    )
  )
  (query (document "memory://snapshot/classifiers.md") (range (start 14 30) (end 14 31)) (probe (position 14 30))
    (reference (id (source (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::F"))) (kind unioning) (ordinal 1) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/classifiers.md") (qualified-name "Classifiers::B")))))
    )
  )
)
~~~
