# META
~~~ini
description=KerML Simple Tests: Associations
type=file
~~~
# SOURCE
~~~kerml
package Associations {
    datatype X;
    class Y;
    
	assoc A {
		end x_cross [1..1] feature x : X; 
		end y_cross [1..*] feature y : Y;
	}
	
	assoc B specializes A {
		end x1;
		end [0..*] feature y1 redefines y;
	}
	
	assoc struct C {
		const end [1] feature a;
		const end feature b;
	}
	
	metaclass M;	
	assoc XY {
		end [0..1] feature x : X {
			@M;
		}
		end feature y : Y;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/associations.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 34) (end 11 35))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:9c1cfd6ca73422e202a633654de22826f37b6d9381a913651af6e4a2d2704f0a") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/associations.md") (qualified-name "Associations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/associations.md") (qualified-name "Associations::A"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/associations.md") (qualified-name "Associations::A::x_cross"))) (kind kerml-end) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))))
    (declaration (id (node (document "memory://snapshot/associations.md") (qualified-name "Associations::A::x_cross::x"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "X")))))
    (declaration (id (node (document "memory://snapshot/associations.md") (qualified-name "Associations::A::y_cross"))) (kind kerml-end) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper unbounded))))
    (declaration (id (node (document "memory://snapshot/associations.md") (qualified-name "Associations::A::y_cross::y"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Y")))))
    (declaration (id (node (document "memory://snapshot/associations.md") (qualified-name "Associations::B"))) (kind kerml-association) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "A")))))
    (declaration (id (node (document "memory://snapshot/associations.md") (path (named (kind package) (name "Associations")) (named (kind kerml-association) (name "B")) (anonymous (kind kerml-end) (ordinal 0))))) (kind kerml-end) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper unbounded))))
    (declaration (id (node (document "memory://snapshot/associations.md") (path (named (kind package) (name "Associations")) (named (kind kerml-association) (name "B")) (anonymous (kind kerml-end) (ordinal 0)) (named (kind kerml-feature) (name "y1"))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "y")))))
    (declaration (id (node (document "memory://snapshot/associations.md") (qualified-name "Associations::B::x1"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)))
    (declaration (id (node (document "memory://snapshot/associations.md") (qualified-name "Associations::C"))) (kind kerml-association-structure) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/associations.md") (path (named (kind package) (name "Associations")) (named (kind kerml-association-structure) (name "C")) (anonymous (kind kerml-end) (ordinal 0))))) (kind kerml-end) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))))
    (declaration (id (node (document "memory://snapshot/associations.md") (path (named (kind package) (name "Associations")) (named (kind kerml-association-structure) (name "C")) (anonymous (kind kerml-end) (ordinal 0)) (named (kind kerml-feature) (name "a"))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/associations.md") (qualified-name "Associations::C::b"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)))
    (declaration (id (node (document "memory://snapshot/associations.md") (qualified-name "Associations::M"))) (kind kerml-metaclass) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/associations.md") (qualified-name "Associations::X"))) (kind kerml-datatype) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/associations.md") (qualified-name "Associations::XY"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/associations.md") (path (named (kind package) (name "Associations")) (named (kind kerml-association) (name "XY")) (anonymous (kind kerml-end) (ordinal 0))))) (kind kerml-end) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))))
    (declaration (id (node (document "memory://snapshot/associations.md") (path (named (kind package) (name "Associations")) (named (kind kerml-association) (name "XY")) (anonymous (kind kerml-end) (ordinal 0)) (named (kind kerml-feature) (name "x"))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "X")) (metadataAnnotation (reference "M")))))
    (declaration (id (node (document "memory://snapshot/associations.md") (qualified-name "Associations::XY::y"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Y")))))
    (declaration (id (node (document "memory://snapshot/associations.md") (qualified-name "Associations::Y"))) (kind class-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/associations.md") (qualified-name "Associations::A::x_cross::x"))) (kind featureTyping) (ordinal 0))
      (authored-target "X")
      (outcome (status resolved) (target (node (document "memory://snapshot/associations.md") (qualified-name "Associations::X")))))
    (reference (id (source (node (document "memory://snapshot/associations.md") (qualified-name "Associations::A::y_cross::y"))) (kind featureTyping) (ordinal 0))
      (authored-target "Y")
      (outcome (status resolved) (target (node (document "memory://snapshot/associations.md") (qualified-name "Associations::Y")))))
    (reference (id (source (node (document "memory://snapshot/associations.md") (qualified-name "Associations::B"))) (kind specialization) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/associations.md") (qualified-name "Associations::A")))))
    (reference (id (source (node (document "memory://snapshot/associations.md") (path (named (kind package) (name "Associations")) (named (kind kerml-association) (name "B")) (anonymous (kind kerml-end) (ordinal 0)) (named (kind kerml-feature) (name "y1"))))) (kind redefinition) (ordinal 0))
      (authored-target "y")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/associations.md") (path (named (kind package) (name "Associations")) (named (kind kerml-association) (name "XY")) (anonymous (kind kerml-end) (ordinal 0)) (named (kind kerml-feature) (name "x"))))) (kind featureTyping) (ordinal 0))
      (authored-target "X")
      (outcome (status resolved) (target (node (document "memory://snapshot/associations.md") (qualified-name "Associations::X")))))
    (reference (id (source (node (document "memory://snapshot/associations.md") (path (named (kind package) (name "Associations")) (named (kind kerml-association) (name "XY")) (anonymous (kind kerml-end) (ordinal 0)) (named (kind kerml-feature) (name "x"))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "M")
      (outcome (status resolved) (target (node (document "memory://snapshot/associations.md") (qualified-name "Associations::M")))))
    (reference (id (source (node (document "memory://snapshot/associations.md") (qualified-name "Associations::XY::y"))) (kind featureTyping) (ordinal 0))
      (authored-target "Y")
      (outcome (status resolved) (target (node (document "memory://snapshot/associations.md") (qualified-name "Associations::Y")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/associations.md") (qualified-name "Associations::A::x_cross::x"))) (target (node (document "memory://snapshot/associations.md") (qualified-name "Associations::X"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/associations.md") (qualified-name "Associations::A::x_cross::x"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/associations.md") (qualified-name "Associations::A::y_cross::y"))) (target (node (document "memory://snapshot/associations.md") (qualified-name "Associations::Y"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/associations.md") (qualified-name "Associations::A::y_cross::y"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/associations.md") (qualified-name "Associations::B"))) (target (node (document "memory://snapshot/associations.md") (qualified-name "Associations::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/associations.md") (qualified-name "Associations::B"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/associations.md") (path (named (kind package) (name "Associations")) (named (kind kerml-association) (name "XY")) (anonymous (kind kerml-end) (ordinal 0)) (named (kind kerml-feature) (name "x"))))) (target (node (document "memory://snapshot/associations.md") (qualified-name "Associations::X"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/associations.md") (path (named (kind package) (name "Associations")) (named (kind kerml-association) (name "XY")) (anonymous (kind kerml-end) (ordinal 0)) (named (kind kerml-feature) (name "x"))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/associations.md") (path (named (kind package) (name "Associations")) (named (kind kerml-association) (name "XY")) (anonymous (kind kerml-end) (ordinal 0)) (named (kind kerml-feature) (name "x"))))) (target (node (document "memory://snapshot/associations.md") (qualified-name "Associations::M"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/associations.md") (path (named (kind package) (name "Associations")) (named (kind kerml-association) (name "XY")) (anonymous (kind kerml-end) (ordinal 0)) (named (kind kerml-feature) (name "x"))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/associations.md") (qualified-name "Associations::XY::y"))) (target (node (document "memory://snapshot/associations.md") (qualified-name "Associations::Y"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/associations.md") (qualified-name "Associations::XY::y"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/associations.md") (qualified-name "Associations::A")))
      (subtype (node (document "memory://snapshot/associations.md") (qualified-name "Associations::B")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/associations.md") (qualified-name "Associations::A::x_cross")))
      (featured-by (node (document "memory://snapshot/associations.md") (qualified-name "Associations::A")))
    )
    (declaration (id (node (document "memory://snapshot/associations.md") (qualified-name "Associations::A::x_cross::x")))
      (featured-by (node (document "memory://snapshot/associations.md") (qualified-name "Associations::A::x_cross")))
      (type (node (document "memory://snapshot/associations.md") (qualified-name "Associations::X")) (provenance authored))
      (effective-type (node (document "memory://snapshot/associations.md") (qualified-name "Associations::X")) (source direct))
      (supertype (node (document "memory://snapshot/associations.md") (qualified-name "Associations::X")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/associations.md") (qualified-name "Associations::A::y_cross")))
      (featured-by (node (document "memory://snapshot/associations.md") (qualified-name "Associations::A")))
    )
    (declaration (id (node (document "memory://snapshot/associations.md") (qualified-name "Associations::A::y_cross::y")))
      (featured-by (node (document "memory://snapshot/associations.md") (qualified-name "Associations::A::y_cross")))
      (type (node (document "memory://snapshot/associations.md") (qualified-name "Associations::Y")) (provenance authored))
      (effective-type (node (document "memory://snapshot/associations.md") (qualified-name "Associations::Y")) (source direct))
      (supertype (node (document "memory://snapshot/associations.md") (qualified-name "Associations::Y")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/associations.md") (qualified-name "Associations::B")))
      (supertype (node (document "memory://snapshot/associations.md") (qualified-name "Associations::A")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/associations.md") (path (named (kind package) (name "Associations")) (named (kind kerml-association) (name "B")) (anonymous (kind kerml-end) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/associations.md") (qualified-name "Associations::B")))
    )
    (declaration (id (node (document "memory://snapshot/associations.md") (path (named (kind package) (name "Associations")) (named (kind kerml-association) (name "B")) (anonymous (kind kerml-end) (ordinal 0)) (named (kind kerml-feature) (name "y1")))))
      (featured-by (node (document "memory://snapshot/associations.md") (path (named (kind package) (name "Associations")) (named (kind kerml-association) (name "B")) (anonymous (kind kerml-end) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/associations.md") (qualified-name "Associations::B::x1")))
      (featured-by (node (document "memory://snapshot/associations.md") (qualified-name "Associations::B")))
    )
    (declaration (id (node (document "memory://snapshot/associations.md") (path (named (kind package) (name "Associations")) (named (kind kerml-association-structure) (name "C")) (anonymous (kind kerml-end) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/associations.md") (qualified-name "Associations::C")))
    )
    (declaration (id (node (document "memory://snapshot/associations.md") (path (named (kind package) (name "Associations")) (named (kind kerml-association-structure) (name "C")) (anonymous (kind kerml-end) (ordinal 0)) (named (kind kerml-feature) (name "a")))))
      (featured-by (node (document "memory://snapshot/associations.md") (path (named (kind package) (name "Associations")) (named (kind kerml-association-structure) (name "C")) (anonymous (kind kerml-end) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/associations.md") (qualified-name "Associations::C::b")))
      (featured-by (node (document "memory://snapshot/associations.md") (qualified-name "Associations::C")))
    )
    (declaration (id (node (document "memory://snapshot/associations.md") (qualified-name "Associations::X")))
      (subtype (node (document "memory://snapshot/associations.md") (qualified-name "Associations::A::x_cross::x")) (scopes any))
      (subtype (node (document "memory://snapshot/associations.md") (path (named (kind package) (name "Associations")) (named (kind kerml-association) (name "XY")) (anonymous (kind kerml-end) (ordinal 0)) (named (kind kerml-feature) (name "x")))) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/associations.md") (path (named (kind package) (name "Associations")) (named (kind kerml-association) (name "XY")) (anonymous (kind kerml-end) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/associations.md") (qualified-name "Associations::XY")))
    )
    (declaration (id (node (document "memory://snapshot/associations.md") (path (named (kind package) (name "Associations")) (named (kind kerml-association) (name "XY")) (anonymous (kind kerml-end) (ordinal 0)) (named (kind kerml-feature) (name "x")))))
      (featured-by (node (document "memory://snapshot/associations.md") (path (named (kind package) (name "Associations")) (named (kind kerml-association) (name "XY")) (anonymous (kind kerml-end) (ordinal 0)))))
      (type (node (document "memory://snapshot/associations.md") (qualified-name "Associations::X")) (provenance authored))
      (effective-type (node (document "memory://snapshot/associations.md") (qualified-name "Associations::X")) (source direct))
      (supertype (node (document "memory://snapshot/associations.md") (qualified-name "Associations::X")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/associations.md") (qualified-name "Associations::XY::y")))
      (featured-by (node (document "memory://snapshot/associations.md") (qualified-name "Associations::XY")))
      (type (node (document "memory://snapshot/associations.md") (qualified-name "Associations::Y")) (provenance authored))
      (effective-type (node (document "memory://snapshot/associations.md") (qualified-name "Associations::Y")) (source direct))
      (supertype (node (document "memory://snapshot/associations.md") (qualified-name "Associations::Y")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/associations.md") (qualified-name "Associations::Y")))
      (subtype (node (document "memory://snapshot/associations.md") (qualified-name "Associations::A::y_cross::y")) (scopes any))
      (subtype (node (document "memory://snapshot/associations.md") (qualified-name "Associations::XY::y")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/associations.md") (range (start 5 33) (end 5 34)) (probe (position 5 33))
    (reference (id (source (node (document "memory://snapshot/associations.md") (qualified-name "Associations::A::x_cross::x"))) (kind featureTyping) (ordinal 0) (authored-target "X")
      (outcome (status resolved) (target (node (document "memory://snapshot/associations.md") (qualified-name "Associations::X")))))
    )
  )
  (query (document "memory://snapshot/associations.md") (range (start 6 33) (end 6 34)) (probe (position 6 33))
    (reference (id (source (node (document "memory://snapshot/associations.md") (qualified-name "Associations::A::y_cross::y"))) (kind featureTyping) (ordinal 0) (authored-target "Y")
      (outcome (status resolved) (target (node (document "memory://snapshot/associations.md") (qualified-name "Associations::Y")))))
    )
  )
  (query (document "memory://snapshot/associations.md") (range (start 9 21) (end 9 22)) (probe (position 9 21))
    (reference (id (source (node (document "memory://snapshot/associations.md") (qualified-name "Associations::B"))) (kind specialization) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/associations.md") (qualified-name "Associations::A")))))
    )
  )
  (query (document "memory://snapshot/associations.md") (range (start 11 34) (end 11 35)) (probe (position 11 34))
    (reference (id (source (node (document "memory://snapshot/associations.md") (path (named (kind package) (name "Associations")) (named (kind kerml-association) (name "B")) (anonymous (kind kerml-end) (ordinal 0)) (named (kind kerml-feature) (name "y1"))))) (kind redefinition) (ordinal 0) (authored-target "y")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/associations.md") (range (start 21 25) (end 21 26)) (probe (position 21 25))
    (reference (id (source (node (document "memory://snapshot/associations.md") (path (named (kind package) (name "Associations")) (named (kind kerml-association) (name "XY")) (anonymous (kind kerml-end) (ordinal 0)) (named (kind kerml-feature) (name "x"))))) (kind featureTyping) (ordinal 0) (authored-target "X")
      (outcome (status resolved) (target (node (document "memory://snapshot/associations.md") (qualified-name "Associations::X")))))
    )
  )
  (query (document "memory://snapshot/associations.md") (range (start 22 4) (end 22 5)) (probe (position 22 4))
    (reference (id (source (node (document "memory://snapshot/associations.md") (path (named (kind package) (name "Associations")) (named (kind kerml-association) (name "XY")) (anonymous (kind kerml-end) (ordinal 0)) (named (kind kerml-feature) (name "x"))))) (kind metadataAnnotation) (ordinal 0) (authored-target "M")
      (outcome (status resolved) (target (node (document "memory://snapshot/associations.md") (qualified-name "Associations::M")))))
    )
  )
  (query (document "memory://snapshot/associations.md") (range (start 24 18) (end 24 19)) (probe (position 24 18))
    (reference (id (source (node (document "memory://snapshot/associations.md") (qualified-name "Associations::XY::y"))) (kind featureTyping) (ordinal 0) (authored-target "Y")
      (outcome (status resolved) (target (node (document "memory://snapshot/associations.md") (qualified-name "Associations::Y")))))
    )
  )
)
~~~
