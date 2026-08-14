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
  (document "memory://snapshot/23_state_definition_example_1.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:7ef4b1b27e3d08e6bb597e627a083862f3384e9d57fd6f80f08f4268493160c4") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleOffSignal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleOnSignal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStartSignal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_1.md") (path (named (kind package) (name "State Definition Example-1")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind initial-state) (ordinal 0))))) (kind initial-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (initialState (reference "off")))))
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::off"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::off_to_starting"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "off")) (transitionTarget (reference "starting")) (transitionTrigger (reference "VehicleStartSignal")))))
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::on"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::on_to_off"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "on")) (transitionTarget (reference "off")) (transitionTrigger (reference "VehicleOffSignal")))))
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::starting"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::starting_to_on"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "starting")) (transitionTarget (reference "on")) (transitionTrigger (reference "VehicleOnSignal")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/23_state_definition_example_1.md") (path (named (kind package) (name "State Definition Example-1")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0))
      (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::off")))))
    (reference (id (source (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::off_to_starting"))) (kind transitionSource) (ordinal 0))
      (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::off")))))
    (reference (id (source (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::off_to_starting"))) (kind transitionTarget) (ordinal 0))
      (authored-target "starting")
      (outcome (status resolved) (target (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::starting")))))
    (reference (id (source (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::off_to_starting"))) (kind transitionTrigger) (ordinal 0))
      (authored-target "VehicleStartSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStartSignal")))))
    (reference (id (source (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::on_to_off"))) (kind transitionSource) (ordinal 0))
      (authored-target "on")
      (outcome (status resolved) (target (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::on")))))
    (reference (id (source (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::on_to_off"))) (kind transitionTarget) (ordinal 0))
      (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::off")))))
    (reference (id (source (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::on_to_off"))) (kind transitionTrigger) (ordinal 0))
      (authored-target "VehicleOffSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleOffSignal")))))
    (reference (id (source (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::starting_to_on"))) (kind transitionSource) (ordinal 0))
      (authored-target "starting")
      (outcome (status resolved) (target (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::starting")))))
    (reference (id (source (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::starting_to_on"))) (kind transitionTarget) (ordinal 0))
      (authored-target "on")
      (outcome (status resolved) (target (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::on")))))
    (reference (id (source (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::starting_to_on"))) (kind transitionTrigger) (ordinal 0))
      (authored-target "VehicleOnSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleOnSignal")))))
  )
  (relationships
    (relationship (kind initialState) (source (node (document "memory://snapshot/23_state_definition_example_1.md") (path (named (kind package) (name "State Definition Example-1")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::off"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/23_state_definition_example_1.md") (path (named (kind package) (name "State Definition Example-1")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::off_to_starting"))) (target (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::off"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::off_to_starting"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::off_to_starting"))) (target (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::starting"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::off_to_starting"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::off_to_starting"))) (target (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStartSignal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::off_to_starting"))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::on_to_off"))) (target (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::on"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::on_to_off"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::on_to_off"))) (target (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::off"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::on_to_off"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::on_to_off"))) (target (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleOffSignal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::on_to_off"))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::starting_to_on"))) (target (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::starting"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::starting_to_on"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::starting_to_on"))) (target (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::on"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::starting_to_on"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::starting_to_on"))) (target (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleOnSignal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::starting_to_on"))) (kind transitionTrigger) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/23_state_definition_example_1.md") (range (start 7 14) (end 7 17)) (probe (position 7 14))
    (reference (id (source (node (document "memory://snapshot/23_state_definition_example_1.md") (path (named (kind package) (name "State Definition Example-1")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0) (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::off")))))
    )
  )
  (query (document "memory://snapshot/23_state_definition_example_1.md") (range (start 12 9) (end 12 12)) (probe (position 12 9))
    (reference (id (source (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::off_to_starting"))) (kind transitionSource) (ordinal 0) (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::off")))))
    )
  )
  (query (document "memory://snapshot/23_state_definition_example_1.md") (range (start 14 8) (end 14 16)) (probe (position 14 8))
    (reference (id (source (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::off_to_starting"))) (kind transitionTarget) (ordinal 0) (authored-target "starting")
      (outcome (status resolved) (target (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::starting")))))
    )
  )
  (query (document "memory://snapshot/23_state_definition_example_1.md") (range (start 13 10) (end 13 28)) (probe (position 13 10))
    (reference (id (source (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::off_to_starting"))) (kind transitionTrigger) (ordinal 0) (authored-target "VehicleStartSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStartSignal")))))
    )
  )
  (query (document "memory://snapshot/23_state_definition_example_1.md") (range (start 26 9) (end 26 11)) (probe (position 26 9))
    (reference (id (source (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::on_to_off"))) (kind transitionSource) (ordinal 0) (authored-target "on")
      (outcome (status resolved) (target (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::on")))))
    )
  )
  (query (document "memory://snapshot/23_state_definition_example_1.md") (range (start 28 8) (end 28 11)) (probe (position 28 8))
    (reference (id (source (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::on_to_off"))) (kind transitionTarget) (ordinal 0) (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::off")))))
    )
  )
  (query (document "memory://snapshot/23_state_definition_example_1.md") (range (start 27 10) (end 27 26)) (probe (position 27 10))
    (reference (id (source (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::on_to_off"))) (kind transitionTrigger) (ordinal 0) (authored-target "VehicleOffSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleOffSignal")))))
    )
  )
  (query (document "memory://snapshot/23_state_definition_example_1.md") (range (start 19 9) (end 19 17)) (probe (position 19 9))
    (reference (id (source (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::starting_to_on"))) (kind transitionSource) (ordinal 0) (authored-target "starting")
      (outcome (status resolved) (target (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::starting")))))
    )
  )
  (query (document "memory://snapshot/23_state_definition_example_1.md") (range (start 21 8) (end 21 10)) (probe (position 21 8))
    (reference (id (source (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::starting_to_on"))) (kind transitionTarget) (ordinal 0) (authored-target "on")
      (outcome (status resolved) (target (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::on")))))
    )
  )
  (query (document "memory://snapshot/23_state_definition_example_1.md") (range (start 20 10) (end 20 25)) (probe (position 20 10))
    (reference (id (source (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleStates::starting_to_on"))) (kind transitionTrigger) (ordinal 0) (authored-target "VehicleOnSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/23_state_definition_example_1.md") (qualified-name "State Definition Example-1::VehicleOnSignal")))))
    )
  )
)
~~~
