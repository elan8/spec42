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
  (document "memory://snapshot/25_transition_actions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 24) (end 9 45))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 28 2) (end 31 11))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 35 13) (end 35 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 36 15) (end 36 32))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:ac736c7798260aaa6c7eb92a19c0a38b83d51e378db2be1aa48520bb12c907cc") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::ControllerStartSignal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::Vehicle::brakePedalDepressed"))) (kind default-reference) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValues::Boolean"))))
    (declaration (id (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::VehicleController"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::VehicleOffSignal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::VehicleOnSignal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::VehicleStartSignal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::VehicleStates"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::performSelfTest"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::performSelfTest::vehicle"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle") (direction in))))
    (declaration (id (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::vehicleStates"))) (kind state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleStates"))))
    (declaration (id (node (document "memory://snapshot/25_transition_actions.md") (path (named (kind package) (name "Transition Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind initial-state) (ordinal 0)))))) (kind initial-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (initialState (reference "off"))))
    (declaration (id (node (document "memory://snapshot/25_transition_actions.md") (path (named (kind package) (name "Transition Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 0)))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionTarget (reference "starting")) (transitionTrigger (reference "VehicleStartSignal"))))
    (declaration (id (node (document "memory://snapshot/25_transition_actions.md") (path (named (kind package) (name "Transition Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 1)))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionTarget (reference "on")) (transitionTrigger (reference "VehicleOnSignal")) (memberAccessOperand (reference "operatingVehicle::brakePedalDepressed"))))
    (declaration (id (node (document "memory://snapshot/25_transition_actions.md") (path (named (kind package) (name "Transition Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 2)))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionTarget (reference "off")) (transitionTrigger (reference "VehicleOffSignal"))))
    (declaration (id (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::vehicleStates::controller"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleController") (direction in))))
    (declaration (id (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::vehicleStates::off"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::vehicleStates::on"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/25_transition_actions.md") (path (named (kind package) (name "Transition Actions")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "on")) (anonymous (kind entry-action-binding) (ordinal 0)))))) (kind entry-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (entryActionBinding (reference "performSelfTest"))))
    (declaration (id (node (document "memory://snapshot/25_transition_actions.md") (path (named (kind package) (name "Transition Actions")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "on")) (anonymous (kind do-action-binding) (ordinal 0)))))) (kind do-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (doActionBinding (reference "providePower"))))
    (declaration (id (node (document "memory://snapshot/25_transition_actions.md") (path (named (kind package) (name "Transition Actions")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "on")) (anonymous (kind exit-action-binding) (ordinal 0)))))) (kind exit-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (exitActionBinding (reference "applyParkingBrake"))))
    (declaration (id (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::vehicleStates::operatingVehicle"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle") (direction in))))
    (declaration (id (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::vehicleStates::starting"))) (kind state) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::Vehicle::brakePedalDepressed"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::performSelfTest::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::vehicleStates"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleStates")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::VehicleStates")))))
    (reference (id (source (node (document "memory://snapshot/25_transition_actions.md") (path (named (kind package) (name "Transition Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind initial-state) (ordinal 0)))))) (kind initialState) (ordinal 0))
      (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::vehicleStates::off")))))
    (reference (id (source (node (document "memory://snapshot/25_transition_actions.md") (path (named (kind package) (name "Transition Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 0)))))) (kind transitionTarget) (ordinal 0))
      (authored-target "starting")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::vehicleStates::starting")))))
    (reference (id (source (node (document "memory://snapshot/25_transition_actions.md") (path (named (kind package) (name "Transition Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 1)))))) (kind transitionTarget) (ordinal 0))
      (authored-target "on")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::vehicleStates::on")))))
    (reference (id (source (node (document "memory://snapshot/25_transition_actions.md") (path (named (kind package) (name "Transition Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 2)))))) (kind transitionTarget) (ordinal 0))
      (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::vehicleStates::off")))))
    (reference (id (source (node (document "memory://snapshot/25_transition_actions.md") (path (named (kind package) (name "Transition Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 0)))))) (kind transitionTrigger) (ordinal 0))
      (authored-target "VehicleStartSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::VehicleStartSignal")))))
    (reference (id (source (node (document "memory://snapshot/25_transition_actions.md") (path (named (kind package) (name "Transition Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 1)))))) (kind transitionTrigger) (ordinal 0))
      (authored-target "VehicleOnSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::VehicleOnSignal")))))
    (reference (id (source (node (document "memory://snapshot/25_transition_actions.md") (path (named (kind package) (name "Transition Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 2)))))) (kind transitionTrigger) (ordinal 0))
      (authored-target "VehicleOffSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::VehicleOffSignal")))))
    (reference (id (source (node (document "memory://snapshot/25_transition_actions.md") (path (named (kind package) (name "Transition Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 1)))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "operatingVehicle::brakePedalDepressed")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::Vehicle::brakePedalDepressed")))))
    (reference (id (source (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::vehicleStates::controller"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleController")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::VehicleController")))))
    (reference (id (source (node (document "memory://snapshot/25_transition_actions.md") (path (named (kind package) (name "Transition Actions")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "on")) (anonymous (kind entry-action-binding) (ordinal 0)))))) (kind entryActionBinding) (ordinal 0))
      (authored-target "performSelfTest")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::performSelfTest")))))
    (reference (id (source (node (document "memory://snapshot/25_transition_actions.md") (path (named (kind package) (name "Transition Actions")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "on")) (anonymous (kind do-action-binding) (ordinal 0)))))) (kind doActionBinding) (ordinal 0))
      (authored-target "providePower")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/25_transition_actions.md") (path (named (kind package) (name "Transition Actions")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "on")) (anonymous (kind exit-action-binding) (ordinal 0)))))) (kind exitActionBinding) (ordinal 0))
      (authored-target "applyParkingBrake")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::vehicleStates::operatingVehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::Vehicle")))))
  )
  (relationships
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::performSelfTest::vehicle"))) (target (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::performSelfTest::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::vehicleStates"))) (target (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::VehicleStates"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::vehicleStates"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind initialState) (source (node (document "memory://snapshot/25_transition_actions.md") (path (named (kind package) (name "Transition Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind initial-state) (ordinal 0)))))) (target (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::vehicleStates::off"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/25_transition_actions.md") (path (named (kind package) (name "Transition Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind initial-state) (ordinal 0)))))) (kind initialState) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/25_transition_actions.md") (path (named (kind package) (name "Transition Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 0)))))) (target (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::vehicleStates::starting"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/25_transition_actions.md") (path (named (kind package) (name "Transition Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 0)))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/25_transition_actions.md") (path (named (kind package) (name "Transition Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 1)))))) (target (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::vehicleStates::on"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/25_transition_actions.md") (path (named (kind package) (name "Transition Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 1)))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/25_transition_actions.md") (path (named (kind package) (name "Transition Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 2)))))) (target (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::vehicleStates::off"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/25_transition_actions.md") (path (named (kind package) (name "Transition Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 2)))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/25_transition_actions.md") (path (named (kind package) (name "Transition Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 0)))))) (target (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::VehicleStartSignal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/25_transition_actions.md") (path (named (kind package) (name "Transition Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 0)))))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/25_transition_actions.md") (path (named (kind package) (name "Transition Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 1)))))) (target (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::VehicleOnSignal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/25_transition_actions.md") (path (named (kind package) (name "Transition Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 1)))))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/25_transition_actions.md") (path (named (kind package) (name "Transition Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 2)))))) (target (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::VehicleOffSignal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/25_transition_actions.md") (path (named (kind package) (name "Transition Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 2)))))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/25_transition_actions.md") (path (named (kind package) (name "Transition Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 1)))))) (target (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::Vehicle::brakePedalDepressed"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/25_transition_actions.md") (path (named (kind package) (name "Transition Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 1)))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::vehicleStates::controller"))) (target (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::VehicleController"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::vehicleStates::controller"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind entryActionBinding) (source (node (document "memory://snapshot/25_transition_actions.md") (path (named (kind package) (name "Transition Actions")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "on")) (anonymous (kind entry-action-binding) (ordinal 0)))))) (target (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::performSelfTest"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/25_transition_actions.md") (path (named (kind package) (name "Transition Actions")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "on")) (anonymous (kind entry-action-binding) (ordinal 0)))))) (kind entryActionBinding) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::vehicleStates::operatingVehicle"))) (target (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::vehicleStates::operatingVehicle"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::performSelfTest::vehicle")))
      (supertype (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::Vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::vehicleStates")))
      (supertype (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::VehicleStates")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::vehicleStates::controller")))
      (supertype (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::VehicleController")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::vehicleStates::operatingVehicle")))
      (supertype (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::Vehicle")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/25_transition_actions.md") (range (start 9 24) (end 9 45)) (probe (position 9 24))
    (reference (id (source (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::Vehicle::brakePedalDepressed"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/25_transition_actions.md") (range (start 13 39) (end 13 46)) (probe (position 13 39))
    (reference (id (source (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::performSelfTest::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::Vehicle")))))
  )
  (query (document "memory://snapshot/25_transition_actions.md") (range (start 17 23) (end 17 36)) (probe (position 17 23))
    (reference (id (source (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::vehicleStates"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleStates")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::VehicleStates")))))
  )
  (query (document "memory://snapshot/25_transition_actions.md") (range (start 21 14) (end 21 17)) (probe (position 21 14))
    (reference (id (source (node (document "memory://snapshot/25_transition_actions.md") (path (named (kind package) (name "Transition Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind initial-state) (ordinal 0)))))) (kind initialState) (ordinal 0) (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::vehicleStates::off")))))
  )
  (query (document "memory://snapshot/25_transition_actions.md") (range (start 25 8) (end 25 16)) (probe (position 25 8))
    (reference (id (source (node (document "memory://snapshot/25_transition_actions.md") (path (named (kind package) (name "Transition Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 0)))))) (kind transitionTarget) (ordinal 0) (authored-target "starting")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::vehicleStates::starting")))))
  )
  (query (document "memory://snapshot/25_transition_actions.md") (range (start 31 8) (end 31 10)) (probe (position 31 8))
    (reference (id (source (node (document "memory://snapshot/25_transition_actions.md") (path (named (kind package) (name "Transition Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 1)))))) (kind transitionTarget) (ordinal 0) (authored-target "on")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::vehicleStates::on")))))
  )
  (query (document "memory://snapshot/25_transition_actions.md") (range (start 39 8) (end 39 11)) (probe (position 39 8))
    (reference (id (source (node (document "memory://snapshot/25_transition_actions.md") (path (named (kind package) (name "Transition Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 2)))))) (kind transitionTarget) (ordinal 0) (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::vehicleStates::off")))))
  )
  (query (document "memory://snapshot/25_transition_actions.md") (range (start 24 9) (end 24 27)) (probe (position 24 9))
    (reference (id (source (node (document "memory://snapshot/25_transition_actions.md") (path (named (kind package) (name "Transition Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 0)))))) (kind transitionTrigger) (ordinal 0) (authored-target "VehicleStartSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::VehicleStartSignal")))))
  )
  (query (document "memory://snapshot/25_transition_actions.md") (range (start 28 9) (end 28 24)) (probe (position 28 9))
    (reference (id (source (node (document "memory://snapshot/25_transition_actions.md") (path (named (kind package) (name "Transition Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 1)))))) (kind transitionTrigger) (ordinal 0) (authored-target "VehicleOnSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::VehicleOnSignal")))))
  )
  (query (document "memory://snapshot/25_transition_actions.md") (range (start 38 9) (end 38 25)) (probe (position 38 9))
    (reference (id (source (node (document "memory://snapshot/25_transition_actions.md") (path (named (kind package) (name "Transition Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 2)))))) (kind transitionTrigger) (ordinal 0) (authored-target "VehicleOffSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::VehicleOffSignal")))))
  )
  (query (document "memory://snapshot/25_transition_actions.md") (range (start 29 6) (end 29 42)) (probe (position 29 6))
    (reference (id (source (node (document "memory://snapshot/25_transition_actions.md") (path (named (kind package) (name "Transition Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 1)))))) (kind memberAccessOperand) (ordinal 0) (authored-target "operatingVehicle::brakePedalDepressed")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::Vehicle::brakePedalDepressed")))))
  )
  (query (document "memory://snapshot/25_transition_actions.md") (range (start 19 18) (end 19 35)) (probe (position 19 18))
    (reference (id (source (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::vehicleStates::controller"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleController")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::VehicleController")))))
  )
  (query (document "memory://snapshot/25_transition_actions.md") (range (start 34 9) (end 34 24)) (probe (position 34 9))
    (reference (id (source (node (document "memory://snapshot/25_transition_actions.md") (path (named (kind package) (name "Transition Actions")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "on")) (anonymous (kind entry-action-binding) (ordinal 0)))))) (kind entryActionBinding) (ordinal 0) (authored-target "performSelfTest")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::performSelfTest")))))
  )
  (query (document "memory://snapshot/25_transition_actions.md") (range (start 35 13) (end 35 25)) (probe (position 35 13))
    (reference (id (source (node (document "memory://snapshot/25_transition_actions.md") (path (named (kind package) (name "Transition Actions")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "on")) (anonymous (kind do-action-binding) (ordinal 0)))))) (kind doActionBinding) (ordinal 0) (authored-target "providePower")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/25_transition_actions.md") (range (start 36 15) (end 36 32)) (probe (position 36 15))
    (reference (id (source (node (document "memory://snapshot/25_transition_actions.md") (path (named (kind package) (name "Transition Actions")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "on")) (anonymous (kind exit-action-binding) (ordinal 0)))))) (kind exitActionBinding) (ordinal 0) (authored-target "applyParkingBrake")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/25_transition_actions.md") (range (start 18 24) (end 18 31)) (probe (position 18 24))
    (reference (id (source (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::vehicleStates::operatingVehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::Vehicle")))))
  )
)
~~~
