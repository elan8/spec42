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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Requirement Satisfaction"))) (name "Requirement Satisfaction") (declared-name "Requirement Satisfaction")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Requirement Satisfaction::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Requirement Satisfaction::*#import"))) (name "*") (declared-name "*"))
        (element (kind "part") (id (node (document "d0") (qualified-name "Requirement Satisfaction::Vehicle c1 Design Context"))) (name "Vehicle c1 Design Context") (declared-name "Vehicle c1 Design Context") (declared (properties (ordered false))))
        (element (kind "action") (id (node (document "d0") (qualified-name "Requirement Satisfaction::provide power"))) (name "provide power") (declared-name "provide power") (declared)
          (contains
            (element (kind "action") (id (node (document "d0") (qualified-name "Requirement Satisfaction::provide power::generate torque"))) (name "generate torque") (declared-name "generate torque") (declared) (effective (implied-feature-ownership (composite true) (reference false))))
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "Requirement Satisfaction::vehicle_c1"))) (name "vehicle_c1") (declared-name "vehicle_c1") (declared (properties (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Requirement Satisfaction::vehicle_c1::engine_v1"))) (name "engine_v1") (declared-name "engine_v1") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
              (contains
                (element (kind "port") (id (node (document "d0") (qualified-name "Requirement Satisfaction::vehicle_c1::engine_v1::clutchPort"))) (name "clutchPort") (declared-name "clutchPort") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false))))
                (element (kind "action") (id (node (document "d0") (qualified-name "Requirement Satisfaction::vehicle_c1::engine_v1::provide power.generate torque"))) (name "provide power.generate torque") (declared-name "provide power.generate torque"))
              )
            )
            (element (kind "action") (id (node (document "d0") (qualified-name "Requirement Satisfaction::vehicle_c1::provide power"))) (name "provide power") (declared-name "provide power"))
          )
        )
      )
    )
    (element (kind "diagnostic") (id (node (document "d0") (qualified-name "Requirement Satisfaction::Vehicle c1 Design Context::unresolved_satisfy_source"))) (name "unresolved_satisfy_source") (declared-name "unresolved_satisfy_source"))
    (element (kind "diagnostic") (id (node (document "d0") (qualified-name "Requirement Satisfaction::Vehicle c1 Design Context::unresolved_satisfy_source#diagnostic"))) (name "unresolved_satisfy_source") (declared-name "unresolved_satisfy_source"))
  )
  (relationships
    (perform (status resolved) (from (node (document "d0") (qualified-name "Requirement Satisfaction::provide power"))) (to (node (document "d0") (qualified-name "Requirement Satisfaction::provide power::generate torque"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Requirement Satisfaction::vehicle_c1"))) (to (node (document "d0") (qualified-name "Requirement Satisfaction::vehicle_c1::provide power"))))
  )
  (pending-relationships
    (perform (status pending) (document "d0") (source-qualified "Requirement Satisfaction::vehicle_c1::engine_v1") (target-qualified "Requirement Satisfaction::vehicle_c1::engine_v1::provide power::generate torque"))
  )
  (pending-expression-relationships
    (satisfy (status pending-expression) (document "d0") (source-expression "engineSpecification") (target-expression "vehicle_design::engine_v1") (container-prefix "Requirement Satisfaction::Vehicle c1 Design Context"))
    (satisfy (status pending-expression) (document "d0") (source-expression "vehicleSpecification") (target-expression "vehicle_design") (container-prefix "Requirement Satisfaction::Vehicle c1 Design Context"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/training/32_requirement_satisfaction.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unresolved_pending_relationship")
        (source "semantic")
        (range (start 0 0) (end 0 0))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 1) (end 1 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 1) (end 2 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 8 19) (end 8 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 18) (end 11 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 12 3) (end 12 23))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "sysml")
        (range (start 19 2) (end 19 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_satisfy_source")
        (source "semantic")
        (range (start 21 10) (end 21 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_satisfy_source")
        (source "semantic")
        (range (start 22 10) (end 22 29))
      )
    )
  )
)
~~~
