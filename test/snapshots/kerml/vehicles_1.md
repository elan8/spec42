# META
~~~ini
description=KerML Mass Roll-up: Vehicles_1
type=file
~~~
# SOURCE
~~~kerml
package Vehicles_1 {
	private import ScalarValues::String;
	private import MassRollup_1::*;

	class Vehicle specializes MassedThing {
		feature vin: String;
		feature m redefines mass;
	
		composite engine: Engine subsets subcomponents;
		composite transmission: Transmission subsets subcomponents;
	}
	
	class Engine specializes MassedThing {
		feature serialNumber: String;
		feature m redefines mass;
		
		// ...
	}
	
	class Transmission specializes MassedThing {
		feature serialNumber: String;
		feature m redefines mass;
		
		// ...
	}
	
	// Example usage
	
	private import SI::*;
	feature v: Vehicle {
		feature m redefines Vehicle::m = 1000;
		composite engine redefines Vehicle::engine {
			feature m redefines Engine::m = 100;
		}
		composite transmission redefines Vehicle::transmission {
			feature m redefines Transmission::m = 50;
		}
	}

	// v.totalMass evaluates to 1150.0
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "vehicles_1.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 28 16) (end 28 18))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "2d6cd38c606521b11ebe60d31365cc337262c51ed2b57e6f68b31b661bd40284") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Vehicles_1"))) (kind "package") (name "Vehicles_1") (declared-name "Vehicles_1"))
    (element (id (node (document "d0") (qualified-name "Vehicles_1::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Vehicles_1"))) (authored (membership (kind Import) (visibility "private") (import (reference "MassRollup_1::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Vehicles_1::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Vehicles_1"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Vehicles_1::Engine"))) (kind "classifier decl") (name "Engine") (declared-name "Engine") (parent (node (document "d0") (qualified-name "Vehicles_1"))))
    (element (id (node (document "d0") (qualified-name "Vehicles_1::String"))) (kind "import") (name "String") (declared-name "String") (parent (node (document "d0") (qualified-name "Vehicles_1"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::String") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Vehicles_1::Transmission"))) (kind "classifier decl") (name "Transmission") (declared-name "Transmission") (parent (node (document "d0") (qualified-name "Vehicles_1"))))
    (element (id (node (document "d0") (qualified-name "Vehicles_1::Vehicle"))) (kind "classifier decl") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "Vehicles_1"))))
    (element (id (node (document "d0") (qualified-name "Vehicles_1::v"))) (kind "feature decl") (name "v") (declared-name "v") (parent (node (document "d0") (qualified-name "Vehicles_1"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Vehicles_1::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "MassRollup_1::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicles_1::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "SI::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicles_1::String"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::String") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
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
  (document "d0"
    (query (range (start 28 16) (end 28 18)) (probe (position 28 16))
      (reference
        (source (document "d0") (qualified-name "Vehicles_1::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "SI::*")
        (range (start 28 16) (end 28 18))
        (outcome (status unresolved))
      )
    )
    (query (range (start 2 16) (end 2 28)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "Vehicles_1::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "MassRollup_1::*")
        (range (start 2 16) (end 2 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 36)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "Vehicles_1::String"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::String")
        (range (start 1 16) (end 1 36))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
