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
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 9 2) (end 9 8))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 12 2) (end 12 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 16 2) (end 16 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 20 2) (end 20 2))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:ba8aa97927d5b322080ff94802bac65694d6775ed207686e135add1006126b83") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::VehicleOffSignal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::VehicleOnSignal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::VehicleStartSignal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::VehicleStates"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::vehicleStates"))) (kind state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleStates"))))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_1.md") (anonymous (kind initial-state) (ordinal 0))))) (kind initial-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (initialState (reference "off"))))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::vehicleStates::off"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::vehicleStates::on"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::vehicleStates::starting"))) (kind state) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::vehicleStates"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleStates")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::VehicleStates")))))
    (reference (id (source (node (document "memory://snapshot/24_state_decomposition_1.md") (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0))
      (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::vehicleStates::off")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::vehicleStates"))) (target (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::VehicleStates"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::vehicleStates"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind initialState) (source (node (document "memory://snapshot/24_state_decomposition_1.md") (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::vehicleStates::off"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/24_state_decomposition_1.md") (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0)))
  )
  (evaluation
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
    (reference (id (source (node (document "memory://snapshot/24_state_decomposition_1.md") (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0) (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/24_state_decomposition_1.md") (qualified-name "State Decomposition-1::vehicleStates::off")))))
  )
)
~~~
