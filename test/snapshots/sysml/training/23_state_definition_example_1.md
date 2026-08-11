# META
~~~ini
description=SysML Training 23 (State Definitions): State Definition Example-1
type=file
~~~
# SOURCE
~~~sysml
package 'State Definition Example-1' {
	
	attribute def VehicleStartSignal;
	attribute def VehicleOnSignal;
	attribute def VehicleOffSignal;
		
	state def VehicleStates {
		entry; then off;
		
		state off;
		
		transition off_to_starting
			first off
			accept VehicleStartSignal 
			then starting;
			
		state starting;
		
		transition starting_to_on
			first starting
			accept VehicleOnSignal
			then on;
			
		state on;
		
		transition on_to_off
			first on
			accept VehicleOffSignal
			then off;
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "23_state_definition_example_1.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'State Definition Example-1' {

    attribute def VehicleStartSignal;
    attribute def VehicleOnSignal;
    attribute def VehicleOffSignal;

    state def VehicleStates {
        entry; then off;

        state off;

        transition off_to_starting
        first off
        accept VehicleStartSignal
        then starting;

        state starting;

        transition starting_to_on
        first starting
        accept VehicleOnSignal
        then on;

        state on;

        transition on_to_off
        first on
        accept VehicleOffSignal
        then off;
    }

}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "b1cf084f6582b64b19e4355911c073ec14540e55c1aa252f5319fd9f6ce8929a") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "State Definition Example-1"))) (kind "package") (name "State Definition Example-1") (declared-name "State Definition Example-1"))
    (element (id (node (document "d0") (qualified-name "State Definition Example-1::VehicleOffSignal"))) (kind "attribute def") (name "VehicleOffSignal") (declared-name "VehicleOffSignal") (parent (node (document "d0") (qualified-name "State Definition Example-1"))))
    (element (id (node (document "d0") (qualified-name "State Definition Example-1::VehicleOnSignal"))) (kind "attribute def") (name "VehicleOnSignal") (declared-name "VehicleOnSignal") (parent (node (document "d0") (qualified-name "State Definition Example-1"))))
    (element (id (node (document "d0") (qualified-name "State Definition Example-1::VehicleStartSignal"))) (kind "attribute def") (name "VehicleStartSignal") (declared-name "VehicleStartSignal") (parent (node (document "d0") (qualified-name "State Definition Example-1"))))
    (element (id (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates"))) (kind "state def") (name "VehicleStates") (declared-name "VehicleStates") (parent (node (document "d0") (qualified-name "State Definition Example-1"))) (authored (membership (kind Owning)) (relationships (initial-state (reference "State Definition Example-1::VehicleStates::off")))))
    (element (id (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates::_entry"))) (kind "action") (name "entry") (declared-name "entry") (parent (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates"))))
    (element (id (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates::off"))) (kind "state") (name "off") (declared-name "off") (parent (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates"))) (authored (membership (kind Feature)) (relationships (transition (reference "State Definition Example-1::VehicleStates::starting")))))
    (element (id (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates::off_to_starting"))) (kind "transition") (name "off_to_starting") (declared-name "off_to_starting") (parent (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates"))))
    (element (id (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates::off_to_starting::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates::off_to_starting"))))
    (element (id (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates::on"))) (kind "state") (name "on") (declared-name "on") (parent (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates"))) (authored (membership (kind Feature)) (relationships (transition (reference "State Definition Example-1::VehicleStates::off")))))
    (element (id (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates::on_to_off"))) (kind "transition") (name "on_to_off") (declared-name "on_to_off") (parent (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates"))))
    (element (id (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates::on_to_off::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates::on_to_off"))))
    (element (id (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates::starting"))) (kind "state") (name "starting") (declared-name "starting") (parent (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates"))) (authored (membership (kind Feature)) (relationships (transition (reference "State Definition Example-1::VehicleStates::on")))))
    (element (id (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates::starting_to_on"))) (kind "transition") (name "starting_to_on") (declared-name "starting_to_on") (parent (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates"))))
    (element (id (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates::starting_to_on::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates::starting_to_on"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates"))) (kind initialStateSource) (ordinal 0)) (authored-target "State Definition Example-1::VehicleStates::off") (outcome (status resolved) (target (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates::off")))))
    (reference (id (source (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates::off"))) (kind transitionSource) (ordinal 0)) (authored-target "State Definition Example-1::VehicleStates::starting") (outcome (status resolved) (target (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates::starting")))))
    (reference (id (source (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates::on"))) (kind transitionSource) (ordinal 0)) (authored-target "State Definition Example-1::VehicleStates::off") (outcome (status resolved) (target (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates::off")))))
    (reference (id (source (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates::starting"))) (kind transitionSource) (ordinal 0)) (authored-target "State Definition Example-1::VehicleStates::on") (outcome (status resolved) (target (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates::on")))))
  )
  (relationships
    (relationship (kind initialState) (source (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates"))) (target (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates::off"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates"))) (kind initialStateSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates::off"))) (target (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates::starting"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates::off"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates::on"))) (target (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates::off"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates::on"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates::starting"))) (target (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates::on"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates::starting"))) (kind transitionSource) (ordinal 0)))
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
