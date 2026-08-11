# META
~~~ini
description=SysML Example (Simple Tests): AllocationTest
type=file
~~~
# SOURCE
~~~sysml
package AllocationTest {
	part def Logical {
		part component;
	}
	
	part def Physical {
		part assembly {
			part element;
		}
	}
	
	part l : Logical {
		part :>> component;
	}
	part p : Physical {
		part :>> assembly {
			part :>> element;
		}
        allocate l.component to assembly.element;
	}
	
	allocation def A;
	
	allocation def Logical_to_Physical :> A {
		end logical : Logical;
		end physical : Physical;
	}
	
	allocation allocation1 : Logical_to_Physical allocate l to p;	
	allocation allocation2 : Logical_to_Physical allocate (
		logical ::> l,
		physical ::> p
	);

	allocate l.component to p.assembly.element;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "allocation_test.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 2 2) (end 2 17))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 7 3) (end 7 16))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package AllocationTest {
	part def Logical {
		part component;
	}
	
	part def Physical {
		part assembly {
			part element;
		}
	}
	
	part l : Logical {
		part :>> component;
	}
	part p : Physical {
		part :>> assembly {
			part :>> element;
		}
        allocate l.component to assembly.element;
	}
	
	allocation def A;
	
	allocation def Logical_to_Physical :> A {
		end logical : Logical;
		end physical : Physical;
	}
	
	allocation allocation1 : Logical_to_Physical allocate l to p;	
	allocation allocation2 : Logical_to_Physical allocate (
		logical ::> l,
		physical ::> p
	);

	allocate l.component to p.assembly.element;
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "0c0bb7b4ec74566787a3d1321e44606474d252559ba10a6208233eda2a0275d4") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "AllocationTest"))) (kind "package") (name "AllocationTest") (declared-name "AllocationTest"))
    (element (id (node (document "d0") (qualified-name "AllocationTest::"))) (kind "allocation") (name "") (parent (node (document "d0") (qualified-name "AllocationTest"))))
    (element (id (node (document "d0") (qualified-name "AllocationTest::A"))) (kind "allocation def") (name "A") (declared-name "A") (parent (node (document "d0") (qualified-name "AllocationTest"))))
    (element (id (node (document "d0") (qualified-name "AllocationTest::Logical"))) (kind "part def") (name "Logical") (declared-name "Logical") (parent (node (document "d0") (qualified-name "AllocationTest"))))
    (element (id (node (document "d0") (qualified-name "AllocationTest::Logical::component"))) (kind "part") (name "component") (declared-name "component") (parent (node (document "d0") (qualified-name "AllocationTest::Logical"))))
    (element (id (node (document "d0") (qualified-name "AllocationTest::Logical_to_Physical"))) (kind "allocation def") (name "Logical_to_Physical") (declared-name "Logical_to_Physical") (parent (node (document "d0") (qualified-name "AllocationTest"))) (authored (membership (kind Owning)) (relationships (specializes (reference "A")))))
    (element (id (node (document "d0") (qualified-name "AllocationTest::Logical_to_Physical::logical"))) (kind "interface end") (name "logical") (declared-name "logical") (parent (node (document "d0") (qualified-name "AllocationTest::Logical_to_Physical"))) (authored (relationships (typing (reference "Logical")))))
    (element (id (node (document "d0") (qualified-name "AllocationTest::Logical_to_Physical::physical"))) (kind "interface end") (name "physical") (declared-name "physical") (parent (node (document "d0") (qualified-name "AllocationTest::Logical_to_Physical"))) (authored (relationships (typing (reference "Physical")))))
    (element (id (node (document "d0") (qualified-name "AllocationTest::Physical"))) (kind "part def") (name "Physical") (declared-name "Physical") (parent (node (document "d0") (qualified-name "AllocationTest"))))
    (element (id (node (document "d0") (qualified-name "AllocationTest::Physical::assembly"))) (kind "part") (name "assembly") (declared-name "assembly") (parent (node (document "d0") (qualified-name "AllocationTest::Physical"))))
    (element (id (node (document "d0") (qualified-name "AllocationTest::Physical::assembly::element"))) (kind "part") (name "element") (declared-name "element") (parent (node (document "d0") (qualified-name "AllocationTest::Physical::assembly"))))
    (element (id (node (document "d0") (qualified-name "AllocationTest::allocation1"))) (kind "allocation") (name "allocation1") (declared-name "allocation1") (parent (node (document "d0") (qualified-name "AllocationTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "Logical_to_Physical")))))
    (element (id (node (document "d0") (qualified-name "AllocationTest::allocation2"))) (kind "kermlDecl") (name "allocation2") (declared-name "allocation2") (parent (node (document "d0") (qualified-name "AllocationTest"))))
    (element (id (node (document "d0") (qualified-name "AllocationTest::l"))) (kind "part") (name "l") (declared-name "l") (parent (node (document "d0") (qualified-name "AllocationTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "Logical")))))
    (element (id (node (document "d0") (qualified-name "AllocationTest::l::component"))) (kind "part") (name "component") (parent (node (document "d0") (qualified-name "AllocationTest::l"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "component")))))
    (element (id (node (document "d0") (qualified-name "AllocationTest::p"))) (kind "part") (name "p") (declared-name "p") (parent (node (document "d0") (qualified-name "AllocationTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "Physical")))))
    (element (id (node (document "d0") (qualified-name "AllocationTest::p::assembly"))) (kind "part") (name "assembly") (parent (node (document "d0") (qualified-name "AllocationTest::p"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "assembly")))))
    (element (id (node (document "d0") (qualified-name "AllocationTest::p::assembly::element"))) (kind "part") (name "element") (parent (node (document "d0") (qualified-name "AllocationTest::p::assembly"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "element")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "AllocationTest"))) (kind allocateSource) (ordinal 0)) (authored-target "l") (outcome (status resolved) (target (node (document "d0") (qualified-name "AllocationTest::l")))))
    (reference (id (source (node (document "d0") (qualified-name "AllocationTest"))) (kind allocateSource) (ordinal 1)) (authored-target "l::component") (outcome (status resolved) (target (node (document "d0") (qualified-name "AllocationTest::l::component")))))
    (reference (id (source (node (document "d0") (qualified-name "AllocationTest"))) (kind allocateTarget) (ordinal 0)) (authored-target "p") (outcome (status resolved) (target (node (document "d0") (qualified-name "AllocationTest::p")))))
    (reference (id (source (node (document "d0") (qualified-name "AllocationTest"))) (kind allocateTarget) (ordinal 1)) (authored-target "p::assembly::element") (outcome (status resolved) (target (node (document "d0") (qualified-name "AllocationTest::p::assembly::element")))))
    (reference (id (source (node (document "d0") (qualified-name "AllocationTest::Logical_to_Physical"))) (kind specialization) (ordinal 0)) (authored-target "A") (outcome (status resolved) (target (node (document "d0") (qualified-name "AllocationTest::A")))))
    (reference (id (source (node (document "d0") (qualified-name "AllocationTest::Logical_to_Physical::logical"))) (kind featureTyping) (ordinal 0)) (authored-target "Logical") (outcome (status resolved) (target (node (document "d0") (qualified-name "AllocationTest::Logical")))))
    (reference (id (source (node (document "d0") (qualified-name "AllocationTest::Logical_to_Physical::physical"))) (kind featureTyping) (ordinal 0)) (authored-target "Physical") (outcome (status resolved) (target (node (document "d0") (qualified-name "AllocationTest::Physical")))))
    (reference (id (source (node (document "d0") (qualified-name "AllocationTest::allocation1"))) (kind featureTyping) (ordinal 0)) (authored-target "Logical_to_Physical") (outcome (status resolved) (target (node (document "d0") (qualified-name "AllocationTest::Logical_to_Physical")))))
    (reference (id (source (node (document "d0") (qualified-name "AllocationTest::l"))) (kind featureTyping) (ordinal 0)) (authored-target "Logical") (outcome (status resolved) (target (node (document "d0") (qualified-name "AllocationTest::Logical")))))
    (reference (id (source (node (document "d0") (qualified-name "AllocationTest::l::component"))) (kind redefinition) (ordinal 0)) (authored-target "component") (outcome (status resolved) (target (node (document "d0") (qualified-name "AllocationTest::l::component")))))
    (reference (id (source (node (document "d0") (qualified-name "AllocationTest::p"))) (kind featureTyping) (ordinal 0)) (authored-target "Physical") (outcome (status resolved) (target (node (document "d0") (qualified-name "AllocationTest::Physical")))))
    (reference (id (source (node (document "d0") (qualified-name "AllocationTest::p"))) (kind allocateSource) (ordinal 0)) (authored-target "l::component") (outcome (status resolved) (target (node (document "d0") (qualified-name "AllocationTest::l::component")))))
    (reference (id (source (node (document "d0") (qualified-name "AllocationTest::p"))) (kind allocateTarget) (ordinal 0)) (authored-target "assembly::element") (outcome (status resolved) (target (node (document "d0") (qualified-name "AllocationTest::p::assembly::element")))))
    (reference (id (source (node (document "d0") (qualified-name "AllocationTest::p::assembly"))) (kind redefinition) (ordinal 0)) (authored-target "assembly") (outcome (status resolved) (target (node (document "d0") (qualified-name "AllocationTest::p::assembly")))))
    (reference (id (source (node (document "d0") (qualified-name "AllocationTest::p::assembly::element"))) (kind redefinition) (ordinal 0)) (authored-target "element") (outcome (status resolved) (target (node (document "d0") (qualified-name "AllocationTest::p::assembly::element")))))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "AllocationTest::Logical_to_Physical"))) (target (node (document "d0") (qualified-name "AllocationTest::A"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AllocationTest::Logical_to_Physical"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "AllocationTest::Logical_to_Physical::logical"))) (target (node (document "d0") (qualified-name "AllocationTest::Logical"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AllocationTest::Logical_to_Physical::logical"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "AllocationTest::Logical_to_Physical::physical"))) (target (node (document "d0") (qualified-name "AllocationTest::Physical"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AllocationTest::Logical_to_Physical::physical"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "AllocationTest::allocation1"))) (target (node (document "d0") (qualified-name "AllocationTest::Logical_to_Physical"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AllocationTest::allocation1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "AllocationTest::l"))) (target (node (document "d0") (qualified-name "AllocationTest::Logical"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AllocationTest::l"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind allocate) (source (node (document "d0") (qualified-name "AllocationTest::l"))) (target (node (document "d0") (qualified-name "AllocationTest::p"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AllocationTest"))) (kind allocateSource) (ordinal 0)) (expression (kind allocate) (source "l") (target "p")))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "AllocationTest::l::component"))) (target (node (document "d0") (qualified-name "AllocationTest::l::component"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AllocationTest::l::component"))) (kind redefinition) (ordinal 0)))
    (relationship (kind allocate) (source (node (document "d0") (qualified-name "AllocationTest::l::component"))) (target (node (document "d0") (qualified-name "AllocationTest::p::assembly::element"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AllocationTest"))) (kind allocateSource) (ordinal 1)) (expression (kind allocate) (source "l::component") (target "p::assembly::element")))
    (relationship (kind allocate) (source (node (document "d0") (qualified-name "AllocationTest::l::component"))) (target (node (document "d0") (qualified-name "AllocationTest::p::assembly::element"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AllocationTest::p"))) (kind allocateSource) (ordinal 0)) (expression (kind allocate) (source "l::component") (target "assembly::element")))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "AllocationTest::p"))) (target (node (document "d0") (qualified-name "AllocationTest::Physical"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AllocationTest::p"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "AllocationTest::p::assembly"))) (target (node (document "d0") (qualified-name "AllocationTest::p::assembly"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AllocationTest::p::assembly"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "AllocationTest::p::assembly::element"))) (target (node (document "d0") (qualified-name "AllocationTest::p::assembly::element"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AllocationTest::p::assembly::element"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 23 39) (end 23 40)) (probe (position 23 39))
      (reference
        (source (document "d0") (qualified-name "AllocationTest::Logical_to_Physical"))
        (kind specialization) (ordinal 0) (authored-target "A")
        (range (start 23 39) (end 23 40))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AllocationTest::A") (range (start 21 1) (end 21 18)))
        )
      )
    )
    (query (range (start 28 55) (end 28 56)) (probe (position 28 55))
      (reference
        (source (document "d0") (qualified-name "AllocationTest"))
        (kind allocateSource) (ordinal 0) (authored-target "l")
        (range (start 28 55) (end 28 56))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AllocationTest::l") (range (start 11 1) (end 11 44)))
        )
      )
    )
    (query (range (start 28 60) (end 28 61)) (probe (position 28 60))
      (reference
        (source (document "d0") (qualified-name "AllocationTest"))
        (kind allocateTarget) (ordinal 0) (authored-target "p")
        (range (start 28 60) (end 28 61))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AllocationTest::p") (range (start 14 1) (end 14 120)))
        )
      )
    )
    (query (range (start 11 10) (end 11 17)) (probe (position 11 10))
      (reference
        (source (document "d0") (qualified-name "AllocationTest::l"))
        (kind featureTyping) (ordinal 0) (authored-target "Logical")
        (range (start 11 10) (end 11 17))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AllocationTest::Logical") (range (start 1 1) (end 1 40)))
        )
      )
    )
    (query (range (start 16 12) (end 16 19)) (probe (position 16 12))
      (reference
        (source (document "d0") (qualified-name "AllocationTest::p::assembly::element"))
        (kind redefinition) (ordinal 0) (authored-target "element")
        (range (start 16 12) (end 16 19))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AllocationTest::p::assembly::element") (range (start 16 3) (end 16 20)))
        )
      )
    )
    (query (range (start 14 10) (end 14 18)) (probe (position 14 10))
      (reference
        (source (document "d0") (qualified-name "AllocationTest::p"))
        (kind featureTyping) (ordinal 0) (authored-target "Physical")
        (range (start 14 10) (end 14 18))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AllocationTest::Physical") (range (start 5 1) (end 5 62)))
        )
      )
    )
    (query (range (start 15 11) (end 15 19)) (probe (position 15 11))
      (reference
        (source (document "d0") (qualified-name "AllocationTest::p::assembly"))
        (kind redefinition) (ordinal 0) (authored-target "assembly")
        (range (start 15 11) (end 15 19))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AllocationTest::p::assembly") (range (start 15 2) (end 15 46)))
        )
      )
    )
    (query (range (start 12 11) (end 12 20)) (probe (position 12 11))
      (reference
        (source (document "d0") (qualified-name "AllocationTest::l::component"))
        (kind redefinition) (ordinal 0) (authored-target "component")
        (range (start 12 11) (end 12 20))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AllocationTest::l::component") (range (start 12 2) (end 12 21)))
        )
      )
    )
    (query (range (start 18 17) (end 18 28)) (probe (position 18 17))
      (reference
        (source (document "d0") (qualified-name "AllocationTest::p"))
        (kind allocateSource) (ordinal 0) (authored-target "l::component")
        (range (start 18 17) (end 18 28))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AllocationTest::l::component") (range (start 12 2) (end 12 21)))
        )
      )
    )
    (query (range (start 34 10) (end 34 21)) (probe (position 34 10))
      (reference
        (source (document "d0") (qualified-name "AllocationTest"))
        (kind allocateSource) (ordinal 1) (authored-target "l::component")
        (range (start 34 10) (end 34 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AllocationTest::l::component") (range (start 12 2) (end 12 21)))
        )
      )
    )
    (query (range (start 18 32) (end 18 48)) (probe (position 18 32))
      (reference
        (source (document "d0") (qualified-name "AllocationTest::p"))
        (kind allocateTarget) (ordinal 0) (authored-target "assembly::element")
        (range (start 18 32) (end 18 48))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AllocationTest::p::assembly::element") (range (start 16 3) (end 16 20)))
        )
      )
    )
    (query (range (start 34 25) (end 34 43)) (probe (position 34 25))
      (reference
        (source (document "d0") (qualified-name "AllocationTest"))
        (kind allocateTarget) (ordinal 1) (authored-target "p::assembly::element")
        (range (start 34 25) (end 34 43))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AllocationTest::p::assembly::element") (range (start 16 3) (end 16 20)))
        )
      )
    )
  )
)
~~~
