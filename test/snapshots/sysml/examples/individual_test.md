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
  (document "individual_test.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_package_body_element")
        (source "sysml")
        (range (start 2 1) (end 2 62))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 2 1) (end 2 62))
      )
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "da4bcfced120889430d987b4507305f3dcf842dac76e1f3bab317e3222307367") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "IndividualTest"))) (kind "package") (name "IndividualTest") (declared-name "IndividualTest") (range (start (line 0) (character 0)) (end (line 0) (character 593))))
    (element (id (node (document "d0") (qualified-name "IndividualTest::A"))) (kind "action def") (name "A") (declared-name "A") (range (start (line 32) (character 1)) (end (line 32) (character 34))) (parent (node (document "d0") (qualified-name "IndividualTest"))) (authored (membership (kind Owning)) (relationships (perform (reference "IndividualTest::A::a") (range none)))))
    (element (id (node (document "d0") (qualified-name "IndividualTest::A::a"))) (kind "action") (name "a") (declared-name "a") (range (start (line 33) (character 2)) (end (line 33) (character 15))) (parent (node (document "d0") (qualified-name "IndividualTest::A"))) (authored (membership (kind Feature)) (relationships (typing (reference "A") (range none)))))
    (element (id (node (document "d0") (qualified-name "IndividualTest::I"))) (kind "item def") (name "I") (declared-name "I") (range (start (line 10) (character 1)) (end (line 10) (character 30))) (parent (node (document "d0") (qualified-name "IndividualTest"))))
    (element (id (node (document "d0") (qualified-name "IndividualTest::I::i"))) (kind "part") (name "i") (declared-name "i") (range (start (line 11) (character 2)) (end (line 11) (character 13))) (parent (node (document "d0") (qualified-name "IndividualTest::I"))) (authored (membership (kind Feature)) (relationships (typing (reference "I") (range (start (line 11) (character 11)) (end (line 11) (character 12)))))))
    (element (id (node (document "d0") (qualified-name "IndividualTest::IO1"))) (kind "individual def") (name "IO1") (declared-name "IO1") (range (start (line 1) (character 1)) (end (line 1) (character 20))) (parent (node (document "d0") (qualified-name "IndividualTest"))))
    (element (id (node (document "d0") (qualified-name "IndividualTest::IP1"))) (kind "part def") (name "IP1") (declared-name "IP1") (range (start (line 17) (character 1)) (end (line 17) (character 56))) (parent (node (document "d0") (qualified-name "IndividualTest"))))
    (element (id (node (document "d0") (qualified-name "IndividualTest::IP1::p"))) (kind "part") (name "p") (declared-name "p") (range (start (line 18) (character 2)) (end (line 18) (character 26))) (parent (node (document "d0") (qualified-name "IndividualTest::IP1"))) (authored (membership (kind Feature)) (relationships (typing (reference "IP1") (range (start (line 18) (character 22)) (end (line 18) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "IndividualTest::IP2"))) (kind "part def") (name "IP2") (declared-name "IP2") (range (start (line 24) (character 1)) (end (line 24) (character 65))) (parent (node (document "d0") (qualified-name "IndividualTest"))) (authored (membership (kind Owning)) (relationships (specializes (reference "P") (range (start (line 24) (character 28)) (end (line 24) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "IndividualTest::IP2::p"))) (kind "part") (name "p") (declared-name "p") (range (start (line 25) (character 2)) (end (line 25) (character 30))) (parent (node (document "d0") (qualified-name "IndividualTest::IP2"))) (authored (membership (kind Feature)) (relationships (typing (reference "IP2") (range (start (line 25) (character 26)) (end (line 25) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "IndividualTest::P"))) (kind "part def") (name "P") (declared-name "P") (range (start (line 21) (character 1)) (end (line 21) (character 30))) (parent (node (document "d0") (qualified-name "IndividualTest"))))
    (element (id (node (document "d0") (qualified-name "IndividualTest::P::p"))) (kind "part") (name "p") (declared-name "p") (range (start (line 22) (character 2)) (end (line 22) (character 13))) (parent (node (document "d0") (qualified-name "IndividualTest::P"))) (authored (membership (kind Feature)) (relationships (typing (reference "P") (range (start (line 22) (character 11)) (end (line 22) (character 12)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "IndividualTest::A"))) (kind performSource) (ordinal 0)) (authored-target "IndividualTest::A::a") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "IndividualTest::A::a")))))
    (reference (id (source (node (document "d0") (qualified-name "IndividualTest::A::a"))) (kind featureTyping) (ordinal 0)) (authored-target "A") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "IndividualTest::A")))))
    (reference (id (source (node (document "d0") (qualified-name "IndividualTest::I::i"))) (kind featureTyping) (ordinal 0)) (authored-target "I") (range (start (line 11) (character 11)) (end (line 11) (character 12))) (outcome (status resolved) (target (node (document "d0") (qualified-name "IndividualTest::I")))))
    (reference (id (source (node (document "d0") (qualified-name "IndividualTest::IP1::p"))) (kind featureTyping) (ordinal 0)) (authored-target "IP1") (range (start (line 18) (character 22)) (end (line 18) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "IndividualTest::IP1")))))
    (reference (id (source (node (document "d0") (qualified-name "IndividualTest::IP2"))) (kind specialization) (ordinal 0)) (authored-target "P") (range (start (line 24) (character 28)) (end (line 24) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "IndividualTest::P")))))
    (reference (id (source (node (document "d0") (qualified-name "IndividualTest::IP2::p"))) (kind featureTyping) (ordinal 0)) (authored-target "IP2") (range (start (line 25) (character 26)) (end (line 25) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "IndividualTest::IP2")))))
    (reference (id (source (node (document "d0") (qualified-name "IndividualTest::P::p"))) (kind featureTyping) (ordinal 0)) (authored-target "P") (range (start (line 22) (character 11)) (end (line 22) (character 12))) (outcome (status resolved) (target (node (document "d0") (qualified-name "IndividualTest::P")))))
  )
  (relationships
    (relationship (kind perform) (source (node (document "d0") (qualified-name "IndividualTest::A"))) (target (node (document "d0") (qualified-name "IndividualTest::A::a"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "IndividualTest::A"))) (kind performSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "IndividualTest::A::a"))) (target (node (document "d0") (qualified-name "IndividualTest::A"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "IndividualTest::A::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "IndividualTest::I::i"))) (target (node (document "d0") (qualified-name "IndividualTest::I"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "IndividualTest::I::i"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "IndividualTest::IP1::p"))) (target (node (document "d0") (qualified-name "IndividualTest::IP1"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "IndividualTest::IP1::p"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "IndividualTest::IP2"))) (target (node (document "d0") (qualified-name "IndividualTest::P"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "IndividualTest::IP2"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "IndividualTest::IP2::p"))) (target (node (document "d0") (qualified-name "IndividualTest::IP2"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "IndividualTest::IP2::p"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "IndividualTest::P::p"))) (target (node (document "d0") (qualified-name "IndividualTest::P"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "IndividualTest::P::p"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
