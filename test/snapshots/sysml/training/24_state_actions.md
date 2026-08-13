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
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 8 26) (end 8 47))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 10 27) (end 10 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 13 2) (end 13 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 15 2) (end 15 8))
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
        (range (start 22 2) (end 22 2))
      )
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
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 30 2) (end 30 2))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:0f9fa97583cf0795272097e2329c23c15a2830547c2c8e9695c66899e0cba16b") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::VehicleOffSignal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::VehicleOnSignal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::VehicleStartSignal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::VehicleStates"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::performSelfTest"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates"))) (kind state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleStates"))))
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (anonymous (kind initial-state) (ordinal 0))))) (kind initial-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (initialState (reference "off"))))
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates::off"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates::on"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entry-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (entryActionBinding (reference "performSelfTest"))))
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (anonymous (kind do-action-binding) (ordinal 0))))) (kind do-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (doActionBinding (reference "providePower"))))
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (anonymous (kind exit-action-binding) (ordinal 0))))) (kind exit-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (exitActionBinding (reference "applyParkingBrake"))))
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates::starting"))) (kind state) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleStates")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::VehicleStates")))))
    (reference (id (source (node (document "memory://snapshot/24_state_actions.md") (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0))
      (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates::off")))))
    (reference (id (source (node (document "memory://snapshot/24_state_actions.md") (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0))
      (authored-target "performSelfTest")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::performSelfTest")))))
    (reference (id (source (node (document "memory://snapshot/24_state_actions.md") (anonymous (kind do-action-binding) (ordinal 0))))) (kind doActionBinding) (ordinal 0))
      (authored-target "providePower")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/24_state_actions.md") (anonymous (kind exit-action-binding) (ordinal 0))))) (kind exitActionBinding) (ordinal 0))
      (authored-target "applyParkingBrake")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates"))) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::VehicleStates"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind initialState) (source (node (document "memory://snapshot/24_state_actions.md") (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates::off"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/24_state_actions.md") (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0)))
    (relationship (kind entryActionBinding) (source (node (document "memory://snapshot/24_state_actions.md") (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::performSelfTest"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/24_state_actions.md") (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/24_state_actions.md") (range (start 12 23) (end 12 36)) (probe (position 12 23))
    (reference (id (source (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleStates")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::VehicleStates")))))
  )
  (query (document "memory://snapshot/24_state_actions.md") (range (start 15 14) (end 15 17)) (probe (position 15 14))
    (reference (id (source (node (document "memory://snapshot/24_state_actions.md") (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0) (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates::off")))))
  )
  (query (document "memory://snapshot/24_state_actions.md") (range (start 26 9) (end 26 24)) (probe (position 26 9))
    (reference (id (source (node (document "memory://snapshot/24_state_actions.md") (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0) (authored-target "performSelfTest")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::performSelfTest")))))
  )
  (query (document "memory://snapshot/24_state_actions.md") (range (start 27 13) (end 27 25)) (probe (position 27 13))
    (reference (id (source (node (document "memory://snapshot/24_state_actions.md") (anonymous (kind do-action-binding) (ordinal 0))))) (kind doActionBinding) (ordinal 0) (authored-target "providePower")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/24_state_actions.md") (range (start 28 15) (end 28 32)) (probe (position 28 15))
    (reference (id (source (node (document "memory://snapshot/24_state_actions.md") (anonymous (kind exit-action-binding) (ordinal 0))))) (kind exitActionBinding) (ordinal 0) (authored-target "applyParkingBrake")
      (outcome (status unresolved)))
  )
)
~~~
