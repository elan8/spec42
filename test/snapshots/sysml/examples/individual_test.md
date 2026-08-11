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
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "b76a4469e63155fbcd8ec7b245d4d06e86f564a52687ae9aa2a22af52d659b2d") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "IndividualTest"))) (kind "package") (name "IndividualTest") (declared-name "IndividualTest"))
    (element (id (node (document "d0") (qualified-name "IndividualTest::A"))) (kind "action def") (name "A") (declared-name "A") (parent (node (document "d0") (qualified-name "IndividualTest"))) (authored (membership (kind Owning)) (relationships (perform (reference "IndividualTest::A::a")))))
    (element (id (node (document "d0") (qualified-name "IndividualTest::A::a"))) (kind "action") (name "a") (declared-name "a") (parent (node (document "d0") (qualified-name "IndividualTest::A"))) (authored (membership (kind Feature)) (relationships (typing (reference "A")))))
    (element (id (node (document "d0") (qualified-name "IndividualTest::I"))) (kind "item def") (name "I") (declared-name "I") (parent (node (document "d0") (qualified-name "IndividualTest"))))
    (element (id (node (document "d0") (qualified-name "IndividualTest::I::i"))) (kind "part") (name "i") (declared-name "i") (parent (node (document "d0") (qualified-name "IndividualTest::I"))) (authored (membership (kind Feature)) (relationships (typing (reference "I")))))
    (element (id (node (document "d0") (qualified-name "IndividualTest::IO1"))) (kind "individual def") (name "IO1") (declared-name "IO1") (parent (node (document "d0") (qualified-name "IndividualTest"))))
    (element (id (node (document "d0") (qualified-name "IndividualTest::IP1"))) (kind "part def") (name "IP1") (declared-name "IP1") (parent (node (document "d0") (qualified-name "IndividualTest"))))
    (element (id (node (document "d0") (qualified-name "IndividualTest::IP1::p"))) (kind "part") (name "p") (declared-name "p") (parent (node (document "d0") (qualified-name "IndividualTest::IP1"))) (authored (membership (kind Feature)) (relationships (typing (reference "IP1")))))
    (element (id (node (document "d0") (qualified-name "IndividualTest::IP2"))) (kind "part def") (name "IP2") (declared-name "IP2") (parent (node (document "d0") (qualified-name "IndividualTest"))) (authored (membership (kind Owning)) (relationships (specializes (reference "P")))))
    (element (id (node (document "d0") (qualified-name "IndividualTest::IP2::p"))) (kind "part") (name "p") (declared-name "p") (parent (node (document "d0") (qualified-name "IndividualTest::IP2"))) (authored (membership (kind Feature)) (relationships (typing (reference "IP2")))))
    (element (id (node (document "d0") (qualified-name "IndividualTest::P"))) (kind "part def") (name "P") (declared-name "P") (parent (node (document "d0") (qualified-name "IndividualTest"))))
    (element (id (node (document "d0") (qualified-name "IndividualTest::P::p"))) (kind "part") (name "p") (declared-name "p") (parent (node (document "d0") (qualified-name "IndividualTest::P"))) (authored (membership (kind Feature)) (relationships (typing (reference "P")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "IndividualTest::A"))) (kind performSource) (ordinal 0)) (authored-target "IndividualTest::A::a") (outcome (status resolved) (target (node (document "d0") (qualified-name "IndividualTest::A::a")))))
    (reference (id (source (node (document "d0") (qualified-name "IndividualTest::A::a"))) (kind featureTyping) (ordinal 0)) (authored-target "A") (outcome (status resolved) (target (node (document "d0") (qualified-name "IndividualTest::A")))))
    (reference (id (source (node (document "d0") (qualified-name "IndividualTest::I::i"))) (kind featureTyping) (ordinal 0)) (authored-target "I") (outcome (status resolved) (target (node (document "d0") (qualified-name "IndividualTest::I")))))
    (reference (id (source (node (document "d0") (qualified-name "IndividualTest::IP1::p"))) (kind featureTyping) (ordinal 0)) (authored-target "IP1") (outcome (status resolved) (target (node (document "d0") (qualified-name "IndividualTest::IP1")))))
    (reference (id (source (node (document "d0") (qualified-name "IndividualTest::IP2"))) (kind specialization) (ordinal 0)) (authored-target "P") (outcome (status resolved) (target (node (document "d0") (qualified-name "IndividualTest::P")))))
    (reference (id (source (node (document "d0") (qualified-name "IndividualTest::IP2::p"))) (kind featureTyping) (ordinal 0)) (authored-target "IP2") (outcome (status resolved) (target (node (document "d0") (qualified-name "IndividualTest::IP2")))))
    (reference (id (source (node (document "d0") (qualified-name "IndividualTest::P::p"))) (kind featureTyping) (ordinal 0)) (authored-target "P") (outcome (status resolved) (target (node (document "d0") (qualified-name "IndividualTest::P")))))
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
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 11 11) (end 11 12)) (probe (position 11 11))
      (reference
        (source (document "d0") (qualified-name "IndividualTest::I::i"))
        (kind featureTyping) (ordinal 0) (authored-target "I")
        (range (start 11 11) (end 11 12))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "IndividualTest::I") (range (start 10 1) (end 10 30)))
        )
      )
    )
    (query (range (start 22 11) (end 22 12)) (probe (position 22 11))
      (reference
        (source (document "d0") (qualified-name "IndividualTest::P::p"))
        (kind featureTyping) (ordinal 0) (authored-target "P")
        (range (start 22 11) (end 22 12))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "IndividualTest::P") (range (start 21 1) (end 21 30)))
        )
      )
    )
    (query (range (start 24 28) (end 24 29)) (probe (position 24 28))
      (reference
        (source (document "d0") (qualified-name "IndividualTest::IP2"))
        (kind specialization) (ordinal 0) (authored-target "P")
        (range (start 24 28) (end 24 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "IndividualTest::P") (range (start 21 1) (end 21 30)))
        )
      )
    )
    (query (range (start 18 22) (end 18 25)) (probe (position 18 22))
      (reference
        (source (document "d0") (qualified-name "IndividualTest::IP1::p"))
        (kind featureTyping) (ordinal 0) (authored-target "IP1")
        (range (start 18 22) (end 18 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "IndividualTest::IP1") (range (start 17 1) (end 17 56)))
        )
      )
    )
    (query (range (start 25 26) (end 25 29)) (probe (position 25 26))
      (reference
        (source (document "d0") (qualified-name "IndividualTest::IP2::p"))
        (kind featureTyping) (ordinal 0) (authored-target "IP2")
        (range (start 25 26) (end 25 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "IndividualTest::IP2") (range (start 24 1) (end 24 65)))
        )
      )
    )
  )
)
~~~
