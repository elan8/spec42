# META
~~~ini
description=SysML Example (Simple Tests): IndividualTest
type=file
~~~
# SOURCE
~~~sysml
package IndividualTest {
	individual def IO1;
	individual occurrence def IO2 {
		individual io : IO1;
	}
	
	individual item def II1 {
		individual item ii : II1;
	}
	
	item def I {
		part i : I;
	}
	individual item def II2 :> I {
		individual item :>> i : II2;
	}
	
	individual part def IP1 {
		individual part p : IP1;
	}
	
	part def P {
		part p : P;
	}
	individual part def IP2 :> P {
		individual part :>> p : IP2;
	}
	
	individual action def AP1 {
		individual action a : AP1;
	}
	
	action def A {
		action a : A;
	}
	individual action def IA2 :> A {
		individual action :>> a : IA2;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/individual_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1 1) (end 1 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 3 18) (end 3 21))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 36 2) (end 37 1))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:8aaa76503e399a4d53a6404f73f0aaba1f59f191abfdc503b8a7730f70e140fd") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::A"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::A::a"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A"))))
    (declaration (id (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::AP1"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::AP1::a"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AP1"))))
    (declaration (id (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::I"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::I::i"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "I"))))
    (declaration (id (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::IA2"))) (kind action-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "A"))))
    (declaration (id (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::II1"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::II1::ii"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "II1"))))
    (declaration (id (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::II2"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "I"))))
    (declaration (id (node (document "memory://snapshot/individual_test.md") (anonymous (kind item) (ordinal 0))))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "II2")) (redefinition (reference "i"))))
    (declaration (id (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::IO2"))) (kind occurrence-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::IO2::io"))) (kind occurrence) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "IO1"))))
    (declaration (id (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::IP1"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::IP1::p"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "IP1"))))
    (declaration (id (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::IP2"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "P"))))
    (declaration (id (node (document "memory://snapshot/individual_test.md") (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "IP2")) (redefinition (reference "p"))))
    (declaration (id (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::P"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::P::p"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "P"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::A::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::A")))))
    (reference (id (source (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::AP1::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "AP1")
      (outcome (status resolved) (target (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::AP1")))))
    (reference (id (source (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::I::i"))) (kind featureTyping) (ordinal 0))
      (authored-target "I")
      (outcome (status resolved) (target (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::I")))))
    (reference (id (source (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::IA2"))) (kind specialization) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::A")))))
    (reference (id (source (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::II1::ii"))) (kind featureTyping) (ordinal 0))
      (authored-target "II1")
      (outcome (status resolved) (target (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::II1")))))
    (reference (id (source (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::II2"))) (kind specialization) (ordinal 0))
      (authored-target "I")
      (outcome (status resolved) (target (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::I")))))
    (reference (id (source (node (document "memory://snapshot/individual_test.md") (anonymous (kind item) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "II2")
      (outcome (status resolved) (target (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::II2")))))
    (reference (id (source (node (document "memory://snapshot/individual_test.md") (anonymous (kind item) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "i")
      (outcome (status resolved) (target (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::I::i")))))
    (reference (id (source (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::IO2::io"))) (kind featureTyping) (ordinal 0))
      (authored-target "IO1")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::IP1::p"))) (kind featureTyping) (ordinal 0))
      (authored-target "IP1")
      (outcome (status resolved) (target (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::IP1")))))
    (reference (id (source (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::IP2"))) (kind specialization) (ordinal 0))
      (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::P")))))
    (reference (id (source (node (document "memory://snapshot/individual_test.md") (anonymous (kind part) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "IP2")
      (outcome (status resolved) (target (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::IP2")))))
    (reference (id (source (node (document "memory://snapshot/individual_test.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "p")
      (outcome (status resolved) (target (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::P::p")))))
    (reference (id (source (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::P::p"))) (kind featureTyping) (ordinal 0))
      (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::P")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::A::a"))) (target (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::A::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::AP1::a"))) (target (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::AP1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::AP1::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::I::i"))) (target (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::I"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::I::i"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::IA2"))) (target (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::IA2"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::II1::ii"))) (target (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::II1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::II1::ii"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::II2"))) (target (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::I"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::II2"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/individual_test.md") (anonymous (kind item) (ordinal 0))))) (target (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::II2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/individual_test.md") (anonymous (kind item) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/individual_test.md") (anonymous (kind item) (ordinal 0))))) (target (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::I::i"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/individual_test.md") (anonymous (kind item) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::IP1::p"))) (target (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::IP1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::IP1::p"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::IP2"))) (target (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::P"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::IP2"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/individual_test.md") (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::IP2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/individual_test.md") (anonymous (kind part) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/individual_test.md") (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::P::p"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/individual_test.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::P::p"))) (target (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::P"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::P::p"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/individual_test.md") (range (start 33 13) (end 33 14)) (probe (position 33 13))
    (reference (id (source (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::A::a"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::A")))))
  )
  (query (document "memory://snapshot/individual_test.md") (range (start 29 24) (end 29 27)) (probe (position 29 24))
    (reference (id (source (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::AP1::a"))) (kind featureTyping) (ordinal 0) (authored-target "AP1")
      (outcome (status resolved) (target (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::AP1")))))
  )
  (query (document "memory://snapshot/individual_test.md") (range (start 11 11) (end 11 12)) (probe (position 11 11))
    (reference (id (source (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::I::i"))) (kind featureTyping) (ordinal 0) (authored-target "I")
      (outcome (status resolved) (target (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::I")))))
  )
  (query (document "memory://snapshot/individual_test.md") (range (start 35 30) (end 35 31)) (probe (position 35 30))
    (reference (id (source (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::IA2"))) (kind specialization) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::A")))))
  )
  (query (document "memory://snapshot/individual_test.md") (range (start 7 23) (end 7 26)) (probe (position 7 23))
    (reference (id (source (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::II1::ii"))) (kind featureTyping) (ordinal 0) (authored-target "II1")
      (outcome (status resolved) (target (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::II1")))))
  )
  (query (document "memory://snapshot/individual_test.md") (range (start 13 28) (end 13 29)) (probe (position 13 28))
    (reference (id (source (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::II2"))) (kind specialization) (ordinal 0) (authored-target "I")
      (outcome (status resolved) (target (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::I")))))
  )
  (query (document "memory://snapshot/individual_test.md") (range (start 14 26) (end 14 29)) (probe (position 14 26))
    (reference (id (source (node (document "memory://snapshot/individual_test.md") (anonymous (kind item) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "II2")
      (outcome (status resolved) (target (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::II2")))))
  )
  (query (document "memory://snapshot/individual_test.md") (range (start 14 22) (end 14 23)) (probe (position 14 22))
    (reference (id (source (node (document "memory://snapshot/individual_test.md") (anonymous (kind item) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "i")
      (outcome (status resolved) (target (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::I::i")))))
  )
  (query (document "memory://snapshot/individual_test.md") (range (start 3 18) (end 3 21)) (probe (position 3 18))
    (reference (id (source (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::IO2::io"))) (kind featureTyping) (ordinal 0) (authored-target "IO1")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/individual_test.md") (range (start 18 22) (end 18 25)) (probe (position 18 22))
    (reference (id (source (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::IP1::p"))) (kind featureTyping) (ordinal 0) (authored-target "IP1")
      (outcome (status resolved) (target (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::IP1")))))
  )
  (query (document "memory://snapshot/individual_test.md") (range (start 24 28) (end 24 29)) (probe (position 24 28))
    (reference (id (source (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::IP2"))) (kind specialization) (ordinal 0) (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::P")))))
  )
  (query (document "memory://snapshot/individual_test.md") (range (start 25 26) (end 25 29)) (probe (position 25 26))
    (reference (id (source (node (document "memory://snapshot/individual_test.md") (anonymous (kind part) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "IP2")
      (outcome (status resolved) (target (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::IP2")))))
  )
  (query (document "memory://snapshot/individual_test.md") (range (start 25 22) (end 25 23)) (probe (position 25 22))
    (reference (id (source (node (document "memory://snapshot/individual_test.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "p")
      (outcome (status resolved) (target (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::P::p")))))
  )
  (query (document "memory://snapshot/individual_test.md") (range (start 22 11) (end 22 12)) (probe (position 22 11))
    (reference (id (source (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::P::p"))) (kind featureTyping) (ordinal 0) (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/individual_test.md") (qualified-name "IndividualTest::P")))))
  )
)
~~~
