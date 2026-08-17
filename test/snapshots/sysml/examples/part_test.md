# META
~~~ini
description=SysML Example (Simple Tests): PartTest
type=file
~~~
# SOURCE
~~~sysml
package PartTest {
	
	part f: A;

	public part def A {
		part <'1'> b: B;
		protected port c: C;
		constant attribute x[0..2];
		derived constant ref attribute y :> x;
		ref z : ScalarValues::Integer;
	}
	
	item def S;
	
	abstract part def <xx> B {
		public abstract part a: A[1..2];
		public abstract part b subsets a;
		public abstract part c[0..1] subsets a;
		port x: ~C {
		    port p;
		    ref port q;
		}
		package P { }
		
		succession flow x.p to a1.aa.receiver;
		
		action a1 {
			accept S via x;
			action aa accept S;
		}
		perform action a2;
		
		state s1;
		exhibit state s2;
	}
	
	private port def C {
		private in ref y: A, B {
		    part B_b redefines B::b;
		    part B_c redefines B::c;
		    port B_x redefines B::x;
		}
		alias z1 for y;
		alias z2 for y;
		port c1 : C;
		ref port c2 : C;
	}
	
    part p1 :> p2;
    part p2 :> p3; 
    part p3 :> p1;
    
    part p4 :> p4;
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/part_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 10) (end 9 31))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 20 6) (end 21 2))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 22 2) (end 24 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 24 2) (end 24 40))
      )
      (diagnostic
        (severity error)
        (code "recovered_action_body_element")
        (source "parser")
        (range (start 27 3) (end 28 3))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 27 3) (end 28 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 33 2) (end 33 19))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 37 2) (end 42 2))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 42 2) (end 43 2))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 43 2) (end 44 2))
      )
      (diagnostic
        (severity error)
        (code "specialization_cycle")
        (source "semantic")
        (range (start 48 4) (end 48 18))
      )
      (diagnostic
        (severity error)
        (code "specialization_cycle")
        (source "semantic")
        (range (start 49 4) (end 49 18))
      )
      (diagnostic
        (severity error)
        (code "specialization_cycle")
        (source "semantic")
        (range (start 50 4) (end 50 18))
      )
      (diagnostic
        (severity error)
        (code "specialization_cycle")
        (source "semantic")
        (range (start 52 4) (end 52 18))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:d73a8d0290bb5fbe966de22ff1bd454d55fb5a39dfb60b7a577bc6a80badef05") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A"))) (kind part-def) (membership (kind owning) (visibility public)))
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A::b"))) (kind part) (membership (kind feature) (visibility default)) (facts (short-name "1")) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "B")))))
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A::c"))) (kind port) (membership (kind feature) (visibility protected)) (authored (membership (kind feature) (visibility protected)) (relationships (featureTyping (reference "C")))))
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A::x"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (modifiers constant) (multiplicity (lower 0) (upper 2))))
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A::y"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (modifiers derived reference constant)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "x")))))
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A::z"))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValues::Integer")))))
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B"))) (kind part-def) (membership (kind owning) (visibility default)) (facts (short-name "xx") (modifiers abstract)))
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::a"))) (kind part) (membership (kind feature) (visibility public)) (facts (modifiers abstract) (multiplicity (lower 1) (upper 2))) (authored (membership (kind feature) (visibility public)) (relationships (featureTyping (reference "A")))))
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::a1"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::a1::aa"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::a2"))) (kind perform-action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::b"))) (kind part) (membership (kind feature) (visibility public)) (facts (modifiers abstract)) (authored (membership (kind feature) (visibility public)) (relationships (subsetting (reference "a")))))
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::c"))) (kind part) (membership (kind feature) (visibility public)) (facts (modifiers abstract) (multiplicity (lower 0) (upper 1))) (authored (membership (kind feature) (visibility public)) (relationships (subsetting (reference "a")))))
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::s1"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::x"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "C") (conjugated true)))))
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::x::p"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::C"))) (kind port-def) (membership (kind owning) (visibility private)))
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::C::c1"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "C")))))
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::C::c2"))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "C")))))
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::S"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::f"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A")))))
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "p2")))))
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p2"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "p3")))))
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p3"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "p1")))))
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p4"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "p4")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A::b"))) (kind featureTyping) (ordinal 0))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B")))))
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A::c"))) (kind featureTyping) (ordinal 0))
      (authored-target "C")
      (outcome (status resolved) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::C")))))
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A::y"))) (kind subsetting) (ordinal 0))
      (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A::x")))))
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A::z"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValues::Integer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A")))))
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::b"))) (kind subsetting) (ordinal 0))
      (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::a")))))
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::c"))) (kind subsetting) (ordinal 0))
      (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::a")))))
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::x"))) (kind featureTyping) (ordinal 0))
      (authored-target "C")
      (outcome (status resolved) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::C")))))
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::C::c1"))) (kind featureTyping) (ordinal 0))
      (authored-target "C")
      (outcome (status resolved) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::C")))))
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::C::c2"))) (kind featureTyping) (ordinal 0))
      (authored-target "C")
      (outcome (status resolved) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::C")))))
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::f"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A")))))
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p1"))) (kind subsetting) (ordinal 0))
      (authored-target "p2")
      (outcome (status resolved) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p2")))))
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p2"))) (kind subsetting) (ordinal 0))
      (authored-target "p3")
      (outcome (status resolved) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p3")))))
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p3"))) (kind subsetting) (ordinal 0))
      (authored-target "p1")
      (outcome (status resolved) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p1")))))
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p4"))) (kind subsetting) (ordinal 0))
      (authored-target "p4")
      (outcome (status resolved) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p4")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A::b"))) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A::c"))) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::C"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A::c"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A::y"))) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A::x"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A::y"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::a"))) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::b"))) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::b"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::c"))) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::c"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::x"))) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::C"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::x"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::C::c1"))) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::C"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::C::c1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::C::c2"))) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::C"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::C::c2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::f"))) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::f"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p1"))) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p1"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p2"))) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p3"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p2"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p3"))) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p3"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p4"))) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p4"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p4"))) (kind subsetting) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A")))
      (subtype (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::a")) (scopes any))
      (subtype (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::f")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A::b")))
      (featured-by (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A")))
      (type (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B")) (provenance authored))
      (effective-type (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B")) (source direct))
      (supertype (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A::c")))
      (featured-by (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A")))
      (type (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::C")) (provenance authored))
      (effective-type (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::C")) (source direct))
      (supertype (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::C")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A::x")))
      (featured-by (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A")))
      (subtype (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A::y")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A::y")))
      (featured-by (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A")))
      (supertype (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A::x")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A::z")))
      (featured-by (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A")))
    )
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B")))
      (subtype (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A::b")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::a")))
      (featured-by (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B")))
      (type (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A")) (provenance authored))
      (effective-type (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A")) (source direct))
      (supertype (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A")) (scopes any))
      (subtype (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::b")) (scopes any feature))
      (subtype (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::c")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::a1")))
      (featured-by (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B")))
    )
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::a1::aa")))
      (featured-by (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::a1")))
    )
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::a2")))
      (featured-by (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B")))
    )
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::b")))
      (featured-by (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B")))
      (effective-type (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A")) (source inherited) (from (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::a"))))
      (supertype (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A")) (scopes any))
      (supertype (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::a")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::c")))
      (featured-by (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B")))
      (effective-type (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A")) (source inherited) (from (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::a"))))
      (supertype (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A")) (scopes any))
      (supertype (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::a")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::s1")))
      (featured-by (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B")))
    )
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::x")))
      (featured-by (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B")))
      (type (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::C")) (provenance authored))
      (effective-type (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::C")) (source direct))
      (supertype (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::C")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::x::p")))
      (featured-by (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::x")))
    )
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::C")))
      (subtype (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A::c")) (scopes any))
      (subtype (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::x")) (scopes any))
      (subtype (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::C::c1")) (scopes any))
      (subtype (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::C::c2")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::C::c1")))
      (featured-by (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::C")))
      (type (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::C")) (provenance authored))
      (effective-type (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::C")) (source direct))
      (supertype (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::C")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::C::c2")))
      (featured-by (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::C")))
      (type (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::C")) (provenance authored))
      (effective-type (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::C")) (source direct))
      (supertype (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::C")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::f")))
      (type (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A")) (provenance authored))
      (effective-type (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A")) (source direct))
      (supertype (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p1"))) (cyclic true)
      (supertype (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p2")) (scopes any feature))
      (supertype (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p3")) (scopes any feature))
      (subtype (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p3")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p2"))) (cyclic true)
      (supertype (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p1")) (scopes any feature))
      (supertype (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p3")) (scopes any feature))
      (subtype (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p1")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p3"))) (cyclic true)
      (supertype (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p1")) (scopes any feature))
      (supertype (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p2")) (scopes any feature))
      (subtype (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p2")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p4"))) (cyclic true)
      (subtype (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p4")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/part_test.md") (range (start 5 16) (end 5 17)) (probe (position 5 16))
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A::b"))) (kind featureTyping) (ordinal 0) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B")))))
    )
  )
  (query (document "memory://snapshot/part_test.md") (range (start 6 20) (end 6 21)) (probe (position 6 20))
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A::c"))) (kind featureTyping) (ordinal 0) (authored-target "C")
      (outcome (status resolved) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::C")))))
    )
  )
  (query (document "memory://snapshot/part_test.md") (range (start 8 38) (end 8 39)) (probe (position 8 38))
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A::y"))) (kind subsetting) (ordinal 0) (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A::x")))))
    )
  )
  (query (document "memory://snapshot/part_test.md") (range (start 9 10) (end 9 31)) (probe (position 9 10))
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A::z"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValues::Integer")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/part_test.md") (range (start 15 26) (end 15 27)) (probe (position 15 26))
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::a"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A")))))
    )
  )
  (query (document "memory://snapshot/part_test.md") (range (start 16 33) (end 16 34)) (probe (position 16 33))
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::b"))) (kind subsetting) (ordinal 0) (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::a")))))
    )
  )
  (query (document "memory://snapshot/part_test.md") (range (start 17 39) (end 17 40)) (probe (position 17 39))
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::c"))) (kind subsetting) (ordinal 0) (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::a")))))
    )
  )
  (query (document "memory://snapshot/part_test.md") (range (start 18 11) (end 18 12)) (probe (position 18 11))
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::x"))) (kind featureTyping) (ordinal 0) (authored-target "C")
      (outcome (status resolved) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::C")))))
    )
  )
  (query (document "memory://snapshot/part_test.md") (range (start 44 12) (end 44 13)) (probe (position 44 12))
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::C::c1"))) (kind featureTyping) (ordinal 0) (authored-target "C")
      (outcome (status resolved) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::C")))))
    )
  )
  (query (document "memory://snapshot/part_test.md") (range (start 45 16) (end 45 17)) (probe (position 45 16))
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::C::c2"))) (kind featureTyping) (ordinal 0) (authored-target "C")
      (outcome (status resolved) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::C")))))
    )
  )
  (query (document "memory://snapshot/part_test.md") (range (start 2 9) (end 2 10)) (probe (position 2 9))
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::f"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A")))))
    )
  )
  (query (document "memory://snapshot/part_test.md") (range (start 48 15) (end 48 17)) (probe (position 48 15))
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p1"))) (kind subsetting) (ordinal 0) (authored-target "p2")
      (outcome (status resolved) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p2")))))
    )
  )
  (query (document "memory://snapshot/part_test.md") (range (start 49 15) (end 49 17)) (probe (position 49 15))
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p2"))) (kind subsetting) (ordinal 0) (authored-target "p3")
      (outcome (status resolved) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p3")))))
    )
  )
  (query (document "memory://snapshot/part_test.md") (range (start 50 15) (end 50 17)) (probe (position 50 15))
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p3"))) (kind subsetting) (ordinal 0) (authored-target "p1")
      (outcome (status resolved) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p1")))))
    )
  )
  (query (document "memory://snapshot/part_test.md") (range (start 52 15) (end 52 17)) (probe (position 52 15))
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p4"))) (kind subsetting) (ordinal 0) (authored-target "p4")
      (outcome (status resolved) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p4")))))
    )
  )
)
~~~
