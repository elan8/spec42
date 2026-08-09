# META
~~~ini
description=SysML Training 28 (Individuals): Individuals and Roles-1
type=file
~~~
# SOURCE
~~~sysml
package 'Individuals and Roles' {
	private import 'Part Definition Example'::*;
	
	part def Wheel;
	
	individual part def Vehicle_1 :> Vehicle {
		part leftFrontWheel : Wheel;
		part rightFrontWheel : Wheel;
	}
	
	individual part def Wheel_1 :> Wheel;
	
	individual part vehicle_1 : Vehicle_1 {
		snapshot part vehicle_1_t0 {
			snapshot leftFrontWheel_t0 : Wheel_1 :>> leftFrontWheel;
		}
		
		then snapshot part vehicle_1_t1 {
			snapshot rightFrontWheel_t1 : Wheel_1 :>> rightFrontWheel;
		}
	}
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwIndividual,KwPart,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwIndividual,KwPart,KwDef,Ident,ColonGt,Ident,Semicolon,
KwIndividual,KwPart,Ident,Colon,Ident,OpenCurly,
KwSnapshot,KwPart,Ident,OpenCurly,
KwSnapshot,Ident,Colon,Ident,ColonGtGt,Ident,Semicolon,
CloseCurly,
KwThen,KwSnapshot,KwPart,Ident,OpenCurly,
KwSnapshot,Ident,Colon,Ident,ColonGtGt,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Individuals and Roles''
    (import_decl private ''Part Definition Example'::*')
    (part_def 'Wheel')
    (part_def individual 'Vehicle_1' :> 'Vehicle'
      (part_usage 'leftFrontWheel' : 'Wheel')
      (part_usage 'rightFrontWheel' : 'Wheel'))
    (part_def individual 'Wheel_1' :> 'Wheel')
    (part_usage individual 'vehicle_1' : 'Vehicle_1'
      (malformed)
      (part_usage 'vehicle_1_t0'
        (portion_usage snapshot 'leftFrontWheel_t0' : 'Wheel_1' :>> 'leftFrontWheel'))
      (source_succession
        (malformed))
      (part_usage 'vehicle_1_t1'
        (portion_usage snapshot 'rightFrontWheel_t1' : 'Wheel_1' :>> 'rightFrontWheel')))))
~~~
# FORMAT
~~~sysml
package 'Individuals and Roles' {
    private import 'Part Definition Example'::*;

    part def Wheel;

    individual part def Vehicle_1 :> Vehicle {
        part leftFrontWheel : Wheel;
        part rightFrontWheel : Wheel;
    }

    individual part def Wheel_1 :> Wheel;

    individual part vehicle_1 : Vehicle_1 {
        snapshot
        part vehicle_1_t0 {
            snapshot leftFrontWheel_t0 : Wheel_1 :>> leftFrontWheel;
        }

        then snapshot
        part vehicle_1_t1 {
            snapshot rightFrontWheel_t1 : Wheel_1 :>> rightFrontWheel;
        }
    }
}
~~~
# EXPECTED
~~~
parse.expected_usage_declaration
parse.expected_usage_declaration
semantic.unresolved_name 'Vehicle'
~~~
# PROBLEMS
~~~
parse.expected_usage_declaration
parse.expected_usage_declaration
semantic.unresolved_name 'Vehicle'
~~~
# SMG
~~~
(model
  (namespace
    (package 'Individuals and Roles'
      (namespace_import private -> 'Part Definition Example'[unresolved])
      (part_def 'Wheel')
      (part_def individual 'Vehicle_1' :> 'Vehicle'[unresolved]
        (part_usage composite 'leftFrontWheel' : 'Individuals and Roles::Wheel'[part_def])
        (part_usage composite 'rightFrontWheel' : 'Individuals and Roles::Wheel'[part_def]))
      (part_def individual 'Wheel_1' :> 'Individuals and Roles::Wheel'[part_def])
      (part_usage individual 'vehicle_1' : 'Individuals and Roles::Vehicle_1'[part_def]
        (not_implemented 'malformed')
        (part_usage composite 'vehicle_1_t0'
          (occurrence_usage composite 'leftFrontWheel_t0' : 'Individuals and Roles::Wheel_1'[part_def] :>> 'Individuals and Roles::Vehicle_1::leftFrontWheel'[part_usage]))
        (source_succession
          (not_implemented 'malformed'))
        (part_usage composite 'vehicle_1_t1'
          (occurrence_usage composite 'rightFrontWheel_t1' : 'Individuals and Roles::Wheel_1'[part_def] :>> 'Individuals and Roles::Vehicle_1::rightFrontWheel'[part_usage]))))))
~~~
