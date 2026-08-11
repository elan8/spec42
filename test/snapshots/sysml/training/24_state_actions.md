# META
~~~ini
description=SysML Training 24 (States): State Actions
type=file
~~~
# SOURCE
~~~sysml
package 'State Actions' {
	
	attribute def VehicleStartSignal;
	attribute def VehicleOnSignal;
	attribute def VehicleOffSignal;
	
	part def Vehicle;
	
	action performSelfTest { in vehicle : Vehicle; }
	
	state def VehicleStates { in operatingVehicle : Vehicle; }
		
	state vehicleStates : VehicleStates {
		in operatingVehicle : Vehicle;
			
		entry; then off;
		
		state off;
		accept VehicleStartSignal 
			then starting;
			
		state starting;
		accept VehicleOnSignal
			then on;
			
		state on {
			entry performSelfTest{ in vehicle = operatingVehicle; }
			do action providePower { /* ... */ }
			exit action applyParkingBrake { /* ... */ }
		}
		accept VehicleOffSignal
			then off;
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "24_state_actions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 26 26) (end 26 56))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'State Actions' {

    attribute def VehicleStartSignal;
    attribute def VehicleOnSignal;
    attribute def VehicleOffSignal;

    part def Vehicle;

    action performSelfTest { in vehicle : Vehicle; }

    state def VehicleStates { in operatingVehicle : Vehicle; }

    state vehicleStates : VehicleStates {
        in operatingVehicle : Vehicle;

        entry; then off;

        state off;
        accept VehicleStartSignal
        then starting;

        state starting;
        accept VehicleOnSignal
        then on;

        state on {
            entry performSelfTest{ in vehicle = operatingVehicle; }
            do action providePower { /* ... */ }
            exit action applyParkingBrake { /* ... */ }
        }
        accept VehicleOffSignal
        then off;
    }

}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "83e0d86056f6d4fdc2440dc8860f3ae18a7a8b88981d7c92a9563f82a348da01") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "State Actions"))) (kind "package") (name "State Actions") (declared-name "State Actions"))
    (element (id (node (document "d0") (qualified-name "State Actions::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "State Actions"))))
    (element (id (node (document "d0") (qualified-name "State Actions::VehicleOffSignal"))) (kind "attribute def") (name "VehicleOffSignal") (declared-name "VehicleOffSignal") (parent (node (document "d0") (qualified-name "State Actions"))))
    (element (id (node (document "d0") (qualified-name "State Actions::VehicleOnSignal"))) (kind "attribute def") (name "VehicleOnSignal") (declared-name "VehicleOnSignal") (parent (node (document "d0") (qualified-name "State Actions"))))
    (element (id (node (document "d0") (qualified-name "State Actions::VehicleStartSignal"))) (kind "attribute def") (name "VehicleStartSignal") (declared-name "VehicleStartSignal") (parent (node (document "d0") (qualified-name "State Actions"))))
    (element (id (node (document "d0") (qualified-name "State Actions::VehicleStates"))) (kind "state def") (name "VehicleStates") (declared-name "VehicleStates") (parent (node (document "d0") (qualified-name "State Actions"))))
    (element (id (node (document "d0") (qualified-name "State Actions::VehicleStates::operatingVehicle"))) (kind "in out parameter") (name "operatingVehicle") (declared-name "operatingVehicle") (parent (node (document "d0") (qualified-name "State Actions::VehicleStates"))) (authored (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "State Actions::performSelfTest"))) (kind "action") (name "performSelfTest") (declared-name "performSelfTest") (parent (node (document "d0") (qualified-name "State Actions"))))
    (element (id (node (document "d0") (qualified-name "State Actions::performSelfTest::vehicle"))) (kind "in out parameter") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "State Actions::performSelfTest"))) (authored (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "State Actions::vehicleStates"))) (kind "state") (name "vehicleStates") (declared-name "vehicleStates") (parent (node (document "d0") (qualified-name "State Actions"))) (authored (membership (kind Feature)) (relationships (typing (reference "VehicleStates")) (transition (reference "State Actions::vehicleStates::starting")) (transition (reference "State Actions::vehicleStates::on")) (transition (reference "State Actions::vehicleStates::off")) (initial-state (reference "State Actions::vehicleStates::off")))))
    (element (id (node (document "d0") (qualified-name "State Actions::vehicleStates::_entry"))) (kind "action") (name "entry") (declared-name "entry") (parent (node (document "d0") (qualified-name "State Actions::vehicleStates"))))
    (element (id (node (document "d0") (qualified-name "State Actions::vehicleStates::off"))) (kind "state") (name "off") (declared-name "off") (parent (node (document "d0") (qualified-name "State Actions::vehicleStates"))))
    (element (id (node (document "d0") (qualified-name "State Actions::vehicleStates::on"))) (kind "state") (name "on") (declared-name "on") (parent (node (document "d0") (qualified-name "State Actions::vehicleStates"))))
    (element (id (node (document "d0") (qualified-name "State Actions::vehicleStates::on::_do"))) (kind "action") (name "do") (declared-name "do") (parent (node (document "d0") (qualified-name "State Actions::vehicleStates::on"))))
    (element (id (node (document "d0") (qualified-name "State Actions::vehicleStates::on::_entry"))) (kind "action") (name "entry") (declared-name "entry") (parent (node (document "d0") (qualified-name "State Actions::vehicleStates::on"))))
    (element (id (node (document "d0") (qualified-name "State Actions::vehicleStates::on::_entry::vehicle"))) (kind "in out parameter") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "State Actions::vehicleStates::on::_entry"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "State Actions::vehicleStates::on::_exit"))) (kind "action") (name "exit") (declared-name "exit") (parent (node (document "d0") (qualified-name "State Actions::vehicleStates::on"))))
    (element (id (node (document "d0") (qualified-name "State Actions::vehicleStates::operatingVehicle"))) (kind "in out parameter") (name "operatingVehicle") (declared-name "operatingVehicle") (parent (node (document "d0") (qualified-name "State Actions::vehicleStates"))) (authored (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "State Actions::vehicleStates::starting"))) (kind "state") (name "starting") (declared-name "starting") (parent (node (document "d0") (qualified-name "State Actions::vehicleStates"))))
    (element (id (node (document "d0") (qualified-name "State Actions::vehicleStates::transition_vehicleStates_to_off"))) (kind "transition") (name "transition_vehicleStates_to_off") (declared-name "transition_vehicleStates_to_off") (parent (node (document "d0") (qualified-name "State Actions::vehicleStates"))))
    (element (id (node (document "d0") (qualified-name "State Actions::vehicleStates::transition_vehicleStates_to_off::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "State Actions::vehicleStates::transition_vehicleStates_to_off"))))
    (element (id (node (document "d0") (qualified-name "State Actions::vehicleStates::transition_vehicleStates_to_on"))) (kind "transition") (name "transition_vehicleStates_to_on") (declared-name "transition_vehicleStates_to_on") (parent (node (document "d0") (qualified-name "State Actions::vehicleStates"))))
    (element (id (node (document "d0") (qualified-name "State Actions::vehicleStates::transition_vehicleStates_to_on::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "State Actions::vehicleStates::transition_vehicleStates_to_on"))))
    (element (id (node (document "d0") (qualified-name "State Actions::vehicleStates::transition_vehicleStates_to_starting"))) (kind "transition") (name "transition_vehicleStates_to_starting") (declared-name "transition_vehicleStates_to_starting") (parent (node (document "d0") (qualified-name "State Actions::vehicleStates"))))
    (element (id (node (document "d0") (qualified-name "State Actions::vehicleStates::transition_vehicleStates_to_starting::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "State Actions::vehicleStates::transition_vehicleStates_to_starting"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "State Actions::VehicleStates::operatingVehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "State Actions::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "State Actions::performSelfTest::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "State Actions::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "State Actions::vehicleStates"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleStates") (outcome (status resolved) (target (node (document "d0") (qualified-name "State Actions::VehicleStates")))))
    (reference (id (source (node (document "d0") (qualified-name "State Actions::vehicleStates"))) (kind transitionSource) (ordinal 0)) (authored-target "State Actions::vehicleStates::starting") (outcome (status resolved) (target (node (document "d0") (qualified-name "State Actions::vehicleStates::starting")))))
    (reference (id (source (node (document "d0") (qualified-name "State Actions::vehicleStates"))) (kind transitionSource) (ordinal 1)) (authored-target "State Actions::vehicleStates::on") (outcome (status resolved) (target (node (document "d0") (qualified-name "State Actions::vehicleStates::on")))))
    (reference (id (source (node (document "d0") (qualified-name "State Actions::vehicleStates"))) (kind transitionSource) (ordinal 2)) (authored-target "State Actions::vehicleStates::off") (outcome (status resolved) (target (node (document "d0") (qualified-name "State Actions::vehicleStates::off")))))
    (reference (id (source (node (document "d0") (qualified-name "State Actions::vehicleStates"))) (kind initialStateSource) (ordinal 0)) (authored-target "State Actions::vehicleStates::off") (outcome (status resolved) (target (node (document "d0") (qualified-name "State Actions::vehicleStates::off")))))
    (reference (id (source (node (document "d0") (qualified-name "State Actions::vehicleStates::on::_entry::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "State Actions::vehicleStates::operatingVehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "State Actions::Vehicle")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "State Actions::VehicleStates::operatingVehicle"))) (target (node (document "d0") (qualified-name "State Actions::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "State Actions::VehicleStates::operatingVehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "State Actions::performSelfTest::vehicle"))) (target (node (document "d0") (qualified-name "State Actions::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "State Actions::performSelfTest::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "State Actions::vehicleStates"))) (target (node (document "d0") (qualified-name "State Actions::VehicleStates"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "State Actions::vehicleStates"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "State Actions::vehicleStates"))) (target (node (document "d0") (qualified-name "State Actions::vehicleStates::off"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "State Actions::vehicleStates"))) (kind transitionSource) (ordinal 2)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "State Actions::vehicleStates"))) (target (node (document "d0") (qualified-name "State Actions::vehicleStates::on"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "State Actions::vehicleStates"))) (kind transitionSource) (ordinal 1)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "State Actions::vehicleStates"))) (target (node (document "d0") (qualified-name "State Actions::vehicleStates::starting"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "State Actions::vehicleStates"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind initialState) (source (node (document "d0") (qualified-name "State Actions::vehicleStates"))) (target (node (document "d0") (qualified-name "State Actions::vehicleStates::off"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "State Actions::vehicleStates"))) (kind initialStateSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "State Actions::vehicleStates::operatingVehicle"))) (target (node (document "d0") (qualified-name "State Actions::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "State Actions::vehicleStates::operatingVehicle"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "State Actions::vehicleStates::on::_entry::vehicle")) (expression (status "incomplete") (error "expression is incomplete")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
