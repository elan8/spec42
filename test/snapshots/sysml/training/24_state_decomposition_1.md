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
  (document "memory://snapshot/24_state_decomposition_1.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:ba8aa97927d5b322080ff94802bac65694d6775ed207686e135add1006126b83") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::VehicleOffSignal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::VehicleOnSignal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::VehicleStartSignal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::VehicleStates"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::vehicleStates"))) (kind state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleStates"))))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_1.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (anonymous (kind initial-state) (ordinal 0)))))) (kind initial-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (initialState (reference "off"))))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_1.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 0)))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionTarget (reference "starting")) (transitionTrigger (reference "VehicleStartSignal"))))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_1.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 1)))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionTarget (reference "on")) (transitionTrigger (reference "VehicleOnSignal"))))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_1.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 2)))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionTarget (reference "off")) (transitionTrigger (reference "VehicleOffSignal"))))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::vehicleStates::off"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::vehicleStates::on"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::vehicleStates::starting"))) (kind state) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::vehicleStates"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleStates")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::VehicleStates")))))
    (reference (id (source (node (document "memory://snapshot/24_state_decomposition_1.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (anonymous (kind initial-state) (ordinal 0)))))) (kind initialState) (ordinal 0))
      (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::vehicleStates::off")))))
    (reference (id (source (node (document "memory://snapshot/24_state_decomposition_1.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 0)))))) (kind transitionTarget) (ordinal 0))
      (authored-target "starting")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::vehicleStates::starting")))))
    (reference (id (source (node (document "memory://snapshot/24_state_decomposition_1.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 1)))))) (kind transitionTarget) (ordinal 0))
      (authored-target "on")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::vehicleStates::on")))))
    (reference (id (source (node (document "memory://snapshot/24_state_decomposition_1.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 2)))))) (kind transitionTarget) (ordinal 0))
      (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::vehicleStates::off")))))
    (reference (id (source (node (document "memory://snapshot/24_state_decomposition_1.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 0)))))) (kind transitionTrigger) (ordinal 0))
      (authored-target "VehicleStartSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::VehicleStartSignal")))))
    (reference (id (source (node (document "memory://snapshot/24_state_decomposition_1.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 1)))))) (kind transitionTrigger) (ordinal 0))
      (authored-target "VehicleOnSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::VehicleOnSignal")))))
    (reference (id (source (node (document "memory://snapshot/24_state_decomposition_1.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 2)))))) (kind transitionTrigger) (ordinal 0))
      (authored-target "VehicleOffSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::VehicleOffSignal")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::vehicleStates"))) (target (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::VehicleStates"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::vehicleStates"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind initialState) (source (node (document "memory://snapshot/24_state_decomposition_1.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (anonymous (kind initial-state) (ordinal 0)))))) (target (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::vehicleStates::off"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/24_state_decomposition_1.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (anonymous (kind initial-state) (ordinal 0)))))) (kind initialState) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/24_state_decomposition_1.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 0)))))) (target (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::vehicleStates::starting"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/24_state_decomposition_1.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 0)))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/24_state_decomposition_1.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 1)))))) (target (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::vehicleStates::on"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/24_state_decomposition_1.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 1)))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/24_state_decomposition_1.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 2)))))) (target (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::vehicleStates::off"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/24_state_decomposition_1.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 2)))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/24_state_decomposition_1.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 0)))))) (target (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::VehicleStartSignal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/24_state_decomposition_1.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 0)))))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/24_state_decomposition_1.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 1)))))) (target (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::VehicleOnSignal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/24_state_decomposition_1.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 1)))))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/24_state_decomposition_1.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 2)))))) (target (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::VehicleOffSignal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/24_state_decomposition_1.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 2)))))) (kind transitionTrigger) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::vehicleStates")))
      (supertype (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::VehicleStates")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/24_state_decomposition_1.md") (range (start 8 23) (end 8 36)) (probe (position 8 23))
    (reference (id (source (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::vehicleStates"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleStates")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::VehicleStates")))))
  )
  (query (document "memory://snapshot/24_state_decomposition_1.md") (range (start 9 14) (end 9 17)) (probe (position 9 14))
    (reference (id (source (node (document "memory://snapshot/24_state_decomposition_1.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (anonymous (kind initial-state) (ordinal 0)))))) (kind initialState) (ordinal 0) (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::vehicleStates::off")))))
  )
  (query (document "memory://snapshot/24_state_decomposition_1.md") (range (start 13 8) (end 13 16)) (probe (position 13 8))
    (reference (id (source (node (document "memory://snapshot/24_state_decomposition_1.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 0)))))) (kind transitionTarget) (ordinal 0) (authored-target "starting")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::vehicleStates::starting")))))
  )
  (query (document "memory://snapshot/24_state_decomposition_1.md") (range (start 17 8) (end 17 10)) (probe (position 17 8))
    (reference (id (source (node (document "memory://snapshot/24_state_decomposition_1.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 1)))))) (kind transitionTarget) (ordinal 0) (authored-target "on")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::vehicleStates::on")))))
  )
  (query (document "memory://snapshot/24_state_decomposition_1.md") (range (start 21 8) (end 21 11)) (probe (position 21 8))
    (reference (id (source (node (document "memory://snapshot/24_state_decomposition_1.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 2)))))) (kind transitionTarget) (ordinal 0) (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::vehicleStates::off")))))
  )
  (query (document "memory://snapshot/24_state_decomposition_1.md") (range (start 12 9) (end 12 27)) (probe (position 12 9))
    (reference (id (source (node (document "memory://snapshot/24_state_decomposition_1.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 0)))))) (kind transitionTrigger) (ordinal 0) (authored-target "VehicleStartSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::VehicleStartSignal")))))
  )
  (query (document "memory://snapshot/24_state_decomposition_1.md") (range (start 16 9) (end 16 24)) (probe (position 16 9))
    (reference (id (source (node (document "memory://snapshot/24_state_decomposition_1.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 1)))))) (kind transitionTrigger) (ordinal 0) (authored-target "VehicleOnSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::VehicleOnSignal")))))
  )
  (query (document "memory://snapshot/24_state_decomposition_1.md") (range (start 20 9) (end 20 25)) (probe (position 20 9))
    (reference (id (source (node (document "memory://snapshot/24_state_decomposition_1.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (anonymous (kind transition) (ordinal 2)))))) (kind transitionTrigger) (ordinal 0) (authored-target "VehicleOffSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::VehicleOffSignal")))))
  )
)
~~~
