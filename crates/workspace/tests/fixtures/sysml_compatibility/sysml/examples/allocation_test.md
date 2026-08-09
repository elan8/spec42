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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPart,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPart,Ident,OpenCurly,
KwPart,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,ColonGtGt,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,ColonGtGt,Ident,OpenCurly,
KwPart,ColonGtGt,Ident,Semicolon,
CloseCurly,
KwAllocate,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwAllocation,KwDef,Ident,Semicolon,
KwAllocation,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwEnd,Ident,Colon,Ident,Semicolon,
KwEnd,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAllocation,Ident,Colon,Ident,KwAllocate,Ident,KwTo,Ident,Semicolon,
KwAllocation,Ident,Colon,Ident,KwAllocate,OpenParen,
Ident,ColonColonGt,Ident,Comma,
Ident,ColonColonGt,Ident,
CloseParen,Semicolon,
KwAllocate,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Dot,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'AllocationTest'
    (part_def 'Logical'
      (part_usage 'component'))
    (part_def 'Physical'
      (part_usage 'assembly'
        (part_usage 'element')))
    (part_usage 'l' : 'Logical'
      (part_usage :>> 'component'))
    (part_usage 'p' : 'Physical'
      (part_usage :>> 'assembly'
        (part_usage :>> 'element'))
      (allocation_usage
        (connector_end)
        (connector_end)))
    (allocation_def 'A')
    (allocation_def 'Logical_to_Physical' :> 'A'
      (interface_end end 'logical' : 'Logical')
      (interface_end end 'physical' : 'Physical'))
    (allocation_usage 'Logical_to_Physical' 'allocation1'
      (connector_end)
      (connector_end))
    (allocation_usage 'Logical_to_Physical' 'allocation2')
    (allocation_usage
      (connector_end)
      (connector_end))))
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
# EXPECTED
~~~
semantic.invalid_connection_end_count
semantic.invalid_allocation_end_count
~~~
# PROBLEMS
~~~
semantic.invalid_connection_end_count
semantic.invalid_allocation_end_count
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "AllocationTest"))) (name "AllocationTest") (declared-name "AllocationTest")
      (contains
        (element (kind "allocation") (id (node (document "d0") (qualified-name "AllocationTest::"))) (name ""))
        (element (kind "allocation def") (id (node (document "d0") (qualified-name "AllocationTest::A"))) (name "A") (declared-name "A"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "AllocationTest::Logical"))) (name "Logical") (declared-name "Logical") (declared)
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "AllocationTest::Logical::component"))) (name "component") (declared-name "component") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "AllocationTest::Logical")))))
          )
        )
        (element (kind "allocation def") (id (node (document "d0") (qualified-name "AllocationTest::Logical_to_Physical"))) (name "Logical_to_Physical") (declared-name "Logical_to_Physical")
          (contains
            (element (kind "interface end") (id (node (document "d0") (qualified-name "AllocationTest::Logical_to_Physical::logical"))) (name "logical") (declared-name "logical") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "AllocationTest::Logical_to_Physical")))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "AllocationTest::Logical_to_Physical::physical"))) (name "physical") (declared-name "physical") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "AllocationTest::Logical_to_Physical")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "AllocationTest::Physical"))) (name "Physical") (declared-name "Physical") (declared)
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "AllocationTest::Physical::assembly"))) (name "assembly") (declared-name "assembly") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "AllocationTest::Physical"))))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "AllocationTest::Physical::assembly::element"))) (name "element") (declared-name "element") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "AllocationTest::Physical")))))
              )
            )
          )
        )
        (element (kind "allocation") (id (node (document "d0") (qualified-name "AllocationTest::allocation1"))) (name "allocation1") (declared-name "allocation1"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "AllocationTest::allocation2"))) (name "allocation2") (declared-name "allocation2"))
        (element (kind "part") (id (node (document "d0") (qualified-name "AllocationTest::l"))) (name "l") (declared-name "l") (declared (properties (composite true) (reference false) (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "AllocationTest::l::component"))) (name "component") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "AllocationTest::Logical")))))
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "AllocationTest::p"))) (name "p") (declared-name "p") (declared (properties (composite true) (reference false) (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "AllocationTest::p::assembly"))) (name "assembly") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "AllocationTest::Physical"))))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "AllocationTest::p::assembly::element"))) (name "element") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "AllocationTest::Physical")))))
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (allocate (status resolved) (from (node (document "d0") (qualified-name "AllocationTest::l"))) (to (node (document "d0") (qualified-name "AllocationTest::p"))))
    (allocate (status resolved) (from (node (document "d0") (qualified-name "AllocationTest::l::component"))) (to (node (document "d0") (qualified-name "AllocationTest::p::assembly::element"))))
    (allocate (status resolved) (from (node (document "d0") (qualified-name "AllocationTest::l::component"))) (to (node (document "d0") (qualified-name "AllocationTest::p::assembly::element"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "AllocationTest::l::component"))) (to (node (document "d0") (qualified-name "AllocationTest::Logical::component"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "AllocationTest::p::assembly"))) (to (node (document "d0") (qualified-name "AllocationTest::Physical::assembly"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "AllocationTest::Logical_to_Physical"))) (to (node (document "d0") (qualified-name "AllocationTest::A"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "AllocationTest::Logical_to_Physical::logical"))) (to (node (document "d0") (qualified-name "AllocationTest::Logical"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "AllocationTest::Logical_to_Physical::physical"))) (to (node (document "d0") (qualified-name "AllocationTest::Physical"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "AllocationTest::allocation1"))) (to (node (document "d0") (qualified-name "AllocationTest::Logical_to_Physical"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "AllocationTest::l"))) (to (node (document "d0") (qualified-name "AllocationTest::Logical"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "AllocationTest::p"))) (to (node (document "d0") (qualified-name "AllocationTest::Physical"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
