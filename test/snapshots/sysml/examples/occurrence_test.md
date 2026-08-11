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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwOccurrence,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Semicolon,
KwRef,KwOccurrence,Ident,Colon,Ident,Semicolon,
KwOccurrence,Ident,Colon,Ident,Semicolon,
KwItem,Ident,Semicolon,
KwPart,Ident,Semicolon,
KwIndividual,KwSnapshot,Ident,Colon,Ident,Semicolon,
KwTimeslice,Ident,Semicolon,
CloseCurly,
KwOccurrence,Ident,Colon,Ident,OpenCurly,
KwOccurrence,Ident,Colon,Ident,Semicolon,
KwRef,KwOccurrence,Ident,Colon,Ident,Semicolon,
KwItem,Ident,Semicolon,
CloseCurly,
KwIndividual,KwOccurrence,KwDef,Ident,OpenCurly,
KwSnapshot,Ident,Semicolon,
KwTimeslice,Ident,Semicolon,
CloseCurly,
KwIndividual,KwOccurrence,Ident,Colon,Ident,Comma,Ident,OpenCurly,
KwSnapshot,Ident,Semicolon,
KwIndividual,KwTimeslice,Ident,ColonGt,Ident,Semicolon,
KwIndividual,KwSnapshot,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwOccurrence,Ident,OpenCurly,
KwOccurrence,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'OccurrenceTest'
    (occurrence_def 'Occ'
      (attribute_usage 'a')
      (occurrence_usage ref 'occ1' : 'Occ')
      (occurrence_usage 'occ2' : 'Occ')
      (item_usage 'x')
      (part_usage 'y')
      (individual_usage individual snapshot 's' : 'Ind')
      (portion_usage timeslice 't'))
    (occurrence_usage 'occ' : 'Occ'
      (occurrence_usage 'o1' : 'Occ')
      (occurrence_usage ref 'o2' : 'Occ')
      (item_usage 'z'))
    (occurrence_def individual 'Ind'
      (portion_usage snapshot 's2')
      (portion_usage timeslice 't2'))
    (occurrence_usage individual 'ind' : 'Ind', 'Occ'
      (portion_usage snapshot 's3')
      (individual_usage individual timeslice 't3' :> 'ind')
      (individual_usage individual snapshot 's4' : 'Ind'))
    (occurrence_usage 'o1'
      (occurrence_usage 'o2'))))
~~~
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "0e6b905f03a39f694b949e62e7f07a1aa921221ce56ae9467058d2225b52b0c6") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "OccurrenceTest"))) (kind "package") (name "OccurrenceTest") (declared-name "OccurrenceTest") (range (start (line 0) (character 0)) (end (line 0) (character 515))))
    (element (id (node (document "d0") (qualified-name "OccurrenceTest::Occ"))) (kind "occurrence def") (name "Occ") (declared-name "Occ") (range (start (line 1) (character 1)) (end (line 1) (character 162))) (parent (node (document "d0") (qualified-name "OccurrenceTest"))))
    (element (id (node (document "d0") (qualified-name "OccurrenceTest::Occ::a"))) (kind "attribute") (name "a") (declared-name "a") (range (start (line 2) (character 2)) (end (line 2) (character 14))) (parent (node (document "d0") (qualified-name "OccurrenceTest::Occ"))))
    (element (id (node (document "d0") (qualified-name "OccurrenceTest::Occ::occ2"))) (kind "occurrence") (name "occ2") (declared-name "occ2") (range (start (line 4) (character 13)) (end (line 4) (character 24))) (parent (node (document "d0") (qualified-name "OccurrenceTest::Occ"))) (authored (membership (kind Feature)) (relationships (typing (reference "Occ") (range none)))))
    (element (id (node (document "d0") (qualified-name "OccurrenceTest::Occ::t"))) (kind "occurrence") (name "t") (declared-name "t") (range (start (line 9) (character 12)) (end (line 9) (character 14))) (parent (node (document "d0") (qualified-name "OccurrenceTest::Occ"))))
    (element (id (node (document "d0") (qualified-name "OccurrenceTest::Occ::y"))) (kind "part") (name "y") (declared-name "y") (range (start (line 6) (character 2)) (end (line 6) (character 9))) (parent (node (document "d0") (qualified-name "OccurrenceTest::Occ"))) (authored (membership (kind Feature)) (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "OccurrenceTest::o1"))) (kind "occurrence") (name "o1") (declared-name "o1") (range (start (line 28) (character 12)) (end (line 28) (character 37))) (parent (node (document "d0") (qualified-name "OccurrenceTest"))))
    (element (id (node (document "d0") (qualified-name "OccurrenceTest::o1::o2"))) (kind "occurrence") (name "o2") (declared-name "o2") (range (start (line 29) (character 14)) (end (line 29) (character 17))) (parent (node (document "d0") (qualified-name "OccurrenceTest::o1"))))
    (element (id (node (document "d0") (qualified-name "OccurrenceTest::occ"))) (kind "occurrence") (name "occ") (declared-name "occ") (range (start (line 12) (character 12)) (end (line 12) (character 86))) (parent (node (document "d0") (qualified-name "OccurrenceTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "Occ") (range none)))))
    (element (id (node (document "d0") (qualified-name "OccurrenceTest::occ::o1"))) (kind "occurrence") (name "o1") (declared-name "o1") (range (start (line 13) (character 13)) (end (line 13) (character 22))) (parent (node (document "d0") (qualified-name "OccurrenceTest::occ"))) (authored (membership (kind Feature)) (relationships (typing (reference "Occ") (range none)))))
    (element (id (node (document "d0") (qualified-name "OccurrenceTest::occ::o2"))) (kind "occurrence") (name "o2") (declared-name "o2") (range (start (line 14) (character 17)) (end (line 14) (character 26))) (parent (node (document "d0") (qualified-name "OccurrenceTest::occ"))) (authored (membership (kind Feature)) (relationships (typing (reference "Occ") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "OccurrenceTest::Occ::occ2"))) (kind featureTyping) (ordinal 0)) (authored-target "Occ") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "OccurrenceTest::Occ")))))
    (reference (id (source (node (document "d0") (qualified-name "OccurrenceTest::Occ::y"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "OccurrenceTest::occ"))) (kind featureTyping) (ordinal 0)) (authored-target "Occ") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "OccurrenceTest::Occ")))))
    (reference (id (source (node (document "d0") (qualified-name "OccurrenceTest::occ::o1"))) (kind featureTyping) (ordinal 0)) (authored-target "Occ") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "OccurrenceTest::Occ")))))
    (reference (id (source (node (document "d0") (qualified-name "OccurrenceTest::occ::o2"))) (kind featureTyping) (ordinal 0)) (authored-target "Occ") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "OccurrenceTest::Occ")))))
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
