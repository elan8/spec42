# META
~~~ini
description=SysML Example (Simple Tests): ConjugationTest
type=file
~~~
# SOURCE
~~~sysml
package ConjugationTest {
	port def P;
	
	part def B {
		port p1: P;
		port p2: ~P;
	}
	
	connection def A {
		end port p1: P;
		end port p2: ~P;
	}
	
	interface def I {
		end p1: P;
		end p2: ~P;
	}
	
	part def B1 {
		part p {
			port p1: P;
			port p2: ~P;		
		}
	
		connection a: A {
			end port p3: P ::> p.p1;
			end port p4: ~P ::> p.p2;
		}
		interface i: I {
			end port p3: P ::> p.p1;
			end port p4: ~P ::> p.p2;
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "conjugation_test.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "bf85d66eeb20581e744c9bf6318694dea0c67ee386be50bab69e89618b664326") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ConjugationTest"))) (kind "package") (name "ConjugationTest") (declared-name "ConjugationTest"))
    (element (id (node (document "d0") (qualified-name "ConjugationTest::A"))) (kind "connection def") (name "A") (declared-name "A") (parent (node (document "d0") (qualified-name "ConjugationTest"))))
    (element (id (node (document "d0") (qualified-name "ConjugationTest::A::p1"))) (kind "interface end") (name "p1") (declared-name "p1") (parent (node (document "d0") (qualified-name "ConjugationTest::A"))) (authored (relationships (typing (reference "P")))))
    (element (id (node (document "d0") (qualified-name "ConjugationTest::A::p2"))) (kind "interface end") (name "p2") (declared-name "p2") (parent (node (document "d0") (qualified-name "ConjugationTest::A"))) (authored (relationships (typing (reference "~P")))))
    (element (id (node (document "d0") (qualified-name "ConjugationTest::B"))) (kind "part def") (name "B") (declared-name "B") (parent (node (document "d0") (qualified-name "ConjugationTest"))))
    (element (id (node (document "d0") (qualified-name "ConjugationTest::B1"))) (kind "part def") (name "B1") (declared-name "B1") (parent (node (document "d0") (qualified-name "ConjugationTest"))))
    (element (id (node (document "d0") (qualified-name "ConjugationTest::B1::a"))) (kind "connection") (name "a") (declared-name "a") (parent (node (document "d0") (qualified-name "ConjugationTest::B1"))) (authored (membership (kind Feature)) (relationships (typing (reference "A")))))
    (element (id (node (document "d0") (qualified-name "ConjugationTest::B1::a::p3"))) (kind "interface end") (name "p3") (declared-name "p3") (parent (node (document "d0") (qualified-name "ConjugationTest::B1::a"))) (authored (relationships (typing (reference "P")) (reference-subsetting (reference "p.p1")))))
    (element (id (node (document "d0") (qualified-name "ConjugationTest::B1::a::p4"))) (kind "interface end") (name "p4") (declared-name "p4") (parent (node (document "d0") (qualified-name "ConjugationTest::B1::a"))) (authored (relationships (typing (reference "~P")) (reference-subsetting (reference "p.p2")))))
    (element (id (node (document "d0") (qualified-name "ConjugationTest::B1::p"))) (kind "part") (name "p") (declared-name "p") (parent (node (document "d0") (qualified-name "ConjugationTest::B1"))))
    (element (id (node (document "d0") (qualified-name "ConjugationTest::B1::p::p1"))) (kind "port") (name "p1") (declared-name "p1") (parent (node (document "d0") (qualified-name "ConjugationTest::B1::p"))) (authored (membership (kind Feature)) (relationships (typing (reference "P")))))
    (element (id (node (document "d0") (qualified-name "ConjugationTest::B1::p::p2"))) (kind "port") (name "p2") (declared-name "p2") (parent (node (document "d0") (qualified-name "ConjugationTest::B1::p"))) (authored (membership (kind Feature)) (relationships (typing (reference "~P")))))
    (element (id (node (document "d0") (qualified-name "ConjugationTest::B::p1"))) (kind "port") (name "p1") (declared-name "p1") (parent (node (document "d0") (qualified-name "ConjugationTest::B"))) (authored (membership (kind Feature)) (relationships (typing (reference "P")))))
    (element (id (node (document "d0") (qualified-name "ConjugationTest::B::p2"))) (kind "port") (name "p2") (declared-name "p2") (parent (node (document "d0") (qualified-name "ConjugationTest::B"))) (authored (membership (kind Feature)) (relationships (typing (reference "~P")))))
    (element (id (node (document "d0") (qualified-name "ConjugationTest::I"))) (kind "interface def") (name "I") (declared-name "I") (parent (node (document "d0") (qualified-name "ConjugationTest"))))
    (element (id (node (document "d0") (qualified-name "ConjugationTest::I::p1"))) (kind "interface end") (name "p1") (declared-name "p1") (parent (node (document "d0") (qualified-name "ConjugationTest::I"))) (authored (relationships (typing (reference "P")))))
    (element (id (node (document "d0") (qualified-name "ConjugationTest::I::p2"))) (kind "interface end") (name "p2") (declared-name "p2") (parent (node (document "d0") (qualified-name "ConjugationTest::I"))) (authored (relationships (typing (reference "~P")))))
    (element (id (node (document "d0") (qualified-name "ConjugationTest::P"))) (kind "port def") (name "P") (declared-name "P") (parent (node (document "d0") (qualified-name "ConjugationTest"))))
    (element (id (node (document "d0") (qualified-name "ConjugationTest::P::~P"))) (kind "conjugated port definition") (name "~P") (declared-name "~P") (parent (node (document "d0") (qualified-name "ConjugationTest::P"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ConjugationTest::A::p1"))) (kind featureTyping) (ordinal 0)) (authored-target "P") (outcome (status resolved) (target (node (document "d0") (qualified-name "ConjugationTest::P")))))
    (reference (id (source (node (document "d0") (qualified-name "ConjugationTest::A::p2"))) (kind featureTyping) (ordinal 0)) (authored-target "~P") (outcome (status resolved) (target (node (document "d0") (qualified-name "ConjugationTest::P")))))
    (reference (id (source (node (document "d0") (qualified-name "ConjugationTest::B1::a"))) (kind featureTyping) (ordinal 0)) (authored-target "A") (outcome (status resolved) (target (node (document "d0") (qualified-name "ConjugationTest::A")))))
    (reference (id (source (node (document "d0") (qualified-name "ConjugationTest::B1::a::p3"))) (kind featureTyping) (ordinal 0)) (authored-target "P") (outcome (status resolved) (target (node (document "d0") (qualified-name "ConjugationTest::P")))))
    (reference (id (source (node (document "d0") (qualified-name "ConjugationTest::B1::a::p3"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "p.p1") (outcome (status resolved) (target (node (document "d0") (qualified-name "ConjugationTest::B1::p::p1")))))
    (reference (id (source (node (document "d0") (qualified-name "ConjugationTest::B1::a::p4"))) (kind featureTyping) (ordinal 0)) (authored-target "~P") (outcome (status resolved) (target (node (document "d0") (qualified-name "ConjugationTest::P")))))
    (reference (id (source (node (document "d0") (qualified-name "ConjugationTest::B1::a::p4"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "p.p2") (outcome (status resolved) (target (node (document "d0") (qualified-name "ConjugationTest::B1::p::p2")))))
    (reference (id (source (node (document "d0") (qualified-name "ConjugationTest::B1::p::p1"))) (kind featureTyping) (ordinal 0)) (authored-target "P") (outcome (status resolved) (target (node (document "d0") (qualified-name "ConjugationTest::P")))))
    (reference (id (source (node (document "d0") (qualified-name "ConjugationTest::B1::p::p2"))) (kind featureTyping) (ordinal 0)) (authored-target "~P") (outcome (status resolved) (target (node (document "d0") (qualified-name "ConjugationTest::P")))))
    (reference (id (source (node (document "d0") (qualified-name "ConjugationTest::B::p1"))) (kind featureTyping) (ordinal 0)) (authored-target "P") (outcome (status resolved) (target (node (document "d0") (qualified-name "ConjugationTest::P")))))
    (reference (id (source (node (document "d0") (qualified-name "ConjugationTest::B::p2"))) (kind featureTyping) (ordinal 0)) (authored-target "~P") (outcome (status resolved) (target (node (document "d0") (qualified-name "ConjugationTest::P")))))
    (reference (id (source (node (document "d0") (qualified-name "ConjugationTest::I::p1"))) (kind featureTyping) (ordinal 0)) (authored-target "P") (outcome (status resolved) (target (node (document "d0") (qualified-name "ConjugationTest::P")))))
    (reference (id (source (node (document "d0") (qualified-name "ConjugationTest::I::p2"))) (kind featureTyping) (ordinal 0)) (authored-target "~P") (outcome (status resolved) (target (node (document "d0") (qualified-name "ConjugationTest::P")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ConjugationTest::A::p1"))) (target (node (document "d0") (qualified-name "ConjugationTest::P"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConjugationTest::A::p1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ConjugationTest::A::p2"))) (target (node (document "d0") (qualified-name "ConjugationTest::P"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConjugationTest::A::p2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ConjugationTest::B1::a"))) (target (node (document "d0") (qualified-name "ConjugationTest::A"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConjugationTest::B1::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ConjugationTest::B1::a::p3"))) (target (node (document "d0") (qualified-name "ConjugationTest::P"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConjugationTest::B1::a::p3"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "ConjugationTest::B1::a::p3"))) (target (node (document "d0") (qualified-name "ConjugationTest::B1::p::p1"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConjugationTest::B1::a::p3"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ConjugationTest::B1::a::p4"))) (target (node (document "d0") (qualified-name "ConjugationTest::P"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConjugationTest::B1::a::p4"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "ConjugationTest::B1::a::p4"))) (target (node (document "d0") (qualified-name "ConjugationTest::B1::p::p2"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConjugationTest::B1::a::p4"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ConjugationTest::B1::p::p1"))) (target (node (document "d0") (qualified-name "ConjugationTest::P"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConjugationTest::B1::p::p1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ConjugationTest::B1::p::p2"))) (target (node (document "d0") (qualified-name "ConjugationTest::P"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConjugationTest::B1::p::p2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ConjugationTest::B::p1"))) (target (node (document "d0") (qualified-name "ConjugationTest::P"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConjugationTest::B::p1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ConjugationTest::B::p2"))) (target (node (document "d0") (qualified-name "ConjugationTest::P"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConjugationTest::B::p2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ConjugationTest::I::p1"))) (target (node (document "d0") (qualified-name "ConjugationTest::P"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConjugationTest::I::p1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ConjugationTest::I::p2"))) (target (node (document "d0") (qualified-name "ConjugationTest::P"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConjugationTest::I::p2"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 25 22) (end 25 26)) (probe (position 25 22))
      (reference
        (source (document "d0") (qualified-name "ConjugationTest::B1::a::p3"))
        (kind referenceSubsetting) (ordinal 0) (authored-target "p.p1")
        (range (start 25 22) (end 25 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ConjugationTest::B1::p::p1") (range (start 20 3) (end 20 14)))
        )
      )
    )
    (query (range (start 26 23) (end 26 27)) (probe (position 26 23))
      (reference
        (source (document "d0") (qualified-name "ConjugationTest::B1::a::p4"))
        (kind referenceSubsetting) (ordinal 0) (authored-target "p.p2")
        (range (start 26 23) (end 26 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ConjugationTest::B1::p::p2") (range (start 21 3) (end 21 15)))
        )
      )
    )
  )
)
~~~
