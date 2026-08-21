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
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 4 2) (end 4 13))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 5 2) (end 5 14))
      )
      (diagnostic
        (severity error)
        (code "recovered_connection_def_body_element")
        (source "parser")
        (range (start 9 2) (end 10 2))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 9 2) (end 10 2))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 19 2) (end 22 3))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 20 3) (end 20 14))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 21 3) (end 21 15))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:fc429c63d138f3d922e1a28db34c8af3faaea1fdf328f6ff2546d7cc9b568c9b") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::A"))) (kind connection-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::a"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A")))))
    (declaration (id (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::p"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::p::p1"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "P")))))
    (declaration (id (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::p::p2"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "P") (conjugated true)))))
    (declaration (id (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B::p1"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "P")))))
    (declaration (id (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B::p2"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "P") (conjugated true)))))
    (declaration (id (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::I"))) (kind interface-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::I::p1"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 0)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "P")))))
    (declaration (id (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::I::p2"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 1)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "P") (conjugated true)))))
    (declaration (id (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P"))) (kind port-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::A")))))
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
    (reference (id (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::I::p1"))) (kind featureTyping) (ordinal 0))
      (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P")))))
    (reference (id (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::I::p2"))) (kind featureTyping) (ordinal 0))
      (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::a"))) (target (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::p::p1"))) (target (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::p::p1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::p::p2"))) (target (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::p::p2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B::p1"))) (target (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B::p1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B::p2"))) (target (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B::p2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::I::p1"))) (target (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::I::p1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::I::p2"))) (target (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::I::p2"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::A")))
      (subtype (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::a")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::a")))
      (featured-by (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1")))
      (type (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::A")) (provenance authored))
      (effective-type (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::A")) (source direct))
      (supertype (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::A")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::p")))
      (featured-by (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1")))
    )
    (declaration (id (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::p::p1")))
      (featured-by (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::p")))
      (type (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P")) (provenance authored))
      (effective-type (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P")) (source direct))
      (supertype (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::p::p2")))
      (featured-by (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::p")))
      (type (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P")) (provenance authored))
      (effective-type (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P")) (source direct))
      (supertype (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B::p1")))
      (featured-by (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B")))
      (type (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P")) (provenance authored))
      (effective-type (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P")) (source direct))
      (supertype (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B::p2")))
      (featured-by (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B")))
      (type (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P")) (provenance authored))
      (effective-type (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P")) (source direct))
      (supertype (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::I")))
      (positional-ends (authored 2) (effective 2))
    )
    (declaration (id (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::I::p1")))
      (featured-by (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::I")))
      (type (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P")) (provenance authored))
      (effective-type (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P")) (source direct))
      (supertype (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::I::p2")))
      (featured-by (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::I")))
      (type (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P")) (provenance authored))
      (effective-type (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P")) (source direct))
      (supertype (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P")))
      (subtype (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::p::p1")) (scopes any))
      (subtype (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::p::p2")) (scopes any))
      (subtype (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B::p1")) (scopes any))
      (subtype (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B::p2")) (scopes any))
      (subtype (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::I::p1")) (scopes any))
      (subtype (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::I::p2")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/conjugation_test.md") (range (start 24 16) (end 24 17)) (probe (position 24 16))
    (reference (id (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::a"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::A")))))
    )
  )
  (query (document "memory://snapshot/conjugation_test.md") (range (start 20 12) (end 20 13)) (probe (position 20 12))
    (reference (id (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::p::p1"))) (kind featureTyping) (ordinal 0) (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P")))))
    )
  )
  (query (document "memory://snapshot/conjugation_test.md") (range (start 21 13) (end 21 14)) (probe (position 21 13))
    (reference (id (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B1::p::p2"))) (kind featureTyping) (ordinal 0) (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P")))))
    )
  )
  (query (document "memory://snapshot/conjugation_test.md") (range (start 4 11) (end 4 12)) (probe (position 4 11))
    (reference (id (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B::p1"))) (kind featureTyping) (ordinal 0) (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P")))))
    )
  )
  (query (document "memory://snapshot/conjugation_test.md") (range (start 5 12) (end 5 13)) (probe (position 5 12))
    (reference (id (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::B::p2"))) (kind featureTyping) (ordinal 0) (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P")))))
    )
  )
  (query (document "memory://snapshot/conjugation_test.md") (range (start 14 10) (end 14 11)) (probe (position 14 10))
    (reference (id (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::I::p1"))) (kind featureTyping) (ordinal 0) (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P")))))
    )
  )
  (query (document "memory://snapshot/conjugation_test.md") (range (start 15 11) (end 15 12)) (probe (position 15 11))
    (reference (id (source (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::I::p2"))) (kind featureTyping) (ordinal 0) (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/conjugation_test.md") (qualified-name "ConjugationTest::P")))))
    )
  )
)
~~~
