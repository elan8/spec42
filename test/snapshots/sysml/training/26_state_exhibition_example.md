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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "26_state_exhibition_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 36))
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
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 8 3) (end 8 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 3) (end 9 37))
      )
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "1cdd722b5d4691a2dcef003cdb8c984555674ceec1318135514aa0861a7be87d") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "State Exhibition Example"))) (kind "package") (name "State Exhibition Example") (declared-name "State Exhibition Example") (range (start (line 0) (character 0)) (end (line 0) (character 267))))
    (element (id (node (document "d0") (qualified-name "State Exhibition Example::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 40))) (parent (node (document "d0") (qualified-name "State Exhibition Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transition Actions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 36))))))
    (element (id (node (document "d0") (qualified-name "State Exhibition Example::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (range (start (line 3) (character 1)) (end (line 3) (character 183))) (parent (node (document "d0") (qualified-name "State Exhibition Example"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 3) (character 16)) (end (line 3) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "State Exhibition Example::vehicle::vehicleController"))) (kind "part") (name "vehicleController") (declared-name "vehicleController") (range (start (line 5) (character 2)) (end (line 5) (character 45))) (parent (node (document "d0") (qualified-name "State Exhibition Example::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "VehicleController") (range (start (line 5) (character 27)) (end (line 5) (character 44)))))))
    (element (id (node (document "d0") (qualified-name "State Exhibition Example::vehicle::vehicleStates"))) (kind "state") (name "vehicleStates") (declared-name "vehicleStates") (range (start (line 7) (character 2)) (end (line 7) (character 101))) (parent (node (document "d0") (qualified-name "State Exhibition Example::vehicle"))))
    (element (id (node (document "d0") (qualified-name "State Exhibition Example::vehicle::vehicleStates::controller"))) (kind "in out parameter") (name "controller") (declared-name "controller") (range (start (line 9) (character 3)) (end (line 9) (character 37))) (parent (node (document "d0") (qualified-name "State Exhibition Example::vehicle::vehicleStates"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "State Exhibition Example::vehicle::vehicleStates::operatingVehicle"))) (kind "in out parameter") (name "operatingVehicle") (declared-name "operatingVehicle") (range (start (line 8) (character 3)) (end (line 8) (character 33))) (parent (node (document "d0") (qualified-name "State Exhibition Example::vehicle::vehicleStates"))) (authored (relationships (typing (reference "") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "State Exhibition Example::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Transition Actions::*") (range (start (line 1) (character 16)) (end (line 1) (character 36))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "State Exhibition Example::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 3) (character 16)) (end (line 3) (character 23))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "State Exhibition Example::vehicle::vehicleController"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleController") (range (start (line 5) (character 27)) (end (line 5) (character 44))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "State Exhibition Example::vehicle::vehicleStates::controller"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "State Exhibition Example::vehicle::vehicleStates::operatingVehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
    (node (node (document "d0") (qualified-name "State Exhibition Example::vehicle::vehicleStates::controller")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "State Exhibition Example::vehicle::vehicleStates::operatingVehicle")) (expression (status "incomplete") (error "expression is incomplete")))
  )
)
~~~
