# META
~~~ini
description=KerML Mass Roll-up: Vehicles_2
type=file
~~~
# SOURCE
~~~kerml
package Vehicles_2 {
	private import ScalarValues::String;
	private import MassRollup_1::*;
	
	class CarPart specializes MassedThing {		
		feature serialNumber: String;
		feature m redefines mass;
		
		composite subparts: CarPart[0..*] redefines subcomponents;
	}
	
	feature vehicle: CarPart {	
		feature vin redefines serialNumber;
		
		composite engine: CarPart subsets subparts {
			//...
		}
		
		composite transmission: CarPart subsets subparts {
			//...
		}
	}
	
	// Example usage
	
	private import SI::*;
	feature v: vehicle {
		feature m redefines CarPart::m = 1000;
		composite engine redefines vehicle::engine {
			feature m redefines CarPart::m = 100;
		}
		composite transmission redefines vehicle::transmission {
			feature m redefines CarPart::m = 50;
		}
	}
	
	// v.totalMass evaluates to 1150.0
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "vehicles_2.md"
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
        (range (start 25 16) (end 25 18))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "c4b6b8a2932c8d57b2390e8d872817ed56b6214751812e0f65db10417b933645") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Vehicles_2"))) (kind "package") (name "Vehicles_2") (declared-name "Vehicles_2"))
    (element (id (node (document "d0") (qualified-name "Vehicles_2::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Vehicles_2"))) (authored (membership (kind Import) (visibility "private") (import (reference "MassRollup_1::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Vehicles_2::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Vehicles_2"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Vehicles_2::CarPart"))) (kind "classifier decl") (name "CarPart") (declared-name "CarPart") (parent (node (document "d0") (qualified-name "Vehicles_2"))))
    (element (id (node (document "d0") (qualified-name "Vehicles_2::String"))) (kind "import") (name "String") (declared-name "String") (parent (node (document "d0") (qualified-name "Vehicles_2"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::String") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Vehicles_2::v"))) (kind "feature decl") (name "v") (declared-name "v") (parent (node (document "d0") (qualified-name "Vehicles_2"))))
    (element (id (node (document "d0") (qualified-name "Vehicles_2::vehicle"))) (kind "feature decl") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "Vehicles_2"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Vehicles_2::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "MassRollup_1::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicles_2::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "SI::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicles_2::String"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::String") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
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
    (query (range (start 25 16) (end 25 18)) (probe (position 25 16))
      (reference
        (source (document "d0") (qualified-name "Vehicles_2::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "SI::*")
        (range (start 25 16) (end 25 18))
        (outcome (status unresolved))
      )
    )
    (query (range (start 2 16) (end 2 28)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "Vehicles_2::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "MassRollup_1::*")
        (range (start 2 16) (end 2 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 36)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "Vehicles_2::String"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::String")
        (range (start 1 16) (end 1 36))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
