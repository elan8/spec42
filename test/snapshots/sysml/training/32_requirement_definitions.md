# META
~~~ini
description=SysML Training 32 (Requirements): Requirement Definitions
type=file
~~~
# SOURCE
~~~sysml
package 'Requirement Definitions' {
	private import ISQ::*;
	private import SI::*;

	requirement def MassLimitationRequirement {
		doc /* The actual mass shall be less than or equal to the required mass. */
		
		attribute massActual: MassValue;
		attribute massReqd: MassValue;
		
		require constraint { massActual <= massReqd }
	}
	
	part def Vehicle {
		attribute dryMass: MassValue;
		attribute fuelMass: MassValue;
		attribute fuelFullMass: MassValue;
	}
	
	requirement def <'1'> VehicleMassLimitationRequirement :> MassLimitationRequirement {
		doc /* The total mass of a vehicle shall be less than or equal to the required mass. */
		
		subject vehicle : Vehicle;
		
		attribute redefines massActual = vehicle.dryMass + vehicle.fuelMass;
		
		assume constraint { vehicle.fuelMass > 0[kg] }
	}
	
	port def ClutchPort;
	action def GenerateTorque;
	
	requirement def <'2'> DrivePowerInterface {
		doc /* The engine shall transfer its generated torque to the transmission via the clutch interface. */
		subject clutchPort: ClutchPort;
	}
		
	requirement def <'3'> TorqueGeneration {
		doc /* The engine shall generate torque as a function of RPM as shown in Table 1. */
		subject generateTorque: GenerateTorque;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/32_requirement_definitions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 24) (end 7 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 8 22) (end 8 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 10 2) (end 10 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 21) (end 14 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 22) (end 15 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 26) (end 16 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 22 2) (end 22 28))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 24 22) (end 24 32))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 26 2) (end 26 48))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 34 2) (end 34 33))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 39 2) (end 39 41))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:f8e8702388e9249c93504cf8e632dee4abfd928cbc3a93dd387d41f83ee18d05") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ISQ") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "SI") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::ClutchPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::DrivePowerInterface"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::GenerateTorque"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::MassLimitationRequirement"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::MassLimitationRequirement::massActual"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue"))))
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::MassLimitationRequirement::massReqd"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue"))))
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::TorqueGeneration"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::Vehicle::dryMass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue"))))
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::Vehicle::fuelFullMass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue"))))
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::Vehicle::fuelMass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue"))))
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement"))) (kind requirement-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "MassLimitationRequirement"))))
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "massActual"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/32_requirement_definitions.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ISQ")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_definitions.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "SI")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::MassLimitationRequirement::massActual"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::MassLimitationRequirement::massReqd"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::Vehicle::dryMass"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::Vehicle::fuelFullMass"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::Vehicle::fuelMass"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement"))) (kind specialization) (ordinal 0))
      (authored-target "MassLimitationRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::MassLimitationRequirement")))))
    (reference (id (source (node (document "memory://snapshot/32_requirement_definitions.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "massActual")
      (outcome (status unsupported)))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement"))) (target (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::MassLimitationRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/32_requirement_definitions.md") (range (start 1 16) (end 1 22)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/32_requirement_definitions.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ISQ")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/32_requirement_definitions.md") (range (start 2 16) (end 2 21)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/32_requirement_definitions.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "SI")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/32_requirement_definitions.md") (range (start 7 24) (end 7 33)) (probe (position 7 24))
    (reference (id (source (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::MassLimitationRequirement::massActual"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/32_requirement_definitions.md") (range (start 8 22) (end 8 31)) (probe (position 8 22))
    (reference (id (source (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::MassLimitationRequirement::massReqd"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/32_requirement_definitions.md") (range (start 14 21) (end 14 30)) (probe (position 14 21))
    (reference (id (source (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::Vehicle::dryMass"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/32_requirement_definitions.md") (range (start 16 26) (end 16 35)) (probe (position 16 26))
    (reference (id (source (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::Vehicle::fuelFullMass"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/32_requirement_definitions.md") (range (start 15 22) (end 15 31)) (probe (position 15 22))
    (reference (id (source (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::Vehicle::fuelMass"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/32_requirement_definitions.md") (range (start 19 59) (end 19 84)) (probe (position 19 59))
    (reference (id (source (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement"))) (kind specialization) (ordinal 0) (authored-target "MassLimitationRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::MassLimitationRequirement")))))
  )
  (query (document "memory://snapshot/32_requirement_definitions.md") (range (start 24 22) (end 24 32)) (probe (position 24 22))
    (reference (id (source (node (document "memory://snapshot/32_requirement_definitions.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "massActual")
      (outcome (status unsupported)))
  )
)
~~~
