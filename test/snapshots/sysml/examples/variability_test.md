# META
~~~ini
description=SysML Example (Simple Tests): VariabilityTest
type=file
~~~
# SOURCE
~~~sysml
package VariabilityTest {
	part def P {
		attribute a;
	}
	
	part def Q :> P;
	attribute def B;
	variation part def V :> P {
		variant part x : Q {
			attribute b : B :>> a;
		}
	}
	
	part q : Q;
	variation part v : P {
		variant q {
			attribute b : B :>> a;
		}
	}
	
	part y : P = v::q;
	
	variation action def A {
		variant action a1;
		variant action a2;
	}
	
	variation use case uc1 {
    	variant use case uc11;
    	variant use case uc12;
    }

    variation analysis a1;
    
    variation verification v1;
    
    variation requirement r {
    	variant requirement r1;
    }
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "variability_test.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "sysml")
        (range (start 15 2) (end 15 45))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 20 1) (end 20 19))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "c6a33e2b494e3d4360d961e3980c292998d2ef4206c8b486f89730483d4f43c5") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "VariabilityTest"))) (kind "package") (name "VariabilityTest") (declared-name "VariabilityTest"))
    (element (id (node (document "d0") (qualified-name "VariabilityTest::A"))) (kind "kermlDecl") (name "A") (declared-name "A") (parent (node (document "d0") (qualified-name "VariabilityTest"))))
    (element (id (node (document "d0") (qualified-name "VariabilityTest::B"))) (kind "attribute def") (name "B") (declared-name "B") (parent (node (document "d0") (qualified-name "VariabilityTest"))))
    (element (id (node (document "d0") (qualified-name "VariabilityTest::P"))) (kind "part def") (name "P") (declared-name "P") (parent (node (document "d0") (qualified-name "VariabilityTest"))))
    (element (id (node (document "d0") (qualified-name "VariabilityTest::P::a"))) (kind "attribute") (name "a") (declared-name "a") (parent (node (document "d0") (qualified-name "VariabilityTest::P"))))
    (element (id (node (document "d0") (qualified-name "VariabilityTest::Q"))) (kind "part def") (name "Q") (declared-name "Q") (parent (node (document "d0") (qualified-name "VariabilityTest"))) (authored (membership (kind Owning)) (relationships (specializes (reference "P")))))
    (element (id (node (document "d0") (qualified-name "VariabilityTest::V"))) (kind "part def") (name "V") (declared-name "V") (parent (node (document "d0") (qualified-name "VariabilityTest"))) (authored (membership (kind Owning)) (relationships (specializes (reference "P")))))
    (element (id (node (document "d0") (qualified-name "VariabilityTest::V::x"))) (kind "part") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "VariabilityTest::V"))) (authored (membership (kind Feature)) (relationships (typing (reference "Q")))))
    (element (id (node (document "d0") (qualified-name "VariabilityTest::V::x::b"))) (kind "attribute") (name "b") (declared-name "b") (parent (node (document "d0") (qualified-name "VariabilityTest::V::x"))) (authored (membership (kind Feature)) (relationships (typing (reference "B")) (typing (reference "B")) (redefinition (reference "a")))))
    (element (id (node (document "d0") (qualified-name "VariabilityTest::a1"))) (kind "kermlDecl") (name "a1") (declared-name "a1") (parent (node (document "d0") (qualified-name "VariabilityTest"))))
    (element (id (node (document "d0") (qualified-name "VariabilityTest::q"))) (kind "part") (name "q") (declared-name "q") (parent (node (document "d0") (qualified-name "VariabilityTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "Q")))))
    (element (id (node (document "d0") (qualified-name "VariabilityTest::r"))) (kind "requirement") (name "r") (declared-name "r") (parent (node (document "d0") (qualified-name "VariabilityTest"))))
    (element (id (node (document "d0") (qualified-name "VariabilityTest::uc1"))) (kind "kermlDecl") (name "uc1") (declared-name "uc1") (parent (node (document "d0") (qualified-name "VariabilityTest"))))
    (element (id (node (document "d0") (qualified-name "VariabilityTest::v"))) (kind "part") (name "v") (declared-name "v") (parent (node (document "d0") (qualified-name "VariabilityTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "P")))))
    (element (id (node (document "d0") (qualified-name "VariabilityTest::v1"))) (kind "kermlDecl") (name "v1") (declared-name "v1") (parent (node (document "d0") (qualified-name "VariabilityTest"))))
    (element (id (node (document "d0") (qualified-name "VariabilityTest::y"))) (kind "part") (name "y") (declared-name "y") (parent (node (document "d0") (qualified-name "VariabilityTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "P")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "VariabilityTest::Q"))) (kind specialization) (ordinal 0)) (authored-target "P") (outcome (status resolved) (target (node (document "d0") (qualified-name "VariabilityTest::P")))))
    (reference (id (source (node (document "d0") (qualified-name "VariabilityTest::V"))) (kind specialization) (ordinal 0)) (authored-target "P") (outcome (status resolved) (target (node (document "d0") (qualified-name "VariabilityTest::P")))))
    (reference (id (source (node (document "d0") (qualified-name "VariabilityTest::V::x"))) (kind featureTyping) (ordinal 0)) (authored-target "Q") (outcome (status resolved) (target (node (document "d0") (qualified-name "VariabilityTest::Q")))))
    (reference (id (source (node (document "d0") (qualified-name "VariabilityTest::V::x::b"))) (kind featureTyping) (ordinal 0)) (authored-target "B") (outcome (status resolved) (target (node (document "d0") (qualified-name "VariabilityTest::B")))))
    (reference (id (source (node (document "d0") (qualified-name "VariabilityTest::V::x::b"))) (kind featureTyping) (ordinal 1)) (authored-target "B") (outcome (status resolved) (target (node (document "d0") (qualified-name "VariabilityTest::B")))))
    (reference (id (source (node (document "d0") (qualified-name "VariabilityTest::V::x::b"))) (kind redefinition) (ordinal 0)) (authored-target "a") (outcome (status resolved) (target (node (document "d0") (qualified-name "VariabilityTest::P::a")))))
    (reference (id (source (node (document "d0") (qualified-name "VariabilityTest::q"))) (kind featureTyping) (ordinal 0)) (authored-target "Q") (outcome (status resolved) (target (node (document "d0") (qualified-name "VariabilityTest::Q")))))
    (reference (id (source (node (document "d0") (qualified-name "VariabilityTest::v"))) (kind featureTyping) (ordinal 0)) (authored-target "P") (outcome (status resolved) (target (node (document "d0") (qualified-name "VariabilityTest::P")))))
    (reference (id (source (node (document "d0") (qualified-name "VariabilityTest::y"))) (kind featureTyping) (ordinal 0)) (authored-target "P") (outcome (status resolved) (target (node (document "d0") (qualified-name "VariabilityTest::P")))))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "VariabilityTest::Q"))) (target (node (document "d0") (qualified-name "VariabilityTest::P"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VariabilityTest::Q"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "VariabilityTest::V"))) (target (node (document "d0") (qualified-name "VariabilityTest::P"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VariabilityTest::V"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VariabilityTest::V::x"))) (target (node (document "d0") (qualified-name "VariabilityTest::Q"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VariabilityTest::V::x"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VariabilityTest::V::x::b"))) (target (node (document "d0") (qualified-name "VariabilityTest::B"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VariabilityTest::V::x::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VariabilityTest::V::x::b"))) (target (node (document "d0") (qualified-name "VariabilityTest::B"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VariabilityTest::V::x::b"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "VariabilityTest::V::x::b"))) (target (node (document "d0") (qualified-name "VariabilityTest::P::a"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VariabilityTest::V::x::b"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VariabilityTest::q"))) (target (node (document "d0") (qualified-name "VariabilityTest::Q"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VariabilityTest::q"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VariabilityTest::v"))) (target (node (document "d0") (qualified-name "VariabilityTest::P"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VariabilityTest::v"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VariabilityTest::y"))) (target (node (document "d0") (qualified-name "VariabilityTest::P"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VariabilityTest::y"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "VariabilityTest::y")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 5 15) (end 5 16)) (probe (position 5 15))
      (reference
        (source (document "d0") (qualified-name "VariabilityTest::Q"))
        (kind specialization) (ordinal 0) (authored-target "P")
        (range (start 5 15) (end 5 16))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VariabilityTest::P") (range (start 1 1) (end 1 31)))
        )
      )
    )
    (query (range (start 7 25) (end 7 26)) (probe (position 7 25))
      (reference
        (source (document "d0") (qualified-name "VariabilityTest::V"))
        (kind specialization) (ordinal 0) (authored-target "P")
        (range (start 7 25) (end 7 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VariabilityTest::P") (range (start 1 1) (end 1 31)))
        )
      )
    )
    (query (range (start 8 19) (end 8 20)) (probe (position 8 19))
      (reference
        (source (document "d0") (qualified-name "VariabilityTest::V::x"))
        (kind featureTyping) (ordinal 0) (authored-target "Q")
        (range (start 8 19) (end 8 20))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VariabilityTest::Q") (range (start 5 1) (end 5 17)))
        )
      )
    )
    (query (range (start 9 17) (end 9 18)) (probe (position 9 17))
      (reference
        (source (document "d0") (qualified-name "VariabilityTest::V::x::b"))
        (kind featureTyping) (ordinal 1) (authored-target "B")
        (range (start 9 17) (end 9 18))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VariabilityTest::B") (range (start 6 1) (end 6 17)))
        )
      )
    )
    (query (range (start 9 23) (end 9 24)) (probe (position 9 23))
      (reference
        (source (document "d0") (qualified-name "VariabilityTest::V::x::b"))
        (kind redefinition) (ordinal 0) (authored-target "a")
        (range (start 9 23) (end 9 24))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VariabilityTest::P::a") (range (start 2 2) (end 2 14)))
        )
      )
    )
    (query (range (start 13 10) (end 13 11)) (probe (position 13 10))
      (reference
        (source (document "d0") (qualified-name "VariabilityTest::q"))
        (kind featureTyping) (ordinal 0) (authored-target "Q")
        (range (start 13 10) (end 13 11))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VariabilityTest::Q") (range (start 5 1) (end 5 17)))
        )
      )
    )
    (query (range (start 14 20) (end 14 21)) (probe (position 14 20))
      (reference
        (source (document "d0") (qualified-name "VariabilityTest::v"))
        (kind featureTyping) (ordinal 0) (authored-target "P")
        (range (start 14 20) (end 14 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VariabilityTest::P") (range (start 1 1) (end 1 31)))
        )
      )
    )
    (query (range (start 20 10) (end 20 11)) (probe (position 20 10))
      (reference
        (source (document "d0") (qualified-name "VariabilityTest::y"))
        (kind featureTyping) (ordinal 0) (authored-target "P")
        (range (start 20 10) (end 20 11))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VariabilityTest::P") (range (start 1 1) (end 1 31)))
        )
      )
    )
  )
)
~~~
