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
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 6 2) (end 6 9))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:5713b3e65de90ae6afce9f70b7be3835330f9e71ae13e586775c1e658931b07e") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Ind"))) (kind occurrence-def) (membership (kind owning) (visibility default)) (facts (modifiers individual)))
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Ind::s2"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (portion snapshot)))
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Ind::t2"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (portion timeslice)))
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ"))) (kind occurrence-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ::a"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ::occ1"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers reference)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occ")))))
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ::occ2"))) (kind occurrence) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occ")))))
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ::s"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers individual) (portion snapshot)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Ind")))))
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ::t"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (portion timeslice)))
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ::x"))) (kind item) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ::y"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::ind"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers individual)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Ind")))))
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::ind::s3"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (portion snapshot)))
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::ind::s4"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers individual) (portion snapshot)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Ind")))))
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::ind::t3"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers individual) (portion timeslice)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ind")))))
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::o1"))) (kind occurrence) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::o1::o2"))) (kind occurrence) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::occ"))) (kind occurrence) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occ")))))
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::occ::o1"))) (kind occurrence) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occ")))))
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::occ::o2"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers reference)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occ")))))
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::occ::z"))) (kind item) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ::occ1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occ")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ")))))
    (reference (id (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ::occ2"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occ")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ")))))
    (reference (id (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ::s"))) (kind featureTyping) (ordinal 0))
      (authored-target "Ind")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Ind")))))
    (reference (id (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::ind"))) (kind featureTyping) (ordinal 0))
      (authored-target "Ind")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Ind")))))
    (reference (id (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::ind::s4"))) (kind featureTyping) (ordinal 0))
      (authored-target "Ind")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Ind")))))
    (reference (id (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::ind::t3"))) (kind subsetting) (ordinal 0))
      (authored-target "ind")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::ind")))))
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
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ::occ1"))) (target (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ::occ1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ::occ2"))) (target (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ::occ2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ::s"))) (target (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Ind"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ::s"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::ind"))) (target (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Ind"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::ind"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::ind::s4"))) (target (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Ind"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::ind::s4"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::ind::t3"))) (target (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::ind"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::ind::t3"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::occ"))) (target (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::occ"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::occ::o1"))) (target (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::occ::o1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::occ::o2"))) (target (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::occ::o2"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Ind")))
      (subtype (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ::s")) (scopes any))
      (subtype (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::ind")) (scopes any))
      (subtype (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::ind::s4")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Ind::s2")))
      (featured-by (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Ind")))
    )
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Ind::t2")))
      (featured-by (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Ind")))
    )
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ")))
      (subtype (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ::occ1")) (scopes any))
      (subtype (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ::occ2")) (scopes any))
      (subtype (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::occ")) (scopes any))
      (subtype (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::occ::o1")) (scopes any))
      (subtype (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::occ::o2")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ::a")))
      (featured-by (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ")))
    )
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ::occ1")))
      (featured-by (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ")))
      (type (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ")) (provenance authored))
      (effective-type (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ")) (source direct))
      (supertype (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ::occ2")))
      (featured-by (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ")))
      (type (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ")) (provenance authored))
      (effective-type (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ")) (source direct))
      (supertype (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ::s")))
      (featured-by (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ")))
      (type (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Ind")) (provenance authored))
      (effective-type (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Ind")) (source direct))
      (supertype (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Ind")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ::t")))
      (featured-by (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ")))
    )
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ::x")))
      (featured-by (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ")))
    )
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ::y")))
      (featured-by (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ")))
    )
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::ind")))
      (type (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Ind")) (provenance authored))
      (effective-type (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Ind")) (source direct))
      (supertype (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Ind")) (scopes any))
      (subtype (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::ind::t3")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::ind::s3")))
      (featured-by (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::ind")))
    )
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::ind::s4")))
      (featured-by (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::ind")))
      (type (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Ind")) (provenance authored))
      (effective-type (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Ind")) (source direct))
      (supertype (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Ind")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::ind::t3")))
      (featured-by (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::ind")))
      (effective-type (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Ind")) (source inherited) (from (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::ind"))))
      (supertype (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Ind")) (scopes any))
      (supertype (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::ind")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::o1::o2")))
      (featured-by (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::o1")))
    )
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::occ")))
      (type (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ")) (provenance authored))
      (effective-type (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ")) (source direct))
      (supertype (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::occ::o1")))
      (featured-by (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::occ")))
      (type (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ")) (provenance authored))
      (effective-type (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ")) (source direct))
      (supertype (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::occ::o2")))
      (featured-by (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::occ")))
      (type (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ")) (provenance authored))
      (effective-type (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ")) (source direct))
      (supertype (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::occ::z")))
      (featured-by (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::occ")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/occurrence_test.md") (range (start 3 24) (end 3 27)) (probe (position 3 24))
    (reference (id (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ::occ1"))) (kind featureTyping) (ordinal 0) (authored-target "Occ")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ")))))
    )
  )
  (query (document "memory://snapshot/occurrence_test.md") (range (start 4 20) (end 4 23)) (probe (position 4 20))
    (reference (id (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ::occ2"))) (kind featureTyping) (ordinal 0) (authored-target "Occ")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ")))))
    )
  )
  (query (document "memory://snapshot/occurrence_test.md") (range (start 8 26) (end 8 29)) (probe (position 8 26))
    (reference (id (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ::s"))) (kind featureTyping) (ordinal 0) (authored-target "Ind")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Ind")))))
    )
  )
  (query (document "memory://snapshot/occurrence_test.md") (range (start 22 29) (end 22 32)) (probe (position 22 29))
    (reference (id (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::ind"))) (kind featureTyping) (ordinal 0) (authored-target "Ind")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Ind")))))
    )
  )
  (query (document "memory://snapshot/occurrence_test.md") (range (start 25 33) (end 25 36)) (probe (position 25 33))
    (reference (id (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::ind::s4"))) (kind featureTyping) (ordinal 0) (authored-target "Ind")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Ind")))))
    )
  )
  (query (document "memory://snapshot/occurrence_test.md") (range (start 24 29) (end 24 32)) (probe (position 24 29))
    (reference (id (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::ind::t3"))) (kind subsetting) (ordinal 0) (authored-target "ind")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::ind")))))
    )
  )
  (query (document "memory://snapshot/occurrence_test.md") (range (start 12 18) (end 12 21)) (probe (position 12 18))
    (reference (id (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::occ"))) (kind featureTyping) (ordinal 0) (authored-target "Occ")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ")))))
    )
  )
  (query (document "memory://snapshot/occurrence_test.md") (range (start 13 18) (end 13 21)) (probe (position 13 18))
    (reference (id (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::occ::o1"))) (kind featureTyping) (ordinal 0) (authored-target "Occ")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ")))))
    )
  )
  (query (document "memory://snapshot/occurrence_test.md") (range (start 14 22) (end 14 25)) (probe (position 14 22))
    (reference (id (source (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::occ::o2"))) (kind featureTyping) (ordinal 0) (authored-target "Occ")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_test.md") (qualified-name "OccurrenceTest::Occ")))))
    )
  )
)
~~~
