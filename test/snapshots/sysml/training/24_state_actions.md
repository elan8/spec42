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
        (range (start 15 2) (end 15 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 15 9) (end 15 9))
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
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 26 3) (end 26 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 27 3) (end 27 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 28 3) (end 28 3))
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
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates::off"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates::on"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates::starting"))) (kind state) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleStates")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::VehicleStates")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates"))) (target (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::VehicleStates"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/24_state_actions.md") (qualified-name "State Actions::vehicleStates"))) (kind featureTyping) (ordinal 0)))
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
)
~~~
