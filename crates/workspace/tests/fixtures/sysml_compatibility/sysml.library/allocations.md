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
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Allocations"))) (name "Allocations") (declared-name "Allocations")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Allocations::*"))) (name "*") (declared-name "*"))
        (element (kind "allocation def") (id (node (document "d0") (qualified-name "Allocations::Allocation"))) (name "Allocation") (declared-name "Allocation")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Allocations::Allocation::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Allocations::Allocation")))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "Allocations::Allocation::source"))) (name "source") (declared-name "source") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "Allocations::Allocation")))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "Allocations::Allocation::target"))) (name "target") (declared-name "target") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "Allocations::Allocation")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Allocations::Anything"))) (name "Anything") (declared-name "Anything"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "Allocations::_documentation"))) (name ""))
        (element (kind "allocation") (id (node (document "d0") (qualified-name "Allocations::allocations"))) (name "allocations") (declared-name "allocations"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Allocations::Allocation::_documentation"))) (to (node (document "d0") (qualified-name "Allocations::Allocation"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Allocations::_documentation"))) (to (node (document "d0") (qualified-name "Allocations"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Allocations::allocations"))) (to (node (document "d0") (qualified-name "Allocations::Allocation"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml.library/allocations.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 1) (end 7 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 1) (end 8 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 10 1) (end 10 384))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 18 2) (end 18 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 18 2) (end 18 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 19 2) (end 19 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 19 2) (end 19 52))
      )
    )
  )
)
~~~
