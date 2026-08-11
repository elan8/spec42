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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "allocations.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 10 30) (end 10 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 18 2) (end 18 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 18 27) (end 18 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 19 2) (end 19 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 19 27) (end 19 51))
      )
    )
  )
)
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
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "7e2c6643f248a86608bc2dda70336c419ddbe039b5e88168a5cac75e0edaa579") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Allocations"))) (kind "package") (name "Allocations") (declared-name "Allocations"))
    (element (id (node (document "d0") (qualified-name "Allocations::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Allocations"))) (authored (membership (kind Import) (visibility "private") (import (reference "Connections::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Allocations::Allocation"))) (kind "allocation def") (name "Allocation") (declared-name "Allocation") (parent (node (document "d0") (qualified-name "Allocations"))) (authored (membership (kind Owning)) (relationships (specializes (reference "BinaryConnection")))))
    (element (id (node (document "d0") (qualified-name "Allocations::Allocation::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Allocations::Allocation"))))
    (element (id (node (document "d0") (qualified-name "Allocations::Allocation::source"))) (kind "interface end") (name "source") (declared-name "source") (parent (node (document "d0") (qualified-name "Allocations::Allocation"))) (authored (relationships (typing (reference "Anything")) (redefinition (reference "BinaryConnection::source")))))
    (element (id (node (document "d0") (qualified-name "Allocations::Allocation::target"))) (kind "interface end") (name "target") (declared-name "target") (parent (node (document "d0") (qualified-name "Allocations::Allocation"))) (authored (relationships (typing (reference "Anything")) (redefinition (reference "BinaryConnection::target")))))
    (element (id (node (document "d0") (qualified-name "Allocations::Anything"))) (kind "import") (name "Anything") (declared-name "Anything") (parent (node (document "d0") (qualified-name "Allocations"))) (authored (membership (kind Import) (visibility "private") (import (reference "Base::Anything") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Allocations::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Allocations"))))
    (element (id (node (document "d0") (qualified-name "Allocations::allocations"))) (kind "allocation") (name "allocations") (declared-name "allocations") (parent (node (document "d0") (qualified-name "Allocations"))) (authored (membership (kind Feature)) (relationships (typing (reference "Allocation")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Allocations::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Connections::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Allocations::Allocation"))) (kind specialization) (ordinal 0)) (authored-target "BinaryConnection") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Allocations::Allocation::source"))) (kind featureTyping) (ordinal 0)) (authored-target "Anything") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Allocations::Allocation::source"))) (kind redefinition) (ordinal 0)) (authored-target "BinaryConnection::source") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Allocations::Allocation::target"))) (kind featureTyping) (ordinal 0)) (authored-target "Anything") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Allocations::Allocation::target"))) (kind redefinition) (ordinal 0)) (authored-target "BinaryConnection::target") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Allocations::Anything"))) (kind membershipImport) (ordinal 0)) (authored-target "Base::Anything") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Allocations::allocations"))) (kind featureTyping) (ordinal 0)) (authored-target "Allocation") (outcome (status resolved) (target (node (document "d0") (qualified-name "Allocations::Allocation")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Allocations::allocations"))) (target (node (document "d0") (qualified-name "Allocations::Allocation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Allocations::allocations"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 8 16) (end 8 27)) (probe (position 8 16))
      (reference
        (source (document "d0") (qualified-name "Allocations::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Connections::*")
        (range (start 8 16) (end 8 27))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 16) (end 7 30)) (probe (position 7 16))
      (reference
        (source (document "d0") (qualified-name "Allocations::Anything"))
        (kind membershipImport) (ordinal 0) (authored-target "Base::Anything")
        (range (start 7 16) (end 7 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 30) (end 10 46)) (probe (position 10 30))
      (reference
        (source (document "d0") (qualified-name "Allocations::Allocation"))
        (kind specialization) (ordinal 0) (authored-target "BinaryConnection")
        (range (start 10 30) (end 10 46))
        (outcome (status unresolved))
      )
    )
    (query (range (start 18 27) (end 18 51)) (probe (position 18 27))
      (reference
        (source (document "d0") (qualified-name "Allocations::Allocation::source"))
        (kind redefinition) (ordinal 0) (authored-target "BinaryConnection::source")
        (range (start 18 27) (end 18 51))
        (outcome (status unresolved))
      )
    )
    (query (range (start 19 27) (end 19 51)) (probe (position 19 27))
      (reference
        (source (document "d0") (qualified-name "Allocations::Allocation::target"))
        (kind redefinition) (ordinal 0) (authored-target "BinaryConnection::target")
        (range (start 19 27) (end 19 51))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
