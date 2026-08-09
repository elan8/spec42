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
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwRequirement,KwDef,Ident,OpenCurly,
KwDoc,RegularComment,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwRequire,KwConstraint,OpenCurly,Ident,LtEq,Ident,CloseCurly,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwRequirement,KwDef,OpenAngle,UnrestrictedName,CloseAngle,Ident,ColonGt,Ident,OpenCurly,
KwDoc,RegularComment,
KwSubject,Ident,Colon,Ident,Semicolon,
KwAttribute,KwRedefines,Ident,Eq,Ident,Dot,Ident,Plus,Ident,Dot,Ident,Semicolon,
KwAssume,KwConstraint,OpenCurly,Ident,Dot,Ident,CloseAngle,DecimalValue,OpenSquare,Ident,CloseSquare,CloseCurly,
CloseCurly,
KwPort,KwDef,Ident,Semicolon,
KwAction,KwDef,Ident,Semicolon,
KwRequirement,KwDef,OpenAngle,UnrestrictedName,CloseAngle,Ident,OpenCurly,
KwDoc,RegularComment,
KwSubject,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwRequirement,KwDef,OpenAngle,UnrestrictedName,CloseAngle,Ident,OpenCurly,
KwDoc,RegularComment,
KwSubject,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Requirement Definitions''
    (import_decl private 'ISQ::*')
    (import_decl private 'SI::*')
    (requirement_def 'MassLimitationRequirement'
      (documentation)
      (attribute_usage 'massActual' : 'MassValue')
      (attribute_usage 'massReqd' : 'MassValue')
      (sysml_decl
        (result_expr_member)))
    (part_def 'Vehicle'
      (attribute_usage 'dryMass' : 'MassValue')
      (attribute_usage 'fuelMass' : 'MassValue')
      (attribute_usage 'fuelFullMass' : 'MassValue'))
    (requirement_def 'VehicleMassLimitationRequirement' :> 'MassLimitationRequirement'
      (documentation)
      (sysml_decl 'vehicle' : 'Vehicle')
      (attribute_usage :>> 'massActual' value)
      (sysml_decl
        (result_expr_member)))
    (port_def 'ClutchPort')
    (action_def 'GenerateTorque')
    (requirement_def 'DrivePowerInterface'
      (documentation)
      (sysml_decl 'clutchPort' : 'ClutchPort'))
    (requirement_def 'TorqueGeneration'
      (documentation)
      (sysml_decl 'generateTorque' : 'GenerateTorque'))))
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
# EXPECTED
~~~
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Requirement Definitions"))) (name "Requirement Definitions") (declared-name "Requirement Definitions")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Requirement Definitions::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Requirement Definitions::*#import"))) (name "*") (declared-name "*"))
        (element (kind "port def") (id (node (document "d0") (qualified-name "Requirement Definitions::ClutchPort"))) (name "ClutchPort") (declared-name "ClutchPort")
          (contains
            (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "Requirement Definitions::ClutchPort::~ClutchPort"))) (name "~ClutchPort") (declared-name "~ClutchPort") (effective (featuring-type (node (document "d0") (qualified-name "Requirement Definitions::ClutchPort")))))
          )
        )
        (element (kind "requirement def") (id (node (document "d0") (qualified-name "Requirement Definitions::DrivePowerInterface"))) (name "DrivePowerInterface") (declared-name "DrivePowerInterface")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Requirement Definitions::DrivePowerInterface::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Requirement Definitions::DrivePowerInterface")))))
            (element (kind "subject") (id (node (document "d0") (qualified-name "Requirement Definitions::DrivePowerInterface::clutchPort"))) (name "clutchPort") (declared-name "clutchPort") (effective (featuring-type (node (document "d0") (qualified-name "Requirement Definitions::DrivePowerInterface")))))
          )
        )
        (element (kind "action def") (id (node (document "d0") (qualified-name "Requirement Definitions::GenerateTorque"))) (name "GenerateTorque") (declared-name "GenerateTorque"))
        (element (kind "requirement def") (id (node (document "d0") (qualified-name "Requirement Definitions::MassLimitationRequirement"))) (name "MassLimitationRequirement") (declared-name "MassLimitationRequirement")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Requirement Definitions::MassLimitationRequirement::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Requirement Definitions::MassLimitationRequirement")))))
            (element (kind "require constraint") (id (node (document "d0") (qualified-name "Requirement Definitions::MassLimitationRequirement::_requireConstraint_0"))) (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (effective (featuring-type (node (document "d0") (qualified-name "Requirement Definitions::MassLimitationRequirement")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Requirement Definitions::MassLimitationRequirement::massActual"))) (name "massActual") (declared-name "massActual") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Requirement Definitions::MassLimitationRequirement")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Requirement Definitions::MassLimitationRequirement::massReqd"))) (name "massReqd") (declared-name "massReqd") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Requirement Definitions::MassLimitationRequirement")))))
          )
        )
        (element (kind "requirement def") (id (node (document "d0") (qualified-name "Requirement Definitions::TorqueGeneration"))) (name "TorqueGeneration") (declared-name "TorqueGeneration")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Requirement Definitions::TorqueGeneration::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Requirement Definitions::TorqueGeneration")))))
            (element (kind "subject") (id (node (document "d0") (qualified-name "Requirement Definitions::TorqueGeneration::generateTorque"))) (name "generateTorque") (declared-name "generateTorque") (effective (featuring-type (node (document "d0") (qualified-name "Requirement Definitions::TorqueGeneration")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "Requirement Definitions::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared)
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Requirement Definitions::Vehicle::dryMass"))) (name "dryMass") (declared-name "dryMass") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Requirement Definitions::Vehicle")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Requirement Definitions::Vehicle::fuelFullMass"))) (name "fuelFullMass") (declared-name "fuelFullMass") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Requirement Definitions::Vehicle")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Requirement Definitions::Vehicle::fuelMass"))) (name "fuelMass") (declared-name "fuelMass") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Requirement Definitions::Vehicle")))))
          )
        )
        (element (kind "requirement def") (id (node (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement"))) (name "VehicleMassLimitationRequirement") (declared-name "VehicleMassLimitationRequirement")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement")))))
            (element (kind "require constraint") (id (node (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement::_requireConstraint_0"))) (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (effective (featuring-type (node (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement::massActual"))) (name "massActual") (declared-name "massActual") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement")))))
            (element (kind "subject") (id (node (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement::vehicle"))) (name "vehicle") (declared-name "vehicle") (effective (featuring-type (node (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement")))))
          )
        )
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Requirement Definitions::DrivePowerInterface::_documentation"))) (to (node (document "d0") (qualified-name "Requirement Definitions::DrivePowerInterface"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Requirement Definitions::MassLimitationRequirement::_documentation"))) (to (node (document "d0") (qualified-name "Requirement Definitions::MassLimitationRequirement"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Requirement Definitions::TorqueGeneration::_documentation"))) (to (node (document "d0") (qualified-name "Requirement Definitions::TorqueGeneration"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement::_documentation"))) (to (node (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "Requirement Definitions::ClutchPort::~ClutchPort"))) (to (node (document "d0") (qualified-name "Requirement Definitions::ClutchPort"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement::massActual"))) (to (node (document "d0") (qualified-name "Requirement Definitions::MassLimitationRequirement::massActual"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement"))) (to (node (document "d0") (qualified-name "Requirement Definitions::MassLimitationRequirement"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "Requirement Definitions::DrivePowerInterface"))) (to (node (document "d0") (qualified-name "Requirement Definitions::ClutchPort"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "Requirement Definitions::DrivePowerInterface"))) (to (node (document "d0") (qualified-name "Requirement Definitions::DrivePowerInterface::clutchPort"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "Requirement Definitions::TorqueGeneration"))) (to (node (document "d0") (qualified-name "Requirement Definitions::GenerateTorque"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "Requirement Definitions::TorqueGeneration"))) (to (node (document "d0") (qualified-name "Requirement Definitions::TorqueGeneration::generateTorque"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement"))) (to (node (document "d0") (qualified-name "Requirement Definitions::Vehicle"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement"))) (to (node (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement::vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Requirement Definitions::DrivePowerInterface::clutchPort"))) (to (node (document "d0") (qualified-name "Requirement Definitions::ClutchPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Requirement Definitions::TorqueGeneration::generateTorque"))) (to (node (document "d0") (qualified-name "Requirement Definitions::GenerateTorque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Requirement Definitions::VehicleMassLimitationRequirement::vehicle"))) (to (node (document "d0") (qualified-name "Requirement Definitions::Vehicle"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/training/32_requirement_definitions.md"
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
        (range (start 15 2) (end 15 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 2) (end 16 36))
      )
    )
  )
)
~~~
