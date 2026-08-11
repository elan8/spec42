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
  (document "24_state_decomposition_1.md"
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
KwState,Ident,Colon,Ident,OpenCurly,
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
    (state_usage 'vehicleStates' : 'VehicleStates'
      (entry_action)
      (source_succession
        (default_ref_usage 'off'))
      (state_usage 'off')
      (target_transition)
      (state_usage 'starting')
      (target_transition)
      (state_usage 'on')
      (target_transition))))
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "5a24abc7516f9df34ab166a582652cb45cfb2b48a9b0529842792dbbcf2b5865") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "State Decomposition-1"))) (kind "package") (name "State Decomposition-1") (declared-name "State Decomposition-1") (range (start (line 0) (character 0)) (end (line 0) (character 408))))
    (element (id (node (document "d0") (qualified-name "State Decomposition-1::VehicleOffSignal"))) (kind "attribute def") (name "VehicleOffSignal") (declared-name "VehicleOffSignal") (range (start (line 4) (character 1)) (end (line 4) (character 32))) (parent (node (document "d0") (qualified-name "State Decomposition-1"))))
    (element (id (node (document "d0") (qualified-name "State Decomposition-1::VehicleOnSignal"))) (kind "attribute def") (name "VehicleOnSignal") (declared-name "VehicleOnSignal") (range (start (line 3) (character 1)) (end (line 3) (character 31))) (parent (node (document "d0") (qualified-name "State Decomposition-1"))))
    (element (id (node (document "d0") (qualified-name "State Decomposition-1::VehicleStartSignal"))) (kind "attribute def") (name "VehicleStartSignal") (declared-name "VehicleStartSignal") (range (start (line 2) (character 1)) (end (line 2) (character 34))) (parent (node (document "d0") (qualified-name "State Decomposition-1"))))
    (element (id (node (document "d0") (qualified-name "State Decomposition-1::VehicleStates"))) (kind "state def") (name "VehicleStates") (declared-name "VehicleStates") (range (start (line 6) (character 1)) (end (line 6) (character 25))) (parent (node (document "d0") (qualified-name "State Decomposition-1"))))
    (element (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))) (kind "state") (name "vehicleStates") (declared-name "vehicleStates") (range (start (line 8) (character 1)) (end (line 8) (character 237))) (parent (node (document "d0") (qualified-name "State Decomposition-1"))) (authored (membership (kind Feature)) (relationships (typing (reference "VehicleStates") (range none)) (transition (reference "State Decomposition-1::vehicleStates::starting") (range none)) (transition (reference "State Decomposition-1::vehicleStates::on") (range none)) (transition (reference "State Decomposition-1::vehicleStates::off") (range none)) (initial-state (reference "State Decomposition-1::vehicleStates::off") (range none)))))
    (element (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::_entry"))) (kind "action") (name "entry") (declared-name "entry") (range (start (line 9) (character 2)) (end (line 9) (character 8))) (parent (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))))
    (element (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::off"))) (kind "state") (name "off") (declared-name "off") (range (start (line 11) (character 2)) (end (line 11) (character 12))) (parent (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))))
    (element (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::on"))) (kind "state") (name "on") (declared-name "on") (range (start (line 19) (character 2)) (end (line 19) (character 11))) (parent (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))))
    (element (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::starting"))) (kind "state") (name "starting") (declared-name "starting") (range (start (line 15) (character 2)) (end (line 15) (character 17))) (parent (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))))
    (element (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::transition_vehicleStates_to_off"))) (kind "transition") (name "transition_vehicleStates_to_off") (declared-name "transition_vehicleStates_to_off") (range (start (line 20) (character 2)) (end (line 20) (character 38))) (parent (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))))
    (element (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::transition_vehicleStates_to_off::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 20) (character 2)) (end (line 20) (character 38))) (parent (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::transition_vehicleStates_to_off"))))
    (element (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::transition_vehicleStates_to_on"))) (kind "transition") (name "transition_vehicleStates_to_on") (declared-name "transition_vehicleStates_to_on") (range (start (line 16) (character 2)) (end (line 16) (character 36))) (parent (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))))
    (element (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::transition_vehicleStates_to_on::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 16) (character 2)) (end (line 16) (character 36))) (parent (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::transition_vehicleStates_to_on"))))
    (element (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::transition_vehicleStates_to_starting"))) (kind "transition") (name "transition_vehicleStates_to_starting") (declared-name "transition_vehicleStates_to_starting") (range (start (line 12) (character 2)) (end (line 12) (character 46))) (parent (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))))
    (element (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::transition_vehicleStates_to_starting::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 12) (character 2)) (end (line 12) (character 46))) (parent (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::transition_vehicleStates_to_starting"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleStates") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "State Decomposition-1::VehicleStates")))))
    (reference (id (source (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))) (kind transitionSource) (ordinal 0)) (authored-target "State Decomposition-1::vehicleStates::starting") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::starting")))))
    (reference (id (source (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))) (kind transitionSource) (ordinal 1)) (authored-target "State Decomposition-1::vehicleStates::on") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::on")))))
    (reference (id (source (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))) (kind transitionSource) (ordinal 2)) (authored-target "State Decomposition-1::vehicleStates::off") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::off")))))
    (reference (id (source (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))) (kind initialStateSource) (ordinal 0)) (authored-target "State Decomposition-1::vehicleStates::off") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::off")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))) (target (node (document "d0") (qualified-name "State Decomposition-1::VehicleStates"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))) (target (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::off"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))) (kind transitionSource) (ordinal 2)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))) (target (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::on"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))) (kind transitionSource) (ordinal 1)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))) (target (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::starting"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind initialState) (source (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))) (target (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::off"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))) (kind initialStateSource) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
