# META
~~~ini
description=SysML Training 32 (Requirements): Requirement Groups
type=file
~~~
# SOURCE
~~~sysml
package 'Requirement Groups' {
	private import 'Requirement Definitions'::*;
	private import 'Requirement Usages'::*;
	
	part def Engine {
		port clutchPort: ClutchPort;
		perform action generateTorque: GenerateTorque;
	}
	
	requirement vehicleSpecification {
		doc /* Overall vehicle requirements group */
		
		subject vehicle : Vehicle;
		
		require fullVehicleMassLimit;
		require emptyVehicleMassLimit;
	}
	
	requirement engineSpecification {
		doc /* Engine power requirements group */
		
		subject engine : Engine;
		
		requirement drivePowerInterface : DrivePowerInterface {
			subject = engine.clutchPort;
		}
		
		requirement torqueGeneration : TorqueGeneration {
			subject = engine.generateTorque;	
		}
	}
	
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwPort,Ident,Colon,Ident,Semicolon,
KwPerform,KwAction,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwRequirement,Ident,OpenCurly,
KwDoc,RegularComment,
KwSubject,Ident,Colon,Ident,Semicolon,
KwRequire,Ident,Semicolon,
KwRequire,Ident,Semicolon,
CloseCurly,
KwRequirement,Ident,OpenCurly,
KwDoc,RegularComment,
KwSubject,Ident,Colon,Ident,Semicolon,
KwRequirement,Ident,Colon,Ident,OpenCurly,
KwSubject,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwRequirement,Ident,Colon,Ident,OpenCurly,
KwSubject,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Requirement Groups''
    (import_decl private ''Requirement Definitions'::*')
    (import_decl private ''Requirement Usages'::*')
    (part_def 'Engine'
      (port_usage 'clutchPort' : 'ClutchPort')
      (perform_action 'generateTorque' : 'GenerateTorque'))
    (requirement_usage 'vehicleSpecification'
      (documentation)
      (sysml_decl 'vehicle' : 'Vehicle')
      (sysml_decl 'fullVehicleMassLimit')
      (sysml_decl 'emptyVehicleMassLimit'))
    (requirement_usage 'engineSpecification'
      (documentation)
      (sysml_decl 'engine' : 'Engine')
      (requirement_usage 'drivePowerInterface' : 'DrivePowerInterface'
        (sysml_decl value))
      (requirement_usage 'torqueGeneration' : 'TorqueGeneration'
        (sysml_decl value)))))
~~~
# FORMAT
~~~sysml
package 'Requirement Groups' {
    private import 'Requirement Definitions'::*;
    private import 'Requirement Usages'::*;

    part def Engine {
        port clutchPort : ClutchPort;
        perform action generateTorque : GenerateTorque;
    }

    requirement vehicleSpecification {
        doc /* Overall vehicle requirements group */

        subject vehicle : Vehicle;

        require constraint fullVehicleMassLimit;
        require constraint emptyVehicleMassLimit;
    }

    requirement engineSpecification {
        doc /* Engine power requirements group */

        subject engine : Engine;

        requirement drivePowerInterface : DrivePowerInterface {
            subject = engine.clutchPort;
        }

        requirement torqueGeneration : TorqueGeneration {
            subject = engine.generateTorque;
        }
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'ClutchPort'
semantic.unresolved_name 'GenerateTorque'
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'DrivePowerInterface'
semantic.unresolved_name 'TorqueGeneration'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'ClutchPort'
semantic.unresolved_name 'GenerateTorque'
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'DrivePowerInterface'
semantic.unresolved_name 'TorqueGeneration'
~~~
# SMG
~~~
(model
  (namespace
    (package 'Requirement Groups'
      (namespace_import private -> 'Requirement Definitions'[unresolved])
      (namespace_import private -> 'Requirement Usages'[unresolved])
      (part_def 'Engine'
        (port_usage composite 'clutchPort' : 'ClutchPort'[unresolved])
        (perform_action_usage 'generateTorque' : 'GenerateTorque'[unresolved]))
      (requirement_usage 'vehicleSpecification'
        (documentation)
        (subject_membership in 'vehicle' : 'Vehicle'[unresolved])
        (require_constraint_usage composite 'fullVehicleMassLimit')
        (require_constraint_usage composite 'emptyVehicleMassLimit'))
      (requirement_usage 'engineSpecification'
        (documentation)
        (subject_membership in 'engine' : 'Requirement Groups::Engine'[part_def])
        (requirement_usage composite 'drivePowerInterface' : 'DrivePowerInterface'[unresolved]
          (subject_membership in
            (feature_value (=))))
        (requirement_usage composite 'torqueGeneration' : 'TorqueGeneration'[unresolved]
          (subject_membership in
            (feature_value (=))))))))
~~~
