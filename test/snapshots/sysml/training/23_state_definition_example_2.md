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
  (document "23_state_definition_example_2.md"
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
KwState,KwDef,Ident,OpenCurly,
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
  (package_def ''State Definition Example-2''
    (attribute_def 'VehicleStartSignal')
    (attribute_def 'VehicleOnSignal')
    (attribute_def 'VehicleOffSignal')
    (state_def 'VehicleStates'
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "acf2c44b4042c03244deb95157bff0cccfc44b596775ad752d7be226cc8e54ce") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "State Definition Example-2"))) (kind "package") (name "State Definition Example-2") (declared-name "State Definition Example-2") (range (start (line 0) (character 0)) (end (line 0) (character 373))))
    (element (id (node (document "d0") (qualified-name "State Definition Example-2::VehicleOffSignal"))) (kind "attribute def") (name "VehicleOffSignal") (declared-name "VehicleOffSignal") (range (start (line 4) (character 1)) (end (line 4) (character 32))) (parent (node (document "d0") (qualified-name "State Definition Example-2"))))
    (element (id (node (document "d0") (qualified-name "State Definition Example-2::VehicleOnSignal"))) (kind "attribute def") (name "VehicleOnSignal") (declared-name "VehicleOnSignal") (range (start (line 3) (character 1)) (end (line 3) (character 31))) (parent (node (document "d0") (qualified-name "State Definition Example-2"))))
    (element (id (node (document "d0") (qualified-name "State Definition Example-2::VehicleStartSignal"))) (kind "attribute def") (name "VehicleStartSignal") (declared-name "VehicleStartSignal") (range (start (line 2) (character 1)) (end (line 2) (character 34))) (parent (node (document "d0") (qualified-name "State Definition Example-2"))))
    (element (id (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates"))) (kind "state def") (name "VehicleStates") (declared-name "VehicleStates") (range (start (line 6) (character 1)) (end (line 6) (character 225))) (parent (node (document "d0") (qualified-name "State Definition Example-2"))) (authored (membership (kind Owning)) (relationships (transition (reference "State Definition Example-2::VehicleStates::starting") (range none)) (transition (reference "State Definition Example-2::VehicleStates::on") (range none)) (transition (reference "State Definition Example-2::VehicleStates::off") (range none)) (initial-state (reference "State Definition Example-2::VehicleStates::off") (range none)))))
    (element (id (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates::_entry"))) (kind "action") (name "entry") (declared-name "entry") (range (start (line 7) (character 2)) (end (line 7) (character 8))) (parent (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates"))))
    (element (id (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates::off"))) (kind "state") (name "off") (declared-name "off") (range (start (line 9) (character 2)) (end (line 9) (character 12))) (parent (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates"))))
    (element (id (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates::on"))) (kind "state") (name "on") (declared-name "on") (range (start (line 17) (character 2)) (end (line 17) (character 11))) (parent (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates"))))
    (element (id (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates::starting"))) (kind "state") (name "starting") (declared-name "starting") (range (start (line 13) (character 2)) (end (line 13) (character 17))) (parent (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates"))))
    (element (id (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates::transition_VehicleStates_to_off"))) (kind "transition") (name "transition_VehicleStates_to_off") (declared-name "transition_VehicleStates_to_off") (range (start (line 18) (character 2)) (end (line 18) (character 38))) (parent (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates"))))
    (element (id (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates::transition_VehicleStates_to_off::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 18) (character 2)) (end (line 18) (character 38))) (parent (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates::transition_VehicleStates_to_off"))))
    (element (id (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates::transition_VehicleStates_to_on"))) (kind "transition") (name "transition_VehicleStates_to_on") (declared-name "transition_VehicleStates_to_on") (range (start (line 14) (character 2)) (end (line 14) (character 36))) (parent (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates"))))
    (element (id (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates::transition_VehicleStates_to_on::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 14) (character 2)) (end (line 14) (character 36))) (parent (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates::transition_VehicleStates_to_on"))))
    (element (id (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates::transition_VehicleStates_to_starting"))) (kind "transition") (name "transition_VehicleStates_to_starting") (declared-name "transition_VehicleStates_to_starting") (range (start (line 10) (character 2)) (end (line 10) (character 46))) (parent (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates"))))
    (element (id (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates::transition_VehicleStates_to_starting::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 10) (character 2)) (end (line 10) (character 46))) (parent (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates::transition_VehicleStates_to_starting"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates"))) (kind transitionSource) (ordinal 0)) (authored-target "State Definition Example-2::VehicleStates::starting") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates::starting")))))
    (reference (id (source (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates"))) (kind transitionSource) (ordinal 1)) (authored-target "State Definition Example-2::VehicleStates::on") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates::on")))))
    (reference (id (source (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates"))) (kind transitionSource) (ordinal 2)) (authored-target "State Definition Example-2::VehicleStates::off") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates::off")))))
    (reference (id (source (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates"))) (kind initialStateSource) (ordinal 0)) (authored-target "State Definition Example-2::VehicleStates::off") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates::off")))))
  )
  (relationships
    (relationship (kind transition) (source (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates"))) (target (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates::off"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates"))) (kind transitionSource) (ordinal 2)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates"))) (target (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates::on"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates"))) (kind transitionSource) (ordinal 1)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates"))) (target (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates::starting"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind initialState) (source (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates"))) (target (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates::off"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "State Definition Example-2::VehicleStates"))) (kind initialStateSource) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
