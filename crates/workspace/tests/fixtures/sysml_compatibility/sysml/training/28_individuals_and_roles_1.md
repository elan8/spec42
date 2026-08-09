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
        snapshot part vehicle_1_t0 {
            snapshot leftFrontWheel_t0 : Wheel_1 :>> leftFrontWheel;
        }

        then snapshot part vehicle_1_t1 {
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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Individuals and Roles"))) (name "Individuals and Roles") (declared-name "Individuals and Roles")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Individuals and Roles::*"))) (name "*") (declared-name "*"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Individuals and Roles::Vehicle_1"))) (name "Vehicle_1") (declared-name "Vehicle_1") (declared (properties (individual true)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Individuals and Roles::Vehicle_1::leftFrontWheel"))) (name "leftFrontWheel") (declared-name "leftFrontWheel") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Individuals and Roles::Vehicle_1")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "Individuals and Roles::Vehicle_1::rightFrontWheel"))) (name "rightFrontWheel") (declared-name "rightFrontWheel") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Individuals and Roles::Vehicle_1")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "Individuals and Roles::Wheel"))) (name "Wheel") (declared-name "Wheel") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Individuals and Roles::Wheel_1"))) (name "Wheel_1") (declared-name "Wheel_1") (declared (properties (individual true))))
        (element (kind "part") (id (node (document "d0") (qualified-name "Individuals and Roles::vehicle_1"))) (name "vehicle_1") (declared-name "vehicle_1") (declared (properties (individual true) (composite true) (reference false) (ordered false))))
      )
    )
  )
  (relationships
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Individuals and Roles::Wheel_1"))) (to (node (document "d0") (qualified-name "Individuals and Roles::Wheel"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Individuals and Roles::Vehicle_1::leftFrontWheel"))) (to (node (document "d0") (qualified-name "Individuals and Roles::Wheel"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Individuals and Roles::Vehicle_1::rightFrontWheel"))) (to (node (document "d0") (qualified-name "Individuals and Roles::Wheel"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Individuals and Roles::vehicle_1"))) (to (node (document "d0") (qualified-name "Individuals and Roles::Vehicle_1"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
