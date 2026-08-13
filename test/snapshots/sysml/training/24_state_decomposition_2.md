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
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 11 3) (end 11 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 11 10) (end 11 10))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 14 3) (end 14 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 18 3) (end 18 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 22 3) (end 22 3))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:3bf71b9cad0732f9b7c83cfcfe1d2d4e82f6ace6cbb5dc29aaed59652b207b73") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::VehicleOffSignal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::VehicleOnSignal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::VehicleStartSignal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::VehicleStates"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates"))) (kind state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleStates"))))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::healthStates"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::off"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::on"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::starting"))) (kind state) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleStates")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::VehicleStates")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates"))) (target (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::VehicleStates"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/24_state_decomposition_2.md") (qualified-name "State Decomposition-1::vehicleStates"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
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
~~~
