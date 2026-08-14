# META
~~~ini
description=SysML Training 32 (Requirements): Requirement Usages
type=file
~~~
# SOURCE
~~~sysml
package 'Requirement Usages' {
	private import SI::*;
	private import 'Requirement Definitions'::*;
	
	requirement <'1.1'> fullVehicleMassLimit : VehicleMassLimitationRequirement {
		subject vehicle : Vehicle;
		attribute :>> massReqd = 2000[kg];
		
		assume constraint {
			doc /* Full tank is full. */
			vehicle.fuelMass == vehicle.fuelFullMass
		}
	}
	
	requirement <'1.2'> emptyVehicleMassLimit : VehicleMassLimitationRequirement {
		subject vehicle : Vehicle;
		attribute :>> massReqd = 1500[kg];
		
		assume constraint {
			doc /* Full tank is empty. */
			vehicle.fuelMass == 0[kg]
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/32_requirement_usages.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 4 44) (end 4 76))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 20) (end 5 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 6 16) (end 6 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 10 3) (end 10 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 10 23) (end 10 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 45) (end 14 77))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 20) (end 15 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 16 16) (end 16 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 20 3) (end 20 19))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:7330865c3611c2ae66217b7db3030ee73531944bf191d84e5f789335317febb9") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/32_requirement_usages.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "SI") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/32_requirement_usages.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Requirement Definitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages::emptyVehicleMassLimit"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleMassLimitationRequirement"))))
    (declaration (id (node (document "memory://snapshot/32_requirement_usages.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "massReqd"))))
    (declaration (id (node (document "memory://snapshot/32_requirement_usages.md") (anonymous (kind constraint) (ordinal 0))))) (kind constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "vehicle::fuelMass"))))
    (declaration (id (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages::emptyVehicleMassLimit::vehicle"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages::fullVehicleMassLimit"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleMassLimitationRequirement"))))
    (declaration (id (node (document "memory://snapshot/32_requirement_usages.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "massReqd"))))
    (declaration (id (node (document "memory://snapshot/32_requirement_usages.md") (anonymous (kind constraint) (ordinal 0))))) (kind constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "vehicle::fuelMass")) (memberAccessOperand (reference "vehicle::fuelFullMass"))))
    (declaration (id (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages::fullVehicleMassLimit::vehicle"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "SI")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Requirement Definitions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages::emptyVehicleMassLimit"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleMassLimitationRequirement")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "massReqd")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (anonymous (kind constraint) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "vehicle::fuelMass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages::emptyVehicleMassLimit::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages::fullVehicleMassLimit"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleMassLimitationRequirement")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "massReqd")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (anonymous (kind constraint) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "vehicle::fuelMass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (anonymous (kind constraint) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "vehicle::fuelFullMass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages::fullVehicleMassLimit::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/32_requirement_usages.md") (anonymous (kind attribute) (ordinal 0))))) (value (kind quantity) (magnitude (value (kind integer) (integer 1500))) (unit "kg")))
    (evaluated (declaration (node (document "memory://snapshot/32_requirement_usages.md") (anonymous (kind attribute) (ordinal 0))))) (value (kind quantity) (magnitude (value (kind integer) (integer 2000))) (unit "kg")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/32_requirement_usages.md") (range (start 1 16) (end 1 21)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "SI")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/32_requirement_usages.md") (range (start 2 16) (end 2 44)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "Requirement Definitions")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/32_requirement_usages.md") (range (start 14 45) (end 14 77)) (probe (position 14 45))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages::emptyVehicleMassLimit"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleMassLimitationRequirement")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/32_requirement_usages.md") (range (start 16 16) (end 16 24)) (probe (position 16 16))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "massReqd")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/32_requirement_usages.md") (range (start 20 3) (end 20 19)) (probe (position 20 3))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (anonymous (kind constraint) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "vehicle::fuelMass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/32_requirement_usages.md") (range (start 15 20) (end 15 27)) (probe (position 15 20))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages::emptyVehicleMassLimit::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/32_requirement_usages.md") (range (start 4 44) (end 4 76)) (probe (position 4 44))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages::fullVehicleMassLimit"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleMassLimitationRequirement")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/32_requirement_usages.md") (range (start 6 16) (end 6 24)) (probe (position 6 16))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "massReqd")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/32_requirement_usages.md") (range (start 10 3) (end 10 19)) (probe (position 10 3))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (anonymous (kind constraint) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "vehicle::fuelMass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/32_requirement_usages.md") (range (start 10 23) (end 10 43)) (probe (position 10 23))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (anonymous (kind constraint) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1) (authored-target "vehicle::fuelFullMass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/32_requirement_usages.md") (range (start 5 20) (end 5 27)) (probe (position 5 20))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages::fullVehicleMassLimit::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status unresolved)))
  )
)
~~~
