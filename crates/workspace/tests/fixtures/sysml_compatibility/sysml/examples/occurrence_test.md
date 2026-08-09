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
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "OccurrenceTest"))) (name "OccurrenceTest") (declared-name "OccurrenceTest")
      (contains
        (element (kind "occurrence def") (id (node (document "d0") (qualified-name "OccurrenceTest::Occ"))) (name "Occ") (declared-name "Occ") (declared)
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "OccurrenceTest::Occ::a"))) (name "a") (declared-name "a") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "OccurrenceTest::Occ")))))
            (element (kind "occurrence") (id (node (document "d0") (qualified-name "OccurrenceTest::Occ::occ2"))) (name "occ2") (declared-name "occ2") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "OccurrenceTest::Occ")))))
            (element (kind "occurrence") (id (node (document "d0") (qualified-name "OccurrenceTest::Occ::t"))) (name "t") (declared-name "t") (declared (properties (portion true) (portion-kind "timeslice"))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "OccurrenceTest::Occ")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "OccurrenceTest::Occ::y"))) (name "y") (declared-name "y") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "OccurrenceTest::Occ")))))
          )
        )
        (element (kind "occurrence") (id (node (document "d0") (qualified-name "OccurrenceTest::o1"))) (name "o1") (declared-name "o1") (declared)
          (contains
            (element (kind "occurrence") (id (node (document "d0") (qualified-name "OccurrenceTest::o1::o2"))) (name "o2") (declared-name "o2") (declared))
          )
        )
        (element (kind "occurrence") (id (node (document "d0") (qualified-name "OccurrenceTest::occ"))) (name "occ") (declared-name "occ") (declared)
          (contains
            (element (kind "occurrence") (id (node (document "d0") (qualified-name "OccurrenceTest::occ::o1"))) (name "o1") (declared-name "o1") (declared) (effective (featuring-type (node (document "d0") (qualified-name "OccurrenceTest::Occ")))))
            (element (kind "occurrence") (id (node (document "d0") (qualified-name "OccurrenceTest::occ::o2"))) (name "o2") (declared-name "o2") (declared (properties (composite false) (reference true))) (effective (featuring-type (node (document "d0") (qualified-name "OccurrenceTest::Occ")))))
          )
        )
      )
    )
  )
  (relationships
    (typing (status resolved) (from (node (document "d0") (qualified-name "OccurrenceTest::Occ::occ2"))) (to (node (document "d0") (qualified-name "OccurrenceTest::Occ"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "OccurrenceTest::occ"))) (to (node (document "d0") (qualified-name "OccurrenceTest::Occ"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "OccurrenceTest::occ::o1"))) (to (node (document "d0") (qualified-name "OccurrenceTest::Occ"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "OccurrenceTest::occ::o2"))) (to (node (document "d0") (qualified-name "OccurrenceTest::Occ"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/occurrence_test.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 5 2) (end 5 12))
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
