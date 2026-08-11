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
  (document "occurrence_test.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 5 2) (end 5 12))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 2) (end 6 9))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 6 2) (end 6 9))
      )
      (diagnostic
        (severity error)
        (code "recovered_occurrence_def_body_element")
        (source "sysml")
        (range (start 8 2) (end 8 33))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 15 2) (end 15 11))
      )
      (diagnostic
        (severity error)
        (code "recovered_package_body_element")
        (source "sysml")
        (range (start 18 1) (end 18 68))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 18 1) (end 18 68))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "0e6b905f03a39f694b949e62e7f07a1aa921221ce56ae9467058d2225b52b0c6") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "OccurrenceTest"))) (kind "package") (name "OccurrenceTest") (declared-name "OccurrenceTest"))
    (element (id (node (document "d0") (qualified-name "OccurrenceTest::Occ"))) (kind "occurrence def") (name "Occ") (declared-name "Occ") (parent (node (document "d0") (qualified-name "OccurrenceTest"))))
    (element (id (node (document "d0") (qualified-name "OccurrenceTest::Occ::a"))) (kind "attribute") (name "a") (declared-name "a") (parent (node (document "d0") (qualified-name "OccurrenceTest::Occ"))))
    (element (id (node (document "d0") (qualified-name "OccurrenceTest::Occ::occ2"))) (kind "occurrence") (name "occ2") (declared-name "occ2") (parent (node (document "d0") (qualified-name "OccurrenceTest::Occ"))) (authored (membership (kind Feature)) (relationships (typing (reference "Occ")))))
    (element (id (node (document "d0") (qualified-name "OccurrenceTest::Occ::t"))) (kind "occurrence") (name "t") (declared-name "t") (parent (node (document "d0") (qualified-name "OccurrenceTest::Occ"))))
    (element (id (node (document "d0") (qualified-name "OccurrenceTest::Occ::y"))) (kind "part") (name "y") (declared-name "y") (parent (node (document "d0") (qualified-name "OccurrenceTest::Occ"))) (authored (membership (kind Feature)) (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "OccurrenceTest::o1"))) (kind "occurrence") (name "o1") (declared-name "o1") (parent (node (document "d0") (qualified-name "OccurrenceTest"))))
    (element (id (node (document "d0") (qualified-name "OccurrenceTest::o1::o2"))) (kind "occurrence") (name "o2") (declared-name "o2") (parent (node (document "d0") (qualified-name "OccurrenceTest::o1"))))
    (element (id (node (document "d0") (qualified-name "OccurrenceTest::occ"))) (kind "occurrence") (name "occ") (declared-name "occ") (parent (node (document "d0") (qualified-name "OccurrenceTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "Occ")))))
    (element (id (node (document "d0") (qualified-name "OccurrenceTest::occ::o1"))) (kind "occurrence") (name "o1") (declared-name "o1") (parent (node (document "d0") (qualified-name "OccurrenceTest::occ"))) (authored (membership (kind Feature)) (relationships (typing (reference "Occ")))))
    (element (id (node (document "d0") (qualified-name "OccurrenceTest::occ::o2"))) (kind "occurrence") (name "o2") (declared-name "o2") (parent (node (document "d0") (qualified-name "OccurrenceTest::occ"))) (authored (membership (kind Feature)) (relationships (typing (reference "Occ")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "OccurrenceTest::Occ::occ2"))) (kind featureTyping) (ordinal 0)) (authored-target "Occ") (outcome (status resolved) (target (node (document "d0") (qualified-name "OccurrenceTest::Occ")))))
    (reference (id (source (node (document "d0") (qualified-name "OccurrenceTest::Occ::y"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "OccurrenceTest::occ"))) (kind featureTyping) (ordinal 0)) (authored-target "Occ") (outcome (status resolved) (target (node (document "d0") (qualified-name "OccurrenceTest::Occ")))))
    (reference (id (source (node (document "d0") (qualified-name "OccurrenceTest::occ::o1"))) (kind featureTyping) (ordinal 0)) (authored-target "Occ") (outcome (status resolved) (target (node (document "d0") (qualified-name "OccurrenceTest::Occ")))))
    (reference (id (source (node (document "d0") (qualified-name "OccurrenceTest::occ::o2"))) (kind featureTyping) (ordinal 0)) (authored-target "Occ") (outcome (status resolved) (target (node (document "d0") (qualified-name "OccurrenceTest::Occ")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "OccurrenceTest::Occ::occ2"))) (target (node (document "d0") (qualified-name "OccurrenceTest::Occ"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "OccurrenceTest::Occ::occ2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "OccurrenceTest::occ"))) (target (node (document "d0") (qualified-name "OccurrenceTest::Occ"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "OccurrenceTest::occ"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "OccurrenceTest::occ::o1"))) (target (node (document "d0") (qualified-name "OccurrenceTest::Occ"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "OccurrenceTest::occ::o1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "OccurrenceTest::occ::o2"))) (target (node (document "d0") (qualified-name "OccurrenceTest::Occ"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "OccurrenceTest::occ::o2"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
