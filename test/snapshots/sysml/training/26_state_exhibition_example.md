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
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "ca8f9d15577b769188afa95a851809e919c7c5aa8eff5c159d4cbc94a62faae6") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "State Exhibition Example"))) (kind "package") (name "State Exhibition Example") (declared-name "State Exhibition Example"))
    (element (id (node (document "d0") (qualified-name "State Exhibition Example::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "State Exhibition Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "Transition Actions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "State Exhibition Example::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "State Exhibition Example"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "State Exhibition Example::vehicle::vehicleController"))) (kind "part") (name "vehicleController") (declared-name "vehicleController") (parent (node (document "d0") (qualified-name "State Exhibition Example::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "VehicleController")))))
    (element (id (node (document "d0") (qualified-name "State Exhibition Example::vehicle::vehicleStates"))) (kind "state") (name "vehicleStates") (declared-name "vehicleStates") (parent (node (document "d0") (qualified-name "State Exhibition Example::vehicle"))))
    (element (id (node (document "d0") (qualified-name "State Exhibition Example::vehicle::vehicleStates::controller"))) (kind "in out parameter") (name "controller") (declared-name "controller") (parent (node (document "d0") (qualified-name "State Exhibition Example::vehicle::vehicleStates"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "State Exhibition Example::vehicle::vehicleStates::operatingVehicle"))) (kind "in out parameter") (name "operatingVehicle") (declared-name "operatingVehicle") (parent (node (document "d0") (qualified-name "State Exhibition Example::vehicle::vehicleStates"))) (authored (relationships (typing (reference "")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "State Exhibition Example::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Transition Actions::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "State Exhibition Example::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "State Exhibition Example::vehicle::vehicleController"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleController") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "State Exhibition Example::vehicle::vehicleStates::controller"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "State Exhibition Example::vehicle::vehicleStates::operatingVehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
    (node (node (document "d0") (qualified-name "State Exhibition Example::vehicle::vehicleStates::controller")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "State Exhibition Example::vehicle::vehicleStates::operatingVehicle")) (expression (status "incomplete") (error "expression is incomplete")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 3 16) (end 3 23)) (probe (position 3 16))
      (reference
        (source (document "d0") (qualified-name "State Exhibition Example::vehicle"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
        (range (start 3 16) (end 3 23))
        (outcome (status unresolved))
      )
    )
    (query (range (start 5 27) (end 5 44)) (probe (position 5 27))
      (reference
        (source (document "d0") (qualified-name "State Exhibition Example::vehicle::vehicleController"))
        (kind featureTyping) (ordinal 0) (authored-target "VehicleController")
        (range (start 5 27) (end 5 44))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 36)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "State Exhibition Example::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Transition Actions::*")
        (range (start 1 16) (end 1 36))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
