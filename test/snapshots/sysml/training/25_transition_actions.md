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
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 9 2) (end 9 46))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 13 26) (end 13 47))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 18 2) (end 18 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 19 2) (end 19 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 21 2) (end 21 8))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 24 2) (end 24 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 28 2) (end 28 2))
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
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 38 2) (end 38 2))
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
    (declaration (id (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::VehicleController"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::VehicleOffSignal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::VehicleOnSignal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::VehicleStartSignal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::VehicleStates"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::performSelfTest"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::vehicleStates"))) (kind state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleStates"))))
    (declaration (id (node (document "memory://snapshot/25_transition_actions.md") (anonymous (kind initial-state) (ordinal 0))))) (kind initial-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (initialState (reference "off"))))
    (declaration (id (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::vehicleStates::off"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::vehicleStates::on"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/25_transition_actions.md") (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entry-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (entryActionBinding (reference "performSelfTest"))))
    (declaration (id (node (document "memory://snapshot/25_transition_actions.md") (anonymous (kind do-action-binding) (ordinal 0))))) (kind do-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (doActionBinding (reference "providePower"))))
    (declaration (id (node (document "memory://snapshot/25_transition_actions.md") (anonymous (kind exit-action-binding) (ordinal 0))))) (kind exit-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (exitActionBinding (reference "applyParkingBrake"))))
    (declaration (id (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::vehicleStates::starting"))) (kind state) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::vehicleStates"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleStates")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::VehicleStates")))))
    (reference (id (source (node (document "memory://snapshot/25_transition_actions.md") (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0))
      (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::vehicleStates::off")))))
    (reference (id (source (node (document "memory://snapshot/25_transition_actions.md") (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0))
      (authored-target "performSelfTest")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::performSelfTest")))))
    (reference (id (source (node (document "memory://snapshot/25_transition_actions.md") (anonymous (kind do-action-binding) (ordinal 0))))) (kind doActionBinding) (ordinal 0))
      (authored-target "providePower")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/25_transition_actions.md") (anonymous (kind exit-action-binding) (ordinal 0))))) (kind exitActionBinding) (ordinal 0))
      (authored-target "applyParkingBrake")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::vehicleStates"))) (target (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::VehicleStates"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::vehicleStates"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind initialState) (source (node (document "memory://snapshot/25_transition_actions.md") (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::vehicleStates::off"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/25_transition_actions.md") (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0)))
    (relationship (kind entryActionBinding) (source (node (document "memory://snapshot/25_transition_actions.md") (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::performSelfTest"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/25_transition_actions.md") (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/25_transition_actions.md") (range (start 17 23) (end 17 36)) (probe (position 17 23))
    (reference (id (source (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::vehicleStates"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleStates")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::VehicleStates")))))
  )
  (query (document "memory://snapshot/25_transition_actions.md") (range (start 21 14) (end 21 17)) (probe (position 21 14))
    (reference (id (source (node (document "memory://snapshot/25_transition_actions.md") (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0) (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::vehicleStates::off")))))
  )
  (query (document "memory://snapshot/25_transition_actions.md") (range (start 34 9) (end 34 24)) (probe (position 34 9))
    (reference (id (source (node (document "memory://snapshot/25_transition_actions.md") (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0) (authored-target "performSelfTest")
      (outcome (status resolved) (target (node (document "memory://snapshot/25_transition_actions.md") (qualified-name "Transition Actions::performSelfTest")))))
  )
  (query (document "memory://snapshot/25_transition_actions.md") (range (start 35 13) (end 35 25)) (probe (position 35 13))
    (reference (id (source (node (document "memory://snapshot/25_transition_actions.md") (anonymous (kind do-action-binding) (ordinal 0))))) (kind doActionBinding) (ordinal 0) (authored-target "providePower")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/25_transition_actions.md") (range (start 36 15) (end 36 32)) (probe (position 36 15))
    (reference (id (source (node (document "memory://snapshot/25_transition_actions.md") (anonymous (kind exit-action-binding) (ordinal 0))))) (kind exitActionBinding) (ordinal 0) (authored-target "applyParkingBrake")
      (outcome (status unresolved)))
  )
)
~~~
