# META
~~~ini
description=SysML Example (Simple Tests): OccurrenceTest
type=file
~~~
# SOURCE
~~~sysml
package OccurrenceTest {
	occurrence def Occ {
		attribute a;
		ref occurrence occ1 : Occ;
		occurrence occ2 : Occ;
		item x;
		part y;
		
		individual snapshot s : Ind;
		timeslice t;
	}
	
	occurrence occ : Occ {
		occurrence o1 : Occ;
		ref occurrence o2 : Occ;
		item z;
	}

	individual occurrence def Ind {
		snapshot s2;
		timeslice t2;
	}
	individual occurrence ind : Ind, Occ {
		snapshot s3;
		individual timeslice t3 :> ind;
        individual snapshot s4 : Ind;
	}

	occurrence o1 {
	  occurrence o2;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/occurrence_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_occurrence_definition_member")
        (source "semantic")
        (range (start 3 2) (end 3 28))
      )
      (diagnostic
        (severity error)
        (code "recovered_occurrence_def_body_element")
        (source "parser")
        (range (start 8 2) (end 9 2))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 8 2) (end 9 2))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:5713b3e65de90ae6afce9f70b7be3835330f9e71ae13e586775c1e658931b07e") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Ind"))) (kind occurrence-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Ind::s2"))) (kind occurrence) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Ind::t2"))) (kind occurrence) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ"))) (kind occurrence-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ::a"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ::occ2"))) (kind occurrence) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occ"))))
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ::t"))) (kind occurrence) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ::x"))) (kind item) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ::y"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::o1"))) (kind occurrence) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::o1::o2"))) (kind occurrence) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::occ"))) (kind occurrence) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occ"))))
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::occ::o1"))) (kind occurrence) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occ"))))
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::occ::o2"))) (kind occurrence) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occ"))))
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::occ::z"))) (kind item) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ::occ2"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occ")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ")))))
    (reference (id (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::occ"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occ")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ")))))
    (reference (id (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::occ::o1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occ")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ")))))
    (reference (id (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::occ::o2"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occ")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ::occ2"))) (target (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ::occ2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::occ"))) (target (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::occ"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::occ::o1"))) (target (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::occ::o1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::occ::o2"))) (target (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::occ::o2"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/occurrence_test.md") (range (start 4 20) (end 4 23)) (probe (position 4 20))
    (reference (id (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ::occ2"))) (kind featureTyping) (ordinal 0) (authored-target "Occ")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ")))))
  )
  (query (document "memory://snapshot/occurrence_test.md") (range (start 12 18) (end 12 21)) (probe (position 12 18))
    (reference (id (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::occ"))) (kind featureTyping) (ordinal 0) (authored-target "Occ")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ")))))
  )
  (query (document "memory://snapshot/occurrence_test.md") (range (start 13 18) (end 13 21)) (probe (position 13 18))
    (reference (id (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::occ::o1"))) (kind featureTyping) (ordinal 0) (authored-target "Occ")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ")))))
  )
  (query (document "memory://snapshot/occurrence_test.md") (range (start 14 22) (end 14 25)) (probe (position 14 22))
    (reference (id (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::occ::o2"))) (kind featureTyping) (ordinal 0) (authored-target "Occ")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ")))))
  )
)
~~~
