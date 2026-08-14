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
  (document "memory://snapshot/allocations.md"
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
        (range (start 8 16) (end 8 30))
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
        (range (start 18 14) (end 18 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 19 14) (end 19 22))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 22 1) (end 27 2))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:5d55bec1f5011521f0f18e85b64d4ea78e1407b2892472bca2181ab513f91c8d") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/allocations.md") (qualified-name "Allocations"))) (kind library-package) (membership (kind owning) (visibility default)) (facts (modifiers standard)) (documentation (doc (text "\n\t * This package defines the base types for allocations and related structural elements\n\t * in the SysML language.\n\t "))))
    (declaration (id (node (document "memory://snapshot/allocations.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Base::Anything") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/allocations.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Connections") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/allocations.md") (qualified-name "Allocations::Allocation"))) (kind allocation-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * Allocation is the most general class of allocation, represented as a connection \n\t\t * between the source of the allocation and the target. Allocation is the base type \n\t\t * of all AllocationDefinitions.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "BinaryConnection"))))
    (declaration (id (node (document "memory://snapshot/allocations.md") (qualified-name "Allocations::Allocation::source"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Anything"))))
    (declaration (id (node (document "memory://snapshot/allocations.md") (qualified-name "Allocations::Allocation::target"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Anything"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/allocations.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Connections")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/allocations.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Base::Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/allocations.md") (qualified-name "Allocations::Allocation"))) (kind specialization) (ordinal 0))
      (authored-target "BinaryConnection")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/allocations.md") (qualified-name "Allocations::Allocation::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/allocations.md") (qualified-name "Allocations::Allocation::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "Anything")
      (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/allocations.md") (range (start 8 16) (end 8 30)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/allocations.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "Connections")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/allocations.md") (range (start 7 16) (end 7 30)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/allocations.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Base::Anything")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/allocations.md") (range (start 10 30) (end 10 46)) (probe (position 10 30))
    (reference (id (source (node (document "memory://snapshot/allocations.md") (qualified-name "Allocations::Allocation"))) (kind specialization) (ordinal 0) (authored-target "BinaryConnection")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/allocations.md") (range (start 18 14) (end 18 22)) (probe (position 18 14))
    (reference (id (source (node (document "memory://snapshot/allocations.md") (qualified-name "Allocations::Allocation::source"))) (kind featureTyping) (ordinal 0) (authored-target "Anything")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/allocations.md") (range (start 19 14) (end 19 22)) (probe (position 19 14))
    (reference (id (source (node (document "memory://snapshot/allocations.md") (qualified-name "Allocations::Allocation::target"))) (kind featureTyping) (ordinal 0) (authored-target "Anything")
      (outcome (status unresolved)))
  )
)
~~~
