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
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 16) (end 1 22))
      )
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
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:f8e8702388e9249c93504cf8e632dee4abfd928cbc3a93dd387d41f83ee18d05"))
  (declarations
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ISQ") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "SI") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::ClutchPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::DrivePowerInterface"))) (kind requirement-def) (membership (kind owning) (visibility default)) (facts (short-name "2")) (documentation (doc (text " The engine shall transfer its generated torque to the transmission via the clutch interface. "))))
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::DrivePowerInterface::clutchPort"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ClutchPort")))))
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::GenerateTorque"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::MassLimitationRequirement"))) (kind requirement-def) (membership (kind owning) (visibility default)) (documentation (doc (text " The actual mass shall be less than or equal to the required mass. "))))
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (named (kind requirement-def) (name "MassLimitationRequirement")) (anonymous (kind require-constraint) (ordinal 0))))) (kind require-constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "massActual")) (expressionOperand (reference "massReqd")))))
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::MassLimitationRequirement::massActual"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue")))))
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::MassLimitationRequirement::massReqd"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue")))))
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::TorqueGeneration"))) (kind requirement-def) (membership (kind owning) (visibility default)) (facts (short-name "3")) (documentation (doc (text " The engine shall generate torque as a function of RPM as shown in Table 1. "))))
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::TorqueGeneration::generateTorque"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "GenerateTorque")))))
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::Vehicle::dryMass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue")))))
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::Vehicle::fuelFullMass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue")))))
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::Vehicle::fuelMass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue")))))
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement"))) (kind requirement-def) (membership (kind owning) (visibility default)) (facts (short-name "1")) (documentation (doc (text " The total mass of a vehicle shall be less than or equal to the required mass. "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "MassLimitationRequirement")))))
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (named (kind requirement-def) (name "VehicleMassLimitationRequirement")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (effective-identification (name "massActual") (short-name absent) (provenance first-redefinition)) (feature-value (kind bind) (value (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (named (kind requirement-def) (name "VehicleMassLimitationRequirement")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (named (kind requirement-def) (name "VehicleMassLimitationRequirement")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "massActual")))))
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (named (kind requirement-def) (name "VehicleMassLimitationRequirement")) (anonymous (kind assume-constraint) (ordinal 0))))) (kind assume-constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "vehicle::fuelMass")))))
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (named (kind requirement-def) (name "VehicleMassLimitationRequirement")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (named (kind requirement-def) (name "VehicleMassLimitationRequirement")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (memberAccessOperand (reference "vehicle::dryMass")) (memberAccessOperand (reference "vehicle::fuelMass")))))
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (named (kind requirement-def) (name "VehicleMassLimitationRequirement")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement::vehicle"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ISQ")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "SI")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::DrivePowerInterface::clutchPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "ClutchPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::ClutchPort")))))
    (reference (id (source (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (named (kind requirement-def) (name "MassLimitationRequirement")) (anonymous (kind require-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "massActual")
      (outcome (status resolved) (target (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::MassLimitationRequirement::massActual")))))
    (reference (id (source (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (named (kind requirement-def) (name "MassLimitationRequirement")) (anonymous (kind require-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 1))
      (authored-target "massReqd")
      (outcome (status resolved) (target (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::MassLimitationRequirement::massReqd")))))
    (reference (id (source (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::MassLimitationRequirement::massActual"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::MassLimitationRequirement::massReqd"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::TorqueGeneration::generateTorque"))) (kind featureTyping) (ordinal 0))
      (authored-target "GenerateTorque")
      (outcome (status resolved) (target (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::GenerateTorque")))))
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
    (reference (id (source (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (named (kind requirement-def) (name "VehicleMassLimitationRequirement")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "massActual")
      (outcome (status resolved) (target (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::MassLimitationRequirement::massActual")))))
    (reference (id (source (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (named (kind requirement-def) (name "VehicleMassLimitationRequirement")) (anonymous (kind assume-constraint) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "vehicle::fuelMass")
      (outcome (status resolved) (target (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::Vehicle::fuelMass")))))
    (reference (id (source (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (named (kind requirement-def) (name "VehicleMassLimitationRequirement")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "vehicle::dryMass")
      (outcome (status resolved) (target (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::Vehicle::dryMass")))))
    (reference (id (source (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (named (kind requirement-def) (name "VehicleMassLimitationRequirement")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "vehicle::fuelMass")
      (outcome (status resolved) (target (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::Vehicle::fuelMass")))))
    (reference (id (source (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::Vehicle")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::DrivePowerInterface::clutchPort"))) (target (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::ClutchPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::DrivePowerInterface::clutchPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (named (kind requirement-def) (name "MassLimitationRequirement")) (anonymous (kind require-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::MassLimitationRequirement::massActual"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (named (kind requirement-def) (name "MassLimitationRequirement")) (anonymous (kind require-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (named (kind requirement-def) (name "MassLimitationRequirement")) (anonymous (kind require-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::MassLimitationRequirement::massReqd"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (named (kind requirement-def) (name "MassLimitationRequirement")) (anonymous (kind require-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::TorqueGeneration::generateTorque"))) (target (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::GenerateTorque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::TorqueGeneration::generateTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement"))) (target (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::MassLimitationRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (named (kind requirement-def) (name "VehicleMassLimitationRequirement")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::MassLimitationRequirement::massActual"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (named (kind requirement-def) (name "VehicleMassLimitationRequirement")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (named (kind requirement-def) (name "VehicleMassLimitationRequirement")) (anonymous (kind assume-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::Vehicle::fuelMass"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (named (kind requirement-def) (name "VehicleMassLimitationRequirement")) (anonymous (kind assume-constraint) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (named (kind requirement-def) (name "VehicleMassLimitationRequirement")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::Vehicle::dryMass"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (named (kind requirement-def) (name "VehicleMassLimitationRequirement")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (named (kind requirement-def) (name "VehicleMassLimitationRequirement")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::Vehicle::fuelMass"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (named (kind requirement-def) (name "VehicleMassLimitationRequirement")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement::vehicle"))) (target (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::DrivePowerInterface::clutchPort"))) (target (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::DrivePowerInterface"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (named (kind requirement-def) (name "MassLimitationRequirement")) (anonymous (kind require-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::MassLimitationRequirement"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::MassLimitationRequirement::massActual"))) (target (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::MassLimitationRequirement"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::MassLimitationRequirement::massReqd"))) (target (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::MassLimitationRequirement"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::TorqueGeneration::generateTorque"))) (target (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::TorqueGeneration"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::Vehicle::dryMass"))) (target (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::Vehicle::fuelFullMass"))) (target (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::Vehicle::fuelMass"))) (target (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (named (kind requirement-def) (name "VehicleMassLimitationRequirement")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (named (kind requirement-def) (name "VehicleMassLimitationRequirement")) (anonymous (kind assume-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (named (kind requirement-def) (name "VehicleMassLimitationRequirement")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (named (kind requirement-def) (name "VehicleMassLimitationRequirement")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement::vehicle"))) (target (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (named (kind requirement-def) (name "MassLimitationRequirement")) (anonymous (kind require-constraint) (ordinal 0))))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (named (kind requirement-def) (name "VehicleMassLimitationRequirement")) (anonymous (kind assume-constraint) (ordinal 0))))) (state unsupported))
    (evaluated (declaration (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (named (kind requirement-def) (name "VehicleMassLimitationRequirement")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (state unsupported))
    (unit (declaration (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (named (kind requirement-def) (name "VehicleMassLimitationRequirement")) (anonymous (kind assume-constraint) (ordinal 0))))) (ordinal 0) (authored "kg") (start 26 43) (end 26 45) (outcome (status catalog-unavailable)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::ClutchPort")))
      (subtype (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::DrivePowerInterface::clutchPort")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::DrivePowerInterface::clutchPort")))
      (featured-by (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::DrivePowerInterface")))
      (type (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::ClutchPort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::ClutchPort")) (source direct))
      (supertype (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::ClutchPort")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::GenerateTorque")))
      (subtype (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::TorqueGeneration::generateTorque")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::MassLimitationRequirement")))
      (subtype (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (named (kind requirement-def) (name "MassLimitationRequirement")) (anonymous (kind require-constraint) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::MassLimitationRequirement")))
    )
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::MassLimitationRequirement::massActual")))
      (featured-by (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::MassLimitationRequirement")))
      (subtype (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (named (kind requirement-def) (name "VehicleMassLimitationRequirement")) (anonymous (kind attribute) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::MassLimitationRequirement::massReqd")))
      (featured-by (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::MassLimitationRequirement")))
    )
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::TorqueGeneration::generateTorque")))
      (featured-by (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::TorqueGeneration")))
      (type (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::GenerateTorque")) (provenance authored))
      (effective-type (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::GenerateTorque")) (source direct))
      (supertype (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::GenerateTorque")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::Vehicle")))
      (subtype (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement::vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::Vehicle::dryMass")))
      (featured-by (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::Vehicle::fuelFullMass")))
      (featured-by (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::Vehicle::fuelMass")))
      (featured-by (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement")))
      (supertype (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::MassLimitationRequirement")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (named (kind requirement-def) (name "VehicleMassLimitationRequirement")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement")))
      (supertype (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::MassLimitationRequirement::massActual")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (named (kind requirement-def) (name "VehicleMassLimitationRequirement")) (anonymous (kind assume-constraint) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement")))
    )
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (named (kind requirement-def) (name "VehicleMassLimitationRequirement")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (named (kind requirement-def) (name "VehicleMassLimitationRequirement")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement::vehicle")))
      (featured-by (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement")))
      (type (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::Vehicle")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/32_requirement_definitions.md") (range (start 1 16) (end 1 22)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ISQ")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/32_requirement_definitions.md") (range (start 2 16) (end 2 21)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "SI")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/32_requirement_definitions.md") (range (start 34 22) (end 34 32)) (probe (position 34 22))
    (reference (id (source (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::DrivePowerInterface::clutchPort"))) (kind featureTyping) (ordinal 0) (authored-target "ClutchPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::ClutchPort")))))
    )
  )
  (query (document "memory://snapshot/32_requirement_definitions.md") (range (start 10 23) (end 10 33)) (probe (position 10 23))
    (reference (id (source (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (named (kind requirement-def) (name "MassLimitationRequirement")) (anonymous (kind require-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "massActual")
      (outcome (status resolved) (target (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::MassLimitationRequirement::massActual")))))
    )
  )
  (query (document "memory://snapshot/32_requirement_definitions.md") (range (start 10 37) (end 10 45)) (probe (position 10 37))
    (reference (id (source (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (named (kind requirement-def) (name "MassLimitationRequirement")) (anonymous (kind require-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 1) (authored-target "massReqd")
      (outcome (status resolved) (target (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::MassLimitationRequirement::massReqd")))))
    )
  )
  (query (document "memory://snapshot/32_requirement_definitions.md") (range (start 7 24) (end 7 33)) (probe (position 7 24))
    (reference (id (source (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::MassLimitationRequirement::massActual"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/32_requirement_definitions.md") (range (start 8 22) (end 8 31)) (probe (position 8 22))
    (reference (id (source (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::MassLimitationRequirement::massReqd"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/32_requirement_definitions.md") (range (start 39 26) (end 39 40)) (probe (position 39 26))
    (reference (id (source (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::TorqueGeneration::generateTorque"))) (kind featureTyping) (ordinal 0) (authored-target "GenerateTorque")
      (outcome (status resolved) (target (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::GenerateTorque")))))
    )
  )
  (query (document "memory://snapshot/32_requirement_definitions.md") (range (start 14 21) (end 14 30)) (probe (position 14 21))
    (reference (id (source (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::Vehicle::dryMass"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/32_requirement_definitions.md") (range (start 16 26) (end 16 35)) (probe (position 16 26))
    (reference (id (source (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::Vehicle::fuelFullMass"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/32_requirement_definitions.md") (range (start 15 22) (end 15 31)) (probe (position 15 22))
    (reference (id (source (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::Vehicle::fuelMass"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/32_requirement_definitions.md") (range (start 19 59) (end 19 84)) (probe (position 19 59))
    (reference (id (source (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement"))) (kind specialization) (ordinal 0) (authored-target "MassLimitationRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::MassLimitationRequirement")))))
    )
  )
  (query (document "memory://snapshot/32_requirement_definitions.md") (range (start 24 22) (end 24 32)) (probe (position 24 22))
    (reference (id (source (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (named (kind requirement-def) (name "VehicleMassLimitationRequirement")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "massActual")
      (outcome (status resolved) (target (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::MassLimitationRequirement::massActual")))))
    )
  )
  (query (document "memory://snapshot/32_requirement_definitions.md") (range (start 26 22) (end 26 38)) (probe (position 26 22))
    (reference (id (source (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (named (kind requirement-def) (name "VehicleMassLimitationRequirement")) (anonymous (kind assume-constraint) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "vehicle::fuelMass")
      (outcome (status resolved) (target (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::Vehicle::fuelMass")))))
    )
  )
  (query (document "memory://snapshot/32_requirement_definitions.md") (range (start 24 35) (end 24 50)) (probe (position 24 35))
    (reference (id (source (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (named (kind requirement-def) (name "VehicleMassLimitationRequirement")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "vehicle::dryMass")
      (outcome (status resolved) (target (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::Vehicle::dryMass")))))
    )
  )
  (query (document "memory://snapshot/32_requirement_definitions.md") (range (start 24 53) (end 24 69)) (probe (position 24 53))
    (reference (id (source (node (document "memory://snapshot/32_requirement_definitions.md") (path (named (kind package) (name "Requirement Definitions")) (named (kind requirement-def) (name "VehicleMassLimitationRequirement")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1) (authored-target "vehicle::fuelMass")
      (outcome (status resolved) (target (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::Vehicle::fuelMass")))))
    )
  )
  (query (document "memory://snapshot/32_requirement_definitions.md") (range (start 22 20) (end 22 27)) (probe (position 22 20))
    (reference (id (source (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/32_requirement_definitions.md") (qualified-name "Requirement Definitions::Vehicle")))))
    )
  )
)
~~~
