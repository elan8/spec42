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
# EXPECTED
~~~
semantic.duplicate_name 'off'
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'off'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "State Decomposition-1"))) (name "State Decomposition-1") (declared-name "State Decomposition-1")
      (contains
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "State Decomposition-1::VehicleOffSignal"))) (name "VehicleOffSignal") (declared-name "VehicleOffSignal") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "State Decomposition-1::VehicleOnSignal"))) (name "VehicleOnSignal") (declared-name "VehicleOnSignal") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "State Decomposition-1::VehicleStartSignal"))) (name "VehicleStartSignal") (declared-name "VehicleStartSignal") (declared (properties (ordered false) (unique true))))
        (element (kind "state def") (id (node (document "d0") (qualified-name "State Decomposition-1::VehicleStates"))) (name "VehicleStates") (declared-name "VehicleStates"))
        (element (kind "state") (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))) (name "vehicleStates") (declared-name "vehicleStates") (declared (properties (composite true) (reference false)))
          (contains
            (element (kind "state") (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::healthStates"))) (name "healthStates") (declared-name "healthStates") (effective (featuring-type (node (document "d0") (qualified-name "State Decomposition-1::VehicleStates")))))
            (element (kind "state") (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates"))) (name "operationalStates") (declared-name "operationalStates") (effective (featuring-type (node (document "d0") (qualified-name "State Decomposition-1::VehicleStates"))))
              (contains
                (element (kind "action") (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::_entry"))) (name "entry") (declared-name "entry") (effective (featuring-type (node (document "d0") (qualified-name "State Decomposition-1::VehicleStates")))))
                (element (kind "state") (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::off"))) (name "off") (declared-name "off") (effective (featuring-type (node (document "d0") (qualified-name "State Decomposition-1::VehicleStates")))))
                (element (kind "state") (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::on"))) (name "on") (declared-name "on") (effective (featuring-type (node (document "d0") (qualified-name "State Decomposition-1::VehicleStates")))))
                (element (kind "state") (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::starting"))) (name "starting") (declared-name "starting") (effective (featuring-type (node (document "d0") (qualified-name "State Decomposition-1::VehicleStates")))))
                (element (kind "transition") (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::transition_operationalStates_to_off"))) (name "transition_operationalStates_to_off") (declared-name "transition_operationalStates_to_off") (effective (featuring-type (node (document "d0") (qualified-name "State Decomposition-1::VehicleStates"))))
                  (contains
                    (element (kind "transition trigger") (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::transition_operationalStates_to_off::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "State Decomposition-1::VehicleStates")))))
                  )
                )
                (element (kind "transition") (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::transition_operationalStates_to_on"))) (name "transition_operationalStates_to_on") (declared-name "transition_operationalStates_to_on") (effective (featuring-type (node (document "d0") (qualified-name "State Decomposition-1::VehicleStates"))))
                  (contains
                    (element (kind "transition trigger") (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::transition_operationalStates_to_on::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "State Decomposition-1::VehicleStates")))))
                  )
                )
                (element (kind "transition") (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::transition_operationalStates_to_starting"))) (name "transition_operationalStates_to_starting") (declared-name "transition_operationalStates_to_starting") (effective (featuring-type (node (document "d0") (qualified-name "State Decomposition-1::VehicleStates"))))
                  (contains
                    (element (kind "transition trigger") (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::transition_operationalStates_to_starting::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "State Decomposition-1::VehicleStates")))))
                  )
                )
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (initialState (status resolved) (from (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates"))) (to (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::off"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates"))) (to (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::off"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates"))) (to (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::on"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates"))) (to (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::operationalStates::starting"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))) (to (node (document "d0") (qualified-name "State Decomposition-1::VehicleStates"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
