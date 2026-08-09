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
(model
  (namespace
    (package 'IndividualTest'
      (occurrence_def individual 'IO1')
      (occurrence_def individual 'IO2'
        (occurrence_usage individual composite 'io' : 'IndividualTest::IO1'[occurrence_def]))
      (item_def individual 'II1'
        (item_usage individual composite 'ii' : 'IndividualTest::II1'[item_def]))
      (item_def 'I'
        (part_usage composite 'i' : 'IndividualTest::I'[item_def]))
      (item_def individual 'II2' :> 'IndividualTest::I'[item_def]
        (item_usage individual composite :>> 'IndividualTest::I::i'[part_usage] : 'IndividualTest::II2'[item_def]))
      (part_def individual 'IP1'
        (part_usage individual composite 'p' : 'IndividualTest::IP1'[part_def]))
      (part_def 'P'
        (part_usage composite 'p' : 'IndividualTest::P'[part_def]))
      (part_def individual 'IP2' :> 'IndividualTest::P'[part_def]
        (part_usage individual composite :>> 'IndividualTest::P::p'[part_usage] : 'IndividualTest::IP2'[part_def]))
      (action_def individual 'AP1'
        (action_usage individual composite 'a' : 'IndividualTest::AP1'[action_def]))
      (action_def 'A'
        (action_usage composite 'a' : 'IndividualTest::A'[action_def]))
      (action_def individual 'IA2' :> 'IndividualTest::A'[action_def]
        (action_usage individual composite :>> 'IndividualTest::A::a'[action_usage] : 'IndividualTest::IA2'[action_def])))))
~~~
