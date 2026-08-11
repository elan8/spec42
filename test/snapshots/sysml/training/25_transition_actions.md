# META
~~~ini
description=SysML Training 25 (Transitions): Transition Actions
type=file
~~~
# SOURCE
~~~sysml
package 'Transition Actions' {
	
	attribute def VehicleStartSignal;
	attribute def VehicleOnSignal;
	attribute def VehicleOffSignal;
	
	attribute def ControllerStartSignal;
	
	part def Vehicle {
		brakePedalDepressed : ScalarValues::Boolean;
	}
	part def VehicleController;
	
	action performSelfTest { in vehicle : Vehicle; }
	
	state def VehicleStates;
		
	state vehicleStates : VehicleStates {
		in operatingVehicle : Vehicle;
		in controller : VehicleController;

		entry; then off;
		
		state off;
		accept VehicleStartSignal 
			then starting;
			
		state starting;
		accept VehicleOnSignal
			if operatingVehicle.brakePedalDepressed
			do send new ControllerStartSignal() to controller
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
  (document "25_transition_actions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 34 26) (end 34 56))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "3522b43ef6a5749334eb09326fd6ce6985906fcd4abe5a4baa71ba1f03f1b21b") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Transition Actions"))) (kind "package") (name "Transition Actions") (declared-name "Transition Actions"))
    (element (id (node (document "d0") (qualified-name "Transition Actions::ControllerStartSignal"))) (kind "attribute def") (name "ControllerStartSignal") (declared-name "ControllerStartSignal") (parent (node (document "d0") (qualified-name "Transition Actions"))))
    (element (id (node (document "d0") (qualified-name "Transition Actions::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "Transition Actions"))))
    (element (id (node (document "d0") (qualified-name "Transition Actions::VehicleController"))) (kind "part def") (name "VehicleController") (declared-name "VehicleController") (parent (node (document "d0") (qualified-name "Transition Actions"))))
    (element (id (node (document "d0") (qualified-name "Transition Actions::VehicleOffSignal"))) (kind "attribute def") (name "VehicleOffSignal") (declared-name "VehicleOffSignal") (parent (node (document "d0") (qualified-name "Transition Actions"))))
    (element (id (node (document "d0") (qualified-name "Transition Actions::VehicleOnSignal"))) (kind "attribute def") (name "VehicleOnSignal") (declared-name "VehicleOnSignal") (parent (node (document "d0") (qualified-name "Transition Actions"))))
    (element (id (node (document "d0") (qualified-name "Transition Actions::VehicleStartSignal"))) (kind "attribute def") (name "VehicleStartSignal") (declared-name "VehicleStartSignal") (parent (node (document "d0") (qualified-name "Transition Actions"))))
    (element (id (node (document "d0") (qualified-name "Transition Actions::VehicleStates"))) (kind "state def") (name "VehicleStates") (declared-name "VehicleStates") (parent (node (document "d0") (qualified-name "Transition Actions"))))
    (element (id (node (document "d0") (qualified-name "Transition Actions::performSelfTest"))) (kind "action") (name "performSelfTest") (declared-name "performSelfTest") (parent (node (document "d0") (qualified-name "Transition Actions"))))
    (element (id (node (document "d0") (qualified-name "Transition Actions::performSelfTest::vehicle"))) (kind "in out parameter") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "Transition Actions::performSelfTest"))) (authored (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "Transition Actions::vehicleStates"))) (kind "state") (name "vehicleStates") (declared-name "vehicleStates") (parent (node (document "d0") (qualified-name "Transition Actions"))) (authored (membership (kind Feature)) (relationships (typing (reference "VehicleStates")) (transition (reference "Transition Actions::vehicleStates::starting")) (transition (reference "Transition Actions::vehicleStates::on")) (transition (reference "Transition Actions::vehicleStates::off")) (initial-state (reference "Transition Actions::vehicleStates::off")))))
    (element (id (node (document "d0") (qualified-name "Transition Actions::vehicleStates::_entry"))) (kind "action") (name "entry") (declared-name "entry") (parent (node (document "d0") (qualified-name "Transition Actions::vehicleStates"))))
    (element (id (node (document "d0") (qualified-name "Transition Actions::vehicleStates::controller"))) (kind "in out parameter") (name "controller") (declared-name "controller") (parent (node (document "d0") (qualified-name "Transition Actions::vehicleStates"))) (authored (relationships (typing (reference "VehicleController")))))
    (element (id (node (document "d0") (qualified-name "Transition Actions::vehicleStates::off"))) (kind "state") (name "off") (declared-name "off") (parent (node (document "d0") (qualified-name "Transition Actions::vehicleStates"))))
    (element (id (node (document "d0") (qualified-name "Transition Actions::vehicleStates::on"))) (kind "state") (name "on") (declared-name "on") (parent (node (document "d0") (qualified-name "Transition Actions::vehicleStates"))))
    (element (id (node (document "d0") (qualified-name "Transition Actions::vehicleStates::on::_do"))) (kind "action") (name "do") (declared-name "do") (parent (node (document "d0") (qualified-name "Transition Actions::vehicleStates::on"))))
    (element (id (node (document "d0") (qualified-name "Transition Actions::vehicleStates::on::_entry"))) (kind "action") (name "entry") (declared-name "entry") (parent (node (document "d0") (qualified-name "Transition Actions::vehicleStates::on"))))
    (element (id (node (document "d0") (qualified-name "Transition Actions::vehicleStates::on::_entry::vehicle"))) (kind "in out parameter") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "Transition Actions::vehicleStates::on::_entry"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Transition Actions::vehicleStates::on::_exit"))) (kind "action") (name "exit") (declared-name "exit") (parent (node (document "d0") (qualified-name "Transition Actions::vehicleStates::on"))))
    (element (id (node (document "d0") (qualified-name "Transition Actions::vehicleStates::operatingVehicle"))) (kind "in out parameter") (name "operatingVehicle") (declared-name "operatingVehicle") (parent (node (document "d0") (qualified-name "Transition Actions::vehicleStates"))) (authored (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "Transition Actions::vehicleStates::starting"))) (kind "state") (name "starting") (declared-name "starting") (parent (node (document "d0") (qualified-name "Transition Actions::vehicleStates"))))
    (element (id (node (document "d0") (qualified-name "Transition Actions::vehicleStates::transition_vehicleStates_to_off"))) (kind "transition") (name "transition_vehicleStates_to_off") (declared-name "transition_vehicleStates_to_off") (parent (node (document "d0") (qualified-name "Transition Actions::vehicleStates"))))
    (element (id (node (document "d0") (qualified-name "Transition Actions::vehicleStates::transition_vehicleStates_to_off::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "Transition Actions::vehicleStates::transition_vehicleStates_to_off"))))
    (element (id (node (document "d0") (qualified-name "Transition Actions::vehicleStates::transition_vehicleStates_to_on"))) (kind "transition") (name "transition_vehicleStates_to_on") (declared-name "transition_vehicleStates_to_on") (parent (node (document "d0") (qualified-name "Transition Actions::vehicleStates"))))
    (element (id (node (document "d0") (qualified-name "Transition Actions::vehicleStates::transition_vehicleStates_to_on::effect"))) (kind "transition effect") (name "effect") (declared-name "effect") (parent (node (document "d0") (qualified-name "Transition Actions::vehicleStates::transition_vehicleStates_to_on"))))
    (element (id (node (document "d0") (qualified-name "Transition Actions::vehicleStates::transition_vehicleStates_to_on::guard"))) (kind "transition guard") (name "guard") (declared-name "guard") (parent (node (document "d0") (qualified-name "Transition Actions::vehicleStates::transition_vehicleStates_to_on"))))
    (element (id (node (document "d0") (qualified-name "Transition Actions::vehicleStates::transition_vehicleStates_to_on::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "Transition Actions::vehicleStates::transition_vehicleStates_to_on"))))
    (element (id (node (document "d0") (qualified-name "Transition Actions::vehicleStates::transition_vehicleStates_to_starting"))) (kind "transition") (name "transition_vehicleStates_to_starting") (declared-name "transition_vehicleStates_to_starting") (parent (node (document "d0") (qualified-name "Transition Actions::vehicleStates"))))
    (element (id (node (document "d0") (qualified-name "Transition Actions::vehicleStates::transition_vehicleStates_to_starting::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "Transition Actions::vehicleStates::transition_vehicleStates_to_starting"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Transition Actions::performSelfTest::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "Transition Actions::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Transition Actions::vehicleStates"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleStates") (outcome (status resolved) (target (node (document "d0") (qualified-name "Transition Actions::VehicleStates")))))
    (reference (id (source (node (document "d0") (qualified-name "Transition Actions::vehicleStates"))) (kind transitionSource) (ordinal 0)) (authored-target "Transition Actions::vehicleStates::starting") (outcome (status resolved) (target (node (document "d0") (qualified-name "Transition Actions::vehicleStates::starting")))))
    (reference (id (source (node (document "d0") (qualified-name "Transition Actions::vehicleStates"))) (kind transitionSource) (ordinal 1)) (authored-target "Transition Actions::vehicleStates::on") (outcome (status resolved) (target (node (document "d0") (qualified-name "Transition Actions::vehicleStates::on")))))
    (reference (id (source (node (document "d0") (qualified-name "Transition Actions::vehicleStates"))) (kind transitionSource) (ordinal 2)) (authored-target "Transition Actions::vehicleStates::off") (outcome (status resolved) (target (node (document "d0") (qualified-name "Transition Actions::vehicleStates::off")))))
    (reference (id (source (node (document "d0") (qualified-name "Transition Actions::vehicleStates"))) (kind initialStateSource) (ordinal 0)) (authored-target "Transition Actions::vehicleStates::off") (outcome (status resolved) (target (node (document "d0") (qualified-name "Transition Actions::vehicleStates::off")))))
    (reference (id (source (node (document "d0") (qualified-name "Transition Actions::vehicleStates::controller"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleController") (outcome (status resolved) (target (node (document "d0") (qualified-name "Transition Actions::VehicleController")))))
    (reference (id (source (node (document "d0") (qualified-name "Transition Actions::vehicleStates::on::_entry::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Transition Actions::vehicleStates::operatingVehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "Transition Actions::Vehicle")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Transition Actions::performSelfTest::vehicle"))) (target (node (document "d0") (qualified-name "Transition Actions::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Transition Actions::performSelfTest::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Transition Actions::vehicleStates"))) (target (node (document "d0") (qualified-name "Transition Actions::VehicleStates"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Transition Actions::vehicleStates"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "Transition Actions::vehicleStates"))) (target (node (document "d0") (qualified-name "Transition Actions::vehicleStates::off"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Transition Actions::vehicleStates"))) (kind transitionSource) (ordinal 2)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "Transition Actions::vehicleStates"))) (target (node (document "d0") (qualified-name "Transition Actions::vehicleStates::on"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Transition Actions::vehicleStates"))) (kind transitionSource) (ordinal 1)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "Transition Actions::vehicleStates"))) (target (node (document "d0") (qualified-name "Transition Actions::vehicleStates::starting"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Transition Actions::vehicleStates"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind initialState) (source (node (document "d0") (qualified-name "Transition Actions::vehicleStates"))) (target (node (document "d0") (qualified-name "Transition Actions::vehicleStates::off"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Transition Actions::vehicleStates"))) (kind initialStateSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Transition Actions::vehicleStates::controller"))) (target (node (document "d0") (qualified-name "Transition Actions::VehicleController"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Transition Actions::vehicleStates::controller"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Transition Actions::vehicleStates::operatingVehicle"))) (target (node (document "d0") (qualified-name "Transition Actions::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Transition Actions::vehicleStates::operatingVehicle"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Transition Actions::vehicleStates::on::_entry::vehicle")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "Transition Actions::vehicleStates::transition_vehicleStates_to_on::guard")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
