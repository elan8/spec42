# META
~~~ini
description=SysML Training 25 (Transitions): Transition Actions
type=file
~~~
# SOURCE
~~~sysml
package 'Transition Actions' {
	
	attribute def VehicleStartSignal;
	attribute def VehicleOnSignal;
	attribute def VehicleOffSignal;
	
	attribute def ControllerStartSignal;
	
	part def Vehicle {
		brakePedalDepressed : ScalarValues::Boolean;
	}
	part def VehicleController;
	
	action performSelfTest { in vehicle : Vehicle; }
	
	state def VehicleStates;
		
	state vehicleStates : VehicleStates {
		in operatingVehicle : Vehicle;
		in controller : VehicleController;

		entry; then off;
		
		state off;
		accept VehicleStartSignal 
			then starting;
			
		state starting;
		accept VehicleOnSignal
			if operatingVehicle.brakePedalDepressed
			do send new ControllerStartSignal() to controller
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
KwAttribute,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
Ident,Colon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,Semicolon,
KwAction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,CloseCurly,
KwState,KwDef,Ident,Semicolon,
KwState,Ident,Colon,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwEntry,Semicolon,KwThen,Ident,Semicolon,
KwState,Ident,Semicolon,
KwAccept,Ident,
KwThen,Ident,Semicolon,
KwState,Ident,Semicolon,
KwAccept,Ident,
KwIf,Ident,Dot,Ident,
KwDo,KwSend,Ident,Ident,OpenParen,CloseParen,KwTo,Ident,
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
  (package_def ''Transition Actions''
    (attribute_def 'VehicleStartSignal')
    (attribute_def 'VehicleOnSignal')
    (attribute_def 'VehicleOffSignal')
    (attribute_def 'ControllerStartSignal')
    (part_def 'Vehicle'
      (default_ref_usage 'brakePedalDepressed' : 'ScalarValues::Boolean'))
    (part_def 'VehicleController')
    (action_usage 'performSelfTest'
      (default_ref_usage in 'vehicle' : 'Vehicle'))
    (state_def 'VehicleStates')
    (state_usage 'vehicleStates' : 'VehicleStates'
      (default_ref_usage in 'operatingVehicle' : 'Vehicle')
      (default_ref_usage in 'controller' : 'VehicleController')
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
package 'Transition Actions' {
    attribute def VehicleStartSignal;
    attribute def VehicleOnSignal;
    attribute def VehicleOffSignal;

    attribute def ControllerStartSignal;

    part def Vehicle {
        brakePedalDepressed : ScalarValues::Boolean;
    }
    part def VehicleController;

    action performSelfTest {
        in vehicle : Vehicle;
    }

    state def VehicleStates;

    state vehicleStates : VehicleStates {
        in operatingVehicle : Vehicle;
        in controller : VehicleController;

        entry;
        then off;

        state off;
        accept VehicleStartSignal then starting;

        state starting;
        accept VehicleOnSignal if operatingVehicle . brakePedalDepressed do send new ControllerStartSignal ( ) to controller then on;

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
semantic.unresolved_name 'ScalarValues::Boolean'
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'off'
semantic.unresolved_name 'ScalarValues::Boolean'
~~~
# SMG
~~~
(model
  (namespace
    (package 'Transition Actions'
      (attribute_def 'VehicleStartSignal')
      (attribute_def 'VehicleOnSignal')
      (attribute_def 'VehicleOffSignal')
      (attribute_def 'ControllerStartSignal')
      (part_def 'Vehicle'
        (reference_usage reference 'brakePedalDepressed' : 'ScalarValues::Boolean'[unresolved]))
      (part_def 'VehicleController')
      (action_usage 'performSelfTest'
        (reference_usage in reference 'vehicle' : 'Transition Actions::Vehicle'[part_def]))
      (state_def 'VehicleStates')
      (state_usage 'vehicleStates' : 'Transition Actions::VehicleStates'[state_def]
        (reference_usage in reference 'operatingVehicle' : 'Transition Actions::Vehicle'[part_def])
        (reference_usage in reference 'controller' : 'Transition Actions::VehicleController'[part_def])
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
