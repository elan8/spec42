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
(model
  (namespace
    (package 'State Actions'
      (attribute_def 'VehicleStartSignal')
      (attribute_def 'VehicleOnSignal')
      (attribute_def 'VehicleOffSignal')
      (part_def 'Vehicle')
      (action_usage 'performSelfTest'
        (reference_usage in reference 'vehicle' : 'State Actions::Vehicle'[part_def]))
      (state_def 'VehicleStates'
        (reference_usage in reference 'operatingVehicle' : 'State Actions::Vehicle'[part_def]))
      (state_usage 'vehicleStates' : 'State Actions::VehicleStates'[state_def]
        (reference_usage in reference 'operatingVehicle' : 'State Actions::Vehicle'[part_def])
        (state_subaction_membership 'entry'
          (action_usage))
        (source_succession
          (reference_usage reference 'off'))
        (state_usage composite 'off')
        (transition_usage)
        (state_usage composite 'starting')
        (transition_usage)
        (state_usage composite 'on'
          (state_subaction_membership 'entry'
            (action_usage 'performSelfTest'
              (reference_usage in reference 'vehicle'
                (feature_value (=)))))
          (state_subaction_membership 'do'
            (action_usage 'providePower'))
          (state_subaction_membership 'exit'
            (action_usage 'applyParkingBrake')))
        (transition_usage)))))
~~~
