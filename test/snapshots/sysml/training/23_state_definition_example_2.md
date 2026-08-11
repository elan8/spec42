# META
~~~ini
description=SysML Training 23 (State Definitions): State Definition Example-2
type=file
~~~
# SOURCE
~~~sysml
package 'State Definition Example-2' {
	
	attribute def VehicleStartSignal;
	attribute def VehicleOnSignal;
	attribute def VehicleOffSignal;
		
	state def VehicleStates {
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
  (document "23_state_definition_example_2.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'State Definition Example-2' {

    attribute def VehicleStartSignal;
    attribute def VehicleOnSignal;
    attribute def VehicleOffSignal;

    state def VehicleStates {
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
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "fe07c603399aad42c3b86566508f2d8ee7652e7ac44f06ebcad7d5395a2cd8d6") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "State Definition Example-2"))) (kind "package") (name "State Definition Example-2") (declared-name "State Definition Example-2"))
    (element (id (node (document "d0") (qualified-name "State Definition Example-2::VehicleOffSignal"))) (kind "attribute def") (name "VehicleOffSignal") (declared-name "VehicleOffSignal") (parent (node (document "d0") (qualified-name "State Definition Example-2"))))
    (element (id (node (document "d0") (qualified-name "State Definition Example-2::VehicleOnSignal"))) (kind "attribute def") (name "VehicleOnSignal") (declared-name "VehicleOnSignal") (parent (node (document "d0") (qualified-name "State Definition Example-2"))))
    (element (id (node (document "d0") (qualified-name "State Definition Example-2::VehicleStartSignal"))) (kind "attribute def") (name "VehicleStartSignal") (declared-name "VehicleStartSignal") (parent (node (document "d0") (qualified-name "State Definition Example-2"))))
    (element (id (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates"))) (kind "state def") (name "VehicleStates") (declared-name "VehicleStates") (parent (node (document "d0") (qualified-name "State Definition Example-2"))) (authored (membership (kind Owning)) (relationships (transition (reference "State Definition Example-2::VehicleStates::starting")) (transition (reference "State Definition Example-2::VehicleStates::on")) (transition (reference "State Definition Example-2::VehicleStates::off")) (initial-state (reference "State Definition Example-2::VehicleStates::off")))))
    (element (id (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates::_entry"))) (kind "action") (name "entry") (declared-name "entry") (parent (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates"))))
    (element (id (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates::off"))) (kind "state") (name "off") (declared-name "off") (parent (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates"))))
    (element (id (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates::on"))) (kind "state") (name "on") (declared-name "on") (parent (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates"))))
    (element (id (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates::starting"))) (kind "state") (name "starting") (declared-name "starting") (parent (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates"))))
    (element (id (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates::transition_VehicleStates_to_off"))) (kind "transition") (name "transition_VehicleStates_to_off") (declared-name "transition_VehicleStates_to_off") (parent (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates"))))
    (element (id (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates::transition_VehicleStates_to_off::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates::transition_VehicleStates_to_off"))))
    (element (id (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates::transition_VehicleStates_to_on"))) (kind "transition") (name "transition_VehicleStates_to_on") (declared-name "transition_VehicleStates_to_on") (parent (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates"))))
    (element (id (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates::transition_VehicleStates_to_on::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates::transition_VehicleStates_to_on"))))
    (element (id (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates::transition_VehicleStates_to_starting"))) (kind "transition") (name "transition_VehicleStates_to_starting") (declared-name "transition_VehicleStates_to_starting") (parent (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates"))))
    (element (id (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates::transition_VehicleStates_to_starting::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates::transition_VehicleStates_to_starting"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates"))) (kind transitionSource) (ordinal 0)) (authored-target "State Definition Example-2::VehicleStates::starting") (outcome (status resolved) (target (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates::starting")))))
    (reference (id (source (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates"))) (kind transitionSource) (ordinal 1)) (authored-target "State Definition Example-2::VehicleStates::on") (outcome (status resolved) (target (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates::on")))))
    (reference (id (source (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates"))) (kind transitionSource) (ordinal 2)) (authored-target "State Definition Example-2::VehicleStates::off") (outcome (status resolved) (target (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates::off")))))
    (reference (id (source (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates"))) (kind initialStateSource) (ordinal 0)) (authored-target "State Definition Example-2::VehicleStates::off") (outcome (status resolved) (target (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates::off")))))
  )
  (relationships
    (relationship (kind transition) (source (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates"))) (target (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates::off"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates"))) (kind transitionSource) (ordinal 2)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates"))) (target (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates::on"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates"))) (kind transitionSource) (ordinal 1)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates"))) (target (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates::starting"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind initialState) (source (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates"))) (target (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates::off"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates"))) (kind initialStateSource) (ordinal 0)))
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
