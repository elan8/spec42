# META
~~~ini
description=SysML Training 25 (Transitions): Change and Time Triggers
type=file
~~~
# SOURCE
~~~sysml
package 'Change and Time Triggers' {
	private import ISQ::TemperatureValue;
	private import ISQ::DurationValue;
	private import Time::TimeInstantValue;
	private import SI::h;
	
	attribute def OverTemp;
	
	part def Vehicle {
		attribute maintenanceTime : TimeInstantValue;
		attribute maintenanceInterval : DurationValue;
		attribute maxTemperature : TemperatureValue;
	}
	
	part def VehicleController;
	
	action senseTemperature { out temp : TemperatureValue; }
	
	state healthStates {
		in vehicle : Vehicle;
		in controller : VehicleController;
		
		entry; then normal;
		do senseTemperature;
		
		state normal;
		accept at vehicle.maintenanceTime
			then maintenance;
		accept when senseTemperature.temp > vehicle.maxTemperature
			do send new OverTemp() to controller 
			then degraded;
		
		state maintenance {
			entry assign vehicle.maintenanceTime := vehicle.maintenanceTime + vehicle.maintenanceInterval;
		}
		accept after 48 [h]
			then normal;
		
		state degraded;
		accept when senseTemperature.temp <= vehicle.maxTemperature
			then normal;
	}
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwAttribute,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,Semicolon,
KwAction,Ident,OpenCurly,KwOut,Ident,Colon,Ident,Semicolon,CloseCurly,
KwState,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwEntry,Semicolon,KwThen,Ident,Semicolon,
KwDo,Ident,Semicolon,
KwState,Ident,Semicolon,
KwAccept,Ident,Ident,Dot,Ident,
KwThen,Ident,Semicolon,
KwAccept,KwWhen,Ident,Dot,Ident,CloseAngle,Ident,Dot,Ident,
KwDo,KwSend,Ident,Ident,OpenParen,CloseParen,KwTo,Ident,
KwThen,Ident,Semicolon,
KwState,Ident,OpenCurly,
KwEntry,KwAssign,Ident,Dot,Ident,ColonEq,Ident,Dot,Ident,Plus,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwAccept,KwAfter,DecimalValue,OpenSquare,Ident,CloseSquare,
KwThen,Ident,Semicolon,
KwState,Ident,Semicolon,
KwAccept,KwWhen,Ident,Dot,Ident,LtEq,Ident,Dot,Ident,
KwThen,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Change and Time Triggers''
    (import_decl private 'ISQ::TemperatureValue')
    (import_decl private 'ISQ::DurationValue')
    (import_decl private 'Time::TimeInstantValue')
    (import_decl private 'SI::h')
    (attribute_def 'OverTemp')
    (part_def 'Vehicle'
      (attribute_usage 'maintenanceTime' : 'TimeInstantValue')
      (attribute_usage 'maintenanceInterval' : 'DurationValue')
      (attribute_usage 'maxTemperature' : 'TemperatureValue'))
    (part_def 'VehicleController')
    (action_usage 'senseTemperature'
      (default_ref_usage out 'temp' : 'TemperatureValue'))
    (state_usage 'healthStates'
      (default_ref_usage in 'vehicle' : 'Vehicle')
      (default_ref_usage in 'controller' : 'VehicleController')
      (entry_action)
      (source_succession
        (default_ref_usage 'normal'))
      (do_action 'senseTemperature')
      (state_usage 'normal')
      (target_transition)
      (target_transition)
      (state_usage 'maintenance'
        (malformed)
        (target_transition)
        (state_usage 'degraded')
        (target_transition)))))
~~~
# FORMAT
~~~sysml
package 'Change and Time Triggers' {
    private import ISQ::TemperatureValue;
    private import ISQ::DurationValue;
    private import Time::TimeInstantValue;
    private import SI::h;

    attribute def OverTemp;

    part def Vehicle {
        attribute maintenanceTime : TimeInstantValue;
        attribute maintenanceInterval : DurationValue;
        attribute maxTemperature : TemperatureValue;
    }

    part def VehicleController;

    action senseTemperature { out temp : TemperatureValue; }

    state healthStates {
        in vehicle : Vehicle;
        in controller : VehicleController;

        entry; then normal;
        do senseTemperature;

        state normal;
        accept at vehicle.maintenanceTime
        then maintenance;
        accept when senseTemperature.temp > vehicle.maxTemperature
        do send new OverTemp() to controller
        then degraded;

        state maintenance {
            entry assign vehicle.maintenanceTime := vehicle.maintenanceTime + vehicle.maintenanceInterval;
        }
        accept after 48 [h]
        then normal;

        state degraded;
        accept when senseTemperature.temp <= vehicle.maxTemperature
        then normal;
    }
}

~~~
# EXPECTED
~~~
parse.expected_semicolon_or_body
parse.expected_close_curly
semantic.duplicate_name 'normal'
semantic.unresolved_name 'TimeInstantValue'
semantic.unresolved_name 'DurationValue'
semantic.unresolved_name 'TemperatureValue'
semantic.unresolved_name 'TemperatureValue'
~~~
# PROBLEMS
~~~
parse.expected_semicolon_or_body
parse.expected_close_curly
semantic.duplicate_name 'normal'
semantic.unresolved_name 'TimeInstantValue'
semantic.unresolved_name 'DurationValue'
semantic.unresolved_name 'TemperatureValue'
semantic.unresolved_name 'TemperatureValue'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Change and Time Triggers"))) (name "Change and Time Triggers") (declared-name "Change and Time Triggers")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Change and Time Triggers::DurationValue"))) (name "DurationValue") (declared-name "DurationValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "Change and Time Triggers::OverTemp"))) (name "OverTemp") (declared-name "OverTemp") (declared (properties (ordered false) (unique true))))
        (element (kind "import") (id (node (document "d0") (qualified-name "Change and Time Triggers::TemperatureValue"))) (name "TemperatureValue") (declared-name "TemperatureValue"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Change and Time Triggers::TimeInstantValue"))) (name "TimeInstantValue") (declared-name "TimeInstantValue"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared)
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle::maintenanceInterval"))) (name "maintenanceInterval") (declared-name "maintenanceInterval") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle::maintenanceTime"))) (name "maintenanceTime") (declared-name "maintenanceTime") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle::maxTemperature"))) (name "maxTemperature") (declared-name "maxTemperature") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "Change and Time Triggers::VehicleController"))) (name "VehicleController") (declared-name "VehicleController") (declared))
        (element (kind "import") (id (node (document "d0") (qualified-name "Change and Time Triggers::h"))) (name "h") (declared-name "h"))
        (element (kind "state") (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))) (name "healthStates") (declared-name "healthStates") (declared (properties (composite true) (reference false)))
          (contains
            (element (kind "action") (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::_do"))) (name "do") (declared-name "do"))
            (element (kind "action") (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::_entry"))) (name "entry") (declared-name "entry"))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::controller"))) (name "controller") (declared-name "controller"))
            (element (kind "state") (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::degraded"))) (name "degraded") (declared-name "degraded"))
            (element (kind "state") (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::maintenance"))) (name "maintenance") (declared-name "maintenance"))
            (element (kind "state") (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::normal"))) (name "normal") (declared-name "normal"))
            (element (kind "transition") (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::transition_healthStates_to_degraded"))) (name "transition_healthStates_to_degraded") (declared-name "transition_healthStates_to_degraded")
              (contains
                (element (kind "transition effect") (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::transition_healthStates_to_degraded::effect"))) (name "effect") (declared-name "effect"))
                (element (kind "transition trigger") (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::transition_healthStates_to_degraded::trigger"))) (name "trigger") (declared-name "trigger"))
              )
            )
            (element (kind "transition") (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::transition_healthStates_to_maintenance"))) (name "transition_healthStates_to_maintenance") (declared-name "transition_healthStates_to_maintenance")
              (contains
                (element (kind "transition trigger") (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::transition_healthStates_to_maintenance::trigger"))) (name "trigger") (declared-name "trigger"))
              )
            )
            (element (kind "transition") (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::transition_healthStates_to_normal"))) (name "transition_healthStates_to_normal") (declared-name "transition_healthStates_to_normal")
              (contains
                (element (kind "transition trigger") (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::transition_healthStates_to_normal::trigger"))) (name "trigger") (declared-name "trigger"))
              )
            )
            (element (kind "transition") (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::transition_healthStates_to_normal#transition"))) (name "transition_healthStates_to_normal") (declared-name "transition_healthStates_to_normal")
              (contains
                (element (kind "transition trigger") (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::transition_healthStates_to_normal#transition::trigger"))) (name "trigger") (declared-name "trigger"))
              )
            )
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::vehicle"))) (name "vehicle") (declared-name "vehicle"))
          )
        )
        (element (kind "action") (id (node (document "d0") (qualified-name "Change and Time Triggers::senseTemperature"))) (name "senseTemperature") (declared-name "senseTemperature") (declared (properties (composite true) (reference false)))
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Change and Time Triggers::senseTemperature::temp"))) (name "temp") (declared-name "temp"))
          )
        )
      )
    )
  )
  (relationships
    (initialState (status resolved) (from (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))) (to (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::normal"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))) (to (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::degraded"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))) (to (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::maintenance"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))) (to (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::normal"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))) (to (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::normal"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::controller"))) (to (node (document "d0") (qualified-name "Change and Time Triggers::VehicleController"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::vehicle"))) (to (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
