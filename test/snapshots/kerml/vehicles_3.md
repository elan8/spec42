# META
~~~ini
description=KerML Mass Roll-up: Vehicles_3
type=file
~~~
# SOURCE
~~~kerml
package Vehicles_3 {
	private import ScalarValues::*;
	private import MassRollup_2::*;
	
	class CarPart specializes MassedThing {			
		feature serialNumber: String;
		feature m redefines MassedThing::mass;
		
		feature subparts redefines carParts;	
	}
	
	composite feature carParts: CarPart[0..*] subsets massedThings;
	
	feature vehicle subsets carParts {	
		feature vin redefines serialNumber;
		
		feature redefines engine;
		feature redefines transmission;
	}
	
	composite feature engine subsets carParts {
		//...
	}
	
	composite feature transmission subsets carParts {
		//...
	}

	// Example usage
	
	private import SI::*;
	feature v: vehicle {
		feature m redefines CarPart::m = 1000;
		composite :>> engine = e;
		composite :>> transmission = t;
	}
	
	feature e :> engine {
		feature m redefines CarPart::m = 100;
	}
	
	feature t :> transmission {
		feature m redefines CarPart::m = 50;
	}
	
	// v.totalMass evaluates to 1150.0
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/vehicles_3.md"
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
        (range (start 11 51) (end 11 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 14 24) (end 14 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 30 16) (end 30 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 32 22) (end 32 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 38 22) (end 38 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 42 22) (end 42 32))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:fcd65e1426e444ae0a1d57008064d707d1d3d251db73d7755b52e695f79cee58") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicles_3.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/vehicles_3.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "MassRollup_2") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/vehicles_3.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "SI") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::CarPart"))) (kind class-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "MassedThing"))))
    (declaration (id (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::carParts"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CarPart")) (subsetting (reference "massedThings"))))
    (declaration (id (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::e"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "engine"))))
    (declaration (id (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::e::m"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "CarPart::m"))))
    (declaration (id (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::engine"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "carParts"))))
    (declaration (id (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::t"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "transmission"))))
    (declaration (id (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::t::m"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "CarPart::m"))))
    (declaration (id (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::transmission"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "carParts"))))
    (declaration (id (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::v"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "vehicle"))))
    (declaration (id (node (document "memory://snapshot/vehicles_3.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "engine")) (expressionOperand (reference "e"))))
    (declaration (id (node (document "memory://snapshot/vehicles_3.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "transmission")) (expressionOperand (reference "t"))))
    (declaration (id (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::v::m"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "CarPart::m"))))
    (declaration (id (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::vehicle"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "carParts"))))
    (declaration (id (node (document "memory://snapshot/vehicles_3.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "engine"))))
    (declaration (id (node (document "memory://snapshot/vehicles_3.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "transmission"))))
    (declaration (id (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::vehicle::vin"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "serialNumber"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/vehicles_3.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicles_3.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "MassRollup_2")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicles_3.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "SI")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::CarPart"))) (kind specialization) (ordinal 0))
      (authored-target "MassedThing")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::carParts"))) (kind featureTyping) (ordinal 0))
      (authored-target "CarPart")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::CarPart")))))
    (reference (id (source (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::carParts"))) (kind subsetting) (ordinal 0))
      (authored-target "massedThings")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::e"))) (kind subsetting) (ordinal 0))
      (authored-target "engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::engine")))))
    (reference (id (source (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::e::m"))) (kind redefinition) (ordinal 0))
      (authored-target "CarPart::m")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::engine"))) (kind subsetting) (ordinal 0))
      (authored-target "carParts")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::carParts")))))
    (reference (id (source (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::t"))) (kind subsetting) (ordinal 0))
      (authored-target "transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::transmission")))))
    (reference (id (source (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::t::m"))) (kind redefinition) (ordinal 0))
      (authored-target "CarPart::m")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::transmission"))) (kind subsetting) (ordinal 0))
      (authored-target "carParts")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::carParts")))))
    (reference (id (source (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::v"))) (kind featureTyping) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::vehicle")))))
    (reference (id (source (node (document "memory://snapshot/vehicles_3.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::engine")))))
    (reference (id (source (node (document "memory://snapshot/vehicles_3.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::transmission")))))
    (reference (id (source (node (document "memory://snapshot/vehicles_3.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "e")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::e")))))
    (reference (id (source (node (document "memory://snapshot/vehicles_3.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind expressionOperand) (ordinal 0))
      (authored-target "t")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::t")))))
    (reference (id (source (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::v::m"))) (kind redefinition) (ordinal 0))
      (authored-target "CarPart::m")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::vehicle"))) (kind subsetting) (ordinal 0))
      (authored-target "carParts")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::carParts")))))
    (reference (id (source (node (document "memory://snapshot/vehicles_3.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::engine")))))
    (reference (id (source (node (document "memory://snapshot/vehicles_3.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::transmission")))))
    (reference (id (source (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::vehicle::vin"))) (kind redefinition) (ordinal 0))
      (authored-target "serialNumber")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::carParts"))) (target (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::CarPart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::carParts"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::e"))) (target (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::e"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::engine"))) (target (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::carParts"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::engine"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::t"))) (target (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::transmission"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::t"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::transmission"))) (target (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::carParts"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::transmission"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::v"))) (target (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::v"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/vehicles_3.md") (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicles_3.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/vehicles_3.md") (anonymous (kind kerml-feature) (ordinal 1))))) (target (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::transmission"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicles_3.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/vehicles_3.md") (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::e"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicles_3.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/vehicles_3.md") (anonymous (kind kerml-feature) (ordinal 1))))) (target (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::t"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicles_3.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::vehicle"))) (target (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::carParts"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::vehicle"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/vehicles_3.md") (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicles_3.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/vehicles_3.md") (anonymous (kind kerml-feature) (ordinal 1))))) (target (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::transmission"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicles_3.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::e::m"))) (value (kind integer) (integer 100)))
    (evaluated (declaration (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::t::m"))) (value (kind integer) (integer 50)))
    (evaluated (declaration (node (document "memory://snapshot/vehicles_3.md") (anonymous (kind kerml-feature) (ordinal 0))))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/vehicles_3.md") (anonymous (kind kerml-feature) (ordinal 1))))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::v::m"))) (value (kind integer) (integer 1000)))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/vehicles_3.md") (range (start 1 16) (end 1 31)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/vehicles_3.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicles_3.md") (range (start 2 16) (end 2 31)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/vehicles_3.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "MassRollup_2")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicles_3.md") (range (start 30 16) (end 30 21)) (probe (position 30 16))
    (reference (id (source (node (document "memory://snapshot/vehicles_3.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "SI")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicles_3.md") (range (start 4 27) (end 4 38)) (probe (position 4 27))
    (reference (id (source (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::CarPart"))) (kind specialization) (ordinal 0) (authored-target "MassedThing")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicles_3.md") (range (start 11 29) (end 11 36)) (probe (position 11 29))
    (reference (id (source (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::carParts"))) (kind featureTyping) (ordinal 0) (authored-target "CarPart")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::CarPart")))))
  )
  (query (document "memory://snapshot/vehicles_3.md") (range (start 11 51) (end 11 63)) (probe (position 11 51))
    (reference (id (source (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::carParts"))) (kind subsetting) (ordinal 0) (authored-target "massedThings")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicles_3.md") (range (start 37 14) (end 37 20)) (probe (position 37 14))
    (reference (id (source (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::e"))) (kind subsetting) (ordinal 0) (authored-target "engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::engine")))))
  )
  (query (document "memory://snapshot/vehicles_3.md") (range (start 38 22) (end 38 32)) (probe (position 38 22))
    (reference (id (source (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::e::m"))) (kind redefinition) (ordinal 0) (authored-target "CarPart::m")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicles_3.md") (range (start 20 34) (end 20 42)) (probe (position 20 34))
    (reference (id (source (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::engine"))) (kind subsetting) (ordinal 0) (authored-target "carParts")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::carParts")))))
  )
  (query (document "memory://snapshot/vehicles_3.md") (range (start 41 14) (end 41 26)) (probe (position 41 14))
    (reference (id (source (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::t"))) (kind subsetting) (ordinal 0) (authored-target "transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::transmission")))))
  )
  (query (document "memory://snapshot/vehicles_3.md") (range (start 42 22) (end 42 32)) (probe (position 42 22))
    (reference (id (source (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::t::m"))) (kind redefinition) (ordinal 0) (authored-target "CarPart::m")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicles_3.md") (range (start 24 40) (end 24 48)) (probe (position 24 40))
    (reference (id (source (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::transmission"))) (kind subsetting) (ordinal 0) (authored-target "carParts")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::carParts")))))
  )
  (query (document "memory://snapshot/vehicles_3.md") (range (start 31 12) (end 31 19)) (probe (position 31 12))
    (reference (id (source (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::v"))) (kind featureTyping) (ordinal 0) (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::vehicle")))))
  )
  (query (document "memory://snapshot/vehicles_3.md") (range (start 33 16) (end 33 22)) (probe (position 33 16))
    (reference (id (source (node (document "memory://snapshot/vehicles_3.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::engine")))))
  )
  (query (document "memory://snapshot/vehicles_3.md") (range (start 34 16) (end 34 28)) (probe (position 34 16))
    (reference (id (source (node (document "memory://snapshot/vehicles_3.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::transmission")))))
  )
  (query (document "memory://snapshot/vehicles_3.md") (range (start 33 25) (end 33 26)) (probe (position 33 25))
    (reference (id (source (node (document "memory://snapshot/vehicles_3.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "e")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::e")))))
  )
  (query (document "memory://snapshot/vehicles_3.md") (range (start 34 31) (end 34 32)) (probe (position 34 31))
    (reference (id (source (node (document "memory://snapshot/vehicles_3.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind expressionOperand) (ordinal 0) (authored-target "t")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::t")))))
  )
  (query (document "memory://snapshot/vehicles_3.md") (range (start 32 22) (end 32 32)) (probe (position 32 22))
    (reference (id (source (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::v::m"))) (kind redefinition) (ordinal 0) (authored-target "CarPart::m")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicles_3.md") (range (start 13 25) (end 13 33)) (probe (position 13 25))
    (reference (id (source (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::vehicle"))) (kind subsetting) (ordinal 0) (authored-target "carParts")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::carParts")))))
  )
  (query (document "memory://snapshot/vehicles_3.md") (range (start 16 20) (end 16 26)) (probe (position 16 20))
    (reference (id (source (node (document "memory://snapshot/vehicles_3.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::engine")))))
  )
  (query (document "memory://snapshot/vehicles_3.md") (range (start 17 20) (end 17 32)) (probe (position 17 20))
    (reference (id (source (node (document "memory://snapshot/vehicles_3.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::transmission")))))
  )
  (query (document "memory://snapshot/vehicles_3.md") (range (start 14 24) (end 14 36)) (probe (position 14 24))
    (reference (id (source (node (document "memory://snapshot/vehicles_3.md") (qualified-name "Vehicles_3::vehicle::vin"))) (kind redefinition) (ordinal 0) (authored-target "serialNumber")
      (outcome (status unresolved)))
  )
)
~~~
