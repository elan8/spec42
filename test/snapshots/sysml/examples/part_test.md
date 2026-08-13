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
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 6 2) (end 6 22))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 8 38) (end 8 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 9 2) (end 9 32))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 12 1) (end 12 12))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 16 33) (end 16 34))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 17 39) (end 17 40))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 18 2) (end 21 3))
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
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 26 2) (end 29 3))
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
        (range (start 30 2) (end 30 20))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 32 2) (end 32 11))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 33 2) (end 33 19))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 36 1) (end 46 2))
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
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 48 15) (end 48 17))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 49 15) (end 49 17))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 50 15) (end 50 17))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 52 15) (end 52 17))
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
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A::b"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "B"))))
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A::x"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A::y"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "x"))))
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::a"))) (kind part) (membership (kind feature) (visibility public)) (authored (membership (kind feature) (visibility public)) (relationships (featureTyping (reference "A"))))
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::b"))) (kind part) (membership (kind feature) (visibility public)) (authored (membership (kind feature) (visibility public)) (relationships (subsetting (reference "a"))))
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::c"))) (kind part) (membership (kind feature) (visibility public)) (authored (membership (kind feature) (visibility public)) (relationships (subsetting (reference "a"))))
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::f"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A"))))
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "p2"))))
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p2"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "p3"))))
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p3"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "p1"))))
    (declaration (id (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p4"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "p4"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A::b"))) (kind featureTyping) (ordinal 0))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B")))))
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A::y"))) (kind subsetting) (ordinal 0))
      (authored-target "x")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A")))))
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::b"))) (kind subsetting) (ordinal 0))
      (authored-target "a")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::c"))) (kind subsetting) (ordinal 0))
      (authored-target "a")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::f"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A")))))
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p1"))) (kind subsetting) (ordinal 0))
      (authored-target "p2")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p2"))) (kind subsetting) (ordinal 0))
      (authored-target "p3")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p3"))) (kind subsetting) (ordinal 0))
      (authored-target "p1")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p4"))) (kind subsetting) (ordinal 0))
      (authored-target "p4")
      (outcome (status unsupported)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A::b"))) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::a"))) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::f"))) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::f"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
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
  (query (document "memory://snapshot/part_test.md") (range (start 8 38) (end 8 39)) (probe (position 8 38))
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A::y"))) (kind subsetting) (ordinal 0) (authored-target "x")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/part_test.md") (range (start 15 26) (end 15 27)) (probe (position 15 26))
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::a"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A")))))
  )
  (query (document "memory://snapshot/part_test.md") (range (start 16 33) (end 16 34)) (probe (position 16 33))
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::b"))) (kind subsetting) (ordinal 0) (authored-target "a")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/part_test.md") (range (start 17 39) (end 17 40)) (probe (position 17 39))
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::B::c"))) (kind subsetting) (ordinal 0) (authored-target "a")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/part_test.md") (range (start 2 9) (end 2 10)) (probe (position 2 9))
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::f"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::A")))))
  )
  (query (document "memory://snapshot/part_test.md") (range (start 48 15) (end 48 17)) (probe (position 48 15))
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p1"))) (kind subsetting) (ordinal 0) (authored-target "p2")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/part_test.md") (range (start 49 15) (end 49 17)) (probe (position 49 15))
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p2"))) (kind subsetting) (ordinal 0) (authored-target "p3")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/part_test.md") (range (start 50 15) (end 50 17)) (probe (position 50 15))
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p3"))) (kind subsetting) (ordinal 0) (authored-target "p1")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/part_test.md") (range (start 52 15) (end 52 17)) (probe (position 52 15))
    (reference (id (source (node (document "memory://snapshot/part_test.md") (qualified-name "PartTest::p4"))) (kind subsetting) (ordinal 0) (authored-target "p4")
      (outcome (status unsupported)))
  )
)
~~~
