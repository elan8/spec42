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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Transition Actions"))) (name "Transition Actions") (declared-name "Transition Actions")
      (contains
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "Transition Actions::ControllerStartSignal"))) (name "ControllerStartSignal") (declared-name "ControllerStartSignal") (declared (properties (ordered false) (unique true))))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Transition Actions::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Transition Actions::VehicleController"))) (name "VehicleController") (declared-name "VehicleController") (declared))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "Transition Actions::VehicleOffSignal"))) (name "VehicleOffSignal") (declared-name "VehicleOffSignal") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "Transition Actions::VehicleOnSignal"))) (name "VehicleOnSignal") (declared-name "VehicleOnSignal") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "Transition Actions::VehicleStartSignal"))) (name "VehicleStartSignal") (declared-name "VehicleStartSignal") (declared (properties (ordered false) (unique true))))
        (element (kind "state def") (id (node (document "d0") (qualified-name "Transition Actions::VehicleStates"))) (name "VehicleStates") (declared-name "VehicleStates"))
        (element (kind "action") (id (node (document "d0") (qualified-name "Transition Actions::performSelfTest"))) (name "performSelfTest") (declared-name "performSelfTest") (declared)
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Transition Actions::performSelfTest::vehicle"))) (name "vehicle") (declared-name "vehicle"))
          )
        )
        (element (kind "state") (id (node (document "d0") (qualified-name "Transition Actions::vehicleStates"))) (name "vehicleStates") (declared-name "vehicleStates") (declared)
          (contains
            (element (kind "action") (id (node (document "d0") (qualified-name "Transition Actions::vehicleStates::_entry"))) (name "entry") (declared-name "entry") (effective (featuring-type (node (document "d0") (qualified-name "Transition Actions::VehicleStates")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Transition Actions::vehicleStates::controller"))) (name "controller") (declared-name "controller") (effective (featuring-type (node (document "d0") (qualified-name "Transition Actions::VehicleStates")))))
            (element (kind "state") (id (node (document "d0") (qualified-name "Transition Actions::vehicleStates::off"))) (name "off") (declared-name "off") (effective (featuring-type (node (document "d0") (qualified-name "Transition Actions::VehicleStates")))))
            (element (kind "state") (id (node (document "d0") (qualified-name "Transition Actions::vehicleStates::on"))) (name "on") (declared-name "on") (effective (featuring-type (node (document "d0") (qualified-name "Transition Actions::VehicleStates"))))
              (contains
                (element (kind "action") (id (node (document "d0") (qualified-name "Transition Actions::vehicleStates::on::_do"))) (name "do") (declared-name "do") (effective (featuring-type (node (document "d0") (qualified-name "Transition Actions::VehicleStates")))))
                (element (kind "action") (id (node (document "d0") (qualified-name "Transition Actions::vehicleStates::on::_entry"))) (name "entry") (declared-name "entry") (effective (featuring-type (node (document "d0") (qualified-name "Transition Actions::VehicleStates"))))
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Transition Actions::vehicleStates::on::_entry::vehicle"))) (name "vehicle") (declared-name "vehicle") (effective (featuring-type (node (document "d0") (qualified-name "Transition Actions::VehicleStates")))))
                  )
                )
                (element (kind "action") (id (node (document "d0") (qualified-name "Transition Actions::vehicleStates::on::_exit"))) (name "exit") (declared-name "exit") (effective (featuring-type (node (document "d0") (qualified-name "Transition Actions::VehicleStates")))))
              )
            )
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Transition Actions::vehicleStates::operatingVehicle"))) (name "operatingVehicle") (declared-name "operatingVehicle") (effective (featuring-type (node (document "d0") (qualified-name "Transition Actions::VehicleStates")))))
            (element (kind "state") (id (node (document "d0") (qualified-name "Transition Actions::vehicleStates::starting"))) (name "starting") (declared-name "starting") (effective (featuring-type (node (document "d0") (qualified-name "Transition Actions::VehicleStates")))))
            (element (kind "transition") (id (node (document "d0") (qualified-name "Transition Actions::vehicleStates::transition_vehicleStates_to_off"))) (name "transition_vehicleStates_to_off") (declared-name "transition_vehicleStates_to_off") (effective (featuring-type (node (document "d0") (qualified-name "Transition Actions::VehicleStates"))))
              (contains
                (element (kind "transition trigger") (id (node (document "d0") (qualified-name "Transition Actions::vehicleStates::transition_vehicleStates_to_off::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "Transition Actions::VehicleStates")))))
              )
            )
            (element (kind "transition") (id (node (document "d0") (qualified-name "Transition Actions::vehicleStates::transition_vehicleStates_to_on"))) (name "transition_vehicleStates_to_on") (declared-name "transition_vehicleStates_to_on") (effective (featuring-type (node (document "d0") (qualified-name "Transition Actions::VehicleStates"))))
              (contains
                (element (kind "transition effect") (id (node (document "d0") (qualified-name "Transition Actions::vehicleStates::transition_vehicleStates_to_on::effect"))) (name "effect") (declared-name "effect") (effective (featuring-type (node (document "d0") (qualified-name "Transition Actions::VehicleStates")))))
                (element (kind "transition guard") (id (node (document "d0") (qualified-name "Transition Actions::vehicleStates::transition_vehicleStates_to_on::guard"))) (name "guard") (declared-name "guard") (declared (own-expression (expression (kind "memberAccess") (reference "brakePedalDepressed") (children (expression (kind "featureReference") (reference "operatingVehicle")))))) (effective (featuring-type (node (document "d0") (qualified-name "Transition Actions::VehicleStates")))))
                (element (kind "transition trigger") (id (node (document "d0") (qualified-name "Transition Actions::vehicleStates::transition_vehicleStates_to_on::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "Transition Actions::VehicleStates")))))
              )
            )
            (element (kind "transition") (id (node (document "d0") (qualified-name "Transition Actions::vehicleStates::transition_vehicleStates_to_starting"))) (name "transition_vehicleStates_to_starting") (declared-name "transition_vehicleStates_to_starting") (effective (featuring-type (node (document "d0") (qualified-name "Transition Actions::VehicleStates"))))
              (contains
                (element (kind "transition trigger") (id (node (document "d0") (qualified-name "Transition Actions::vehicleStates::transition_vehicleStates_to_starting::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "Transition Actions::VehicleStates")))))
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (initialState (status resolved) (from (node (document "d0") (qualified-name "Transition Actions::vehicleStates"))) (to (node (document "d0") (qualified-name "Transition Actions::vehicleStates::off"))) (provenance authored))
    (transition (status resolved) (from (node (document "d0") (qualified-name "Transition Actions::vehicleStates"))) (to (node (document "d0") (qualified-name "Transition Actions::vehicleStates::off"))) (provenance authored))
    (transition (status resolved) (from (node (document "d0") (qualified-name "Transition Actions::vehicleStates"))) (to (node (document "d0") (qualified-name "Transition Actions::vehicleStates::on"))) (provenance authored))
    (transition (status resolved) (from (node (document "d0") (qualified-name "Transition Actions::vehicleStates"))) (to (node (document "d0") (qualified-name "Transition Actions::vehicleStates::starting"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Transition Actions::performSelfTest::vehicle"))) (to (node (document "d0") (qualified-name "Transition Actions::Vehicle"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Transition Actions::vehicleStates"))) (to (node (document "d0") (qualified-name "Transition Actions::VehicleStates"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Transition Actions::vehicleStates::controller"))) (to (node (document "d0") (qualified-name "Transition Actions::VehicleController"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Transition Actions::vehicleStates::operatingVehicle"))) (to (node (document "d0") (qualified-name "Transition Actions::Vehicle"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Transition Actions::ControllerStartSignal"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Transition Actions::Vehicle"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Transition Actions::VehicleController"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Transition Actions::VehicleOffSignal"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Transition Actions::VehicleOnSignal"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Transition Actions::VehicleStartSignal"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Transition Actions::VehicleStates"))) (status missing-prerequisite) (target "States::StateAction"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Transition Actions::performSelfTest"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Transition Actions::vehicleStates"))) (status missing-prerequisite) (target "States::stateActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Transition Actions::vehicleStates::_entry"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Transition Actions::vehicleStates::off"))) (status missing-prerequisite) (target "States::stateActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Transition Actions::vehicleStates::on"))) (status missing-prerequisite) (target "States::stateActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Transition Actions::vehicleStates::on::_do"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Transition Actions::vehicleStates::on::_entry"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Transition Actions::vehicleStates::on::_exit"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Transition Actions::vehicleStates::starting"))) (status missing-prerequisite) (target "States::stateActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Transition Actions::vehicleStates::transition_vehicleStates_to_off"))) (status missing-prerequisite) (target "Actions::transitionActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Transition Actions::vehicleStates::transition_vehicleStates_to_off::trigger"))) (status missing-prerequisite) (target "Actions::acceptActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Transition Actions::vehicleStates::transition_vehicleStates_to_on"))) (status missing-prerequisite) (target "Actions::transitionActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Transition Actions::vehicleStates::transition_vehicleStates_to_on::effect"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Transition Actions::vehicleStates::transition_vehicleStates_to_on::trigger"))) (status missing-prerequisite) (target "Actions::acceptActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Transition Actions::vehicleStates::transition_vehicleStates_to_starting"))) (status missing-prerequisite) (target "Actions::transitionActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Transition Actions::vehicleStates::transition_vehicleStates_to_starting::trigger"))) (status missing-prerequisite) (target "Actions::acceptActions"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/training/25_transition_actions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "transition_guard_non_boolean")
        (source "semantic")
        (range (start 28 2) (end 28 132))
      )
    )
  )
)
~~~
