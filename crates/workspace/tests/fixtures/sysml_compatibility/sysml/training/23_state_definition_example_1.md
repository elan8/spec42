# META
~~~ini
description=SysML Training 23 (State Definitions): State Definition Example-1
type=file
~~~
# SOURCE
~~~sysml
package 'State Definition Example-1' {
	
	attribute def VehicleStartSignal;
	attribute def VehicleOnSignal;
	attribute def VehicleOffSignal;
		
	state def VehicleStates {
		entry; then off;
		
		state off;
		
		transition off_to_starting
			first off
			accept VehicleStartSignal 
			then starting;
			
		state starting;
		
		transition starting_to_on
			first starting
			accept VehicleOnSignal
			then on;
			
		state on;
		
		transition on_to_off
			first on
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
KwState,KwDef,Ident,OpenCurly,
KwEntry,Semicolon,KwThen,Ident,Semicolon,
KwState,Ident,Semicolon,
KwTransition,Ident,
KwFirst,Ident,
KwAccept,Ident,
KwThen,Ident,Semicolon,
KwState,Ident,Semicolon,
KwTransition,Ident,
KwFirst,Ident,
KwAccept,Ident,
KwThen,Ident,Semicolon,
KwState,Ident,Semicolon,
KwTransition,Ident,
KwFirst,Ident,
KwAccept,Ident,
KwThen,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''State Definition Example-1''
    (attribute_def 'VehicleStartSignal')
    (attribute_def 'VehicleOnSignal')
    (attribute_def 'VehicleOffSignal')
    (state_def 'VehicleStates'
      (entry_action)
      (source_succession
        (default_ref_usage 'off'))
      (state_usage 'off')
      (transition_usage 'off_to_starting')
      (state_usage 'starting')
      (transition_usage 'starting_to_on')
      (state_usage 'on')
      (transition_usage 'on_to_off'))))
~~~
# FORMAT
~~~sysml
package 'State Definition Example-1' {

    attribute def VehicleStartSignal;
    attribute def VehicleOnSignal;
    attribute def VehicleOffSignal;

    state def VehicleStates {
        entry; then off;

        state off;

        transition off_to_starting
        first off
        accept VehicleStartSignal
        then starting;

        state starting;

        transition starting_to_on
        first starting
        accept VehicleOnSignal
        then on;

        state on;

        transition on_to_off
        first on
        accept VehicleOffSignal
        then off;
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
    (element (kind "package") (id (node (document "d0") (qualified-name "State Definition Example-1"))) (name "State Definition Example-1") (declared-name "State Definition Example-1")
      (contains
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "State Definition Example-1::VehicleOffSignal"))) (name "VehicleOffSignal") (declared-name "VehicleOffSignal") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "State Definition Example-1::VehicleOnSignal"))) (name "VehicleOnSignal") (declared-name "VehicleOnSignal") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "State Definition Example-1::VehicleStartSignal"))) (name "VehicleStartSignal") (declared-name "VehicleStartSignal") (declared (properties (ordered false) (unique true))))
        (element (kind "state def") (id (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates"))) (name "VehicleStates") (declared-name "VehicleStates")
          (contains
            (element (kind "action") (id (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates::_entry"))) (name "entry") (declared-name "entry") (effective (featuring-type (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates")))))
            (element (kind "state") (id (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates::off"))) (name "off") (declared-name "off") (effective (featuring-type (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates")))))
            (element (kind "transition") (id (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates::off_to_starting"))) (name "off_to_starting") (declared-name "off_to_starting") (effective (featuring-type (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates"))))
              (contains
                (element (kind "transition trigger") (id (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates::off_to_starting::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates")))))
              )
            )
            (element (kind "state") (id (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates::on"))) (name "on") (declared-name "on") (effective (featuring-type (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates")))))
            (element (kind "transition") (id (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates::on_to_off"))) (name "on_to_off") (declared-name "on_to_off") (effective (featuring-type (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates"))))
              (contains
                (element (kind "transition trigger") (id (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates::on_to_off::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates")))))
              )
            )
            (element (kind "state") (id (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates::starting"))) (name "starting") (declared-name "starting") (effective (featuring-type (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates")))))
            (element (kind "transition") (id (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates::starting_to_on"))) (name "starting_to_on") (declared-name "starting_to_on") (effective (featuring-type (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates"))))
              (contains
                (element (kind "transition trigger") (id (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates::starting_to_on::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates")))))
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (initialState (status resolved) (from (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates"))) (to (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates::off"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates::off"))) (to (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates::starting"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates::on"))) (to (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates::off"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates::starting"))) (to (node (document "d0") (qualified-name "State Definition Example-1::VehicleStates::on"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
