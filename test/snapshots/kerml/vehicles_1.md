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
  (document "memory://snapshot/vehicles_1.md"
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
        (range (start 8 2) (end 9 2))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 9 2) (end 10 1))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 12 26) (end 12 37))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 13 2) (end 14 2))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 14 2) (end 17 1))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 19 32) (end 19 43))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 20 2) (end 21 2))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 21 2) (end 24 1))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 28 16) (end 28 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 30 22) (end 30 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 31 29) (end 31 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 32 23) (end 32 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 34 35) (end 34 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 35 23) (end 35 38))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:5e1ea58ab9230ff5d65a916c54af2f189212ffe42838e897fca72d0533fddc7c") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicles_1.md") (path (named (kind package) (name "Vehicles_1")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::String") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/vehicles_1.md") (path (named (kind package) (name "Vehicles_1")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "MassRollup_1") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/vehicles_1.md") (path (named (kind package) (name "Vehicles_1")) (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "SI") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::Engine"))) (kind class-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "MassedThing")))))
    (declaration (id (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::Transmission"))) (kind class-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "MassedThing")))))
    (declaration (id (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::Vehicle"))) (kind class-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "MassedThing")))))
    (declaration (id (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::v"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::v::engine"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers composite)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "Vehicle::engine")))))
    (declaration (id (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::v::engine::m"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "Engine::m")))))
    (declaration (id (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::v::m"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "Vehicle::m")))))
    (declaration (id (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::v::transmission"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers composite)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "Vehicle::transmission")))))
    (declaration (id (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::v::transmission::m"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "Transmission::m")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/vehicles_1.md") (path (named (kind package) (name "Vehicles_1")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "MassRollup_1")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicles_1.md") (path (named (kind package) (name "Vehicles_1")) (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "SI")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicles_1.md") (path (named (kind package) (name "Vehicles_1")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::Engine"))) (kind specialization) (ordinal 0))
      (authored-target "MassedThing")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::Transmission"))) (kind specialization) (ordinal 0))
      (authored-target "MassedThing")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::Vehicle"))) (kind specialization) (ordinal 0))
      (authored-target "MassedThing")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::v"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::v::engine"))) (kind redefinition) (ordinal 0))
      (authored-target "Vehicle::engine")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::v::engine::m"))) (kind redefinition) (ordinal 0))
      (authored-target "Engine::m")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::v::m"))) (kind redefinition) (ordinal 0))
      (authored-target "Vehicle::m")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::v::transmission"))) (kind redefinition) (ordinal 0))
      (authored-target "Vehicle::transmission")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::v::transmission::m"))) (kind redefinition) (ordinal 0))
      (authored-target "Transmission::m")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::v"))) (target (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::v"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::v::engine::m"))) (state literal) (value (kind integer) (integer 100)))
    (evaluated (declaration (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::v::m"))) (state literal) (value (kind integer) (integer 1000)))
    (evaluated (declaration (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::v::transmission::m"))) (state literal) (value (kind integer) (integer 50)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::Vehicle")))
      (subtype (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::v")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::v")))
      (type (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::Vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::v::engine")))
      (featured-by (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::v")))
    )
    (declaration (id (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::v::engine::m")))
      (featured-by (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::v::engine")))
    )
    (declaration (id (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::v::m")))
      (featured-by (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::v")))
    )
    (declaration (id (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::v::transmission")))
      (featured-by (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::v")))
    )
    (declaration (id (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::v::transmission::m")))
      (featured-by (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::v::transmission")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/vehicles_1.md") (range (start 2 16) (end 2 31)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/vehicles_1.md") (path (named (kind package) (name "Vehicles_1")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "MassRollup_1")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicles_1.md") (range (start 28 16) (end 28 21)) (probe (position 28 16))
    (reference (id (source (node (document "memory://snapshot/vehicles_1.md") (path (named (kind package) (name "Vehicles_1")) (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "SI")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicles_1.md") (range (start 1 16) (end 1 36)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/vehicles_1.md") (path (named (kind package) (name "Vehicles_1")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::String")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicles_1.md") (range (start 12 26) (end 12 37)) (probe (position 12 26))
    (reference (id (source (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::Engine"))) (kind specialization) (ordinal 0) (authored-target "MassedThing")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicles_1.md") (range (start 19 32) (end 19 43)) (probe (position 19 32))
    (reference (id (source (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::Transmission"))) (kind specialization) (ordinal 0) (authored-target "MassedThing")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicles_1.md") (range (start 4 27) (end 4 38)) (probe (position 4 27))
    (reference (id (source (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::Vehicle"))) (kind specialization) (ordinal 0) (authored-target "MassedThing")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicles_1.md") (range (start 29 12) (end 29 19)) (probe (position 29 12))
    (reference (id (source (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::v"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/vehicles_1.md") (range (start 31 29) (end 31 44)) (probe (position 31 29))
    (reference (id (source (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::v::engine"))) (kind redefinition) (ordinal 0) (authored-target "Vehicle::engine")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicles_1.md") (range (start 32 23) (end 32 32)) (probe (position 32 23))
    (reference (id (source (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::v::engine::m"))) (kind redefinition) (ordinal 0) (authored-target "Engine::m")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicles_1.md") (range (start 30 22) (end 30 32)) (probe (position 30 22))
    (reference (id (source (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::v::m"))) (kind redefinition) (ordinal 0) (authored-target "Vehicle::m")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicles_1.md") (range (start 34 35) (end 34 56)) (probe (position 34 35))
    (reference (id (source (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::v::transmission"))) (kind redefinition) (ordinal 0) (authored-target "Vehicle::transmission")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicles_1.md") (range (start 35 23) (end 35 38)) (probe (position 35 23))
    (reference (id (source (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1::v::transmission::m"))) (kind redefinition) (ordinal 0) (authored-target "Transmission::m")
      (outcome (status unresolved)))
    )
  )
)
~~~
