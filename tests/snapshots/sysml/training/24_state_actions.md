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
  (document "memory://snapshot/24_state_actions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 27 13) (end 27 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 28 15) (end 28 32))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:0f9fa97583cf0795272097e2329c23c15a2830547c2c8e9695c66899e0cba16b") (contract-version "feature-value-expression-results-v5"))
  (declarations
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::VehicleOffSignal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::VehicleOnSignal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::VehicleStartSignal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::VehicleStates"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::VehicleStates::operatingVehicle"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle") (direction in)))))
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::performSelfTest"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::performSelfTest::vehicle"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle") (direction in)))))
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates"))) (kind state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleStates")))))
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind initial-state) (ordinal 0))))) (kind initial-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (initialState (reference "off")))))
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 0))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionTarget (reference "starting")) (transitionTrigger (reference "VehicleStartSignal")))))
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 1))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionTarget (reference "on")) (transitionTrigger (reference "VehicleOnSignal")))))
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 2))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionTarget (reference "off")) (transitionTrigger (reference "VehicleOffSignal")))))
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates::off"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates::on"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "on")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entry-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (entryActionBinding (reference "performSelfTest")))))
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "on")) (anonymous (kind do-action-binding) (ordinal 0))))) (kind do-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (doActionBinding (reference "providePower")))))
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "on")) (anonymous (kind exit-action-binding) (ordinal 0))))) (kind exit-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (exitActionBinding (reference "applyParkingBrake")))))
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates::operatingVehicle"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle") (direction in)))))
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates::starting"))) (kind state) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::VehicleStates::operatingVehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::performSelfTest::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleStates")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::VehicleStates")))))
    (reference (id (source (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0))
      (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates::off")))))
    (reference (id (source (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0))
      (authored-target "starting")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates::starting")))))
    (reference (id (source (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 1))))) (kind transitionTarget) (ordinal 0))
      (authored-target "on")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates::on")))))
    (reference (id (source (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 2))))) (kind transitionTarget) (ordinal 0))
      (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates::off")))))
    (reference (id (source (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTrigger) (ordinal 0))
      (authored-target "VehicleStartSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::VehicleStartSignal")))))
    (reference (id (source (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 1))))) (kind transitionTrigger) (ordinal 0))
      (authored-target "VehicleOnSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::VehicleOnSignal")))))
    (reference (id (source (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 2))))) (kind transitionTrigger) (ordinal 0))
      (authored-target "VehicleOffSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::VehicleOffSignal")))))
    (reference (id (source (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "on")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0))
      (authored-target "performSelfTest")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::performSelfTest")))))
    (reference (id (source (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "on")) (anonymous (kind do-action-binding) (ordinal 0))))) (kind doActionBinding) (ordinal 0))
      (authored-target "providePower")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "on")) (anonymous (kind exit-action-binding) (ordinal 0))))) (kind exitActionBinding) (ordinal 0))
      (authored-target "applyParkingBrake")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates::operatingVehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::Vehicle")))))
  )
  (relationships
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::VehicleStates::operatingVehicle"))) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::VehicleStates::operatingVehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::performSelfTest::vehicle"))) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::performSelfTest::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates"))) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::VehicleStates"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind initialState) (source (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates::off"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates::starting"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 1))))) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates::on"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 1))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 2))))) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates::off"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 2))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::VehicleStartSignal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 1))))) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::VehicleOnSignal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 1))))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 2))))) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::VehicleOffSignal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 2))))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind entryActionBinding) (source (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "on")) (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::performSelfTest"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "on")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates::operatingVehicle"))) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates::operatingVehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::VehicleStates::operatingVehicle"))) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::VehicleStates"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::performSelfTest::vehicle"))) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::performSelfTest"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 1))))) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 2))))) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 1))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 2))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates::off"))) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates::on"))) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "on")) (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates::on"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "on")) (anonymous (kind do-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates::on"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "on")) (anonymous (kind exit-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates::on"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates::operatingVehicle"))) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates::starting"))) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::Vehicle")))
      (subtype (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::VehicleStates::operatingVehicle")) (scopes any))
      (subtype (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::performSelfTest::vehicle")) (scopes any))
      (subtype (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates::operatingVehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::VehicleStates")))
      (subtype (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::VehicleStates::operatingVehicle")))
      (featured-by (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::VehicleStates")))
      (type (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::Vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::performSelfTest::vehicle")))
      (featured-by (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::performSelfTest")))
      (type (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::Vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates")))
      (type (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::VehicleStates")) (provenance authored))
      (effective-type (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::VehicleStates")) (source direct))
      (supertype (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::VehicleStates")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind initial-state) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates")))
    )
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates")))
    )
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates")))
    )
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates")))
    )
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 1)))))
    )
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 2)))))
    )
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates::off")))
      (featured-by (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates")))
    )
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates::on")))
      (featured-by (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates")))
    )
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "on")) (anonymous (kind entry-action-binding) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates::on")))
    )
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "on")) (anonymous (kind do-action-binding) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates::on")))
    )
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "on")) (anonymous (kind exit-action-binding) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates::on")))
    )
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates::operatingVehicle")))
      (featured-by (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates")))
      (type (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::Vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates::starting")))
      (featured-by (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/24_state_actions.md") (range (start 10 49) (end 10 56)) (probe (position 10 49))
    (reference (id (source (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::VehicleStates::operatingVehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/24_state_actions.md") (range (start 8 39) (end 8 46)) (probe (position 8 39))
    (reference (id (source (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::performSelfTest::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/24_state_actions.md") (range (start 12 23) (end 12 36)) (probe (position 12 23))
    (reference (id (source (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleStates")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::VehicleStates")))))
    )
  )
  (query (document "memory://snapshot/24_state_actions.md") (range (start 15 14) (end 15 17)) (probe (position 15 14))
    (reference (id (source (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0) (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates::off")))))
    )
  )
  (query (document "memory://snapshot/24_state_actions.md") (range (start 19 8) (end 19 16)) (probe (position 19 8))
    (reference (id (source (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0) (authored-target "starting")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates::starting")))))
    )
  )
  (query (document "memory://snapshot/24_state_actions.md") (range (start 23 8) (end 23 10)) (probe (position 23 8))
    (reference (id (source (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 1))))) (kind transitionTarget) (ordinal 0) (authored-target "on")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates::on")))))
    )
  )
  (query (document "memory://snapshot/24_state_actions.md") (range (start 31 8) (end 31 11)) (probe (position 31 8))
    (reference (id (source (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 2))))) (kind transitionTarget) (ordinal 0) (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates::off")))))
    )
  )
  (query (document "memory://snapshot/24_state_actions.md") (range (start 18 9) (end 18 27)) (probe (position 18 9))
    (reference (id (source (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTrigger) (ordinal 0) (authored-target "VehicleStartSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::VehicleStartSignal")))))
    )
  )
  (query (document "memory://snapshot/24_state_actions.md") (range (start 22 9) (end 22 24)) (probe (position 22 9))
    (reference (id (source (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 1))))) (kind transitionTrigger) (ordinal 0) (authored-target "VehicleOnSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::VehicleOnSignal")))))
    )
  )
  (query (document "memory://snapshot/24_state_actions.md") (range (start 30 9) (end 30 25)) (probe (position 30 9))
    (reference (id (source (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 2))))) (kind transitionTrigger) (ordinal 0) (authored-target "VehicleOffSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::VehicleOffSignal")))))
    )
  )
  (query (document "memory://snapshot/24_state_actions.md") (range (start 26 9) (end 26 24)) (probe (position 26 9))
    (reference (id (source (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "on")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0) (authored-target "performSelfTest")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::performSelfTest")))))
    )
  )
  (query (document "memory://snapshot/24_state_actions.md") (range (start 27 13) (end 27 25)) (probe (position 27 13))
    (reference (id (source (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "on")) (anonymous (kind do-action-binding) (ordinal 0))))) (kind doActionBinding) (ordinal 0) (authored-target "providePower")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/24_state_actions.md") (range (start 28 15) (end 28 32)) (probe (position 28 15))
    (reference (id (source (node (document "memory://snapshot/24_state_actions.md") (path (named (kind package) (name "State Actions")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "on")) (anonymous (kind exit-action-binding) (ordinal 0))))) (kind exitActionBinding) (ordinal 0) (authored-target "applyParkingBrake")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/24_state_actions.md") (range (start 13 24) (end 13 31)) (probe (position 13 24))
    (reference (id (source (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates::operatingVehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::Vehicle")))))
    )
  )
)
~~~
