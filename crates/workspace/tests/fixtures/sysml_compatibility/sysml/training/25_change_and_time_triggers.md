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

    action senseTemperature {
        out temp : TemperatureValue;
    }

    state healthStates {
        in vehicle : Vehicle;
        in controller : VehicleController;

        entry;
        then normal;
        do senseTemperature;

        state normal;
        accept at vehicle . maintenanceTime then maintenance;
        accept when senseTemperature . temp > vehicle . maxTemperature do send new OverTemp ( ) to controller then degraded;

        state maintenance {
            }
            accept after 48 [ h ] then normal;

            state degraded;
            accept when senseTemperature . temp <= vehicle . maxTemperature then normal;
        }
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
(model
  (namespace
    (package 'Change and Time Triggers'
      (membership_import private -> 'ISQ::TemperatureValue'[unresolved])
      (membership_import private -> 'ISQ::DurationValue'[unresolved])
      (membership_import private -> 'Time::TimeInstantValue'[unresolved])
      (membership_import private -> 'SI::h'[unresolved])
      (attribute_def 'OverTemp')
      (part_def 'Vehicle'
        (attribute_usage composite 'maintenanceTime' : 'TimeInstantValue'[unresolved])
        (attribute_usage composite 'maintenanceInterval' : 'DurationValue'[unresolved])
        (attribute_usage composite 'maxTemperature' : 'TemperatureValue'[unresolved]))
      (part_def 'VehicleController')
      (action_usage 'senseTemperature'
        (reference_usage out reference 'temp' : 'TemperatureValue'[unresolved]))
      (state_usage 'healthStates'
        (reference_usage in reference 'vehicle' : 'Change and Time Triggers::Vehicle'[part_def])
        (reference_usage in reference 'controller' : 'Change and Time Triggers::VehicleController'[part_def])
        (state_subaction_membership 'entry'
          (action_usage))
        (source_succession
          (reference_usage reference 'normal'))
        (state_subaction_membership 'do'
          (action_usage 'senseTemperature'))
        (state_usage composite 'normal')
        (transition_usage)
        (transition_usage)
        (state_usage composite 'maintenance'
          (not_implemented 'malformed')
          (transition_usage)
          (state_usage composite 'degraded')
          (transition_usage))))))
~~~
