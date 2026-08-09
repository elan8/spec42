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
# FORMAT
~~~sysml
package 'State Decomposition-1' {
    attribute def VehicleStartSignal;
    attribute def VehicleOnSignal;
    attribute def VehicleOffSignal;

    state def VehicleStates;

    state vehicleStates : VehicleStates {
        entry;
        then off;

        state off;
        accept VehicleStartSignal then starting;

        state starting;
        accept VehicleOnSignal then on;

        state on;
        accept VehicleOffSignal then off;
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
            (element (kind "action") (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::_entry"))) (name "entry") (declared-name "entry") (effective (featuring-type (node (document "d0") (qualified-name "State Decomposition-1::VehicleStates")))))
            (element (kind "state") (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::off"))) (name "off") (declared-name "off") (effective (featuring-type (node (document "d0") (qualified-name "State Decomposition-1::VehicleStates")))))
            (element (kind "state") (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::on"))) (name "on") (declared-name "on") (effective (featuring-type (node (document "d0") (qualified-name "State Decomposition-1::VehicleStates")))))
            (element (kind "state") (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::starting"))) (name "starting") (declared-name "starting") (effective (featuring-type (node (document "d0") (qualified-name "State Decomposition-1::VehicleStates")))))
            (element (kind "transition") (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::transition_vehicleStates_to_off"))) (name "transition_vehicleStates_to_off") (declared-name "transition_vehicleStates_to_off") (effective (featuring-type (node (document "d0") (qualified-name "State Decomposition-1::VehicleStates"))))
              (contains
                (element (kind "transition trigger") (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::transition_vehicleStates_to_off::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "State Decomposition-1::VehicleStates")))))
              )
            )
            (element (kind "transition") (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::transition_vehicleStates_to_on"))) (name "transition_vehicleStates_to_on") (declared-name "transition_vehicleStates_to_on") (effective (featuring-type (node (document "d0") (qualified-name "State Decomposition-1::VehicleStates"))))
              (contains
                (element (kind "transition trigger") (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::transition_vehicleStates_to_on::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "State Decomposition-1::VehicleStates")))))
              )
            )
            (element (kind "transition") (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::transition_vehicleStates_to_starting"))) (name "transition_vehicleStates_to_starting") (declared-name "transition_vehicleStates_to_starting") (effective (featuring-type (node (document "d0") (qualified-name "State Decomposition-1::VehicleStates"))))
              (contains
                (element (kind "transition trigger") (id (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::transition_vehicleStates_to_starting::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "State Decomposition-1::VehicleStates")))))
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (initialState (status resolved) (from (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))) (to (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::off"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))) (to (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::off"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))) (to (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::on"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))) (to (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates::starting"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "State Decomposition-1::vehicleStates"))) (to (node (document "d0") (qualified-name "State Decomposition-1::VehicleStates"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
