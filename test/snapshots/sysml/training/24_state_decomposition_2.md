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
  (document "24_state_decomposition_2.md"
    (diagnostics
    )
  )
)
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwAttribute,KwDef,Ident,Semicolon,
KwAttribute,KwDef,Ident,Semicolon,
KwAttribute,KwDef,Ident,Semicolon,
KwState,KwDef,Ident,Semicolon,
KwState,Ident,Colon,Ident,KwParallel,OpenCurly,
KwState,Ident,OpenCurly,
KwEntry,Semicolon,KwThen,Ident,Semicolon,
KwState,Ident,Semicolon,
KwAccept,Ident,
KwThen,Ident,Semicolon,
KwState,Ident,Semicolon,
KwAccept,Ident,
KwThen,Ident,Semicolon,
KwState,Ident,Semicolon,
KwAccept,Ident,
KwThen,Ident,Semicolon,
CloseCurly,
KwState,Ident,OpenCurly,
RegularComment,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''State Decomposition-1''
    (attribute_def 'VehicleStartSignal')
    (attribute_def 'VehicleOnSignal')
    (attribute_def 'VehicleOffSignal')
    (state_def 'VehicleStates')
    (state_usage parallel 'vehicleStates' : 'VehicleStates'
      (state_usage 'operationalStates'
        (entry_action)
        (source_succession
          (default_ref_usage 'off'))
        (state_usage 'off')
        (target_transition)
        (state_usage 'starting')
        (target_transition)
        (state_usage 'on')
        (target_transition))
      (state_usage 'healthStates'
        (comment)))))
~~~
# EXPECTED
~~~
semantic.duplicate_name 'off'
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'off'
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "d06e07e745d560c61f193d5c6430168aa76c163e24f8d071db9695f2bd06d26c") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "State Decomposition-1"))) (kind "package") (name "State Decomposition-1") (declared-name "State Decomposition-1") (range (start (line 0) (character 0)) (end (line 0) (character 509))))
    (element (id (node (document "d0") (qualified-name "State Decomposition-1::VehicleOffSignal"))) (kind "attribute def") (name "VehicleOffSignal") (declared-name "VehicleOffSignal") (range (start (line 4) (character 1)) (end (line 4) (character 32))) (parent (node (document "d0") (qualified-name "State Decomposition-1"))))
    (element (id (node (document "d0") (qualified-name "State Decomposition-1::VehicleOnSignal"))) (kind "attribute def") (name "VehicleOnSignal") (declared-name "VehicleOnSignal") (range (start (line 3) (character 1)) (end (line 3) (character 31))) (parent (node (document "d0") (qualified-name "State Decomposition-1"))))
    (element (id (node (document "d0") (qualified-name "State Decomposition-1::VehicleStartSignal"))) (kind "attribute def") (name "VehicleStartSignal") (declared-name "VehicleStartSignal") (range (start (line 2) (character 1)) (end (line 2) (character 34))) (parent (node (document "d0") (qualified-name "State Decomposition-1"))))
    (element (id (node (document "d0") (qualified-name "State Decomposition-1::VehicleStates"))) (kind "state def") (name "VehicleStates") (declared-name "VehicleStates") (range (start (line 6) (character 1)) (end (line 6) (character 25))) (parent (node (document "d0") (qualified-name "State Decomposition-1"))))
    (element (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))) (kind "state") (name "vehicleStates") (declared-name "vehicleStates") (range (start (line 8) (character 1)) (end (line 8) (character 338))) (parent (node (document "d0") (qualified-name "State Decomposition-1"))) (authored (membership (kind Feature)) (relationships (typing (reference "VehicleStates") (range none)))))
    (element (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::healthStates"))) (kind "state") (name "healthStates") (declared-name "healthStates") (range (start (line 26) (character 2)) (end (line 26) (character 40))) (parent (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))))
    (element (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates"))) (kind "state") (name "operationalStates") (declared-name "operationalStates") (range (start (line 10) (character 2)) (end (line 10) (character 240))) (parent (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))) (authored (membership (kind Feature)) (relationships (transition (reference "State Decomposition-1::vehicleStates::operationalStates::starting") (range none)) (transition (reference "State Decomposition-1::vehicleStates::operationalStates::on") (range none)) (transition (reference "State Decomposition-1::vehicleStates::operationalStates::off") (range none)) (initial-state (reference "State Decomposition-1::vehicleStates::operationalStates::off") (range none)))))
    (element (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::_entry"))) (kind "action") (name "entry") (declared-name "entry") (range (start (line 11) (character 3)) (end (line 11) (character 9))) (parent (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates"))))
    (element (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::off"))) (kind "state") (name "off") (declared-name "off") (range (start (line 13) (character 3)) (end (line 13) (character 13))) (parent (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates"))))
    (element (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::on"))) (kind "state") (name "on") (declared-name "on") (range (start (line 21) (character 3)) (end (line 21) (character 12))) (parent (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates"))))
    (element (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::starting"))) (kind "state") (name "starting") (declared-name "starting") (range (start (line 17) (character 3)) (end (line 17) (character 18))) (parent (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates"))))
    (element (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::transition_operationalStates_to_off"))) (kind "transition") (name "transition_operationalStates_to_off") (declared-name "transition_operationalStates_to_off") (range (start (line 22) (character 3)) (end (line 22) (character 40))) (parent (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates"))))
    (element (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::transition_operationalStates_to_off::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 22) (character 3)) (end (line 22) (character 40))) (parent (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::transition_operationalStates_to_off"))))
    (element (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::transition_operationalStates_to_on"))) (kind "transition") (name "transition_operationalStates_to_on") (declared-name "transition_operationalStates_to_on") (range (start (line 18) (character 3)) (end (line 18) (character 38))) (parent (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates"))))
    (element (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::transition_operationalStates_to_on::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 18) (character 3)) (end (line 18) (character 38))) (parent (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::transition_operationalStates_to_on"))))
    (element (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::transition_operationalStates_to_starting"))) (kind "transition") (name "transition_operationalStates_to_starting") (declared-name "transition_operationalStates_to_starting") (range (start (line 14) (character 3)) (end (line 14) (character 48))) (parent (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates"))))
    (element (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::transition_operationalStates_to_starting::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 14) (character 3)) (end (line 14) (character 48))) (parent (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::transition_operationalStates_to_starting"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleStates") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "State Decomposition-1::VehicleStates")))))
    (reference (id (source (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates"))) (kind transitionSource) (ordinal 0)) (authored-target "State Decomposition-1::vehicleStates::operationalStates::starting") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::starting")))))
    (reference (id (source (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates"))) (kind transitionSource) (ordinal 1)) (authored-target "State Decomposition-1::vehicleStates::operationalStates::on") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::on")))))
    (reference (id (source (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates"))) (kind transitionSource) (ordinal 2)) (authored-target "State Decomposition-1::vehicleStates::operationalStates::off") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::off")))))
    (reference (id (source (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates"))) (kind initialStateSource) (ordinal 0)) (authored-target "State Decomposition-1::vehicleStates::operationalStates::off") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::off")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))) (target (node (document "d0") (qualified-name "State Decomposition-1::VehicleStates"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates"))) (target (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::off"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates"))) (kind transitionSource) (ordinal 2)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates"))) (target (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::on"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates"))) (kind transitionSource) (ordinal 1)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates"))) (target (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::starting"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind initialState) (source (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates"))) (target (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::off"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates"))) (kind initialStateSource) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
