# META
~~~ini
description=SysML Example (Simple Tests): FeaturePathTest
type=file
~~~
# SOURCE
~~~sysml
package Q {
  part def F {
  	part a : A;
  }
  
  part f : F;
  
  part def A {
    part g = f.a;
  }
  
  part def B {
  	part f : F;
  	part a : A;
  }
  
  part def C {
	part b : B {
	  connect f.a to a.g;
	  bind f.a = a.g;
	}
  
	part c subsets b.f {
	  	part aa subsets a;
	}
	
	flow b.f.a to c.aa;
  }
  
  part e1 {
  	attribute x : E;
  	// Ensure that "e1" resolves correctly.
  	bind e1.x = E::e2;
  }
  
  enum def E {
  	enum e1;
  	enum e2;
  }
  
  part g = new A().g.g.g;
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/feature_path_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 22 16) (end 22 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 23 20) (end 23 21))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 26 1) (end 26 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 32 8) (end 32 12))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:36409e45e4c57189b02b3cd42222164a24447668078488e9d4db008077117f53") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::A"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::A::g"))) (kind part) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::B"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::B::a"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A"))))
    (declaration (id (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::B::f"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "F"))))
    (declaration (id (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::C"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::C::b"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "B"))))
    (declaration (id (node (document "memory://snapshot/feature_path_test.md") (anonymous (kind bare-connect) (ordinal 0))))) (kind bare-connect) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "f::a")) (memberAccessOperand (reference "a::g"))))
    (declaration (id (node (document "memory://snapshot/feature_path_test.md") (anonymous (kind bind) (ordinal 0))))) (kind bind) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "f::a")) (memberAccessOperand (reference "a::g"))))
    (declaration (id (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::C::c"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "b::f"))))
    (declaration (id (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::C::c::aa"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "a"))))
    (declaration (id (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::E"))) (kind enum-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::E::e1"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::E::e2"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::F"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::F::a"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A"))))
    (declaration (id (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::e1"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/feature_path_test.md") (anonymous (kind bind) (ordinal 0))))) (kind bind) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindTarget (reference "E::e2")) (memberAccessOperand (reference "e1::x"))))
    (declaration (id (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::e1::x"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "E"))))
    (declaration (id (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::f"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "F"))))
    (declaration (id (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::g"))) (kind part) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::B::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::A")))))
    (reference (id (source (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::B::f"))) (kind featureTyping) (ordinal 0))
      (authored-target "F")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::F")))))
    (reference (id (source (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::C::b"))) (kind featureTyping) (ordinal 0))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::B")))))
    (reference (id (source (node (document "memory://snapshot/feature_path_test.md") (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "f::a")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::F::a")))))
    (reference (id (source (node (document "memory://snapshot/feature_path_test.md") (anonymous (kind bind) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "f::a")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::F::a")))))
    (reference (id (source (node (document "memory://snapshot/feature_path_test.md") (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "a::g")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::A::g")))))
    (reference (id (source (node (document "memory://snapshot/feature_path_test.md") (anonymous (kind bind) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "a::g")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::A::g")))))
    (reference (id (source (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::C::c"))) (kind subsetting) (ordinal 0))
      (authored-target "b::f")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::C::c::aa"))) (kind subsetting) (ordinal 0))
      (authored-target "a")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::F::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::A")))))
    (reference (id (source (node (document "memory://snapshot/feature_path_test.md") (anonymous (kind bind) (ordinal 0))))) (kind bindTarget) (ordinal 0))
      (authored-target "E::e2")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::E::e2")))))
    (reference (id (source (node (document "memory://snapshot/feature_path_test.md") (anonymous (kind bind) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "e1::x")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::e1::x"))) (kind featureTyping) (ordinal 0))
      (authored-target "E")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::E")))))
    (reference (id (source (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::f"))) (kind featureTyping) (ordinal 0))
      (authored-target "F")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::F")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::B::a"))) (target (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::B::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::B::f"))) (target (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::F"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::B::f"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::C::b"))) (target (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::C::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/feature_path_test.md") (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::F::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/feature_path_test.md") (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/feature_path_test.md") (anonymous (kind bind) (ordinal 0))))) (target (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::F::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/feature_path_test.md") (anonymous (kind bind) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/feature_path_test.md") (anonymous (kind bare-connect) (ordinal 0))))) (target (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::A::g"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/feature_path_test.md") (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/feature_path_test.md") (anonymous (kind bind) (ordinal 0))))) (target (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::A::g"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/feature_path_test.md") (anonymous (kind bind) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::F::a"))) (target (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::F::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind bindTarget) (source (node (document "memory://snapshot/feature_path_test.md") (anonymous (kind bind) (ordinal 0))))) (target (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::E::e2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/feature_path_test.md") (anonymous (kind bind) (ordinal 0))))) (kind bindTarget) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::e1::x"))) (target (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::E"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::e1::x"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::f"))) (target (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::F"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::f"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/feature_path_test.md") (range (start 13 12) (end 13 13)) (probe (position 13 12))
    (reference (id (source (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::B::a"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::A")))))
  )
  (query (document "memory://snapshot/feature_path_test.md") (range (start 12 12) (end 12 13)) (probe (position 12 12))
    (reference (id (source (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::B::f"))) (kind featureTyping) (ordinal 0) (authored-target "F")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::F")))))
  )
  (query (document "memory://snapshot/feature_path_test.md") (range (start 17 10) (end 17 11)) (probe (position 17 10))
    (reference (id (source (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::C::b"))) (kind featureTyping) (ordinal 0) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::B")))))
  )
  (query (document "memory://snapshot/feature_path_test.md") (range (start 18 11) (end 18 14)) (probe (position 18 11))
    (reference (id (source (node (document "memory://snapshot/feature_path_test.md") (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "f::a")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::F::a")))))
  )
  (query (document "memory://snapshot/feature_path_test.md") (range (start 19 8) (end 19 11)) (probe (position 19 8))
    (reference (id (source (node (document "memory://snapshot/feature_path_test.md") (anonymous (kind bind) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "f::a")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::F::a")))))
  )
  (query (document "memory://snapshot/feature_path_test.md") (range (start 18 18) (end 18 21)) (probe (position 18 18))
    (reference (id (source (node (document "memory://snapshot/feature_path_test.md") (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1) (authored-target "a::g")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::A::g")))))
  )
  (query (document "memory://snapshot/feature_path_test.md") (range (start 19 14) (end 19 17)) (probe (position 19 14))
    (reference (id (source (node (document "memory://snapshot/feature_path_test.md") (anonymous (kind bind) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1) (authored-target "a::g")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::A::g")))))
  )
  (query (document "memory://snapshot/feature_path_test.md") (range (start 22 16) (end 22 19)) (probe (position 22 16))
    (reference (id (source (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::C::c"))) (kind subsetting) (ordinal 0) (authored-target "b::f")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_path_test.md") (range (start 23 20) (end 23 21)) (probe (position 23 20))
    (reference (id (source (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::C::c::aa"))) (kind subsetting) (ordinal 0) (authored-target "a")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_path_test.md") (range (start 2 12) (end 2 13)) (probe (position 2 12))
    (reference (id (source (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::F::a"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::A")))))
  )
  (query (document "memory://snapshot/feature_path_test.md") (range (start 32 15) (end 32 20)) (probe (position 32 15))
    (reference (id (source (node (document "memory://snapshot/feature_path_test.md") (anonymous (kind bind) (ordinal 0))))) (kind bindTarget) (ordinal 0) (authored-target "E::e2")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::E::e2")))))
  )
  (query (document "memory://snapshot/feature_path_test.md") (range (start 32 8) (end 32 12)) (probe (position 32 8))
    (reference (id (source (node (document "memory://snapshot/feature_path_test.md") (anonymous (kind bind) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "e1::x")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/feature_path_test.md") (range (start 30 17) (end 30 18)) (probe (position 30 17))
    (reference (id (source (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::e1::x"))) (kind featureTyping) (ordinal 0) (authored-target "E")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::E")))))
  )
  (query (document "memory://snapshot/feature_path_test.md") (range (start 5 11) (end 5 12)) (probe (position 5 11))
    (reference (id (source (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::f"))) (kind featureTyping) (ordinal 0) (authored-target "F")
      (outcome (status resolved) (target (node (document "memory://snapshot/feature_path_test.md") (qualified-name "Q::F")))))
  )
)
~~~
