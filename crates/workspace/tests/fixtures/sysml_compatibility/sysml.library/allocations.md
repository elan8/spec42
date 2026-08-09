# META
~~~ini
description=Standard Library: Systems Library/Allocations
type=file
~~~
# SOURCE
~~~sysml
standard library package Allocations {
	doc
	/*
	 * This package defines the base types for allocations and related structural elements
	 * in the SysML language.
	 */

	private import Base::Anything;
	private import Connections::*;

	allocation def Allocation :> BinaryConnection {
		doc
		/*
		 * Allocation is the most general class of allocation, represented as a connection 
		 * between the source of the allocation and the target. Allocation is the base type 
		 * of all AllocationDefinitions.
		 */
	
		end source: Anything :>> BinaryConnection::source;
		end target: Anything :>> BinaryConnection::target;
	}
	
	abstract allocation allocations: Allocation[0..*] nonunique :> binaryConnections {
		doc
		/*
		 * allocations is the base feature of all AllocationUsages.
		 */
	}
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'BinaryConnection'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'BinaryConnection::source'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'BinaryConnection::target'
semantic.unresolved_name 'binaryConnections'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'BinaryConnection'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'BinaryConnection::source'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'BinaryConnection::target'
semantic.unresolved_name 'binaryConnections'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwAllocation,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwEnd,Ident,Colon,Ident,ColonGtGt,Ident,ColonColon,Ident,Semicolon,
KwEnd,Ident,Colon,Ident,ColonGtGt,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwAbstract,KwAllocation,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'Allocations'
    (documentation)
    (import_decl private 'Base::Anything')
    (import_decl private 'Connections::*')
    (allocation_def 'Allocation' :> 'BinaryConnection'
      (documentation)
      (interface_end end 'source' : 'Anything' :>> 'BinaryConnection::source')
      (interface_end end 'target' : 'Anything' :>> 'BinaryConnection::target'))
    (allocation_usage 'Allocation' :> 'binaryConnections' 'allocations' multiplicity
      (documentation))))
~~~
# FORMAT
~~~sysml
standard library package Allocations {
    doc /*
	 * This package defines the base types for allocations and related structural elements
	 * in the SysML language.
	 */

    private import Base::Anything;
    private import Connections::*;

    allocation def Allocation :> BinaryConnection {
        doc /*
		 * Allocation is the most general class of allocation, represented as a connection 
		 * between the source of the allocation and the target. Allocation is the base type 
		 * of all AllocationDefinitions.
		 */

        end source : Anything :>> BinaryConnection::source;
        end target : Anything :>> BinaryConnection::target;
    }

    abstract allocation allocations : Allocation :> binaryConnections [0..*] {
        doc /*
		 * allocations is the base feature of all AllocationUsages.
		 */
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'Allocations'
      (documentation)
      (membership_import private -> 'Base::Anything'[unresolved])
      (namespace_import private -> 'Connections'[unresolved])
      (allocation_def 'Allocation' :> 'BinaryConnection'[unresolved]
        (documentation)
        (port_usage end 'source' : 'Anything'[unresolved] :>> 'BinaryConnection::source'[unresolved])
        (port_usage end 'target' : 'Anything'[unresolved] :>> 'BinaryConnection::target'[unresolved]))
      (allocation_usage abstract 'allocations' : 'Allocations::Allocation'[allocation_def] :> 'binaryConnections'[unresolved]
        (multiplicity_range [0..*])
        (documentation)))))
~~~
