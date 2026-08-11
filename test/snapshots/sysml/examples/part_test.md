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
    (element (id (node (document "d0") (qualified-name "PartTest"))) (kind "package") (name "PartTest") (declared-name "PartTest"))
    (element (id (node (document "d0") (qualified-name "PartTest::A"))) (kind "part def") (name "A") (declared-name "A") (parent (node (document "d0") (qualified-name "PartTest"))))
    (element (id (node (document "d0") (qualified-name "PartTest::A::b"))) (kind "part") (name "b") (declared-name "b") (parent (node (document "d0") (qualified-name "PartTest::A"))) (authored (membership (kind Feature)) (relationships (typing (reference "B")))))
    (element (id (node (document "d0") (qualified-name "PartTest::A::c"))) (kind "port") (name "c") (declared-name "c") (parent (node (document "d0") (qualified-name "PartTest::A"))) (authored (membership (kind Feature) (visibility "protected")) (relationships (typing (reference "C")))))
    (element (id (node (document "d0") (qualified-name "PartTest::A::x"))) (kind "attribute") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "PartTest::A"))))
    (element (id (node (document "d0") (qualified-name "PartTest::A::z"))) (kind "ref") (name "z") (declared-name "z") (parent (node (document "d0") (qualified-name "PartTest::A"))) (authored (membership (kind Feature)) (relationships (typing (reference "ScalarValues::Integer")))))
    (element (id (node (document "d0") (qualified-name "PartTest::B"))) (kind "part def") (name "B") (declared-name "B") (parent (node (document "d0") (qualified-name "PartTest"))) (authored (membership (kind Owning)) (relationships (perform (reference "PartTest::B::a2")))))
    (element (id (node (document "d0") (qualified-name "PartTest::B::a"))) (kind "part") (name "a") (declared-name "a") (parent (node (document "d0") (qualified-name "PartTest::B"))) (authored (membership (kind Feature) (visibility "public")) (relationships (typing (reference "A")))))
    (element (id (node (document "d0") (qualified-name "PartTest::B::a1"))) (kind "action") (name "a1") (declared-name "a1") (parent (node (document "d0") (qualified-name "PartTest::B"))) (authored (membership (kind Feature)) (relationships (perform (reference "PartTest::B::a1::aa")))))
    (element (id (node (document "d0") (qualified-name "PartTest::B::a1::aa"))) (kind "action") (name "aa") (declared-name "aa") (parent (node (document "d0") (qualified-name "PartTest::B::a1"))))
    (element (id (node (document "d0") (qualified-name "PartTest::B::a2"))) (kind "action") (name "a2") (declared-name "a2") (parent (node (document "d0") (qualified-name "PartTest::B"))))
    (element (id (node (document "d0") (qualified-name "PartTest::B::b"))) (kind "part") (name "b") (declared-name "b") (parent (node (document "d0") (qualified-name "PartTest::B"))) (authored (membership (kind Feature) (visibility "public")) (relationships (subsetting (reference "a")))))
    (element (id (node (document "d0") (qualified-name "PartTest::B::c"))) (kind "part") (name "c") (declared-name "c") (parent (node (document "d0") (qualified-name "PartTest::B"))) (authored (membership (kind Feature) (visibility "public")) (relationships (subsetting (reference "a")))))
    (element (id (node (document "d0") (qualified-name "PartTest::B::s1"))) (kind "state") (name "s1") (declared-name "s1") (parent (node (document "d0") (qualified-name "PartTest::B"))))
    (element (id (node (document "d0") (qualified-name "PartTest::B::s2"))) (kind "exhibit state") (name "s2") (declared-name "s2") (parent (node (document "d0") (qualified-name "PartTest::B"))))
    (element (id (node (document "d0") (qualified-name "PartTest::B::x"))) (kind "port") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "PartTest::B"))) (authored (membership (kind Feature)) (relationships (typing (reference "~C")))))
    (element (id (node (document "d0") (qualified-name "PartTest::B::x::p"))) (kind "port") (name "p") (declared-name "p") (parent (node (document "d0") (qualified-name "PartTest::B::x"))))
    (element (id (node (document "d0") (qualified-name "PartTest::C"))) (kind "port def") (name "C") (declared-name "C") (parent (node (document "d0") (qualified-name "PartTest"))))
    (element (id (node (document "d0") (qualified-name "PartTest::C::c1"))) (kind "port") (name "c1") (declared-name "c1") (parent (node (document "d0") (qualified-name "PartTest::C"))) (authored (membership (kind Feature)) (relationships (typing (reference "C")))))
    (element (id (node (document "d0") (qualified-name "PartTest::C::~C"))) (kind "conjugated port definition") (name "~C") (declared-name "~C") (parent (node (document "d0") (qualified-name "PartTest::C"))))
    (element (id (node (document "d0") (qualified-name "PartTest::S"))) (kind "item def") (name "S") (declared-name "S") (parent (node (document "d0") (qualified-name "PartTest"))))
    (element (id (node (document "d0") (qualified-name "PartTest::f"))) (kind "part") (name "f") (declared-name "f") (parent (node (document "d0") (qualified-name "PartTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "A")))))
    (element (id (node (document "d0") (qualified-name "PartTest::p1"))) (kind "part") (name "p1") (declared-name "p1") (parent (node (document "d0") (qualified-name "PartTest"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "p2")))))
    (element (id (node (document "d0") (qualified-name "PartTest::p2"))) (kind "part") (name "p2") (declared-name "p2") (parent (node (document "d0") (qualified-name "PartTest"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "p3")))))
    (element (id (node (document "d0") (qualified-name "PartTest::p3"))) (kind "part") (name "p3") (declared-name "p3") (parent (node (document "d0") (qualified-name "PartTest"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "p1")))))
    (element (id (node (document "d0") (qualified-name "PartTest::p4"))) (kind "part") (name "p4") (declared-name "p4") (parent (node (document "d0") (qualified-name "PartTest"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "p4")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "PartTest::A::b"))) (kind featureTyping) (ordinal 0)) (authored-target "B") (outcome (status resolved) (target (node (document "d0") (qualified-name "PartTest::B")))))
    (reference (id (source (node (document "d0") (qualified-name "PartTest::A::c"))) (kind featureTyping) (ordinal 0)) (authored-target "C") (outcome (status resolved) (target (node (document "d0") (qualified-name "PartTest::C")))))
    (reference (id (source (node (document "d0") (qualified-name "PartTest::A::z"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarValues::Integer") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "PartTest::B"))) (kind successionFlowSource) (ordinal 0)) (authored-target "x::p") (outcome (status resolved) (target (node (document "d0") (qualified-name "PartTest::B::x::p")))))
    (reference (id (source (node (document "d0") (qualified-name "PartTest::B"))) (kind successionFlowTarget) (ordinal 0)) (authored-target "a1::aa::receiver") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "PartTest::B"))) (kind performSource) (ordinal 0)) (authored-target "PartTest::B::a2") (outcome (status resolved) (target (node (document "d0") (qualified-name "PartTest::B::a2")))))
    (reference (id (source (node (document "d0") (qualified-name "PartTest::B::a"))) (kind featureTyping) (ordinal 0)) (authored-target "A") (outcome (status resolved) (target (node (document "d0") (qualified-name "PartTest::A")))))
    (reference (id (source (node (document "d0") (qualified-name "PartTest::B::a1"))) (kind performSource) (ordinal 0)) (authored-target "PartTest::B::a1::aa") (outcome (status resolved) (target (node (document "d0") (qualified-name "PartTest::B::a1::aa")))))
    (reference (id (source (node (document "d0") (qualified-name "PartTest::B::b"))) (kind subsetting) (ordinal 0)) (authored-target "a") (outcome (status resolved) (target (node (document "d0") (qualified-name "PartTest::B::a")))))
    (reference (id (source (node (document "d0") (qualified-name "PartTest::B::c"))) (kind subsetting) (ordinal 0)) (authored-target "a") (outcome (status resolved) (target (node (document "d0") (qualified-name "PartTest::B::a")))))
    (reference (id (source (node (document "d0") (qualified-name "PartTest::B::x"))) (kind featureTyping) (ordinal 0)) (authored-target "~C") (outcome (status resolved) (target (node (document "d0") (qualified-name "PartTest::C")))))
    (reference (id (source (node (document "d0") (qualified-name "PartTest::C::c1"))) (kind featureTyping) (ordinal 0)) (authored-target "C") (outcome (status resolved) (target (node (document "d0") (qualified-name "PartTest::C")))))
    (reference (id (source (node (document "d0") (qualified-name "PartTest::f"))) (kind featureTyping) (ordinal 0)) (authored-target "A") (outcome (status resolved) (target (node (document "d0") (qualified-name "PartTest::A")))))
    (reference (id (source (node (document "d0") (qualified-name "PartTest::p1"))) (kind subsetting) (ordinal 0)) (authored-target "p2") (outcome (status resolved) (target (node (document "d0") (qualified-name "PartTest::p2")))))
    (reference (id (source (node (document "d0") (qualified-name "PartTest::p2"))) (kind subsetting) (ordinal 0)) (authored-target "p3") (outcome (status resolved) (target (node (document "d0") (qualified-name "PartTest::p3")))))
    (reference (id (source (node (document "d0") (qualified-name "PartTest::p3"))) (kind subsetting) (ordinal 0)) (authored-target "p1") (outcome (status resolved) (target (node (document "d0") (qualified-name "PartTest::p1")))))
    (reference (id (source (node (document "d0") (qualified-name "PartTest::p4"))) (kind subsetting) (ordinal 0)) (authored-target "p4") (outcome (status resolved) (target (node (document "d0") (qualified-name "PartTest::p4")))))
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
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 2 9) (end 2 10)) (probe (position 2 9))
      (reference
        (source (document "d0") (qualified-name "PartTest::f"))
        (kind featureTyping) (ordinal 0) (authored-target "A")
        (range (start 2 9) (end 2 10))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "PartTest::A") (range (start 4 1) (end 4 169)))
        )
      )
    )
    (query (range (start 5 16) (end 5 17)) (probe (position 5 16))
      (reference
        (source (document "d0") (qualified-name "PartTest::A::b"))
        (kind featureTyping) (ordinal 0) (authored-target "B")
        (range (start 5 16) (end 5 17))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "PartTest::B") (range (start 14 1) (end 14 373)))
        )
      )
    )
    (query (range (start 15 26) (end 15 27)) (probe (position 15 26))
      (reference
        (source (document "d0") (qualified-name "PartTest::B::a"))
        (kind featureTyping) (ordinal 0) (authored-target "A")
        (range (start 15 26) (end 15 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "PartTest::A") (range (start 4 1) (end 4 169)))
        )
      )
    )
    (query (range (start 16 33) (end 16 34)) (probe (position 16 33))
      (reference
        (source (document "d0") (qualified-name "PartTest::B::b"))
        (kind subsetting) (ordinal 0) (authored-target "a")
        (range (start 16 33) (end 16 34))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "PartTest::B::a") (range (start 15 2) (end 15 34)))
        )
      )
    )
    (query (range (start 17 39) (end 17 40)) (probe (position 17 39))
      (reference
        (source (document "d0") (qualified-name "PartTest::B::c"))
        (kind subsetting) (ordinal 0) (authored-target "a")
        (range (start 17 39) (end 17 40))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "PartTest::B::a") (range (start 15 2) (end 15 34)))
        )
      )
    )
    (query (range (start 48 15) (end 48 17)) (probe (position 48 15))
      (reference
        (source (document "d0") (qualified-name "PartTest::p1"))
        (kind subsetting) (ordinal 0) (authored-target "p2")
        (range (start 48 15) (end 48 17))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "PartTest::p2") (range (start 49 4) (end 49 18)))
        )
      )
    )
    (query (range (start 49 15) (end 49 17)) (probe (position 49 15))
      (reference
        (source (document "d0") (qualified-name "PartTest::p2"))
        (kind subsetting) (ordinal 0) (authored-target "p3")
        (range (start 49 15) (end 49 17))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "PartTest::p3") (range (start 50 4) (end 50 18)))
        )
      )
    )
    (query (range (start 50 15) (end 50 17)) (probe (position 50 15))
      (reference
        (source (document "d0") (qualified-name "PartTest::p3"))
        (kind subsetting) (ordinal 0) (authored-target "p1")
        (range (start 50 15) (end 50 17))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "PartTest::p1") (range (start 48 4) (end 48 18)))
        )
      )
    )
    (query (range (start 52 15) (end 52 17)) (probe (position 52 15))
      (reference
        (source (document "d0") (qualified-name "PartTest::p4"))
        (kind subsetting) (ordinal 0) (authored-target "p4")
        (range (start 52 15) (end 52 17))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "PartTest::p4") (range (start 52 4) (end 52 18)))
        )
      )
    )
    (query (range (start 24 18) (end 24 21)) (probe (position 24 18))
      (reference
        (source (document "d0") (qualified-name "PartTest::B"))
        (kind successionFlowSource) (ordinal 0) (authored-target "x::p")
        (range (start 24 18) (end 24 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "PartTest::B::x::p") (range (start 19 6) (end 19 13)))
        )
      )
    )
    (query (range (start 24 25) (end 24 39)) (probe (position 24 25))
      (reference
        (source (document "d0") (qualified-name "PartTest::B"))
        (kind successionFlowTarget) (ordinal 0) (authored-target "a1::aa::receiver")
        (range (start 24 25) (end 24 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 10) (end 9 31)) (probe (position 9 10))
      (reference
        (source (document "d0") (qualified-name "PartTest::A::z"))
        (kind featureTyping) (ordinal 0) (authored-target "ScalarValues::Integer")
        (range (start 9 10) (end 9 31))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
