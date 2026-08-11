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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "25_change_and_time_triggers.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 4 16) (end 4 21))
      )
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "3a3066ebab2455ddca9bda19dbb333a5fbfd142066bf415a1e278d452c11befd") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers"))) (kind "package") (name "Change and Time Triggers") (declared-name "Change and Time Triggers") (range (start (line 0) (character 0)) (end (line 0) (character 1059))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::DurationValue"))) (kind "import") (name "DurationValue") (declared-name "DurationValue") (range (start (line 2) (character 1)) (end (line 2) (character 35))) (parent (node (document "d0") (qualified-name "Change and Time Triggers"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::DurationValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 34))))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::OverTemp"))) (kind "attribute def") (name "OverTemp") (declared-name "OverTemp") (range (start (line 6) (character 1)) (end (line 6) (character 24))) (parent (node (document "d0") (qualified-name "Change and Time Triggers"))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::TemperatureValue"))) (kind "import") (name "TemperatureValue") (declared-name "TemperatureValue") (range (start (line 1) (character 1)) (end (line 1) (character 38))) (parent (node (document "d0") (qualified-name "Change and Time Triggers"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::TemperatureValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 37))))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::TimeInstantValue"))) (kind "import") (name "TimeInstantValue") (declared-name "TimeInstantValue") (range (start (line 3) (character 1)) (end (line 3) (character 39))) (parent (node (document "d0") (qualified-name "Change and Time Triggers"))) (authored (membership (kind Import) (visibility "private") (import (reference "Time::TimeInstantValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 3) (character 16)) (end (line 3) (character 38))))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 8) (character 1)) (end (line 8) (character 166))) (parent (node (document "d0") (qualified-name "Change and Time Triggers"))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle::maintenanceInterval"))) (kind "attribute") (name "maintenanceInterval") (declared-name "maintenanceInterval") (range (start (line 10) (character 2)) (end (line 10) (character 48))) (parent (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "DurationValue") (range none)) (typing (reference "DurationValue") (range (start (line 10) (character 34)) (end (line 10) (character 47)))))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle::maintenanceTime"))) (kind "attribute") (name "maintenanceTime") (declared-name "maintenanceTime") (range (start (line 9) (character 2)) (end (line 9) (character 47))) (parent (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "TimeInstantValue") (range none)) (typing (reference "TimeInstantValue") (range (start (line 9) (character 30)) (end (line 9) (character 46)))))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle::maxTemperature"))) (kind "attribute") (name "maxTemperature") (declared-name "maxTemperature") (range (start (line 11) (character 2)) (end (line 11) (character 46))) (parent (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "TemperatureValue") (range none)) (typing (reference "TemperatureValue") (range (start (line 11) (character 29)) (end (line 11) (character 45)))))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::VehicleController"))) (kind "part def") (name "VehicleController") (declared-name "VehicleController") (range (start (line 14) (character 1)) (end (line 14) (character 28))) (parent (node (document "d0") (qualified-name "Change and Time Triggers"))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::h"))) (kind "import") (name "h") (declared-name "h") (range (start (line 4) (character 1)) (end (line 4) (character 22))) (parent (node (document "d0") (qualified-name "Change and Time Triggers"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::h") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 4) (character 16)) (end (line 4) (character 21))))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))) (kind "state") (name "healthStates") (declared-name "healthStates") (range (start (line 18) (character 1)) (end (line 18) (character 593))) (parent (node (document "d0") (qualified-name "Change and Time Triggers"))) (authored (membership (kind Feature)) (relationships (transition (reference "Change and Time Triggers::healthStates::maintenance") (range none)) (transition (reference "Change and Time Triggers::healthStates::degraded") (range none)) (transition (reference "Change and Time Triggers::healthStates::normal") (range none)) (transition (reference "Change and Time Triggers::healthStates::normal") (range none)) (initial-state (reference "Change and Time Triggers::healthStates::normal") (range none)))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::_do"))) (kind "action") (name "do") (declared-name "do") (range (start (line 23) (character 2)) (end (line 23) (character 22))) (parent (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::_entry"))) (kind "action") (name "entry") (declared-name "entry") (range (start (line 22) (character 2)) (end (line 22) (character 8))) (parent (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::controller"))) (kind "in out parameter") (name "controller") (declared-name "controller") (range (start (line 20) (character 2)) (end (line 20) (character 36))) (parent (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))) (authored (relationships (typing (reference "VehicleController") (range none)))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::degraded"))) (kind "state") (name "degraded") (declared-name "degraded") (range (start (line 38) (character 2)) (end (line 38) (character 17))) (parent (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::maintenance"))) (kind "state") (name "maintenance") (declared-name "maintenance") (range (start (line 32) (character 2)) (end (line 32) (character 123))) (parent (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::normal"))) (kind "state") (name "normal") (declared-name "normal") (range (start (line 25) (character 2)) (end (line 25) (character 15))) (parent (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::transition_healthStates_to_degraded"))) (kind "transition") (name "transition_healthStates_to_degraded") (declared-name "transition_healthStates_to_degraded") (range (start (line 28) (character 2)) (end (line 28) (character 119))) (parent (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::transition_healthStates_to_degraded::effect"))) (kind "transition effect") (name "effect") (declared-name "effect") (range (start (line 28) (character 2)) (end (line 28) (character 119))) (parent (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::transition_healthStates_to_degraded"))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::transition_healthStates_to_degraded::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 28) (character 2)) (end (line 28) (character 119))) (parent (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::transition_healthStates_to_degraded"))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::transition_healthStates_to_maintenance"))) (kind "transition") (name "transition_healthStates_to_maintenance") (declared-name "transition_healthStates_to_maintenance") (range (start (line 26) (character 2)) (end (line 26) (character 56))) (parent (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::transition_healthStates_to_maintenance::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 26) (character 2)) (end (line 26) (character 56))) (parent (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::transition_healthStates_to_maintenance"))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::transition_healthStates_to_normal"))) (kind "transition") (name "transition_healthStates_to_normal") (declared-name "transition_healthStates_to_normal") (range (start (line 35) (character 2)) (end (line 35) (character 37))) (parent (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::transition_healthStates_to_normal#transition"))) (kind "transition") (name "transition_healthStates_to_normal") (declared-name "transition_healthStates_to_normal") (range (start (line 39) (character 2)) (end (line 39) (character 77))) (parent (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::transition_healthStates_to_normal#transition::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 39) (character 2)) (end (line 39) (character 77))) (parent (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::transition_healthStates_to_normal#transition"))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::transition_healthStates_to_normal::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 35) (character 2)) (end (line 35) (character 37))) (parent (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::transition_healthStates_to_normal"))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::vehicle"))) (kind "in out parameter") (name "vehicle") (declared-name "vehicle") (range (start (line 19) (character 2)) (end (line 19) (character 23))) (parent (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))) (authored (relationships (typing (reference "Vehicle") (range none)))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::senseTemperature"))) (kind "action") (name "senseTemperature") (declared-name "senseTemperature") (range (start (line 16) (character 1)) (end (line 16) (character 57))) (parent (node (document "d0") (qualified-name "Change and Time Triggers"))))
    (element (id (node (document "d0") (qualified-name "Change and Time Triggers::senseTemperature::temp"))) (kind "in out parameter") (name "temp") (declared-name "temp") (range (start (line 16) (character 27)) (end (line 16) (character 55))) (parent (node (document "d0") (qualified-name "Change and Time Triggers::senseTemperature"))) (authored (relationships (typing (reference "TemperatureValue") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Change and Time Triggers::DurationValue"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQ::DurationValue") (range (start (line 2) (character 16)) (end (line 2) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Change and Time Triggers::TemperatureValue"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQ::TemperatureValue") (range (start (line 1) (character 16)) (end (line 1) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Change and Time Triggers::TimeInstantValue"))) (kind membershipImport) (ordinal 0)) (authored-target "Time::TimeInstantValue") (range (start (line 3) (character 16)) (end (line 3) (character 38))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle::maintenanceInterval"))) (kind featureTyping) (ordinal 0)) (authored-target "DurationValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Change and Time Triggers::DurationValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle::maintenanceInterval"))) (kind featureTyping) (ordinal 1)) (authored-target "DurationValue") (range (start (line 10) (character 34)) (end (line 10) (character 47))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Change and Time Triggers::DurationValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle::maintenanceTime"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeInstantValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Change and Time Triggers::TimeInstantValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle::maintenanceTime"))) (kind featureTyping) (ordinal 1)) (authored-target "TimeInstantValue") (range (start (line 9) (character 30)) (end (line 9) (character 46))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Change and Time Triggers::TimeInstantValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle::maxTemperature"))) (kind featureTyping) (ordinal 0)) (authored-target "TemperatureValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Change and Time Triggers::TemperatureValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle::maxTemperature"))) (kind featureTyping) (ordinal 1)) (authored-target "TemperatureValue") (range (start (line 11) (character 29)) (end (line 11) (character 45))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Change and Time Triggers::TemperatureValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Change and Time Triggers::h"))) (kind membershipImport) (ordinal 0)) (authored-target "SI::h") (range (start (line 4) (character 16)) (end (line 4) (character 21))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))) (kind transitionSource) (ordinal 0)) (authored-target "Change and Time Triggers::healthStates::maintenance") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::maintenance")))))
    (reference (id (source (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))) (kind transitionSource) (ordinal 1)) (authored-target "Change and Time Triggers::healthStates::degraded") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::degraded")))))
    (reference (id (source (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))) (kind transitionSource) (ordinal 2)) (authored-target "Change and Time Triggers::healthStates::normal") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::normal")))))
    (reference (id (source (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))) (kind transitionSource) (ordinal 3)) (authored-target "Change and Time Triggers::healthStates::normal") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::normal")))))
    (reference (id (source (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))) (kind initialStateSource) (ordinal 0)) (authored-target "Change and Time Triggers::healthStates::normal") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::normal")))))
    (reference (id (source (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::controller"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleController") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Change and Time Triggers::VehicleController")))))
    (reference (id (source (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Change and Time Triggers::senseTemperature::temp"))) (kind featureTyping) (ordinal 0)) (authored-target "TemperatureValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Change and Time Triggers::TemperatureValue")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle::maintenanceInterval"))) (target (node (document "d0") (qualified-name "Change and Time Triggers::DurationValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle::maintenanceInterval"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle::maintenanceInterval"))) (target (node (document "d0") (qualified-name "Change and Time Triggers::DurationValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle::maintenanceInterval"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle::maintenanceTime"))) (target (node (document "d0") (qualified-name "Change and Time Triggers::TimeInstantValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle::maintenanceTime"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle::maintenanceTime"))) (target (node (document "d0") (qualified-name "Change and Time Triggers::TimeInstantValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle::maintenanceTime"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle::maxTemperature"))) (target (node (document "d0") (qualified-name "Change and Time Triggers::TemperatureValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle::maxTemperature"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle::maxTemperature"))) (target (node (document "d0") (qualified-name "Change and Time Triggers::TemperatureValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle::maxTemperature"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))) (target (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::degraded"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))) (kind transitionSource) (ordinal 1)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))) (target (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::maintenance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))) (target (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::normal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))) (kind transitionSource) (ordinal 2)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))) (target (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::normal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))) (kind transitionSource) (ordinal 3)))
    (relationship (kind initialState) (source (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))) (target (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::normal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Change and Time Triggers::healthStates"))) (kind initialStateSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::controller"))) (target (node (document "d0") (qualified-name "Change and Time Triggers::VehicleController"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::controller"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::vehicle"))) (target (node (document "d0") (qualified-name "Change and Time Triggers::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Change and Time Triggers::healthStates::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Change and Time Triggers::senseTemperature::temp"))) (target (node (document "d0") (qualified-name "Change and Time Triggers::TemperatureValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Change and Time Triggers::senseTemperature::temp"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
