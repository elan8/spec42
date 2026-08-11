# META
~~~ini
description=SysML Training 24 (States): State Decomposition-1
type=file
~~~
# SOURCE
~~~sysml
package 'State Decomposition-1' {
	
	attribute def VehicleStartSignal;
	attribute def VehicleOnSignal;
	attribute def VehicleOffSignal;
	
	state def VehicleStates;
		
	state vehicleStates : VehicleStates {
		entry; then off;
		
		state off;
		accept VehicleStartSignal 
			then starting;
			
		state starting;
		accept VehicleOnSignal
			then on;
			
		state on;
		accept VehicleOffSignal
			then off;
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "24_state_decomposition_1.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'State Decomposition-1' {

    attribute def VehicleStartSignal;
    attribute def VehicleOnSignal;
    attribute def VehicleOffSignal;

    state def VehicleStates;

    state vehicleStates : VehicleStates {
        entry; then off;

        state off;
        accept VehicleStartSignal
        then starting;

        state starting;
        accept VehicleOnSignal
        then on;

        state on;
        accept VehicleOffSignal
        then off;
    }

}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "a79b396b14e0b00ac3fffbf25e805b1eb774578b53e8b8c23ef84df1f0295d74") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "State Decomposition-1"))) (kind "package") (name "State Decomposition-1") (declared-name "State Decomposition-1"))
    (element (id (node (document "d0") (qualified-name "State Decomposition-1::VehicleOffSignal"))) (kind "attribute def") (name "VehicleOffSignal") (declared-name "VehicleOffSignal") (parent (node (document "d0") (qualified-name "State Decomposition-1"))))
    (element (id (node (document "d0") (qualified-name "State Decomposition-1::VehicleOnSignal"))) (kind "attribute def") (name "VehicleOnSignal") (declared-name "VehicleOnSignal") (parent (node (document "d0") (qualified-name "State Decomposition-1"))))
    (element (id (node (document "d0") (qualified-name "State Decomposition-1::VehicleStartSignal"))) (kind "attribute def") (name "VehicleStartSignal") (declared-name "VehicleStartSignal") (parent (node (document "d0") (qualified-name "State Decomposition-1"))))
    (element (id (node (document "d0") (qualified-name "State Decomposition-1::VehicleStates"))) (kind "state def") (name "VehicleStates") (declared-name "VehicleStates") (parent (node (document "d0") (qualified-name "State Decomposition-1"))))
    (element (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))) (kind "state") (name "vehicleStates") (declared-name "vehicleStates") (parent (node (document "d0") (qualified-name "State Decomposition-1"))) (authored (membership (kind Feature)) (relationships (typing (reference "VehicleStates")) (transition (reference "State Decomposition-1::vehicleStates::starting")) (transition (reference "State Decomposition-1::vehicleStates::on")) (transition (reference "State Decomposition-1::vehicleStates::off")) (initial-state (reference "State Decomposition-1::vehicleStates::off")))))
    (element (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::_entry"))) (kind "action") (name "entry") (declared-name "entry") (parent (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))))
    (element (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::off"))) (kind "state") (name "off") (declared-name "off") (parent (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))))
    (element (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::on"))) (kind "state") (name "on") (declared-name "on") (parent (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))))
    (element (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::starting"))) (kind "state") (name "starting") (declared-name "starting") (parent (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))))
    (element (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::transition_vehicleStates_to_off"))) (kind "transition") (name "transition_vehicleStates_to_off") (declared-name "transition_vehicleStates_to_off") (parent (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))))
    (element (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::transition_vehicleStates_to_off::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::transition_vehicleStates_to_off"))))
    (element (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::transition_vehicleStates_to_on"))) (kind "transition") (name "transition_vehicleStates_to_on") (declared-name "transition_vehicleStates_to_on") (parent (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))))
    (element (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::transition_vehicleStates_to_on::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::transition_vehicleStates_to_on"))))
    (element (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::transition_vehicleStates_to_starting"))) (kind "transition") (name "transition_vehicleStates_to_starting") (declared-name "transition_vehicleStates_to_starting") (parent (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))))
    (element (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::transition_vehicleStates_to_starting::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::transition_vehicleStates_to_starting"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleStates") (outcome (status resolved) (target (node (document "d0") (qualified-name "State Decomposition-1::VehicleStates")))))
    (reference (id (source (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))) (kind transitionSource) (ordinal 0)) (authored-target "State Decomposition-1::vehicleStates::starting") (outcome (status resolved) (target (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::starting")))))
    (reference (id (source (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))) (kind transitionSource) (ordinal 1)) (authored-target "State Decomposition-1::vehicleStates::on") (outcome (status resolved) (target (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::on")))))
    (reference (id (source (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))) (kind transitionSource) (ordinal 2)) (authored-target "State Decomposition-1::vehicleStates::off") (outcome (status resolved) (target (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::off")))))
    (reference (id (source (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))) (kind initialStateSource) (ordinal 0)) (authored-target "State Decomposition-1::vehicleStates::off") (outcome (status resolved) (target (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::off")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))) (target (node (document "d0") (qualified-name "State Decomposition-1::VehicleStates"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))) (target (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::off"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))) (kind transitionSource) (ordinal 2)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))) (target (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::on"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))) (kind transitionSource) (ordinal 1)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))) (target (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::starting"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind initialState) (source (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))) (target (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::off"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))) (kind initialStateSource) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
