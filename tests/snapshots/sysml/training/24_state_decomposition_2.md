# META
~~~ini
description=SysML Training 24 (States): State Decomposition-2
type=file
~~~
# SOURCE
~~~sysml
package 'State Decomposition-1' {
	
	attribute def VehicleStartSignal;
	attribute def VehicleOnSignal;
	attribute def VehicleOffSignal;
	
	state def VehicleStates;
		
	state vehicleStates : VehicleStates parallel {
		
		state operationalStates {
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
		
		state healthStates { 
			/* ... */
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/24_state_decomposition_2.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:3bf71b9cad0732f9b7c83cfcfe1d2d4e82f6ace6cbb5dc29aaed59652b207b73"))
  (declarations
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::VehicleOffSignal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::VehicleOnSignal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::VehicleStartSignal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::VehicleStates"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates"))) (kind state) (membership (kind feature) (visibility default)) (facts (modifiers parallel)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleStates")))))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::healthStates"))) (kind state) (membership (kind feature) (visibility default)) (documentation (comment (text " ... "))))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind initial-state) (ordinal 0))))) (kind initial-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (initialState (reference "off")))))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 0))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionTarget (reference "starting")) (transitionTrigger (reference "VehicleStartSignal")))))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 1))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionTarget (reference "on")) (transitionTrigger (reference "VehicleOnSignal")))))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 2))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionTarget (reference "off")) (transitionTrigger (reference "VehicleOffSignal")))))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (succession (reference "starting")))))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (succession (reference "on")))))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (succession (reference "off")))))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::off"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::on"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::starting"))) (kind state) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleStates")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::VehicleStates")))))
    (reference (id (source (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0))
      (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::off")))))
    (reference (id (source (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0))
      (authored-target "starting")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::starting")))))
    (reference (id (source (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 1))))) (kind transitionTarget) (ordinal 0))
      (authored-target "on")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::on")))))
    (reference (id (source (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 2))))) (kind transitionTarget) (ordinal 0))
      (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::off")))))
    (reference (id (source (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTrigger) (ordinal 0))
      (authored-target "VehicleStartSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::VehicleStartSignal")))))
    (reference (id (source (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 1))))) (kind transitionTrigger) (ordinal 0))
      (authored-target "VehicleOnSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::VehicleOnSignal")))))
    (reference (id (source (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 2))))) (kind transitionTrigger) (ordinal 0))
      (authored-target "VehicleOffSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::VehicleOffSignal")))))
    (reference (id (source (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "starting")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::starting")))))
    (reference (id (source (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "on")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::on")))))
    (reference (id (source (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::off")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates"))) (target (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::VehicleStates"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind initialState) (source (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::off"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::starting"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 1))))) (target (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::on"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 1))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 2))))) (target (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::off"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 2))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::VehicleStartSignal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 1))))) (target (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::VehicleOnSignal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 1))))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 2))))) (target (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::VehicleOffSignal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 2))))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::starting"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind succession) (source (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::on"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind succession) (source (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::off"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::healthStates"))) (target (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates"))) (target (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 1))))) (target (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 2))))) (target (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 1))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 2))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::off"))) (target (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::on"))) (target (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::starting"))) (target (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::VehicleStates")))
      (subtype (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates")))
      (type (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::VehicleStates")) (provenance authored))
      (effective-type (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::VehicleStates")) (source direct))
      (supertype (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::VehicleStates")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::healthStates")))
      (featured-by (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates")))
    )
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates")))
      (featured-by (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates")))
    )
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind initial-state) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates")))
    )
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates")))
    )
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates")))
    )
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates")))
    )
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 1)))))
    )
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 2)))))
    )
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::off")))
      (featured-by (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates")))
    )
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::on")))
      (featured-by (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates")))
    )
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::starting")))
      (featured-by (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/24_state_decomposition_2.md") (range (start 8 23) (end 8 36)) (probe (position 8 23))
    (reference (id (source (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleStates")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::VehicleStates")))))
    )
  )
  (query (document "memory://snapshot/24_state_decomposition_2.md") (range (start 11 15) (end 11 18)) (probe (position 11 15))
    (reference (id (source (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0) (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::off")))))
    )
  )
  (query (document "memory://snapshot/24_state_decomposition_2.md") (range (start 15 9) (end 15 17)) (probe (position 15 9))
    (reference (id (source (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0) (authored-target "starting")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::starting")))))
    )
  )
  (query (document "memory://snapshot/24_state_decomposition_2.md") (range (start 19 9) (end 19 11)) (probe (position 19 9))
    (reference (id (source (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 1))))) (kind transitionTarget) (ordinal 0) (authored-target "on")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::on")))))
    )
  )
  (query (document "memory://snapshot/24_state_decomposition_2.md") (range (start 23 9) (end 23 12)) (probe (position 23 9))
    (reference (id (source (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 2))))) (kind transitionTarget) (ordinal 0) (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::off")))))
    )
  )
  (query (document "memory://snapshot/24_state_decomposition_2.md") (range (start 14 10) (end 14 28)) (probe (position 14 10))
    (reference (id (source (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTrigger) (ordinal 0) (authored-target "VehicleStartSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::VehicleStartSignal")))))
    )
  )
  (query (document "memory://snapshot/24_state_decomposition_2.md") (range (start 18 10) (end 18 25)) (probe (position 18 10))
    (reference (id (source (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 1))))) (kind transitionTrigger) (ordinal 0) (authored-target "VehicleOnSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::VehicleOnSignal")))))
    )
  )
  (query (document "memory://snapshot/24_state_decomposition_2.md") (range (start 22 10) (end 22 26)) (probe (position 22 10))
    (reference (id (source (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 2))))) (kind transitionTrigger) (ordinal 0) (authored-target "VehicleOffSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::VehicleOffSignal")))))
    )
  )
  (query (document "memory://snapshot/24_state_decomposition_2.md") (range (start 15 9) (end 15 17)) (probe (position 15 9))
    (reference (id (source (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "starting")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::starting")))))
    )
  )
  (query (document "memory://snapshot/24_state_decomposition_2.md") (range (start 19 9) (end 19 11)) (probe (position 19 9))
    (reference (id (source (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "on")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::on")))))
    )
  )
  (query (document "memory://snapshot/24_state_decomposition_2.md") (range (start 23 9) (end 23 12)) (probe (position 23 9))
    (reference (id (source (node (document "memory://snapshot/24_state_decomposition_2.md") (path (named (kind package) (name "State Decomposition-1")) (named (kind state) (name "vehicleStates")) (named (kind state) (name "operationalStates")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::off")))))
    )
  )
)
~~~
