# META
~~~ini
description=SysML Example (Simple Tests): ItemTest
type=file
~~~
# SOURCE
~~~sysml
package ItemTest {
	
	item f: A;

	public item def A {
		item b: B;
		protected ref part c: C;
	}
	
	abstract item def B {
		public abstract part a: A;
	}
	
	private part def C {
		private in ref y: A, B;
	}
	
	port def P {
		in item a1: A;
		out item a2: A;
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "item_test.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_part_def_body_element")
        (source "sysml")
        (range (start 14 2) (end 14 27))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package ItemTest {

    item f: A;

    public item def A {
        item b: B;
        protected ref part c: C;
    }

    abstract item def B {
        public abstract part a: A;
    }

    private part def C {
        private in ref y: A, B;
    }

    port def P {
        in item a1: A;
        out item a2: A;
    }

}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "8d33d0572df50297d3c46ebce554902b2c1d0072bc43d46237bee5a56322d011") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ItemTest"))) (kind "package") (name "ItemTest") (declared-name "ItemTest"))
    (element (id (node (document "d0") (qualified-name "ItemTest::A"))) (kind "item def") (name "A") (declared-name "A") (parent (node (document "d0") (qualified-name "ItemTest"))))
    (element (id (node (document "d0") (qualified-name "ItemTest::A::c"))) (kind "ref") (name "c") (declared-name "c") (parent (node (document "d0") (qualified-name "ItemTest::A"))) (authored (membership (kind Feature) (visibility "protected")) (relationships (typing (reference "C")))))
    (element (id (node (document "d0") (qualified-name "ItemTest::B"))) (kind "item def") (name "B") (declared-name "B") (parent (node (document "d0") (qualified-name "ItemTest"))))
    (element (id (node (document "d0") (qualified-name "ItemTest::B::a"))) (kind "part") (name "a") (declared-name "a") (parent (node (document "d0") (qualified-name "ItemTest::B"))) (authored (membership (kind Feature) (visibility "public")) (relationships (typing (reference "A")))))
    (element (id (node (document "d0") (qualified-name "ItemTest::C"))) (kind "part def") (name "C") (declared-name "C") (parent (node (document "d0") (qualified-name "ItemTest"))))
    (element (id (node (document "d0") (qualified-name "ItemTest::P"))) (kind "port def") (name "P") (declared-name "P") (parent (node (document "d0") (qualified-name "ItemTest"))))
    (element (id (node (document "d0") (qualified-name "ItemTest::P::a1"))) (kind "item") (name "a1") (declared-name "a1") (parent (node (document "d0") (qualified-name "ItemTest::P"))) (authored (membership (kind Feature)) (relationships (typing (reference "A")))))
    (element (id (node (document "d0") (qualified-name "ItemTest::P::a2"))) (kind "item") (name "a2") (declared-name "a2") (parent (node (document "d0") (qualified-name "ItemTest::P"))) (authored (membership (kind Feature)) (relationships (typing (reference "A")))))
    (element (id (node (document "d0") (qualified-name "ItemTest::P::~P"))) (kind "conjugated port definition") (name "~P") (declared-name "~P") (parent (node (document "d0") (qualified-name "ItemTest::P"))))
    (element (id (node (document "d0") (qualified-name "ItemTest::f"))) (kind "item def") (name "f") (declared-name "f") (parent (node (document "d0") (qualified-name "ItemTest"))) (authored (membership (kind Owning)) (relationships (specializes (reference "A")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ItemTest::A::c"))) (kind featureTyping) (ordinal 0)) (authored-target "C") (outcome (status resolved) (target (node (document "d0") (qualified-name "ItemTest::C")))))
    (reference (id (source (node (document "d0") (qualified-name "ItemTest::B::a"))) (kind featureTyping) (ordinal 0)) (authored-target "A") (outcome (status resolved) (target (node (document "d0") (qualified-name "ItemTest::A")))))
    (reference (id (source (node (document "d0") (qualified-name "ItemTest::P::a1"))) (kind featureTyping) (ordinal 0)) (authored-target "A") (outcome (status resolved) (target (node (document "d0") (qualified-name "ItemTest::A")))))
    (reference (id (source (node (document "d0") (qualified-name "ItemTest::P::a2"))) (kind featureTyping) (ordinal 0)) (authored-target "A") (outcome (status resolved) (target (node (document "d0") (qualified-name "ItemTest::A")))))
    (reference (id (source (node (document "d0") (qualified-name "ItemTest::f"))) (kind specialization) (ordinal 0)) (authored-target "A") (outcome (status resolved) (target (node (document "d0") (qualified-name "ItemTest::A")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ItemTest::A::c"))) (target (node (document "d0") (qualified-name "ItemTest::C"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ItemTest::A::c"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ItemTest::B::a"))) (target (node (document "d0") (qualified-name "ItemTest::A"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ItemTest::B::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ItemTest::P::a1"))) (target (node (document "d0") (qualified-name "ItemTest::A"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ItemTest::P::a1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ItemTest::P::a2"))) (target (node (document "d0") (qualified-name "ItemTest::A"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ItemTest::P::a2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ItemTest::f"))) (target (node (document "d0") (qualified-name "ItemTest::A"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ItemTest::f"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 0 0) (end 0 1)) (probe (position 0 0))
      (reference
        (source (document "d0") (qualified-name "ItemTest::f"))
        (kind specialization) (ordinal 0) (authored-target "A")
        (range (start 0 0) (end 0 1))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ItemTest::A") (range (start 4 1) (end 4 63)))
        )
      )
    )
    (query (range (start 6 24) (end 6 25)) (probe (position 6 24))
      (reference
        (source (document "d0") (qualified-name "ItemTest::A::c"))
        (kind featureTyping) (ordinal 0) (authored-target "C")
        (range (start 6 24) (end 6 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ItemTest::C") (range (start 13 1) (end 13 50)))
        )
      )
    )
    (query (range (start 10 26) (end 10 27)) (probe (position 10 26))
      (reference
        (source (document "d0") (qualified-name "ItemTest::B::a"))
        (kind featureTyping) (ordinal 0) (authored-target "A")
        (range (start 10 26) (end 10 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ItemTest::A") (range (start 4 1) (end 4 63)))
        )
      )
    )
  )
)
~~~
