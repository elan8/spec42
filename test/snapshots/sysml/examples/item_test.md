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
  (document "memory://snapshot/item_test.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:17aff5b295c11e4a8c2444e4e7abc4f4b80c26d1eb872ab5bb3bb67aca633c68") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::A"))) (kind item-def) (membership (kind owning) (visibility public)))
    (declaration (id (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::A::b"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "B"))))
    (declaration (id (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::A::c"))) (kind ref) (membership (kind feature) (visibility protected)) (authored (membership (kind feature) (visibility protected)) (relationships (featureTyping (reference "C"))))
    (declaration (id (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::B"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::B::a"))) (kind part) (membership (kind feature) (visibility public)) (facts (modifiers abstract)) (authored (membership (kind feature) (visibility public)) (relationships (featureTyping (reference "A"))))
    (declaration (id (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::C"))) (kind part-def) (membership (kind owning) (visibility private)))
    (declaration (id (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::C::y"))) (kind ref) (membership (kind feature) (visibility private)) (facts (direction in)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "A")) (featureTyping (reference "B"))))
    (declaration (id (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::P"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::P::a1"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A"))))
    (declaration (id (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::P::a2"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A"))))
    (declaration (id (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::f"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::A::b"))) (kind featureTyping) (ordinal 0))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::B")))))
    (reference (id (source (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::A::c"))) (kind featureTyping) (ordinal 0))
      (authored-target "C")
      (outcome (status resolved) (target (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::C")))))
    (reference (id (source (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::B::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::A")))))
    (reference (id (source (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::C::y"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::A")))))
    (reference (id (source (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::C::y"))) (kind featureTyping) (ordinal 1))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::B")))))
    (reference (id (source (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::P::a1"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::A")))))
    (reference (id (source (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::P::a2"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::A")))))
    (reference (id (source (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::f"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::A")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::A::b"))) (target (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::A::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::A::c"))) (target (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::C"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::A::c"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::B::a"))) (target (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::B::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::C::y"))) (target (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::C::y"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::C::y"))) (target (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::C::y"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::P::a1"))) (target (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::P::a1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::P::a2"))) (target (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::P::a2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::f"))) (target (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::f"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/item_test.md") (range (start 5 10) (end 5 11)) (probe (position 5 10))
    (reference (id (source (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::A::b"))) (kind featureTyping) (ordinal 0) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::B")))))
  )
  (query (document "memory://snapshot/item_test.md") (range (start 6 24) (end 6 25)) (probe (position 6 24))
    (reference (id (source (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::A::c"))) (kind featureTyping) (ordinal 0) (authored-target "C")
      (outcome (status resolved) (target (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::C")))))
  )
  (query (document "memory://snapshot/item_test.md") (range (start 10 26) (end 10 27)) (probe (position 10 26))
    (reference (id (source (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::B::a"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::A")))))
  )
  (query (document "memory://snapshot/item_test.md") (range (start 14 20) (end 14 21)) (probe (position 14 20))
    (reference (id (source (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::C::y"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::A")))))
  )
  (query (document "memory://snapshot/item_test.md") (range (start 14 23) (end 14 24)) (probe (position 14 23))
    (reference (id (source (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::C::y"))) (kind featureTyping) (ordinal 1) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::B")))))
  )
  (query (document "memory://snapshot/item_test.md") (range (start 18 14) (end 18 15)) (probe (position 18 14))
    (reference (id (source (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::P::a1"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::A")))))
  )
  (query (document "memory://snapshot/item_test.md") (range (start 19 15) (end 19 16)) (probe (position 19 15))
    (reference (id (source (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::P::a2"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::A")))))
  )
  (query (document "memory://snapshot/item_test.md") (range (start 2 9) (end 2 10)) (probe (position 2 9))
    (reference (id (source (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::f"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/item_test.md") (qualified-name "ItemTest::A")))))
  )
)
~~~
