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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwIndividual,KwDef,Ident,Semicolon,
KwIndividual,KwOccurrence,KwDef,Ident,OpenCurly,
KwIndividual,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwIndividual,KwItem,KwDef,Ident,OpenCurly,
KwIndividual,KwItem,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwItem,KwDef,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwIndividual,KwItem,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwIndividual,KwItem,ColonGtGt,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwIndividual,KwPart,KwDef,Ident,OpenCurly,
KwIndividual,KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwIndividual,KwPart,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwIndividual,KwPart,ColonGtGt,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwIndividual,KwAction,KwDef,Ident,OpenCurly,
KwIndividual,KwAction,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAction,KwDef,Ident,OpenCurly,
KwAction,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwIndividual,KwAction,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwIndividual,KwAction,ColonGtGt,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'IndividualTest'
    (individual_def individual 'IO1')
    (occurrence_def individual 'IO2'
      (individual_usage individual 'io' : 'IO1'))
    (item_def individual 'II1'
      (item_usage individual 'ii' : 'II1'))
    (item_def 'I'
      (part_usage 'i' : 'I'))
    (item_def individual 'II2' :> 'I'
      (item_usage individual :>> 'i' : 'II2'))
    (part_def individual 'IP1'
      (part_usage individual 'p' : 'IP1'))
    (part_def 'P'
      (part_usage 'p' : 'P'))
    (part_def individual 'IP2' :> 'P'
      (part_usage individual :>> 'p' : 'IP2'))
    (action_def individual 'AP1'
      (action_usage individual 'a' : 'AP1'))
    (action_def 'A'
      (action_usage 'a' : 'A'))
    (action_def individual 'IA2' :> 'A'
      (action_usage individual :>> 'a' : 'IA2'))))
~~~
# FORMAT
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
    (element (kind "package") (id (node (document "d0") (qualified-name "IndividualTest"))) (name "IndividualTest") (declared-name "IndividualTest")
      (contains
        (element (kind "action def") (id (node (document "d0") (qualified-name "IndividualTest::A"))) (name "A") (declared-name "A")
          (contains
            (element (kind "action") (id (node (document "d0") (qualified-name "IndividualTest::A::a"))) (name "a") (declared-name "a") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "IndividualTest::A")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "IndividualTest::I"))) (name "I") (declared-name "I")
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "IndividualTest::I::i"))) (name "i") (declared-name "i") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "IndividualTest::I")))))
          )
        )
        (element (kind "individual def") (id (node (document "d0") (qualified-name "IndividualTest::IO1"))) (name "IO1") (declared-name "IO1"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "IndividualTest::IP1"))) (name "IP1") (declared-name "IP1") (declared (properties (individual true)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "IndividualTest::IP1::p"))) (name "p") (declared-name "p") (declared (properties (individual true) (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "IndividualTest::IP1")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "IndividualTest::IP2"))) (name "IP2") (declared-name "IP2") (declared (properties (individual true)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "IndividualTest::IP2::p"))) (name "p") (declared-name "p") (declared (properties (individual true) (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "IndividualTest::IP2")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "IndividualTest::P"))) (name "P") (declared-name "P") (declared)
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "IndividualTest::P::p"))) (name "p") (declared-name "p") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "IndividualTest::P")))))
          )
        )
      )
    )
  )
  (relationships
    (perform (status resolved) (from (node (document "d0") (qualified-name "IndividualTest::A"))) (to (node (document "d0") (qualified-name "IndividualTest::A::a"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "IndividualTest::IP2"))) (to (node (document "d0") (qualified-name "IndividualTest::P"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "IndividualTest::A::a"))) (to (node (document "d0") (qualified-name "IndividualTest::A"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "IndividualTest::I::i"))) (to (node (document "d0") (qualified-name "IndividualTest::I"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "IndividualTest::IP1::p"))) (to (node (document "d0") (qualified-name "IndividualTest::IP1"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "IndividualTest::IP2::p"))) (to (node (document "d0") (qualified-name "IndividualTest::IP2"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "IndividualTest::P::p"))) (to (node (document "d0") (qualified-name "IndividualTest::P"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
