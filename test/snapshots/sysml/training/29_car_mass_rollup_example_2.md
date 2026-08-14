# META
~~~ini
description=SysML Training 29 (Expressions): Car Mass Rollup Example 2
type=file
~~~
# SOURCE
~~~sysml
package 'Car Mass Rollup 1' {
	private import ScalarValues::*;
	private import MassRollup2::*;
	
	part def CarPart :> MassedThing {			
		attribute serialNumber: String;
	}
	
	part car: CarPart :> compositeThing {	
		attribute vin :>> serialNumber;
		
		part carParts: CarPart[*] :>> subcomponents;
		
		part engine :> carParts {
			//...
		}
		
		part transmission :> carParts {
			//...
		}
	}

	// Example usage
	
	private import SI::kg;
	part c :> car {
		attribute :>> simpleMass = 1000[kg];
		part :>> engine {
			attribute :>> simpleMass = 100[kg];
		}
		
		part redefines transmission {
			attribute :>> simpleMass = 50[kg];
		}	
	}
	
	// c::totalMass --> 1150.0[kg]
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/29_car_mass_rollup_example_2.md"
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
        (range (start 2 16) (end 2 30))
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
        (range (start 11 32) (end 11 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 24 16) (end 24 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 26 16) (end 26 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 27 11) (end 27 17))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 28 17) (end 28 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 31 17) (end 31 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 32 17) (end 32 27))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:7c88b7f6cbd5b6fe0c9363937f989945d6adf710281412efaa43795297dd1d82") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (path (name "Car Mass Rollup 1") (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (path (name "Car Mass Rollup 1") (anonymous (kind import) (ordinal 1)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "MassRollup2") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (path (name "Car Mass Rollup 1") (anonymous (kind import) (ordinal 2)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SI::kg") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::CarPart"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "MassedThing"))))
    (declaration (id (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::CarPart::serialNumber"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String"))))
    (declaration (id (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::c"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "car"))))
    (declaration (id (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (path (name "Car Mass Rollup 1") (name "c") (anonymous (kind attribute) (ordinal 0)))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "simpleMass"))))
    (declaration (id (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (path (name "Car Mass Rollup 1") (name "c") (anonymous (kind part) (ordinal 0)))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "engine"))))
    (declaration (id (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (path (name "Car Mass Rollup 1") (name "c") (anonymous (kind part) (ordinal 1)))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "transmission"))))
    (declaration (id (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (path (name "Car Mass Rollup 1") (name "c") (anonymous (kind part) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "simpleMass"))))
    (declaration (id (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (path (name "Car Mass Rollup 1") (name "c") (anonymous (kind part) (ordinal 1)) (anonymous (kind attribute) (ordinal 0)))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "simpleMass"))))
    (declaration (id (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::car"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CarPart")) (subsetting (reference "compositeThing"))))
    (declaration (id (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::car::carParts"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower unbounded) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CarPart")) (redefinition (reference "subcomponents"))))
    (declaration (id (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::car::engine"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "carParts"))))
    (declaration (id (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::car::transmission"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "carParts"))))
    (declaration (id (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::car::vin"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "serialNumber"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (path (name "Car Mass Rollup 1") (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (path (name "Car Mass Rollup 1") (anonymous (kind import) (ordinal 1)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "MassRollup2")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (path (name "Car Mass Rollup 1") (anonymous (kind import) (ordinal 2)))))) (kind membershipImport) (ordinal 0))
      (authored-target "SI::kg")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::CarPart"))) (kind specialization) (ordinal 0))
      (authored-target "MassedThing")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::CarPart::serialNumber"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::c"))) (kind subsetting) (ordinal 0))
      (authored-target "car")
      (outcome (status resolved) (target (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::car")))))
    (reference (id (source (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (path (name "Car Mass Rollup 1") (name "c") (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "simpleMass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (path (name "Car Mass Rollup 1") (name "c") (anonymous (kind part) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "engine")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (path (name "Car Mass Rollup 1") (name "c") (anonymous (kind part) (ordinal 1)))))) (kind redefinition) (ordinal 0))
      (authored-target "transmission")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (path (name "Car Mass Rollup 1") (name "c") (anonymous (kind part) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "simpleMass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (path (name "Car Mass Rollup 1") (name "c") (anonymous (kind part) (ordinal 1)) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "simpleMass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::car"))) (kind featureTyping) (ordinal 0))
      (authored-target "CarPart")
      (outcome (status resolved) (target (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::CarPart")))))
    (reference (id (source (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::car"))) (kind subsetting) (ordinal 0))
      (authored-target "compositeThing")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::car::carParts"))) (kind featureTyping) (ordinal 0))
      (authored-target "CarPart")
      (outcome (status resolved) (target (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::CarPart")))))
    (reference (id (source (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::car::carParts"))) (kind redefinition) (ordinal 0))
      (authored-target "subcomponents")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::car::engine"))) (kind subsetting) (ordinal 0))
      (authored-target "carParts")
      (outcome (status resolved) (target (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::car::carParts")))))
    (reference (id (source (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::car::transmission"))) (kind subsetting) (ordinal 0))
      (authored-target "carParts")
      (outcome (status resolved) (target (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::car::carParts")))))
    (reference (id (source (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::car::vin"))) (kind redefinition) (ordinal 0))
      (authored-target "serialNumber")
      (outcome (status resolved) (target (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::CarPart::serialNumber")))))
  )
  (relationships
    (relationship (kind subsetting) (source (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::c"))) (target (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::car"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::c"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::car"))) (target (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::CarPart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::car"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::car::carParts"))) (target (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::CarPart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::car::carParts"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::car::engine"))) (target (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::car::carParts"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::car::engine"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::car::transmission"))) (target (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::car::carParts"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::car::transmission"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::car::vin"))) (target (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::CarPart::serialNumber"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::car::vin"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (path (name "Car Mass Rollup 1") (name "c") (anonymous (kind attribute) (ordinal 0)))))) (value (kind quantity) (magnitude (value (kind integer) (integer 1000))) (unit "kg")))
    (evaluated (declaration (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (path (name "Car Mass Rollup 1") (name "c") (anonymous (kind part) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)))))) (value (kind quantity) (magnitude (value (kind integer) (integer 100))) (unit "kg")))
    (evaluated (declaration (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (path (name "Car Mass Rollup 1") (name "c") (anonymous (kind part) (ordinal 1)) (anonymous (kind attribute) (ordinal 0)))))) (value (kind quantity) (magnitude (value (kind integer) (integer 50))) (unit "kg")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/29_car_mass_rollup_example_2.md") (range (start 1 16) (end 1 31)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (path (name "Car Mass Rollup 1") (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/29_car_mass_rollup_example_2.md") (range (start 2 16) (end 2 30)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (path (name "Car Mass Rollup 1") (anonymous (kind import) (ordinal 1)))))) (kind namespaceImport) (ordinal 0) (authored-target "MassRollup2")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/29_car_mass_rollup_example_2.md") (range (start 24 16) (end 24 22)) (probe (position 24 16))
    (reference (id (source (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (path (name "Car Mass Rollup 1") (anonymous (kind import) (ordinal 2)))))) (kind membershipImport) (ordinal 0) (authored-target "SI::kg")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/29_car_mass_rollup_example_2.md") (range (start 4 21) (end 4 32)) (probe (position 4 21))
    (reference (id (source (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::CarPart"))) (kind specialization) (ordinal 0) (authored-target "MassedThing")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/29_car_mass_rollup_example_2.md") (range (start 5 26) (end 5 32)) (probe (position 5 26))
    (reference (id (source (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::CarPart::serialNumber"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/29_car_mass_rollup_example_2.md") (range (start 25 11) (end 25 14)) (probe (position 25 11))
    (reference (id (source (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::c"))) (kind subsetting) (ordinal 0) (authored-target "car")
      (outcome (status resolved) (target (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::car")))))
  )
  (query (document "memory://snapshot/29_car_mass_rollup_example_2.md") (range (start 26 16) (end 26 26)) (probe (position 26 16))
    (reference (id (source (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (path (name "Car Mass Rollup 1") (name "c") (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "simpleMass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/29_car_mass_rollup_example_2.md") (range (start 27 11) (end 27 17)) (probe (position 27 11))
    (reference (id (source (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (path (name "Car Mass Rollup 1") (name "c") (anonymous (kind part) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "engine")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/29_car_mass_rollup_example_2.md") (range (start 31 17) (end 31 29)) (probe (position 31 17))
    (reference (id (source (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (path (name "Car Mass Rollup 1") (name "c") (anonymous (kind part) (ordinal 1)))))) (kind redefinition) (ordinal 0) (authored-target "transmission")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/29_car_mass_rollup_example_2.md") (range (start 28 17) (end 28 27)) (probe (position 28 17))
    (reference (id (source (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (path (name "Car Mass Rollup 1") (name "c") (anonymous (kind part) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "simpleMass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/29_car_mass_rollup_example_2.md") (range (start 32 17) (end 32 27)) (probe (position 32 17))
    (reference (id (source (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (path (name "Car Mass Rollup 1") (name "c") (anonymous (kind part) (ordinal 1)) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "simpleMass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/29_car_mass_rollup_example_2.md") (range (start 8 11) (end 8 18)) (probe (position 8 11))
    (reference (id (source (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::car"))) (kind featureTyping) (ordinal 0) (authored-target "CarPart")
      (outcome (status resolved) (target (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::CarPart")))))
  )
  (query (document "memory://snapshot/29_car_mass_rollup_example_2.md") (range (start 8 22) (end 8 36)) (probe (position 8 22))
    (reference (id (source (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::car"))) (kind subsetting) (ordinal 0) (authored-target "compositeThing")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/29_car_mass_rollup_example_2.md") (range (start 11 17) (end 11 24)) (probe (position 11 17))
    (reference (id (source (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::car::carParts"))) (kind featureTyping) (ordinal 0) (authored-target "CarPart")
      (outcome (status resolved) (target (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::CarPart")))))
  )
  (query (document "memory://snapshot/29_car_mass_rollup_example_2.md") (range (start 11 32) (end 11 45)) (probe (position 11 32))
    (reference (id (source (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::car::carParts"))) (kind redefinition) (ordinal 0) (authored-target "subcomponents")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/29_car_mass_rollup_example_2.md") (range (start 13 17) (end 13 25)) (probe (position 13 17))
    (reference (id (source (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::car::engine"))) (kind subsetting) (ordinal 0) (authored-target "carParts")
      (outcome (status resolved) (target (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::car::carParts")))))
  )
  (query (document "memory://snapshot/29_car_mass_rollup_example_2.md") (range (start 17 23) (end 17 31)) (probe (position 17 23))
    (reference (id (source (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::car::transmission"))) (kind subsetting) (ordinal 0) (authored-target "carParts")
      (outcome (status resolved) (target (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::car::carParts")))))
  )
  (query (document "memory://snapshot/29_car_mass_rollup_example_2.md") (range (start 9 20) (end 9 32)) (probe (position 9 20))
    (reference (id (source (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::car::vin"))) (kind redefinition) (ordinal 0) (authored-target "serialNumber")
      (outcome (status resolved) (target (node (document "memory://snapshot/29_car_mass_rollup_example_2.md") (qualified-name "Car Mass Rollup 1::CarPart::serialNumber")))))
  )
)
~~~
