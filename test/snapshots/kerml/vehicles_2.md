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
  (document "memory://snapshot/vehicles_2.md"
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
        (range (start 2 16) (end 2 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 4 27) (end 4 38))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 5 2) (end 6 2))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 6 2) (end 8 2))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 8 2) (end 9 1))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 12 24) (end 12 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 14 36) (end 14 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 18 42) (end 18 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 25 16) (end 25 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 27 22) (end 27 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 29 23) (end 29 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 32 23) (end 32 33))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:9e2726b35b0b6a9216d76e27aebe0d6870b6cf702c5ead1ab386fa401e1b6959") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicles_2.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::String") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/vehicles_2.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "MassRollup_1") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/vehicles_2.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "SI") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::CarPart"))) (kind class-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "MassedThing"))))
    (declaration (id (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::v"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "vehicle"))))
    (declaration (id (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::v::engine"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "vehicle::engine"))))
    (declaration (id (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::v::engine::m"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "CarPart::m"))))
    (declaration (id (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::v::m"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "CarPart::m"))))
    (declaration (id (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::v::transmission"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "vehicle::transmission"))))
    (declaration (id (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::v::transmission::m"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "CarPart::m"))))
    (declaration (id (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::vehicle"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CarPart"))))
    (declaration (id (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::vehicle::engine"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CarPart")) (subsetting (reference "subparts"))))
    (declaration (id (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::vehicle::transmission"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CarPart")) (subsetting (reference "subparts"))))
    (declaration (id (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::vehicle::vin"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "serialNumber"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/vehicles_2.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "MassRollup_1")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicles_2.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "SI")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicles_2.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::CarPart"))) (kind specialization) (ordinal 0))
      (authored-target "MassedThing")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::v"))) (kind featureTyping) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::vehicle")))))
    (reference (id (source (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::v::engine"))) (kind redefinition) (ordinal 0))
      (authored-target "vehicle::engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::vehicle::engine")))))
    (reference (id (source (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::v::engine::m"))) (kind redefinition) (ordinal 0))
      (authored-target "CarPart::m")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::v::m"))) (kind redefinition) (ordinal 0))
      (authored-target "CarPart::m")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::v::transmission"))) (kind redefinition) (ordinal 0))
      (authored-target "vehicle::transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::vehicle::transmission")))))
    (reference (id (source (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::v::transmission::m"))) (kind redefinition) (ordinal 0))
      (authored-target "CarPart::m")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "CarPart")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::CarPart")))))
    (reference (id (source (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::vehicle::engine"))) (kind featureTyping) (ordinal 0))
      (authored-target "CarPart")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::CarPart")))))
    (reference (id (source (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::vehicle::engine"))) (kind subsetting) (ordinal 0))
      (authored-target "subparts")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::vehicle::transmission"))) (kind featureTyping) (ordinal 0))
      (authored-target "CarPart")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::CarPart")))))
    (reference (id (source (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::vehicle::transmission"))) (kind subsetting) (ordinal 0))
      (authored-target "subparts")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::vehicle::vin"))) (kind redefinition) (ordinal 0))
      (authored-target "serialNumber")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::v"))) (target (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::v"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::v::engine"))) (target (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::vehicle::engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::v::engine"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::v::transmission"))) (target (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::vehicle::transmission"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::v::transmission"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::vehicle"))) (target (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::CarPart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::vehicle::engine"))) (target (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::CarPart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::vehicle::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::vehicle::transmission"))) (target (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::CarPart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::vehicle::transmission"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::v::engine::m"))) (value (kind integer) (integer 100)))
    (evaluated (declaration (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::v::m"))) (value (kind integer) (integer 1000)))
    (evaluated (declaration (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::v::transmission::m"))) (value (kind integer) (integer 50)))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/vehicles_2.md") (range (start 2 16) (end 2 31)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/vehicles_2.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "MassRollup_1")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicles_2.md") (range (start 25 16) (end 25 21)) (probe (position 25 16))
    (reference (id (source (node (document "memory://snapshot/vehicles_2.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "SI")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicles_2.md") (range (start 1 16) (end 1 36)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/vehicles_2.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::String")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicles_2.md") (range (start 4 27) (end 4 38)) (probe (position 4 27))
    (reference (id (source (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::CarPart"))) (kind specialization) (ordinal 0) (authored-target "MassedThing")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicles_2.md") (range (start 26 12) (end 26 19)) (probe (position 26 12))
    (reference (id (source (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::v"))) (kind featureTyping) (ordinal 0) (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::vehicle")))))
  )
  (query (document "memory://snapshot/vehicles_2.md") (range (start 28 29) (end 28 44)) (probe (position 28 29))
    (reference (id (source (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::v::engine"))) (kind redefinition) (ordinal 0) (authored-target "vehicle::engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::vehicle::engine")))))
  )
  (query (document "memory://snapshot/vehicles_2.md") (range (start 29 23) (end 29 33)) (probe (position 29 23))
    (reference (id (source (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::v::engine::m"))) (kind redefinition) (ordinal 0) (authored-target "CarPart::m")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicles_2.md") (range (start 27 22) (end 27 32)) (probe (position 27 22))
    (reference (id (source (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::v::m"))) (kind redefinition) (ordinal 0) (authored-target "CarPart::m")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicles_2.md") (range (start 31 35) (end 31 56)) (probe (position 31 35))
    (reference (id (source (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::v::transmission"))) (kind redefinition) (ordinal 0) (authored-target "vehicle::transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::vehicle::transmission")))))
  )
  (query (document "memory://snapshot/vehicles_2.md") (range (start 32 23) (end 32 33)) (probe (position 32 23))
    (reference (id (source (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::v::transmission::m"))) (kind redefinition) (ordinal 0) (authored-target "CarPart::m")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicles_2.md") (range (start 11 18) (end 11 25)) (probe (position 11 18))
    (reference (id (source (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "CarPart")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::CarPart")))))
  )
  (query (document "memory://snapshot/vehicles_2.md") (range (start 14 20) (end 14 27)) (probe (position 14 20))
    (reference (id (source (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::vehicle::engine"))) (kind featureTyping) (ordinal 0) (authored-target "CarPart")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::CarPart")))))
  )
  (query (document "memory://snapshot/vehicles_2.md") (range (start 14 36) (end 14 44)) (probe (position 14 36))
    (reference (id (source (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::vehicle::engine"))) (kind subsetting) (ordinal 0) (authored-target "subparts")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicles_2.md") (range (start 18 26) (end 18 33)) (probe (position 18 26))
    (reference (id (source (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::vehicle::transmission"))) (kind featureTyping) (ordinal 0) (authored-target "CarPart")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::CarPart")))))
  )
  (query (document "memory://snapshot/vehicles_2.md") (range (start 18 42) (end 18 50)) (probe (position 18 42))
    (reference (id (source (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::vehicle::transmission"))) (kind subsetting) (ordinal 0) (authored-target "subparts")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicles_2.md") (range (start 12 24) (end 12 36)) (probe (position 12 24))
    (reference (id (source (node (document "memory://snapshot/vehicles_2.md") (qualified-name "Vehicles_2::vehicle::vin"))) (kind redefinition) (ordinal 0) (authored-target "serialNumber")
      (outcome (status unresolved)))
  )
)
~~~
