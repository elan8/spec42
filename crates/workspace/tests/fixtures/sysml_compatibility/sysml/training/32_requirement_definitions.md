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

        attribute massActual : MassValue;
        attribute massReqd : MassValue;

        require constraint {
            = massActual <= massReqd;
        }
    }

    part def Vehicle {
        attribute dryMass : MassValue;
        attribute fuelMass : MassValue;
        attribute fuelFullMass : MassValue;
    }

    requirement def <'1'> VehicleMassLimitationRequirement :> MassLimitationRequirement {
        doc /* The total mass of a vehicle shall be less than or equal to the required mass. */

        subject vehicle : Vehicle;

        attribute redefines massActual = vehicle.dryMass + vehicle.fuelMass;

        assume constraint {
            = vehicle.fuelMass > 0[kg];
        }
    }

    port def ClutchPort;
    action def GenerateTorque;

    requirement def <'2'> DrivePowerInterface {
        doc /* The engine shall transfer its generated torque to the transmission via the clutch interface. */
        subject clutchPort : ClutchPort;
    }

    requirement def <'3'> TorqueGeneration {
        doc /* The engine shall generate torque as a function of RPM as shown in Table 1. */
        subject generateTorque : GenerateTorque;
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
(model
  (namespace
    (package 'Requirement Definitions'
      (namespace_import private -> 'ISQ'[unresolved])
      (namespace_import private -> 'SI'[unresolved])
      (requirement_def 'MassLimitationRequirement'
        (documentation)
        (attribute_usage composite 'massActual' : 'MassValue'[unresolved])
        (attribute_usage composite 'massReqd' : 'MassValue'[unresolved])
        (require_constraint_usage composite
          (result_expr_membership)))
      (part_def 'Vehicle'
        (attribute_usage composite 'dryMass' : 'MassValue'[unresolved])
        (attribute_usage composite 'fuelMass' : 'MassValue'[unresolved])
        (attribute_usage composite 'fuelFullMass' : 'MassValue'[unresolved]))
      (requirement_def 'VehicleMassLimitationRequirement' :> 'Requirement Definitions::MassLimitationRequirement'[requirement_def]
        (documentation)
        (subject_membership in 'vehicle' : 'Requirement Definitions::Vehicle'[part_def])
        (attribute_usage composite :>> 'Requirement Definitions::MassLimitationRequirement::massActual'[attribute_usage]
          (feature_value (=)))
        (assume_constraint_usage composite
          (result_expr_membership)))
      (port_def 'ClutchPort')
      (action_def 'GenerateTorque')
      (requirement_def 'DrivePowerInterface'
        (documentation)
        (subject_membership in 'clutchPort' : 'Requirement Definitions::ClutchPort'[port_def]))
      (requirement_def 'TorqueGeneration'
        (documentation)
        (subject_membership in 'generateTorque' : 'Requirement Definitions::GenerateTorque'[action_def])))))
~~~
