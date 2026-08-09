# META
~~~ini
description=SysML Training 32 (Requirements): Requirement Satisfaction
type=file
~~~
# SOURCE
~~~sysml
package 'Requirement Satisfaction' {
	private import 'Requirement Definitions'::*;
	private import 'Requirement Groups'::*;
	
	action 'provide power' {
		action 'generate torque' { }
	}
	
	part vehicle_c1 : Vehicle {
		perform 'provide power';
			
		part engine_v1: Engine {
			port :>> clutchPort;
			perform 'provide power'.'generate torque' :>> generateTorque;
		}	
	}
	
	part 'Vehicle c1 Design Context' {
		
		ref vehicle_design :> vehicle_c1;
	
		satisfy vehicleSpecification by vehicle_design;
		satisfy engineSpecification by vehicle_design.engine_v1;
	
	}
	
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwAction,UnrestrictedName,OpenCurly,
KwAction,UnrestrictedName,OpenCurly,CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPerform,UnrestrictedName,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPort,ColonGtGt,Ident,Semicolon,
KwPerform,UnrestrictedName,Dot,UnrestrictedName,ColonGtGt,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPart,UnrestrictedName,OpenCurly,
KwRef,Ident,ColonGt,Ident,Semicolon,
KwSatisfy,Ident,KwBy,Ident,Semicolon,
KwSatisfy,Ident,KwBy,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Requirement Satisfaction''
    (import_decl private ''Requirement Definitions'::*')
    (import_decl private ''Requirement Groups'::*')
    (action_usage ''provide power''
      (action_usage ''generate torque''))
    (part_usage 'vehicle_c1' : 'Vehicle'
      (perform_action :>> ''provide power'')
      (part_usage 'engine_v1' : 'Engine'
        (port_usage :>> 'clutchPort')
        (perform_action :>> ''provide power'.'generate torque'')
        (default_ref_usage :>> 'generateTorque')))
    (part_usage ''Vehicle c1 Design Context''
      (ref_usage ref 'vehicle_design' :> 'vehicle_c1')
      (sysml_decl 'vehicleSpecification')
      (sysml_decl 'engineSpecification'))))
~~~
# FORMAT
~~~sysml
package 'Requirement Satisfaction' {
    private import 'Requirement Definitions'::*;
    private import 'Requirement Groups'::*;

    action 'provide power' {
        action 'generate torque' { }
    }

    part vehicle_c1 : Vehicle {
        perform :>> 'provide power';

        part engine_v1 : Engine {
            port :>> clutchPort;
            perform :>> 'provide power'.'generate torque';
            :>> generateTorque;
        }
    }

    part 'Vehicle c1 Design Context' {
        ref vehicle_design :> vehicle_c1;

        satisfy vehicleSpecification by vehicle_design;
        satisfy engineSpecification by vehicle_design.engine_v1;
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'Engine'
semantic.unresolved_name 'clutchPort'
semantic.unresolved_name 'generateTorque'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'Engine'
semantic.unresolved_name 'clutchPort'
semantic.unresolved_name 'generateTorque'
~~~
# SMG
~~~
(model
  (namespace
    (package 'Requirement Satisfaction'
      (namespace_import private -> 'Requirement Definitions'[unresolved])
      (namespace_import private -> 'Requirement Groups'[unresolved])
      (action_usage 'provide power'
        (action_usage composite 'generate torque'))
      (part_usage 'vehicle_c1' : 'Vehicle'[unresolved]
        (perform_action_usage :>> 'Requirement Satisfaction::provide power'[action_usage])
        (part_usage composite 'engine_v1' : 'Engine'[unresolved]
          (port_usage composite :>> 'clutchPort'[unresolved])
          (perform_action_usage :>> 'Requirement Satisfaction::provide power::generate torque'[action_usage])
          (reference_usage reference :>> 'generateTorque'[unresolved])))
      (part_usage 'Vehicle c1 Design Context'
        (reference_usage reference 'vehicle_design' :> 'Requirement Satisfaction::vehicle_c1'[part_usage])
        (satisfy_requirement_usage 'vehicleSpecification' by 'Requirement Satisfaction::Vehicle c1 Design Context::vehicle_design'[reference_usage])
        (satisfy_requirement_usage 'engineSpecification' by 'Requirement Satisfaction::vehicle_c1::engine_v1'[part_usage])))))
~~~
