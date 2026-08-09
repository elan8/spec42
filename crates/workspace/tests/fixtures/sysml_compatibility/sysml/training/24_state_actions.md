# META
~~~ini
description=SysML Training 24 (States): State Actions
type=file
~~~
# SOURCE
~~~sysml
package 'State Actions' {
	
	attribute def VehicleStartSignal;
	attribute def VehicleOnSignal;
	attribute def VehicleOffSignal;
	
	part def Vehicle;
	
	action performSelfTest { in vehicle : Vehicle; }
	
	state def VehicleStates { in operatingVehicle : Vehicle; }
		
	state vehicleStates : VehicleStates {
		in operatingVehicle : Vehicle;
			
		entry; then off;
		
		state off;
		accept VehicleStartSignal 
			then starting;
			
		state starting;
		accept VehicleOnSignal
			then on;
			
		state on {
			entry performSelfTest{ in vehicle = operatingVehicle; }
			do action providePower { /* ... */ }
			exit action applyParkingBrake { /* ... */ }
		}
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
KwPart,KwDef,Ident,Semicolon,
KwAction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,CloseCurly,
KwState,KwDef,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,CloseCurly,
KwState,Ident,Colon,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwEntry,Semicolon,KwThen,Ident,Semicolon,
KwState,Ident,Semicolon,
KwAccept,Ident,
KwThen,Ident,Semicolon,
KwState,Ident,Semicolon,
KwAccept,Ident,
KwThen,Ident,Semicolon,
KwState,Ident,OpenCurly,
KwEntry,Ident,OpenCurly,KwIn,Ident,Eq,Ident,Semicolon,CloseCurly,
KwDo,KwAction,Ident,OpenCurly,RegularComment,CloseCurly,
KwExit,KwAction,Ident,OpenCurly,RegularComment,CloseCurly,
CloseCurly,
KwAccept,Ident,
KwThen,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''State Actions''
    (attribute_def 'VehicleStartSignal')
    (attribute_def 'VehicleOnSignal')
    (attribute_def 'VehicleOffSignal')
    (part_def 'Vehicle')
    (action_usage 'performSelfTest'
      (default_ref_usage in 'vehicle' : 'Vehicle'))
    (state_def 'VehicleStates'
      (default_ref_usage in 'operatingVehicle' : 'Vehicle'))
    (state_usage 'vehicleStates' : 'VehicleStates'
      (default_ref_usage in 'operatingVehicle' : 'Vehicle')
      (entry_action)
      (source_succession
        (default_ref_usage 'off'))
      (state_usage 'off')
      (target_transition)
      (state_usage 'starting')
      (target_transition)
      (state_usage 'on'
        (entry_action 'performSelfTest'
          (default_ref_usage in 'vehicle' value))
        (do_action 'providePower'
          (comment))
        (exit_action 'applyParkingBrake'
          (comment)))
      (target_transition))))
~~~
# FORMAT
~~~sysml
package 'State Actions' {
    attribute def VehicleStartSignal;
    attribute def VehicleOnSignal;
    attribute def VehicleOffSignal;

    part def Vehicle;

    action performSelfTest {
        in vehicle : Vehicle;
    }

    state def VehicleStates {
        in operatingVehicle : Vehicle;
    }

    state vehicleStates : VehicleStates {
        in operatingVehicle : Vehicle;

        entry;
        then off;

        state off;
        accept VehicleStartSignal then starting;

        state starting;
        accept VehicleOnSignal then on;

        state on {
            entry performSelfTest {
                in vehicle = operatingVehicle;
            }
            do providePower {
                /* ... */
            }
            exit applyParkingBrake {
                /* ... */
            }
        }
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
    (element (kind "package") (id (node (document "d0") (qualified-name "State Actions"))) (name "State Actions") (declared-name "State Actions")
      (contains
        (element (kind "part def") (id (node (document "d0") (qualified-name "State Actions::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "State Actions::VehicleOffSignal"))) (name "VehicleOffSignal") (declared-name "VehicleOffSignal") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "State Actions::VehicleOnSignal"))) (name "VehicleOnSignal") (declared-name "VehicleOnSignal") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "State Actions::VehicleStartSignal"))) (name "VehicleStartSignal") (declared-name "VehicleStartSignal") (declared (properties (ordered false) (unique true))))
        (element (kind "state def") (id (node (document "d0") (qualified-name "State Actions::VehicleStates"))) (name "VehicleStates") (declared-name "VehicleStates")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "State Actions::VehicleStates::operatingVehicle"))) (name "operatingVehicle") (declared-name "operatingVehicle") (effective (featuring-type (node (document "d0") (qualified-name "State Actions::VehicleStates")))))
          )
        )
        (element (kind "action") (id (node (document "d0") (qualified-name "State Actions::performSelfTest"))) (name "performSelfTest") (declared-name "performSelfTest") (declared (properties (composite true) (reference false)))
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "State Actions::performSelfTest::vehicle"))) (name "vehicle") (declared-name "vehicle"))
          )
        )
        (element (kind "state") (id (node (document "d0") (qualified-name "State Actions::vehicleStates"))) (name "vehicleStates") (declared-name "vehicleStates") (declared (properties (composite true) (reference false)))
          (contains
            (element (kind "action") (id (node (document "d0") (qualified-name "State Actions::vehicleStates::_entry"))) (name "entry") (declared-name "entry") (effective (featuring-type (node (document "d0") (qualified-name "State Actions::VehicleStates")))))
            (element (kind "state") (id (node (document "d0") (qualified-name "State Actions::vehicleStates::off"))) (name "off") (declared-name "off") (effective (featuring-type (node (document "d0") (qualified-name "State Actions::VehicleStates")))))
            (element (kind "state") (id (node (document "d0") (qualified-name "State Actions::vehicleStates::on"))) (name "on") (declared-name "on") (effective (featuring-type (node (document "d0") (qualified-name "State Actions::VehicleStates"))))
              (contains
                (element (kind "action") (id (node (document "d0") (qualified-name "State Actions::vehicleStates::on::_do"))) (name "do") (declared-name "do") (effective (featuring-type (node (document "d0") (qualified-name "State Actions::VehicleStates")))))
                (element (kind "action") (id (node (document "d0") (qualified-name "State Actions::vehicleStates::on::_entry"))) (name "entry") (declared-name "entry") (effective (featuring-type (node (document "d0") (qualified-name "State Actions::VehicleStates"))))
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "State Actions::vehicleStates::on::_entry::vehicle"))) (name "vehicle") (declared-name "vehicle") (effective (featuring-type (node (document "d0") (qualified-name "State Actions::VehicleStates")))))
                  )
                )
                (element (kind "action") (id (node (document "d0") (qualified-name "State Actions::vehicleStates::on::_exit"))) (name "exit") (declared-name "exit") (effective (featuring-type (node (document "d0") (qualified-name "State Actions::VehicleStates")))))
              )
            )
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "State Actions::vehicleStates::operatingVehicle"))) (name "operatingVehicle") (declared-name "operatingVehicle") (effective (featuring-type (node (document "d0") (qualified-name "State Actions::VehicleStates")))))
            (element (kind "state") (id (node (document "d0") (qualified-name "State Actions::vehicleStates::starting"))) (name "starting") (declared-name "starting") (effective (featuring-type (node (document "d0") (qualified-name "State Actions::VehicleStates")))))
            (element (kind "transition") (id (node (document "d0") (qualified-name "State Actions::vehicleStates::transition_vehicleStates_to_off"))) (name "transition_vehicleStates_to_off") (declared-name "transition_vehicleStates_to_off") (effective (featuring-type (node (document "d0") (qualified-name "State Actions::VehicleStates"))))
              (contains
                (element (kind "transition trigger") (id (node (document "d0") (qualified-name "State Actions::vehicleStates::transition_vehicleStates_to_off::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "State Actions::VehicleStates")))))
              )
            )
            (element (kind "transition") (id (node (document "d0") (qualified-name "State Actions::vehicleStates::transition_vehicleStates_to_on"))) (name "transition_vehicleStates_to_on") (declared-name "transition_vehicleStates_to_on") (effective (featuring-type (node (document "d0") (qualified-name "State Actions::VehicleStates"))))
              (contains
                (element (kind "transition trigger") (id (node (document "d0") (qualified-name "State Actions::vehicleStates::transition_vehicleStates_to_on::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "State Actions::VehicleStates")))))
              )
            )
            (element (kind "transition") (id (node (document "d0") (qualified-name "State Actions::vehicleStates::transition_vehicleStates_to_starting"))) (name "transition_vehicleStates_to_starting") (declared-name "transition_vehicleStates_to_starting") (effective (featuring-type (node (document "d0") (qualified-name "State Actions::VehicleStates"))))
              (contains
                (element (kind "transition trigger") (id (node (document "d0") (qualified-name "State Actions::vehicleStates::transition_vehicleStates_to_starting::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "State Actions::VehicleStates")))))
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (initialState (status resolved) (from (node (document "d0") (qualified-name "State Actions::vehicleStates"))) (to (node (document "d0") (qualified-name "State Actions::vehicleStates::off"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "State Actions::vehicleStates"))) (to (node (document "d0") (qualified-name "State Actions::vehicleStates::off"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "State Actions::vehicleStates"))) (to (node (document "d0") (qualified-name "State Actions::vehicleStates::on"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "State Actions::vehicleStates"))) (to (node (document "d0") (qualified-name "State Actions::vehicleStates::starting"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "State Actions::VehicleStates::operatingVehicle"))) (to (node (document "d0") (qualified-name "State Actions::Vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "State Actions::performSelfTest::vehicle"))) (to (node (document "d0") (qualified-name "State Actions::Vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "State Actions::vehicleStates"))) (to (node (document "d0") (qualified-name "State Actions::VehicleStates"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "State Actions::vehicleStates::operatingVehicle"))) (to (node (document "d0") (qualified-name "State Actions::Vehicle"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
