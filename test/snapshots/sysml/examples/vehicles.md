# META
~~~ini
description=SysML Example (Mass Roll-up): Vehicles
type=file
~~~
# SOURCE
~~~sysml
package VehicleMasses {
	private import ScalarValues::*;
	private import MassRollup::*;
	
	part def CarPart :> MassedThing {			
		attribute serialNumber: String;
	}
	
	part car: CarPart :> compositeThing {	
		attribute vin redefines serialNumber;
		
		part carParts: CarPart[*] redefines subcomponents;
		
		part engine :> simpleThing, carParts {
			//...
		}
		
		part transmission :> simpleThing, carParts {
			//...
		}
	}

	// Example usage
	private import SI::*;	
	part c :> car {
		redefines mass = 1000 [kg];
		part redefines engine {
			redefines mass = 100 [kg];
		}
		
		part redefines transmission {
			redefines mass = 50 [kg];
		}	
	}
	
	// c.totalMass --> 1150.0 [kg]
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "vehicles.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 4 21) (end 4 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 2) (end 5 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 26) (end 5 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 8 22) (end 8 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 9 26) (end 9 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 38) (end 11 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 13 17) (end 13 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 17 23) (end 17 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 23 16) (end 23 18))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 25 2) (end 25 32))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 27 3) (end 27 32))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 31 3) (end 31 31))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package VehicleMasses {
    private import ScalarValues::*;
    private import MassRollup::*;

    part def CarPart :> MassedThing {
        attribute serialNumber: String;
    }

    part car: CarPart :> compositeThing {
        attribute vin redefines serialNumber;

        part carParts: CarPart[*] redefines subcomponents;

        part engine :> simpleThing, carParts {
            //...
        }

        part transmission :> simpleThing, carParts {
            //...
        }
    }

    // Example usage
    private import SI::*;
    part c :> car {
        redefines mass = 1000 [kg];
        part redefines engine {
            redefines mass = 100 [kg];
        }

        part redefines transmission {
            redefines mass = 50 [kg];
        }
    }

    // c.totalMass --> 1150.0 [kg]
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "ba97ef78975a2484e4f6454572093c677dbda4d7d913eaff4a1fb8383de23264") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "VehicleMasses"))) (kind "package") (name "VehicleMasses") (declared-name "VehicleMasses"))
    (element (id (node (document "d0") (qualified-name "VehicleMasses::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "VehicleMasses"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "VehicleMasses::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "VehicleMasses"))) (authored (membership (kind Import) (visibility "private") (import (reference "MassRollup::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "VehicleMasses::*#import2"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "VehicleMasses"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "VehicleMasses::CarPart"))) (kind "part def") (name "CarPart") (declared-name "CarPart") (parent (node (document "d0") (qualified-name "VehicleMasses"))) (authored (membership (kind Owning)) (relationships (specializes (reference "MassedThing")))))
    (element (id (node (document "d0") (qualified-name "VehicleMasses::CarPart::serialNumber"))) (kind "attribute") (name "serialNumber") (declared-name "serialNumber") (parent (node (document "d0") (qualified-name "VehicleMasses::CarPart"))) (authored (membership (kind Feature)) (relationships (typing (reference "String")) (typing (reference "String")))))
    (element (id (node (document "d0") (qualified-name "VehicleMasses::c"))) (kind "part") (name "c") (declared-name "c") (parent (node (document "d0") (qualified-name "VehicleMasses"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "car")))))
    (element (id (node (document "d0") (qualified-name "VehicleMasses::c::engine"))) (kind "part") (name "engine") (parent (node (document "d0") (qualified-name "VehicleMasses::c"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "engine")))))
    (element (id (node (document "d0") (qualified-name "VehicleMasses::c::transmission"))) (kind "part") (name "transmission") (parent (node (document "d0") (qualified-name "VehicleMasses::c"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "transmission")))))
    (element (id (node (document "d0") (qualified-name "VehicleMasses::car"))) (kind "part") (name "car") (declared-name "car") (parent (node (document "d0") (qualified-name "VehicleMasses"))) (authored (membership (kind Feature)) (relationships (typing (reference "CarPart")) (subsetting (reference "compositeThing")))))
    (element (id (node (document "d0") (qualified-name "VehicleMasses::car::carParts"))) (kind "part") (name "carParts") (declared-name "carParts") (parent (node (document "d0") (qualified-name "VehicleMasses::car"))) (authored (membership (kind Feature)) (relationships (typing (reference "CarPart")) (redefinition (reference "subcomponents")))))
    (element (id (node (document "d0") (qualified-name "VehicleMasses::car::engine"))) (kind "part") (name "engine") (declared-name "engine") (parent (node (document "d0") (qualified-name "VehicleMasses::car"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "simpleThing")) (subsetting (reference "carParts")))))
    (element (id (node (document "d0") (qualified-name "VehicleMasses::car::transmission"))) (kind "part") (name "transmission") (declared-name "transmission") (parent (node (document "d0") (qualified-name "VehicleMasses::car"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "simpleThing")) (subsetting (reference "carParts")))))
    (element (id (node (document "d0") (qualified-name "VehicleMasses::car::vin"))) (kind "attribute") (name "vin") (declared-name "vin") (parent (node (document "d0") (qualified-name "VehicleMasses::car"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "serialNumber")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "VehicleMasses::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleMasses::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "MassRollup::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleMasses::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "SI::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleMasses::CarPart"))) (kind specialization) (ordinal 0)) (authored-target "MassedThing") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleMasses::CarPart::serialNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleMasses::CarPart::serialNumber"))) (kind featureTyping) (ordinal 1)) (authored-target "String") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleMasses::c"))) (kind subsetting) (ordinal 0)) (authored-target "car") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleMasses::car")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleMasses::c::engine"))) (kind redefinition) (ordinal 0)) (authored-target "engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleMasses::c::engine")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleMasses::c::transmission"))) (kind redefinition) (ordinal 0)) (authored-target "transmission") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleMasses::c::transmission")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleMasses::car"))) (kind featureTyping) (ordinal 0)) (authored-target "CarPart") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleMasses::CarPart")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleMasses::car"))) (kind subsetting) (ordinal 0)) (authored-target "compositeThing") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleMasses::car::carParts"))) (kind featureTyping) (ordinal 0)) (authored-target "CarPart") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleMasses::CarPart")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleMasses::car::carParts"))) (kind redefinition) (ordinal 0)) (authored-target "subcomponents") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleMasses::car::engine"))) (kind subsetting) (ordinal 0)) (authored-target "simpleThing") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleMasses::car::engine"))) (kind subsetting) (ordinal 1)) (authored-target "carParts") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleMasses::car::carParts")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleMasses::car::transmission"))) (kind subsetting) (ordinal 0)) (authored-target "simpleThing") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleMasses::car::transmission"))) (kind subsetting) (ordinal 1)) (authored-target "carParts") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleMasses::car::carParts")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleMasses::car::vin"))) (kind redefinition) (ordinal 0)) (authored-target "serialNumber") (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "VehicleMasses::c"))) (target (node (document "d0") (qualified-name "VehicleMasses::car"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleMasses::c"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "VehicleMasses::c::engine"))) (target (node (document "d0") (qualified-name "VehicleMasses::c::engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleMasses::c::engine"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "VehicleMasses::c::transmission"))) (target (node (document "d0") (qualified-name "VehicleMasses::c::transmission"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleMasses::c::transmission"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VehicleMasses::car"))) (target (node (document "d0") (qualified-name "VehicleMasses::CarPart"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleMasses::car"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VehicleMasses::car::carParts"))) (target (node (document "d0") (qualified-name "VehicleMasses::CarPart"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleMasses::car::carParts"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "VehicleMasses::car::engine"))) (target (node (document "d0") (qualified-name "VehicleMasses::car::carParts"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleMasses::car::engine"))) (kind subsetting) (ordinal 1)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "VehicleMasses::car::transmission"))) (target (node (document "d0") (qualified-name "VehicleMasses::car::carParts"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleMasses::car::transmission"))) (kind subsetting) (ordinal 1)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 23 16) (end 23 18)) (probe (position 23 16))
      (reference
        (source (document "d0") (qualified-name "VehicleMasses::*#import2"))
        (kind namespaceImport) (ordinal 0) (authored-target "SI::*")
        (range (start 23 16) (end 23 18))
        (outcome (status unresolved))
      )
    )
    (query (range (start 24 11) (end 24 14)) (probe (position 24 11))
      (reference
        (source (document "d0") (qualified-name "VehicleMasses::c"))
        (kind subsetting) (ordinal 0) (authored-target "car")
        (range (start 24 11) (end 24 14))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleMasses::car") (range (start 8 1) (end 8 258)))
        )
      )
    )
    (query (range (start 5 26) (end 5 32)) (probe (position 5 26))
      (reference
        (source (document "d0") (qualified-name "VehicleMasses::CarPart::serialNumber"))
        (kind featureTyping) (ordinal 1) (authored-target "String")
        (range (start 5 26) (end 5 32))
        (outcome (status unresolved))
      )
    )
    (query (range (start 26 17) (end 26 23)) (probe (position 26 17))
      (reference
        (source (document "d0") (qualified-name "VehicleMasses::c::engine"))
        (kind redefinition) (ordinal 0) (authored-target "engine")
        (range (start 26 17) (end 26 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleMasses::c::engine") (range (start 26 2) (end 26 59)))
        )
      )
    )
    (query (range (start 8 11) (end 8 18)) (probe (position 8 11))
      (reference
        (source (document "d0") (qualified-name "VehicleMasses::car"))
        (kind featureTyping) (ordinal 0) (authored-target "CarPart")
        (range (start 8 11) (end 8 18))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleMasses::CarPart") (range (start 4 1) (end 4 74)))
        )
      )
    )
    (query (range (start 11 17) (end 11 24)) (probe (position 11 17))
      (reference
        (source (document "d0") (qualified-name "VehicleMasses::car::carParts"))
        (kind featureTyping) (ordinal 0) (authored-target "CarPart")
        (range (start 11 17) (end 11 24))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleMasses::CarPart") (range (start 4 1) (end 4 74)))
        )
      )
    )
    (query (range (start 13 30) (end 13 38)) (probe (position 13 30))
      (reference
        (source (document "d0") (qualified-name "VehicleMasses::car::engine"))
        (kind subsetting) (ordinal 1) (authored-target "carParts")
        (range (start 13 30) (end 13 38))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleMasses::car::carParts") (range (start 11 2) (end 11 52)))
        )
      )
    )
    (query (range (start 17 36) (end 17 44)) (probe (position 17 36))
      (reference
        (source (document "d0") (qualified-name "VehicleMasses::car::transmission"))
        (kind subsetting) (ordinal 1) (authored-target "carParts")
        (range (start 17 36) (end 17 44))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleMasses::car::carParts") (range (start 11 2) (end 11 52)))
        )
      )
    )
    (query (range (start 2 16) (end 2 26)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "VehicleMasses::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "MassRollup::*")
        (range (start 2 16) (end 2 26))
        (outcome (status unresolved))
      )
    )
    (query (range (start 4 21) (end 4 32)) (probe (position 4 21))
      (reference
        (source (document "d0") (qualified-name "VehicleMasses::CarPart"))
        (kind specialization) (ordinal 0) (authored-target "MassedThing")
        (range (start 4 21) (end 4 32))
        (outcome (status unresolved))
      )
    )
    (query (range (start 13 17) (end 13 28)) (probe (position 13 17))
      (reference
        (source (document "d0") (qualified-name "VehicleMasses::car::engine"))
        (kind subsetting) (ordinal 0) (authored-target "simpleThing")
        (range (start 13 17) (end 13 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 17 23) (end 17 34)) (probe (position 17 23))
      (reference
        (source (document "d0") (qualified-name "VehicleMasses::car::transmission"))
        (kind subsetting) (ordinal 0) (authored-target "simpleThing")
        (range (start 17 23) (end 17 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 28)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "VehicleMasses::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 1 16) (end 1 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 26) (end 9 38)) (probe (position 9 26))
      (reference
        (source (document "d0") (qualified-name "VehicleMasses::car::vin"))
        (kind redefinition) (ordinal 0) (authored-target "serialNumber")
        (range (start 9 26) (end 9 38))
        (outcome (status unresolved))
      )
    )
    (query (range (start 30 17) (end 30 29)) (probe (position 30 17))
      (reference
        (source (document "d0") (qualified-name "VehicleMasses::c::transmission"))
        (kind redefinition) (ordinal 0) (authored-target "transmission")
        (range (start 30 17) (end 30 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleMasses::c::transmission") (range (start 30 2) (end 30 64)))
        )
      )
    )
    (query (range (start 11 38) (end 11 51)) (probe (position 11 38))
      (reference
        (source (document "d0") (qualified-name "VehicleMasses::car::carParts"))
        (kind redefinition) (ordinal 0) (authored-target "subcomponents")
        (range (start 11 38) (end 11 51))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 22) (end 8 36)) (probe (position 8 22))
      (reference
        (source (document "d0") (qualified-name "VehicleMasses::car"))
        (kind subsetting) (ordinal 0) (authored-target "compositeThing")
        (range (start 8 22) (end 8 36))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
