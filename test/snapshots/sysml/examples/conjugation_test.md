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
  (document "memory://snapshot/conjugation_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 13 1) (end 16 2))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 25 22) (end 25 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 26 23) (end 26 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 28 2) (end 31 3))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:fc429c63d138f3d922e1a28db34c8af3faaea1fdf328f6ff2546d7cc9b568c9b") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::A"))) (kind connection-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::A::p1"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "P"))))
    (declaration (id (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::A::p2"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "P") (conjugated true))))
    (declaration (id (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::a"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A"))))
    (declaration (id (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::a::p3"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "P")) (connectorEnd (reference "p::p1"))))
    (declaration (id (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::a::p4"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "P") (conjugated true)) (connectorEnd (reference "p::p2"))))
    (declaration (id (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::p"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::p::p1"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "P"))))
    (declaration (id (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::p::p2"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "P") (conjugated true))))
    (declaration (id (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B::p1"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "P"))))
    (declaration (id (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B::p2"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "P") (conjugated true))))
    (declaration (id (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P"))) (kind port-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::A::p1"))) (kind featureTyping) (ordinal 0))
      (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P")))))
    (reference (id (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::A::p2"))) (kind featureTyping) (ordinal 0))
      (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P")))))
    (reference (id (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::A")))))
    (reference (id (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::a::p3"))) (kind featureTyping) (ordinal 0))
      (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P")))))
    (reference (id (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::a::p3"))) (kind connectorEnd) (ordinal 0))
      (authored-target "p::p1")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::a::p4"))) (kind featureTyping) (ordinal 0))
      (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P")))))
    (reference (id (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::a::p4"))) (kind connectorEnd) (ordinal 0))
      (authored-target "p::p2")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::p::p1"))) (kind featureTyping) (ordinal 0))
      (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P")))))
    (reference (id (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::p::p2"))) (kind featureTyping) (ordinal 0))
      (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P")))))
    (reference (id (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B::p1"))) (kind featureTyping) (ordinal 0))
      (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P")))))
    (reference (id (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B::p2"))) (kind featureTyping) (ordinal 0))
      (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::A::p1"))) (target (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::A::p1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::A::p2"))) (target (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::A::p2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::a"))) (target (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::a::p3"))) (target (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::a::p3"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::a::p4"))) (target (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::a::p4"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::p::p1"))) (target (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::p::p1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::p::p2"))) (target (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::p::p2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B::p1"))) (target (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B::p1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B::p2"))) (target (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B::p2"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/conjugation_test.md") (range (start 9 15) (end 9 16)) (probe (position 9 15))
    (reference (id (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::A::p1"))) (kind featureTyping) (ordinal 0) (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P")))))
  )
  (query (document "memory://snapshot/conjugation_test.md") (range (start 10 16) (end 10 17)) (probe (position 10 16))
    (reference (id (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::A::p2"))) (kind featureTyping) (ordinal 0) (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P")))))
  )
  (query (document "memory://snapshot/conjugation_test.md") (range (start 24 16) (end 24 17)) (probe (position 24 16))
    (reference (id (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::a"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::A")))))
  )
  (query (document "memory://snapshot/conjugation_test.md") (range (start 25 16) (end 25 17)) (probe (position 25 16))
    (reference (id (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::a::p3"))) (kind featureTyping) (ordinal 0) (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P")))))
  )
  (query (document "memory://snapshot/conjugation_test.md") (range (start 25 22) (end 25 26)) (probe (position 25 22))
    (reference (id (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::a::p3"))) (kind connectorEnd) (ordinal 0) (authored-target "p::p1")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/conjugation_test.md") (range (start 26 17) (end 26 18)) (probe (position 26 17))
    (reference (id (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::a::p4"))) (kind featureTyping) (ordinal 0) (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P")))))
  )
  (query (document "memory://snapshot/conjugation_test.md") (range (start 26 23) (end 26 27)) (probe (position 26 23))
    (reference (id (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::a::p4"))) (kind connectorEnd) (ordinal 0) (authored-target "p::p2")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/conjugation_test.md") (range (start 20 12) (end 20 13)) (probe (position 20 12))
    (reference (id (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::p::p1"))) (kind featureTyping) (ordinal 0) (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P")))))
  )
  (query (document "memory://snapshot/conjugation_test.md") (range (start 21 13) (end 21 14)) (probe (position 21 13))
    (reference (id (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::p::p2"))) (kind featureTyping) (ordinal 0) (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P")))))
  )
  (query (document "memory://snapshot/conjugation_test.md") (range (start 4 11) (end 4 12)) (probe (position 4 11))
    (reference (id (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B::p1"))) (kind featureTyping) (ordinal 0) (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P")))))
  )
  (query (document "memory://snapshot/conjugation_test.md") (range (start 5 12) (end 5 13)) (probe (position 5 12))
    (reference (id (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B::p2"))) (kind featureTyping) (ordinal 0) (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P")))))
  )
)
~~~
