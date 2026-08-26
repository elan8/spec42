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
  (document "memory://snapshot/inheritance.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 10 2) (end 10 19))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:bcf46385f7f1e9a2b8e6d606e86f99b9d21f2aa81f60309b3bfba5794642d5e0"))
  (declarations
    (declaration (id (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::A"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::A::f"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::B"))) (kind class-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "A")))))
    (declaration (id (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::us"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "w::g")))))
    (declaration (id (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::w"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "y")))))
    (declaration (id (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::y"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A")))))
    (declaration (id (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::y::g"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "f")))))
    (declaration (id (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::yy"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "y")))))
    (declaration (id (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::z"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "y::g")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::B"))) (kind specialization) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::A")))))
    (reference (id (source (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::us"))) (kind aliasBinding) (ordinal 0))
      (authored-target "w::g")
      (outcome (status resolved) (target (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::y::g")))))
    (reference (id (source (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::w"))) (kind subsetting) (ordinal 0))
      (authored-target "y")
      (outcome (status resolved) (target (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::y")))))
    (reference (id (source (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::y"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::A")))))
    (reference (id (source (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::y::g"))) (kind redefinition) (ordinal 0))
      (authored-target "f")
      (outcome (status resolved) (target (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::A::f")))))
    (reference (id (source (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::yy"))) (kind featureTyping) (ordinal 0))
      (authored-target "y")
      (outcome (status resolved) (target (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::y")))))
    (reference (id (source (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::z"))) (kind aliasBinding) (ordinal 0))
      (authored-target "y::g")
      (outcome (status resolved) (target (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::y::g")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::B"))) (target (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::B"))) (kind specialization) (ordinal 0)))
    (relationship (kind aliasBinding) (source (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::us"))) (target (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::y::g"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::us"))) (kind aliasBinding) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::w"))) (target (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::y"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::w"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::y"))) (target (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::y"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::y::g"))) (target (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::A::f"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::y::g"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::yy"))) (target (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::y"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::yy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind aliasBinding) (source (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::z"))) (target (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::y::g"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::z"))) (kind aliasBinding) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::A::f"))) (target (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::A"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::y::g"))) (target (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::y"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::A")))
      (subtype (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::B")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::y")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::A::f")))
      (featured-by (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::A")))
      (subtype (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::y::g")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::B")))
      (supertype (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::A")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::w")))
      (effective-type (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::A")) (source inherited) (from (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::y"))))
      (supertype (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::A")) (scopes any))
      (supertype (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::y")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::y")))
      (type (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::A")) (provenance authored))
      (effective-type (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::A")) (source direct))
      (supertype (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::A")) (scopes any))
      (subtype (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::w")) (scopes any feature))
      (subtype (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::yy")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::y::g")))
      (featured-by (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::y")))
      (supertype (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::A::f")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::yy")))
      (type (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::y")) (provenance authored))
      (effective-type (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::y")) (source direct))
      (supertype (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::A")) (scopes any))
      (supertype (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::y")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/inheritance.md") (range (start 5 21) (end 5 22)) (probe (position 5 21))
    (reference (id (source (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::B"))) (kind specialization) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::A")))))
    )
  )
  (query (document "memory://snapshot/inheritance.md") (range (start 18 14) (end 18 18)) (probe (position 18 14))
    (reference (id (source (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::us"))) (kind aliasBinding) (ordinal 0) (authored-target "w::g")
      (outcome (status resolved) (target (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::y::g")))))
    )
  )
  (query (document "memory://snapshot/inheritance.md") (range (start 16 19) (end 16 20)) (probe (position 16 19))
    (reference (id (source (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::w"))) (kind subsetting) (ordinal 0) (authored-target "y")
      (outcome (status resolved) (target (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::y")))))
    )
  )
  (query (document "memory://snapshot/inheritance.md") (range (start 9 12) (end 9 13)) (probe (position 9 12))
    (reference (id (source (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::y"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::A")))))
    )
  )
  (query (document "memory://snapshot/inheritance.md") (range (start 11 22) (end 11 23)) (probe (position 11 22))
    (reference (id (source (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::y::g"))) (kind redefinition) (ordinal 0) (authored-target "f")
      (outcome (status resolved) (target (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::A::f")))))
    )
  )
  (query (document "memory://snapshot/inheritance.md") (range (start 20 13) (end 20 14)) (probe (position 20 13))
    (reference (id (source (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::yy"))) (kind featureTyping) (ordinal 0) (authored-target "y")
      (outcome (status resolved) (target (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::y")))))
    )
  )
  (query (document "memory://snapshot/inheritance.md") (range (start 14 13) (end 14 17)) (probe (position 14 13))
    (reference (id (source (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::z"))) (kind aliasBinding) (ordinal 0) (authored-target "y::g")
      (outcome (status resolved) (target (node (document "memory://snapshot/inheritance.md") (qualified-name "Inheritance::y::g")))))
    )
  )
)
~~~
