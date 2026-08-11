# META
~~~ini
description=SysML Training 11 (Interfaces): Interface Example
type=file
~~~
# SOURCE
~~~sysml
package 'Interface Example' {
	private import 'Port Example'::*;
	
	part def Vehicle;
	
	interface def FuelInterface {
		end supplierPort : FuelOutPort;
		end consumerPort : FuelInPort;
	}
	
	part vehicle : Vehicle {	
		part tankAssy : FuelTankAssembly;		
		part eng : Engine;
		
		interface : FuelInterface connect 
			supplierPort ::> tankAssy.fuelTankPort to 
			consumerPort ::> eng.engineFuelPort;
	} 
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "11_interface_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 2) (end 6 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 2) (end 7 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 18) (end 11 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 13) (end 12 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 15 20) (end 15 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 16 20) (end 16 38))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "6df93f3caecb225ed597f54b87a3fdea3e2e2d428817c6f9c4a2d8cd016c4c85") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Interface Example"))) (kind "package") (name "Interface Example") (declared-name "Interface Example"))
    (element (id (node (document "d0") (qualified-name "Interface Example::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Interface Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "Port Example::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Interface Example::FuelInterface"))) (kind "interface def") (name "FuelInterface") (declared-name "FuelInterface") (parent (node (document "d0") (qualified-name "Interface Example"))))
    (element (id (node (document "d0") (qualified-name "Interface Example::FuelInterface::consumerPort"))) (kind "interface end") (name "consumerPort") (declared-name "consumerPort") (parent (node (document "d0") (qualified-name "Interface Example::FuelInterface"))) (authored (relationships (typing (reference "FuelInPort")))))
    (element (id (node (document "d0") (qualified-name "Interface Example::FuelInterface::supplierPort"))) (kind "interface end") (name "supplierPort") (declared-name "supplierPort") (parent (node (document "d0") (qualified-name "Interface Example::FuelInterface"))) (authored (relationships (typing (reference "FuelOutPort")))))
    (element (id (node (document "d0") (qualified-name "Interface Example::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "Interface Example"))))
    (element (id (node (document "d0") (qualified-name "Interface Example::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "Interface Example"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "Interface Example::vehicle::eng"))) (kind "part") (name "eng") (declared-name "eng") (parent (node (document "d0") (qualified-name "Interface Example::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "Interface Example::vehicle::tankAssy"))) (kind "part") (name "tankAssy") (declared-name "tankAssy") (parent (node (document "d0") (qualified-name "Interface Example::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "FuelTankAssembly")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Interface Example::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Port Example::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interface Example::FuelInterface::consumerPort"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelInPort") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interface Example::FuelInterface::supplierPort"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelOutPort") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interface Example::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "Interface Example::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Interface Example::vehicle"))) (kind connectionSource) (ordinal 0)) (authored-target "tankAssy::fuelTankPort") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interface Example::vehicle"))) (kind connectionTarget) (ordinal 0)) (authored-target "eng::engineFuelPort") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interface Example::vehicle::eng"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interface Example::vehicle::tankAssy"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelTankAssembly") (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Interface Example::vehicle"))) (target (node (document "d0") (qualified-name "Interface Example::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Interface Example::vehicle"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 12 13) (end 12 19)) (probe (position 12 13))
      (reference
        (source (document "d0") (qualified-name "Interface Example::vehicle::eng"))
        (kind featureTyping) (ordinal 0) (authored-target "Engine")
        (range (start 12 13) (end 12 19))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 16) (end 10 23)) (probe (position 10 16))
      (reference
        (source (document "d0") (qualified-name "Interface Example::vehicle"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
        (range (start 10 16) (end 10 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Interface Example::Vehicle") (range (start 3 1) (end 3 18)))
        )
      )
    )
    (query (range (start 1 16) (end 1 30)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "Interface Example::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Port Example::*")
        (range (start 1 16) (end 1 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 11 18) (end 11 34)) (probe (position 11 18))
      (reference
        (source (document "d0") (qualified-name "Interface Example::vehicle::tankAssy"))
        (kind featureTyping) (ordinal 0) (authored-target "FuelTankAssembly")
        (range (start 11 18) (end 11 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 16 20) (end 16 38)) (probe (position 16 20))
      (reference
        (source (document "d0") (qualified-name "Interface Example::vehicle"))
        (kind connectionTarget) (ordinal 0) (authored-target "eng::engineFuelPort")
        (range (start 16 20) (end 16 38))
        (outcome (status unresolved))
      )
    )
    (query (range (start 15 20) (end 15 41)) (probe (position 15 20))
      (reference
        (source (document "d0") (qualified-name "Interface Example::vehicle"))
        (kind connectionSource) (ordinal 0) (authored-target "tankAssy::fuelTankPort")
        (range (start 15 20) (end 15 41))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
