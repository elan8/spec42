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
(model
  (namespace
    (package 'AllocationTest'
      (part_def 'Logical'
        (part_usage composite 'component'))
      (part_def 'Physical'
        (part_usage composite 'assembly'
          (part_usage composite 'element')))
      (part_usage 'l' : 'AllocationTest::Logical'[part_def]
        (part_usage composite :>> 'AllocationTest::Logical::component'[part_usage]))
      (part_usage 'p' : 'AllocationTest::Physical'[part_def]
        (part_usage composite :>> 'AllocationTest::Physical::assembly'[part_usage]
          (part_usage composite :>> 'AllocationTest::Physical::assembly::element'[part_usage]))
        (allocation_usage composite
          (connector_end 'l.component')
          (connector_end 'assembly.element')))
      (allocation_def 'A')
      (allocation_def 'Logical_to_Physical' :> 'AllocationTest::A'[allocation_def]
        (port_usage end 'logical' : 'AllocationTest::Logical'[part_def])
        (port_usage end 'physical' : 'AllocationTest::Physical'[part_def]))
      (allocation_usage 'allocation1' : 'AllocationTest::Logical_to_Physical'[allocation_def]
        (connector_end 'l')
        (connector_end 'p'))
      (allocation_usage 'allocation2' : 'AllocationTest::Logical_to_Physical'[allocation_def])
      (allocation_usage
        (connector_end 'l.component')
        (connector_end 'p.assembly.element')))))
~~~
