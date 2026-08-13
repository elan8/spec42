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
  (document "memory://snapshot/23_state_definition_example_2.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 7 2) (end 7 8))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 10 2) (end 10 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 14 2) (end 14 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 18 2) (end 18 2))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:3439a623d2b789f94922d893e1db86bad5ac569ffd85180eb3e8a4dbc9a62cfd") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleOffSignal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleOnSignal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStartSignal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_2.md") (anonymous (kind initial-state) (ordinal 0))))) (kind initial-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (initialState (reference "off"))))
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates::off"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates::on"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates::starting"))) (kind state) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/23_state_definition_example_2.md") (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0))
      (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates::off")))))
  )
  (relationships
    (relationship (kind initialState) (source (node (document "memory://snapshot/23_state_definition_example_2.md") (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates::off"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/23_state_definition_example_2.md") (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/23_state_definition_example_2.md") (range (start 7 14) (end 7 17)) (probe (position 7 14))
    (reference (id (source (node (document "memory://snapshot/23_state_definition_example_2.md") (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0) (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates::off")))))
  )
)
~~~
