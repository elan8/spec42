# META
~~~ini
description=SysML Training 26 (State Exhibition): State Exhibition Example
type=file
~~~
# SOURCE
~~~sysml
package 'State Exhibition Example' {
	private import 'Transition Actions'::*;
	
	part vehicle : Vehicle {
		
		part vehicleController : VehicleController;
		
		exhibit vehicleStates {
			in operatingVehicle = vehicle;
			in controller = vehicleController;
		}

	}
	
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,Semicolon,
KwExhibit,Ident,OpenCurly,
KwIn,Ident,Eq,Ident,Semicolon,
KwIn,Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''State Exhibition Example''
    (import_decl private ''Transition Actions'::*')
    (part_usage 'vehicle' : 'Vehicle'
      (part_usage 'vehicleController' : 'VehicleController')
      (exhibit_state 'vehicleStates'
        (default_ref_usage in 'operatingVehicle' value)
        (default_ref_usage in 'controller' value)))))
~~~
# FORMAT
~~~sysml
package 'State Exhibition Example' {
    private import 'Transition Actions'::*;

    part vehicle : Vehicle {

        part vehicleController : VehicleController;

        exhibit vehicleStates {
            in operatingVehicle = vehicle;
            in controller = vehicleController;
        }

    }

}

~~~
# EXPECTED
~~~
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'VehicleController'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'VehicleController'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "State Exhibition Example"))) (name "State Exhibition Example") (declared-name "State Exhibition Example")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "State Exhibition Example::*"))) (name "*") (declared-name "*"))
        (element (kind "part") (id (node (document "d0") (qualified-name "State Exhibition Example::vehicle"))) (name "vehicle") (declared-name "vehicle") (declared (properties (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "State Exhibition Example::vehicle::vehicleController"))) (name "vehicleController") (declared-name "vehicleController") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
            (element (kind "state") (id (node (document "d0") (qualified-name "State Exhibition Example::vehicle::vehicleStates"))) (name "vehicleStates") (declared-name "vehicleStates") (declared)
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "State Exhibition Example::vehicle::vehicleStates::controller"))) (name "controller") (declared-name "controller"))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "State Exhibition Example::vehicle::vehicleStates::operatingVehicle"))) (name "operatingVehicle") (declared-name "operatingVehicle"))
              )
            )
          )
        )
      )
    )
  )
  (relationships
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
  (document "sysml/training/26_state_exhibition_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 1) (end 1 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 3 16) (end 3 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 27) (end 5 44))
      )
    )
  )
)
~~~
