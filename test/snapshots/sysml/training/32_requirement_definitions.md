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
  (document "32_requirement_definitions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 2) (end 7 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 8 2) (end 8 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 2) (end 14 31))
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
        (range (start 15 2) (end 15 32))
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
        (range (start 16 2) (end 16 36))
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
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "299f545d607e2782c3514c4749f92fcc2ceeb065efaca7110d6257f885917024") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Requirement Definitions"))) (kind "package") (name "Requirement Definitions") (declared-name "Requirement Definitions") (range (start (line 0) (character 0)) (end (line 0) (character 1218))))
    (element (id (node (document "d0") (qualified-name "Requirement Definitions::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 23))) (parent (node (document "d0") (qualified-name "Requirement Definitions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 19))))))
    (element (id (node (document "d0") (qualified-name "Requirement Definitions::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 22))) (parent (node (document "d0") (qualified-name "Requirement Definitions"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 18))))))
    (element (id (node (document "d0") (qualified-name "Requirement Definitions::ClutchPort"))) (kind "port def") (name "ClutchPort") (declared-name "ClutchPort") (range (start (line 29) (character 1)) (end (line 29) (character 21))) (parent (node (document "d0") (qualified-name "Requirement Definitions"))))
    (element (id (node (document "d0") (qualified-name "Requirement Definitions::ClutchPort::~ClutchPort"))) (kind "conjugated port definition") (name "~ClutchPort") (declared-name "~ClutchPort") (range (start (line 29) (character 1)) (end (line 29) (character 21))) (parent (node (document "d0") (qualified-name "Requirement Definitions::ClutchPort"))))
    (element (id (node (document "d0") (qualified-name "Requirement Definitions::DrivePowerInterface"))) (kind "requirement def") (name "DrivePowerInterface") (declared-name "DrivePowerInterface") (range (start (line 32) (character 1)) (end (line 32) (character 186))) (parent (node (document "d0") (qualified-name "Requirement Definitions"))) (authored (membership (kind Owning)) (relationships (subject (reference "Requirement Definitions::DrivePowerInterface::clutchPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "Requirement Definitions::DrivePowerInterface::_documentation"))) (kind "documentation") (name "") (range (start (line 32) (character 1)) (end (line 32) (character 186))) (parent (node (document "d0") (qualified-name "Requirement Definitions::DrivePowerInterface"))))
    (element (id (node (document "d0") (qualified-name "Requirement Definitions::DrivePowerInterface::clutchPort"))) (kind "subject") (name "clutchPort") (declared-name "clutchPort") (range (start (line 34) (character 2)) (end (line 34) (character 33))) (parent (node (document "d0") (qualified-name "Requirement Definitions::DrivePowerInterface"))) (authored (relationships (typing (reference "ClutchPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "Requirement Definitions::GenerateTorque"))) (kind "action def") (name "GenerateTorque") (declared-name "GenerateTorque") (range (start (line 30) (character 1)) (end (line 30) (character 27))) (parent (node (document "d0") (qualified-name "Requirement Definitions"))))
    (element (id (node (document "d0") (qualified-name "Requirement Definitions::MassLimitationRequirement"))) (kind "requirement def") (name "MassLimitationRequirement") (declared-name "MassLimitationRequirement") (range (start (line 4) (character 1)) (end (line 4) (character 247))) (parent (node (document "d0") (qualified-name "Requirement Definitions"))))
    (element (id (node (document "d0") (qualified-name "Requirement Definitions::MassLimitationRequirement::_documentation"))) (kind "documentation") (name "") (range (start (line 4) (character 1)) (end (line 4) (character 247))) (parent (node (document "d0") (qualified-name "Requirement Definitions::MassLimitationRequirement"))))
    (element (id (node (document "d0") (qualified-name "Requirement Definitions::MassLimitationRequirement::_requireConstraint_0"))) (kind "require constraint") (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (range (start (line 10) (character 2)) (end (line 10) (character 47))) (parent (node (document "d0") (qualified-name "Requirement Definitions::MassLimitationRequirement"))))
    (element (id (node (document "d0") (qualified-name "Requirement Definitions::MassLimitationRequirement::massActual"))) (kind "attribute") (name "massActual") (declared-name "massActual") (range (start (line 7) (character 2)) (end (line 7) (character 34))) (parent (node (document "d0") (qualified-name "Requirement Definitions::MassLimitationRequirement"))) (authored (relationships (typing (reference "MassValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Requirement Definitions::MassLimitationRequirement::massReqd"))) (kind "attribute") (name "massReqd") (declared-name "massReqd") (range (start (line 8) (character 2)) (end (line 8) (character 32))) (parent (node (document "d0") (qualified-name "Requirement Definitions::MassLimitationRequirement"))) (authored (relationships (typing (reference "MassValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Requirement Definitions::TorqueGeneration"))) (kind "requirement def") (name "TorqueGeneration") (declared-name "TorqueGeneration") (range (start (line 37) (character 1)) (end (line 37) (character 173))) (parent (node (document "d0") (qualified-name "Requirement Definitions"))) (authored (membership (kind Owning)) (relationships (subject (reference "Requirement Definitions::TorqueGeneration::generateTorque") (range none)))))
    (element (id (node (document "d0") (qualified-name "Requirement Definitions::TorqueGeneration::_documentation"))) (kind "documentation") (name "") (range (start (line 37) (character 1)) (end (line 37) (character 173))) (parent (node (document "d0") (qualified-name "Requirement Definitions::TorqueGeneration"))))
    (element (id (node (document "d0") (qualified-name "Requirement Definitions::TorqueGeneration::generateTorque"))) (kind "subject") (name "generateTorque") (declared-name "generateTorque") (range (start (line 39) (character 2)) (end (line 39) (character 41))) (parent (node (document "d0") (qualified-name "Requirement Definitions::TorqueGeneration"))) (authored (relationships (typing (reference "GenerateTorque") (range none)))))
    (element (id (node (document "d0") (qualified-name "Requirement Definitions::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 13) (character 1)) (end (line 13) (character 124))) (parent (node (document "d0") (qualified-name "Requirement Definitions"))))
    (element (id (node (document "d0") (qualified-name "Requirement Definitions::Vehicle::dryMass"))) (kind "attribute") (name "dryMass") (declared-name "dryMass") (range (start (line 14) (character 2)) (end (line 14) (character 31))) (parent (node (document "d0") (qualified-name "Requirement Definitions::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue") (range none)) (typing (reference "MassValue") (range (start (line 14) (character 21)) (end (line 14) (character 30)))))))
    (element (id (node (document "d0") (qualified-name "Requirement Definitions::Vehicle::fuelFullMass"))) (kind "attribute") (name "fuelFullMass") (declared-name "fuelFullMass") (range (start (line 16) (character 2)) (end (line 16) (character 36))) (parent (node (document "d0") (qualified-name "Requirement Definitions::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue") (range none)) (typing (reference "MassValue") (range (start (line 16) (character 26)) (end (line 16) (character 35)))))))
    (element (id (node (document "d0") (qualified-name "Requirement Definitions::Vehicle::fuelMass"))) (kind "attribute") (name "fuelMass") (declared-name "fuelMass") (range (start (line 15) (character 2)) (end (line 15) (character 32))) (parent (node (document "d0") (qualified-name "Requirement Definitions::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue") (range none)) (typing (reference "MassValue") (range (start (line 15) (character 22)) (end (line 15) (character 31)))))))
    (element (id (node (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement"))) (kind "requirement def") (name "VehicleMassLimitationRequirement") (declared-name "VehicleMassLimitationRequirement") (range (start (line 19) (character 1)) (end (line 19) (character 337))) (parent (node (document "d0") (qualified-name "Requirement Definitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "MassLimitationRequirement") (range (start (line 19) (character 59)) (end (line 19) (character 84)))) (subject (reference "Requirement Definitions::VehicleMassLimitationRequirement::vehicle") (range none)))))
    (element (id (node (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement::_documentation"))) (kind "documentation") (name "") (range (start (line 19) (character 1)) (end (line 19) (character 337))) (parent (node (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement"))))
    (element (id (node (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement::_requireConstraint_0"))) (kind "require constraint") (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (range (start (line 26) (character 2)) (end (line 26) (character 48))) (parent (node (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement"))))
    (element (id (node (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement::massActual"))) (kind "attribute") (name "massActual") (declared-name "massActual") (range (start (line 24) (character 2)) (end (line 24) (character 70))) (parent (node (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement"))) (authored (relationships (redefinition (reference "massActual") (range (start (line 24) (character 22)) (end (line 24) (character 32)))))))
    (element (id (node (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement::vehicle"))) (kind "subject") (name "vehicle") (declared-name "vehicle") (range (start (line 22) (character 2)) (end (line 22) (character 28))) (parent (node (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement"))) (authored (relationships (typing (reference "Vehicle") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Requirement Definitions::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (range (start (line 1) (character 16)) (end (line 1) (character 19))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Definitions::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "SI::*") (range (start (line 2) (character 16)) (end (line 2) (character 18))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Definitions::DrivePowerInterface"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "Requirement Definitions::DrivePowerInterface::clutchPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirement Definitions::DrivePowerInterface::clutchPort")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Definitions::DrivePowerInterface::clutchPort"))) (kind featureTyping) (ordinal 0)) (authored-target "ClutchPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirement Definitions::ClutchPort")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Definitions::MassLimitationRequirement::massActual"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Definitions::MassLimitationRequirement::massReqd"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Definitions::TorqueGeneration"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "Requirement Definitions::TorqueGeneration::generateTorque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirement Definitions::TorqueGeneration::generateTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Definitions::TorqueGeneration::generateTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "GenerateTorque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirement Definitions::GenerateTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Definitions::Vehicle::dryMass"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Definitions::Vehicle::dryMass"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (range (start (line 14) (character 21)) (end (line 14) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Definitions::Vehicle::fuelFullMass"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Definitions::Vehicle::fuelFullMass"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (range (start (line 16) (character 26)) (end (line 16) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Definitions::Vehicle::fuelMass"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Definitions::Vehicle::fuelMass"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (range (start (line 15) (character 22)) (end (line 15) (character 31))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement"))) (kind specialization) (ordinal 0)) (authored-target "MassLimitationRequirement") (range (start (line 19) (character 59)) (end (line 19) (character 84))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirement Definitions::MassLimitationRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "Requirement Definitions::VehicleMassLimitationRequirement::vehicle") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement::vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement::massActual"))) (kind redefinition) (ordinal 0)) (authored-target "massActual") (range (start (line 24) (character 22)) (end (line 24) (character 32))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement::massActual")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirement Definitions::Vehicle")))))
  )
  (relationships
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "Requirement Definitions::DrivePowerInterface"))) (target (node (document "d0") (qualified-name "Requirement Definitions::DrivePowerInterface::clutchPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirement Definitions::DrivePowerInterface"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind subject) (source (node (document "d0") (qualified-name "Requirement Definitions::DrivePowerInterface"))) (target (node (document "d0") (qualified-name "Requirement Definitions::ClutchPort"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Requirement Definitions::DrivePowerInterface::clutchPort"))) (target (node (document "d0") (qualified-name "Requirement Definitions::ClutchPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirement Definitions::DrivePowerInterface::clutchPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "Requirement Definitions::TorqueGeneration"))) (target (node (document "d0") (qualified-name "Requirement Definitions::TorqueGeneration::generateTorque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirement Definitions::TorqueGeneration"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind subject) (source (node (document "d0") (qualified-name "Requirement Definitions::TorqueGeneration"))) (target (node (document "d0") (qualified-name "Requirement Definitions::GenerateTorque"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Requirement Definitions::TorqueGeneration::generateTorque"))) (target (node (document "d0") (qualified-name "Requirement Definitions::GenerateTorque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirement Definitions::TorqueGeneration::generateTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement"))) (target (node (document "d0") (qualified-name "Requirement Definitions::MassLimitationRequirement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement"))) (kind specialization) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement"))) (target (node (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement::vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind subject) (source (node (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement"))) (target (node (document "d0") (qualified-name "Requirement Definitions::Vehicle"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement::massActual"))) (target (node (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement::massActual"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement::massActual"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement::vehicle"))) (target (node (document "d0") (qualified-name "Requirement Definitions::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement::vehicle"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Requirement Definitions::MassLimitationRequirement")) (expression (status "incomplete") (error "expression is incomplete")) (analysis (status "incomplete")))
    (node (node (document "d0") (qualified-name "Requirement Definitions::MassLimitationRequirement::_requireConstraint_0")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement")) (expression (status "incomplete") (error "expression is incomplete")) (analysis (status "incomplete")))
    (node (node (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement::_requireConstraint_0")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement::massActual")) (expression (status "incomplete") (error "expression is incomplete")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 2 16) (end 2 18)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "Requirement Definitions::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "SI::*")
        (range (start 2 16) (end 2 18))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 19)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "Requirement Definitions::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ISQ::*")
        (range (start 1 16) (end 1 19))
        (outcome (status unresolved))
      )
    )
    (query (range (start 14 21) (end 14 30)) (probe (position 14 21))
      (reference
        (source (document "d0") (qualified-name "Requirement Definitions::Vehicle::dryMass"))
        (kind featureTyping) (ordinal 1) (authored-target "MassValue")
        (range (start 14 21) (end 14 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 15 22) (end 15 31)) (probe (position 15 22))
      (reference
        (source (document "d0") (qualified-name "Requirement Definitions::Vehicle::fuelMass"))
        (kind featureTyping) (ordinal 1) (authored-target "MassValue")
        (range (start 15 22) (end 15 31))
        (outcome (status unresolved))
      )
    )
    (query (range (start 16 26) (end 16 35)) (probe (position 16 26))
      (reference
        (source (document "d0") (qualified-name "Requirement Definitions::Vehicle::fuelFullMass"))
        (kind featureTyping) (ordinal 1) (authored-target "MassValue")
        (range (start 16 26) (end 16 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 24 22) (end 24 32)) (probe (position 24 22))
      (reference
        (source (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement::massActual"))
        (kind redefinition) (ordinal 0) (authored-target "massActual")
        (range (start 24 22) (end 24 32))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement::massActual") (range (start 24 2) (end 24 70)))
        )
      )
    )
    (query (range (start 19 59) (end 19 84)) (probe (position 19 59))
      (reference
        (source (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement"))
        (kind specialization) (ordinal 0) (authored-target "MassLimitationRequirement")
        (range (start 19 59) (end 19 84))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Requirement Definitions::MassLimitationRequirement") (range (start 4 1) (end 4 247)))
        )
      )
    )
  )
)
~~~
