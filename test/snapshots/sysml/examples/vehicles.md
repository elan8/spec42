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
  (document "memory://snapshot/vehicles.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 29))
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
        (range (start 23 16) (end 23 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 25 12) (end 25 16))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 26 17) (end 26 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 27 13) (end 27 17))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 30 17) (end 30 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 31 13) (end 31 17))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:646944c143f5e6564093651f47ba3ef3da45addc59d556f7e601081c71594bf9") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicles.md") (path (named (kind package) (name "VehicleMasses")) (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/vehicles.md") (path (named (kind package) (name "VehicleMasses")) (anonymous (kind import) (ordinal 1)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "MassRollup") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/vehicles.md") (path (named (kind package) (name "VehicleMasses")) (anonymous (kind import) (ordinal 2)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "SI") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::CarPart"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "MassedThing"))))
    (declaration (id (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::CarPart::serialNumber"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String"))))
    (declaration (id (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::c"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "car"))))
    (declaration (id (node (document "memory://snapshot/vehicles.md") (path (named (kind package) (name "VehicleMasses")) (named (kind part) (name "c")) (anonymous (kind attribute) (ordinal 0)))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "mass"))))
    (declaration (id (node (document "memory://snapshot/vehicles.md") (path (named (kind package) (name "VehicleMasses")) (named (kind part) (name "c")) (anonymous (kind part) (ordinal 0)))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "engine"))))
    (declaration (id (node (document "memory://snapshot/vehicles.md") (path (named (kind package) (name "VehicleMasses")) (named (kind part) (name "c")) (anonymous (kind part) (ordinal 1)))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "transmission"))))
    (declaration (id (node (document "memory://snapshot/vehicles.md") (path (named (kind package) (name "VehicleMasses")) (named (kind part) (name "c")) (anonymous (kind part) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "mass"))))
    (declaration (id (node (document "memory://snapshot/vehicles.md") (path (named (kind package) (name "VehicleMasses")) (named (kind part) (name "c")) (anonymous (kind part) (ordinal 1)) (anonymous (kind attribute) (ordinal 0)))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "mass"))))
    (declaration (id (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::car"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CarPart")) (subsetting (reference "compositeThing"))))
    (declaration (id (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::car::carParts"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower unbounded) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CarPart")) (redefinition (reference "subcomponents"))))
    (declaration (id (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::car::engine"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "simpleThing")) (subsetting (reference "carParts"))))
    (declaration (id (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::car::transmission"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "simpleThing")) (subsetting (reference "carParts"))))
    (declaration (id (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::car::vin"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "serialNumber"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/vehicles.md") (path (named (kind package) (name "VehicleMasses")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicles.md") (path (named (kind package) (name "VehicleMasses")) (anonymous (kind import) (ordinal 1)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "MassRollup")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicles.md") (path (named (kind package) (name "VehicleMasses")) (anonymous (kind import) (ordinal 2)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "SI")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::CarPart"))) (kind specialization) (ordinal 0))
      (authored-target "MassedThing")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::CarPart::serialNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::c"))) (kind subsetting) (ordinal 0))
      (authored-target "car")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::car")))))
    (reference (id (source (node (document "memory://snapshot/vehicles.md") (path (named (kind package) (name "VehicleMasses")) (named (kind part) (name "c")) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicles.md") (path (named (kind package) (name "VehicleMasses")) (named (kind part) (name "c")) (anonymous (kind part) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "engine")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicles.md") (path (named (kind package) (name "VehicleMasses")) (named (kind part) (name "c")) (anonymous (kind part) (ordinal 1)))))) (kind redefinition) (ordinal 0))
      (authored-target "transmission")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicles.md") (path (named (kind package) (name "VehicleMasses")) (named (kind part) (name "c")) (anonymous (kind part) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicles.md") (path (named (kind package) (name "VehicleMasses")) (named (kind part) (name "c")) (anonymous (kind part) (ordinal 1)) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::car"))) (kind featureTyping) (ordinal 0))
      (authored-target "CarPart")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::CarPart")))))
    (reference (id (source (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::car"))) (kind subsetting) (ordinal 0))
      (authored-target "compositeThing")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::car::carParts"))) (kind featureTyping) (ordinal 0))
      (authored-target "CarPart")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::CarPart")))))
    (reference (id (source (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::car::carParts"))) (kind redefinition) (ordinal 0))
      (authored-target "subcomponents")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::car::engine"))) (kind subsetting) (ordinal 0))
      (authored-target "simpleThing")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::car::engine"))) (kind subsetting) (ordinal 1))
      (authored-target "carParts")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::car::carParts")))))
    (reference (id (source (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::car::transmission"))) (kind subsetting) (ordinal 0))
      (authored-target "simpleThing")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::car::transmission"))) (kind subsetting) (ordinal 1))
      (authored-target "carParts")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::car::carParts")))))
    (reference (id (source (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::car::vin"))) (kind redefinition) (ordinal 0))
      (authored-target "serialNumber")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::CarPart::serialNumber")))))
  )
  (relationships
    (relationship (kind subsetting) (source (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::c"))) (target (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::car"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::c"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::car"))) (target (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::CarPart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::car"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::car::carParts"))) (target (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::CarPart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::car::carParts"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::car::engine"))) (target (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::car::carParts"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::car::engine"))) (kind subsetting) (ordinal 1)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::car::transmission"))) (target (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::car::carParts"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::car::transmission"))) (kind subsetting) (ordinal 1)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::car::vin"))) (target (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::CarPart::serialNumber"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::car::vin"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/vehicles.md") (path (named (kind package) (name "VehicleMasses")) (named (kind part) (name "c")) (anonymous (kind attribute) (ordinal 0)))))) (value (kind quantity) (magnitude (value (kind integer) (integer 1000))) (unit "kg")))
    (evaluated (declaration (node (document "memory://snapshot/vehicles.md") (path (named (kind package) (name "VehicleMasses")) (named (kind part) (name "c")) (anonymous (kind part) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)))))) (value (kind quantity) (magnitude (value (kind integer) (integer 100))) (unit "kg")))
    (evaluated (declaration (node (document "memory://snapshot/vehicles.md") (path (named (kind package) (name "VehicleMasses")) (named (kind part) (name "c")) (anonymous (kind part) (ordinal 1)) (anonymous (kind attribute) (ordinal 0)))))) (value (kind quantity) (magnitude (value (kind integer) (integer 50))) (unit "kg")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/vehicles.md") (range (start 1 16) (end 1 31)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/vehicles.md") (path (named (kind package) (name "VehicleMasses")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicles.md") (range (start 2 16) (end 2 29)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/vehicles.md") (path (named (kind package) (name "VehicleMasses")) (anonymous (kind import) (ordinal 1)))))) (kind namespaceImport) (ordinal 0) (authored-target "MassRollup")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicles.md") (range (start 23 16) (end 23 21)) (probe (position 23 16))
    (reference (id (source (node (document "memory://snapshot/vehicles.md") (path (named (kind package) (name "VehicleMasses")) (anonymous (kind import) (ordinal 2)))))) (kind namespaceImport) (ordinal 0) (authored-target "SI")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicles.md") (range (start 4 21) (end 4 32)) (probe (position 4 21))
    (reference (id (source (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::CarPart"))) (kind specialization) (ordinal 0) (authored-target "MassedThing")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicles.md") (range (start 5 26) (end 5 32)) (probe (position 5 26))
    (reference (id (source (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::CarPart::serialNumber"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicles.md") (range (start 24 11) (end 24 14)) (probe (position 24 11))
    (reference (id (source (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::c"))) (kind subsetting) (ordinal 0) (authored-target "car")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::car")))))
  )
  (query (document "memory://snapshot/vehicles.md") (range (start 25 12) (end 25 16)) (probe (position 25 12))
    (reference (id (source (node (document "memory://snapshot/vehicles.md") (path (named (kind package) (name "VehicleMasses")) (named (kind part) (name "c")) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "mass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicles.md") (range (start 26 17) (end 26 23)) (probe (position 26 17))
    (reference (id (source (node (document "memory://snapshot/vehicles.md") (path (named (kind package) (name "VehicleMasses")) (named (kind part) (name "c")) (anonymous (kind part) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "engine")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicles.md") (range (start 30 17) (end 30 29)) (probe (position 30 17))
    (reference (id (source (node (document "memory://snapshot/vehicles.md") (path (named (kind package) (name "VehicleMasses")) (named (kind part) (name "c")) (anonymous (kind part) (ordinal 1)))))) (kind redefinition) (ordinal 0) (authored-target "transmission")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicles.md") (range (start 27 13) (end 27 17)) (probe (position 27 13))
    (reference (id (source (node (document "memory://snapshot/vehicles.md") (path (named (kind package) (name "VehicleMasses")) (named (kind part) (name "c")) (anonymous (kind part) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "mass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicles.md") (range (start 31 13) (end 31 17)) (probe (position 31 13))
    (reference (id (source (node (document "memory://snapshot/vehicles.md") (path (named (kind package) (name "VehicleMasses")) (named (kind part) (name "c")) (anonymous (kind part) (ordinal 1)) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "mass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicles.md") (range (start 8 11) (end 8 18)) (probe (position 8 11))
    (reference (id (source (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::car"))) (kind featureTyping) (ordinal 0) (authored-target "CarPart")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::CarPart")))))
  )
  (query (document "memory://snapshot/vehicles.md") (range (start 8 22) (end 8 36)) (probe (position 8 22))
    (reference (id (source (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::car"))) (kind subsetting) (ordinal 0) (authored-target "compositeThing")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicles.md") (range (start 11 17) (end 11 24)) (probe (position 11 17))
    (reference (id (source (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::car::carParts"))) (kind featureTyping) (ordinal 0) (authored-target "CarPart")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::CarPart")))))
  )
  (query (document "memory://snapshot/vehicles.md") (range (start 11 38) (end 11 51)) (probe (position 11 38))
    (reference (id (source (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::car::carParts"))) (kind redefinition) (ordinal 0) (authored-target "subcomponents")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicles.md") (range (start 13 17) (end 13 28)) (probe (position 13 17))
    (reference (id (source (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::car::engine"))) (kind subsetting) (ordinal 0) (authored-target "simpleThing")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicles.md") (range (start 13 30) (end 13 38)) (probe (position 13 30))
    (reference (id (source (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::car::engine"))) (kind subsetting) (ordinal 1) (authored-target "carParts")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::car::carParts")))))
  )
  (query (document "memory://snapshot/vehicles.md") (range (start 17 23) (end 17 34)) (probe (position 17 23))
    (reference (id (source (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::car::transmission"))) (kind subsetting) (ordinal 0) (authored-target "simpleThing")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicles.md") (range (start 17 36) (end 17 44)) (probe (position 17 36))
    (reference (id (source (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::car::transmission"))) (kind subsetting) (ordinal 1) (authored-target "carParts")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::car::carParts")))))
  )
  (query (document "memory://snapshot/vehicles.md") (range (start 9 26) (end 9 38)) (probe (position 9 26))
    (reference (id (source (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::car::vin"))) (kind redefinition) (ordinal 0) (authored-target "serialNumber")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles.md") (qualified-name "VehicleMasses::CarPart::serialNumber")))))
  )
)
~~~
