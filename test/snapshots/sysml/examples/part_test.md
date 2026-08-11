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
  (document "part_test.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 8 2) (end 8 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 10) (end 9 31))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 22 2) (end 22 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 24 25) (end 24 39))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 37 2) (end 37 126))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 42 2) (end 42 20))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 43 2) (end 43 20))
      )
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "1ed283dc510ab825d7ea78a8ae36a21354caf52d6482c0f1da032b77aafd730e") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "PartTest"))) (kind "package") (name "PartTest") (declared-name "PartTest") (range (start (line 0) (character 0)) (end (line 0) (character 903))))
    (element (id (node (document "d0") (qualified-name "PartTest::A"))) (kind "part def") (name "A") (declared-name "A") (range (start (line 4) (character 1)) (end (line 4) (character 169))) (parent (node (document "d0") (qualified-name "PartTest"))))
    (element (id (node (document "d0") (qualified-name "PartTest::A::b"))) (kind "part") (name "b") (declared-name "b") (range (start (line 5) (character 2)) (end (line 5) (character 18))) (parent (node (document "d0") (qualified-name "PartTest::A"))) (authored (membership (kind Feature)) (relationships (typing (reference "B") (range (start (line 5) (character 16)) (end (line 5) (character 17)))))))
    (element (id (node (document "d0") (qualified-name "PartTest::A::c"))) (kind "port") (name "c") (declared-name "c") (range (start (line 6) (character 2)) (end (line 6) (character 22))) (parent (node (document "d0") (qualified-name "PartTest::A"))) (authored (membership (kind Feature) (visibility "protected")) (relationships (typing (reference "C") (range none)))))
    (element (id (node (document "d0") (qualified-name "PartTest::A::x"))) (kind "attribute") (name "x") (declared-name "x") (range (start (line 7) (character 2)) (end (line 7) (character 29))) (parent (node (document "d0") (qualified-name "PartTest::A"))))
    (element (id (node (document "d0") (qualified-name "PartTest::A::z"))) (kind "ref") (name "z") (declared-name "z") (range (start (line 9) (character 2)) (end (line 9) (character 32))) (parent (node (document "d0") (qualified-name "PartTest::A"))) (authored (membership (kind Feature)) (relationships (typing (reference "ScalarValues::Integer") (range (start (line 9) (character 10)) (end (line 9) (character 31)))))))
    (element (id (node (document "d0") (qualified-name "PartTest::B"))) (kind "part def") (name "B") (declared-name "B") (range (start (line 14) (character 1)) (end (line 14) (character 373))) (parent (node (document "d0") (qualified-name "PartTest"))) (authored (membership (kind Owning)) (relationships (perform (reference "PartTest::B::a2") (range none)))))
    (element (id (node (document "d0") (qualified-name "PartTest::B::a"))) (kind "part") (name "a") (declared-name "a") (range (start (line 15) (character 2)) (end (line 15) (character 34))) (parent (node (document "d0") (qualified-name "PartTest::B"))) (authored (membership (kind Feature) (visibility "public")) (relationships (typing (reference "A") (range (start (line 15) (character 26)) (end (line 15) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "PartTest::B::a1"))) (kind "action") (name "a1") (declared-name "a1") (range (start (line 26) (character 2)) (end (line 26) (character 59))) (parent (node (document "d0") (qualified-name "PartTest::B"))) (authored (membership (kind Feature)) (relationships (perform (reference "PartTest::B::a1::aa") (range none)))))
    (element (id (node (document "d0") (qualified-name "PartTest::B::a1::aa"))) (kind "action") (name "aa") (declared-name "aa") (range (start (line 28) (character 3)) (end (line 28) (character 13))) (parent (node (document "d0") (qualified-name "PartTest::B::a1"))))
    (element (id (node (document "d0") (qualified-name "PartTest::B::a2"))) (kind "action") (name "a2") (declared-name "a2") (range (start (line 30) (character 2)) (end (line 30) (character 20))) (parent (node (document "d0") (qualified-name "PartTest::B"))))
    (element (id (node (document "d0") (qualified-name "PartTest::B::b"))) (kind "part") (name "b") (declared-name "b") (range (start (line 16) (character 2)) (end (line 16) (character 35))) (parent (node (document "d0") (qualified-name "PartTest::B"))) (authored (membership (kind Feature) (visibility "public")) (relationships (subsetting (reference "a") (range (start (line 16) (character 33)) (end (line 16) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "PartTest::B::c"))) (kind "part") (name "c") (declared-name "c") (range (start (line 17) (character 2)) (end (line 17) (character 41))) (parent (node (document "d0") (qualified-name "PartTest::B"))) (authored (membership (kind Feature) (visibility "public")) (relationships (subsetting (reference "a") (range (start (line 17) (character 39)) (end (line 17) (character 40)))))))
    (element (id (node (document "d0") (qualified-name "PartTest::B::s1"))) (kind "state") (name "s1") (declared-name "s1") (range (start (line 32) (character 2)) (end (line 32) (character 11))) (parent (node (document "d0") (qualified-name "PartTest::B"))))
    (element (id (node (document "d0") (qualified-name "PartTest::B::s2"))) (kind "exhibit state") (name "s2") (declared-name "s2") (range (start (line 33) (character 2)) (end (line 33) (character 19))) (parent (node (document "d0") (qualified-name "PartTest::B"))))
    (element (id (node (document "d0") (qualified-name "PartTest::B::x"))) (kind "port") (name "x") (declared-name "x") (range (start (line 18) (character 2)) (end (line 18) (character 50))) (parent (node (document "d0") (qualified-name "PartTest::B"))) (authored (membership (kind Feature)) (relationships (typing (reference "~C") (range none)))))
    (element (id (node (document "d0") (qualified-name "PartTest::B::x::p"))) (kind "port") (name "p") (declared-name "p") (range (start (line 19) (character 6)) (end (line 19) (character 13))) (parent (node (document "d0") (qualified-name "PartTest::B::x"))))
    (element (id (node (document "d0") (qualified-name "PartTest::C"))) (kind "port def") (name "C") (declared-name "C") (range (start (line 36) (character 1)) (end (line 36) (character 218))) (parent (node (document "d0") (qualified-name "PartTest"))))
    (element (id (node (document "d0") (qualified-name "PartTest::C::c1"))) (kind "port") (name "c1") (declared-name "c1") (range (start (line 44) (character 2)) (end (line 44) (character 14))) (parent (node (document "d0") (qualified-name "PartTest::C"))) (authored (membership (kind Feature)) (relationships (typing (reference "C") (range none)))))
    (element (id (node (document "d0") (qualified-name "PartTest::C::~C"))) (kind "conjugated port definition") (name "~C") (declared-name "~C") (range (start (line 36) (character 1)) (end (line 36) (character 218))) (parent (node (document "d0") (qualified-name "PartTest::C"))))
    (element (id (node (document "d0") (qualified-name "PartTest::S"))) (kind "item def") (name "S") (declared-name "S") (range (start (line 12) (character 1)) (end (line 12) (character 12))) (parent (node (document "d0") (qualified-name "PartTest"))))
    (element (id (node (document "d0") (qualified-name "PartTest::f"))) (kind "part") (name "f") (declared-name "f") (range (start (line 2) (character 1)) (end (line 2) (character 11))) (parent (node (document "d0") (qualified-name "PartTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "A") (range (start (line 2) (character 9)) (end (line 2) (character 10)))))))
    (element (id (node (document "d0") (qualified-name "PartTest::p1"))) (kind "part") (name "p1") (declared-name "p1") (range (start (line 48) (character 4)) (end (line 48) (character 18))) (parent (node (document "d0") (qualified-name "PartTest"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "p2") (range (start (line 48) (character 15)) (end (line 48) (character 17)))))))
    (element (id (node (document "d0") (qualified-name "PartTest::p2"))) (kind "part") (name "p2") (declared-name "p2") (range (start (line 49) (character 4)) (end (line 49) (character 18))) (parent (node (document "d0") (qualified-name "PartTest"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "p3") (range (start (line 49) (character 15)) (end (line 49) (character 17)))))))
    (element (id (node (document "d0") (qualified-name "PartTest::p3"))) (kind "part") (name "p3") (declared-name "p3") (range (start (line 50) (character 4)) (end (line 50) (character 18))) (parent (node (document "d0") (qualified-name "PartTest"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "p1") (range (start (line 50) (character 15)) (end (line 50) (character 17)))))))
    (element (id (node (document "d0") (qualified-name "PartTest::p4"))) (kind "part") (name "p4") (declared-name "p4") (range (start (line 52) (character 4)) (end (line 52) (character 18))) (parent (node (document "d0") (qualified-name "PartTest"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "p4") (range (start (line 52) (character 15)) (end (line 52) (character 17)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "PartTest::A::b"))) (kind featureTyping) (ordinal 0)) (authored-target "B") (range (start (line 5) (character 16)) (end (line 5) (character 17))) (outcome (status resolved) (target (node (document "d0") (qualified-name "PartTest::B")))))
    (reference (id (source (node (document "d0") (qualified-name "PartTest::A::c"))) (kind featureTyping) (ordinal 0)) (authored-target "C") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "PartTest::C")))))
    (reference (id (source (node (document "d0") (qualified-name "PartTest::A::z"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarValues::Integer") (range (start (line 9) (character 10)) (end (line 9) (character 31))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "PartTest::B"))) (kind successionFlowSource) (ordinal 0)) (authored-target "x::p") (range (start (line 24) (character 18)) (end (line 24) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "PartTest::B::x::p")))))
    (reference (id (source (node (document "d0") (qualified-name "PartTest::B"))) (kind successionFlowTarget) (ordinal 0)) (authored-target "a1::aa::receiver") (range (start (line 24) (character 25)) (end (line 24) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "PartTest::B"))) (kind performSource) (ordinal 0)) (authored-target "PartTest::B::a2") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "PartTest::B::a2")))))
    (reference (id (source (node (document "d0") (qualified-name "PartTest::B::a"))) (kind featureTyping) (ordinal 0)) (authored-target "A") (range (start (line 15) (character 26)) (end (line 15) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "PartTest::A")))))
    (reference (id (source (node (document "d0") (qualified-name "PartTest::B::a1"))) (kind performSource) (ordinal 0)) (authored-target "PartTest::B::a1::aa") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "PartTest::B::a1::aa")))))
    (reference (id (source (node (document "d0") (qualified-name "PartTest::B::b"))) (kind subsetting) (ordinal 0)) (authored-target "a") (range (start (line 16) (character 33)) (end (line 16) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "PartTest::B::a")))))
    (reference (id (source (node (document "d0") (qualified-name "PartTest::B::c"))) (kind subsetting) (ordinal 0)) (authored-target "a") (range (start (line 17) (character 39)) (end (line 17) (character 40))) (outcome (status resolved) (target (node (document "d0") (qualified-name "PartTest::B::a")))))
    (reference (id (source (node (document "d0") (qualified-name "PartTest::B::x"))) (kind featureTyping) (ordinal 0)) (authored-target "~C") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "PartTest::C")))))
    (reference (id (source (node (document "d0") (qualified-name "PartTest::C::c1"))) (kind featureTyping) (ordinal 0)) (authored-target "C") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "PartTest::C")))))
    (reference (id (source (node (document "d0") (qualified-name "PartTest::f"))) (kind featureTyping) (ordinal 0)) (authored-target "A") (range (start (line 2) (character 9)) (end (line 2) (character 10))) (outcome (status resolved) (target (node (document "d0") (qualified-name "PartTest::A")))))
    (reference (id (source (node (document "d0") (qualified-name "PartTest::p1"))) (kind subsetting) (ordinal 0)) (authored-target "p2") (range (start (line 48) (character 15)) (end (line 48) (character 17))) (outcome (status resolved) (target (node (document "d0") (qualified-name "PartTest::p2")))))
    (reference (id (source (node (document "d0") (qualified-name "PartTest::p2"))) (kind subsetting) (ordinal 0)) (authored-target "p3") (range (start (line 49) (character 15)) (end (line 49) (character 17))) (outcome (status resolved) (target (node (document "d0") (qualified-name "PartTest::p3")))))
    (reference (id (source (node (document "d0") (qualified-name "PartTest::p3"))) (kind subsetting) (ordinal 0)) (authored-target "p1") (range (start (line 50) (character 15)) (end (line 50) (character 17))) (outcome (status resolved) (target (node (document "d0") (qualified-name "PartTest::p1")))))
    (reference (id (source (node (document "d0") (qualified-name "PartTest::p4"))) (kind subsetting) (ordinal 0)) (authored-target "p4") (range (start (line 52) (character 15)) (end (line 52) (character 17))) (outcome (status resolved) (target (node (document "d0") (qualified-name "PartTest::p4")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "PartTest::A::b"))) (target (node (document "d0") (qualified-name "PartTest::B"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "PartTest::A::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "PartTest::A::c"))) (target (node (document "d0") (qualified-name "PartTest::C"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "PartTest::A::c"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "PartTest::B"))) (target (node (document "d0") (qualified-name "PartTest::B::a2"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "PartTest::B"))) (kind performSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "PartTest::B::a"))) (target (node (document "d0") (qualified-name "PartTest::A"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "PartTest::B::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "PartTest::B::a1"))) (target (node (document "d0") (qualified-name "PartTest::B::a1::aa"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "PartTest::B::a1"))) (kind performSource) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "PartTest::B::b"))) (target (node (document "d0") (qualified-name "PartTest::B::a"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "PartTest::B::b"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "PartTest::B::c"))) (target (node (document "d0") (qualified-name "PartTest::B::a"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "PartTest::B::c"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "PartTest::B::x"))) (target (node (document "d0") (qualified-name "PartTest::C"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "PartTest::B::x"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "PartTest::C::c1"))) (target (node (document "d0") (qualified-name "PartTest::C"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "PartTest::C::c1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "PartTest::f"))) (target (node (document "d0") (qualified-name "PartTest::A"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "PartTest::f"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "PartTest::p1"))) (target (node (document "d0") (qualified-name "PartTest::p2"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "PartTest::p1"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "PartTest::p2"))) (target (node (document "d0") (qualified-name "PartTest::p3"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "PartTest::p2"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "PartTest::p3"))) (target (node (document "d0") (qualified-name "PartTest::p1"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "PartTest::p3"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "PartTest::p4"))) (target (node (document "d0") (qualified-name "PartTest::p4"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "PartTest::p4"))) (kind subsetting) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
