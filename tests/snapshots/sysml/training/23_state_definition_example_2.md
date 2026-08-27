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
        (severity information)
        (code "missing_final_state")
        (source "semantic")
        (range (start 6 1) (end 20 2))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:3439a623d2b789f94922d893e1db86bad5ac569ffd85180eb3e8a4dbc9a62cfd"))
  (declarations
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleOffSignal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleOnSignal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStartSignal"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind initial-state) (ordinal 0))))) (kind initial-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (initialState (reference "off")))))
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 0))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionTarget (reference "starting")) (transitionTrigger (reference "VehicleStartSignal")))))
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 1))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionTarget (reference "on")) (transitionTrigger (reference "VehicleOnSignal")))))
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 2))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionTarget (reference "off")) (transitionTrigger (reference "VehicleOffSignal")))))
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (succession (reference "starting")))))
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (succession (reference "on")))))
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (succession (reference "off")))))
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates::off"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates::on"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates::starting"))) (kind state) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0))
      (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates::off")))))
    (reference (id (source (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0))
      (authored-target "starting")
      (outcome (status resolved) (target (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates::starting")))))
    (reference (id (source (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 1))))) (kind transitionTarget) (ordinal 0))
      (authored-target "on")
      (outcome (status resolved) (target (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates::on")))))
    (reference (id (source (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 2))))) (kind transitionTarget) (ordinal 0))
      (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates::off")))))
    (reference (id (source (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTrigger) (ordinal 0))
      (authored-target "VehicleStartSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStartSignal")))))
    (reference (id (source (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 1))))) (kind transitionTrigger) (ordinal 0))
      (authored-target "VehicleOnSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleOnSignal")))))
    (reference (id (source (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 2))))) (kind transitionTrigger) (ordinal 0))
      (authored-target "VehicleOffSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleOffSignal")))))
    (reference (id (source (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "starting")
      (outcome (status resolved) (target (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates::starting")))))
    (reference (id (source (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "on")
      (outcome (status resolved) (target (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates::on")))))
    (reference (id (source (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates::off")))))
  )
  (relationships
    (relationship (kind initialState) (source (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates::off"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates::starting"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 1))))) (target (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates::on"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 1))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 2))))) (target (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates::off"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 2))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStartSignal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 1))))) (target (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleOnSignal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 1))))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 2))))) (target (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleOffSignal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 2))))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates::starting"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind succession) (source (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates::on"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind succession) (source (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates::off"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 1))))) (target (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 2))))) (target (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 1))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 2))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates::off"))) (target (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates::on"))) (target (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates::starting"))) (target (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind initial-state) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates")))
    )
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates")))
    )
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates")))
    )
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates")))
    )
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 1)))))
    )
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 2)))))
    )
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates::off")))
      (featured-by (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates")))
    )
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates::on")))
      (featured-by (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates")))
    )
    (declaration (id (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates::starting")))
      (featured-by (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/23_state_definition_example_2.md") (range (start 7 14) (end 7 17)) (probe (position 7 14))
    (reference (id (source (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0) (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates::off")))))
    )
  )
  (query (document "memory://snapshot/23_state_definition_example_2.md") (range (start 11 8) (end 11 16)) (probe (position 11 8))
    (reference (id (source (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0) (authored-target "starting")
      (outcome (status resolved) (target (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates::starting")))))
    )
  )
  (query (document "memory://snapshot/23_state_definition_example_2.md") (range (start 15 8) (end 15 10)) (probe (position 15 8))
    (reference (id (source (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 1))))) (kind transitionTarget) (ordinal 0) (authored-target "on")
      (outcome (status resolved) (target (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates::on")))))
    )
  )
  (query (document "memory://snapshot/23_state_definition_example_2.md") (range (start 19 8) (end 19 11)) (probe (position 19 8))
    (reference (id (source (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 2))))) (kind transitionTarget) (ordinal 0) (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates::off")))))
    )
  )
  (query (document "memory://snapshot/23_state_definition_example_2.md") (range (start 10 9) (end 10 27)) (probe (position 10 9))
    (reference (id (source (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTrigger) (ordinal 0) (authored-target "VehicleStartSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStartSignal")))))
    )
  )
  (query (document "memory://snapshot/23_state_definition_example_2.md") (range (start 14 9) (end 14 24)) (probe (position 14 9))
    (reference (id (source (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 1))))) (kind transitionTrigger) (ordinal 0) (authored-target "VehicleOnSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleOnSignal")))))
    )
  )
  (query (document "memory://snapshot/23_state_definition_example_2.md") (range (start 18 9) (end 18 25)) (probe (position 18 9))
    (reference (id (source (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 2))))) (kind transitionTrigger) (ordinal 0) (authored-target "VehicleOffSignal")
      (outcome (status resolved) (target (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleOffSignal")))))
    )
  )
  (query (document "memory://snapshot/23_state_definition_example_2.md") (range (start 11 8) (end 11 16)) (probe (position 11 8))
    (reference (id (source (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "starting")
      (outcome (status resolved) (target (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates::starting")))))
    )
  )
  (query (document "memory://snapshot/23_state_definition_example_2.md") (range (start 15 8) (end 15 10)) (probe (position 15 8))
    (reference (id (source (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "on")
      (outcome (status resolved) (target (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates::on")))))
    )
  )
  (query (document "memory://snapshot/23_state_definition_example_2.md") (range (start 19 8) (end 19 11)) (probe (position 19 8))
    (reference (id (source (node (document "memory://snapshot/23_state_definition_example_2.md") (path (named (kind package) (name "State Definition Example-2")) (named (kind state-def) (name "VehicleStates")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/23_state_definition_example_2.md") (qualified-name "State Definition Example-2::VehicleStates::off")))))
    )
  )
)
~~~
