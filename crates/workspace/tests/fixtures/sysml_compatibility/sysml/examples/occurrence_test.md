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
(model
  (namespace
    (package 'OccurrenceTest'
      (occurrence_def 'Occ'
        (attribute_usage composite 'a')
        (occurrence_usage reference 'occ1' : 'OccurrenceTest::Occ'[occurrence_def])
        (occurrence_usage composite 'occ2' : 'OccurrenceTest::Occ'[occurrence_def])
        (item_usage composite 'x')
        (part_usage composite 'y')
        (occurrence_usage individual composite 's' : 'OccurrenceTest::Ind'[occurrence_def])
        (occurrence_usage composite 't'))
      (occurrence_usage 'occ' : 'OccurrenceTest::Occ'[occurrence_def]
        (occurrence_usage composite 'o1' : 'OccurrenceTest::Occ'[occurrence_def])
        (occurrence_usage reference 'o2' : 'OccurrenceTest::Occ'[occurrence_def])
        (item_usage composite 'z'))
      (occurrence_def individual 'Ind'
        (occurrence_usage composite 's2')
        (occurrence_usage composite 't2'))
      (occurrence_usage individual 'ind' : 'OccurrenceTest::Ind'[occurrence_def] : 'OccurrenceTest::Occ'[occurrence_def]
        (occurrence_usage composite 's3')
        (occurrence_usage individual composite 't3' :> 'OccurrenceTest::ind'[occurrence_usage])
        (occurrence_usage individual composite 's4' : 'OccurrenceTest::Ind'[occurrence_def]))
      (occurrence_usage 'o1'
        (occurrence_usage composite 'o2')))))
~~~
