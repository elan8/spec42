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
    (element (id (node (document "d0") (qualified-name "AllocationTest"))) (kind "package") (name "AllocationTest") (declared-name "AllocationTest") (range (start (line 0) (character 0)) (end (line 0) (character 628))))
    (element (id (node (document "d0") (qualified-name "AllocationTest::"))) (kind "allocation") (name "") (range (start (line 34) (character 1)) (end (line 34) (character 44))) (parent (node (document "d0") (qualified-name "AllocationTest"))))
    (element (id (node (document "d0") (qualified-name "AllocationTest::A"))) (kind "allocation def") (name "A") (declared-name "A") (range (start (line 21) (character 1)) (end (line 21) (character 18))) (parent (node (document "d0") (qualified-name "AllocationTest"))))
    (element (id (node (document "d0") (qualified-name "AllocationTest::Logical"))) (kind "part def") (name "Logical") (declared-name "Logical") (range (start (line 1) (character 1)) (end (line 1) (character 40))) (parent (node (document "d0") (qualified-name "AllocationTest"))))
    (element (id (node (document "d0") (qualified-name "AllocationTest::Logical::component"))) (kind "part") (name "component") (declared-name "component") (range (start (line 2) (character 2)) (end (line 2) (character 17))) (parent (node (document "d0") (qualified-name "AllocationTest::Logical"))))
    (element (id (node (document "d0") (qualified-name "AllocationTest::Logical_to_Physical"))) (kind "allocation def") (name "Logical_to_Physical") (declared-name "Logical_to_Physical") (range (start (line 23) (character 1)) (end (line 23) (character 97))) (parent (node (document "d0") (qualified-name "AllocationTest"))) (authored (membership (kind Owning)) (relationships (specializes (reference "A") (range (start (line 23) (character 39)) (end (line 23) (character 40)))))))
    (element (id (node (document "d0") (qualified-name "AllocationTest::Logical_to_Physical::logical"))) (kind "interface end") (name "logical") (declared-name "logical") (range (start (line 24) (character 2)) (end (line 24) (character 24))) (parent (node (document "d0") (qualified-name "AllocationTest::Logical_to_Physical"))) (authored (relationships (typing (reference "Logical") (range none)))))
    (element (id (node (document "d0") (qualified-name "AllocationTest::Logical_to_Physical::physical"))) (kind "interface end") (name "physical") (declared-name "physical") (range (start (line 25) (character 2)) (end (line 25) (character 26))) (parent (node (document "d0") (qualified-name "AllocationTest::Logical_to_Physical"))) (authored (relationships (typing (reference "Physical") (range none)))))
    (element (id (node (document "d0") (qualified-name "AllocationTest::Physical"))) (kind "part def") (name "Physical") (declared-name "Physical") (range (start (line 5) (character 1)) (end (line 5) (character 62))) (parent (node (document "d0") (qualified-name "AllocationTest"))))
    (element (id (node (document "d0") (qualified-name "AllocationTest::Physical::assembly"))) (kind "part") (name "assembly") (declared-name "assembly") (range (start (line 6) (character 2)) (end (line 6) (character 38))) (parent (node (document "d0") (qualified-name "AllocationTest::Physical"))))
    (element (id (node (document "d0") (qualified-name "AllocationTest::Physical::assembly::element"))) (kind "part") (name "element") (declared-name "element") (range (start (line 7) (character 3)) (end (line 7) (character 16))) (parent (node (document "d0") (qualified-name "AllocationTest::Physical::assembly"))))
    (element (id (node (document "d0") (qualified-name "AllocationTest::allocation1"))) (kind "allocation") (name "allocation1") (declared-name "allocation1") (range (start (line 28) (character 1)) (end (line 28) (character 62))) (parent (node (document "d0") (qualified-name "AllocationTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "Logical_to_Physical") (range none)))))
    (element (id (node (document "d0") (qualified-name "AllocationTest::allocation2"))) (kind "kermlDecl") (name "allocation2") (declared-name "allocation2") (range (start (line 29) (character 1)) (end (line 29) (character 94))) (parent (node (document "d0") (qualified-name "AllocationTest"))))
    (element (id (node (document "d0") (qualified-name "AllocationTest::l"))) (kind "part") (name "l") (declared-name "l") (range (start (line 11) (character 1)) (end (line 11) (character 44))) (parent (node (document "d0") (qualified-name "AllocationTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "Logical") (range (start (line 11) (character 10)) (end (line 11) (character 17)))))))
    (element (id (node (document "d0") (qualified-name "AllocationTest::l::component"))) (kind "part") (name "component") (range (start (line 12) (character 2)) (end (line 12) (character 21))) (parent (node (document "d0") (qualified-name "AllocationTest::l"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "component") (range (start (line 12) (character 11)) (end (line 12) (character 20)))))))
    (element (id (node (document "d0") (qualified-name "AllocationTest::p"))) (kind "part") (name "p") (declared-name "p") (range (start (line 14) (character 1)) (end (line 14) (character 120))) (parent (node (document "d0") (qualified-name "AllocationTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "Physical") (range (start (line 14) (character 10)) (end (line 14) (character 18)))))))
    (element (id (node (document "d0") (qualified-name "AllocationTest::p::assembly"))) (kind "part") (name "assembly") (range (start (line 15) (character 2)) (end (line 15) (character 46))) (parent (node (document "d0") (qualified-name "AllocationTest::p"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "assembly") (range (start (line 15) (character 11)) (end (line 15) (character 19)))))))
    (element (id (node (document "d0") (qualified-name "AllocationTest::p::assembly::element"))) (kind "part") (name "element") (range (start (line 16) (character 3)) (end (line 16) (character 20))) (parent (node (document "d0") (qualified-name "AllocationTest::p::assembly"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "element") (range (start (line 16) (character 12)) (end (line 16) (character 19)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "AllocationTest"))) (kind allocateSource) (ordinal 0)) (authored-target "l") (range (start (line 28) (character 55)) (end (line 28) (character 56))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AllocationTest::l")))))
    (reference (id (source (node (document "d0") (qualified-name "AllocationTest"))) (kind allocateSource) (ordinal 1)) (authored-target "l::component") (range (start (line 34) (character 10)) (end (line 34) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AllocationTest::l::component")))))
    (reference (id (source (node (document "d0") (qualified-name "AllocationTest"))) (kind allocateTarget) (ordinal 0)) (authored-target "p") (range (start (line 28) (character 60)) (end (line 28) (character 61))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AllocationTest::p")))))
    (reference (id (source (node (document "d0") (qualified-name "AllocationTest"))) (kind allocateTarget) (ordinal 1)) (authored-target "p::assembly::element") (range (start (line 34) (character 25)) (end (line 34) (character 43))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AllocationTest::p::assembly::element")))))
    (reference (id (source (node (document "d0") (qualified-name "AllocationTest::Logical_to_Physical"))) (kind specialization) (ordinal 0)) (authored-target "A") (range (start (line 23) (character 39)) (end (line 23) (character 40))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AllocationTest::A")))))
    (reference (id (source (node (document "d0") (qualified-name "AllocationTest::Logical_to_Physical::logical"))) (kind featureTyping) (ordinal 0)) (authored-target "Logical") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "AllocationTest::Logical")))))
    (reference (id (source (node (document "d0") (qualified-name "AllocationTest::Logical_to_Physical::physical"))) (kind featureTyping) (ordinal 0)) (authored-target "Physical") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "AllocationTest::Physical")))))
    (reference (id (source (node (document "d0") (qualified-name "AllocationTest::allocation1"))) (kind featureTyping) (ordinal 0)) (authored-target "Logical_to_Physical") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "AllocationTest::Logical_to_Physical")))))
    (reference (id (source (node (document "d0") (qualified-name "AllocationTest::l"))) (kind featureTyping) (ordinal 0)) (authored-target "Logical") (range (start (line 11) (character 10)) (end (line 11) (character 17))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AllocationTest::Logical")))))
    (reference (id (source (node (document "d0") (qualified-name "AllocationTest::l::component"))) (kind redefinition) (ordinal 0)) (authored-target "component") (range (start (line 12) (character 11)) (end (line 12) (character 20))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AllocationTest::l::component")))))
    (reference (id (source (node (document "d0") (qualified-name "AllocationTest::p"))) (kind featureTyping) (ordinal 0)) (authored-target "Physical") (range (start (line 14) (character 10)) (end (line 14) (character 18))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AllocationTest::Physical")))))
    (reference (id (source (node (document "d0") (qualified-name "AllocationTest::p"))) (kind allocateSource) (ordinal 0)) (authored-target "l::component") (range (start (line 18) (character 17)) (end (line 18) (character 28))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AllocationTest::l::component")))))
    (reference (id (source (node (document "d0") (qualified-name "AllocationTest::p"))) (kind allocateTarget) (ordinal 0)) (authored-target "assembly::element") (range (start (line 18) (character 32)) (end (line 18) (character 48))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AllocationTest::p::assembly::element")))))
    (reference (id (source (node (document "d0") (qualified-name "AllocationTest::p::assembly"))) (kind redefinition) (ordinal 0)) (authored-target "assembly") (range (start (line 15) (character 11)) (end (line 15) (character 19))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AllocationTest::p::assembly")))))
    (reference (id (source (node (document "d0") (qualified-name "AllocationTest::p::assembly::element"))) (kind redefinition) (ordinal 0)) (authored-target "element") (range (start (line 16) (character 12)) (end (line 16) (character 19))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AllocationTest::p::assembly::element")))))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "AllocationTest::Logical_to_Physical"))) (target (node (document "d0") (qualified-name "AllocationTest::A"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AllocationTest::Logical_to_Physical"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "AllocationTest::Logical_to_Physical::logical"))) (target (node (document "d0") (qualified-name "AllocationTest::Logical"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AllocationTest::Logical_to_Physical::logical"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "AllocationTest::Logical_to_Physical::physical"))) (target (node (document "d0") (qualified-name "AllocationTest::Physical"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AllocationTest::Logical_to_Physical::physical"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "AllocationTest::allocation1"))) (target (node (document "d0") (qualified-name "AllocationTest::Logical_to_Physical"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AllocationTest::allocation1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "AllocationTest::l"))) (target (node (document "d0") (qualified-name "AllocationTest::Logical"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AllocationTest::l"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind allocate) (source (node (document "d0") (qualified-name "AllocationTest::l"))) (target (node (document "d0") (qualified-name "AllocationTest::p"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AllocationTest"))) (kind allocateSource) (ordinal 0)) (expression (kind allocate) (source "l") (target "p") (source-range (start (line 28) (character 55)) (end (line 28) (character 56))) (target-range (start (line 28) (character 60)) (end (line 28) (character 61)))))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "AllocationTest::l::component"))) (target (node (document "d0") (qualified-name "AllocationTest::l::component"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AllocationTest::l::component"))) (kind redefinition) (ordinal 0)))
    (relationship (kind allocate) (source (node (document "d0") (qualified-name "AllocationTest::l::component"))) (target (node (document "d0") (qualified-name "AllocationTest::p::assembly::element"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AllocationTest"))) (kind allocateSource) (ordinal 1)) (expression (kind allocate) (source "l::component") (target "p::assembly::element") (source-range (start (line 34) (character 10)) (end (line 34) (character 21))) (target-range (start (line 34) (character 25)) (end (line 34) (character 43)))))
    (relationship (kind allocate) (source (node (document "d0") (qualified-name "AllocationTest::l::component"))) (target (node (document "d0") (qualified-name "AllocationTest::p::assembly::element"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AllocationTest::p"))) (kind allocateSource) (ordinal 0)) (expression (kind allocate) (source "l::component") (target "assembly::element") (source-range (start (line 18) (character 17)) (end (line 18) (character 28))) (target-range (start (line 18) (character 32)) (end (line 18) (character 48)))))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "AllocationTest::p"))) (target (node (document "d0") (qualified-name "AllocationTest::Physical"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AllocationTest::p"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "AllocationTest::p::assembly"))) (target (node (document "d0") (qualified-name "AllocationTest::p::assembly"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AllocationTest::p::assembly"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "AllocationTest::p::assembly::element"))) (target (node (document "d0") (qualified-name "AllocationTest::p::assembly::element"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AllocationTest::p::assembly::element"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
